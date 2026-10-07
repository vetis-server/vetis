use crate::{VetisResult, server::ServerConfig};
use std::future::Future;

/// Base trait for Vetis server
pub trait VetisServer {
    /// Async runtime listener type
    type RuntimeListener;
    /// Async runtime host type
    type RuntimeHost;
    /// Get server configuration
    fn config(&self) -> &ServerConfig;
    /// Run the server
    fn run(&mut self) -> impl Future<Output = VetisResult<()>>;
    /// Start the server
    fn start(&mut self) -> impl Future<Output = VetisResult<()>>;
    /// Stop the server
    fn stop(self) -> impl Future<Output = VetisResult<()>>;
}
