//! Path module for virtual host configuration.
use crate::{Request, Response, VetisFutureResult, host::HostContext};

/// Trait for handling different types of paths in the server
pub trait Path {
    /// Returns the URI of path
    ///
    /// # Returns
    ///
    /// * `&str` - The URI of path
    fn uri(&self) -> &str;

    /// Handles the request for path
    ///
    /// # Arguments
    ///
    /// * `request` - The request to handle
    /// * `host_context` - The host context
    ///
    /// # Returns
    ///
    /// * `VetisFutureResult<'a, Response>` - The future that will handle the request
    fn handle<'a>(
        &'a self,
        request: Request,
        host_context: HostContext,
    ) -> VetisFutureResult<'a, Response>
    where
        Self: Send;
}

#[typetag::serde(tag = "type")]
/// A trait which describe path configuration
pub trait PathConfig {
    /// Sets the URI for the path
    ///
    /// # Arguments
    ///
    /// * `value` - The URI to set
    fn uri(&mut self, value: &str);

    /// Allow clone any typw which implements
    /// PathConfig
    ///
    /// # Returns
    ///
    /// * `Box<dyn PathConfig>` - Clone of boxed path config
    fn boxed_clone(&self) -> Box<dyn PathConfig>;

    /// Returns a boxed path out of it
    ///
    /// # Returns
    ///
    /// * `Box<dyn Path>` - A boxed path
    fn boxed_path(&self) -> Box<dyn Path>;

    /// Returns a boxed path out of it
    ///
    /// # Returns
    ///
    /// * `Box<dyn Path>` - A boxed path
    fn boxed_sync(&self) -> Box<dyn Path + Send + Sync>;
}

impl Clone for Box<dyn PathConfig> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}
