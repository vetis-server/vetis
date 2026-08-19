#[cfg(any(feature = "http1", feature = "http2"))]
use crate::listener::tcp::TcpListener;
#[cfg(feature = "http3")]
use crate::listener::udp::UdpListener;
use crate::{host::HostImpl, VetisHosts};
use vetis::listener::{Listener, ListenerResult};

#[cfg(any(feature = "http1", feature = "http2"))]
pub(crate) mod tcp;

#[cfg(feature = "http3")]
pub(crate) mod udp;

pub(crate) fn supported_alpns() -> Vec<Vec<u8>> {
    vec![
        #[cfg(feature = "http3")]
        b"h3".to_vec(),
        #[cfg(feature = "http2")]
        b"h2".to_vec(),
        #[cfg(feature = "http1")]
        b"http/1.1".to_vec(),
    ]
}

/// Server listener
pub enum ServerListener {
    /// TCP listener
    #[cfg(any(feature = "http1", feature = "http2"))]
    Tcp(TcpListener),
    /// UDP listener
    #[cfg(feature = "http3")]
    Udp(UdpListener),
}

#[cfg(any(feature = "http1", feature = "http2"))]
impl From<TcpListener> for ServerListener {
    fn from(value: TcpListener) -> Self {
        ServerListener::Tcp(value)
    }
}

#[cfg(feature = "http3")]
impl From<UdpListener> for ServerListener {
    fn from(value: UdpListener) -> Self {
        ServerListener::Udp(value)
    }
}

impl Listener for ServerListener {
    type Host = HostImpl;

    /// Set the virtual hosts
    fn set_hosts(&mut self, hosts: VetisHosts<HostImpl>) {
        match self {
            #[cfg(any(feature = "http1", feature = "http2"))]
            ServerListener::Tcp(tcp_listener) => {
                tcp_listener.set_hosts(hosts);
            }
            #[cfg(feature = "http3")]
            ServerListener::Udp(ref mut udp_listener) => {
                udp_listener.set_hosts(hosts);
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
