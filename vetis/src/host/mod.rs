use crate::{
    LogSender, Request, Response, VetisFutureResult, VetisPathRouter, VetisResult,
    errors::{ConfigError, VetisError},
    host::path::PathConfig,
    log::{self, LogConfig, Logger},
    security::TlsConfig,
};
use http::{Version, uri::Authority};
use serde::Deserialize;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

/// Path configuration for hosts.
pub mod path;

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
    hostname: Arc<str>,
    root_directory: Option<PathBuf>,
    protos: Vec<Version>,
    allow_unsafe_conn: bool,
    default_headers: Option<Vec<(Arc<str>, Arc<str>)>>,
    tls: Option<TlsConfig>,
    status_pages: Option<HashMap<u16, Arc<str>>>,
    enable_hsts: bool,
    bind_addresses: Vec<(IpAddr, u16)>,
    paths: Vec<Box<dyn path::PathConfig>>,
    log: Option<Box<dyn log::LogConfig>>,
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
        self.hostname = hostname.into();
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
    pub fn protos(mut self, protos: &[Version]) -> Self {
        self.protos = protos.to_vec();
        self
    }

    /// Sets the HTTP to allow unsafe connections for this listener.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use http::Version;
    /// use vetis::{listener::ListenerConfig};
    ///
    /// #[cfg(feature = "http1")]
    /// let config = ListenerConfig::builder()
    ///     .allow_unsafe_connections(true)
    ///     .build();
    /// ```
    ///
    /// # Notes
    ///
    /// Enable unsafe connections should be only enabled for testing purposes.
    /// Please be cautious when using this setting.
    ///
    pub fn allow_unsafe_connections(mut self, allow_unsafe_conn: bool) -> Self {
        self.allow_unsafe_conn = allow_unsafe_conn;
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
    pub fn root_directory(mut self, root_directory: impl Into<PathBuf>) -> Self {
        self.root_directory = Some(root_directory.into());
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
                let vec = vec![(key.into(), value.into())];
                self.default_headers = Some(vec);
            }
            Some(ref mut headers) => {
                headers.push((key.into(), value.into()));
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
    ///     .tls(security)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn tls(mut self, tls: TlsConfig) -> Self {
        self.tls = Some(tls);
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
    pub fn status_pages(mut self, status_pages: HashMap<u16, Arc<str>>) -> Self {
        self.status_pages = Some(status_pages);
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
    pub fn bind_addresses(mut self, addresses: &[(IpAddr, u16)]) -> Self {
        self.bind_addresses = addresses.into();
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

    /// Log host log
    ///
    /// # Examples
    ///
    /// ```rust,norun
    ///
    /// ```
    pub fn log<L>(mut self, log: L) -> Self
    where
        L: LogConfig + 'static,
    {
        self.log = Some(Box::new(log));
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
            protos: self.protos,
            allow_unsafe_conn: self.allow_unsafe_conn,
            default_headers: self
                .default_headers
                .map_or(None, |val| {
                    Some(
                        val.as_slice()
                            .into(),
                    )
                }),
            tls: self.tls,
            status_pages: self.status_pages,
            enable_hsts: self.enable_hsts,
            bind_addresses: self
                .bind_addresses
                .as_slice()
                .into(),
            paths: self.paths.into(),
            log: self.log,
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
#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct HostConfig {
    hostname: Arc<str>,
    root_directory: Option<PathBuf>,
    #[serde(with = "http_serde_ext::version::vec")]
    protos: Vec<Version>,
    allow_unsafe_conn: bool,
    default_headers: Option<Arc<[(Arc<str>, Arc<str>)]>>,
    tls: Option<TlsConfig>,
    status_pages: Option<HashMap<u16, Arc<str>>>,
    enable_hsts: bool,
    bind_addresses: Arc<[(IpAddr, u16)]>,
    paths: Arc<[Box<dyn path::PathConfig>]>,
    log: Option<Box<dyn log::LogConfig>>,
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
            hostname: "localhost".into(),
            root_directory: None,
            protos: vec![Version::HTTP_11],
            allow_unsafe_conn: false,
            default_headers: None,
            tls: None,
            status_pages: None,
            enable_hsts: false,
            bind_addresses: [(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 80)].into(),
            paths: [].into(),
            log: None,
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

    /// Returns HTTP protocol.
    pub fn protos(&self) -> &Vec<Version> {
        &self.protos
    }

    /// Returns HTTP protocol.
    pub fn allow_unsafe_connections(&self) -> bool {
        self.allow_unsafe_conn
    }

    /// Returns the default headers.
    ///
    /// # Returns
    ///
    /// * `Option<&Arc<[(Arc<str>, Arc<str>)]>>` - The default headers.
    pub fn default_headers(&self) -> Option<&Arc<[(Arc<str>, Arc<str>)]>> {
        self.default_headers
            .as_ref()
    }

    /// Returns the tls configuration if present.
    ///
    /// # Returns
    ///
    /// * `&Option<TlsConfig>` - The security configuration if present.
    pub fn tls(&self) -> &Option<TlsConfig> {
        &self.tls
    }

    /// Returns the status pages.
    ///
    /// # Returns
    ///
    /// * `&Option<HashMap<u16, String>>` - The status pages.
    pub fn status_pages(&self) -> Option<&HashMap<u16, Arc<str>>> {
        self.status_pages
            .as_ref()
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
    /// * `&Vec<(IpAddr, u16)>` - The bind addresses.
    pub fn bind_addresses(&self) -> &Arc<[(IpAddr, u16)]> {
        &self.bind_addresses
    }

    /// Return paths configuration.
    ///
    /// # Returns
    ///
    /// * `&Vec<Box<dyn Path>>` - The paths configuration.
    pub fn paths(&self) -> &Arc<[Box<dyn path::PathConfig>]> {
        &self.paths
    }

    /// Return log config instance.
    ///
    /// # Returns
    ///
    /// * `&Vec<Box<dyn LogConfig>>` - The log config.
    pub fn log(&self) -> Option<&Box<dyn log::LogConfig>> {
        self.log.as_ref()
    }
}

impl Default for HostConfig {
    fn default() -> Self {
        HostConfig {
            hostname: "localhost".into(),
            root_directory: None,
            protos: vec![Version::HTTP_11],
            allow_unsafe_conn: false,
            default_headers: None,
            tls: None,
            status_pages: None,
            enable_hsts: false,
            bind_addresses: [(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 80)].into(),
            paths: [].into(),
            log: None,
        }
    }
}

impl From<&str> for HostConfig {
    fn from(hostname: &str) -> Self {
        HostConfig { hostname: hostname.into(), ..Default::default() }
    }
}

impl From<(&str, &str)> for HostConfig {
    fn from((hostname, root_directory): (&str, &str)) -> Self {
        HostConfig {
            hostname: hostname.into(),
            root_directory: Some(root_directory.into()),
            ..Default::default()
        }
    }
}

/// A type to give more context about the
/// host served by this vetis instance
pub struct HostContext {
    path_uri: String,
    root_directory: Option<PathBuf>,
    logger: Option<Logger<LogSender>>,
}

impl HostContext {
    /// Create a HostContext from Uri
    pub fn from_uri(uri: &str) -> Self {
        Self { root_directory: None, path_uri: uri.into(), logger: None }
    }

    /// Allow set root directory
    pub fn with_root_directory(mut self, directory: Option<PathBuf>) -> Self {
        self.root_directory = directory;
        self
    }

    /// Allow set logger
    pub fn with_logger(mut self, logger: Option<Logger<LogSender>>) -> Self {
        self.logger = logger;
        self
    }

    /// Returns the root directory
    pub fn root_directory(&self) -> &Option<PathBuf> {
        &self.root_directory
    }

    /// Returns the current Path uri
    pub fn path_uri(&self) -> &str {
        &self.path_uri
    }

    /// Returns the current Logger
    pub fn logger(&self) -> &Option<Logger<LogSender>> {
        &self.logger
    }
}

/// Host trait
pub trait Host {
    /// Path associated type
    type Path;
    /// Returns the paths router
    ///
    /// # Returns
    ///
    /// * `VetisPathRouter` - The paths router.
    fn paths(&self) -> &VetisPathRouter<Self::Path>;

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
    /// * `Pin<Box<dyn Future<Output = VetisResult<Response>> + Send>>` - A pinned box
    ///    containing the future that will resolve to a `Result<Response, VetisError>`.
    fn serve_status_page<'a>(
        &'a self,
        status: u16,
        logger: Option<Logger<LogSender>>,
    ) -> VetisFutureResult<'a, Response>;

    /// Route request to the appropriate handler
    ///
    /// # Arguments
    ///
    /// * `request` - A `Request` instance containing the request information.
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = VetisResult<Response>> + Send>>` - A pinned box
    ///    containing the future that will resolve to a `Result<Response, VetisError>`.
    fn route<'a>(
        &'a self,
        request: Request,
        logger: Option<Logger<LogSender>>,
    ) -> VetisFutureResult<'a, Response>;
}
