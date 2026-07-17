use crate::{
    tests::{
        deboa_default_protocol, vetis_default_protocol, CA_CERT, IP6_SERVER_CERT, IP6_SERVER_KEY,
        SERVER_CERT, SERVER_KEY,
    },
    virtual_host::{path::HandlerPath, VirtualHostImpl},
};
use deboa::{
    cert::{Certificate, ContentEncoding},
    request,
};
use http::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::test;
use std::error::Error;
use vetis::{
    listener::ListenerConfig,
    security::SecurityConfig,
    server::ServerConfig,
    virtual_host::{handler_fn, VirtualHostConfig},
    Response, VetisServer as _,
};

#[apply(test!)]
async fn test_multiple_interfaces() -> Result<(), Box<dyn Error>> {
    let host = if cfg!(windows) { "localhost" } else { "ip6-localhost" };

    let ipv4 = ListenerConfig::builder()
        .port(8080)
        .protocol(vetis_default_protocol())
        .interface("0.0.0.0")
        .build()?;

    let ipv6 = ListenerConfig::builder()
        .port(8081)
        .protocol(vetis_default_protocol())
        .interface("::")
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

    let localhost_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .port(8080)
        .root_directory("src/tests")
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

    let ip6_localhost_config = VirtualHostConfig::builder()
        .hostname(host)
        .port(8081)
        .root_directory("src/tests")
        .security(ip6_security_config)
        .build()?;

    let mut localhost_virtual_host = VirtualHostImpl::new(localhost_config);
    let mut ip6_localhost_virtual_host = VirtualHostImpl::new(ip6_localhost_config);

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

    localhost_virtual_host.add_path(ip4_root_path);
    ip6_localhost_virtual_host.add_path(ip6_root_path);

    let mut server = crate::Vetis::new(config);
    server
        .add_virtual_host(localhost_virtual_host)
        .await;
    server
        .add_virtual_host(ip6_localhost_virtual_host)
        .await;

    server
        .start()
        .await?;

    let client = deboa_smol::Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .protocol(deboa_default_protocol())
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

    let client = deboa_smol::Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .bind_addr(
            "::1"
                .parse()
                .unwrap(),
        )
        .protocol(deboa_default_protocol())
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
