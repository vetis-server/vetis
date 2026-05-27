use deboa::request::get;
use deboa_tokio::{
    cert::{Certificate, ContentEncoding},
    Client,
};
use vetis::{virtual_host::handler_fn, Response};
use vetis_macros::{http, security};

use crate::common::{deboa_default_protocol, vetis_default_protocol};

async fn do_test_http() -> Result<(), Box<dyn std::error::Error>> {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });

    let mut server = http!(
        from_crate => vetis_tokio,
        hostname => "localhost",
        root_directory => "src",
        protocol => vetis_default_protocol(),
        port => 8080,
        interface => "0.0.0.0",
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

    let certificate = Certificate::from_file("../certs/ca.der", ContentEncoding::DER)?;

    let client = Client::builder()
        .protocol(deboa_default_protocol())
        .certificate(certificate)
        .build();

    let response = get("http://localhost:8080")?
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
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    do_test_http().await
}
