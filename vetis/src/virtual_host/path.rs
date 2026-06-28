//! Path module for virtual host configuration.

use crate::{errors::VetisError, Request, Response};
use std::{future::Future, pin::Pin, sync::Arc};

/// Trait for handling different types of paths in the server
pub trait Path: Sync + Send {
    /// Returns the URI of the path
    ///
    /// # Returns
    ///
    /// * `&str` - The URI of the path
    fn uri(&self) -> &str;

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
        uri: Arc<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + 'a>>;
}

#[typetag::serde]
/// A trait which describe path configuration
pub trait PathConfig: Send + Sync {
    /// Sets the URI for the path
    ///
    /// # Arguments
    ///
    /// * `value` - The URI to set
    fn uri(&mut self, value: &str);
}
