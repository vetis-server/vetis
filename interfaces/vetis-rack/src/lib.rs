#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use std::{future::Future, pin::Pin, sync::Arc};

use http::StatusCode;
use hyper_body_utils::HttpBody;
use vetis::{
    errors::VetisError, virtual_host::path::interface::InterfaceWorker, Request, Response,
};

mod callback;
mod tests;

/// Rack worker implementation
pub struct RackWorker {
    directory: String,
    target: String,
}

impl RackWorker {
    /// Create a new Rack worker
    pub fn new(directory: String, target: String) -> RackWorker {
        RackWorker { directory, target }
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

impl InterfaceWorker for RackWorker {
    fn handle(
        &self,
        _request: Arc<Request>,
        _uri: Arc<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'static>> {
        Box::pin(async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(HttpBody::from_text("Ok!")))
        })
    }
}
