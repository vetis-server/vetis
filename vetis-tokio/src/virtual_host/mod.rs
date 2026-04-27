/// # Examples
///
/// ```rust,ignore
/// use vetis::{
///     virtual_host::VirtualHostConfig,
///     server::virtual_host::{VirtualHost, handler_fn},
///     Request, Response,
/// };
///
/// // Create a virtual host with a simple handler
/// let config = VirtualHostConfig::builder()
///     .hostname("example.com")
///     .port(80)
///     .build()?;
///
/// let mut vhost = VirtualHost::new(config);
/// vhost.set_handler(handler_fn(|request: Request| async move {
///     let response = Response::builder()
///         .status(http::StatusCode::OK)
///         .body(http_body_util::Full::new(bytes::Bytes::from("Hello, World!")));
///     Ok(response)
/// }));
/// ```
use std::{future::Future, path::PathBuf, pin::Pin};

use futures_util::TryStreamExt;
use http::StatusCode;
use http_body_util::StreamBody;
use hyper::body::Frame;
use hyper_body_utils::HttpBody;
use radix_trie::Trie;
use std::sync::Arc;
use vetis::{
    errors::{FileError, VetisError, VirtualHostError},
    virtual_host::VirtualHostConfig,
    Request, Response,
};

use crate::virtual_host::path::{HostPath, Path};

use tokio::fs::File;

#[cfg(feature = "static-files")]
use crate::virtual_host::path::static_files::StaticPath;

#[cfg(feature = "reverse-proxy")]
use crate::virtual_host::path::proxy::ProxyPath;

#[cfg(feature = "interface")]
use crate::virtual_host::path::interface::InterfacePath;

pub mod path;

/// Virtual host structure
pub struct VirtualHost {
    config: VirtualHostConfig,
    paths: Trie<String, HostPath>,
}

impl VirtualHost {
    /// Create a new virtual host
    ///
    /// # Arguments
    ///
    /// * `host_config` - A `VirtualHostConfig` instance containing the virtual host configuration.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `VirtualHost` instance.
    pub fn new(host_config: VirtualHostConfig) -> Self {
        let mut host = Self { config: host_config.clone(), paths: Trie::new() };

        #[cfg(feature = "static-files")]
        if let Some(static_paths) = &host_config.static_paths() {
            for static_path in static_paths {
                host.add_path(StaticPath::new(static_path.clone()));
            }
        }

        #[cfg(feature = "reverse-proxy")]
        if let Some(proxy_paths) = &host_config.proxy_paths() {
            for proxy_path in proxy_paths {
                host.add_path(ProxyPath::new(proxy_path.clone()));
            }
        }

        #[cfg(feature = "interface")]
        if let Some(interface_paths) = &host_config.interface_paths() {
            for interface_path in interface_paths {
                host.add_path(InterfacePath::new(interface_path.clone()));
            }
        }

        host
    }

    /// Add a path to the virtual host
    ///
    /// # Arguments
    ///
    /// * `path` - A `HostPath` instance containing the path configuration.
    pub fn add_path<P>(&mut self, path: P)
    where
        P: Into<HostPath>,
    {
        let path = path.into();
        self.paths.insert(
            path.uri()
                .to_string(),
            path,
        );
    }

    /// Returns virtual host configuration
    ///
    /// # Returns
    ///
    /// * `&VirtualHostConfig` - A reference to the virtual host configuration.
    pub fn config(&self) -> &VirtualHostConfig {
        &self.config
    }

    /// Returns virtual host hostname
    ///
    /// # Returns
    ///
    /// * `&str` - A reference to the virtual host hostname.
    pub fn hostname(&self) -> &str {
        self.config
            .hostname()
    }

    /// Returns virtual host port number
    ///
    /// # Returns
    ///
    /// * `u16` - The virtual host port number.
    pub fn port(&self) -> u16 {
        self.config.port()
    }

    /// Returns virtual host security configuration
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the virtual host is secure or not.
    pub fn is_secure(&self) -> bool {
        self.config
            .security()
            .is_some()
    }

    async fn serve_status_page(&self, status: u16) -> Result<Response, VetisError> {
        let status_code = match StatusCode::from_u16(status) {
            Ok(code) => code,
            Err(_) => {
                return Err(VetisError::VirtualHost(VirtualHostError::Interface(
                    "Invalid status code".to_string(),
                )))
            }
        };

        let static_status_response = Response::builder()
            .status(status_code)
            .text(
                status_code
                    .canonical_reason()
                    .unwrap_or("Unknown status code"),
            );

        if let Some(status_pages) = &self
            .config
            .status_pages()
        {
            let root_directory = PathBuf::from(
                self.config
                    .root_directory(),
            );
            if let Some(page) = status_pages.get(&status) {
                let file = root_directory.join(page);
                if file.exists() {
                    let result = File::open(file).await;
                    if let Ok(data) = result {
                        let content = tokio_util::io::ReaderStream::new(data).map_ok(Frame::data);
                        let body = StreamBody::new(content);
                        return Ok(Response::builder()
                            .status(status_code)
                            .body(HttpBody::from_stream(body)));
                    }
                }
            }
        }
        Ok(static_status_response)
    }

    /// Route request to the appropriate handler
    ///
    /// # Arguments
    ///
    /// * `request` - A `Request` instance containing the request information.
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + '_>>` - A pinned box containing the future that will resolve to a `Result<Response, VetisError>`.
    pub fn route(
        &self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + '_>> {
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

        let matches = self
            .paths
            .get_ancestor_value(&uri_path);

        let Some(path) = matches else {
            return Box::pin(async move {
                self.serve_status_page(http::StatusCode::NOT_FOUND.as_u16())
                    .await
            });
        };

        let target_path: String = uri_path
            .strip_prefix(path.uri())
            .unwrap_or(&uri_path)
            .into();

        let result = path.handle(request, Arc::from(target_path));

        Box::pin(async move {
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
