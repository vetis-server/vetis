#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use genswap::GenSwap;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

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
/// use vetis::host::HostConfig;
/// use vetis::{VetisHosts};
/// use std::{sync::Arc, collections::HashMap};
///
/// let hosts: VetisHosts<HostConfig> =
///     Arc::new(VetisRwLock::new(HashMap::new()));
/// ```
pub type VetisHosts<T> = Arc<GenSwap<HashMap<String, Arc<T>>>>;

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
