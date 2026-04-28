use std::{collections::HashMap, future::Future, path::Path, pin::Pin};

use serde::Deserialize;

#[cfg(feature = "interface")]
use crate::virtual_host::path::interface::InterfacePathConfig;
#[cfg(feature = "reverse-proxy")]
use crate::virtual_host::path::proxy::ProxyPathConfig;
#[cfg(feature = "static-files")]
use crate::virtual_host::path::static_files::StaticPathConfig;

use crate::{
    errors::{ConfigError, VetisError},
    security::SecurityConfig,
    Request, Response,
};

/// Path configuration for virtual hosts.
pub mod path;

/// Type alias for boxed handler closures.
///
/// This represents an async function that takes a `Request` and returns
/// a `Response` or an error. Handlers are the core of request processing
/// in VeTiS virtual hosts.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::virtual_host::BoxedHandlerClosure;
/// use vetis::{Request, Response, errors::VetisError};
///
/// let handler: BoxedHandlerClosure = Box::new(|request: Request| {
///     Box::pin(async move {
///         // Process request...
///         Ok(Response::builder()
///             .status(http::StatusCode::OK)
///             .body(http_body_util::Full::new(bytes::Bytes::from("OK"))))
///     })
/// });
/// ```
pub type BoxedHandlerClosure = Box<
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send>>
        + Send
        + Sync,
>;

