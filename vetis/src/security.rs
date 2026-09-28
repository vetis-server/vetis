use crate::{
    VetisResult,
    errors::{ConfigError, VetisError},
};
use serde::Deserialize;
use std::{error::Error, fmt::Display, path::PathBuf, str::FromStr, sync::Arc};

/// Builder for creating `SecurityConfig` instances.
///
/// Provides a fluent API for configuring TLS/SSL security settings,
/// including certificates, private keys, and client authentication.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::security::TlsConfig;
///
/// let security = TlsConfig::builder()
///     .cert_file("../../certs/server.der")
///     .key_file("../../certs/server.key.der")
///     .ca_file("../../certs/ca.der")
///     .client_auth(true)
///     .build();
/// ```
#[derive(Clone)]
pub struct TlsConfigBuilder {
    cert_file: Option<PathBuf>,
    key_file: Option<PathBuf>,
    ca_file: Option<PathBuf>,
    client_auth: bool,
    supported_alpns: Arc<[Alpn]>,
}

impl TlsConfigBuilder {
    /// Sets the server certificate from bytes.
    ///
    /// The certificate should be in DER format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::security::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .cert_file("../../certs/server.der")
    ///     .build();
    /// ```
    pub fn cert_file(mut self, cert: impl Into<PathBuf>) -> Self {
        self.cert_file = Some(cert.into());
        self
    }

    /// Sets the private key from bytes.
    ///
    /// The key should be in DER format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::security::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .key_file("../../certs/server.key.der")
    ///     .build();
    /// ```
    pub fn key_file(mut self, key: impl Into<PathBuf>) -> Self {
        self.key_file = Some(key.into());
        self
    }

    /// Sets the CA certificate file path.
    ///
    /// The CA certificate is used for client authentication and should be in DER format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::security::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .ca_file("../../certs/ca.der")
    ///     .build();
    /// ```
    pub fn ca_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.ca_file = Some(file.into());
        self
    }

    /// Sets whether client authentication is required.
    ///
    /// When enabled, clients must present a valid certificate signed by the CA.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::security::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .client_auth(true)
    ///     .build();
    /// ```
    pub fn client_auth(mut self, client_auth: bool) -> Self {
        self.client_auth = client_auth;
        self
    }

    /// Sets the supported alpn extesions.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{Alpn, {listener::ListenerConfig}};
    ///
    /// #[cfg(feature = "http1")]
    /// let config = ListenerConfig::builder()
    ///     .alpn_protos(vec![Alpn::Http11])
    ///     .build();
    /// ```
    pub fn supported_alpns(mut self, alpn: &[Alpn]) -> Self {
        self.supported_alpns = alpn.into();
        self
    }

    /// Creates the `TlsConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `Result<TlsConfig, VetisError>` - The `TlsConfig` with the configured settings.
    pub fn build(self) -> VetisResult<TlsConfig> {
        let Some(cert_file) = self.cert_file else {
            return Err(VetisError::Config(ConfigError::Tls(
                "Missing certificate file".to_string(),
            )));
        };

        let Some(key_file) = self.key_file else {
            return Err(VetisError::Config(ConfigError::Tls(
                "Missing certificate key file".to_string(),
            )));
        };

        Ok(TlsConfig {
            cert_file,
            key_file,
            ca_file: self.ca_file,
            client_auth: self.client_auth,
            supported_alpns: self.supported_alpns,
        })
    }
}

/// Configuration for TLS/SSL.
///
/// Contains the certificates and keys needed to establish secure HTTPS connections.
/// This configuration is used by virtual hosts to enable TLS.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::security::TlsConfig;
///
/// let security = TlsConfig::builder()
///     .cert_file("../../certs/server.der")
///     .key_file("../../certs/server.key.der")
///     .build()
///     .unwrap();
///
/// println!("Certificate length: {} bytes", security.cert().len());
/// ```
#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct TlsConfig {
    cert_file: PathBuf,
    key_file: PathBuf,
    #[serde(default)]
    ca_file: Option<PathBuf>,
    #[serde(default)]
    client_auth: bool,
    #[serde(default)]
    supported_alpns: Arc<[Alpn]>,
}

impl From<(PathBuf, PathBuf, PathBuf)> for TlsConfig {
    fn from((cert_file, key_file, ca_cert_file): (PathBuf, PathBuf, PathBuf)) -> Self {
        TlsConfig {
            cert_file,
            key_file,
            ca_file: Some(ca_cert_file),
            client_auth: false,
            supported_alpns: [].into(),
        }
    }
}

impl TlsConfig {
    /// Creates a new `TlsConfigBuilder` with default settings.
    ///
    /// Default values:
    /// - cert: empty (must be set)
    /// - key: empty (must be set)
    /// - ca_cert: None
    /// - client_auth: false
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::security::TlsConfig;
    ///
    /// let security = TlsConfig::builder()
    ///     .cert_from_bytes(vec![])
    ///     .key_from_bytes(vec![])
    ///     .build();
    /// ```
    pub fn builder() -> TlsConfigBuilder {
        TlsConfigBuilder {
            ca_file: None,
            key_file: None,
            cert_file: None,
            client_auth: false,
            supported_alpns: [].into(),
        }
    }

