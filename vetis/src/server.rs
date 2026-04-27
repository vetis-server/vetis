use std::{collections::HashMap, sync::Arc};

use http::HeaderMap;
use hyper_body_utils::HttpBody;
use serde::Deserialize;

use crate::{
    errors::{ConfigError, VetisError},
    listener::{Listener, ListenerConfig},
    Server, VetisRwLock, VetisVirtualHosts,
};

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
