use crate::{
    host::{path::HandlerPath, HostImpl},
    tests::{default_protocol_version, CA_CERT, SERVER_CERT, SERVER_KEY},
};
use deboa::{
    cert::{CertificateExt, ContentEncoding},
    request,
};
use deboa_tokio::{cert::DeboaCertificate, Client};
use http::StatusCode;
use rand::random_range;
use vetis::{
    host::{handler_fn, HostConfig},
    listener::ListenerConfig,
    security::SecurityConfig,
    server::ServerConfig,
    VetisServer as _,
};

#[tokio::test]
async fn test_handler() -> Result<(), Box<dyn std::error::Error>> {
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

    let security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build()?;

    let host_config = HostConfig::builder()
        .hostname("localhost")
        .root_directory("src/tests".into())
        .security(security_config)
        .build()?;

    let mut server = crate::Vetis::new(
        ServerConfig::builder()
            .add_listener(ipv4)
            .build()?,
    );

    let root_path = HandlerPath::builder()
        .uri("/hello")
        .handler(handler_fn(|_request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Hello from localhost");
            Ok(response)
        }))
        .build()?;

    let mut host = HostImpl::new(host_config);

    host.add_path(root_path);

    server
        .add_host(host)
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