/// Creates a handler closure from a function.
///
/// This utility function converts any compatible async function into a
/// `BoxedHandlerClosure` that can be used with virtual hosts.
///
/// # Arguments
///
/// * `f` - An async function that takes a `Request` and returns a `Result<Response, VetisError>`
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{
///     virtual_host::{handler_fn, VirtualHostConfig},
/// };
///
/// let config = VirtualHostConfig::builder()
///     .hostname("example.com")
///     .port(80)
///     .build()
///     .unwrap();
///
/// assert_eq!(80, config.port());
/// ```
pub fn handler_fn<F, Fut>(f: F) -> BoxedHandlerClosure
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response, VetisError>> + Send + Sync + 'static,
{
    Box::new(move |req| Box::pin(f(req)))
}

/// Builder for creating `VirtualHostConfig` instances.
///
/// Provides a fluent API for configuring virtual hosts,
/// including hostname, port, and security settings.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{
///     security::SecurityConfig,
///     virtual_host::VirtualHostConfig
/// };
///
/// let security = SecurityConfig::builder()
///     .cert_from_bytes(vec![])
///     .key_from_bytes(vec![])
///     .build()
///     .unwrap();
///
/// let config = VirtualHostConfig::builder()
///     .hostname("example.com")
///     .port(443)
///     .security(security)
///     .build()
///     .unwrap();
/// ```
pub struct VirtualHostConfigBuilder {
    hostname: String,
    port: u16,
    root_directory: String,
    default_headers: Option<Vec<(String, String)>>,
    security: Option<SecurityConfig>,
    status_pages: Option<HashMap<u16, String>>,
    enable_logging: bool,
    #[cfg(feature = "static-files")]
    static_paths: Option<Vec<StaticPathConfig>>,
    #[cfg(feature = "reverse-proxy")]
    proxy_paths: Option<Vec<ProxyPathConfig>>,
    #[cfg(feature = "interface")]
    interface_paths: Option<Vec<InterfacePathConfig>>,
}

impl VirtualHostConfigBuilder {
    /// Sets the hostname for the virtual host.
    ///
    /// This is used to match incoming requests to the correct virtual host.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .hostname("api.example.com")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn hostname(mut self, hostname: &str) -> Self {
        self.hostname = hostname.to_string();
        self
    }

    /// Sets the port for the virtual host.
    ///
    /// This should match one of the ports configured in the server listeners.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .port(8443)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the root directory for the virtual host.
    ///
    /// This is the base directory for all static file paths.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .root_directory("/var/www")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn root_directory(mut self, root_directory: &str) -> Self {
        self.root_directory = root_directory.to_string();
        self
    }

    /// Adds a default header to the virtual host.
    ///
    /// These headers will be added to all responses from this virtual host.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .header("X-Custom", "value")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn header(mut self, key: &str, value: &str) -> Self {
        match self.default_headers {
            None => {
                let vec = vec![(key.to_string(), value.to_string())];
                self.default_headers = Some(vec);
            }
            Some(ref mut headers) => {
                headers.push((key.to_string(), value.to_string()));
            }
        }
        self
    }

    /// Sets the security configuration for HTTPS.
    ///
    /// When provided, the virtual host will use TLS for secure connections.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{
    ///     security::SecurityConfig,
    ///     virtual_host::VirtualHostConfig,
    /// };
    ///
    /// let security = SecurityConfig::builder()
    ///     .cert_from_bytes(vec![])
    ///     .key_from_bytes(vec![])
    ///     .build()
    ///     .unwrap();
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .security(security)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn security(mut self, security: SecurityConfig) -> Self {
        self.security = Some(security);
        self
    }

    /// Sets the status pages for the virtual host.
    ///
    /// These status pages will be used to serve custom error pages.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .status_pages(vec![(404, "404.html".to_string())])
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn status_pages(mut self, status_pages: HashMap<u16, String>) -> Self {
        self.status_pages = Some(status_pages);
        self
    }

    /// Enables or disables logging for this virtual host.
    ///
    /// When enabled, all requests to this virtual host will be logged.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .enable_logging(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn enable_logging(mut self, logging: bool) -> Self {
        self.enable_logging = logging;
        self
    }

    #[cfg(feature = "static-files")]
    /// Sets the status pages for the virtual host.
    ///
    /// These status pages will be used to serve custom error pages.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .static_paths(vec![(404, "404.html".to_string())])
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn static_paths(mut self, static_paths: Vec<StaticPathConfig>) -> Self {
        self.static_paths = Some(static_paths);
        self
    }

    #[cfg(feature = "reverse-proxy")]
    /// Sets the reverse proxy paths for the virtual host.
    ///
    /// These reverse proxy paths will be used to serve custom error pages.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .proxy_paths(vec![(404, "404.html".to_string())])
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn proxy_paths(mut self, proxy_paths: Vec<ProxyPathConfig>) -> Self {
        self.proxy_paths = Some(proxy_paths);
        self
    }

    #[cfg(feature = "interface")]
    /// Sets the interface paths for the virtual host.
    ///
    /// These interface paths will be used to serve custom error pages.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .proxy_paths(vec![(404, "404.html".to_string())])
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn interface_paths(mut self, interface_paths: Vec<InterfacePathConfig>) -> Self {
        self.interface_paths = Some(interface_paths);
        self
    }

    /// Creates the `VirtualHostConfig` with the configured settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the hostname is empty.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .hostname("example.com")
    ///     .port(443)
    ///     .header("X-Custom", "value")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> Result<VirtualHostConfig, VetisError> {
        if self
            .hostname
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::VirtualHost(
                "Missing hostname".to_string(),
            )));
        }

        if self
            .root_directory
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::VirtualHost(
                "Missing root directory".to_string(),
            )));
        } else {
            let root_path = Path::new(&self.root_directory);
            if !root_path.exists() {
                return Err(VetisError::Config(ConfigError::VirtualHost(format!(
                    "root_directory does not exist: {}",
                    self.root_directory
                ))));
            }
        }

        Ok(VirtualHostConfig {
            hostname: self.hostname,
            port: self.port,
            root_directory: self.root_directory,
            default_headers: self.default_headers,
            security: self.security,
            status_pages: self.status_pages,
            enable_logging: self.enable_logging,
            #[cfg(feature = "static-files")]
            static_paths: self.static_paths,
            #[cfg(feature = "reverse-proxy")]
            proxy_paths: self.proxy_paths,
            #[cfg(feature = "interface")]
            interface_paths: self.interface_paths,
        })
    }
}

