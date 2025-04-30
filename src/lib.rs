/*!
# Example

# Crate features

*/
#![allow(dead_code)]
#[cfg(feature = "http")]
/// `AniDB` HTTP Api definition: <https://wiki.anidb.net/HTTP_API_Definition>
pub mod http;

#[cfg(feature = "udp")]
/// `AniDB` UDP Api definition: <https://wiki.anidb.net/UDP_API_Definition>
pub mod udp;
