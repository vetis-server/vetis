---
layout: default
title: Vetis - HTTP Server
nav_order: 2
---

## Vetis

**VeTiS** is a lightweight yet powerful web server that brings simplicity and performance together. Designed with Rust's safety guarantees in mind, it delivers HTTP/1, HTTP/2, and HTTP/3 support with a clean, intuitive API that makes building web services a breeze.

Built on top of [hyper](https://github.com/hyperium/hyper).

## Installation

Add VeTiS to your `Cargo.toml`:

```toml
[dependencies]
vetis = { version = "0.1.4-beta.7" }
vetis-smol = { version = "0.1.0-beta.2", features = ["http2", "rust-tls"] }
```

## Usage

```rust
use hyper::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::main;
use vetis::{
    listener::ListenerConfig,
    security::SecurityConfig,
    server::{Protocol, ServerConfig},
    virtual_host::{handler_fn, VirtualHostConfig},
    VetisServer as _
};
use vetis_macros::status_pages;
use vetis_tokio::{
    virtual_host::{path::HandlerPath, VirtualHostImpl},
    Vetis,
};

pub(crate) const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

#[apply(main)]
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
        .status_pages(status_pages! {
            404 => "404.html".to_string(),
            500 => "500.html".to_string(),
        })
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

    let proxy_path = ProxyPathConfig::builder()
        .uri("/proxy")
        .target("http://localhost:5230")
        .build()?;

    localhost_virtual_host.add_path(ProxyPath::new(proxy_path));

    let images_path = StaticPathConfig::builder()
        .uri("/images")
        .directory("/home/rogerio/Downloads")
        .extensions("\\.(jpg|png|gif|html)$")
        .index_files(vec!["index.html".to_string()])
        .build()?;

    localhost_virtual_host.add_path(StaticPath::new(images_path));

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

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/vetis).
