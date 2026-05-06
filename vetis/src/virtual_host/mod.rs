use std::{collections::HashMap, future::Future, path::Path, pin::Pin, sync::Arc};

use radix_trie::Trie;
use serde::Deserialize;

use crate::{
    errors::{ConfigError, FileError, VetisError, VirtualHostError},
    security::SecurityConfig,
    virtual_host::path::PathConfig,
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
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + Sync>>
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
    paths: Option<Vec<Box<dyn path::PathConfig>>>,
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
            paths: self.paths,
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
///     .build()
///     .unwrap();
///
/// println!("Virtual host: {}:{}", config.hostname(), config.port());
/// ```
#[derive(Deserialize)]
pub struct VirtualHostConfig {
    hostname: String,
    port: u16,
    root_directory: String,
    default_headers: Option<Vec<(String, String)>>,
    #[serde(deserialize_with = "crate::security::deserialize_security_from_file")]
    security: Option<SecurityConfig>,
    status_pages: Option<HashMap<u16, String>>,
    enable_logging: bool,
    paths: Option<Vec<Box<dyn path::PathConfig>>>,
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
    /// use vetis::virtual_host::VirtualHostConfig;
    ///
    /// let config = VirtualHostConfig::builder()
    ///     .hostname("example.com")
    ///     .port(443)
    ///     .build()
    ///     .unwrap();
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
            paths: None,
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

    /// Returns the paths.
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<Box<dyn PathConfig>>>` - The static paths.
    pub fn paths(&self) -> &Option<Vec<Box<dyn PathConfig>>> {
        &self.paths
    }
}

/// Virtual host trait
pub trait VirtualHost {
    /// Returns the paths trie
    ///
    /// # Returns
    ///
    /// * `Trie<String, Box<dyn path::Path>>` - The paths trie.
    fn paths(&self) -> Trie<String, Arc<Box<dyn path::Path>>>;

    /// Returns virtual host configuration
    ///
    /// # Returns
    ///
    /// * `&VirtualHostConfig` - A reference to the virtual host configuration.
    fn config(&self) -> &VirtualHostConfig;

    /// Returns virtual host hostname
    ///
    /// # Returns
    ///
    /// * `&str` - A reference to the virtual host hostname.
    fn hostname(&self) -> &str {
        self.config()
            .hostname()
    }

    /// Returns virtual host port number
    ///
    /// # Returns
    ///
    /// * `u16` - The virtual host port number.
    fn port(&self) -> u16 {
        self.config().port()
    }

    /// Returns virtual host security configuration
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the virtual host is secure or not.
    fn is_secure(&self) -> bool {
        self.config()
            .security()
            .is_some()
    }

    /// Serve a status page
    ///
    /// # Arguments
    ///
    /// * `status` - The status code to serve.
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + Sync + '_>>` - A pinned box containing the future that will resolve to a `Result<Response, VetisError>`.
    fn serve_status_page(
        &self,
        status: u16,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + Sync + '_>>;

    /// Route request to the appropriate handler
    ///
    /// # Arguments
    ///
    /// * `request` - A `Request` instance containing the request information.
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + Sync + '_>>` - A pinned box containing the future that will resolve to a `Result<Response, VetisError>`.
    fn route(
        &self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + Sync + '_>>
    where
        Self: Sync,
    {
        let uri_path: String = request
            .uri()
            .path()
            .into();

        if uri_path.starts_with("..") {
            return Box::pin(async move {
                self.serve_status_page(http::StatusCode::FORBIDDEN.as_u16())
                    .await
            });
        }

        let paths = self.paths();

        let matches = paths.get_ancestor_value(&uri_path);

        let Some(path) = matches else {
            return Box::pin(async move {
                self.serve_status_page(http::StatusCode::NOT_FOUND.as_u16())
                    .await
            });
        };

        let path = path.clone();

        let target_path: String = uri_path
            .strip_prefix(path.uri())
            .unwrap_or(&uri_path)
            .into();

        Box::pin(async move {
            let result = path.handle(request, Arc::from(target_path));
            match result.await {
                Ok(response) => Ok(response),
                Err(error) => {
                    match error {
                        VetisError::VirtualHost(VirtualHostError::File(FileError::NotFound)) => {
                            log::error!("Invalid path: {}", error);
                            return self
                                .serve_status_page(http::StatusCode::NOT_FOUND.as_u16())
                                .await;
                        }
                        VetisError::VirtualHost(VirtualHostError::Proxy(ref error)) => {
                            log::error!("Proxy error: {}", error);
                            return self
                                .serve_status_page(http::StatusCode::BAD_GATEWAY.as_u16())
                                .await;
                        }
                        VetisError::VirtualHost(VirtualHostError::Auth(e)) => {
                            log::error!("Auth error: {}", e);
                            return self
                                .serve_status_page(http::StatusCode::UNAUTHORIZED.as_u16())
                                .await;
                        }
                        _ => {}
                    }

                    Err(error)
                }
            }
        })
    }
}
