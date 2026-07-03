#![allow(unreachable_code, dead_code)]
use deboa_smol::HttpVersion;
use vetis::server::Protocol;

#[cfg(feature = "auth")]
mod auth;
mod config;
mod http;
mod lib;
mod paths;

#[cfg(target_os = "linux")]
mod server;

mod tls;
mod virtual_host;

pub(crate) const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");

pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

pub(crate) const IP6_SERVER_CERT: &[u8] = include_bytes!("../../../certs/ip6-server.der");
pub(crate) const IP6_SERVER_KEY: &[u8] = include_bytes!("../../../certs/ip6-server.key.der");

pub(crate) const fn vetis_default_protocol() -> Protocol {
    #[cfg(feature = "http1")]
    return Protocol::Http1;
    #[cfg(feature = "http2")]
    return Protocol::Http2;
    #[cfg(feature = "http3")]
    return Protocol::Http3;
}

pub(crate) const fn deboa_default_protocol() -> HttpVersion {
    #[cfg(feature = "http1")]
    return HttpVersion::Http1;
    #[cfg(feature = "http2")]
    return HttpVersion::Http2;
    #[cfg(feature = "http3")]
    return HttpVersion::Http3;
}
