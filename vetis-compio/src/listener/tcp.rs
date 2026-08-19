use crate::{
    host::HostImpl,
    listener::{supported_alpns, Listener, ListenerResult},
    tls::TlsFactory,
    VetisHosts, VetisRwLock,
};
use compio::io::{util::Splittable, AsyncRead, AsyncWrite};
use compio::runtime::JoinHandle;
use compio_tls::TlsAcceptor;
#[cfg(feature = "http2")]
use cyper_core::CompioExecutor;
use cyper_core::HyperStream;
use http::{header, HeaderName, HeaderValue, Response, Version};
#[cfg(feature = "http1")]
use hyper::server::conn::http1;
#[cfg(feature = "http2")]
use hyper::server::conn::http2;
use hyper::{body::Incoming, service::service_fn};
use hyper_body_utils::HttpBody;
use log::{debug, error, info};
use send_wrapper::SendWrapper;
use std::{borrow::Cow, collections::HashMap, net::SocketAddr, sync::Arc};
use vetis::{errors::VetisError, host::Host, listener::ListenerConfig, Request, VetisResult};

/// TCP listener
pub struct TcpListener {
    task: Option<JoinHandle<()>>,
    config: ListenerConfig,
    hosts: VetisHosts<HostImpl>,
}

impl TcpListener {
    /// Create a new listener
    ///
    /// # Arguments
    ///
    /// * `config` - A `ListenerConfig` instance containing the listener configuration.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `TcpListener` instance.
    pub fn new(config: ListenerConfig) -> Self {
        Self { task: None, config, hosts: Arc::new(VetisRwLock::new(HashMap::new())) }
    }
}

impl Listener for TcpListener {
    type Host = HostImpl;

    /// Set the virtual hosts
    ///
    /// # Arguments
    ///
    /// * `hosts` - A `VetisHosts` instance containing the virtual hosts.
    fn set_hosts(&mut self, hosts: VetisHosts<HostImpl>) {
        self.hosts = hosts;
    }

    /// Listen for incoming connections
    ///
    /// # Returns
    ///
    /// * `ListenerResult<'_, ()>` - A `ListenerResult` instance containing the result of the listener.
    fn listen(&mut self) -> ListenerResult<'_, ()> {
        let future = async move {
            let addr = SocketAddr::new(
                *self
                    .config
                    .interface(),
                self.config.port(),
            );

            let listener = compio::net::TcpListener::bind(addr)
                .await
                .map_err(|e| VetisError::Bind(e.to_string()))?;

            let task = self
                .handle_connections(listener, self.hosts.clone())
                .await?;

            self.task = Some(task);

            Ok(())
        };

        Box::pin(SendWrapper::new(future))
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
        listener: compio::net::TcpListener,
        hosts: VetisHosts<HostImpl>,
    ) -> VetisResult<JoinHandle<()>> {
        let tls_config = TlsFactory::create_tls_config(hosts.clone(), supported_alpns()).await?;
        let tls_config = match tls_config {
            Some(config) => config,
            None => {
                error!("Missing TLS config");
                return Err(VetisError::Tls("Missing TLS config".to_string()));
            }
        };

        let http11_only = self
            .config
            .protos()
            .contains(&Version::HTTP_11);

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

                let result = stream
                    .peek([0u8; 2])
                    .await;
                if result.is_err() {
                    error!("Cannot peek connection");
                    continue;
                }

                let (_, data) = result.unwrap();
                let is_tls = data.starts_with(&[0x16, 0x03]);
                if is_tls {
                    let tls_stream = tls_acceptor
                        .accept(stream)
                        .await;

                    let tls_stream = match tls_stream {
                        Ok(tls_stream) => tls_stream,
                        Err(e) => {
                            error!("Cannot accept connection: {:?}", e);
                            continue;
                        }
                    };

                    let alpn = &tls_stream.negotiated_alpn();
                    if let Some(alpn_code) = alpn {
                        let Cow::Borrowed(alpn_code) = String::from_utf8_lossy(alpn_code) else {
                            error!("Cannot accept connection");
                            continue;
                        };

                        match alpn_code {
                            #[cfg(feature = "http1")]
                            "http1.1" => {
                                let _ = handle_http1_request(
                                    HyperStream::new_tls(tls_stream),
                                    hosts.clone(),
                                    client_addr,
                                );
                            }
                            #[cfg(feature = "http2")]
                            "h2" => {
                                let _ = handle_http2_request(
                                    HyperStream::new_tls(tls_stream),
                                    hosts.clone(),
                                    client_addr,
                                );
                            }
                            _ => {
                                panic!("Unsupported protocol");
                            }
                        }
                    }
                } else {
                    #[cfg(feature = "http1")]
                    {
                        let io = HyperStream::new_plain(stream);
                        if http11_only {
                            let _ = handle_http1_request(io, hosts.clone(), client_addr);
                        } else {
                            panic!("Unsupported protocol");
                        }
                    }

                    #[cfg(any(feature = "http2", feature = "http3"))]
                    {
                        panic!("Unsupported protocol");
                    }
                }
            }
        };

        let task = compio::runtime::spawn(future);

        Ok(task)
    }
}

