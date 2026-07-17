use crate::{server::ServerConfig, VetisResult, VetisVirtualHosts};
use std::future::Future;

/// Base trait for Vetis server
pub trait VetisServer {
    /// Virtual host type
    type VirtualHost;
    /// Virtual host configuration type
    type VirtualHostConfig;
    /// Add a virtual host to the server
    fn add_virtual_host(&mut self, virtual_host: Self::VirtualHost) -> impl Future<Output = ()>;
    /// Remove a virtual host from the server
    fn remove_virtual_host(&mut self, hostname: &str, port: u16) -> impl Future<Output = ()>;
    /// Get the virtual hosts
    fn virtual_hosts(&self) -> &VetisVirtualHosts<Self::VirtualHost>;
    /// Get the server configuration
    fn config(&self) -> &ServerConfig;
    /// Run the server
    fn run(&mut self) -> impl Future<Output = VetisResult<()>>;
    /// Start the server
    fn start(&mut self) -> impl Future<Output = VetisResult<()>>;
    /// Stop the server
    fn stop(&mut self) -> impl Future<Output = VetisResult<()>>;
    /// Reload the server configuration
    fn reload(
        &mut self,
        new_config: ServerConfig,
        new_virtual_hosts: Vec<Self::VirtualHostConfig>,
    ) -> impl Future<Output = VetisResult<()>>;
}
