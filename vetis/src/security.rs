use std::fs;

use log::error;
use serde::{Deserialize, Deserializer};

use crate::errors::{ConfigError, VetisError};

/// Builder for creating `SecurityConfig` instances.
///
/// Provides a fluent API for configuring TLS/SSL security settings,
/// including certificates, private keys, and client authentication.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::config::SecurityConfig;
///
/// let security = SecurityConfig::builder()
///     .cert_from_bytes(include_bytes!("server.der").to_vec())
///     .key_from_bytes(include_bytes!("server.key.der").to_vec())
///     .ca_cert_from_bytes(include_bytes!("ca.der").to_vec())
///     .client_auth(true)
///     .build();
/// ```
#[derive(Clone)]
pub struct SecurityConfigBuilder {
    cert: Vec<u8>,
    key: Vec<u8>,
    ca_cert: Option<Vec<u8>>,
    client_auth: bool,
}

impl SecurityConfigBuilder {
    /// Sets the server certificate from bytes.
    ///
    /// The certificate should be in DER format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .cert_from_bytes(include_bytes!("server.der").to_vec())
    ///     .build();
    /// ```
    pub fn cert_from_bytes(mut self, cert: Vec<u8>) -> Self {
        self.cert = cert;
        self
    }

    /// Sets the server certificate from a file.
    ///
    /// Reads the certificate file in DER format.
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .cert_from_file("/path/to/server.der")
    ///     .build();
    /// ```
    pub fn cert_from_file(mut self, path: &str) -> Self {
        let cert = fs::read(path);
        match cert {
            Ok(cert) => self.cert = cert,
            Err(e) => {
                error!("Failed to read certificate file: {}", e);
            }
        }
        self
    }

    /// Sets the private key from bytes.
    ///
    /// The key should be in DER format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .key_from_bytes(include_bytes!("server.key.der").to_vec())
    ///     .build();
    /// ```
    pub fn key_from_bytes(mut self, key: Vec<u8>) -> Self {
        self.key = key;
        self
    }

    /// Sets the private key from a file.
    ///
    /// Reads the key file in DER format.
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .key_from_file("/path/to/server.key.der")
    ///     .build();
    /// ```
    pub fn key_from_file(mut self, path: &str) -> Self {
        let key = fs::read(path);
        match key {
            Ok(key) => self.key = key,
            Err(e) => {
                error!("Failed to read key file: {}", e);
            }
        }
        self
    }

    /// Sets the CA certificate from bytes.
    ///
    /// The CA certificate is used for client authentication and should be in DER format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .ca_cert_from_bytes(include_bytes!("ca.der").to_vec())
    ///     .build();
    /// ```
    pub fn ca_cert_from_bytes(mut self, ca_cert: Vec<u8>) -> Self {
        self.ca_cert = Some(ca_cert);
        self
    }

    /// Sets the CA certificate from a file.
    ///
    /// Reads the CA certificate file in DER format.
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be read.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .ca_cert_from_file("/path/to/ca.der")
    ///     .build();
    /// ```
    pub fn ca_cert_from_file(mut self, path: &str) -> Self {
        let ca_cert = fs::read(path);
        match ca_cert {
            Ok(ca_cert) => self.ca_cert = Some(ca_cert),
            Err(e) => {
                error!("Failed to read CA certificate file: {}", e);
            }
        }
        self
    }

    /// Sets whether client authentication is required.
    ///
    /// When enabled, clients must present a valid certificate signed by the CA.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::config::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .client_auth(true)
    ///     .build();
    /// ```
    pub fn client_auth(mut self, client_auth: bool) -> Self {
        self.client_auth = client_auth;
        self
    }

    /// Creates the `SecurityConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `Result<SecurityConfig, VetisError>` - The `SecurityConfig` with the configured settings.
    pub fn build(self) -> Result<SecurityConfig, VetisError> {
        if self.cert.is_empty() {
            return Err(VetisError::Config(ConfigError::Security(
                "Missing certificate".to_string(),
            )));
        }

        if self.key.is_empty() {
            return Err(VetisError::Config(ConfigError::Security("Missing key".to_string())));
        }

        Ok(SecurityConfig {
            cert: self.cert,
            key: self.key,
            ca_cert: self.ca_cert,
            client_auth: self.client_auth,
        })
    }
}

/// Security configuration for TLS/SSL.
///
/// Contains the certificates and keys needed to establish secure HTTPS connections.
/// This configuration is used by virtual hosts to enable TLS.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::config::SecurityConfig;
///
/// let security = SecurityConfig::builder()
///     .cert_from_bytes(include_bytes!("server.der").to_vec())
///     .key_from_bytes(include_bytes!("server.key.der").to_vec())
///     .build();
///
/// println!("Certificate length: {} bytes", security.cert().len());
/// ```
#[derive(Clone, Deserialize)]
pub struct SecurityConfig {
    cert: Vec<u8>,
    key: Vec<u8>,
    ca_cert: Option<Vec<u8>>,
    client_auth: bool,
}

impl SecurityConfig {
    /// Creates a new `SecurityConfigBuilder` with default settings.
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
    /// use vetis::config::SecurityConfig;
    ///
    /// let security = SecurityConfig::builder()
    ///     .cert_from_bytes(vec![])
    ///     .key_from_bytes(vec![])
    ///     .build();
    /// ```
    pub fn builder() -> SecurityConfigBuilder {
        SecurityConfigBuilder {
            cert: Vec::new(),
            key: Vec::new(),
            ca_cert: None,
            client_auth: false,
        }
    }

    /// Returns the server certificate bytes.
    ///
    /// # Returns
    ///
    /// * `&Vec<u8>` - The server certificate bytes.
    pub fn cert(&self) -> &Vec<u8> {
        &self.cert
    }

    /// Returns the private key bytes.
    ///
    /// # Returns
    ///
    /// * `&Vec<u8>` - The private key bytes.
    pub fn key(&self) -> &Vec<u8> {
        &self.key
    }

    /// Returns the CA certificate bytes if present.
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<u8>>` - The CA certificate bytes if present.
    pub fn ca_cert(&self) -> &Option<Vec<u8>> {
        &self.ca_cert
    }

    /// Returns whether client authentication is enabled.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether client authentication is enabled.
    pub fn client_auth(&self) -> bool {
        self.client_auth
    }
}

/// Security configuration loaded from files.
#[derive(Clone, Deserialize)]
pub struct SecurityConfigFromFile {
    cert_from_file: String,
    key_from_file: String,
    ca_cert_from_file: Option<String>,
    client_auth: Option<bool>,
}

pub(crate) fn deserialize_security_from_file<'de, D>(
    deserializer: D,
) -> Result<Option<SecurityConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    let security =
        SecurityConfigFromFile::deserialize(deserializer).map_err(serde::de::Error::custom)?;

    let mut builder = SecurityConfig::builder()
        .cert_from_file(&security.cert_from_file)
        .key_from_file(&security.key_from_file);

    if let Some(ca_cert_from_file) = security.ca_cert_from_file {
        builder = builder.ca_cert_from_file(&ca_cert_from_file);
    }

    if let Some(client_auth) = security.client_auth {
        builder = builder.client_auth(client_auth);
    }

    builder
        .build()
        .map_err(serde::de::Error::custom)
        .map(Some)
}
