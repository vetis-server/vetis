use crate::{Request, Response, VetisFutureResult};

/// A trait for authentication methods.
pub trait Auth {
    /// Authenticate method takes a reference to a `Request` and returns a `VetisFutureResult<'a, bool>`.
    ///
    /// # Arguments
    ///
    /// * `request` - A reference to a `Request.
    ///
    /// # Returns
    ///
    /// * `VetisFutureResult<'a, bool>` - A result containing a boolean indicating whether the authentication
    ///    was successful, or a `VetisError` if the authentication failed.
    fn authenticate<'a>(&self, request: &'a Request) -> VetisFutureResult<'a, Response>;
}

#[typetag::serde(tag = "method")]
/// A trait which describe path configuration
pub trait AuthConfig {
    /// Allow clone any typw which implements
    /// AuthConfig
    ///
    /// # Returns
    ///
    /// * `Box<dyn AuthConfig>` - Clone of boxed path config
    fn boxed_clone(&self) -> Box<dyn AuthConfig>;

    /// Returns a boxed path out of it
    ///
    /// # Returns
    ///
    /// * `Box<dyn Auth>` - A boxed path
    fn boxed_auth(&self) -> Box<dyn Auth>;

    /// Returns a boxed path out of it
    ///
    /// # Returns
    ///
    /// * `Box<dyn Auth>` - A boxed path
    fn boxed_sync(&self) -> Box<dyn Auth + Send + Sync>;
}

impl Clone for Box<dyn AuthConfig> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}
