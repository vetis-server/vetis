//! Virtual host module
//!
//! This module provides functionality for creating and managing virtual hosts,
//! including path routing and request handling.
use std::{future::Future, path::PathBuf, pin::Pin};

use futures_util::TryStreamExt;
use http::StatusCode;
use http_body_util::StreamBody;
use hyper::body::Frame;
use hyper_body_utils::HttpBody;
use radix_trie::Trie;
use std::sync::Arc;
use vetis::{
    errors::{VetisError, VirtualHostError},
    virtual_host::{path::Path, VirtualHost, VirtualHostConfig},
    Response,
};

use tokio::fs::File;

pub mod path;

/// Virtual host structure
pub struct VirtualHostImpl {
    config: VirtualHostConfig,
    paths: Trie<String, Arc<Box<dyn Path>>>,
}

impl VirtualHost for VirtualHostImpl {
    fn paths(&self) -> Trie<String, Arc<Box<dyn vetis::virtual_host::path::Path>>> {
        self.paths.clone()
    }

    fn config(&self) -> &VirtualHostConfig {
        &self.config
    }

    fn serve_status_page<'a>(
        &'a self,
        status: u16,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'a>> {
        let future = async move {
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
                            let content =
                                tokio_util::io::ReaderStream::new(data).map_ok(Frame::data);
                            let body = StreamBody::new(content);
                            return Ok(Response::builder()
                                .status(status_code)
                                .body(HttpBody::from_stream(body)));
                        }
                    }
                }
            }
            Ok(static_status_response)
        };
        Box::pin(future)
    }
}

impl VirtualHostImpl {
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
