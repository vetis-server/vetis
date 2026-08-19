use crate::{
    host::{path::HandlerPath, HostImpl},
    tests::{default_protocol_version, CA_CERT, SERVER_CERT, SERVER_KEY},
};
use deboa::{
    cert::{CertificateExt, ContentEncoding},
    request,
};
use deboa_smol::{cert::DeboaCertificate, Client};
use http::StatusCode;
use macro_rules_attribute::apply;
use rand::random_range;
use smol_macros::test;
use std::error::Error;
use vetis::{
    host::{handler_fn, HostConfig},
    listener::ListenerConfig,
    security::SecurityConfig,
    server::ServerConfig,
    Response, VetisServer as _,
};

#[apply(test!)]
async fn test_handler() -> Result<(), Box<dyn Error>> {
    let port = random_range(9000..=20000);
    let ipv4 = ListenerConfig::builder()
        .port(port)
        .protos(vec![default_protocol_version()])
        .interface(
            "0.0.0.0"
                .parse()
                .unwrap(),
        )
        .build()?;

    let config = ServerConfig::builder()
        .add_listener(ipv4)
        .build()?;

    let security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build()?;

    let localhost_config = HostConfig::builder()
        .hostname("localhost")
        .root_directory("src/tests".into())
        .security(security_config)
        .build()?;

    let mut localhost_host = HostImpl::new(localhost_config);

    let root_path = HandlerPath::builder()
        .uri("/hello")
        .handler(handler_fn(|_request| async move {
            let response = Response::builder()
                .status(StatusCode::OK)
                .text("Hello from localhost");
            Ok(response)
        }))
        .build()?;

    localhost_host.add_path(root_path);

    let mut server = crate::Vetis::new(config);
    server
        .add_host(localhost_host)
        .await;

    server
        .start()
        .await?;

    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
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
