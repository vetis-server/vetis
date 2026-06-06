#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

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
/// ```rust, ignore
/// use vetis::{Response, virtual_host::handler_fn};
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
      @internal
      from_crate => $from_crate:ident,
      hostname => $hostname:expr,
      protocol => $protocol:expr,
      port => $port:expr,
      interface => $interface:expr,
      handler => $handler:ident
      $(, root_directory => $root_directory:expr)?
      $(, security_config => $security_config:expr)?
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

            let mut virtual_host_config = VirtualHostConfig::builder()
                .hostname($hostname)
                $(.root_directory($root_directory))?
                .port($port)
                $(.security($security_config))?
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
        http! {
            @internal
            from_crate => $from_crate,
            hostname => $hostname,
            protocol => $protocol,
            port => $port,
            interface => $interface,
            handler => $handler,
            root_directory => $root_directory,
            security_config => $security_config
        }
    };

    (
      from_crate => $from_crate:ident,
      root_directory => $root_directory:expr,
      hostname => $hostname:expr,
      protocol => $protocol:expr,
      port => $port:expr,
      interface => $interface:expr,
      handler => $handler:ident,
      security_config => $security_config:expr
    ) => {
        http! {
            @internal
            from_crate => $from_crate,
            hostname => $hostname,
            protocol => $protocol,
            port => $port,
            interface => $interface,
            handler => $handler,
            root_directory => $root_directory,
            security_config => $security_config
        }
    };

    (
      from_crate => $from_crate:ident,
      hostname => $hostname:expr,
      protocol => $protocol:expr,
      port => $port:expr,
      interface => $interface:expr,
      handler => $handler:ident
    ) => {
        http! {
            @internal
            from_crate => $from_crate,
            hostname => $hostname,
            protocol => $protocol,
            port => $port,
            interface => $interface,
            handler => $handler
        }
    };

    (
      from_crate => $from_crate:ident,
      protocol => $protocol:expr,
      hostname => $hostname:expr,
      port => $port:expr,
      interface => $interface:expr,
      handler => $handler:ident
    ) => {
        http! {
            @internal
            from_crate => $from_crate,
            hostname => $hostname,
            protocol => $protocol,
            port => $port,
            interface => $interface,
            handler => $handler
        }
    };

    (
      from_crate => $from_crate:ident,
      port => $port:expr,
      protocol => $protocol:expr,
      hostname => $hostname:expr,
      interface => $interface:expr,
      handler => $handler:ident
    ) => {
        http! {
            @internal
            from_crate => $from_crate,
            hostname => $hostname,
            protocol => $protocol,
            port => $port,
            interface => $interface,
            handler => $handler
        }
    };

    (
      from_crate => $from_crate:ident,
      interface => $interface:expr,
      port => $port:expr,
      protocol => $protocol:expr,
      hostname => $hostname:expr,
      handler => $handler:ident
    ) => {
        http! {
            @internal
            from_crate => $from_crate,
            hostname => $hostname,
            protocol => $protocol,
            port => $port,
            interface => $interface,
            handler => $handler
        }
    };

    (
      from_crate => $from_crate:ident,
      handler => $handler:ident
      interface => $interface:expr,
      port => $port:expr,
      protocol => $protocol:expr,
      hostname => $hostname:expr,
    ) => {
        http! {
            @internal
            from_crate => $from_crate,
            hostname => $hostname,
            protocol => $protocol,
            port => $port,
            interface => $interface,
            handler => $handler
        }
    };

    ($($tts:tt)*) => {
        http!(@internal $($tts)*)
    };
}

