use crate::{errors::ConfigError, host::HostConfig, listener::ListenerConfig};
use ::http::Version;
use serde::Deserialize;

/// Builder for creating `ServerConfig` instances.
///
/// Provides a fluent API for configuring the overall server,
/// including multiple listeners for different ports and protocols.
///
/// # Examples
///
/// ```rust,ignore
/// use vetis::{listener::ListenerConfig, server::{ServerConfig}};
/// use http::Version;
///
/// let http_listener = ListenerConfig::builder()
///     .port(80)
///     .protos(vec![Version::HTTP_11])
///     .build(!)
///     .unwrap();
///
/// let https_listener = ListenerConfig::builder()
///     .port(443)
///     .protos(vec![Version::HTTP_11])
///     .build()
///     .unwrap();
///
/// let config = ServerConfig::builder()
///     .add_listener(http_listener)
///     .add_listener(https_listener)
///     .build();
/// ```
pub struct ServerConfigBuilder {
    hosts: Vec<HostConfig>,
    listeners: Vec<ListenerConfig>,
}

impl ServerConfigBuilder {
    /// Adds a host configuration to the server.
    ///
    /// Multiple host can be added to serve different domains.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{host::HostConfig, server::ServerConfig};
    ///
    /// let host = HostConfig::default();
    /// let config = ServerConfig::builder()
    ///     .add_host(host)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_host(mut self, host: HostConfig) -> Self {
        self.hosts
            .push(host);
        self
    }

    /// Adds a listener configuration to the server.
    ///
    /// Multiple listeners can be added to support different
    /// ports, protocols, or interfaces simultaneously.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{listener::ListenerConfig, server::ServerConfig};
    ///
    /// let listener = ListenerConfig::builder()
    ///     .port(8080)
    ///     .build()
    ///     .unwrap();
    /// let config = ServerConfig::builder()
    ///     .add_listener(listener)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn add_listener(mut self, listener: ListenerConfig) -> Self {
        self.listeners
            .push(listener);
        self
    }

    /// Creates the `ServerConfig` with the configured listeners.
    ///
    /// # Errors
    ///
    /// * Return an error if no listeners are configured or if HTTP/2 and HTTP/3 support enabled and no security setting provided for the host.
    pub fn build(self) -> Result<ServerConfig, ConfigError> {
        if self
            .listeners
            .is_empty()
        {
            return Err(ConfigError::Server("No listeners configured".to_string()));
        }

        let requires_tls = self
            .listeners
            .iter()
            .any(|listener| {
                listener
                    .protos()
                    .contains(&Version::HTTP_2)
                    || listener
                        .protos()
                        .contains(&Version::HTTP_3)
            });

        for host in &self.hosts {
            if requires_tls
                && host
                    .security()
                    .is_none()
            {
                return Err(ConfigError::Server(format!("You enabled HTTP/2 and HTTP/3 support in your listeners, but your hosts doesnt't have TLS configuration provided.")));
            }
        }

        Ok(ServerConfig { hosts: self.hosts, listeners: self.listeners })
    }
}

/// Global server configuration.
///
/// Contains all the hosts that the server should use to accept
/// incoming connections. Each host can have different settings
/// for port, protocol, and interface.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{host::HostConfig, server::ServerConfig};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = ServerConfig::builder()
///         .add_host(HostConfig::builder().hostname("example.com").build()?)
///         .add_host(HostConfig::builder().hostname("sample.com").build()?)
///         .build()?;
///
///     println!("Server has {} hosts", config.hosts().len());
///     Ok(())
/// }
/// ```
#[derive(Default, Deserialize)]
pub struct ServerConfig {
    hosts: Vec<HostConfig>,
    listeners: Vec<ListenerConfig>,
}

impl ServerConfig {
    /// Creates a new `ServerConfigBuilder` with no hosts.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{host::HostConfig, server::ServerConfig};
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let host_config = HostConfig::builder().hostname("example.com").build()?;
    ///     let server_config = ServerConfig::builder()
    ///         .add_host(host_config)
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn builder() -> ServerConfigBuilder {
        ServerConfigBuilder { hosts: vec![], listeners: vec![] }
    }

