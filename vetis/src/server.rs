use std::sync::Arc;

use crate::{errors::ConfigError, host::HostConfig, log::LogConfig};
use serde::Deserialize;

/// Builder for creating `ServerConfig` instances.
///
/// Provides a fluent API for configuring the overall server,
/// including multiple hosts for different ports and protocols.
///
/// # Examples
///
/// ```rust,ignore
/// use vetis::{server::ServerConfig};
/// use http::Version;
///
/// let config = ServerConfig::builder()
///     .host(some_host)?
///     .build();
/// ```
pub struct ServerConfigBuilder {
    hosts: Vec<HostConfig>,
    logger_queue_size: usize,
    log: Option<Box<dyn LogConfig>>,
    workers: usize,
}

impl ServerConfigBuilder {
    /// Set logger queue size
    ///
    /// # Arguments
    ///
    /// - `size` - The size of logger queue
    pub fn logger_queue_size(mut self, size: usize) -> Self {
        self.logger_queue_size = size;
        self
    }

    /// Log instance
    ///
    /// # Arguments
    ///
    /// - `log` - Enable log if set to true
    pub fn log<L>(mut self, log: L) -> Self
    where
        L: LogConfig + 'static,
    {
        self.log = Some(Box::new(log));
        self
    }

    /// Adds a host configuration to the server.
    ///
    /// Multiple host can be added to serve different domains.
    /// It is not mandatory add a host here, you can still add
    /// hosts while building Vetis using its builder.
    pub fn add_host(mut self, host: HostConfig) -> Self {
        self.hosts
            .push(host);
        self
    }

    /// Set number of workers to spawn
    ///
    /// # Arguments
    ///
    /// - `number` - The number of workers to spawn
    pub fn workers(mut self, number: usize) -> Self {
        self.workers = number;
        self
    }

    /// Creates the `ServerConfig` with the configured listeners.
    ///
    /// # Errors
    ///
    /// * Return an error if no listeners are configured or if HTTP/2 and HTTP/3 support enabled and no security setting provided for the host.
    pub fn build(self) -> Result<ServerConfig, ConfigError> {
        if self.workers < 1 {
            return Err(ConfigError::Server(
                "You must have at least one worker running.".to_string(),
            ));
        }

        if self.log.is_some() && self.logger_queue_size < 100 {
            return Err(ConfigError::Server("Log queue size too small".to_string()));
        }

        Ok(ServerConfig {
            hosts: self.hosts.into(),
            logger_queue_size: self.logger_queue_size,
            log: self.log,
            workers: self.workers,
        })
    }
}

/// Global server configuration.
///
/// Contains all the hosts that the server should use to accept
/// incoming connections. Each host can have different settings
/// for port, protocol, and interface.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{host::HostConfig, server::ServerConfig};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = ServerConfig::builder()
///         .add_host(HostConfig::builder().hostname("example.com").build()?)
///         .add_host(HostConfig::builder().hostname("sample.com").build()?)
///         .build()?;
///
///     println!("Server has {} hosts", config.hosts().len());
///     Ok(())
/// }
/// ```
#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct ServerConfig {
    hosts: Arc<[HostConfig]>,
    log: Option<Box<dyn LogConfig>>,
    logger_queue_size: usize,
    workers: usize, // Add file rolling support alongside stdout for logging
}

impl ServerConfig {
    /// Creates a new `ServerConfigBuilder` with no hosts.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{host::HostConfig, server::ServerConfig};
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let host_config = HostConfig::builder().hostname("example.com").build()?;
    ///     let server_config = ServerConfig::builder()
    ///         .add_host(host_config)
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn builder() -> ServerConfigBuilder {
        ServerConfigBuilder { hosts: vec![], logger_queue_size: 2000, log: None, workers: 1 }
    }

    /// Returns a reference to all configured hosts.
    pub fn hosts(&self) -> &Arc<[HostConfig]> {
        &self.hosts
    }

    /// Returns log
    pub fn log(&self) -> &Option<Box<dyn LogConfig>> {
        &self.log
    }

    /// Returns size of logger queue
    pub fn logger_queue_size(&self) -> usize {
        self.logger_queue_size
    }

    /// Returns number of workers
    pub fn workers(&self) -> usize {
        self.workers
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { hosts: [].into(), logger_queue_size: 20000, log: None, workers: 1 }
    }
}
