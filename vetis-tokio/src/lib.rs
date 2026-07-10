#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#[cfg(all(any(feature = "http2", feature = "http3"), not(feature = "rust-tls")))]
compile_error!("http2 and http3 requires rust-tls!");

/// Module for handling basic authentication
#[cfg(feature = "auth")]
pub mod auth;
/// HTTP module
pub mod http;
/// Listener module
pub mod listener;
/// Runtime module
pub mod rt;
/// Tests module
#[cfg(test)]
mod tests;
/// TLS module
mod tls;
/// Virtual host module
pub mod virtual_host;

pub use crate::rt::Vetis;
pub use vetis::{
    errors,
    listener::ListenerConfig,
    security::SecurityConfig,
    server::{Protocol, Server, ServerConfig},
    virtual_host::{handler_fn, VirtualHostConfig},
    VetisRwLock, VetisVirtualHosts,
};
