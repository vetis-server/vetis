use crate::common::default_protocol_version;
use deboa::{
    cert::{CertificateExt as _, ContentEncoding},
    request::get,
};
use deboa_smol::{cert::DeboaCertificate, Client};
use macro_rules_attribute::apply;
use smol_macros::test;
use vetis::{host::handler_fn, Response, VetisServer as _};
use vetis_macros::{http, security};

#[cfg(feature = "http1")]
#[apply(test!)]
async fn test_http_localhost() -> Result<(), Box<dyn std::error::Error>> {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });

    let mut server = http!(
        from_crate => vetis_smol,
        port => 8888,
        handler => handler,
        protos => vec![default_protocol_version()],
    )
    .await?;

    server
        .start()
        .await?;

    let client = Client::builder().build();

    let response = get("http://localhost:8888")?
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

#[apply(test!)]
async fn test_https() -> Result<(), Box<dyn std::error::Error>> {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });

    let mut server = http!(
        from_crate => vetis_smol,
        hostname => "localhost",
        root_directory => "src".into(),
        protos => vec![default_protocol_version()],
        port => 60000,
        interface => "0.0.0.0".parse().unwrap(),
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

    let response = get("https://localhost:60000")?
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
