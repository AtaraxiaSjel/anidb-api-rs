use super::{
    ApiError, command::Command, config::UdpConfig, response::UdpResponse, socket::AnidbSocket,
    status::StatusCode,
};
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
use tracing::{instrument, trace};

pub type Result<T> = std::result::Result<T, ApiError>;
type RateLimiter = governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;
type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;

pub(crate) const ANIDB_API_VER: &str = "3";

#[derive(Debug)]
pub struct UdpClient {
    pub(crate) config: Arc<UdpConfig>,
    pub(crate) socket: Arc<AnidbSocket>,
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
        let timeout = Duration::from_secs(30);

        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), config.local_port);
        let socket = AnidbSocket::new(bind_addr, timeout).await?;

        Ok(Self {
            config: Arc::new(config),
            socket: Arc::new(socket),
            session_key: None,
            authenticated: false,
            behind_nat: false,
            rate_limiter: Arc::new(rate_limiter),
            enc_enabled: false,
            enc_key: Default::default(),
            timeout,
        })
    }

    pub(crate) fn reset(&mut self) {
        self.session_key = None;
        self.authenticated = false;
        self.behind_nat = false;
        self.enc_enabled = false;
        self.enc_key = Default::default();
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
    pub(crate) async fn request(&self, buf: &[u8]) -> Result<UdpResponse> {
        let mut recv_buf = [0u8; 2048];
        self.rate_limiter.until_ready().await;

        let (data, len) = if self.enc_enabled {
            let key = &self.enc_key;
            let encrypted = Aes128EcbEnc::new(key.into()).encrypt_padded_vec_mut::<Pkcs7>(buf);
            trace!(plain = ?buf, encrypted = ?encrypted, "data encrypted");
            let len = self.socket.send_raw(&encrypted, &mut recv_buf).await?;

            let decrypted = Aes128EcbDec::new(key.into())
                .decrypt_padded_mut::<Pkcs7>(&mut recv_buf[..len])
                .map_err(|_| ApiError::Decrypt)?;
            trace!(encrypted = ?encrypted, decrypted = ?decrypted, "data decrypted");
            (decrypted, decrypted.len())
        } else {
            let len = self.socket.send_raw(buf, &mut recv_buf).await?;
            (&recv_buf[..], len)
        };

        // deflate
        let response = if len > 2 && data[0] == 0 && data[1] == 0 {
            let mut decoder = DeflateDecoder::new(&data[2..len]);
            let mut buf: Vec<u8> = Vec::with_capacity(len + 10);
            decoder
                .read_to_end(&mut buf)
                .map_err(|err| ApiError::Io(err.kind()))?;
            trace!(data = ?buf, "deflate decoded");
            UdpResponse::from(&buf)
        } else {
            UdpResponse::from(&data[..len])
        };
        if let Ok(resp) = &response {
            if resp.status == StatusCode::Banned {
                let data = resp.data.clone();
                let desc = itertools::intersperse(data, " ".to_string()).collect::<String>();
                return Err(ApiError::Banned(desc));
            }
        }
        response
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn execute<T: From<UdpResponse> + Debug>(
        &mut self,
        command: impl Command + Debug,
    ) -> Result<T> {
        command.execute(self).await
    }
}

// !TODO: implement drop with logout
// impl Drop for UdpClient {
//     fn drop(&mut self) {
//         tokio::spawn(async {
//             self.execute::<UdpResponse>(command::Logout()).await;
//         });
//     }
// }
