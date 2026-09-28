use http::Version;
use hyper::StatusCode;
use tracing_subscriber::{filter::LevelFilter, fmt::format};
use vetis::{host::HostConfig, VetisServer as _};
use vetis_macros::status_pages;
use vetis_tokio::{
    host::{
        path::{handler_fn, HandlerPath},
        Host,
    },
    rt::Vetis,
    TlsConfig,
};

pub(crate) const CA_CERT: &str = "certs/ca.der";
pub(crate) const SERVER_CERT: &str = "certs/server.der";
pub(crate) const SERVER_KEY: &str = "certs/server.key.der";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(LevelFilter::DEBUG)
        .event_format(format().compact())
        .with_target(false)
        .init();

    let security_config = TlsConfig::builder()
        .ca_file(CA_CERT)
        .cert_file(SERVER_CERT)
        .key_file(SERVER_KEY)
        .build()?;

    let localhost_config = HostConfig::builder()
        .hostname("localhost")
        .protos(&[Version::HTTP_11, Version::HTTP_2, Version::HTTP_3])
        .tls(security_config)
        .root_directory(".")
        .header("alt-svc", "h3=:8443")
        .status_pages(status_pages! {
            404 @ "404.html",
            500 @ "500.html"
        })
        .bind_addresses(&[(
            "0.0.0.0"
                .parse()
                .unwrap(),
            8443,
        )])
        .build()?;

    let mut localhost_host = Host::new(localhost_config).await?;

    let root_path = HandlerPath::builder()
        .uri("/hello")
        .handler(handler_fn(|_req, _ctx| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Hello from localhost");
            Ok(response)
        }))
        .build()?;

    localhost_host.add_path(root_path);

    let health_path = HandlerPath::builder()
        .uri("/health")
        .handler(handler_fn(|_req, _ctx| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Health check");
            Ok(response)
        }))
        .build()?;

    localhost_host.add_path(health_path);

    let mut server = Vetis::builder()
        .add_host(localhost_host)
        .await?
        .build();

    server.run().await?;

    server
        .stop()
        .await?;

    Ok(())
}
