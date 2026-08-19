use crate::{
    host::HostImpl,
    listener::{Listener, ListenerResult},
    tls::TlsFactory,
    VetisHosts, VetisRwLock,
};
use bytes::Bytes;
use futures_util::StreamExt;
use h3::server::{Connection, RequestResolver};
use h3_quinn::{
    quinn::{self, crypto::rustls::QuicServerConfig},
    Connection as QuinnConnection,
};
use http::{HeaderName, HeaderValue, StatusCode};
use hyper_body_utils::HttpBody;
use log::{debug, error, info};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::task::JoinHandle;
use vetis::{
    errors::{StartError, VetisError},
    host::Host,
    listener::ListenerConfig,
    request::Request,
    Response, VetisResult,
};

/// UDP listener
pub struct UdpListener {
    config: ListenerConfig,
    task: Option<JoinHandle<()>>,
    hosts: VetisHosts<HostImpl>,
}

impl UdpListener {
    /// Create a new listener
    ///
    /// # Arguments
    ///
    /// * `config` - A `ListenerConfig` instance containing the listener configuration.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `UdpListener` instance.
    pub fn new(config: ListenerConfig) -> Self {
        Self { config, task: None, hosts: Arc::new(VetisRwLock::new(HashMap::new())) }
    }
}

impl Listener for UdpListener {
    type Host = HostImpl;

    /// Allow set virtual hosts
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

            let tls_config =
                TlsFactory::create_tls_config(self.hosts.clone(), vec![b"h3".to_vec()]).await?;

            if let Some(tls_config) = tls_config {
                let quic_config = QuicServerConfig::try_from(tls_config)
                    .map_err(|e| VetisError::Start(StartError::Tls(e.to_string())))?;

                let server_config = quinn::ServerConfig::with_crypto(Arc::new(quic_config));

                let endpoint = quinn::Endpoint::server(server_config, addr)
                    .map_err(|e| VetisError::Bind(e.to_string()))?;

                let server_task = self
                    .handle_connections(endpoint, self.hosts.clone())
                    .await?;

                self.task = Some(server_task);
            }

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
        Box::pin(async move {
            if let Some(task) = self.task.take() {
                task.abort();
            }
            Ok(())
        })
    }
}

impl UdpListener {
    async fn handle_connections(
        &mut self,
        endpoint: quinn::Endpoint,
        hosts: VetisHosts<HostImpl>,
    ) -> Result<JoinHandle<()>, VetisError> {
        let task = tokio::spawn(async move {
            while let Some(new_conn) = endpoint
                .accept()
                .await
            {
                let hosts = hosts.clone();
                let addr = new_conn.remote_address();
                tokio::spawn(async move {
                    match new_conn.await {
                        Ok(conn) => {
                            let mut h3_conn: Connection<QuinnConnection, Bytes> =
                                match Connection::new(QuinnConnection::new(conn)).await {
                                    Ok(conn) => conn,
                                    Err(err) => {
                                        error!("Cannot create connection: {:?}", err);
                                        return;
                                    }
                                };

                            loop {
                                match h3_conn
                                    .accept()
                                    .await
                                {
                                    Ok(Some(resolver)) => {
                                        let result =
                                            handle_http_request(resolver, hosts.clone(), addr);

                                        if let Err(err) = result {
                                            error!("Error handling HTTP request: {:?}", err);
                                        }
                                    }
                                    Ok(None) => {
                                        break;
                                    }
                                    Err(err) => {
                                        error!("Cannot accept connection: {:?}", err);
                                        break;
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            error!("Accepting connection failed: {:?}", err);
                        }
                    }
                });
            }

            endpoint
                .wait_idle()
                .await;
        });

        Ok(task)
    }
}

fn handle_http_request(
    resolver: RequestResolver<QuinnConnection, Bytes>,
    hosts: VetisHosts<HostImpl>,
    client_addr: SocketAddr,
) -> VetisResult<()> {
    let hosts = hosts.clone();
    tokio::spawn(async move {
        let result = resolver
            .resolve_request()
            .await;
        if let Ok((req, stream)) = result {
            let (mut send_stream, recv_stream) = stream.split();
            let (parts, _) = req.into_parts();
            let method = parts.method.clone();
            let uri = parts.uri.clone();
            let body = HttpBody::from_generic_server(recv_stream);
            let request = http::Request::from_parts(parts, body);

            let host = request
                .uri()
                .authority();

            let hosts = hosts.clone();
            let response = if let Some(authority) = host {
                debug!("Serving request for host: {}", authority);
                let hosts = hosts.read().await;
                let host = hosts.get(authority.host());
                let response = if let Some(host) = host {
                    let (parts, body) = request.into_parts();
                    let request = Request::from_parts(parts, body);

                    let vetis_response = host
                        .route(request)
                        .await;

                    let response = if let Err(err) = vetis_response {
                        error!("Error executing request: {:?}", err);
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .text("Internal server error")
                            .into_inner()
                    } else {
                        let mut response = vetis_response
                            .unwrap()
                            .into_inner();

                        let default_headers = host
                            .config()
                            .default_headers();

                        if let Some(default_headers) = default_headers {
                            for (key, value) in default_headers {
                                let Ok(header_name) = HeaderName::from_bytes(key.as_bytes()) else {
                                    error!("Invalid header name: {}", key);
                                    continue;
                                };

                                let Ok(header_value) = HeaderValue::from_str(value.as_str()) else {
                                    error!("Invalid header value: {}", value);
                                    continue;
                                };

                                response
                                    .headers_mut()
                                    .insert(header_name, header_value);
                            }
                        }

                        response
                    };

                    // TODO: Log request and its response status code (move it to oneshot channel?)
                    info!("{} {} {} {}", client_addr, method, uri, response.status());

                    Ok::<_, VetisError>(response)
                } else {
                    error!("Host not found: {}", authority.host());
                    let response = Response::builder()
                        .status(StatusCode::BAD_GATEWAY)
                        .text("Host not found")
                        .into_inner();
                    Ok(response)
                };

                response
            } else {
                error!("Host not found in request");
                let response = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .text("Host not found")
                    .into_inner();
                Ok(response)
            };

            if let Ok(response) = response {
                let (parts, mut body) = response.into_parts();

                let mut resp = http::Response::builder()
                    .status(parts.status)
                    .version(parts.version)
                    .extension(parts.extensions)
                    .body(())
                    .unwrap();

                resp.headers_mut()
                    .extend(parts.headers);

                match send_stream
                    .send_response(resp)
                    .await
                {
                    Ok(_) => {
                        debug!("Successfully respond to connection");
                    }
                    Err(err) => {
                        error!("Unable to send response to connection: {:?}", err);
                    }
                }

                while let Some(buf) = body.next().await {
                    if let Ok(buf) = buf {
                        if let Ok(bytes) = buf.into_data() {
                            let _ = send_stream
                                .send_data(bytes)
                                .await;
                        }
                    }
                }

                let _ = send_stream
                    .finish()
                    .await;
            } else {
                error!("HttpServer - Error serving connection: {:?}", response.err());
            }
        }
    });

    Ok(())
}
