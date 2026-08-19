use crate::{host::HostImpl, listener::ServerListener};
use async_signal::{Signal, Signals};
use futures_lite::prelude::*;
#[cfg(any(feature = "http1", feature = "http2"))]
use http::Version;
use hyper::rt::Executor;
use log::{error, info};
use signal_hook::low_level;
use std::{collections::HashMap, sync::Arc};
use vetis::{
    base,
    errors::{HostError, ListenerError, VetisError},
    host::{Host, HostConfig},
    listener::Listener as _,
    server::ServerConfig,
    VetisHosts, VetisResult, VetisRwLock,
};

#[derive(Default)]
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
/// ```rust,no_run
/// use macro_rules_attribute::apply;
/// use smol_macros::main;
/// use vetis::{server::ServerConfig, VetisServer as _};
/// use vetis_smol::Vetis;
///
/// #[apply(main!)]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = ServerConfig::builder().build()?;
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
    hosts: VetisHosts<HostImpl>,
    listeners: Vec<ServerListener>,
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
    /// ```rust,no_run
    /// use vetis::server::ServerConfig;
    /// use vetis_smol::Vetis;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder().build()?;
    ///     let server = Vetis::new(config);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(config: ServerConfig) -> Vetis {
        Vetis { config, hosts: Arc::new(VetisRwLock::new(HashMap::new())), listeners: Vec::new() }
    }
}

impl base::VetisServer for Vetis {
    /// Virtual host type
    type Host = HostImpl;
    /// Virtual host configuration type
    type HostConfig = HostConfig;

