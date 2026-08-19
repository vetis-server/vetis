#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#[cfg(all(any(feature = "http2", feature = "http3"), not(feature = "rust-tls")))]
compile_error!("http2 and http3 requires rust-tls!");

/// Host module
pub mod host;
/// Listener module
pub mod listener;
/// Runtime module
pub mod rt;
/// Tests module
#[cfg(test)]
mod tests;
/// TLS module
mod tls;

pub use crate::rt::Vetis;
pub use vetis::{
    base::VetisServer,
    errors,
    host::{handler_fn, HostConfig},
    listener::ListenerConfig,
    request::Request,
    response::Response,
    security::SecurityConfig,
    server::ServerConfig,
    VetisHosts, VetisRwLock,
};
