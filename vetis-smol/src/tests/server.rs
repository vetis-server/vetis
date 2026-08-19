use crate::{
    host::{path::HandlerPath, HostImpl},
    tests::{
        default_protocol_version, CA_CERT, IP6_SERVER_CERT, IP6_SERVER_KEY, SERVER_CERT, SERVER_KEY,
    },
};
use deboa::{
    cert::{CertificateExt, ContentEncoding},
    request,
};
use deboa_smol::{cert::DeboaCertificate, Client};
use http::StatusCode;
use macro_rules_attribute::apply;
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
async fn test_multiple_interfaces() -> Result<(), Box<dyn Error>> {
    let host = if cfg!(windows) { "localhost" } else { "ip6-localhost" };

    let ipv4 = ListenerConfig::builder()
        .port(8080)
        .protos(vec![default_protocol_version()])
        .interface(
            "0.0.0.0"
                .parse()
                .unwrap(),
        )
        .build()?;

    let ipv6 = ListenerConfig::builder()
        .port(8081)
        .protos(vec![default_protocol_version()])
        .interface(
            "::".parse()
                .unwrap(),
        )
        .build()?;

    let config = ServerConfig::builder()
        .add_listener(ipv4)
        .add_listener(ipv6)
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

    #[cfg(unix)]
    let ip6_security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(IP6_SERVER_CERT.to_vec())
        .key_from_bytes(IP6_SERVER_KEY.to_vec())
        .build()?;

    #[cfg(windows)]
    let ip6_security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build()?;

    let ip6_localhost_config = HostConfig::builder()
        .hostname(host)
        .root_directory("src/tests".into())
        .security(ip6_security_config)
        .build()?;

    let mut localhost_host = HostImpl::new(localhost_config);
    let mut ip6_localhost_host = HostImpl::new(ip6_localhost_config);

    let ip4_root_path = HandlerPath::builder()
        .uri("/hello")
        .handler(handler_fn(|_request| async move {
            let response = Response::builder()
                .status(StatusCode::OK)
                .text("Hello from ipv4");
            Ok(response)
        }))
        .build()?;

    let ip6_root_path = HandlerPath::builder()
        .uri("/hello")
        .handler(handler_fn(|_request| async move {
            let response = Response::builder()
                .status(StatusCode::OK)
                .text("Hello from ipv6");
            Ok(response)
        }))
        .build()?;

    localhost_host.add_path(ip4_root_path);
    ip6_localhost_host.add_path(ip6_root_path);

    let mut server = crate::Vetis::new(config);
    server
        .add_host(localhost_host)
        .await;
    server
        .add_host(ip6_localhost_host)
        .await;

    server
        .start()
        .await?;

    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .build();

    let request = request::get("https://localhost:8080/hello")?
        .send_with(&client)
        .await?;

    assert_eq!(request.status(), StatusCode::OK);
    assert_eq!(
        request
            .text()
            .await?,
        "Hello from ipv4"
    );

    let client = Client::builder()
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .bind_addr(
            "::1"
                .parse()
                .unwrap(),
        )
        .build();

    let request = request::get(format!("https://{}:8081/hello", host))?
        .send_with(&client)
        .await?;

    assert_eq!(request.status(), StatusCode::OK);
    assert_eq!(
        request
            .text()
            .await?,
        "Hello from ipv6"
    );

    server
        .stop()
        .await?;

    Ok(())
}
