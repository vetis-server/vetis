use crate::{server::ServerConfig, VetisHosts, VetisResult};
use std::future::Future;

/// Base trait for Vetis server
pub trait VetisServer {
    /// Host type
    type Host;
    /// Host configuration type
    type HostConfig;
    /// Add a host to the server
    fn add_host(&mut self, host: Self::Host) -> impl Future<Output = ()>;
    /// Remove a host from the server
    fn remove_host(&mut self, hostname: &str) -> impl Future<Output = ()>;
    /// Get hosts
    fn hosts(&self) -> &VetisHosts<Self::Host>;
    /// Get server configuration
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
        new_hosts: Vec<Self::HostConfig>,
    ) -> impl Future<Output = VetisResult<()>>;
}
