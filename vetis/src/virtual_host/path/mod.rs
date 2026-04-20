//! Path module for virtual host configuration.

/// Interface for path configuration.
pub mod interface;
/// Proxy path configuration.
#[cfg(feature = "reverse-proxy")]
pub mod proxy;
/// Static files path configuration.
#[cfg(feature = "static-files")]
pub mod static_files;
