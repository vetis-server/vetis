use crate::{
    errors::{ConfigError, VetisError},
    VetisHosts, VetisResult,
};
use http::Version;
use serde::Deserialize;
use std::{
    future::Future,
    net::{IpAddr, Ipv4Addr},
    pin::Pin,
};

/// A pinned future that resolves to a result of type T or a VetisError
pub type ListenerResult<'a, T> = Pin<Box<dyn Future<Output = VetisResult<T>> + Send + 'a>>;

/// A trait for defining server listeners that can handle HTTP requests
pub trait Listener {
    /// The type of host that this listener can handle
    type Host;

    /// Sets the hosts for this listener
    fn set_hosts(&mut self, hosts: VetisHosts<Self::Host>);

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
/// use http::Version;
/// use vetis::{listener::ListenerConfig};
///
/// let config = ListenerConfig::builder()
///     .port(8080)
///     .protos(vec![Version::HTTP_11])
///     .interface("127.0.0.1".parse().unwrap())
///     .build();
/// ```
#[derive(Clone)]
pub struct ListenerConfigBuilder {
    port: u16,
    protos: Vec<Version>,
    interface: IpAddr,
}

impl ListenerConfigBuilder {
    /// Sets the port number for the listener.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::listener::ListenerConfig;
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
    /// use std::net::Ipv4Addr;
    /// use vetis::listener::ListenerConfig;
    ///
    /// let config = ListenerConfig::builder()
    ///     .interface(Ipv4Addr::LOCALHOST.into())
    ///     .build();
    /// ```
    pub fn interface(mut self, interface: IpAddr) -> Self {
        self.interface = interface;
        self
    }

    /// Sets the HTTP protocol for this listener.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use http::Version;
    /// use vetis::{listener::ListenerConfig};
    ///
    /// #[cfg(feature = "http1")]
    /// let config = ListenerConfig::builder()
    ///     .protos(Version::HTTP_11)
    ///     .build();
    /// ```
    pub fn protos(mut self, protos: Vec<Version>) -> Self {
        self.protos = protos;
        self
    }

    /// Creates the `ListenerConfig` with the configured settings.
    pub fn build(self) -> VetisResult<ListenerConfig> {
        if self.port == 0 {
            return Err(VetisError::Config(ConfigError::Listener("Port cannot be 0".to_string())));
        }

        Ok(ListenerConfig { port: self.port, protos: self.protos, interface: self.interface })
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
/// use http::Version;
/// use std::net::Ipv4Addr;
/// use vetis::{listener::ListenerConfig};
///
/// let config = ListenerConfig::builder()
///     .port(8443)
///     .protos(vec![Version::HTTP_11])
///     .interface(Ipv4Addr::UNSPECIFIED.into()) // or (0, 0, 0, 0).into()
///     .build()
///     .unwrap();
///
/// println!("Listening on port {}", config.port());
/// ```
#[derive(Clone, Deserialize, PartialEq)]
pub struct ListenerConfig {
    port: u16,
    #[serde(with = "http_serde_ext::version::vec")]
    protos: Vec<Version>,
    interface: IpAddr,
}

impl Default for ListenerConfig {
    fn default() -> Self {
        ListenerConfig {
            port: 80,
            protos: vec![Version::HTTP_11],
            interface: Ipv4Addr::UNSPECIFIED.into(),
        }
    }
}

impl From<u16> for ListenerConfig {
    fn from(port: u16) -> Self {
        ListenerConfig { port, ..Default::default() }
    }
}

impl From<Version> for ListenerConfig {
    fn from(protos: Version) -> Self {
        ListenerConfig { protos: vec![protos], ..Default::default() }
    }
}

impl From<(u16, Version)> for ListenerConfig {
    fn from((port, protos): (u16, Version)) -> Self {
        ListenerConfig { port, protos: vec![protos], ..Default::default() }
    }
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
    /// use vetis::listener::ListenerConfig;
    ///
    /// let builder = ListenerConfig::builder();
    /// let config = builder.port(8080).build();
    /// ```
    pub fn builder() -> ListenerConfigBuilder {
        ListenerConfigBuilder {
            port: 80,
            protos: vec![Version::HTTP_11],
            interface: Ipv4Addr::UNSPECIFIED.into(),
        }
    }

    /// Returns the port number.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the HTTP protocol.
    pub fn protos(&self) -> &Vec<Version> {
        &self.protos
    }

    /// Returns the network interface.
    pub fn interface(&self) -> &IpAddr {
        &self.interface
    }
}
