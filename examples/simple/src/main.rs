use std::net::IpAddr;

use http::Version;
use hyper::StatusCode;
use tracing_subscriber::{filter::LevelFilter, fmt::format};
use vetis::{VetisServer as _, host::HostConfig};
use vetis_matchers::request::PathMatcherConfig;
use vetis_tokio::{
    TlsConfig,
    hook::{HandlerHookConfig, handler_fn},
    rt::Vetis,
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

    let tls_config = TlsConfig::builder()
        .ca_file(CA_CERT)
        .cert_file(SERVER_CERT)
        .key_file(SERVER_KEY)
        .build()?;

    let root_hook = HandlerHookConfig::builder()
        .matcher(
            PathMatcherConfig::builder()
                .add_path("/hello*")
                .build()?,
        )
        .handler(handler_fn(|_request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Hello from localhost");
            Ok(response)
        }))
        .build()?;

    let health_path = HandlerHookConfig::builder()
        .matcher(
            PathMatcherConfig::builder()
                .add_path("/health*")
                .build()?,
        )
        .handler(handler_fn(|_request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Health check");
            Ok(response)
        }))
        .build()?;

    let localhost_config = HostConfig::builder()
        .hostname("localhost")
        .tls(tls_config)
        .root_directory(".")
        .add_proto(Version::HTTP_11)
        .add_proto(Version::HTTP_2)
        .add_proto(Version::HTTP_3)
        .add_default_header("alt-svc", "h3=:8443")
        .add_status_page(404, "404.html".into())
        .add_status_page(500, "500.html".into())
        .add_bind_address(
            "0.0.0.0"
                .parse::<IpAddr>()
                .unwrap(),
            8443,
        )
        .add_hook(root_hook)
        .add_hook(health_path)
        .build()?;

    let mut server = Vetis::builder()
        .add_host(localhost_config)
        .build();

    server
        .start()
        .await?;

    server
        .stop()
        .await?;

    Ok(())
}
