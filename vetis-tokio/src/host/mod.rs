//! Virtual host module
//!
//! This module provides functionality for creating and managing virtual hosts,
//! including path routing and request handling.
use futures_util::TryStreamExt;
use http::StatusCode;
use http_body_util::StreamBody;
use hyper::body::Frame;
use hyper_body_utils::HttpBody;
use radix_trie::Trie;
use std::sync::Arc;
use tokio::fs::File;
use vetis::{
    errors::{FileError, HostError, VetisError},
    host::{path::Path, Host, HostConfig},
    Request, Response, VetisFutureResult,
};

pub mod path;

/// Virtual host structure
pub struct HostImpl {
    config: HostConfig,
    paths: Trie<String, Arc<Box<dyn Path>>>,
}

impl Host for HostImpl {
    fn paths(&self) -> Trie<String, Arc<Box<dyn vetis::host::path::Path>>> {
        self.paths.clone()
    }

    fn config(&self) -> &HostConfig {
        &self.config
    }

    fn serve_status_page<'a>(&'a self, status: u16) -> VetisFutureResult<'a, Response> {
        let future = async move {
            let status_code = match StatusCode::from_u16(status) {
                Ok(code) => code,
                Err(_) => {
                    return Err(VetisError::Host(HostError::Interface(
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
                if let Some(page) = status_pages.get(&status) {
                    if let Some(dir) = self
                        .config
                        .root_directory()
                    {
                        let file = dir.join(page);
                        if dir.exists() {
                            let result = File::open(file).await;
                            if let Ok(data) = result {
                                let content =
                                    tokio_util::io::ReaderStream::new(data).map_ok(Frame::data);
                                let body = StreamBody::new(content);
                                return Ok(Response::builder()
                                    .status(status_code)
                                    .body(HttpBody::from_generic_stream(body)));
                            }
                        }
                    }
                }
            }
            Ok(static_status_response)
        };
        Box::pin(future)
    }

    /// Route request to the appropriate handler
    ///
    /// # Arguments
    ///
    /// * `request` - A `Request` instance containing the request information.
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send>>` - A pinned box containing the future that will resolve to a `Result<Response, VetisError>`.
    fn route<'a>(&'a self, request: Request) -> VetisFutureResult<'a, Response>
    where
        Self: Sync,
    {
        let uri_path: String = request
            .uri()
            .path()
            .into();

        if uri_path.starts_with("..") {
            return self.serve_status_page(http::StatusCode::FORBIDDEN.as_u16());
        }

        let paths = self.paths();

        let matches = paths.get_ancestor_value(&uri_path);

        let Some(path) = matches else {
            return self.serve_status_page(http::StatusCode::NOT_FOUND.as_u16());
        };

        let path = path.clone();

        let target_path: String = uri_path
            .strip_prefix(path.uri())
            .unwrap_or(&uri_path)
            .into();

        let future = async move {
            let result = path.handle(request, Arc::from(target_path));
            match result.await {
                Ok(response) => Ok(response),
                Err(error) => {
                    match error {
                        VetisError::Host(HostError::File(FileError::NotFound)) => {
                            log::error!("Invalid path: {}", error);
                            return self
                                .serve_status_page(http::StatusCode::NOT_FOUND.as_u16())
                                .await;
                        }
                        VetisError::Host(HostError::Proxy(ref error)) => {
                            log::error!("Proxy error: {}", error);
                            return self
                                .serve_status_page(http::StatusCode::BAD_GATEWAY.as_u16())
                                .await;
                        }
                        VetisError::Host(HostError::Auth(e)) => {
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
        };

        Box::pin(future)
    }
}

impl HostImpl {
    /// Create a new virtual host
    ///
    /// # Arguments
    ///
    /// * `host_config` - A `HostConfig` instance containing the virtual host configuration.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `Host` instance.
    pub fn new(host_config: HostConfig) -> Self {
        Self { config: host_config, paths: Trie::new() }
    }

    /// Add a path to the virtual host
    ///
    /// # Arguments
    ///
    /// * `path` - A `HostPath` instance containing the path configuration.
    pub fn add_path<P>(&mut self, path: P)
    where
        P: Path + 'static,
    {
        self.paths.insert(
            path.uri()
                .to_string(),
            Arc::new(Box::new(path)),
        );
    }
}
