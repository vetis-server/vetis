# VeTiS (Very Tiny Server)

[![Crates.io downloads](https://img.shields.io/crates/d/vetis)](https://crates.io/crates/vetis) [![crates.io](https://img.shields.io/crates/v/vetis?style=flat-square)](https://crates.io/crates/vetis) [![Build Status](https://github.com/ararog/vetis/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/vetis/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis) [![Documentation](https://docs.rs/vetis/badge.svg)](https://docs.rs/vetis/latest/vetis) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/vetis/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/vetis/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/vetis)

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

## Crate features

- static-files
- reverse-proxy
- auth

## Usage Example

Please refer to the [vetis-tokio](../vetis-tokio/README.md) or [vetis-smol](../vetis-smol/README.md) crates for usage examples.

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
- **Static File Serving** - Efficient static asset delivery (feature gated, disabled by default)

### Languages

- **Python** - Support for ASGI/WSGI/RSGI applications
- **PHP** - Support for FastCGI compatible applications
- **Ruby** - Support for FastCGI compatible applications
- **Java** - Support for FastCGI compatible applications
- **Perl** - Support for FastCGI compatible applications

See [LANGUAGE_SUPPORT.md](LANGUAGE_SUPPORT.md) for detailed language support information.

## Roadmap

VeTiS is continuously evolving! Here's what we're working on:

### Core Features

- **WebSockets** - Real-time bidirectional communication
- **Load Balancing** - Distribute traffic across multiple servers

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
