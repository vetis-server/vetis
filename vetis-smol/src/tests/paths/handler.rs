use crate::{
    tests::{CA_CERT, SERVER_CERT, SERVER_KEY, deboa_default_protocol, vetis_default_protocol}, virtual_host::{VirtualHostImpl, path::HandlerPath},
};
use deboa::request;
use deboa_smol::{cert::Certificate, cert::ContentEncoding, Client};
use http::StatusCode;
use macro_rules_attribute::apply;
use rand::random_range;
use smol_macros::test;
use std::error::Error;
use vetis::{
    listener::ListenerConfig,
    security::SecurityConfig,
    server::ServerConfig,
    virtual_host::{handler_fn, VirtualHostConfig},
    Response, Vetis as _,
};

async fn do_test_handler() -> Result<(), Box<dyn Error>> {
    let port = random_range(9000..=20000);
    let ipv4 = ListenerConfig::builder()
        .port(port)
        .protocol(vetis_default_protocol())
        .interface("0.0.0.0")
        .build()?;

    let config = ServerConfig::builder()
        .add_listener(ipv4)
        .build()?;

    let security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build()?;

    let localhost_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .root_directory("src/tests")
        .port(port)
        .security(security_config)
        .build()?;

    let mut localhost_virtual_host = VirtualHostImpl::new(localhost_config);

    let root_path = HandlerPath::builder()
        .uri("/hello")
        .handler(handler_fn(|_request| async move {
            let response = Response::builder()
                .status(StatusCode::OK)
                .text("Hello from localhost");
            Ok(response)
        }))
        .build()?;

    localhost_virtual_host.add_path(root_path);

    let mut server = crate::Vetis::new(config);
    server
        .add_virtual_host(localhost_virtual_host)
        .await;

    server
        .start()
        .await?;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .protocol(deboa_default_protocol())
        .build();

    let request = request::get(format!("https://localhost:{}{}", port, "/hello"))?
        .send_with(&client)
        .await?;

    assert_eq!(request.status(), StatusCode::OK);

    server
        .stop()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_handler() -> Result<(), Box<dyn Error>> {
    do_test_handler().await
}
