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
use http::Version;
use macro_rules_attribute::apply;
use smol_macros::main;
use vetis::{
    listener::ListenerConfig,
    tls::TlsConfig,
    server::{ServerConfig},
    host::HostConfig,
};
use vetis_macros::status_pages;
use vetis_tokio::{
    host::Host,
    hook::{handler_fn, HandlerHook}
    Vetis, VetisServer as _
};

pub(crate) const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

#[apply(main)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "error")).init();

    let https = ListenerConfig::builder()
        .port(8443)
        .protocol_version(Version::HTTP_11)
        .interface("0.0.0.0")
        .build()?;

    let config = ServerConfig::builder()
        .add_listener(https)
        .build()?;

    let tls_config = TlsConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build()?;

    let localhost_config = HostConfig::builder()
        .hostname("localhost")
        .port(8443)
        .security(tls_config)
        .root_directory("/home/rogerio/Downloads")
        .status_pages(status_pages! {
            404 => "404.html".to_string(),
            500 => "500.html".to_string(),
        })
        .build()?;

    let mut localhost_host = Host::new(localhost_config);

    let root_path = HandlerHook::builder()
        .matcher(PathMatcher::new(
            PathMatcherConfig::builder()
                .add_path("/hello*")
                .build()?,
        ))
        .handler(handler_fn(|request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Hello from localhost");
            Ok(response)
        }))
        .build()?;

    localhost_host.add_hook(root_path);

    let health_path = HandlerHook::builder()
        .matcher(PathMatcher::new(
            PathMatcherConfig::builder()
                .add_path("/health*")
                .build()?,
        ))
        .handler(handler_fn(|request| async move {
            let response = vetis::Response::builder()
                .status(StatusCode::OK)
                .text("Health check");
            Ok(response)
        }))
        .build()?;

    localhost_host.add_hook(health_path);

    let proxy_path = ProxyHookConfig::builder()
        .matcher(PathMatcher::new(
            PathMatcherConfig::builder()
                .add_path("/proxy*")
                .build()?,
        ))
        .target("http://localhost:5230")
        .build()?;

    localhost_host.add_hook(ProxyHook::new(proxy_path));

    let images_path = StaticHookConfig::builder()
        .matcher(PathMatcher::new(
            PathMatcherConfig::builder()
                .add_path("/images*")
                .build()?,
        ))
        .directory("/home/rogerio/Downloads")
        .extensions("\\.(jpg|png|gif|html)$")
        .index_files(vec!["index.html".to_string()])
        .build()?;

    localhost_host.add_hook(StaticHook::new(images_path));

    let mut server = Vetis::new(config);
    server
        .add_host(localhost_host)
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
