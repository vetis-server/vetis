use http::Version;
use std::collections::HashMap;
use vetis::{Response, VetisServer, security::TlsConfig};
use vetis_macros::{http, status_pages, tls};
use vetis_tokio::host::path::handler_fn;

#[test]
fn status_pages_expands_to_a_map() {
    let pages = status_pages! {
        404 @ "not-found.html",
        500 @ "server-error.html",
    };

    let expected =
        HashMap::from([(404, "not-found.html".into()), (500, "server-error.html".into())]);

    assert_eq!(pages, expected);
}

#[test]
fn security_expands_with_default_client_auth() -> Result<(), vetis::errors::VetisError> {
    let config: TlsConfig = tls! {
        cert => "../certs/server.der",
        key => "../certs/server.key.der",
        ca_cert => "../certs/ca.der",
    };

    assert!(!config.client_auth());
    assert_eq!(config.cert_file(), "../certs/server.der");
    assert_eq!(config.key_file(), "../certs/server.key.der");

    Ok(())
}

#[test]
fn security_expands_with_client_auth_enabled() -> Result<(), vetis::errors::VetisError> {
    let config: TlsConfig = tls! {
        cert => "../certs/server.der",
        key => "../certs/server.key.der",
        ca_cert => "../certs/ca.der",
        client_auth => true,
    };

    assert!(config.client_auth());

    Ok(())
}

#[tokio::test]
async fn http_expands_with_requirements() -> Result<(), vetis::errors::VetisError> {
    let handler =
        handler_fn(|_req, _ctx| async move { Ok(Response::builder().text("Hello, World!")) });
    let mut http = http! {
        from_crate => vetis_tokio,
        handler => handler,
        port => 8080,
        protos => &[Version::HTTP_11]
    }
    .await?;

    http.start().await?;

    Ok(())
}
