use super::{
    ApiError, config::UdpConfig, response::UdpResponse, status::StatusCode, util::check_status,
};

use ascii::AsciiChar;
use governor::{
    Quota,
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};
use tokio::{net::UdpSocket, time::timeout};
use tracing::{instrument, trace};

pub type Result<T> = std::result::Result<T, ApiError>;
type RateLimiter = governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

/// Anidb Api definition states that udp packet size
/// cannot be larger than 1400 bytes
const MAX_UDP_SIZE: usize = 1400;

#[derive(Debug)]
pub struct UdpClient {
    config: Arc<UdpConfig>,
    socket: Arc<UdpSocket>,
    rate_limiter: Arc<RateLimiter>,
    session_key: Option<String>,
    authenticated: bool,
    behind_nat: bool,
}

impl UdpClient {
    /// Construct a new instance of `UdpClient`
    /// # Errors
    /// Return `ApiError::Io` if socket failed to bind to local port
    /// # Panics
    /// Function should not panic at any circumstances
    pub async fn new(config: UdpConfig) -> Result<Self> {
        // !TODO: short and long period delay
        let quota = Quota::with_period(Duration::from_secs(2)).unwrap();
        let rate_limiter: RateLimiter = RateLimiter::direct(quota);

        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), config.local_port);
        let socket = UdpSocket::bind(bind_addr).await.map_err(ApiError::Io)?;
        Ok(Self {
            config: Arc::new(config),
            socket: Arc::new(socket),
            session_key: None,
            authenticated: false,
            behind_nat: false,
            rate_limiter: Arc::new(rate_limiter),
        })
    }

    /// Connect udp socket to `AniDB` server
    /// # Errors
    /// Return `ApiError::Io` if socket failed to connect to `AniDB` server
    pub async fn connect(&self) -> Result<()> {
        self.socket
            .connect(self.config.server_address.clone())
            .await
            .map_err(ApiError::Io)
    }

    #[instrument(skip_all, level = "trace")]
    async fn send_and_recv(&self, send_buf: &[u8], recv_buf: &mut [u8]) -> Result<usize> {
        self.rate_limiter.until_ready().await;

        let len = self.socket.send(send_buf).await.map_err(ApiError::Io)?;
        trace!(length = %len, data = ?String::from_utf8_lossy(send_buf), "send request");

        let len = timeout(self.config.timeout, self.socket.recv(recv_buf))
            .await
            .map_err(|_| ApiError::Timeout)?
            .map_err(ApiError::Io)?;

        trace!(length = %len, data = ?String::from_utf8_lossy(&recv_buf[..len]), "received response");

        Ok(len)
    }

    #[instrument(skip_all, level = "trace")]
    async fn request(&self, buf: &[u8]) -> Result<UdpResponse> {
        let mut recv_buf = [0u8; MAX_UDP_SIZE];
        let len = self.send_and_recv(buf, &mut recv_buf).await?;
        UdpResponse::from(&recv_buf[..len])
    }

    #[instrument(skip_all, level = "debug")]
    pub async fn ping(&self) -> Result<()> {
        let resp = self.request(b"PING").await?;
        check_status(resp.status, StatusCode::Pong)
    }

    #[instrument(skip_all, level = "debug")]
    pub async fn login(&mut self) -> Result<()> {
        let command = format!(
            "AUTH user={}&pass={}&protover={}&client={}&clientver={}&nat=1",
            self.config.username,
            self.config.password,
            self.config.api_ver,
            self.config.client_name,
            self.config.client_ver
        );
        // let command = anidb_encode(&command);
        let resp = self.request(command.as_bytes()).await?;
        // Check if login successful
        if check_status(resp.status, StatusCode::LoginFailed).is_err() {
            self.session_key = None;
            self.authenticated = false;
            return Err(ApiError::IncorrectUsernameOrPassword);
        }
        if check_status(resp.status, StatusCode::LoginAccepted).is_err() {
            check_status(resp.status, StatusCode::LoginAcceptedNewVersion)?;
        }

        let mut header = resp.message.split(AsciiChar::Space);
        // Session key
        let session_key = header.next().ok_or(ApiError::UnexpectedResponse)?;
        self.session_key = Some(session_key.to_string());
        self.authenticated = true;
        // Parse ipv4:port
        let ip_str = header.next().ok_or(ApiError::UnexpectedResponse)?;
        let addr = ip_str
            .as_str()
            .parse::<SocketAddr>()
            .map_err(|_| ApiError::ParseError("Invalid socket address syntax".to_string()))?;
        if addr.port() != self.config.local_port {
            self.behind_nat = true;
        }

        Ok(())
    }

    #[instrument(skip_all, level = "debug")]
    pub async fn logout(&mut self) -> Result<()> {
        if let Some(session_key) = &self.session_key {
            // let command = anidb_encode(&format!("LOGOUT s={}", &session_key));
            let command = format!("LOGOUT s={}", &session_key);
            let resp = self.request(command.as_bytes()).await?;

            if check_status(resp.status, StatusCode::LoggedOut).is_ok() {
                self.session_key = None;
                self.authenticated = false;
                self.behind_nat = false;
                Ok(())
            } else {
                check_status(resp.status, StatusCode::NotLoggedIn)
            }
        } else {
            Err(ApiError::NotLoggedIn)
        }
    }
}
