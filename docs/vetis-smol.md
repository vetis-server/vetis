---
layout: default
title: Vetis Smol - Smol Runtime Support
nav_order: 4
---

## Vetis Smol

Smol runtime support for Vetis HTTP server.

## Installation

```toml
vetis = { version = "0.1.4-beta.23" }
vetis-smol = { version = "0.1.0-beta.12", features = ["http2", "rust-tls"] }
```

## Usage

```rust
use http::Version;
use hyper::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::main;
use vetis::{
    listener::ListenerConfig,
    tls::TlsConfig,
    server::{ServerConfig},
    host::HostConfig,
};
use vetis_smol::{
    host::Host,
    hook::{handler_fn, HandlerHook}
    Vetis, VetisServer as _
};

pub(crate) const CA_CERT: &[u8] = include_bytes!("../../certs/ca.der");
pub(crate) const SERVER_CERT: &[u8] = include_bytes!("../../certs/server.der");
pub(crate) const SERVER_KEY: &[u8] = include_bytes!("../../certs/server.key.der");

#[apply(main!)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "error")).init();

    let https = ListenerConfig::builder()
        .port(8443)
        .protocol_version(Version::HTTP_2)
        .interface("0.0.0.0".parse().unwrap())
        .build()?;

    let tls_config = TlsConfig::builder()
        .ca_cert_from_bytes(CA_CERT.to_vec())
        .cert_from_bytes(SERVER_CERT.to_vec())
        .key_from_bytes(SERVER_KEY.to_vec())
        .build()?;

    let localhost_config = HostConfig::builder()
        .hostname("localhost")
        .tls(tls_config)
        .root_directory("/home/rogerio/Downloads".into())
        .bind_addresses(vec![(
            "0.0.0.0"
                .parse()
                .unwrap(),
            8443,
        )])
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
```

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/vetis-smol).
