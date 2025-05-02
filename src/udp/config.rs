use ecow::EcoString;

use super::secret_string::SecretString;

// !TODO: builder
#[derive(Debug, Clone)]
pub struct UdpConfig {
    /// Name of the client from anidb site
    pub client_name: EcoString,
    /// Version of the client from anidb site
    pub client_ver: EcoString,
    /// Local port to bind udp socket to
    pub local_port: u16,
    /// Username for anidb site
    pub username: EcoString,
    /// Password for anidb site
    pub password: SecretString,
    /// UDP Api key from user profile page. Used for encryption
    pub udp_api_key: Option<SecretString>,
    /// Enable or disable encryption. Needs udp api key
    pub encryption: bool,
}
