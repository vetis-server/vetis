use std::{future::Future, pin::Pin, sync::{Arc, OnceLock}};

use serde::Deserialize;

use vetis::{Request, Response, errors::{ConfigError, VetisError, VirtualHostError}};

static CLIENT: OnceLock<Client> = OnceLock::new();

/// Builder for creating `ProxyPathConfig` instances.
#[derive(Deserialize)]
pub struct ProxyPathConfigBuilder {
    uri: String,
    target: String,
}

impl ProxyPathConfigBuilder {
    /// Allow set the URI of the proxy path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = uri.to_string();
        self
    }

    /// Allow set the target of the proxy path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn target(mut self, target: &str) -> Self {
        self.target = target.to_string();
        self
    }

    /// Build the `ProxyPathConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `Result<ProxyPathConfig, VetisError>` - The `ProxyPathConfig` with the configured settings.
    pub fn build(self) -> Result<ProxyPathConfig, VetisError> {
        if self.uri.is_empty() {
            return Err(VetisError::Config(ConfigError::Path("URI cannot be empty".to_string())));
        }
        if self
            .target
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::Path(
                "Target cannot be empty".to_string(),
            )));
        }

        Ok(ProxyPathConfig { uri: self.uri, target: self.target })
    }
}

/// Configuration for a proxy path.
#[derive(Clone, Deserialize)]
pub struct ProxyPathConfig {
    uri: String,
    target: String,
    // TODO: Add custom proxy rules

    // TODO: Add support for custom headers
}

impl ProxyPathConfig {
    /// Creates a new `ProxyPathConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `ProxyPathConfigBuilder` - The builder.
    pub fn builder() -> ProxyPathConfigBuilder {
        ProxyPathConfigBuilder {
            uri: "/test".to_string(),
            target: "http://localhost:8080".to_string(),
        }
    }

    /// Returns the URI of the proxy path.
    ///
    /// # Returns
    ///
    /// * `&str` - The URI of the proxy path.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the target of the proxy path.
    ///
    /// # Returns
    ///
    /// * `&str` - The target of the proxy path.
    pub fn target(&self) -> &str {
        &self.target
    }
}

/// Proxy path
pub struct ProxyPath {
    config: ProxyPathConfig,
}

impl ProxyPath {
    /// Create a new proxy path with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The proxy path configuration
    ///
    /// # Returns
    ///
    /// * `ProxyPath` - The proxy path
    pub fn new(config: ProxyPathConfig) -> ProxyPath {
        ProxyPath { config }
    }
}

impl From<ProxyPath> for HostPath {
    /// Convert proxy path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The proxy path to convert
    ///
    /// # Returns
    ///
    /// * `HostPath` - The host path
    fn from(value: ProxyPath) -> Self {
        HostPath::Proxy(value)
    }
}

impl Path for ProxyPath {
    /// Get the URI of the proxy path
    ///
    /// # Returns
    ///
    /// * `&str` - The URI of the proxy path
    fn uri(&self) -> &str {
        self.config.uri()
    }

    /// Handle proxy request
    ///
    /// # Arguments
    ///
    /// * `request` - The request to handle
    /// * `uri` - The URI of the request
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + '_>>` - The future that will resolve to the response
    fn handle(
        &self,
        request: Request,
        uri: Arc<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + '_>> {
        let (request_parts, request_body) = request.into_parts();

        let target = self.config.target();

        Box::pin(async move {
            let target_url = format!("{}{}", target, uri);
            let deboa_request = match DeboaRequest::at(target_url, request_parts.method) {
                Ok(request) => request,
                Err(e) => {
                    return Err(VetisError::VirtualHost(VirtualHostError::Proxy(e.to_string())))
                }
            };

            let deboa_request = match deboa_request
                .headers(request_parts.headers)
                .body(request_body)
                .build()
            {
                Ok(request) => request,
                Err(e) => {
                    return Err(VetisError::VirtualHost(VirtualHostError::Proxy(e.to_string())))
                }
            };

            let client = CLIENT.get_or_init(|| {
                Client::builder()
                    .pool(HttpConnectionPool::default())
                    .build()
            });

            // TODO: Check errors and handle them properly by returning a proper response 500, 503 or 504
            let response = client
                .execute(deboa_request)
                .await;

            let response = match response {
                Ok(response) => response,
                Err(e) => {
                    return Err(VetisError::VirtualHost(VirtualHostError::Proxy(e.to_string())))
                }
            };

            let (response_parts, response_body) = response.into_parts();

            let vetis_response = Response::builder()
                .status(response_parts.status)
                .headers(response_parts.headers)
                .body(response_body);

            Ok::<Response, VetisError>(vetis_response)
        })
    }
}
