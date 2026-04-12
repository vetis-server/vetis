//! Server implementation and virtual host system.
//!
//! This module provides the core HTTP server implementation and the virtual host
//! system that allows multiple domains to be served by a single server instance.
//!
//! # Modules
//!
//! - [`conn`]: Connection handling for different protocols
//! - [`http`]: HTTP/1 and HTTP/2 server implementation
//! - [`tls`]: TLS/SSL support for secure connections
//! - [`virtual_host`]: Virtual host system and request handlers
//!
//! # Examples
//!
//! ```rust,ignore
//! use vetis::{
//!     config::{ServerConfig, VirtualHostConfig},
//!     server::virtual_host::{DefaultVirtualHost, VirtualHost, handler_fn},
//! };
//!
//! // Create a virtual host with a custom handler
//! let vhost_config = VirtualHostConfig::builder()
//!     .hostname("example.com")
//!     .port(80)
//!     .build()?;
//!
//! let mut vhost = DefaultVirtualHost::new(vhost_config);
//! vhost.set_handler(handler_fn(|request| async move {
//!     // Handle the request...
//!     Ok(vetis::Response::builder()
//!         .status(http::StatusCode::OK)
//!         .body(http_body_util::Full::new(bytes::Bytes::from("Hello"))))
//! }));
//! ```

pub mod http;
pub mod listener;
pub mod tls;
pub mod virtual_host;
