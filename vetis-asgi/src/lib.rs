#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use std::{future::Future, pin::Pin, sync::Arc};

use http::StatusCode;
use hyper_body_utils::HttpBody;

use vetis::{
    errors::VetisError,
    http::{Request, Response},
    virtual_host::path::interface::InterfaceWorker,
};

mod callback;
mod tests;

/// ASGI worker implementation
pub struct AsgiWorker {
    directory: String,
    target: String,
}

impl AsgiWorker {
    /// Create a new ASGI worker
    pub fn new(directory: String, target: String) -> AsgiWorker {
        AsgiWorker { directory, target }
    }

    /// Get the directory
    pub fn directory(&self) -> &String {
        &self.directory
    }

    /// Get the target
    pub fn target(&self) -> &String {
        &self.target
    }
}

impl InterfaceWorker for AsgiWorker {
    fn handle(
        &self,
        request: Arc<Request>,
        _uri: Arc<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'static>> {
        let mut response_body: Option<Vec<u8>> = None;

        Box::pin(async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(HttpBody::from_bytes(&response_body.unwrap())))
        })
    }
}
