#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#[cfg(all(any(feature = "http2", feature = "http3"), not(feature = "rust-tls")))]
compile_error!("http2 and http3 requires rust-tls!");

use std::{collections::HashMap, sync::Arc};

use async_signal::{Signal, Signals};
use futures_lite::prelude::*;
use log::{error, info};
use signal_hook::low_level;
use vetis::errors::{VetisError, VirtualHostError};

use crate::{http::HttpServer, virtual_host::VirtualHost};
/// HTTP server module
pub mod http;
/// Listener module
pub mod listener;
/// Runtime module
mod rt;
/// Tests module
#[cfg(test)]
mod tests;
/// TLS module
mod tls;
/// Virtual host module
pub mod virtual_host;

pub use vetis::{
    errors,
    listener::ListenerConfig,
    virtual_host::{handler_fn, SecurityConfig, VirtualHostConfig},
    Protocol, Server, ServerConfig, VetisRwLock, VetisVirtualHosts,
};

/// Main server instance that manages virtual hosts and listeners.
///
/// The `Vetis` struct is the core of the VeTiS server. It handles:
/// - Managing multiple virtual hosts
/// - Coordinating server listeners
/// - Starting and stopping the server
/// - Signal handling for graceful shutdown
///
/// # Examples
///
/// ```rust,ignore
/// use vetis::{Vetis, config::ServerConfig};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = ServerConfig::builder().build();
///     let mut server = Vetis::new(config);
///     
///     // Add virtual hosts...
///     
///     server.run().await?;
///     Ok(())
/// }
/// ```
pub struct Vetis {
    config: ServerConfig,
    virtual_hosts: VetisVirtualHosts<VirtualHost>,
    instance: Option<http::HttpServer>,
}

impl Vetis {
    /// Creates a new `Vetis` server instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration containing listeners and global settings
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use vetis::{Vetis, config::ServerConfig};
    ///
    /// let config = ServerConfig::builder().build();
    /// let server = Vetis::new(config);
    /// ```
    pub fn new(config: ServerConfig) -> Vetis {
        Vetis { config, virtual_hosts: Arc::new(VetisRwLock::new(HashMap::new())), instance: None }
    }

    /// Adds a virtual host to the server.
    ///
    /// Virtual hosts allow you to host multiple domains on a single server instance.
    /// Each virtual host is identified by its hostname and port combination.
    ///
    /// # Arguments
    ///
    /// * `virtual_host` - A type implementing the `VirtualHost` trait
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use vetis::{
    ///     Vetis,
    ///     config::{ServerConfig, VirtualHostConfig},
    ///     server::virtual_host::{VirtualHost, handler_fn},
    /// };
    ///
    /// let config = ServerConfig::builder().build();
    /// let mut server = Vetis::new(config);
    ///
    /// let vhost_config = VirtualHostConfig::builder()
    ///     .hostname("example.com")
    ///     .port(80)
    ///     .build()?;
    ///
    /// let mut vhost = VirtualHost::new(vhost_config);
    ///
    /// let mut root_path = HandlerPath::new("/", handler_fn(|request| async move {
    ///     let response = vetis::Response::builder()
    ///         .status(StatusCode::OK)
    ///         .text("Hello, World!");
    ///     Ok(response)
    /// }));
    ///
    /// vhost.add_path(root_path);
    ///
    /// server.add_virtual_host(vhost).await;
    /// ```
    pub async fn add_virtual_host(&mut self, virtual_host: VirtualHost) {
        let key = (Arc::from(virtual_host.hostname()), virtual_host.port());

        self.virtual_hosts
            .write()
            .await
            .insert(key, virtual_host);
    }

    /// Returns a reference to the server configuration.
    ///
    /// This provides access to the listeners and global settings
    /// configured when the server was created.
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    /// Returns a reference to the virtual hosts.
    ///
    /// This provides access to the virtual hosts configured when the server was created.
    pub fn virtual_hosts(&self) -> &VetisVirtualHosts<VirtualHost> {
        &self.virtual_hosts
    }

    /// Starts the server and runs until interrupted.
    ///
    /// This method combines `start()` and graceful shutdown handling:
    /// 1. Starts the server with all configured virtual hosts
    /// 2. Listens for shutdown signals (Ctrl+C on Tokio, SIGQUIT on Smol)
    /// 3. Stops the server gracefully
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No virtual hosts have been added
    /// - Server fails to start
    /// - Server fails to stop
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use vetis::{Vetis, config::ServerConfig};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder().build();
    ///     let mut server = Vetis::new(config);
    ///     
    ///     // Add virtual hosts...
    ///     
    ///     server.run().await?; // Runs until Ctrl+C
    ///     Ok(())
    /// }
    /// ```
    pub async fn run(&mut self) -> Result<(), VetisError> {
        self.start().await?;

        for listener in self
            .config
            .listeners()
        {
            info!("Server listening on port {}:{}", listener.interface(), listener.port());
        }

        let mut signals = Signals::new([Signal::Quit]).unwrap();
        while let Some(signal) = signals.next().await {
            low_level::emulate_default_handler(signal.unwrap() as i32).unwrap();
        }

        info!("\nStopping server...");

        self.stop().await?;

        Ok(())
    }

    /// Starts the server without blocking.
    ///
    /// This method starts the server and returns immediately, allowing
    /// you to perform additional setup or handle shutdown manually.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No virtual hosts have been added
    /// - Server fails to bind to configured addresses
    /// - TLS configuration fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use vetis::{Vetis, config::ServerConfig};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder().build();
    ///     let mut server = Vetis::new(config);
    ///     
    ///     // Add virtual hosts...
    ///     
    ///     server.start().await?;
    ///     
    ///     // Server is now running, do other work...
    ///     
    ///     server.stop().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn start(&mut self) -> Result<(), VetisError> {
        if self
            .virtual_hosts
            .read()
            .await
            .is_empty()
        {
            error!("You must add at least one virtual host");
            return Err(VetisError::VirtualHost(VirtualHostError::NoVirtualHosts));
        }

        let mut server = HttpServer::new(self.config.clone());

        server.set_virtual_hosts(
            self.virtual_hosts
                .clone(),
        );

        server
            .start()
            .await?;
        self.instance = Some(server);

        Ok(())
    }

    /// Stops the server gracefully.
    ///
    /// This method shuts down all listeners and waits for ongoing
    /// requests to complete before returning.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No server instance is running
    /// - Server fails to stop properly
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use vetis::{Vetis, config::ServerConfig};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder().build();
    ///     let mut server = Vetis::new(config);
    ///     
    ///     server.start().await?;
    ///     // Server running...
    ///     server.stop().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn stop(&mut self) -> Result<(), VetisError> {
        if let Some(instance) = &mut self.instance {
            instance
                .stop()
                .await?;
        } else {
            return Err(VetisError::NoInstances);
        }
        Ok(())
    }
}
