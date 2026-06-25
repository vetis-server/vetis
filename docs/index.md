---
layout: default
title: VeTiS - Very Tiny Server
nav_order: 1
description: "🚀 A blazingly fast, minimalist HTTP server built for modern Rust applications"
permalink: /
---
<div align="center">
<h1><b>VeTiS</b></h1>
</div>

[![Crates.io downloads](https://img.shields.io/crates/d/vetis)](https://crates.io/crates/vetis) [![crates.io](https://img.shields.io/crates/v/vetis?style=flat-square)](https://crates.io/crates/vetis) [![Build Status](https://github.com/ararog/vetis/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/vetis/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis) [![Documentation](https://docs.rs/vetis/badge.svg)](https://docs.rs/vetis/latest/vetis) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/vetis/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/vetis/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/vetis)

**VeTiS** is a lightweight yet powerful web server that brings simplicity and performance together. Designed with Rust's safety guarantees in mind, it delivers HTTP/1, HTTP/2, and HTTP/3 support with a clean, intuitive API that makes building web services a breeze.

Built on top of [hyper](https://github.com/hyperium/hyper).

## History

VeTiS started as a component of deboa-tests, a private crate used by deboa http client for integration testing purposes, as it got more features, like HTTP1/2 and 3 support, alongside TLS, I realized project could be reused somehow.

So with reusability in mind, I started EasyHttpMock, a project which aims to be a quick and easy way to start a mock server for integration purposes, it didn't took too much to realized this internal http server used by EasyHttpMock could be reused for other purposes than simply be a mock server.

That's why VeTiS came to reality, by taking advantage of what I started on deboa-tests for testing purposes, it turned into a complete http server project, the goal is make it very flexible, while keeping it small and fast.

## Features

- **Minimalist Design**: Focus on what matters - serving HTTP requests efficiently
- **Flexible Runtime**: Choose between Tokio or Smol async runtimes
- **Protocol Support**: Full HTTP/1, HTTP/2, and HTTP/3 implementation
- **Secure by Default**: Built-in TLS support with modern cryptography
- **Zero-Cost Abstractions**: Leverage Rust's performance without overhead
- **Language Support**: Built-in support for Python, PHP, and RSGI applications
- **Standalone server**: Can be used as a standalone server or as a library
- **Feature-Gated**: Include only what you need for optimal binary size

## Quick Start

Please check crates section below for more information. Each crate has its own documentation and examples.

## Crates

| Crate | Description | Documentation |
|-------|-------------|---------------|
| [vetis](./vetis) | Core HTTP server library | [![docs.rs](https://img.shields.io/docsrs/vetis/latest)](https://docs.rs/vetis) |
| [vetis-smol](./vetis-smol) | Smol runtime support | [![docs.rs](https://img.shields.io/docsrs/vetis-smol/latest)](https://docs.rs/vetis-smol) |
| [vetis-tokio](./vetis-tokio) | Tokio runtime support | [![docs.rs](https://img.shields.io/docsrs/vetis-tokio/latest)](https://docs.rs/vetis-tokio) |
| [vetis-macros](./vetis-macros) | Macros for Vetis | [![docs.rs](https://img.shields.io/docsrs/vetis-macros/latest)](https://docs.rs/vetis-macros) |

## Standalone Server

Check out the [standalone server](./standalone-server.md) for complete examples of how to use Vetis as a standalone server.

## Examples

Check out the [examples](./examples.md) for complete examples of how to use Vetis in your projects.

## Create project from template

You can create a new project from the template using `cargo generate`:

`cargo generate ararog/vetis-templates`

## Benchmarks

Check out the [benchmarks](./benchmarks.md) for performance details.

## Documentation

- [API Reference](https://docs.rs/vetis)
- [Contributing Guide](./CONTRIBUTING.md)
- [Language Support](./LANGUAGE_SUPPORT.md)

## Other Projects

- [caramelo](https://crates.io/crates/caramelo) - Assertion based test framrwork
- [deboa](https://crates.io/crates/deboa) - HTTP client
- [easyhttpmock](https://crates.io/crates/easyhttpmock) - HTTP mock server
- [sofie](https://crates.io/crates/sofie) - Fullstack web framework
- [uget](https://crates.io/crates/uget) - CLI HTTP client

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
