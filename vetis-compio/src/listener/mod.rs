#[cfg(any(feature = "http1", feature = "http2"))]
use crate::listener::tcp::TcpListener;
#[cfg(feature = "http3")]
use crate::listener::udp::UdpListener;
use crate::{virtual_host::VirtualHostImpl, VetisVirtualHosts};
#[cfg(feature = "http2")]
use http::Version;
use vetis::listener::{Listener, ListenerConfig, ListenerResult};

#[cfg(any(feature = "http1", feature = "http2"))]
pub(crate) mod tcp;

#[cfg(feature = "http3")]
pub(crate) mod udp;

/// Server listener
pub enum ServerListener {
    /// TCP listener
    #[cfg(any(feature = "http1", feature = "http2"))]
    Tcp(TcpListener),
    /// UDP listener
    #[cfg(feature = "http3")]
    Udp(UdpListener),
}

impl Listener for ServerListener {
    type VirtualHost = VirtualHostImpl;

    fn new(config: ListenerConfig) -> Self
    where
        Self: Sized,
    {
        match config.protocol_version() {
            #[cfg(feature = "http1")]
            &Version::HTTP_11 => ServerListener::Tcp(TcpListener::new(config)),
            #[cfg(feature = "http2")]
            &Version::HTTP_2 => ServerListener::Tcp(TcpListener::new(config)),
            #[cfg(feature = "http3")]
            &Version::HTTP_3 => ServerListener::Udp(UdpListener::new(config)),
            _ => panic!("Unsupported protocol"),
        }
    }

    /// Set the virtual hosts
    fn set_virtual_hosts(&mut self, virtual_hosts: VetisVirtualHosts<VirtualHostImpl>) {
        match self {
            #[cfg(any(feature = "http1", feature = "http2"))]
            ServerListener::Tcp(tcp_listener) => {
                tcp_listener.set_virtual_hosts(virtual_hosts);
            }
            #[cfg(feature = "http3")]
            ServerListener::Udp(ref mut udp_listener) => {
                udp_listener.set_virtual_hosts(virtual_hosts);
            }
        }
    }

    fn listen(&mut self) -> ListenerResult<'_, ()> {
        Box::pin(async move {
            match self {
                #[cfg(any(feature = "http1", feature = "http2"))]
                ServerListener::Tcp(tcp_listener) => {
                    tcp_listener
                        .listen()
                        .await?
                }
                #[cfg(feature = "http3")]
                ServerListener::Udp(ref mut udp_listener) => {
                    udp_listener
                        .listen()
                        .await?
                }
            }

            Ok(())
        })
    }

    fn stop(&mut self) -> ListenerResult<'_, ()> {
        Box::pin(async move {
            match self {
                #[cfg(any(feature = "http1", feature = "http2"))]
                ServerListener::Tcp(tcp_listener) => {
                    tcp_listener
                        .stop()
                        .await?
                }
                #[cfg(feature = "http3")]
                ServerListener::Udp(ref mut udp_listener) => {
                    udp_listener
                        .stop()
                        .await?
                }
            }
            Ok(())
        })
    }
}
