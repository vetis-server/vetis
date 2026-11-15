use crate::VetisHooks;
use crate::errors::RuntimeError;
use crate::hook::HookConfig;
use crate::{
    LogSender, Request, Response, Str, VetisFutureResult, VetisResult,
    errors::{ConfigError, VetisError},
    hook::{self},
    log::{self, LogConfig, Logger},
    security::TlsConfig,
    telemetry::{self, TelemetryConfig},
};
use ::log::warn;
use http::{
    Version,
    header::{
        CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS,
        X_XSS_PROTECTION,
    },
    uri::Authority,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    time::Duration,
};

const MAX_REQUEST_SIZE: i32 = 10485760;

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
            ma: Duration::from_secs(24 * 60 * 60),
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
    hostname: Str,
    root_directory: Option<PathBuf>,
    protos: Vec<Version>,
    allow_unsafe_conn: bool,
    max_request_size: i32,
    default_headers: Option<Vec<(Str, Str)>>,
    tls: Option<TlsConfig>,
    status_pages: Option<HashMap<u16, Str>>,
    enable_hsts: bool,
    bind_addresses: Vec<(IpAddr, u16)>,
    log: Option<Box<dyn log::LogConfig>>,
    telemetry: Option<Box<dyn telemetry::TelemetryConfig>>,
    hooks: Option<Vec<Box<dyn hook::HookConfig>>>,
}

impl HostConfigBuilder {
    /// Sets the hostname for the virtual host.
    ///
    /// This is used to match incoming requests to the correct virtual host.
    pub fn hostname(mut self, hostname: &str) -> Self {
        self.hostname = hostname.into();
        self
    }

    /// Sets the HTTP protocol for this listener.
    pub fn protos(mut self, protos: &[Version]) -> Self {
        self.protos = protos.to_vec();
        self
    }

    /// Sets the HTTP to allow unsafe connections for this listener.
    ///
    /// # Notes
    ///
    /// Enable unsafe connections should be only enabled for testing purposes.
    /// Please be cautious when using this setting.
    pub fn allow_unsafe_connections(mut self, allow_unsafe_conn: bool) -> Self {
        self.allow_unsafe_conn = allow_unsafe_conn;
        self
    }

    /// Sets the root directory for the virtual host.
    ///
    /// This is the base directory for all static file paths.
    pub fn root_directory(mut self, root_directory: impl Into<PathBuf>) -> Self {
        self.root_directory = Some(root_directory.into());
        self
    }

    /// Adds a default header to the virtual host.
    ///
    /// These headers will be added to all responses from this virtual host.
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
    /// # Notes:
    ///
    /// Enable TLS will also enable HSTS
    pub fn tls(mut self, tls: TlsConfig) -> Self {
        self.tls = Some(tls);
        self.enable_hsts = true;
        self
    }

    /// Sets the status pages for this host.
    ///
    /// These status pages will be used to serve custom error pages.
    pub fn status_pages(mut self, status_pages: HashMap<u16, Str>) -> Self {
        self.status_pages = Some(status_pages);
        self
    }

    /// Enables or disables HSTS for this host.
    ///
    /// # Notes
    ///
    /// When enabled, responses will contain a header to enforce use of HTTPS.
    /// It is recommended to set it to true for production deployments, set tls
    /// will automatically enable HSTS.
    pub fn enable_hsts(mut self, enable_hsts: bool) -> Self {
        self.enable_hsts = enable_hsts;
        self
    }

    /// Set addresses to which bind this host to
    ///
    /// # Arguments
    ///
    /// * `&[(IpAddr, u16)]` - A slice of tuple containing ip address and port
    ///
    /// # Notes
    ///
    /// Add all addresses to bind this host to one or more listeners
    /// For production deployments, do not leave default pair, please
    /// bind to a private interface and port 443 preferebly.
    pub fn bind_addresses(mut self, addresses: &[(IpAddr, u16)]) -> Self {
        self.bind_addresses = addresses.into();
        self
    }

    /// Set host log
    ///
    /// # Arguments
    ///
    /// * `log` - Log configuration
    pub fn log<L>(mut self, log: L) -> Self
    where
        L: LogConfig + 'static,
    {
        self.log = Some(Box::new(log));
        self
    }

    /// Set host telemetry
    ///
    /// # Arguments
    ///
    /// * `telemetry` - Telemetry configuration
    pub fn telemetry<T>(mut self, telemetry: T) -> Self
    where
        T: TelemetryConfig + 'static,
    {
        self.telemetry = Some(Box::new(telemetry));
        self
    }

