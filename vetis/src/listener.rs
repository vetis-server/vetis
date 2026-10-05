use crate::VetisResult;
use serde::Deserialize;
use std::{
    future::Future,
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};

/// A trait for defining server listeners that can handle HTTP requests
pub trait Listener {
    /// The type of host that this listener can handle
    type RuntimeHost;

    /// The type of logger and its inner sender
    type Logger;

    /// Add a host to this listener
    fn add_host(&mut self, host: Arc<Self::RuntimeHost>) -> VetisResult<()>;

    /// Remove a host from this listener
    fn remove_host(&mut self, hostname: &str) -> VetisResult<()>;

    /// Allow set logger
    fn logger(&mut self, logger: Self::Logger);

    /// Returns the number of hosts
    fn total_hosts(&self) -> usize;

    /// Ask OS to reserve a free port
    fn reserve_port(&mut self) -> impl Future<Output = VetisResult<()>>;

    /// Reassign port to listener
    fn reassign_port(&mut self, port: u16);

    /// Returns listener config
    fn config(&self) -> &ListenerConfig;

    /// Starts the listener and begins accepting connections
    fn listen(&mut self) -> impl Future<Output = VetisResult<()>>;

    /// Stops the listener and closes all connections
    fn stop(self) -> impl Future<Output = VetisResult<()>>;
}

#[derive(Deserialize)]
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
pub struct ListenerConfigBuilder {
    workers: usize,
    port: u16,
    interface: IpAddr,
}

impl ListenerConfigBuilder {
    /// Sets the number of workers for this listener.
    /// Default value is 1.
    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    /// Sets the port number for the listener.
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
    pub fn interface(mut self, interface: impl Into<IpAddr>) -> Self {
        self.interface = interface.into();
        self
    }

    /// Creates the `ListenerConfig` with the configured settings.
    pub fn build(self) -> VetisResult<ListenerConfig> {
        Ok(ListenerConfig { workers: self.workers, port: self.port, interface: self.interface })
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
///     .interface(Ipv4Addr::UNSPECIFIED) // or (0, 0, 0, 0)
///     .build()
///     .unwrap();
///
/// println!("Listening on port {}", config.port());
/// ```
#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct ListenerConfig {
    workers: usize,
    port: u16,
    interface: IpAddr,
}

impl Default for ListenerConfig {
    fn default() -> Self {
        ListenerConfig { workers: 1, port: 80, interface: Ipv4Addr::UNSPECIFIED.into() }
    }
}

impl From<u16> for ListenerConfig {
    fn from(port: u16) -> Self {
        ListenerConfig { port, ..Default::default() }
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
    pub fn builder() -> ListenerConfigBuilder {
        ListenerConfigBuilder { workers: 1, port: 80, interface: Ipv4Addr::UNSPECIFIED.into() }
    }

    /// Returns mutable port number.
    pub fn reassign_port(&mut self, port: u16) {
        self.port = port;
    }

    /// Returns number of workers.
    pub fn workers(&self) -> usize {
        self.workers
    }

    /// Returns port number.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns network interface.
    pub fn interface(&self) -> &IpAddr {
        &self.interface
    }
}
