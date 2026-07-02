use crate::{http::HttpServer, virtual_host::VirtualHostImpl};
use log::{error, info};
use std::{collections::HashMap, sync::Arc};
use vetis::{
    errors::{ListenerError, StartError, VetisError, VirtualHostError},
    server::{Server, ServerConfig},
    virtual_host::{VirtualHost, VirtualHostConfig},
    VetisResult, VetisRwLock, VetisVirtualHosts,
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
/// use vetis::{server::ServerConfig, Vetis as _};
/// use vetis_tokio::Vetis;
///
/// #[tokio::main]
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
    virtual_hosts: VetisVirtualHosts<VirtualHostImpl>,
    instance: Option<HttpServer>,
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
    /// ```rust, no_run
    /// use vetis::server::ServerConfig;
    /// use vetis_tokio::Vetis;
    ///
    /// let config = ServerConfig::builder().build()?;
    /// let server = Vetis::new(config);
    ///
    /// Ok::<(), vetis::errors::VetisError>(())
    /// ```
    pub fn new(config: ServerConfig) -> Vetis {
        Vetis { config, virtual_hosts: Arc::new(VetisRwLock::new(HashMap::new())), instance: None }
    }
}

impl vetis::Vetis for Vetis {
    /// Virtual host type
    type VirtualHost = VirtualHostImpl;
    /// Virtual host configuration type
    type VirtualHostConfig = VirtualHostConfig;

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
    /// ```rust,no_run
    /// use http::StatusCode;
    ///
    /// use vetis::{
    ///     server::ServerConfig,
    ///     virtual_host::{path::Path, handler_fn, VirtualHostConfig},
    ///     Vetis as _
    /// };
    ///
    /// use vetis_tokio::{Vetis, virtual_host::{VirtualHostImpl, path::HandlerPath}};
    ///
    /// let config = ServerConfig::builder().build()?;
    /// let mut server = Vetis::new(config);
    ///
    /// let vhost_config = VirtualHostConfig::builder()
    ///     .hostname("example.com")
    ///     .port(80)
    ///     .build()?;
    ///
    /// let mut vhost = VirtualHostImpl::new(vhost_config);
    ///
    /// let mut root_path = HandlerPath::builder()
    ///     .uri("/")
    ///     .handler(handler_fn(|request| async move {
    ///         let response = vetis::Response::builder()
    ///             .status(StatusCode::OK)
    ///             .text("Hello, World!");
    ///         Ok(response)
    ///     }))
    ///     .build()?;
    ///
    /// vhost.add_path(root_path);
    ///
    /// async move {
    ///     server.add_virtual_host(vhost).await;
    /// };
    ///
    /// Ok::<(), vetis::errors::VetisError>(())
    /// ```
    async fn add_virtual_host(&mut self, virtual_host: Self::VirtualHost) {
        let key = (Arc::from(virtual_host.hostname()), virtual_host.port());

        self.virtual_hosts
            .write()
            .await
            .insert(key, virtual_host);
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
    /// use vetis::Vetis as _;
    /// use vetis_tokio::Vetis;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = vetis::server::ServerConfig::builder().build()?;
    ///     let mut server = Vetis::new(config);
    ///
    ///     server.remove_virtual_host("example.com", 80).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn remove_virtual_host(&mut self, hostname: &str, port: u16) {
        let key = (Arc::from(hostname), port);

        self.virtual_hosts
            .write()
            .await
            .remove(&key);
    }

    /// Returns a reference to the virtual hosts.
    ///
    /// This provides access to the virtual hosts configured when the server was created.
    fn virtual_hosts(&self) -> &VetisVirtualHosts<Self::VirtualHost> {
        &self.virtual_hosts
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
    /// use vetis::{server::ServerConfig, Vetis as _};
    /// use vetis_tokio::Vetis;
    ///
    /// #[tokio::main]
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

        let _ = tokio::signal::ctrl_c().await;

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
    /// use vetis::{server::ServerConfig, Vetis as _};
    /// use vetis_tokio::Vetis;
    ///
    /// #[tokio::main]
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
            .instance
            .is_some()
        {
            error!("Server is already running");
            return Err(VetisError::Start(StartError::AlreadyRunning));
        }

        if self
            .config
            .listeners()
            .is_empty()
        {
            error!("You must add at least one listener");
            return Err(VetisError::Listener(ListenerError::NoListeners));
        }

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
    /// ```rust,no_run
    /// use vetis::{server::ServerConfig, Vetis as _};
    /// use vetis_tokio::Vetis;
    ///
    /// #[tokio::main]
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
        if let Some(instance) = &mut self.instance {
            instance
                .stop()
                .await?;
        } else {
            return Err(VetisError::NoInstances);
        }
        Ok(())
    }

    /// Reload the server configuration
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Vetis as _;
    /// use vetis_tokio::Vetis;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = vetis::server::ServerConfig::builder().build()?;
    ///     let mut server = Vetis::new(config);
    ///
    ///     let changed_config = vetis::server::ServerConfig::builder().build()?;
    ///     server.reload(changed_config, vec![]).await;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn reload(
        &mut self,
        new_config: ServerConfig,
        new_virtual_hosts: Vec<Self::VirtualHostConfig>,
    ) -> VetisResult<()> {
        if self.config() != &new_config {
            self.config = new_config;
            self.stop().await?;
            self.start().await?;
        }
        Ok(())
    }
}
