#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
use quote::quote;
use syn::parse_macro_input;

use crate::parsers::{HttpArgs, SecurityArgs, StatusPagesArgs};

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
///         protos => vec![Version::Http1],
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
                .into()
        }
    };

    let handler = match args.handler {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'handler'")
                .to_compile_error()
                .into()
        }
    };

    let protos = match args.protos {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'protos'")
                .to_compile_error()
                .into()
        }
    };

    let root_directory = match args.root_directory {
        Some(root_directory) => quote! { .root_directory(#root_directory) },
        None => TS2::new(),
    };

    let security_config = match args.security {
        Some(security_config) => quote! { .security(#security_config) },
        None => TS2::new(),
    };

    let hostname = match args.hostname {
        Some(hostname) => quote! { #hostname },
        None => {
            quote! { "localhost" }
        }
    };

    let interface = match args.interface {
        Some(interface) => quote! { #interface },
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
                listener::ListenerConfig,
                server::ServerConfig,
                host::{Host as _, HostConfig},
            };

            use #from_crate::{
                host::{path::HandlerPath, Host},
                listener::build_listeners,
                rt::Vetis,
            };

            let listener = ListenerConfig::builder()
                .port(#port)
                .protos(#protos)
                .interface(#interface)
                .allow_unsafe_connections(#allow_unsafe_conn)
                .build()?;

            let mut host_config = HostConfig::builder()
                .hostname(#hostname)
                #root_directory
                #security_config
                .bind_addresses(vec![
                    (#interface, #port)
                ])
                .build()?;

            let mut host = Host::new(host_config);

            let root_path = HandlerPath::builder()
                .uri("/")
                .handler(Box::new(#handler))
                .build()?;

            host.add_path(root_path);

            let vetis = Vetis::builder()
                .add_listeners(build_listeners(listener))?
                .add_host(host)?
                .build();

            Ok::<Vetis, VetisError>(vetis)
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Creates a `SecurityConfig` from file paths.
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
/// use vetis_macros::security;
///
/// #[tokio::main]
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let security = security! {
///         cert => "/path/to/server.der",
///         key => "/path/to/server.key.der",
///         ca_cert => "/path/to/ca.der",
///         client_auth => true
///     };
///
///     Ok(())
/// }
/// ```
pub fn security(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as SecurityArgs);

    let cert = match args.cert {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'cert'")
                .to_compile_error()
                .into()
        }
    };

    let key = match args.key {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'key'")
                .to_compile_error()
                .into()
        }
    };

    let ca_cert = match args.ca_cert {
        Some(e) => e,
        None => {
            return syn::Error::new(Span::call_site(), "Missing required field: 'ca_cert'")
                .to_compile_error()
                .into()
        }
    };

    let client_auth = match args.client_auth {
        Some(client_auth) => quote! { .client_auth(#client_auth) },
        None => quote! { .client_auth(false) },
    };

    let expanded = quote! {
        vetis::security::SecurityConfig::builder()
            .cert_from_file(#cert)
            .key_from_file(#key)
            .ca_cert_from_file(#ca_cert)
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
///     404 @ "404.html".to_string(),
///     500 @ "500.html".to_string()
/// };
/// ```
pub fn status_pages(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as StatusPagesArgs);

    let pages = args
        .pages
        .iter()
        .map(|(code, path)| {
            quote! {
                status_pages.insert(#code, #path);
            }
        });

    let expanded = quote! {
      {
        let mut status_pages = std::collections::HashMap::<u16, String>::new();
        #(#pages)*
        status_pages
      }
    };

    TokenStream::from(expanded)
}
