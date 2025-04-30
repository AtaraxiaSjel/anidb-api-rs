use random_port::{PortPicker, Protocol};
use std::{borrow::Cow, time::Duration};

// !TODO: builder
#[derive(Debug)]
pub struct UdpConfig {
    pub server_address: String,
    pub client_name: String,
    pub client_ver: String,
    pub api_ver: String,

    pub username: String,
    pub password: String,
    pub local_port: u16,
    pub timeout: Duration,
}

impl UdpConfig {
    const ANIDB_UDP_ADDRESS: Cow<'static, str> = Cow::Borrowed("api.anidb.net:9000");
    const ANIDB_CLIENT_NAME: Cow<'static, str> = Cow::Borrowed("anidbudpapirs");
    const ANIDB_CLIENT_VER: Cow<'static, str> = Cow::Borrowed("1");
    const ANIDB_API_VER: Cow<'static, str> = Cow::Borrowed("3");
}

impl Default for UdpConfig {
    fn default() -> Self {
        Self {
            server_address: Self::ANIDB_UDP_ADDRESS.to_string(),
            client_name: Self::ANIDB_CLIENT_NAME.to_string(),
            client_ver: Self::ANIDB_CLIENT_VER.to_string(),
            api_ver: Self::ANIDB_API_VER.to_string(),
            username: String::default(),
            password: String::default(),
            // !TODO: remove unwrap
            local_port: PortPicker::new()
                .random(true)
                .protocol(Protocol::Udp)
                .pick()
                .unwrap(),
            timeout: Duration::from_secs(10),
        }
    }
}