/// Configuration for a virtual host.
///
/// Defines how a specific hostname should be handled, including
/// the port it listens on and optional security settings for HTTPS.
///
/// Virtual hosts allow multiple domains to be served by the same
/// server instance, each with its own configuration and handlers.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::virtual_host::VirtualHostConfig;
///
/// let config = VirtualHostConfig::builder()
///     .hostname("api.example.com")
///     .port(443)
///     .build()?;
///
/// println!("Virtual host: {}:{}", config.hostname(), config.port());
/// ```
#[derive(Clone, Deserialize)]
pub struct VirtualHostConfig {
    hostname: String,
    port: u16,
    root_directory: String,
    default_headers: Option<Vec<(String, String)>>,
    #[serde(deserialize_with = "crate::security::deserialize_security_from_file")]
    security: Option<SecurityConfig>,
    status_pages: Option<HashMap<u16, String>>,
    enable_logging: bool,
    #[cfg(feature = "static-files")]
    static_paths: Option<Vec<StaticPathConfig>>,
    #[cfg(feature = "reverse-proxy")]
    proxy_paths: Option<Vec<ProxyPathConfig>>,
    #[cfg(feature = "interface")]
    interface_paths: Option<Vec<InterfacePathConfig>>,
}

impl VirtualHostConfig {
    /// Creates a new `VirtualHostConfigBuilder` with default settings.
    ///
    /// Default values:
    /// - hostname: empty string (must be set)
    /// - port: 80
    /// - security: None
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .hostname("example.com")
    ///     .port(443)
    ///     .build()?;
    /// ```
    pub fn builder() -> VirtualHostConfigBuilder {
        VirtualHostConfigBuilder {
            hostname: "localhost".to_string(),
            port: 80,
            root_directory: "/var/vetis/www".to_string(),
            default_headers: None,
            security: None,
            status_pages: None,
            enable_logging: true,
            #[cfg(feature = "static-files")]
            static_paths: None,
            #[cfg(feature = "reverse-proxy")]
            proxy_paths: None,
            #[cfg(feature = "interface")]
            interface_paths: None,
        }
    }

    /// Returns the hostname.
    ///
    /// # Returns
    ///
    /// * `&str` - The hostname.
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    /// Returns the port.
    ///
    /// # Returns
    ///
    /// * `u16` - The port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the root directory.
    ///
    /// # Returns
    ///
    /// * `&str` - The root directory.
    pub fn root_directory(&self) -> &str {
        &self.root_directory
    }

    /// Returns the default headers.
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<(String, String)>>` - The default headers.
    pub fn default_headers(&self) -> &Option<Vec<(String, String)>> {
        &self.default_headers
    }

    /// Returns the security configuration if present.
    ///
    /// # Returns
    ///
    /// * `&Option<SecurityConfig>` - The security configuration if present.
    pub fn security(&self) -> &Option<SecurityConfig> {
        &self.security
    }

    /// Returns the status pages.
    ///
    /// # Returns
    ///
    /// * `&Option<HashMap<u16, String>>` - The status pages.
    pub fn status_pages(&self) -> &Option<HashMap<u16, String>> {
        &self.status_pages
    }

    /// Returns the logging setting.
    ///
    /// # Returns
    ///
    /// * `bool` - The logging setting.
    pub fn enable_logging(&self) -> bool {
        self.enable_logging
    }

    #[cfg(feature = "static-files")]
    /// Returns the static paths.
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<StaticPathConfig>>` - The static paths.
    pub fn static_paths(&self) -> &Option<Vec<StaticPathConfig>> {
        &self.static_paths
    }

    #[cfg(feature = "reverse-proxy")]
    /// Returns the proxy paths.
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<ProxyPathConfig>>` - The proxy paths.
    pub fn proxy_paths(&self) -> &Option<Vec<ProxyPathConfig>> {
        &self.proxy_paths
    }

    #[cfg(feature = "interface")]
    /// Returns the interface paths.
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<InterfacePathConfig>>` - The interface paths.
    pub fn interface_paths(&self) -> &Option<Vec<InterfacePathConfig>> {
        &self.interface_paths
    }
}