async fn process_request(
    req: http::Request<Incoming>,
    hosts: VetisHosts<HostImpl>,
    client_addr: SocketAddr,
) -> VetisResult<http::Response<HttpBody>> {
    let host_header = req
        .headers()
        .get(header::HOST);

    let hostname = if let Some(host) = host_header {
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

    if let Some(hostname) = hostname {
        debug!("Serving request for host: {}", hostname);
        let hosts = hosts.read().await;
        let host = hosts.get(hostname);
        if let Some(host) = host {
            // TODO: Save client_addr in request, grab url from request for logging
            let (parts, body) = req.into_parts();
            let request = Request::from_parts(parts, HttpBody::from_incoming(body));

            let method = request
                .method()
                .clone();

            let uri = request
                .uri()
                .clone();

            let vetis_response = host
                .route(request)
                .await?;

            let mut response = vetis_response.into_inner();

            let default_headers = host
                .config()
                .default_headers();
            if let Some(default_headers) = default_headers {
                for (key, value) in default_headers {
                    let Ok(header_name) = HeaderName::from_bytes(key.as_bytes()) else {
                        error!("Invalid header name: {}", key);
                        continue;
                    };

                    let Ok(header_value) = HeaderValue::from_str(value) else {
                        error!("Invalid header value: {}", value);
                        continue;
                    };

                    response
                        .headers_mut()
                        .insert(header_name, header_value);
                }
            }

            // TODO: Log request and its response status code (move it to oneshot channel?)
            info!("{} {} {} {}", client_addr, method, uri, response.status());

            Ok::<http::Response<HttpBody>, VetisError>(response)
        } else {
            error!("Host not found: {}", hostname);
            Response::builder()
                .status(http::StatusCode::BAD_GATEWAY)
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
    io: HyperStream<T>,
    hosts: VetisHosts<HostImpl>,
    client_addr: SocketAddr,
) -> VetisResult<()>
where
    T: AsyncRead + AsyncWrite + Splittable + Unpin + 'static,
    T::ReadHalf: AsyncRead + Unpin,
    T::WriteHalf: AsyncWrite + Unpin,
{
    let service_fn = service_fn(move |req| {
        let value = hosts.clone();
        async move { process_request(req, value, client_addr).await }
    });

    let future = async move {
        if let Err(err) = http1::Builder::new()
            .serve_connection(io, service_fn)
            .await
        {
            error!("Error serving connection: {:?}", err);
        }
    };

    compio::runtime::spawn(future).detach();

    Ok(())
}

#[cfg(feature = "http2")]
pub fn handle_http2_request<T>(
    io: HyperStream<T>,
    hosts: VetisHosts<HostImpl>,
    client_addr: SocketAddr,
) -> VetisResult<()>
where
    T: AsyncRead + AsyncWrite + Splittable + Unpin + 'static,
    T::ReadHalf: AsyncRead + Unpin,
    T::WriteHalf: AsyncWrite + Unpin,
{
    let service_fn = service_fn(move |req| {
        let value = hosts.clone();
        async move { process_request(req, value, client_addr).await }
    });

    let future = async move {
        if let Err(err) = http2::Builder::new(CompioExecutor)
            .serve_connection(io, service_fn)
            .await
        {
            error!("Error serving connection: {:?}", err);
        }
    };

    compio::runtime::spawn(future).detach();

    Ok(())
}
