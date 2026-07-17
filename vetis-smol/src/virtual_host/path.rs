//! Path module for handling different types of paths in the server
use std::sync::Arc;
use std::{future::Future, pin::Pin};
use vetis::{
    errors::{HandlerError, VetisError, VirtualHostError},
    virtual_host::{path::Path, BoxedHandlerClosure},
    Request, Response,
};

/// Builder for handler path
pub struct HandlerPathBuilder {
    uri: Arc<String>,
    handler: Option<BoxedHandlerClosure>,
}

impl HandlerPathBuilder {
    /// Allow set handler uri path
    ///
    /// # Arguments
    ///
    /// * `uri` - The uri of the handler path
    ///
    /// # Returns
    ///
    /// * `Self` - The builder
    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = Arc::from(uri.to_string());
        self
    }

    /// Allow set handler function
    ///
    /// # Arguments
    ///
    /// * `handler` - The handler function
    ///
    /// # Returns
    ///
    /// * `Self` - The builder
    pub fn handler(mut self, handler: BoxedHandlerClosure) -> Self {
        self.handler = Some(handler);
        self
    }

    /// Build the handler path
    ///
    /// # Returns
    ///
    /// * `Result<HostPath, VetisError>` - The handler path or error
    pub fn build(self) -> Result<HandlerPath, VetisError> {
        if self.uri.is_empty() {
            return Err(VetisError::VirtualHost(VirtualHostError::Handler(HandlerError::Uri(
                "URI cannot be empty".to_string(),
            ))));
        }

        let handler = match self.handler {
            Some(handler) => handler,
            None => {
                return Err(VetisError::VirtualHost(VirtualHostError::Handler(
                    HandlerError::Handler("Handler must be set".to_string()),
                )))
            }
        };

        Ok(HandlerPath { uri: self.uri, handler })
    }
}

/// Handler path
pub struct HandlerPath {
    uri: Arc<String>,
    handler: BoxedHandlerClosure,
}

impl HandlerPath {
    /// Allow create a new handler path builder
    ///
    /// # Returns
    ///
    /// * `HandlerPathBuilder` - The builder
    pub fn builder() -> HandlerPathBuilder {
        HandlerPathBuilder { uri: Arc::from("/".to_string()), handler: None }
    }
}

impl Path for HandlerPath {
    /// Allow get handler uri path
    ///
    /// # Returns
    ///
    /// * `&str` - The uri of the handler path
    fn uri(&self) -> &str {
        self.uri.as_ref()
    }

    /// Handles the request for the path
    ///
    /// # Arguments
    ///
    /// * `request` - The request to handle
    /// * `uri` - The URI of the path
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + '_>>` - The future that will handle the request
    fn handle<'a>(
        &'a self,
        request: Request,
        _uri: Arc<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'a>> {
        (self.handler)(request)
    }
}
