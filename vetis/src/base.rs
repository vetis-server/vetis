use crate::{server::ServerConfig, VetisResult};
use std::future::Future;

/// Base trait for Vetis server
pub trait VetisServer {
    /// Async runtime listener type
    type RuntimeListener;
    /// Async runtime host type
    type RuntimeHost;
    /// Get server configuration
    fn config(&self) -> &ServerConfig;
    /// Get mutable server configuration
    fn config_mut(&mut self) -> &mut ServerConfig;
    /// Run the server
    fn run(&mut self) -> impl Future<Output = VetisResult<()>>;
    /// Start the server
    fn start(&mut self) -> impl Future<Output = VetisResult<()>>;
    /// Stop the server
    fn stop(&mut self) -> impl Future<Output = VetisResult<()>>;
    /// Reload the server configuration
    fn reload(&mut self, new_config: ServerConfig) -> impl Future<Output = VetisResult<()>>;
}
