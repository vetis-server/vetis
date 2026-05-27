#[macro_export]
/// Create an HTTP server
///
/// # Arguments
///
/// * `hostname` - The hostname of the server
/// * `root_directory` - The root directory of the server
/// * `protocol` - The protocol of the server
/// * `port` - The port of the server
/// * `interface` - The interface of the server
/// * `handler` - The handler of the server
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
/// ```rust, no_run
/// use vetis::{Response, virtual_host::handler_fn};
/// use vetis_macros::http;
///
/// /// Main function to start the server
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });
///
///     let mut server = http!(
///         from_crate => vetis_smol,
///         hostname => "localhost",
///         root_directory => "src",
///         protocol => vetis::server::Protocol::Http1,
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
/// ```
macro_rules! http {
    (
      from_crate => $from_crate:ident,
      hostname => $hostname:expr,
      root_directory => $root_directory:expr,
      protocol => $protocol:expr,
      port => $port:expr,
      interface => $interface:expr,
      handler => $handler:ident
    ) => {
        async move {
            use vetis::{
                errors::VetisError,
                listener::ListenerConfig,
                server::ServerConfig,
                virtual_host::{VirtualHost, VirtualHostConfig},
            };

            use $from_crate::{
                virtual_host::{path::HandlerPath, VirtualHostImpl},
                Vetis,
            };

            let listener = ListenerConfig::builder()
                .port($port)
                .protocol($protocol)
                .interface($interface)
                .build()?;

            let config = ServerConfig::builder()
                .add_listener(listener)
                .build()?;

            let virtual_host_config = VirtualHostConfig::builder()
                .hostname($hostname)
                .root_directory($root_directory)
                .port($port)
                .build()?;

            let mut virtual_host = VirtualHostImpl::new(virtual_host_config);

            let root_path = HandlerPath::builder()
                .uri("/")
                .handler(Box::new($handler))
                .build()?;

            virtual_host.add_path(root_path);

            let mut vetis = Vetis::new(config);

            vetis
                .add_virtual_host(virtual_host)
                .await;

            Ok::<Vetis, VetisError>(vetis)
        }
    };

    (
      from_crate => $from_crate:ident,
      hostname => $hostname:expr,
      root_directory => $root_directory:expr,
      protocol => $protocol:expr,
      port => $port:expr,
      interface => $interface:expr,
      handler => $handler:ident,
      security_config => $security_config:expr
    ) => {
        async move {
            use vetis::{
                errors::VetisError,
                listener::ListenerConfig,
                server::ServerConfig,
                virtual_host::{VirtualHost, VirtualHostConfig},
            };

            use $from_crate::{
                virtual_host::{path::HandlerPath, VirtualHostImpl},
                Vetis,
            };

            let listener = ListenerConfig::builder()
                .port($port)
                .protocol($protocol)
                .interface($interface)
                .build()?;

            let config = ServerConfig::builder()
                .add_listener(listener)
                .build()?;

            let virtual_host_config = VirtualHostConfig::builder()
                .hostname($hostname)
                .root_directory($root_directory)
                .port($port)
                .security($security_config)
                .build()?;

            let mut virtual_host = VirtualHostImpl::new(virtual_host_config);

            let root_path = HandlerPath::builder()
                .uri("/")
                .handler(Box::new($handler))
                .build()?;

            virtual_host.add_path(root_path);

            let mut vetis = Vetis::new(config);

            vetis
                .add_virtual_host(virtual_host)
                .await;

            Ok::<Vetis, VetisError>(vetis)
        }
    };

    (
      from_crate => $from_crate:ident,
      hostname => $hostname:literal,
      protocol => $protocol:expr,
      port => $port:literal,
      interface => $interface:literal,
      handler => $handler:ident,
      security_config => $security_config:expr
    ) => {
        async move {
            use vetis::{
                errors::VetisError,
                listener::ListenerConfig,
                server::ServerConfig,
                virtual_host::{VirtualHost, VirtualHostConfig},
            };

            use $from_crate::{
                virtual_host::{path::HandlerPath, VirtualHostImpl},
                Vetis,
            };

            let listener = ListenerConfig::builder()
                .port($port)
                .protocol($protocol)
                .interface($interface)
                .build()?;

            let config = ServerConfig::builder()
                .add_listener(listener)
                .build()?;

            let virtual_host_config = VirtualHostConfig::builder()
                .hostname($hostname)
                .port($port)
                .build()?;

            let mut virtual_host = VirtualHostImpl::new(virtual_host_config);

            let root_path = HandlerPath::builder()
                .uri("/")
                .handler(Box::new($handler))
                .build()?;

            virtual_host.add_path(root_path);

            let mut vetis = Vetis::new(config);

            vetis
                .add_virtual_host(virtual_host)
                .await;

            Ok::<Vetis, VetisError>(vetis)
        }
    };

    (
      from_crate => $from_crate:ident,
      hostname => $hostname:literal,
      protocol => $protocol:expr,
      port => $port:literal,
      interface => $interface:literal,
      handler => $handler:ident
    ) => {
        async move {
            use vetis::{
                errors::VetisError,
                listener::ListenerConfig,
                server::ServerConfig,
                virtual_host::{VirtualHost, VirtualHostConfig},
            };

            use $from_crate::{
                virtual_host::{path::HandlerPath, VirtualHostImpl},
                Vetis,
            };

            let listener = ListenerConfig::builder()
                .port($port)
                .protocol($protocol)
                .interface($interface)
                .build()?;

            let config = ServerConfig::builder()
                .add_listener(listener)
                .build()?;

            let virtual_host_config = VirtualHostConfig::builder()
                .hostname($hostname)
                .port($port)
                .build()?;

            let mut virtual_host = VirtualHostImpl::new(virtual_host_config);

            let root_path = HandlerPath::builder()
                .uri("/")
                .handler(Box::new($handler))
                .build()?;

            virtual_host.add_path(root_path);

            let mut vetis = Vetis::new(config);

            vetis
                .add_virtual_host(virtual_host)
                .await;

            Ok::<Vetis, VetisError>(vetis)
        }
    };
}

#[macro_export]
/// Creates a `SecurityConfig` from file paths.
///
/// # Arguments
///
/// * `cert` - The path to the certificate file.
/// * `key` - The path to the private key file.
/// * `ca_cert` - The path to the CA certificate file.
/// * `client_auth` - Whether to require client authentication.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::security::SecurityConfig;
/// use vetis_macros::security;
///
/// let security = security! {
///     cert => "/path/to/server.der",
///     key => "/path/to/server.key.der",
///     ca_cert => "/path/to/ca.der",
///     client_auth => true
/// };
/// ```
macro_rules! security {
    (
      cert => $cert:expr,
      key => $key:expr,
      ca_cert => $ca_cert:expr,
      client_auth => $client_auth:expr
    ) => {{
        vetis::security::SecurityConfig::builder()
            .cert_from_file($cert)
            .key_from_file($key)
            .ca_cert_from_file($ca_cert)
            .client_auth($client_auth)
            .build()?
    }};
}

#[macro_export]
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
///     404 => "404.html",
///     500 => "500.html"
/// };
/// ```
macro_rules! status_pages {
    ($($code:literal => $path:expr),*) => {
        {
            let mut pages = std::collections::HashMap::new();
            $( pages.insert($code, $path); )*
            pages
        }
    };
}
