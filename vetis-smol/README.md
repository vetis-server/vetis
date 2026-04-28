# VeTiS-Smol (Very Tiny Server - Smol runtime support)

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-smol)](https://crates.io/crates/vetis-smol) [![crates.io](https://img.shields.io/crates/v/vetis-smol?style=flat-square)](https://crates.io/crates/vetis-smol) [![Build Status](https://github.com/ararog/vetis/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/vetis/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-smol) [![Documentation](https://docs.rs/vetis-smol/badge.svg)](https://docs.rs/vetis-smol/latest/vetis-smol) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/vetis/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/vetis/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/vetis)

**A blazingly fast, minimalist HTTP server built for modern Rust applications**

VeTiS is a lightweight yet powerful web server that brings simplicity and performance together. Designed with Rust's safety guarantees in mind, it delivers HTTP/1, HTTP/2, and HTTP/3 support with a clean, intuitive API that makes building web services a breeze.

## Quick Start

Add VeTiS to your `Cargo.toml`:

```toml
vetis = { version = "0.1.0" }
vetis-smol = { version = "0.1.0", features = ["http2", "rust-tls"] }
```

## Crate features

- http1
- http2 (default)
- http3
- rust-tls (default)
- static-files
- reverse-proxy
- auth

## Usage Example

Here's how simple it is to create a web server with VeTiS:

```rust
use hyper::StatusCode;

use macro_rules_attribute::apply;
use smol_macros::main;

use vetis::{
    listener::ListenerConfig,
    server::{Protocol, ServerConfig},
    virtual_host::{
        handler_fn,
        SecurityConfig, VirtualHostConfig,
    },
};

use vetis_tokio::{
    virtual_host::{path::HandlerPath, VirtualHost},
    Vetis,
};

pub(crate) const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

#[apply(main!)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
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
        .build()?;

    let mut localhost_virtual_host = VirtualHost::new(localhost_config);

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
