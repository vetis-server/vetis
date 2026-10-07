# VeTiS (Very Tiny Server)

[![Crates.io downloads](https://img.shields.io/crates/d/vetis)](https://crates.io/crates/vetis) [![crates.io](https://img.shields.io/crates/v/vetis?style=flat-square)](https://crates.io/crates/vetis) [![Build Status](https://github.com/vetis-server/vetis/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/vetis-server/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis) [![Documentation](https://docs.rs/vetis/badge.svg)](https://docs.rs/vetis/latest/vetis) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis)

## A blazingly fast, minimalist HTTP server built for modern Rust applications

VeTiS is a lightweight yet powerful web server that brings simplicity and performance together. Designed with Rust's safety guarantees in mind, it delivers HTTP/1, HTTP/2, and HTTP/3 support with a clean, intuitive API that makes building web services a breeze.

## History

VeTiS started as a component of deboa-tests, a private crate used by deboa http client for integration testing purposes, as it got more features, like HTTP1/2 and 3 support, alongside TLS, I realized project could be reused somehow.

So with reusability in mind, I started EasyHttpMock, a project which aims to be a quick and easy way to start a mock server for integration purposes, it didn't took too much to realized this internal http server used by EasyHttpMock could be reused for other purposes than simply be a mock server.

That's why VeTiS came to reality, by taking advantage of what I started on deboa-tests for testing purposes, it turned into a complete http server project, the goal is make it very flexible, while keeping it small and fast.

Nowadays, this repository only contains vetis and vetis-macros, vetis is the core API for async runtimes implementations like vetis-tokio, vetis-smol, vetis-compio and vetis-glommio.

## Why VeTiS?

- **Minimalist Design**: Focus on what matters - serving HTTP requests efficiently
- **Flexible Runtime**: Choose between Tokio, Compio, Glommio or Smol async runtimes
- **Protocol Support**: Full HTTP/1, HTTP/2, and HTTP/3 implementation
- **Secure by Default**: Built-in TLS support with modern cryptography
- **Zero-Cost Abstractions**: Leverage Rust's performance without overhead

## Quick Start

Add VeTiS to your `Cargo.toml`:

```toml
vetis-tokio = { version = "0.1.0" }
```

## Usage Example

Here's how simple it is to create a web server with VeTiS:

```rust
use http::Version;
use hyper::StatusCode;
use vetis::{
    security::TlsConfig,
    server::{ServerConfig},
    host::{handler_fn, HostConfig},
};
use vetis_macros::status_pages;
use vetis_tokio::{
    host::{path::HandlerPath, Host},
    Vetis, VetisServer as _
};

pub(crate) const CA_CERT: &str = "certs/ca.der";
pub(crate) const SERVER_CERT: &str = "certs/server.der";
pub(crate) const SERVER_KEY: &str = "certs/server.key.der";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "error")).init();

    let security_config = TlsConfig::builder()
        .ca_file(CA_CERT)
        .cert_file(SERVER_CERT)
        .key_file(SERVER_KEY)
        .build()?;

    let localhost_config = HostConfig::builder()
        .hostname("localhost")
        .tls(security_config)
        .root_directory("/home/rogerio/Downloads")
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
        .add_host(localhost_host).await?
        .build();

    server.run().await?;

    server
        .stop()
        .await?;

    Ok(())
}
```

## Overview

This repository contains the reference API for vetis, providing the basis of async runtimes implementations like vetis-tokio, vetis-smol, vetis-compio and vetis-glommio.

## Roadmap

VeTiS is continuously evolving! Here's what we're working on:

## Subcrates

### [vetis](https://github.com/ararog/vetis/tree/main/vetis)

The core create of http server.

### [vetis-macros](https://github.com/ararog/vetis/tree/main/vetis-macros)

Macros for VeTiS, make easy to create small http server.

### [vetis-proxy](https://github.com/ararog/vetis/tree/main/vetis-proxy)

Reverse proxy support for VeTiS.

### [vetis-static](https://github.com/ararog/vetis/tree/main/vetis-static)

Static files support for VeTiS.

### [vetis-compio](https://github.com/ararog/vetis/tree/main/vetis-compio)

Compio runtime for VeTiS.

### [vetis-glommio](https://github.com/ararog/vetis/tree/main/vetis-glommio)

Glommio runtime for VeTiS.

### [vetis-smol](https://github.com/ararog/vetis/tree/main/vetis-smol)

Smol runtime for VeTiS.

### [vetis-tokio](https://github.com/ararog/vetis/tree/main/vetis-tokio)

Tokio runtime for VeTiS.

## Benchmarks

See [BENCHMARKS.md](BENCHMARKS.md) for detailed benchmark results.

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