    /// Adds a virtual host to the server.
    ///
    /// Virtual hosts allow you to host multiple domains on a single server instance.
    /// Each virtual host is identified by its hostname and port combination.
    ///
    /// # Arguments
    ///
    /// * `host` - A type implementing the `Host` trait
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use http::StatusCode;
    ///
    /// use macro_rules_attribute::apply;
    /// use smol_macros::main;
    ///
    /// use vetis::{
    ///     server::ServerConfig,
    ///     host::{path::Path, handler_fn, HostConfig},
    ///     VetisServer as _
    /// };
    ///
    /// use vetis_smol::{Vetis, host::{HostImpl, path::HandlerPath}};
    ///
    /// #[apply(main!)]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder().build()?;
    ///     let mut server = Vetis::new(config);
    ///
    ///     let vhost_config = HostConfig::builder()
    ///         .hostname("example.com")
    ///         .port(80)
    ///         .build()?;
    ///
    ///     let mut vhost = HostImpl::new(vhost_config);
    ///
    ///     let mut root_path = HandlerPath::builder()
    ///         .uri("/")
    ///         .handler(handler_fn(|request| async move {
    ///             let response = vetis::Response::builder()
    ///                 .status(StatusCode::OK)
    ///                 .text("Hello, World!");
    ///             Ok(response)
    ///         }))
    ///         .build()?;
    ///
    ///     vhost.add_path(root_path);
    ///
    ///     server.add_host(vhost).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn add_host(&mut self, host: Self::Host) {
        self.hosts
            .write()
            .await
            .insert(Arc::from(host.hostname()), host);
    }

    /// Remove a virtual host from the server
    ///
    /// # Arguments
    ///
    /// * `hostname` - The hostname of the virtual host to remove
    /// * `port` - The port of the virtual host to remove
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use macro_rules_attribute::apply;
    /// use smol_macros::main;
    /// use vetis::{server::ServerConfig, VetisServer as _};
    /// use vetis_smol::Vetis;
    ///
    /// #[apply(main!)]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = vetis::server::ServerConfig::builder().build()?;
    ///     let mut server = Vetis::new(config);
    ///
    ///     server.remove_host("example.com", 80).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn remove_host(&mut self, hostname: &str) {
        self.hosts
            .write()
            .await
            .remove(&Arc::from(hostname));
    }

    /// Returns a reference to the virtual hosts.
    ///
    /// This provides access to the virtual hosts configured when the server was created.
    fn hosts(&self) -> &VetisHosts<Self::Host> {
        &self.hosts
    }

    /// Returns a reference to the server configuration.
    ///
    /// This provides access to the listeners and global settings
    /// configured when the server was created.
    fn config(&self) -> &ServerConfig {
        &self.config
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
    /// ```rust,no_run
    /// use macro_rules_attribute::apply;
    /// use smol_macros::main;
    /// use vetis::{server::ServerConfig, VetisServer as _};
    /// use vetis_smol::Vetis;
    ///
    /// #[apply(main!)]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder().build()?;
    ///     let mut server = Vetis::new(config);
    ///
    ///     // Add virtual hosts...
    ///
    ///     server.run().await?; // Runs until Ctrl+C
    ///     Ok(())
    /// }
    /// ```
    async fn run(&mut self) -> VetisResult<()> {
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
    /// ```rust,no_run
    /// use macro_rules_attribute::apply;
    /// use smol_macros::main;
    /// use vetis::{server::ServerConfig, VetisServer as _};
    /// use vetis_smol::Vetis;
    ///
    /// #[apply(main!)]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder().build()?;
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
    async fn start(&mut self) -> VetisResult<()> {
        if self
            .config
            .listeners()
            .is_empty()
        {
            error!("You must add at least one listener");
            return Err(VetisError::Listener(ListenerError::NoListeners));
        }

        if self
            .hosts
            .read()
            .await
            .is_empty()
        {
            error!("You must add at least one virtual host");
            return Err(VetisError::Host(HostError::NoHosts));
        }

        for listener_config in self
            .config
            .listeners()
        {
            #[cfg(any(feature = "http1", feature = "http2"))]
            if listener_config
                .protos()
                .iter()
                .any(|proto| *proto == Version::HTTP_11 || *proto == Version::HTTP_2)
            {
                use crate::listener::tcp::TcpListener;
                use vetis::listener::Listener as _;
                let mut listener: ServerListener = TcpListener::new(listener_config.clone()).into();
                listener.set_hosts(self.hosts.clone());
                listener
                    .listen()
                    .await?;
                self.listeners
                    .push(listener);
            }

            #[cfg(feature = "http3")]
            if listener_config
                .protos()
                .contains(&Version::HTTP_3)
            {
                use crate::listener::udp::UdpListener;
                use vetis::listener::Listener as _;
                let mut listener: ServerListener = UdpListener::new(listener_config.clone()).into();
                listener.set_hosts(self.hosts.clone());
                listener
                    .listen()
                    .await?;
                self.listeners
                    .push(listener);
            }
        }

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
    /// ```rust,no_run
    /// use macro_rules_attribute::apply;
    /// use smol_macros::main;
    /// use vetis::{server::ServerConfig, VetisServer as _};
    /// use vetis_smol::Vetis;
    ///
    /// #[apply(main!)]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder().build()?;
    ///     let mut server = Vetis::new(config);
    ///
    ///     server.start().await?;
    ///     // Server running...
    ///     server.stop().await?;
    ///     Ok(())
    /// }
    /// ```
    async fn stop(&mut self) -> VetisResult<()> {
        if self
            .listeners
            .is_empty()
        {
            return Err(VetisError::Stop("Vetis is not running".to_string()));
        }

        for listener in &mut self.listeners {
            listener
                .stop()
                .await?
        }
        Ok(())
    }

    /// Reload the server configuration
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use macro_rules_attribute::apply;
    /// use smol_macros::main;
    /// use vetis::VetisServer as _;
    /// use vetis_smol::Vetis;
    ///
    /// #[apply(main!)]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = vetis::server::ServerConfig::builder().build()?;
    ///     let mut server = Vetis::new(config);
    ///
    ///     let new_config = vetis::server::ServerConfig::builder().build()?;
    ///     let new_hosts = vec![];
    ///     server.reload(new_config, new_hosts).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn reload(
        &mut self,
        _new_config: ServerConfig,
        _new_hosts: Vec<Self::HostConfig>,
    ) -> VetisResult<()> {
        Ok(())
    }
}

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
/// Executor for smol runtime
pub struct SmolExecutor {}

impl<Fut> Executor<Fut> for SmolExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        smol::spawn(fut).detach();
    }
}

impl SmolExecutor {
    /// Creates a new `SmolExecutor`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis_smol::rt::SmolExecutor;
    ///
    /// let executor = SmolExecutor::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}
