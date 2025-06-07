use std::time::Duration;

use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    time::timeout,
};
use tracing::{instrument, trace};

use super::{ApiError, Result};

/// Anidb Api definition states that udp packet size cannot be larger than 1400 bytes.
/// Alloc 2kb for safety
const MAX_UDP_SIZE: usize = 2048;
const ANIDB_UDP_ADDRESS: &str = "api.anidb.net:9000";

#[derive(Debug)]
pub(crate) struct AnidbSocket {
    timeout: Duration,
    socket: UdpSocket,
    buffer: [u8; MAX_UDP_SIZE],
}

impl AnidbSocket {
    pub(crate) async fn new<A: ToSocketAddrs>(addr: A, timeout: Duration) -> Result<Self> {
        let socket = UdpSocket::bind(addr)
            .await
            .map_err(|err| ApiError::Io(err.kind()))?;
        socket
            .connect(ANIDB_UDP_ADDRESS)
            .await
            .map_err(|err| ApiError::Io(err.kind()))?;
        Ok(Self {
            timeout,
            socket,
            buffer: [0u8; MAX_UDP_SIZE],
        })
    }

    #[instrument(skip_all, level = "trace")]
    pub(crate) async fn send_raw(&self, send_buf: &[u8], recv_buf: &mut [u8]) -> Result<usize> {
        let len = timeout(self.timeout, self.socket.send(send_buf))
            .await
            .map_err(|_| ApiError::Timeout)?
            .map_err(|err| ApiError::Io(err.kind()))?;

        trace!(length = %len, data = ?send_buf, "send request");

        let len = timeout(self.timeout, self.socket.recv(recv_buf))
            .await
            .map_err(|_| ApiError::Timeout)?
            .map_err(|err| ApiError::Io(err.kind()))?;

        trace!(length = %len, data = ?&recv_buf[..len], "received response");

        Ok(len)
    }
}
