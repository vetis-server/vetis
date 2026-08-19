use crate::{
    host::HostImpl,
    listener::{Listener, ListenerResult},
    tls::TlsFactory,
    VetisHosts, VetisRwLock,
};
use http::Version;
#[cfg(feature = "http1")]
use hyper::server::conn::http1;
#[cfg(feature = "http2")]
use hyper::server::conn::http2;
#[cfg(feature = "http2")]
use hyper_util::rt::TokioExecutor;
#[cfg(any(feature = "http1", feature = "http2"))]
use hyper_util::rt::TokioIo;
use log::error;
use peekable::tokio::AsyncPeekable;
use std::{borrow::Cow, collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;
use vetis::{errors::VetisError, listener::ListenerConfig, server::http::HttpService, VetisResult};

fn supported_alpns() -> Vec<Vec<u8>> {
    vec![
        #[cfg(feature = "http2")]
        b"h2".to_vec(),
        #[cfg(feature = "http1")]
        b"http/1.1".to_vec(),
    ]
}

/// TCP listener
pub struct TcpListener {
    task: Option<JoinHandle<VetisResult<()>>>,
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
    fn set_hosts(&mut self, hosts: VetisHosts<Self::Host>) {
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

            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .map_err(|e| VetisError::Bind(e.to_string()))?;

            let task = self
                .handle_connections(listener, self.hosts.clone())
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
                task.abort();
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
        listener: tokio::net::TcpListener,
        hosts: VetisHosts<HostImpl>,
    ) -> Result<JoinHandle<VetisResult<()>>, VetisError> {
        // Limit supported alpns for TCP only
        let tls_config = TlsFactory::create_tls_config(hosts.clone(), supported_alpns()).await?;
        let tls_config = match tls_config {
            Some(config) => config,
            None => {
                error!("Missing TLS config");
                return Err(VetisError::Tls("Missing TLS config".to_string()));
            }
        };

        #[cfg(feature = "http1")]
        let http11_only = self
            .config
            .protos()
            .contains(&Version::HTTP_11);

        let tls_acceptor: TlsAcceptor = TlsAcceptor::from(Arc::new(tls_config));
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

                    let alpn = &tls_stream
                        .get_ref()
                        .1
                        .alpn_protocol();
                    if let Some(alpn_code) = alpn {
                        let Cow::Borrowed(alpn_code) = String::from_utf8_lossy(alpn_code) else {
                            error!("Cannot accept connection");
                            continue;
                        };

                        match alpn_code {
                            #[cfg(feature = "http1")]
                            "http1.1" => {
                                let service = HttpService::new(hosts.clone(), client_addr);
                                tokio::spawn(
                                    http1::Builder::new()
                                        .serve_connection(TokioIo::new(tls_stream), service),
                                );
                            }
                            #[cfg(feature = "http2")]
                            "h2" => {
                                let service = HttpService::new(hosts.clone(), client_addr);
                                tokio::spawn(
                                    http2::Builder::new(TokioExecutor::new())
                                        .serve_connection(TokioIo::new(tls_stream), service),
                                );
                            }
                            _ => {
                                panic!("Unsupported protocol");
                            }
                        }
                    }
                } else {
                    // Insecure connections are only allowed for HTTP/1.1
                    #[cfg(feature = "http1")]
                    {
                        if http11_only {
                            let service = HttpService::new(hosts.clone(), client_addr);
                            tokio::spawn(
                                http1::Builder::new()
                                    .serve_connection(TokioIo::new(peekable), service),
                            );
                        } else {
                            panic!("Unsupported protocol");
                        }
                    }

                    #[cfg(any(feature = "http2", feature = "http3"))]
                    {
                        panic!("Insecure connections are only allowed with HTTP/1.1");
                    }
                }
            }
        };

        let task = tokio::spawn(future);

        Ok(task)
    }
}