    /// Returns the CA certificate file path if present.
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<u8>>` - The CA certificate file path if present.
    pub fn cert_file(&self) -> &PathBuf {
        &self.cert_file
    }

    /// Returns the certificate key file if present.
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<u8>>` - The certificate key file if present.
    pub fn key_file(&self) -> &PathBuf {
        &self.key_file
    }

    /// Returns the CA certificate file if present.
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<u8>>` - The CA certificate bytes if present.
    pub fn ca_file(&self) -> &Option<PathBuf> {
        &self.ca_file
    }

    /// Returns whether client authentication is enabled.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether client authentication is enabled.
    pub fn client_auth(&self) -> bool {
        self.client_auth
    }

    /// Returns supported alpn extensions.
    pub fn supported_alpns(&self) -> &Arc<[Alpn]> {
        &self.supported_alpns
    }
}

/// Security type holding sensitive data
pub struct Tls {
    cert: Vec<u8>,
    key: Vec<u8>,
    ca: Option<Vec<u8>>,
    client_auth: bool,
    supported_alpns: Vec<Alpn>,
}

impl Tls {
    /// Create security type from a cert
    pub fn from_cert_and_key(cert: &[u8], key: &[u8]) -> Self {
        Self {
            cert: cert.into(),
            key: key.into(),
            ca: None,
            client_auth: false,
            supported_alpns: Vec::new(),
        }
    }

    /// Certificate authority in bytes
    pub fn with_ca(mut self, ca: &[u8]) -> Self {
        self.ca = Some(ca.into());
        self
    }

    /// Allow inidicate if client auth will be useds
    pub fn with_client_auth(mut self, client_auth: bool) -> Self {
        self.client_auth = client_auth;
        self
    }

    /// Returns certificate bytes
    pub fn cert(&self) -> &[u8] {
        &self.cert
    }

    /// Returns certificate key bytes
    pub fn key(&self) -> &[u8] {
        &self.key
    }

    /// Returns ca bytes
    pub fn ca(&self) -> &Option<Vec<u8>> {
        &self.ca
    }

    /// Returns true if client_auth is enabled, false otherwise
    pub fn client_auth(&self) -> bool {
        self.client_auth
    }

    /// Returns a list of supported alpns
    pub fn supported_alpns(&self) -> &Vec<Alpn> {
        &self.supported_alpns
    }
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
/// Enum for ALPN
pub enum Alpn {
    /// HTTP/1.1
    Http11,
    /// H2
    H2,
    /// H2C
    H2c,
    /// H3
    H3,
    /// DOT
    Dot,
    /// DOC
    Doh,
    /// DOQ
    Doq,
    /// ACME-TLS/1
    AcmeTls1,
}

impl From<&str> for Alpn {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "http/1.1" => Alpn::Http11,
            "h2" => Alpn::H2,
            "h2c" => Alpn::H2c,
            "h3" => Alpn::H3,
            "dot" => Alpn::Dot,
            "doh" => Alpn::Doh,
            "doq" => Alpn::Doq,
            "acme-tls/1" => Alpn::AcmeTls1,
            &_ => panic!("Not a valid ALPN protocol"),
        }
    }
}

impl From<Vec<u8>> for Alpn {
    fn from(value: Vec<u8>) -> Self {
        match value.as_slice() {
            b"http/1.1" => Alpn::Http11,
            b"h2" => Alpn::H2,
            b"h2c" => Alpn::H2c,
            b"h3" => Alpn::H3,
            b"dot" => Alpn::Dot,
            b"doh" => Alpn::Doh,
            b"doq" => Alpn::Doq,
            b"acme-tls/1" => Alpn::AcmeTls1,
            &_ => panic!("Not a valid ALPN protocol"),
        }
    }
}

impl From<&Alpn> for Vec<u8> {
    fn from(value: &Alpn) -> Self {
        match value {
            Alpn::Http11 => b"http/1.1".into(),
            Alpn::H2 => b"h2".into(),
            Alpn::H2c => b"h2c".into(),
            Alpn::H3 => b"h3".into(),
            Alpn::Dot => b"dot".into(),
            Alpn::Doh => b"doh".into(),
            Alpn::Doq => b"doq".into(),
            Alpn::AcmeTls1 => b"acme-tls/1".into(),
        }
    }
}

impl FromStr for Alpn {
    type Err = InvalidAlpnError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http/1.1" | "HTTP/1.1" => Ok(Alpn::Http11),
            "h2" | "H2" => Ok(Alpn::H2),
            "h2c" | "H2C" => Ok(Alpn::H2c),
            "h3" | "H3" => Ok(Alpn::H3),
            "dot" | "DOT" => Ok(Alpn::Dot),
            "doh" | "DOH" => Ok(Alpn::Doh),
            "doq" | "DOQ" => Ok(Alpn::Doq),
            "acml-tls-1" | "ACME-TLS-1" => Ok(Alpn::AcmeTls1),
            &_ => Err(InvalidAlpnError(s.into())),
        }
    }
}

/// InvalidAlpnError type is a custom error for invalid alpn code
#[derive(Debug)]
pub struct InvalidAlpnError(String);

impl Display for InvalidAlpnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid alpn: {}", self.0)
    }
}

impl Error for InvalidAlpnError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn description(&self) -> &str {
        "Invalid Alpn Error"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
