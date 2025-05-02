use super::{ApiError, command::Command, config::UdpConfig, response::UdpResponse};
use std::{
    fmt::Debug,
    io::Read,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit, block_padding::Pkcs7};
use ecow::EcoString;
use flate2::read::DeflateDecoder;
use governor::{
    Quota,
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
};
use md5::{Digest, Md5};
use tokio::{net::UdpSocket, time::timeout};
use tracing::{instrument, trace};

pub type Result<T> = std::result::Result<T, ApiError>;
type RateLimiter = governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;
type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

/// Anidb Api definition states that udp packet size cannot be larger than 1400 bytes
const MAX_UDP_SIZE: usize = 2048;
pub(crate) const ANIDB_UDP_ADDRESS: &str = "api.anidb.net:9000";
pub(crate) const ANIDB_API_VER: &str = "3";

#[derive(Debug)]
pub struct UdpClient {
    pub(crate) config: Arc<UdpConfig>,
    pub(crate) socket: Arc<UdpSocket>,
    pub(crate) rate_limiter: Arc<RateLimiter>,
    pub(crate) session_key: Option<EcoString>,
    pub(crate) timeout: Duration,
    pub(crate) authenticated: bool,
    pub(crate) behind_nat: bool,
    pub(crate) enc_enabled: bool,
    pub(crate) enc_key: [u8; 16],
}

// !TODO: handle timeout (logout, reauth, encryption, etc...)

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
            enc_enabled: false,
            enc_key: Default::default(),
            timeout: Duration::from_secs(10),
        })
    }

    /// Connect udp socket to `AniDB` server
    /// # Errors
    /// Return `ApiError::Io` if socket failed to connect to `AniDB` server
    pub async fn connect(&self) -> Result<()> {
        self.socket
            .connect(ANIDB_UDP_ADDRESS)
            .await
            .map_err(ApiError::Io)
    }

    #[instrument(skip(self), level = "trace")]
    pub(crate) fn calculate_enc_key(&mut self, salt: &str) -> Result<()> {
        let config = self.config.clone();
        let api_key = config.udp_api_key.clone().ok_or(ApiError::NoApiKey)?;

        // Create aes encryption key from md5(api_key + salt)
        let key = api_key + salt;
        let mut hasher = Md5::default();
        hasher.update(key.as_ref());
        let hash = hasher.finalize();
        trace!(salted = ?key, key = ?hash[..], "aes key calculated");
        self.enc_key = hash.into();
        Ok(())
    }

    #[instrument(skip_all, level = "trace")]
    pub(crate) async fn send_and_recv(
        &self,
        send_buf: &[u8],
        recv_buf: &mut [u8],
    ) -> Result<usize> {
        self.rate_limiter.until_ready().await;

        let len = timeout(self.timeout, self.socket.send(send_buf))
            .await
            .map_err(|_| ApiError::Timeout)?
            .map_err(ApiError::Io)?;

        trace!(length = %len, data = ?send_buf, "send request");

        let len = timeout(self.timeout, self.socket.recv(recv_buf))
            .await
            .map_err(|_| ApiError::Timeout)?
            .map_err(ApiError::Io)?;

        trace!(length = %len, data = ?&recv_buf[..len], "received response");

        Ok(len)
    }

    #[instrument(skip_all, level = "trace")]
    pub(crate) async fn request(&self, buf: &[u8]) -> Result<UdpResponse> {
        let mut recv_buf = [0u8; MAX_UDP_SIZE];

        let (data, len) = if self.enc_enabled {
            let key = &self.enc_key;
            let encrypted = Aes128EcbEnc::new(key.into()).encrypt_padded_vec_mut::<Pkcs7>(buf);
            trace!(plain = ?buf, encrypted = ?encrypted, "data encrypted");
            let len = self.send_and_recv(&encrypted, &mut recv_buf).await?;

            let decrypted = Aes128EcbDec::new(key.into())
                .decrypt_padded_mut::<Pkcs7>(&mut recv_buf[..len])
                .map_err(|_| ApiError::UnexpectedResponse)?;
            trace!(encrypted = ?encrypted, decrypted = ?decrypted, "data decrypted");
            (decrypted, decrypted.len())
        } else {
            let len = self.send_and_recv(buf, &mut recv_buf).await?;
            (&recv_buf[..], len)
        };

        // deflate
        if len > 2 && data[0] == 0 && data[1] == 0 {
            let mut decoder = DeflateDecoder::new(&data[2..len]);
            let mut buf: Vec<u8> = Vec::with_capacity(len + 10);
            decoder.read_to_end(&mut buf)?;
            trace!(data = ?buf, "deflate decoded");
            UdpResponse::from(&buf)
        } else {
            UdpResponse::from(&data[..len])
        }
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn execute<T, C>(&mut self, command: C) -> Result<T>
    where
        T: From<UdpResponse> + Debug,
        C: Command + Debug,
    {
        command.execute(self).await
    }
}
