#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use papaya::HashMap;
use serde::Deserialize;
use std::{future::Future, pin::Pin, sync::Arc};

pub use base::VetisServer;
pub use request::Request;
pub use response::Response;

/// Basic authentication module
pub mod auth;
/// Base module
pub mod base;
/// Error handling module
pub mod errors;
/// Virtual host configuration and management module
pub mod host;
/// Listener configuration and management module
pub mod listener;
/// HTTP request module
pub mod request;
/// HTTP response module
pub mod response;
/// Security module
pub mod security;
/// Server module
pub mod server;
/// Internal tests module
#[cfg(test)]
mod tests;
/// Utility functions and helpers
pub mod utils;

/// A type alias for Result returned by vetis functions
///
/// This is used to standardize error handling across the library.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{VetisResult, errors::VetisError};
///
/// fn process_data() -> VetisResult<i32> {
///     // Some operation that might fail
///     Ok(42)
/// }
/// ```
pub type VetisResult<T> = Result<T, crate::errors::VetisError>;

/// A type alias for a vector of virtual hosts
///
/// This is used to store virtual hosts in a map with hostname and port as the key.
///
/// # Examples
///
/// ```rust,no_run
/// use papaya::HashMap;
/// use std::{sync::Arc, collections::HashMap};
/// use vetis::{VetisHosts, host::HostConfig};
///
/// let hosts: VetisHosts<HostConfig> =
///     Arc::new(HashMap::new());
/// ```
pub type VetisHosts<T> = Arc<HashMap<String, Arc<T>>>;

/// A pinned future that resolves to a result of type T or a VetisError
///
/// This is used for async operations that return a `VetisResult<T>`.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{Request, Response, errors::VetisError, VetisFutureResult};
///
/// let future: VetisFutureResult<'static, Response> = Box::pin(async move {
///     // Process request...
///     Ok(Response::builder()
///         .status(http::StatusCode::OK)
///         .text("OK"))
/// });
/// ```
pub type VetisFutureResult<'a, T> = Pin<Box<dyn Future<Output = VetisResult<T>> + Send + 'a>>;

/// Type alias for boxed handler closures.
///
/// This represents an async function that takes a `Request` and returns
/// a `Response` or an error. Handlers are the core of request processing
/// in VeTiS hosts.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::HandlerFn;
/// use vetis::{Request, Response, errors::VetisError};
///
/// let handler: HandlerFn = Box::new(|request: Request| {
///     Box::pin(async move {
///         // Process request...
///         Ok(Response::builder()
///             .status(http::StatusCode::OK)
///             .text("OK"))
///     })
/// });
/// ```
pub type HandlerFn = Box<dyn Fn(Request) -> VetisFutureResult<'static, Response> + Send + Sync>;

#[derive(Deserialize, Clone, PartialEq)]
/// Enum for ALPN
pub enum Alpn {
    /// HTTP/1.1
    Http11,
    /// H2
    H2,
    /// H2C
    H2c,
    /// H3
    H3,
    /// DOT
    Dot,
    /// DOC
    Doh,
    /// DOQ
    Doq,
    /// ACME-TLS/1
    AcmeTls1,
}

impl From<&str> for Alpn {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "http/1.1" => Alpn::Http11,
            "h2" => Alpn::H2,
            "h2c" => Alpn::H2c,
            "h3" => Alpn::H3,
            "dot" => Alpn::Dot,
            "doh" => Alpn::Doh,
            "doq" => Alpn::Doq,
            "acme-tls/1" => Alpn::AcmeTls1,
            &_ => panic!("Not a valid ALPN protocol"),
        }
    }
}

impl From<Vec<u8>> for Alpn {
    fn from(value: Vec<u8>) -> Self {
        match value.as_slice() {
            b"http/1.1" => Alpn::Http11,
            b"h2" => Alpn::H2,
            b"h2c" => Alpn::H2c,
            b"h3" => Alpn::H3,
            b"dot" => Alpn::Dot,
            b"doh" => Alpn::Doh,
            b"doq" => Alpn::Doq,
            b"acme-tls/1" => Alpn::AcmeTls1,
            &_ => panic!("Not a valid ALPN protocol"),
        }
    }
}

impl From<&Alpn> for Vec<u8> {
    fn from(value: &Alpn) -> Self {
        match value {
            Alpn::Http11 => b"http/1.1".into(),
            Alpn::H2 => b"h2".into(),
            Alpn::H2c => b"h2c".into(),
            Alpn::H3 => b"h3".into(),
            Alpn::Dot => b"dot".into(),
            Alpn::Doh => b"doh".into(),
            Alpn::Doq => b"doq".into(),
            Alpn::AcmeTls1 => b"acme-tls/1".into(),
        }
    }
}
