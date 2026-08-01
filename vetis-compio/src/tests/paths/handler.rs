use crate::{
    tests::{default_protocol_version, CA_CERT, SERVER_CERT, SERVER_KEY},
    virtual_host::{path::HandlerPath, VirtualHostImpl},
};
use deboa::{
    cert::{CertificateExt, ContentEncoding},
    request,
};
use deboa_compio::{cert::DeboaCertificate, Client};
use http::StatusCode;
use rand::random_range;
use std::error::Error;
use vetis::{
    listener::ListenerConfig,
    security::SecurityConfig,
    server::ServerConfig,
    virtual_host::{handler_fn, VirtualHostConfig},
    Response, VetisServer as _,
};

#[compio::test]
async fn test_handler() -> Result<(), Box<dyn Error>> {
    let port = random_range(9000..=20000);
    let ipv4 = ListenerConfig::builder()
        .port(port)
        .protocol_version(default_protocol_version())
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
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .build();

    let request = request::get(format!("https://localhost:{}{}", port, "/hello"))?
        .version(default_protocol_version())
        .send_with(&client)
        .await?;

    assert_eq!(request.status(), StatusCode::OK);

    server
        .stop()
        .await?;

    Ok(())
}
