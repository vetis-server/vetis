#![allow(unreachable_code, dead_code)]
use http::Version;

pub(crate) const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");

pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

pub(crate) const IP6_SERVER_CERT: &[u8] = include_bytes!("../../../certs/ip6-server.der");
pub(crate) const IP6_SERVER_KEY: &[u8] = include_bytes!("../../../certs/ip6-server.key.der");

pub(crate) const fn default_protocol_version() -> Version {
    #[cfg(feature = "http1")]
    return Version::HTTP_11;
    #[cfg(feature = "http2")]
    return Version::HTTP_2;
    #[cfg(feature = "http3")]
    return Version::HTTP_3;
}
