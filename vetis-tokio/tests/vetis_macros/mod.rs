use crate::common::default_protocol_version;
use deboa::{
    cert::{CertificateExt as _, ContentEncoding},
    request::get,
};
use deboa_tokio::{cert::DeboaCertificate, Client};
use std::net::Ipv4Addr;
use vetis::{host::handler_fn, Response, VetisServer as _};
use vetis_macros::{http, security};

#[cfg(feature = "http1")]
#[tokio::test]
async fn test_http_localhost() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = http!(
        from_crate => vetis_tokio,
        port => 60002,
        protos => vec![default_protocol_version()],
        handler => handler_fn(
            |_req| async move { Ok(Response::builder().text("Hello, World!")) }
        )
    )
    .await?;

    server
        .start()
        .await?;

    let client = Client::builder().build();

    let response = get("http://localhost:60002")?
        .send_with(&client)
        .await?;

    assert_eq!(response.status(), 200);
    assert_eq!(
        response
            .text()
            .await?,
        "Hello, World!"
    );

    server
        .stop()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_https() -> Result<(), Box<dyn std::error::Error>> {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });

    let mut server = http!(
        from_crate => vetis_tokio,
        hostname => "localhost",
        root_directory => "src".into(),
        protos => vec![default_protocol_version()],
        port => 60001,
        interface => Ipv4Addr::UNSPECIFIED.into(),
        handler => handler,
        security_config => security! {
            cert => "../certs/server.der",
            key => "../certs/server.key.der",
            ca_cert => "../certs/ca.der",
            client_auth => false
        }
    )
    .await?;

    server
        .start()
        .await?;

    let certificate = DeboaCertificate::from_file("../certs/ca.der", ContentEncoding::DER).await?;

    let client = Client::builder()
        .certificate(certificate)
        .build();

    let response = get("https://localhost:60001")?
        .version(default_protocol_version())
        .send_with(&client)
        .await?;

    assert_eq!(response.status(), 200);
    assert_eq!(
        response
            .text()
            .await?,
        "Hello, World!"
    );

    server
        .stop()
        .await?;

    Ok(())
}
