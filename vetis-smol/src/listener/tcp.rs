use std::{
    collections::HashMap,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
};

use http::{header, Response};
use hyper::{body::Incoming, service::service_fn};
use hyper_body_utils::HttpBody;
#[cfg(feature = "http1")]
use hyper_util::rt::TokioIo;
use log::{debug, error, info};
use vetis::{errors::VetisError, listener::ListenerConfig, server::Protocol, Request, VetisResult};

use peekable::future::AsyncPeekable;

#[cfg(feature = "http1")]
use hyper::server::conn::http1;
#[cfg(feature = "http2")]
use hyper::server::conn::http2;

#[cfg(feature = "http2")]
use crate::rt::SmolExecutor;

use smol::{
    io::{AsyncRead, AsyncWrite},
    Task,
};

use futures_rustls::TlsAcceptor;
#[cfg(any(feature = "http1", feature = "http2"))]
use smol_hyper::rt::FuturesIo;

use crate::{
    listener::{Listener, ListenerResult},
    tls::TlsFactory,
    virtual_host::VirtualHost,
    VetisRwLock, VetisVirtualHosts,
};

/// TCP listener
pub struct TcpListener {
    task: Option<Task<()>>,
    config: ListenerConfig,
    virtual_hosts: VetisVirtualHosts<VirtualHost>,
}

impl Listener for TcpListener {
    type VirtualHost = VirtualHost;
    /// Create a new listener
    ///
    /// # Arguments
    ///
    /// * `config` - A `ListenerConfig` instance containing the listener configuration.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `TcpListener` instance.
    fn new(config: ListenerConfig) -> Self {
        Self { task: None, config, virtual_hosts: Arc::new(VetisRwLock::new(HashMap::new())) }
    }

    /// Set the virtual hosts
    ///
    /// # Arguments
    ///
    /// * `virtual_hosts` - A `VetisVirtualHosts` instance containing the virtual hosts.
    fn set_virtual_hosts(&mut self, virtual_hosts: VetisVirtualHosts<VirtualHost>) {
        self.virtual_hosts = virtual_hosts;
    }

    /// Listen for incoming connections
    ///
    /// # Returns
    ///
    /// * `ListenerResult<'_, ()>` - A `ListenerResult` instance containing the result of the listener.
    fn listen(&mut self) -> ListenerResult<'_, ()> {
        let future = async move {
            let addr = if let Ok(ip) = self
                .config
                .interface()
                .parse::<Ipv4Addr>()
            {
                SocketAddr::from((ip, self.config.port()))
            } else {
                let addr = self
                    .config
                    .interface()
                    .parse::<Ipv6Addr>();
                if let Ok(addr) = addr {
                    SocketAddr::from((addr, self.config.port()))
                } else {
                    SocketAddr::from(([0, 0, 0, 0], self.config.port()))
                }
            };

            let listener = smol::net::TcpListener::bind(addr)
                .await
                .map_err(|e| VetisError::Bind(e.to_string()))?;

            let task = self
                .handle_connections(
                    self.config
                        .protocol()
                        .clone(),
                    listener,
                    self.virtual_hosts
                        .clone(),
                )
                .await?;

            self.task = Some(task);

            Ok(())
        };

        Box::pin(future)
    }

    /// Stop the listener
    ///
    /// # Returns
    ///
    /// * `ListenerResult<'_, ()>` - A `ListenerResult` instance containing the result of the listener.
    fn stop(&mut self) -> ListenerResult<'_, ()> {
        let future = async move {
            if let Some(task) = self.task.take() {
                task.cancel().await;
            }
            Ok(())
        };

        Box::pin(future)
    }
}

/// Decompose the TCP listener into smaller, more manageable structs
impl TcpListener {
    async fn handle_connections(
        &mut self,
        protocol: Protocol,
        listener: smol::net::TcpListener,
        virtual_hosts: VetisVirtualHosts<VirtualHost>,
    ) -> VetisResult<Task<()>> {
        let alpn = vec![
            #[cfg(feature = "http1")]
            b"http/1.1".to_vec(),
            #[cfg(feature = "http2")]
            b"h2".to_vec(),
            #[cfg(feature = "http3")]
            b"h3".to_vec(),
        ];
        let tls_config = TlsFactory::create_tls_config(virtual_hosts.clone(), alpn).await?;
        let port = Arc::new(self.config.port());
        let tls_config = match tls_config {
            Some(config) => config,
            None => {
                error!("Missing TLS config");
                return Err(VetisError::Tls("Missing TLS config".to_string()));
            }
        };
        let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));
        let future = async move {
            loop {
                let result = listener
                    .accept()
                    .await;

                let (stream, client_addr) = match result {
                    Ok(conn_info) => conn_info,
                    Err(e) => {
                        error!("Cannot accept connection: {:?}", e);
                        continue;
                    }
                };

                // TODO: Check ACL before proceeding

                let mut peekable = AsyncPeekable::from(stream);

                let mut peeked = [0; 2];
                let result = peekable
                    .peek_exact(&mut peeked)
                    .await;

                if let Err(e) = result {
                    error!("Cannot peek connection: {:?}", e);
                    continue;
                }

                let is_tls = peeked.starts_with(&[0x16, 0x03]);

                if is_tls {
                    let tls_stream = tls_acceptor
                        .accept(peekable)
                        .await;

                    let tls_stream = match tls_stream {
                        Ok(tls_stream) => tls_stream,
                        Err(e) => {
                            error!("Cannot accept connection: {:?}", e);
                            continue;
                        }
                    };

                    let io = FuturesIo::new(tls_stream);
                    match protocol {
                        #[cfg(feature = "http1")]
                        Protocol::Http1 => {
                            let _ = handle_http1_request(
                                port.clone(),
                                io,
                                virtual_hosts.clone(),
                                client_addr,
                            );
                        }
                        #[cfg(feature = "http2")]
                        Protocol::Http2 => {
                            let _ = handle_http2_request(
                                port.clone(),
                                io,
                                virtual_hosts.clone(),
                                client_addr,
                            );
                        }
                        #[cfg(feature = "http3")]
                        Protocol::Http3 => {
                            // HTTP/3 is handled by UDP listener
                        }
                        _ => {
                            panic!("Unsupported protocol");
                        }
                    }
                } else {
                    let io = FuturesIo::new(peekable);
                    match protocol {
                        #[cfg(feature = "http1")]
                        Protocol::Http1 => {
                            let _ = handle_http1_request(
                                port.clone(),
                                io,
                                virtual_hosts.clone(),
                                client_addr,
                            );
                        }
                        #[cfg(feature = "http2")]
                        Protocol::Http2 => {
                            let _ = handle_http2_request(
                                port.clone(),
                                io,
                                virtual_hosts.clone(),
                                client_addr,
                            );
                        }
                        #[cfg(feature = "http3")]
                        Protocol::Http3 => {
                            // HTTP/3 is handled by UDP listener
                        }
                        _ => {
                            panic!("Unsupported protocol");
                        }
                    }
                }
            }
        };

        let task = smol::spawn(future);

        Ok(task)
    }
}

