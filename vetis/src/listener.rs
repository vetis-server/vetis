use std::{future::Future, pin::Pin};

use serde::Deserialize;

use crate::{
    errors::{ConfigError, VetisError},
    server::Protocol,
    VetisVirtualHosts,
};

/// A pinned future that resolves to a result of type T or a VetisError
pub type ListenerResult<'a, T> = Pin<Box<dyn Future<Output = Result<T, VetisError>> + Send + 'a>>;

/// A trait for defining server listeners that can handle HTTP requests
pub trait Listener {
    /// The type of virtual host that this listener can handle
    type VirtualHost;

    /// Creates a new listener with the given configuration
    fn new(config: ListenerConfig) -> Self
    where
        Self: Sized;

    /// Sets the virtual hosts for this listener
    fn set_virtual_hosts(&mut self, virtual_hosts: VetisVirtualHosts<Self::VirtualHost>);

    /// Starts the listener and begins accepting connections
    fn listen(&mut self) -> ListenerResult<'_, ()>;

    /// Stops the listener and closes all connections
    fn stop(&mut self) -> ListenerResult<'_, ()>;
}

/// Builder for creating `ListenerConfig` instances.
///
/// Provides a fluent API for configuring server listeners.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::config::{ListenerConfig, Protocol};
///
/// let config = ListenerConfig::builder()
///     .port(8080)
///     .protocol(Protocol::Http1)
///     .interface("127.0.0.1")
///     .build();
/// ```
#[derive(Clone)]
pub struct ListenerConfigBuilder {
    port: u16,
    protocol: Protocol,
    interface: String,
}

impl ListenerConfigBuilder {
    /// Sets the port number for the listener.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::ListenerConfig;
    ///
    /// let config = ListenerConfig::builder()
    ///     .port(8443)
    ///     .build();
    /// ```
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the network interface to bind to.
    ///
    /// Common values:
    /// - "0.0.0.0" - All interfaces
    /// - "127.0.0.1" - Localhost only
    /// - "::1" - IPv6 localhost
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::ListenerConfig;
    ///
    /// let config = ListenerConfig::builder()
    ///     .interface("127.0.0.1")
    ///     .build();
    /// ```
    pub fn interface(mut self, interface: &str) -> Self {
        self.interface = interface.to_string();
        self
    }

    /// Sets the HTTP protocol for this listener.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::{ListenerConfig, Protocol};
    ///
    /// #[cfg(feature = "http1")]
    /// let config = ListenerConfig::builder()
    ///     .protocol(Protocol::HTTP1)
    ///     .build();
    /// ```
    pub fn protocol(mut self, protocol: Protocol) -> Self {
        self.protocol = protocol;
        self
    }

    /// Creates the `ListenerConfig` with the configured settings.
    pub fn build(self) -> Result<ListenerConfig, ConfigError> {
        if self.port == 0 {
            return Err(ConfigError::Listener("Port cannot be 0".to_string()));
        }

        if self
            .interface
            .is_empty()
        {
            return Err(ConfigError::Listener("Interface cannot be empty".to_string()));
        }

        Ok(ListenerConfig { port: self.port, protocol: self.protocol, interface: self.interface })
    }
}

/// Configuration for a server listener.
///
/// Defines how the server should listen for incoming connections,
/// including the port, protocol, interface, and SSL settings.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::config::{ListenerConfig, Protocol};
///
/// let config = ListenerConfig::builder()
///     .port(8443)
///     .protocol(Protocol::Http1)
///     .interface("0.0.0.0")
///     .build();
///
/// println!("Listening on port {}", config.port());
/// ```
#[derive(Clone, Deserialize)]
pub struct ListenerConfig {
    port: u16,
    protocol: Protocol,
    interface: String,
}

impl ListenerConfig {
    /// Creates a new `ListenerConfigBuilder` with default settings.
    ///
    /// Default values:
    /// - port: 80
    /// - ssl: false
    /// - protocol: HTTP1 (if available)
    /// - interface: "0.0.0.0"
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::ListenerConfig;
    ///
    /// let builder = ListenerConfig::builder();
    /// let config = builder.port(8080).build();
    /// ```
    pub fn builder() -> ListenerConfigBuilder {
        ListenerConfigBuilder { port: 80, protocol: Protocol::Http1, interface: "0.0.0.0".into() }
    }

    /// Returns the port number.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the HTTP protocol.
    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    /// Returns the network interface.
    pub fn interface(&self) -> &str {
        &self.interface
    }
}
