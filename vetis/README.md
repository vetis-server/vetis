# VeTiS (Very Tiny Server)

[![Crates.io downloads](https://img.shields.io/crates/d/vetis)](https://crates.io/crates/vetis) [![crates.io](https://img.shields.io/crates/v/vetis?style=flat-square)](https://crates.io/crates/vetis) [![Build Status](https://github.com/vetis-server/vetis/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis) [![Documentation](https://docs.rs/vetis/badge.svg)](https://docs.rs/vetis/latest/vetis) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis)

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
- **Feature-Gated**: Include only what you need for optimal binary size

## Usage Example

Please refer to the [vetis-tokio](../vetis-tokio/README.md) or [vetis-smol](../vetis-smol/README.md) crates for usage examples.

## Overview

This repository contains the reference API for vetis, providing the basis of async runtimes implementations like vetis-tokio, vetis-smol, vetis-compio and vetis-glommio.

## Roadmap

VeTiS is continuously evolving! Here's what we're working on:

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