#[macro_export]
/// Creates a `Vetis` instance with a localhost virtual host.
///
/// # Arguments
///
/// * `from_crate` - The crate to use for the virtual host.
/// * `protocol` - The protocol of the virtual host.
/// * `handler` - The handler of the virtual host.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis_macros::localhost;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let vetis = localhost! {
///         from_crate => vetis,
///         port => 8080,
///         protocol => Protocol::Http1,
///         handler => MyHandler
///     }.await?;
///     
///     Ok(())
/// }
/// ```
macro_rules! localhost {
    (
      @internal
      from_crate => $from_crate:ident,
      port => $port:literal,
      protocol => $protocol:expr,
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
                .interface("127.0.0.1")
                .build()?;

            let config = ServerConfig::builder()
                .add_listener(listener)
                .build()?;

            let virtual_host_config = VirtualHostConfig::builder()
                .hostname("localhost")
                .root_directory(".")
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
      protocol => $protocol:expr,
      port => $port:literal,
      handler => $handler:ident
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    (
      from_crate => $from_crate:ident,
      port => $port:literal,
      protocol => $protocol:expr,
      handler => $handler:ident
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    (
      from_crate => $from_crate:ident,
      handler => $handler:ident,
      port => $port:literal,
      protocol => $protocol:expr
    ) => {
        localhost!(@internal from_crate => $from_crate, port => $port, protocol => $protocol, handler => $handler)
    };

    (
      from_crate => $from_crate:ident,
      handler => $handler:ident,
      protocol => $protocol:expr,
      port => $port:literal
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    (
      from_crate => $from_crate:ident,
      port => $port:literal,
      handler => $handler:ident,
      protocol => $protocol:expr
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    (
      port => $port:literal,
      from_crate => $from_crate:ident,
      protocol => $protocol:expr,
      handler => $handler:ident
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    (
      port => $port:literal,
      protocol => $protocol:expr,
      from_crate => $from_crate:ident,
      handler => $handler:ident
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    (
      port => $port:literal,
      protocol => $protocol:expr,
      handler => $handler:ident,
      from_crate => $from_crate:ident
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    (
      protocol => $protocol:expr,
      handler => $handler:ident,
      from_crate => $from_crate:ident,
      port => $port:literal
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    (
      protocol => $protocol:expr,
      from_crate => $from_crate:ident,
      handler => $handler:ident,
      port => $port:literal
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    (
      protocol => $protocol:expr,
      port => $port:literal,
      from_crate => $from_crate:ident,
      handler => $handler:ident
    ) => {
        localhost!(
          @internal
          from_crate => $from_crate,
          port => $port,
          protocol => $protocol,
          handler => $handler
        )
    };

    ($($tts:tt)*) => {
        localhost!(@internal $($tts)*)
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
/// use vetis_macros::security;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
macro_rules! security {
    (
      @internal
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

    (
      cert => $cert:expr,
      key => $key:expr,
      ca_cert => $ca_cert:expr,
      client_auth => $client_auth:expr
    ) => {
        security!(
            @internal
            cert => $cert,
            key => $key,
            ca_cert => $ca_cert,
            client_auth => $client_auth
        )
    };

    (
      cert => $cert:expr,
      ca_cert => $ca_cert:expr,
      key => $key:expr,
      client_auth => $client_auth:expr
    ) => {
        security!(@internal cert => $cert, key => $key, ca_cert => $ca_cert, client_auth => $client_auth)
    };

    (
      cert => $cert:expr,
      key => $key:expr,
      client_auth => $client_auth:expr,
      ca_cert => $ca_cert:expr
    ) => {
        security!(@internal cert => $cert, key => $key, ca_cert => $ca_cert, client_auth => $client_auth)
    };

    (
      key => $key:expr,
      cert => $cert:expr,
      ca_cert => $ca_cert:expr,
      client_auth => $client_auth:expr
    ) => {
        security!(
          @internal
          cert => $cert,
          key => $key,
          ca_cert => $ca_cert,
          client_auth => $client_auth
        )
    };

    (
      key => $key:expr,
      ca_cert => $ca_cert:expr,
      cert => $cert:expr,
      client_auth => $client_auth:expr
    ) => {
        security!(
          @internal
          cert => $cert,
          key => $key,
          ca_cert => $ca_cert,
          client_auth => $client_auth
        )
    };

    (
      key => $key:expr,
      ca_cert => $ca_cert:expr,
      client_auth => $client_auth:expr,
      cert => $cert:expr
    ) => {
        security!(
          @internal
          cert => $cert,
          key => $key,
          ca_cert => $ca_cert,
          client_auth => $client_auth
        )
    };

    (
      ca_cert => $ca_cert:expr,
      key => $key:expr,
      cert => $cert:expr,
      client_auth => $client_auth:expr
    ) => {
        security!(
          @internal
          cert => $cert,
          key => $key,
          ca_cert => $ca_cert,
          client_auth => $client_auth
        )
    };

    (
      ca_cert => $ca_cert:expr,
      cert => $cert:expr,
      key => $key:expr,
      client_auth => $client_auth:expr
    ) => {
        security!(
          @internal
          cert => $cert,
          key => $key,
          ca_cert => $ca_cert,
          client_auth => $client_auth
        )
    };

    (
      ca_cert => $ca_cert:expr,
      client_auth => $client_auth:expr,
      key => $key:expr,
      cert => $cert:expr
    ) => {
        security!(
          @internal
          cert => $cert,
          key => $key,
          ca_cert => $ca_cert,
          client_auth => $client_auth
        )
    };

    ($($tts:tt)*) => {
        security!(@internal $($tts)*)
    };
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
