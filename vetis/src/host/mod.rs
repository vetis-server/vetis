use crate::{
    errors::{ConfigError, VetisError},
    host::path::PathConfig,
    security::SecurityConfig,
    HandlerFn, Request, Response, VetisFutureResult, VetisResult,
};
use http::{uri::Authority, Version};
use radix_trie::Trie;
use serde::Deserialize;
use std::{
    collections::HashMap, future::Future, net::IpAddr, path::PathBuf, sync::Arc, time::Duration,
};

/// Path configuration for hosts.
pub mod path;

/// Creates a handler function from a function.
///
/// This utility function converts any compatible async function into a
/// `HandlerFn` that can be used with hosts.
///
/// # Arguments
///
/// * `f` - An async function that takes a `Request` and returns a `VetisResult<Response>`
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{
///     host::{handler_fn, HostConfig},
/// };
///
/// let config = HostConfig::builder()
///     .hostname("example.com")
///     .build()
///     .unwrap();
///
/// assert_eq!("example.com", config.hostname());
/// ```
pub fn handler_fn<F, Fut>(f: F) -> HandlerFn
where
    F: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = VetisResult<Response>> + Send + Sync + 'static,
{
    Box::new(move |req| Box::pin(f(req)))
}

/// AltService builder type
pub struct AltServiceBuilder {
    authority: Option<Authority>,
    protocol: Version,
    port: u16,
    ma: Duration,
    persist: bool,
}

impl AltServiceBuilder {
    /// Set authority for this alt svc
    pub fn autority(mut self, authority: Authority) -> Self {
        self.authority = Some(authority);
        self
    }

    /// Set protocol for alt svc
    pub fn protocol(mut self, protocol: Version) -> Self {
        self.protocol = protocol;
        self
    }

    /// Set port for this alt svc
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set ma for this alt svc
    pub fn ma(mut self, max_age: Duration) -> Self {
        self.ma = max_age;
        self
    }

    /// Allow persist for this alt svc
    pub fn persist(mut self, persist: bool) -> Self {
        self.persist = persist;
        self
    }

    /// Set authority for this alt svc
    pub fn build(self) -> AltService {
        AltService {
            authority: self.authority,
            protocol: self.protocol,
            port: self.port,
            ma: self.ma,
            persist: self.persist,
        }
    }
}

/// A helper type to define alt service values
pub struct AltService {
    authority: Option<Authority>,
    protocol: Version,
    port: u16,
    ma: Duration,
    persist: bool,
}

impl AltService {
    /// Create a new AltService builder
    pub fn builder() -> AltServiceBuilder {
        AltServiceBuilder {
            authority: None,
            protocol: Version::HTTP_2,
            port: 443,
            ma: Duration::from_hours(24),
            persist: false,
        }
    }
}

impl From<AltService> for String {
    fn from(value: AltService) -> String {
        let authority = value
            .authority
            .map_or("".into(), |authority| authority.to_string());
        let protocol = match value.protocol {
            Version::HTTP_11 => "http/1.1",
            Version::HTTP_2 => "h2",
            Version::HTTP_3 => "h3",
            _ => "http/1.1",
        };

        format!(
            "{protocol}={authority}:{},ma={},persist={}",
            value.port,
            value.ma.as_secs(),
            if value.persist { 1 } else { 0 }
        )
    }
}

/// Builder for creating `HostConfig` instances.
///
/// Provides a fluent API for configuring virtual hosts,
/// including hostname, port, and security settings.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{
///     security::SecurityConfig,
///     host::HostConfig
/// };
///
/// let security = SecurityConfig::builder()
///     .cert_from_bytes(vec![])
///     .key_from_bytes(vec![])
///     .build()
///     .unwrap();
///
/// let config = HostConfig::builder()
///     .hostname("example.com")
///     .security(security)
///     .build()
///     .unwrap();
/// ```
pub struct HostConfigBuilder {
    hostname: String,
    root_directory: Option<PathBuf>,
    default_headers: Option<Vec<(String, String)>>,
    security: Option<SecurityConfig>,
    status_pages: Option<HashMap<u16, String>>,
    enable_logging: bool,
    enable_hsts: bool,
    bind_addresses: Vec<(IpAddr, u16)>,
    paths: Vec<Box<dyn path::PathConfig>>,
}

impl HostConfigBuilder {
    /// Sets the hostname for the virtual host.
    ///
    /// This is used to match incoming requests to the correct virtual host.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::host::HostConfig;
    ///
    /// let config = HostConfig::builder()
    ///     .hostname("api.example.com")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn hostname(mut self, hostname: &str) -> Self {
        self.hostname = hostname.to_string();
        self
    }

    /// Sets the root directory for the virtual host.
    ///
    /// This is the base directory for all static file paths.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::host::HostConfig;
    ///
    /// let config = HostConfig::builder()
    ///     .root_directory("/var/www".into())
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn root_directory(mut self, root_directory: PathBuf) -> Self {
        self.root_directory = Some(root_directory);
        self
    }

    /// Adds a default header to the virtual host.
    ///
    /// These headers will be added to all responses from this virtual host.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::host::HostConfig;
    ///
    /// let config = HostConfig::builder()
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
    ///     host::HostConfig,
    /// };
    ///
    /// let security = SecurityConfig::builder()
    ///     .cert_from_bytes(vec![])
    ///     .key_from_bytes(vec![])
    ///     .build()
    ///     .unwrap();
    ///
    /// let config = HostConfig::builder()
    ///     .security(security)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn security(mut self, security: SecurityConfig) -> Self {
        self.security = Some(security);
        self
    }

    /// Sets the status pages for this host.
    ///
    /// These status pages will be used to serve custom error pages.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::host::HostConfig;
    /// use std::collections::HashMap;
    ///
    /// let mut status_pages = HashMap::new();
    /// status_pages.insert(404, "404.html".to_string());
    ///
    /// let config = HostConfig::builder()
    ///     .status_pages(status_pages)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn status_pages(mut self, status_pages: HashMap<u16, String>) -> Self {
        self.status_pages = Some(status_pages);
        self
    }

    /// Enables or disables logging for this host.
    ///
    /// When enabled, all requests to this host will be logged.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::host::HostConfig;
    ///
    /// let config = HostConfig::builder()
    ///     .enable_logging(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn enable_logging(mut self, logging: bool) -> Self {
        self.enable_logging = logging;
        self
    }

    /// Enables or disables HSTS for this host.
    ///
    /// When enabled, responses will contain a header to enforce use of HTTPS.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::host::HostConfig;
    ///
    /// let config = HostConfig::builder()
    ///     .enable_hsts(true)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn enable_hsts(mut self, enable_hsts: bool) -> Self {
        self.enable_hsts = enable_hsts;
        self
    }

    /// Add all addresses to bind this host to one or more listeners
    ///
    /// # Examples
    ///
    /// ```rust,norun
    ///
    /// ```
    pub fn bind_addresses(mut self, addresses: Vec<(IpAddr, u16)>) -> Self {
        self.bind_addresses = addresses;
        self
    }

    /// Adds a path configuration to the server.
    ///
    /// Multiple paths can be added to serve different content.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{host::HostConfig, server::ServerConfig};
    ///
    /// let path_config = PathConfig::default();
    /// let config = ServerConfig::builder()
    ///     .add_path(path_config)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_path<P>(mut self, path: P) -> Self
    where
        P: PathConfig + 'static,
    {
        self.paths
            .push(Box::new(path));
        self
    }

    /// Creates the `HostConfig` with the configured settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the hostname is empty.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::host::HostConfig;
    ///
    /// let config = HostConfig::builder()
    ///     .hostname("example.com")
    ///     .header("X-Custom", "value")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn build(self) -> VetisResult<HostConfig> {
        if self
            .hostname
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::Host("Missing hostname".to_string())));
        }

        if let Some(root_dir) = &self.root_directory {
            if !root_dir.exists() {
                return Err(VetisError::Config(ConfigError::Host(format!(
                    "root_directory does not exist: {:?}",
                    root_dir
                ))));
            }
        }

        Ok(HostConfig {
            hostname: self.hostname,
            root_directory: self.root_directory,
            default_headers: self.default_headers,
            security: self.security,
            status_pages: self.status_pages,
            enable_logging: self.enable_logging,
            enable_hsts: self.enable_hsts,
            paths: self.paths,
            bind_addresses: self.bind_addresses,
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
/// use vetis::host::HostConfig;
///
/// let config = HostConfig::builder()
///     .hostname("api.example.com")
///     .build()
///     .unwrap();
///
/// println!("Host: {}", config.hostname());
/// ```
#[derive(Deserialize)]
pub struct HostConfig {
    hostname: String,
    root_directory: Option<PathBuf>,
    default_headers: Option<Vec<(String, String)>>,
    #[serde(deserialize_with = "crate::security::config_from_file")]
    security: Option<SecurityConfig>,
    status_pages: Option<HashMap<u16, String>>,
    enable_logging: bool,
    enable_hsts: bool,
    paths: Vec<Box<dyn path::PathConfig>>,
    bind_addresses: Vec<(IpAddr, u16)>,
}

impl HostConfig {
    /// Creates a new `HostConfigBuilder` with default settings.
    ///
    /// Default values:
    /// - hostname: empty string (must be set)
    /// - port: 80
    /// - security: None
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::host::HostConfig;
    ///
    /// let config = HostConfig::builder()
    ///     .hostname("example.com")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> HostConfigBuilder {
        HostConfigBuilder {
            hostname: "localhost".to_string(),
            root_directory: None,
            default_headers: None,
            security: None,
            status_pages: None,
            enable_logging: true,
            enable_hsts: false,
            paths: Vec::new(),
            bind_addresses: Vec::new(),
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

    /// Returns the root directory.
    ///
    /// # Returns
    ///
    /// * `&str` - The root directory.
    pub fn root_directory(&self) -> &Option<PathBuf> {
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

    /// Returns hsts setting.
    ///
    /// # Returns
    ///
    /// * `bool` - if true, will add HSTS headers to response.
    pub fn enable_hsts(&self) -> bool {
        self.enable_hsts
    }

    /// Return bind addresses.
    ///
    /// # Returns
    ///
    /// * `&Vec<(IpAddr, u16)>` - The static paths.
    pub fn bind_addresses(&self) -> &Vec<(IpAddr, u16)> {
        &self.bind_addresses
    }

    /// Return paths.
    ///
    /// # Returns
    ///
    /// * `&Vec<Box<dyn Path>>` - The static paths.
    pub fn paths(&self) -> &Vec<Box<dyn path::PathConfig>> {
        &self.paths
    }
}

impl Default for HostConfig {
    fn default() -> Self {
        HostConfig {
            hostname: "localhost".to_string(),
            root_directory: None,
            default_headers: None,
            security: None,
            status_pages: None,
            enable_logging: true,
            enable_hsts: false,
            paths: Vec::new(),
            bind_addresses: Vec::new(),
        }
    }
}

impl From<&str> for HostConfig {
    fn from(hostname: &str) -> Self {
        HostConfig { hostname: hostname.to_string(), ..Default::default() }
    }
}

impl From<(&str, &str)> for HostConfig {
    fn from((hostname, root_directory): (&str, &str)) -> Self {
        HostConfig {
            hostname: hostname.to_string(),
            root_directory: Some(root_directory.into()),
            ..Default::default()
        }
    }
}

/// Host trait
pub trait Host {
    /// Returns the paths trie
    ///
    /// # Returns
    ///
    /// * `Trie<String, Box<dyn path::Path>>` - The paths trie.
    fn paths(&self) -> Trie<String, Arc<Box<dyn path::Path>>>;

    /// Returns host configuration
    ///
    /// # Returns
    ///
    /// * `&HostConfig` - A reference to the host configuration.
    fn config(&self) -> &HostConfig;

    /// Returns mutable host configuration
    ///
    /// # Returns
    ///
    /// * `&mut HostConfig` - A reference to the mutable host configuration.
    fn config_mut(&mut self) -> &mut HostConfig;

    /// Returns host name
    ///
    /// # Returns
    ///
    /// * `&str` - A reference to the host hostname.
    fn hostname(&self) -> &str {
        self.config()
            .hostname()
    }

    /// Serve a status page
    ///
    /// # Arguments
    ///
    /// * `status` - The status code to serve.
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = VetisResult<Response>> + Send>>` - A pinned box containing the future that will resolve to a `Result<Response, VetisError>`.
    fn serve_status_page<'a>(&'a self, status: u16) -> VetisFutureResult<'a, Response>;

    /// Route request to the appropriate handler
    ///
    /// # Arguments
    ///
    /// * `request` - A `Request` instance containing the request information.
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = VetisResult<Response>> + Send>>` - A pinned box containing the future that will resolve to a `Result<Response, VetisError>`.
    fn route<'a>(&'a self, request: Request) -> VetisFutureResult<'a, Response>
    where
        Self: Sync;
}
