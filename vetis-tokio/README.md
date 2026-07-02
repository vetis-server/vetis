# VeTiS-Tokio (Very Tiny Server - Tokio runtime support)

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-tokio)](https://crates.io/crates/vetis-tokio) [![crates.io](https://img.shields.io/crates/v/vetis-tokio?style=flat-square)](https://crates.io/crates/vetis-tokio) [![Build Status](https://github.com/vetis-server/vetis/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-tokio) [![Documentation](https://docs.rs/vetis-tokio/badge.svg)](https://docs.rs/vetis-tokio/latest/vetis-tokio) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis)

## Tokio Runtime Support for Vetis

The goal of this crate is provide tokio runtime support for Vetis.

## Quick Start

Add VeTiS and VeTiS-Tokio to your `Cargo.toml`:

```toml
vetis = { version = "0.1.4-beta.7" }
vetis-tokio = { version = "0.1.0-beta.2", features = ["http2", "rust-tls"] }
```

## Crate features

- http1
- http2 (default)
- http3
- rust-tls (default)

## Usage Example

Here's how simple it is to create a web server with VeTiS:

```rust,no_run
use hyper::StatusCode;

use vetis::{
    listener::ListenerConfig,
    security::SecurityConfig,
    server::{Protocol, ServerConfig},
    virtual_host::{handler_fn, VirtualHostConfig},
    Vetis as _
};

use vetis_tokio::{
    virtual_host::{path::HandlerPath, VirtualHostImpl},
    Vetis,
};

pub(crate) const CA_CERT: &[u8] = include_bytes!("../../certs/ca.der");
pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../../certs/server.key.der");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "error")).init();

    let https = ListenerConfig::builder()
        .port(8443)
        .protocol(Protocol::Http1)
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
        .build()?;

    let mut localhost_virtual_host = VirtualHostImpl::new(localhost_config);

    let root_path = HandlerPath::builder()
        .uri("/hello")
        .handler(handler_fn(|request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Hello from localhost");
            Ok(response)
        }))
        .build()?;

    localhost_virtual_host.add_path(root_path);

    let health_path = HandlerPath::builder()
        .uri("/health")
        .handler(handler_fn(|request| async move {
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
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
