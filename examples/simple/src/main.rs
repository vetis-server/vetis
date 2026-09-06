use http::Version;
use hyper::StatusCode;
use vetis::{
    host::{handler_fn, HostConfig},
    listener::ListenerConfig,
    security::SecurityConfig,
    VetisServer as _,
};
use vetis_macros::status_pages;
use vetis_tokio::{
    host::{path::HandlerPath, Host},
    listener::build_listeners,
    rt::Vetis,
};

pub(crate) const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");
pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "debug")).init();

    let https = ListenerConfig::builder()
        .port(8443)
        .protos(vec![Version::HTTP_11, Version::HTTP_3])
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

    let localhost_config = HostConfig::builder()
        .hostname("localhost")
        .security(security_config)
        .root_directory("/home/rogerio/Downloads".into())
        .header("alt-svc", "h3=:8443")
        .status_pages(status_pages! {
            404 @ "404.html".to_string(),
            500 @ "500.html".to_string()
        })
        .bind_addresses(vec![(
            "0.0.0.0"
                .parse()
                .unwrap(),
            8443,
        )])
        .build()?;

    let mut localhost_host = Host::new(localhost_config);

    let root_path = HandlerPath::builder()
        .uri("/hello")
        .handler(handler_fn(|_request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Hello from localhost");
            Ok(response)
        }))
        .build()?;

    localhost_host.add_path(root_path);

    let health_path = HandlerPath::builder()
        .uri("/health")
        .handler(handler_fn(|_request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Health check");
            Ok(response)
        }))
        .build()?;

    localhost_host.add_path(health_path);

    let mut server = Vetis::builder()
        .add_listeners(build_listeners(https))?
        .add_host(localhost_host)?
        .build();

    server.run().await?;

    server
        .stop()
        .await?;

    Ok(())
}
