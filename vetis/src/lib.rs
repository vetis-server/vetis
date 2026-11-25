#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::hook::Hook;
use crate::log::LogMessage;
use crate::matcher::Matcher;
pub use base::VetisServer;
use compact_str::CompactString;
use crossfire::{MAsyncTx, Rx, mpsc};
use papaya::HashMap;
pub use request::Request;
pub use response::Response;
#[cfg(feature = "single-threaded")]
use std::rc::Rc;
#[cfg(feature = "multi-threaded")]
use std::sync::Arc;
use std::{future::Future, pin::Pin};

/// Base module
pub mod base;
/// Cache module
pub mod cache;
/// Error handling module
pub mod errors;
/// Hook module
pub mod hook;
/// Virtual host configuration and management module
pub mod host;
/// Listener configuration and management module
pub mod listener;
/// Log module
pub mod log;
/// Matcher module
pub mod matcher;
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

/// Internal tests module
#[cfg(test)]
mod tests;

/// Utility functions and helpers
pub mod utils;

/// Type alias for CompactString
pub type Str = CompactString;

/// A type alias for Result returned by vetis functions
///
/// This is used to standardize error handling across the library.
pub type VetisResult<T> = Result<T, crate::errors::VetisError>;

/// A type alias for Result returned by vetis functions
///
/// This is used to standardize error handling across the library.
pub type VetisTestResult<T> = Result<T, Box<dyn std::error::Error>>;

/// A pinned future that resolves to a result of type T or a VetisError
///
/// This is used for async operations that return a `VetisResult<T>`.
pub type VetisFutureResult<'a, T> = Pin<Box<dyn Future<Output = VetisResult<T>> + Send + 'a>>;

/// A type alias for a vector of virtual hosts
///
/// This is used to store virtual hosts in a map with hostname and port as the key.
pub type VetisHosts<T> = Arc<HashMap<String, Arc<T>>>;

#[cfg(feature = "single-threaded")]
/// Trait bounds alias for async typed matchers
pub trait VetisHook: Hook + Send {}

#[cfg(feature = "single-threaded")]
impl<T> VetisHook for T where T: Hook + Send {}

#[cfg(feature = "single-threaded")]
/// A type alias for a hashmap of Hooks
///
/// This is used to store virtual hosts in a map with hostname and port as the key.
pub type VetisHooks = Vec<Box<dyn Hook>>;
#[cfg(feature = "single-threaded")]
/// Trait bounds alias for async typed matchers
pub trait VetisMatcher: Hook + Send {}

#[cfg(feature = "single-threaded")]
impl<T> VetisMatcher for T where T: Hook + Send {}

#[cfg(feature = "single-threaded")]
/// A type alias for a hashmap of Hooks
///
/// This is used to store virtual hosts in a map with hostname and port as the key.
pub type VetisMatchers = Vec<Box<dyn Hook>>;

#[cfg(feature = "multi-threaded")]
/// Trait bounds alias for async typed matchers
pub trait VetisHook: Hook + Send + Sync {}
#[cfg(feature = "multi-threaded")]
impl<T> VetisHook for T where T: Hook + Send + Sync {}
#[cfg(feature = "multi-threaded")]

/// A type alias for a hashmap of Hooks
///
/// This is used to store virtual hosts in a map with hostname and port as the key.
pub type VetisHooks = Vec<Box<dyn VetisHook>>;

#[cfg(feature = "multi-threaded")]
/// Trait bounds alias for async typed matchers
pub trait VetisMatcher: Matcher + Send + Sync {}
#[cfg(feature = "multi-threaded")]
impl<T> VetisMatcher for T where T: Matcher + Send + Sync {}
#[cfg(feature = "multi-threaded")]
/// A type alias for a hashmap of Hooks
///
/// This is used to store virtual hosts in a map with hostname and port as the key.
pub type VetisMatchers = Vec<Box<dyn VetisMatcher>>;

/// Type of LogMessage sender halve of channel
pub type LogSender = MAsyncTx<mpsc::Array<LogMessage>>;

/// Type of LogMessage receiver halve of channel
pub type LogReceiver = Rx<mpsc::Array<LogMessage>>;
