#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
use quote::quote;
use syn::parse_macro_input;

use crate::parsers::{HttpArgs, StatusPagesArgs, TlsArgs};

mod parsers;

#[proc_macro]
/// Create an HTTP server
///
/// # Arguments
///
/// ## Required
///
/// * `protocol` - The protocol of the server
/// * `handler` - The handler of the server
///
/// ## Optional
///
/// * `from_crate` - The crate to use for the server, defaults to "vetis_tokio"
/// * `hostname` - The hostname of the server, defaults to "localhost"
/// * `root_directory` - The root directory of the server, defaults to "."
/// * `port` - The port of the server, defaults to 80
/// * `interface` - The interface of the server, defaults to "0.0.0.0"
///
/// # Returns
///
/// * `Vetis` - The HTTP server
///
/// # Errors
///
/// * `VetisError` - If the server fails to start
///
/// # Examples
///
/// ```rust, ignore
/// use http::Version;
/// use vetis::{Response, host::handler_fn};
/// use vetis_macros::http;
///
/// /// Main function to start the server
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });
///
///     let mut server = http!(
///         from_crate => vetis_tokio,
///         hostname => "localhost",
///         root_directory => "src",
///         protos => &[Version::Http1],
///         port => 8080,
///         interface => "0.0.0.0",
///         handler => handler
///     )
///     .await?;
///
///     server
///         .start()
///         .await?;
///
///     // Issue a request to the server
///
///     server
///         .stop()
///         .await?;
///
///     Ok(())
/// }
pub fn http(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as HttpArgs);

    let from_crate = match args.from_crate {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'from_crate'")
                .to_compile_error()
                .into();
        }
    };

    let handler = match args.handler {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'handler'")
                .to_compile_error()
                .into();
        }
    };

    let protos = match args.protos {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'protos'")
                .to_compile_error()
                .into();
        }
    };

    let root_directory = match args.root_directory {
        Some(root_directory) => quote! { .root_directory(#root_directory) },
        None => TS2::new(),
    };

    let workers = match args.workers {
        Some(workers) => quote! { .tls(#workers) },
        None => TS2::new(),
    };

    let logger_queue_size = match args.logger_queue_size {
        Some(logger_queue_size) => quote! { .tls(#logger_queue_size) },
        None => TS2::new(),
    };

    let log = match args.log {
        Some(log) => quote! { .log(#log) },
        None => TS2::new(),
    };

    let tls = match args.tls {
        Some(tls) => quote! { .tls(#tls) },
        None => TS2::new(),
    };

    let hostname = match args.hostname {
        Some(hostname) => quote! { #hostname.into() },
        None => {
            quote! { "localhost".into() }
        }
    };

    let interface = match args.interface {
        Some(interface) => quote! { #interface.into() },
        None => {
            quote! { std::net::Ipv4Addr::UNSPECIFIED.into() }
        }
    };

    let port = match args.port {
        Some(port) => quote! { #port },
        None => {
            quote! { 80 }
        }
    };

    let allow_unsafe_conn = match args.allow_unsafe_conn {
        Some(allow_unsafe) => quote! { #allow_unsafe },
        None => {
            quote! { false }
        }
    };

    let expanded = quote! {
        async move {
            use vetis::{
                errors::VetisError,
                server::ServerConfig,
                host::{Host as _, HostConfig},
            };

            use #from_crate::{
                host::{path::HandlerPath, Host},
                rt::Vetis,
            };

            let server_config = ServerConfig::builder()
                #workers
                #logger_queue_size
                #log
                .build()?;

            let mut host_config = HostConfig::builder()
                .hostname(#hostname)
                #root_directory
                .protos(#protos)
                .allow_unsafe_connections(#allow_unsafe_conn)
                #tls
                .bind_addresses(&[
                    (#interface, #port)
                ])
                .build()?;

            let mut host = Host::new(host_config).await?;

            let root_path = HandlerPath::builder()
                .uri("/")
                .handler(Box::new(#handler))
                .build()?;

            host.add_path(root_path);

            let vetis = Vetis::builder()
                .config(server_config)
                .add_host(host).await?
                .build();

            Ok::<Vetis, VetisError>(vetis)
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Creates a `TlsConfig` from file paths.
///
/// # Arguments
///
/// ## Required
///
/// * `cert` - The path to the certificate file.
/// * `key` - The path to the private key file.
/// * `ca_cert` - The path to the CA certificate file.
///
/// ## Optional
///
/// * `client_auth` - Whether to require client authentication.
///
/// # Examples
///
/// ```rust,ignore
/// use vetis_macros::tls;
///
/// #[tokio::main]
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let security = tls! {
///         cert => "/path/to/server.der",
///         key => "/path/to/server.key.der",
///         ca_cert => "/path/to/ca.der",
///         client_auth => true
///     };
///
///     Ok(())
/// }
/// ```
pub fn tls(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as TlsArgs);

    let cert = match args.cert {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'cert'")
                .to_compile_error()
                .into();
        }
    };

    let key = match args.key {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'key'")
                .to_compile_error()
                .into();
        }
    };

    let ca_cert = match args.ca_cert {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'ca_cert'")
                .to_compile_error()
                .into();
        }
    };

    let client_auth = match args.client_auth {
        Some(client_auth) => quote! { .client_auth(#client_auth) },
        None => quote! { .client_auth(false) },
    };

    let expanded = quote! {
        vetis::security::TlsConfig::builder()
            .cert_file(#cert)
            .key_file(#key)
            .ca_file(#ca_cert)
            #client_auth
            .build()?
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Creates a `HashMap` of status codes to file paths.
///
/// # Arguments
///
/// * `$($code:literal => $path:expr),*` - A list of status codes and file paths.
///
/// # Examples
///
/// ```rust,no_run
/// use std::collections::HashMap;
/// use vetis_macros::status_pages;
///
/// let pages = status_pages! {
///     404 @ "404.html",
///     500 @ "500.html"
/// };
/// ```
pub fn status_pages(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as StatusPagesArgs);

    let pages = args
        .pages
        .iter()
        .map(|(code, path)| {
            quote! {
                status_pages.insert(#code, #path.into());
            }
        });

    let expanded = quote! {
      {
        let mut status_pages = std::collections::HashMap::<u16, std::sync::Arc<str>>::new();
        #(#pages)*
        status_pages
      }
    };

    TokenStream::from(expanded)
}
