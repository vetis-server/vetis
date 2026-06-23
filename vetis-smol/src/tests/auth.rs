use crate::virtual_host::path::auth::basic_auth::BasicAuth;
use http::HeaderMap;
use smol_macros::test;
use std::error::Error;
use vetis::virtual_host::path::auth::{Algorithm, BasicAuthConfig};
use vetis::{
    errors::{VetisError, VirtualHostError},
    virtual_host::path::auth::{Auth, AuthType},
};

#[test]
fn test_auth_type_basic() {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::BCrypt)
        .build()
        .unwrap();

    let basic_auth = BasicAuth::new(config);
    let auth_type = AuthType::Basic(basic_auth);

    let _ = auth_type;
}

#[smol_macros::test]
async fn test_authenticate_missing_header() -> Result<(), Box<dyn Error>> {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::BCrypt)
        .build()
        .unwrap();

    let basic_auth = BasicAuth::new(config);
    let auth_type = AuthType::Basic(basic_auth);

    let headers = HeaderMap::new();

    let result = auth_type
        .authenticate(&headers)
        .await;

    assert!(result.is_err());
    if let Err(VetisError::VirtualHost(VirtualHostError::Auth(msg))) = result {
        assert!(msg.contains("Missing Authorization header"));
    } else {
        panic!("Expected auth error");
    }

    Ok(())
}

#[smol_macros::test]
async fn test_authenticate_invalid_header_format() -> Result<(), Box<dyn Error>> {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::BCrypt)
        .build()
        .unwrap();

    let basic_auth = BasicAuth::new(config);
    let auth_type = AuthType::Basic(basic_auth);

    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        "InvalidFormat"
            .parse()
            .unwrap(),
    );

    let result = auth_type
        .authenticate(&headers)
        .await;

    assert!(result.is_err());

    Ok(())
}

#[smol_macros::test]
async fn test_authenticate_non_basic_auth() -> Result<(), Box<dyn Error>> {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::BCrypt)
        .build()
        .unwrap();

    let basic_auth = BasicAuth::new(config);
    let auth_type = AuthType::Basic(basic_auth);

    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        "Bearer token"
            .parse()
            .unwrap(),
    );

    let result = auth_type
        .authenticate(&headers)
        .await;

    assert!(result.is_err());
    if let Err(VetisError::VirtualHost(VirtualHostError::Auth(msg))) = result {
        assert!(msg.contains("Expected basic authentication"));
    } else {
        panic!("Expected auth error");
    }

    Ok(())
}

#[smol_macros::test]
async fn test_authenticate_invalid_base64() -> Result<(), Box<dyn Error>> {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::BCrypt)
        .build()
        .unwrap();

    let basic_auth = BasicAuth::new(config);
    let auth_type = AuthType::Basic(basic_auth);

    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        "Basic !!!invalid!!!"
            .parse()
            .unwrap(),
    );

    let result = auth_type
        .authenticate(&headers)
        .await;

    assert!(result.is_err());

    Ok(())
}

#[smol_macros::test]
async fn test_authenticate_invalid_utf8() -> Result<(), Box<dyn Error>> {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::BCrypt)
        .build()
        .unwrap();

    let basic_auth = BasicAuth::new(config);
    let auth_type = AuthType::Basic(basic_auth);

    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        "Basic AA=="
            .parse()
            .unwrap(),
    );

    let result = auth_type
        .authenticate(&headers)
        .await;

    assert!(result.is_err());

    Ok(())
}

#[smol_macros::test]
async fn test_authenticate_invalid_credentials_format() -> Result<(), Box<dyn Error>> {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::BCrypt)
        .build()
        .unwrap();

    let basic_auth = BasicAuth::new(config);
    let auth_type = AuthType::Basic(basic_auth);

    let mut headers = HeaderMap::new();
    headers.insert(
        "authorization",
        "Basic dGVzdA=="
            .parse()
            .unwrap(),
    );

    let result = auth_type
        .authenticate(&headers)
        .await;

    assert!(result.is_err());
    if let Err(VetisError::VirtualHost(VirtualHostError::Auth(msg))) = result {
        assert!(msg.contains("Invalid credentials"));
    } else {
        panic!("Expected auth error");
    }

    Ok(())
}

#[test]
fn test_basic_auth_new() {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::BCrypt)
        .build()
        .unwrap();

    let basic_auth = BasicAuth::new(config);

    let _ = basic_auth;
}

#[test]
fn test_basic_auth_clone() {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::Argon2)
        .build()
        .unwrap();

    let basic_auth = BasicAuth::new(config);
    let cloned = basic_auth.clone();

    let _ = (basic_auth, cloned);
}
