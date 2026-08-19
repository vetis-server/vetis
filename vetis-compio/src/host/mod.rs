//! Virtual host module
//!
//! This module provides functionality for creating and managing virtual hosts,
//! including path routing and request handling.
use compio::{fs::File, io::AsyncReadExt};
use http::StatusCode;
use http_body_util::StreamBody;
use hyper_body_utils::HttpBody;
use radix_trie::Trie;
use send_wrapper::SendWrapper;
use std::{io::Cursor, sync::Arc};
use vetis::VetisFutureResult;
use vetis::{
    errors::{FileError, HostError, VetisError},
    host::{path::Path, Host, HostConfig},
    Request, Response,
};

pub mod path;

/// Host structure
pub struct HostImpl {
    /// Host configuration
    config: HostConfig,
    /// Trie of paths
    paths: Trie<String, Arc<Box<dyn Path>>>,
}

impl HostImpl {
    /// Create a new host
    ///
    /// # Arguments
    ///
    /// * `host_config` - A `HostConfig` instance containing the host configuration.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `HostImpl` instance.
    pub fn new(host_config: HostConfig) -> Self {
        Self { config: host_config, paths: Trie::new() }
    }

    /// Add a path to the host
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
                                let content = Cursor::new(data)
                                    .read_only()
                                    .bytes();
                                let body = StreamBody::new(SendWrapper::new(content));
                                return Ok(Response::builder()
                                    .status(status_code)
                                    .body(HttpBody::from_compio_stream(body)));
                            }
                        }
                    }
                }
            }
            Ok(static_status_response)
        };

        Box::pin(SendWrapper::new(future))
    }

    /// Route request to the appropriate handler
    ///
    /// # Arguments
    ///
    /// * `request` - A `Request` instance containing the request information.
    ///
    /// # Returns
    ///
    /// * `VetisFutureResult<'a, Response>` - A pinned box containing the future that will resolve to a `Result<Response, VetisError>`.
    fn route<'a>(&'a self, request: Request) -> VetisFutureResult<'a, Response> {
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
                        VetisError::Host(HostError::File(FileError::InvalidMetadata)) => {
                            log::error!("Invalid file metadata: {}", error);
                            return self
                                .serve_status_page(http::StatusCode::BAD_REQUEST.as_u16())
                                .await;
                        }
                        VetisError::Host(HostError::File(FileError::InvalidRange)) => {
                            log::error!("Invalid file range: {}", error);
                            return self
                                .serve_status_page(http::StatusCode::RANGE_NOT_SATISFIABLE.as_u16())
                                .await;
                        }
                        VetisError::Host(HostError::Interface(e)) => {
                            log::error!("Interface error: {}", e);
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

        Box::pin(SendWrapper::new(future))
    }
}
