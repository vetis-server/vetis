//! Path module for virtual host configuration.
use crate::{Request, Response, VetisFutureResult};
use std::sync::Arc;

/// Trait for handling different types of paths in the server
pub trait Path: Send + Sync {
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
    /// * `VetisFutureResult<'a, Response>` - The future that will handle the request
    fn handle<'a>(&'a self, request: Request, uri: Arc<String>) -> VetisFutureResult<'a, Response>;
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
