# VeTiS (Very Tiny Server)

[![Crates.io downloads](https://img.shields.io/crates/d/vetis)](https://crates.io/crates/vetis) [![crates.io](https://img.shields.io/crates/v/vetis?style=flat-square)](https://crates.io/crates/vetis) [![Build Status](https://github.com/vetis-server/vetis/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/vetis-server/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis) [![Documentation](https://docs.rs/vetis/badge.svg)](https://docs.rs/vetis/latest/vetis) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis)

**A blazingly fast, minimalist HTTP server built for modern Rust applications**

VeTiS is a lightweight yet powerful web server that brings simplicity and performance together. Designed with Rust's safety guarantees in mind, it delivers HTTP/1, HTTP/2, and HTTP/3 support with a clean, intuitive API that makes building web services a breeze.

## History

VeTiS started as a component of deboa-tests, a private crate used by deboa http client for integration testing purposes, as it got more features, like HTTP1/2 and 3 support, alongside TLS, I realized project could be reused somehow.

So with reusability in mind, I started EasyHttpMock, a project which aims to be a quick and easy way to start a mock server for integration purposes, it didn't took too much to realized this internal http server used by EasyHttpMock could be reused for other purposes than simply be a mock server.

That's why VeTiS came to reality, by taking advantage of what I started on deboa-tests for testing purposes, it turned into a complete http server project, the goal is make it very flexible, while keeping it small and fast.

## Why VeTiS?

- **Minimalist Design**: Focus on what matters - serving HTTP requests efficiently
- **Flexible Runtime**: Choose between Tokio or Smol async runtimes
- **Protocol Support**: Full HTTP/1, HTTP/2, and HTTP/3 implementation
- **Secure by Default**: Built-in TLS support with modern cryptography
- **Zero-Cost Abstractions**: Leverage Rust's performance without overhead
- **Feature-Gated**: Include only what you need for optimal binary size

## Quick Start

Add VeTiS to your `Cargo.toml`:

```toml
vetis = { version = "0.1.0" }
```

## Runtimes

- [tokio](https://github.com/tokio-rs/tokio)
- [smol](https://github.com/smol-rs/smol)

## Crate features

- http1 (default)
- http2
- http3
- rust-tls (default)

## External crates

- static-files
- reverse-proxy
- auth

## Usage Example

Here's how simple it is to create a web server with VeTiS:

```rust
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
        .status_pages(maplit::hashmap! {
            404 => "404.html".to_string(),
            500 => "500.html".to_string(),
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
```

## Overview

### Core Features

- **Standalone Server** - Run as a standalone HTTP/HTTPS server
- **Multi-Protocol** - Support for HTTP/1, HTTP/2 and HTTP/3 are disabled by default
- **Virtual Hosts** - Host multiple domains on a single server
- **SNI Support** - Server Name Indication for TLS
- **Reverse Proxy** - Route requests to backend services (feature gated, disabled by default)

### Content & Security

- **Authentication** - Multiple auth methods support
- **Authorization** - Fine-grained access control
- **Dynamic Content** - Template rendering and content generation
- **Logging** - Comprehensive request and error logging
- **Static File Serving** - Efficient static asset delivery

### Languages

- **Python** - Support for ASGI/WSGI/RSGI applications
- **PHP** - Support for PHP applications
- **Ruby** - Support for Ruby applications

## Roadmap

VeTiS is continuously evolving! Here's what we're working on:

### Core Features

- **WebSockets** - Real-time bidirectional communication
- **Load Balancing** - Distribute traffic across multiple servers

## Subprojects

### [vetis](https://github.com/ararog/vetis/tree/main/vetis)

The core create of http server.

### [vetis-macros](https://github.com/ararog/vetis/tree/main/vetis-macros)

Macros for VeTiS, make easy to create small http server.

### [vetis-interface](https://github.com/ararog/vetis/tree/main/vetis-interface)

Interface for VeTiS, make easy to create small http server.

### [vetis-proxy](https://github.com/ararog/vetis/tree/main/vetis-proxy)

Reverse proxy support for VeTiS.

### [vetis-static](https://github.com/ararog/vetis/tree/main/vetis-static)

Static files support for VeTiS.

### [vetis-smol](https://github.com/ararog/vetis/tree/main/vetis-smol)

Smol runtime for VeTiS.

### [vetis-tokio](https://github.com/ararog/vetis/tree/main/vetis-tokio)

Tokio runtime for VeTiS.

## Benchmarks

See [BENCHMARKS.md](BENCHMARKS.md) for detailed benchmark results.

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