    /// Returns a reference to all configured hosts.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{host::HostConfig, server::ServerConfig};
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder()
    ///         .add_host(HostConfig::builder().hostname("example.com").build()?)
    ///         .build()?;
    ///
    ///     for host in config.hosts() {
    ///         println!("Hosting on port {}", host.hostname());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn hosts(&self) -> &Vec<HostConfig> {
        &self.hosts
    }

    /// Returns a reference to all configured listeners.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{listener::ListenerConfig, server::ServerConfig};
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ServerConfig::builder()
    ///         .add_listener(ListenerConfig::builder().port(80).build()?)
    ///         .build()?;
    ///
    ///     for listener in config.listeners() {
    ///         println!("Listening on port {}", listener.port());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn listeners(&self) -> &Vec<ListenerConfig> {
        &self.listeners
    }
}

///! HTTP module
pub mod http {
    use crate::{errors::VetisError, host::Host, Request, VetisFutureResult, VetisHosts};
    use http::{HeaderName, HeaderValue, StatusCode};
    use hyper::{body::Incoming, service::Service};
    use hyper_body_utils::HttpBody;
    use log::{debug, error, info};
    use std::net::SocketAddr;

    /// HttpService is responsible for process HTTP1 and HTTP2 client requests
    pub struct HttpService<H> {
        hosts: VetisHosts<H>,
        client_addr: SocketAddr,
    }

    impl<H> HttpService<H>
    where
        H: Host,
    {
        /// Create a new HttpService
        pub fn new(hosts: VetisHosts<H>, client_addr: SocketAddr) -> Self {
            HttpService { hosts: hosts.clone(), client_addr }
        }
    }

    impl<H> Service<http::request::Request<Incoming>> for HttpService<H>
    where
        H: Host + Sync + Send + 'static,
    {
        type Response = http::response::Response<HttpBody>;

        type Error = VetisError;

        type Future = VetisFutureResult<'static, Self::Response>;

        fn call(&self, req: http::request::Request<Incoming>) -> Self::Future {
            let hosts = self.hosts.clone();
            let client_addr = self
                .client_addr
                .clone();
            let future = async move {
                let Some(authority) = req
                    .uri()
                    .authority()
                else {
                    error!("Host not found in request");
                    let response = crate::Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .text("Host not found in request")
                        .into_inner();
                    return Ok(response);
                };

                let hostname = authority.host();

                debug!("Serving request for host: {}", hostname);
                let hosts = hosts.read().await;

                let host = hosts.get(hostname);

                if let Some(host) = host {
                    // TODO: Save client_addr in request, grab url from request for logging
                    let (parts, body) = req.into_parts();
                    let request = Request::from_parts(parts, HttpBody::from_incoming(body));

                    let method = request
                        .method()
                        .clone();

                    let uri = request
                        .uri()
                        .clone();

                    let vetis_response = host
                        .route(request)
                        .await?;

                    let mut response = vetis_response.into_inner();

                    let default_headers = host
                        .config()
                        .default_headers();

                    if let Some(default_headers) = default_headers {
                        for (key, value) in default_headers {
                            let Ok(header_name) = HeaderName::from_bytes(key.as_bytes()) else {
                                error!("Invalid header name: {}", key);
                                continue;
                            };

                            let Ok(header_value) = HeaderValue::from_str(value) else {
                                error!("Invalid header value: {}", value);
                                continue;
                            };

                            response
                                .headers_mut()
                                .insert(header_name, header_value);
                        }
                    }

                    // TODO: Log request and its response status code (move it to oneshot channel?)
                    info!("{} {} {} {}", client_addr, method, uri, response.status());

                    Ok::<http::Response<HttpBody>, VetisError>(response)
                } else {
                    error!("Host not found: {}", hostname);
                    let response = crate::Response::builder()
                        .status(StatusCode::BAD_GATEWAY)
                        .text("Host not found")
                        .into_inner();
                    Ok(response)
                }
            };

            Box::pin(future)
        }
    }
}
