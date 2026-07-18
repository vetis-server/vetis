use crate::errors::VetisError;
use futures_util::future::BoxFuture;
use http::HeaderMap;

/// A trait for authentication methods.
pub trait Auth {
    /// Authenticate method takes a reference to a `HeaderMap` and returns a `Result<bool, VetisError>`.
    ///
    /// # Arguments
    ///
    /// * `headers` - A reference to a `HeaderMap` containing the request headers.
    ///
    /// # Returns
    ///
    /// * `Result<bool, VetisError>` - A result containing a boolean indicating whether the authentication was successful, or a `VetisError` if the authentication failed.
    fn authenticate(&self, headers: &HeaderMap) -> BoxFuture<'_, Result<bool, VetisError>>;
}
