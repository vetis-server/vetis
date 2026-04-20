#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use std::{collections::HashMap, future::Future, sync::Arc};

use async_lock::RwLock;
use serde::Deserialize;

use crate::{
    errors::{ConfigError, VetisError},
    listener::ListenerConfig,
};

/// Basic authentication module
#[cfg(feature = "basic-auth")]
pub mod auth;
/// Error handling module
pub mod errors;
/// HTTP request/response handling module
pub mod http;
/// Listener configuration and management module
pub mod listener;
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

/// Supported HTTP protocols.
///
/// The protocol enum is feature-gated to only include protocols
/// that are enabled in the crate's feature flags.
///
/// # Examples
///
/// ```rust,ignore
/// use vetis::config::Protocol;
///
/// #[cfg(feature = "http1")]
/// let protocol = Protocol::Http1;
///
/// #[cfg(feature = "http2")]
/// let protocol = Protocol::Http2;
///
/// #[cfg(feature = "http3")]
/// let protocol = Protocol::Http3;
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[non_exhaustive]
pub enum Protocol {
    /// HTTP/1.1 protocol
    Http1,
    /// HTTP/2 protocol (requires TLS)
    Http2,
    /// HTTP/3 protocol over QUIC (requires TLS)
    Http3,
}

/// Trait for server implementations.
///
/// This trait defines the interface that all server implementations must provide.
/// It allows for different server backends while maintaining a consistent API.
///
/// # Examples
///
/// ```rust,ignore
/// use vetis::{Vetis, config::ServerConfig, errors::VetisError};
///
/// // Create a server instance
/// let config = ServerConfig::builder().build();
/// let mut server = Vetis::new(config);
///
/// // Start the server
/// async fn run_server() -> Result<(), VetisError> {
///     server.start().await?;
///     // Server is running...
///     server.stop().await?;
///     Ok(())
/// }
/// ```
pub trait Server {
    /// The type of virtual host that this server can handle
    type VirtualHost;
    /// Creates a new server instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration containing listeners and settings
    fn new(config: ServerConfig) -> Self;

    /// Sets the virtual hosts for the server.
    ///
    /// This must be called before starting the server.
    ///
    /// # Arguments
    ///
    /// * `virtual_hosts` - Arc containing the virtual host registry
    fn set_virtual_hosts(&mut self, virtual_hosts: VetisVirtualHosts<Self::VirtualHost>);

    /// Starts the server and begins accepting connections.
    ///
    /// # Errors
    ///
    /// Returns an error if the server fails to start, bind to addresses,
    /// or initialize TLS.
    fn start(&mut self) -> impl Future<Output = Result<(), VetisError>>;

    /// Stops the server gracefully.
    ///
    /// This method waits for ongoing connections to complete
    /// before shutting down.
    ///
    /// # Errors
    ///
    /// Returns an error if the server fails to stop properly.
    fn stop(&mut self) -> impl Future<Output = Result<(), VetisError>>;
}

/// Builder for creating `ServerConfig` instances.
///
/// Provides a fluent API for configuring the overall server,
/// including multiple listeners for different ports and protocols.
///
/// # Examples
///
/// ```rust,ignore
/// use vetis::{listener::ListenerConfig, ServerConfig, Protocol};
///
/// let http_listener = ListenerConfig::builder()
///     .port(80)
///     .protocol(Protocol::Http1)
///     .build();
///
/// let https_listener = ListenerConfig::builder()
///     .port(443)
///     .protocol(Protocol::Http1)
///     .build();
///
/// let config = ServerConfig::builder()
///     .add_listener(http_listener)
///     .add_listener(https_listener)
///     .build();
/// ```
#[derive(Clone)]
pub struct ServerConfigBuilder {
    listeners: Vec<ListenerConfig>,
}

impl ServerConfigBuilder {
    /// Adds a listener configuration to the server.
    ///
    /// Multiple listeners can be added to support different
    /// ports, protocols, or interfaces simultaneously.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use vetis::{listener::ListenerConfig, ServerConfig};
    ///
    /// let listener = ListenerConfig::builder().port(8080).build();
    /// let config = ServerConfig::builder()
    ///     .add_listener(listener)
    ///     .build();
    /// ```
    pub fn add_listener(mut self, listener: ListenerConfig) -> Self {
        self.listeners
            .push(listener);
        self
    }

    /// Creates the `ServerConfig` with the configured listeners.
    pub fn build(self) -> Result<ServerConfig, ConfigError> {
        if self
            .listeners
            .is_empty()
        {
            return Err(ConfigError::Server("No listeners configured".to_string()));
        }

        Ok(ServerConfig { listeners: self.listeners })
    }
}

/// Global server configuration.
///
/// Contains all the listeners that the server should use to accept
/// incoming connections. Each listener can have different settings
/// for port, protocol, and interface.
///
/// # Examples
///
/// ```rust,ignore
/// use vetis::{listener::ListenerConfig, ServerConfig};
///
/// let config = ServerConfig::builder()
///     .add_listener(ListenerConfig::builder().port(80).build())
///     .add_listener(ListenerConfig::builder().port(443).ssl(true).build())
///     .build();
///
/// println!("Server has {} listeners", config.listeners().len());
/// ```
#[derive(Clone, Default, Deserialize)]
pub struct ServerConfig {
    listeners: Vec<ListenerConfig>,
}

impl ServerConfig {
    /// Creates a new `ServerConfigBuilder` with no listeners.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use vetis::{listener::ListenerConfig, ServerConfig};
    ///
    /// let config = ServerConfig::builder()
    ///     .add_listener(ListenerConfig::builder().port(8080).build())
    ///     .build();
    /// ```
    pub fn builder() -> ServerConfigBuilder {
        ServerConfigBuilder { listeners: vec![] }
    }

    /// Returns a reference to all configured listeners.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use vetis::{listener::ListenerConfig, ServerConfig};
    ///
    /// let config = ServerConfig::builder()
    ///     .add_listener(ListenerConfig::builder().port(80).build())
    ///     .build();
    ///
    /// for listener in config.listeners() {
    ///     println!("Listening on port {}", listener.port());
    /// }
    /// ```
    pub fn listeners(&self) -> &Vec<ListenerConfig> {
        &self.listeners
    }
}
