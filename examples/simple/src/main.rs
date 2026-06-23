use hyper::StatusCode;
use vetis::{
    listener::ListenerConfig,
    security::SecurityConfig,
    server::{Protocol, ServerConfig},
    virtual_host::{handler_fn, VirtualHostConfig},
};
use vetis_macros::status_pages;
use vetis_tokio::{
    virtual_host::{path::HandlerPath, VirtualHostImpl},
    Vetis,
};

pub(crate) const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");
pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "error")).init();

    let https = ListenerConfig::builder()
        .port(8443)
        .protocol(Protocol::Http2)
        .interface("0.0.0.0")
        .build()?;

    let config = ServerConfig::builder()
        .add_listener(https)
        .build()?;

    let security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build()?;

    let localhost_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .port(8443)
        .security(security_config)
        .root_directory("/home/rogerio/Downloads")
        .status_pages(status_pages! {
            404 @ "404.html".to_string(),
            500 @ "500.html".to_string()
        })
        .build()?;

    let mut localhost_virtual_host = VirtualHostImpl::new(localhost_config);

    let root_path = HandlerPath::builder()
        .uri("/hello")
        .handler(handler_fn(|_request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Hello from localhost");
            Ok(response)
        }))
        .build()?;

    localhost_virtual_host.add_path(root_path);

    let health_path = HandlerPath::builder()
        .uri("/health")
        .handler(handler_fn(|_request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Health check");
            Ok(response)
        }))
        .build()?;

    localhost_virtual_host.add_path(health_path);

    let mut server = Vetis::new(config);
    server
        .add_virtual_host(localhost_virtual_host)
        .await;

    server.run().await?;

    server
        .stop()
        .await?;

    Ok(())
}
