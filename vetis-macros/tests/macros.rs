use http::Version;
use std::collections::HashMap;
use vetis::{host::handler_fn, security::SecurityConfig, Response, VetisServer};
use vetis_macros::{http, security, status_pages};

#[test]
fn status_pages_expands_to_a_map() {
    let pages = status_pages! {
        404 @ "not-found.html".to_owned(),
        500 @ String::from("server-error.html"),
    };

    let expected =
        HashMap::from([(404, "not-found.html".to_owned()), (500, "server-error.html".to_owned())]);

    assert_eq!(pages, expected);
}

#[test]
fn security_expands_with_default_client_auth() -> Result<(), vetis::errors::VetisError> {
    let config: SecurityConfig = security! {
        cert => concat!(env!("CARGO_MANIFEST_DIR"), "/../certs/server.der"),
        key => concat!(env!("CARGO_MANIFEST_DIR"), "/../certs/server.key.der"),
        ca_cert => concat!(env!("CARGO_MANIFEST_DIR"), "/../certs/ca.der"),
    };

    assert!(!config.client_auth());
    assert!(!config
        .cert()
        .is_empty());
    assert!(!config
        .key()
        .is_empty());

    Ok(())
}

#[test]
fn security_expands_with_client_auth_enabled() -> Result<(), vetis::errors::VetisError> {
    let config: SecurityConfig = security! {
        cert => concat!(env!("CARGO_MANIFEST_DIR"), "/../certs/server.der"),
        key => concat!(env!("CARGO_MANIFEST_DIR"), "/../certs/server.key.der"),
        ca_cert => concat!(env!("CARGO_MANIFEST_DIR"), "/../certs/ca.der"),
        client_auth => true,
    };

    assert!(config.client_auth());

    Ok(())
}

#[tokio::test]
async fn http_expands_with_requirements() -> Result<(), vetis::errors::VetisError> {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });
    let mut http = http! {
        from_crate => vetis_tokio,
        handler => handler,
        port => 8080,
        protos => vec![Version::HTTP_11]
    }
    .await?;

    http.start().await?;

    Ok(())
}

#[cfg(target_os = "linux")]
#[tokio::test]
#[should_panic = "Bind(\"Permission denied (os error 13)\")"]
async fn http_expands_with_permission_denied() {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });
    let mut http = http! {
        from_crate => vetis_tokio,
        handler => handler,
        protos => vec![Version::HTTP_11]
    }
    .await
    .unwrap();

    http.start()
        .await
        .unwrap();
}
