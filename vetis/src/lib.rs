#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use std::{collections::HashMap, sync::Arc};

use async_lock::RwLock;

/// Basic authentication module
#[cfg(feature = "auth")]
pub mod auth;
/// Error handling module
pub mod errors;
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
mod tests;
/// Utility functions and helpers
pub mod utils;
/// Virtual host configuration and management module
pub mod virtual_host;

/// A type alias for a read-write lock wrapping a value
pub type VetisRwLock<T> = RwLock<T>;

/// A type alias for a vector of virtual hosts
pub type VetisVirtualHosts<T> = Arc<VetisRwLock<HashMap<(Arc<str>, u16), T>>>;

pub use request::Request;
pub use response::Response;
