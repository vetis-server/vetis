#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::log::LogMessage;
use arcstr::ArcStr;
pub use base::VetisServer;
use crossfire::{MAsyncTx, Rx, mpsc};
use papaya::HashMap;
use patricia_tree::StringPatriciaMap;
pub use request::Request;
pub use response::Response;
use std::{future::Future, pin::Pin, sync::Arc};

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
/// Log module
pub mod log;
/// HTTP request module
pub mod request;
/// HTTP response module
pub mod response;
/// Security module
pub mod security;
/// Server module
pub mod server;
/// Telemetry module
pub mod telemetry;
/// Worker module
pub mod worker;

/// Internal tests module
#[cfg(test)]
mod tests;

/// Utility functions and helpers
pub mod utils;

/// Type alias for ArcStr
pub type Str = ArcStr;

/// A type alias for path router
///
/// This is used to standardize reference to path router accross library.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{VetisRouter, errors::VetisError};
///
/// fn process_data() -> VetisResult<i32> {
///     // Some operation that might fail
///     Ok(42)
/// }
/// ```
pub type VetisPathRouter<T> = StringPatriciaMap<Arc<T>>;

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

/// A type alias for Result returned by vetis functions
///
/// This is used to standardize error handling across the library.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{VetisTestResult, Box<dyn Error>};
///
/// fn process_data() -> VetisTestResult<i32> {
///     // Some operation that might fail
///     Ok(42)
/// }
/// ```
pub type VetisTestResult<T> = Result<T, Box<dyn std::error::Error>>;

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

/// Type of LogMessage sender halve of channel
pub type LogSender = MAsyncTx<mpsc::Array<LogMessage>>;

/// Type of LogMessage receiver halve of channel
pub type LogReceiver = Rx<mpsc::Array<LogMessage>>;