    /// Add a Hook to host
    ///
    /// # Arguments
    ///
    /// * `path` - Hook configuration
    ///
    /// # Notes
    ///
    /// Add a Hook configuration to the host, Hooks are
    /// useful to add some relevant behavior, but can also
    /// degrade performance, please use them with careful
    /// consideration
    pub fn add_hook<M>(mut self, hook: M) -> Self
    where
        M: HookConfig + 'static,
    {
        match self.hooks {
            None => {
                self.hooks = Some(vec![Box::new(hook)]);
            }
            Some(ref mut hooks) => {
                hooks.push(Box::new(hook));
            }
        }
        self
    }

    /// Creates the `HostConfig` with the configured settings.
    ///
    /// # Errors
    ///
    /// Returns an error if the hostname is empty.
    pub fn build(self) -> VetisResult<HostConfig> {
        if self
            .hostname
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::host("Missing hostname")));
        }

        if let Some(root_dir) = &self.root_directory {
            if !root_dir.exists() {
                return Err(VetisError::Config(ConfigError::host(compact_str::format_compact!(
                    "root_directory does not exist: {:?}",
                    root_dir
                ))));
            }
        }

        if self
            .bind_addresses
            .iter()
            .find(|p| p.0.is_unspecified())
            .is_none()
        {
            warn!(target: "vetis", "Please refrain from use public interfaces in production!");
        }

        if self.enable_hsts && self.tls.is_none() {
            return Err(VetisError::Config(ConfigError::Host(
                "HSTS is enabled, but there's no TLS configuration set, please configure TLS."
                    .into(),
            )));
        }

        Ok(HostConfig {
            hostname: self.hostname,
            root_directory: self.root_directory,
            protos: self.protos,
            allow_unsafe_conn: self.allow_unsafe_conn,
            max_request_size: self.max_request_size,
            default_headers: self
                .default_headers
                .map_or(None, |val| Some(val.into_boxed_slice())),
            tls: self.tls,
            status_pages: self.status_pages,
            enable_hsts: self.enable_hsts,
            bind_addresses: self
                .bind_addresses
                .as_slice()
                .into(),
            log: self.log,
            telemetry: self.telemetry,
            hooks: self
                .hooks
                .map_or(None, |val| Some(val.into_boxed_slice())),
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
#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct HostConfig {
    hostname: Str,
    root_directory: Option<PathBuf>,
    #[serde(with = "http_serde_ext::version::vec")]
    protos: Vec<Version>,
    allow_unsafe_conn: bool,
    max_request_size: i32,
    default_headers: Option<Box<[(Str, Str)]>>,
    tls: Option<TlsConfig>,
    status_pages: Option<HashMap<u16, Str>>,
    enable_hsts: bool,
    bind_addresses: Box<[(IpAddr, u16)]>,
    log: Option<Box<dyn log::LogConfig>>,
    telemetry: Option<Box<dyn telemetry::TelemetryConfig>>,
    hooks: Option<Box<[Box<dyn hook::HookConfig>]>>,
}

impl HostConfig {
    /// Creates a new `HostConfigBuilder` with default settings.
    ///
    /// Default values:
    /// - hostname: empty string (must be set)
    /// - port: 80
    /// - bind_addresses: [("0.0.0.0", 80)] (for production deployments, do not use this setting)
    /// - tls: None
    /// - root_directory: None
    /// - protos: [Version::HTTP_1]
    /// - allow_unsafe_conn: false
    /// - max_request_size: 10mb (expressed in bytes)
    /// - paths: []
    /// - default_headers: None
    /// - status_pages: None
    /// - log: None
    /// - telemtry: None
    /// - hooks: None
    pub fn builder() -> HostConfigBuilder {
        HostConfigBuilder {
            hostname: "localhost".into(),
            root_directory: None,
            protos: vec![Version::HTTP_11],
            allow_unsafe_conn: false,
            max_request_size: MAX_REQUEST_SIZE,
            default_headers: None,
            tls: None,
            status_pages: None,
            enable_hsts: false,
            bind_addresses: [(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 80)].into(),
            log: None,
            telemetry: None,
            hooks: None,
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
    pub fn root_directory(&self) -> Option<&PathBuf> {
        self.root_directory
            .as_ref()
    }

    /// Returns HTTP protocol.
    pub fn protos(&self) -> &Vec<Version> {
        &self.protos
    }

    /// Returns HTTP protocol.
    pub fn allow_unsafe_connections(&self) -> bool {
        self.allow_unsafe_conn
    }

    /// Return max request size
    pub fn max_request_size(&self) -> i32 {
        self.max_request_size
    }

    /// Returns the default headers.
    ///
    /// # Returns
    ///
    /// * `Option<&Arc<[(Str, Str)]>>` - The default headers.
    pub fn default_headers(&self) -> Option<&Box<[(Str, Str)]>> {
        self.default_headers
            .as_ref()
    }

    /// Returns the tls configuration if present.
    ///
    /// # Returns
    ///
    /// * `&Option<TlsConfig>` - The security configuration if present.
    pub fn tls(&self) -> Option<&TlsConfig> {
        self.tls.as_ref()
    }

    /// Returns the status pages.
    ///
    /// # Returns
    ///
    /// * `&Option<HashMap<u16, String>>` - The status pages.
    pub fn status_pages(&self) -> Option<&HashMap<u16, Str>> {
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
    pub fn bind_addresses(&self) -> &Box<[(IpAddr, u16)]> {
        &self.bind_addresses
    }

    /// Return log config instance.
    ///
    /// # Returns
    ///
    /// * `&Vec<Box<dyn LogConfig>>` - The log config.
    pub fn log(&self) -> Option<&Box<dyn log::LogConfig>> {
        self.log.as_ref()
    }

    /// Return telemetry config instance.
    ///
    /// # Returns
    ///
    /// * `&Vec<Box<dyn TelemetryConfig>>` - The telemetry config.
    pub fn telemetry(&self) -> Option<&Box<dyn telemetry::TelemetryConfig>> {
        self.telemetry
            .as_ref()
    }

    /// Return Hooks configuration.
    ///
    /// # Returns
    ///
    /// * `&Vec<Box<dyn HookConfig>>` - The Hooks configuration.
    pub fn hooks(&self) -> Option<&Box<[Box<dyn hook::HookConfig>]>> {
        self.hooks.as_ref()
    }
}

impl Default for HostConfig {
    fn default() -> Self {
        // Give these headers as suggestion, but they can be overwritten by user
        let default_headers = [
            (
                Str::from(CONTENT_SECURITY_POLICY.as_str()),
                Str::from(
                    "default-src 'self'; object-src 'none'; base-uri 'none'; upgrade-insecure-requests;",
                ),
            ),
            (Str::from(X_FRAME_OPTIONS.as_str()), Str::const_new("DENY")),
            (Str::from(X_CONTENT_TYPE_OPTIONS.as_str()), Str::const_new("nosniff")),
            (
                Str::from(REFERRER_POLICY.as_str()),
                Str::const_new("strict-origin-when-cross-origin"),
            ),
            (Str::from(X_XSS_PROTECTION.as_str()), Str::const_new("0")),
        ];
        HostConfig {
            hostname: "localhost".into(),
            root_directory: None,
            protos: vec![Version::HTTP_11],
            allow_unsafe_conn: false,
            max_request_size: 1000000,
            default_headers: Some(default_headers.into()),
            tls: None,
            status_pages: None,
            enable_hsts: false,
            bind_addresses: [(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 80)].into(),
            log: None,
            telemetry: None,
            hooks: None,
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

/// Host trait
pub trait Host {
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

    /// Returns host Hooks
    ///
    /// # Returns
    ///
    /// * `&HostHook` - A reference to the host hooks.
    fn hooks(&self) -> &VetisHooks;

    /// Returns host hooks
    ///
    /// # Returns
    ///
    /// * `&mut HostHook` - A reference to the host hooks.
    fn hooks_mut(&mut self) -> &mut VetisHooks;

    /// Serve a status page
    ///
    /// # Arguments
    ///
    /// * `status` - The status code to serve.
    ///
    /// # Returns
    ///
    /// * `VetisFutureResult<'a, Response>` - A pinned box containing the future
    ///   that will resolve to a `Result<Response, VetisError>`.
    fn serve_status_page<'a>(
        &'a self,
        error: RuntimeError,
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
    /// * `VetisFutureResult<'a, Response>` - A pinned box containing the future
    ///   that will resolve to a `Result<Response, VetisError>`.
    fn route<'a>(
        &'a self,
        request: Request,
        logger: Option<Logger<LogSender>>,
    ) -> VetisFutureResult<'a, Response>;
}