async fn process_request(
    req: http::Request<Incoming>,
    virtual_hosts: VetisVirtualHosts<VirtualHost>,
    port: Arc<u16>,
    client_addr: SocketAddr,
) -> VetisResult<http::Response<HttpBody>> {
    let host = req
        .headers()
        .get(header::HOST);

    let host = if let Some(host) = host {
        let host_port = host.to_str();
        match host_port {
            Ok(host_port) => Some(
                host_port
                    .split_once(':')
                    .map(|(host, _)| host)
                    .unwrap_or(host_port),
            ),
            Err(_) => Some("localhost"),
        }
    } else {
        match req
            .uri()
            .authority()
        {
            Some(auth) => Some(auth.host()),
            None => Some("localhost"),
        }
    };

    if let Some(host) = host {
        debug!("Serving request for host: {}", host);
        let virtual_hosts = virtual_hosts
            .read()
            .await;

        let virtual_host = virtual_hosts.get(&(host.into(), *port.clone()));

        if let Some(virtual_host) = virtual_host {
            // TODO: Save client_addr in request, grab url from request for logging
            let (parts, body) = req.into_parts();
            let request = Request::from_parts(parts, HttpBody::from_incoming(body));

            let method = request
                .method()
                .clone();

            let uri = request
                .uri()
                .clone();

            let vetis_response = virtual_host
                .route(request)
                .await?;

            let mut response = vetis_response.into_inner();

            let default_headers = virtual_host
                .config()
                .default_headers();
            if let Some(default_headers) = default_headers {
                for (key, value) in default_headers {
                    let header_name = header::HeaderName::from_bytes(key.as_bytes());
                    if header_name.is_err() {
                        error!("Invalid header name: {}", key);
                        continue;
                    }
                    let header_name = header_name.unwrap();

                    let header_value = header::HeaderValue::from_str(value);
                    if header_value.is_err() {
                        error!("Invalid header value: {}", value);
                        continue;
                    }
                    let header_value = header_value.unwrap();

                    response
                        .headers_mut()
                        .insert(header_name, header_value);
                }
            }

            // TODO: Log request and its response status code (move it to oneshot channel?)
            info!("{} {} {} {}", client_addr, method, uri, response.status());

            Ok::<http::Response<HttpBody>, VetisError>(response)
        } else {
            error!("Virtual host not found: {}", host);
            Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(HttpBody::empty())
                .map_err(|e| VetisError::Handler(e.to_string()))
        }
    } else {
        error!("Host not found in request");
        Response::builder()
            .status(http::StatusCode::BAD_REQUEST)
            .body(HttpBody::empty())
            .map_err(|e| VetisError::Handler(e.to_string()))
    }
}

#[cfg(feature = "http1")]
fn handle_http1_request<T>(
    port: Arc<u16>,
    io: FuturesIo<T>,
    virtual_hosts: VetisVirtualHosts<VirtualHost>,
    client_addr: SocketAddr,
) -> VetisResult<()>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let service_fn = service_fn(move |req| {
        let value = virtual_hosts.clone();
        let port = port.clone();
        async move { process_request(req, value, port, client_addr).await }
    });

    let future = async move {
        if let Err(err) = http1::Builder::new()
            .serve_connection(io, service_fn)
            .await
        {
            error!("Error serving connection: {:?}", err);
        }
    };

    smol::spawn(future).detach();

    Ok(())
}

#[cfg(feature = "http2")]
pub fn handle_http2_request<T>(
    port: Arc<u16>,
    io: FuturesIo<T>,
    virtual_hosts: VetisVirtualHosts<VirtualHost>,
    client_addr: SocketAddr,
) -> VetisResult<()>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let service_fn = service_fn(move |req| {
        let value = virtual_hosts.clone();
        let port = port.clone();
        async move { process_request(req, value, port, client_addr).await }
    });

    let future = async move {
        if let Err(err) = http2::Builder::new(SmolExecutor::new())
            .serve_connection(io, service_fn)
            .await
        {
            error!("Error serving connection: {:?}", err);
        }
    };

    smol::spawn(future).detach();

    Ok(())
}
