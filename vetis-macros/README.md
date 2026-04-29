# VeTiS-Macros (Very Tiny Server Macros)

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-macros)](https://crates.io/crates/vetis-macros) [![crates.io](https://img.shields.io/crates/v/vetis-macros?style=flat-square)](https://crates.io/crates/vetis-macros) [![Build Status](https://github.com/ararog/vetis/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/vetis/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-macros) [![Documentation](https://docs.rs/vetis-macros/badge.svg)](https://docs.rs/vetis-macros/latest/vetis_macros) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/vetis/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/vetis/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/vetis)

## Quick Start

Add VeTiS and VeTiS-macros to your `Cargo.toml`:

```toml
vetis = { version = "0.1.0" }
vetis-macros = { version = "0.1.0" }
vetis-smol  = { version = "0.1.0" }
```

## Usage Example

```rust
use vetis::{virtual_host::handler_fn, Response};
use vetis_macros::http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });

    let mut server = http!(
        from_crate => vetis_smol,
        hostname => "localhost",
        root_directory => "src",
        protocol => vetis_default_protocol(),
        port => 8080,
        interface => "0.0.0.0",
        handler => handler
    )
    .await?;

    server
        .start()
        .await?;

    // Do something here, issue requests and something else...

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
