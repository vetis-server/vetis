use std::future::Future;

use crate::server::virtual_host::path::auth::basic_auth::BasicAuth;

use http::HeaderMap;

use serde::Deserialize;
use vetis_core::errors::VetisError;

pub mod basic_auth;

#[derive(Clone, Deserialize)]
/// An enum with authentication configuration.
pub enum AuthType {
    Basic(BasicAuth),
}

impl Auth for AuthType {
    fn authenticate(&self, headers: &HeaderMap) -> impl Future<Output = Result<bool, VetisError>> {
        match self {
            AuthType::Basic(auth) => auth.authenticate(headers),
        }
    }
}
