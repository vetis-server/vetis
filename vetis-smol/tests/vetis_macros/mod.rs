use crate::common::{deboa_default_protocol, vetis_default_protocol};
use deboa::{
    cert::{Certificate, ContentEncoding},
    request::get,
};
use deboa_smol::Client;
use macro_rules_attribute::apply;
use smol_macros::test;
use vetis::{virtual_host::handler_fn, Response, Vetis as _};
use vetis_macros::{http, security};

#[cfg(feature = "http1")]
#[apply(test!)]
async fn test_http_localhost() -> Result<(), Box<dyn std::error::Error>> {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });

    let mut server = http!(
        from_crate => vetis_smol,
        port => 8888,
        handler => handler,
        protocol => vetis_default_protocol()
    )
    .await?;

    server
        .start()
        .await?;

    let client = Client::builder()
        .protocol(deboa_default_protocol())
        .build();

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
        root_directory => "src",
        protocol => vetis_default_protocol(),
        port => 60000,
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
