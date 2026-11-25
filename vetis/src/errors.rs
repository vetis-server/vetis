//! Error handling types for VeTiS.
//!
//! This module defines the comprehensive error types used throughout
//! the VeTiS server, providing detailed error information for
//! configuration, runtime, and TLS-related issues.
//! ```

use crate::Str;
use http::{HeaderMap, StatusCode};
use std::fmt::Display;
use thiserror::Error;

/// Main error type for VeTiS operations.
///
/// This enum encompasses all possible errors that can occur during
/// server configuration, startup, and operation. Each variant provides
/// specific context about what went wrong.
#[derive(Debug, Error, PartialEq)]
pub enum VetisError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    /// Failed to bind to a network address
    #[error("Failed to bind to address: {0}")]
    Bind(Str),
    /// Failed to log message
    #[error("Failed to log message: {0}")]
    Log(Str),
    /// IO error
    #[error("IO error: {0}")]
    Io(Str),
    /// Server startup errors
    #[error("Failed to start server: {0}")]
    Startup(#[from] StartupError),
    /// Server shutdown errors
    #[error("Failed to stop server: {0}")]
    Stop(Str),
    /// Request handler errors
    #[error("Handler error: {0}")]
    Handler(Str),
    /// Request handler errors
    #[error("Hook error: {0}")]
    Hook(Str),
    /// UDP related errors
    #[error("Udp error: {0}")]
    Udp(Str),
    /// TCP related errors
    #[error("Tcp error: {0}")]
    Tcp(Str),
    /// TLS/SSL related errors
    #[error("Tls error: {0}")]
    Tls(Str),
    /// No server instances are running
    #[error("No instances")]
    NoInstances,
    /// Listener related errors
    #[error("Listener error: {0}")]
    Listener(#[from] ListenerError),
    /// Virtual host related errors
    #[error("Host error: {0}")]
    Host(#[from] HostError),
    /// Matcher related errors
    #[error("Matcher error: {0}")]
    Matcher(Str),
}

impl VetisError {
    /// Create a new VetisError::Config variant
    pub fn config(inner: ConfigError) -> Self {
        Self::Config(inner)
    }

    /// Create a new VetisError::Host variant
    pub fn host(inner: HostError) -> Self {
        Self::Host(inner)
    }

    /// Create a new VetisError::Listener variant
    pub fn listener(inner: ListenerError) -> Self {
        Self::Listener(inner)
    }

    /// Create a new VetisError::Startup variant
    pub fn startup(inner: StartupError) -> Self {
        Self::Startup(inner)
    }

    /// Create a new VetisError::Bind variant
    pub fn bind(field: impl Into<Str>) -> Self {
        Self::Bind(field.into())
    }

    /// Create a new ConfigError::Log variant
    pub fn log(field: impl Into<Str>) -> Self {
        Self::Log(field.into())
    }

    /// Create a new ConfigError::Listener variant
    pub fn io(field: impl Into<Str>) -> Self {
        Self::Io(field.into())
    }

    /// Create a new ConfigError::Stop variant
    pub fn stop(field: impl Into<Str>) -> Self {
        Self::Stop(field.into())
    }

    /// Create a new ConfigError::hook variant
    pub fn hook(field: impl Into<Str>) -> Self {
        Self::Hook(field.into())
    }

    /// Create a new ConfigError::Handler variant
    pub fn handler(field: impl Into<Str>) -> Self {
        Self::Handler(field.into())
    }

    /// Create a new ConfigError::Udp variant
    pub fn udp(field: impl Into<Str>) -> Self {
        Self::Udp(field.into())
    }

    /// Create a new ConfigError::Tcp variant
    pub fn tcp(field: impl Into<Str>) -> Self {
        Self::Tcp(field.into())
    }

    /// Create a new ConfigError::Tls variant
    pub fn tls(field: impl Into<Str>) -> Self {
        Self::Tls(field.into())
    }

    /// Create a new ConfigError::Matcher variant
    pub fn matcher(field: impl Into<Str>) -> Self {
        Self::Matcher(field.into())
    }
}

/// Configuration-related errors.
///
/// These errors occur during the parsing and validation of
/// server and virtual host configurations.
#[derive(Debug, Clone, Error, PartialEq)]
pub enum ConfigError {
    /// Invalid auth configuration
    #[error("Invalid auth config: {0}")]
    Auth(Str),
    /// Invalid server configuration
    #[error("Invalid server config: {0}")]
    Server(Str),
    /// Invalid listener configuration
    #[error("Invalid listener config: {0}")]
    Listener(Str),
    /// Invalid host configuration
    #[error("Invalid host config: {0}")]
    Host(Str),
    /// Invalid hook configuration
    #[error("Invalid hook config: {0}")]
    Hook(Str),
    /// Invalid tls configuration
    #[error("Invalid tls config: {0}")]
    Tls(Str),
    /// Invalid matcher configuration
    #[error("Invalid matcher config: {0}")]
    Matcher(Str),
}

impl ConfigError {
    /// Create a new ConfigError::Auth variant
    pub fn auth(field: impl Into<Str>) -> Self {
        Self::Auth(field.into())
    }

    /// Create a new ConfigError::Server variant
    pub fn server(field: impl Into<Str>) -> Self {
        Self::Server(field.into())
    }

    /// Create a new ConfigError::Listener variant
    pub fn listener(field: impl Into<Str>) -> Self {
        Self::Listener(field.into())
    }

    /// Create a new ConfigError::Host variant
    pub fn host(field: impl Into<Str>) -> Self {
        Self::Host(field.into())
    }

    /// Create a new ConfigError::Hook variant
    pub fn hook(field: impl Into<Str>) -> Self {
        Self::Hook(field.into())
    }

    /// Create a new ConfigError::Tls variant
    pub fn tls(field: impl Into<Str>) -> Self {
        Self::Tls(field.into())
    }

    /// Create a new ConfigError::Matcher variant
    pub fn matcher(field: impl Into<Str>) -> Self {
        Self::Matcher(field.into())
    }
}

/// Listener-related errors.
///
/// These errors occur when working with listeners,
/// such as missing listeners or configuration issues.
#[derive(Debug, Clone, Error, PartialEq)]
pub enum ListenerError {
    /// Bind
    #[error("Cannot bind listener: {0}")]
    Bind(Str),
    /// No listeners configured
    #[error("No listeners configured")]
    NoListeners,
}

impl ListenerError {
    /// Create a new StartupError::tls variant
    pub fn bind(field: impl Into<Str>) -> Self {
        Self::Bind(field.into())
    }
}

/// Server startup errors.
///
/// These errors occur when the server fails to start properly,
/// typically due to TLS initialization issues.
#[derive(Debug, Clone, Error, PartialEq)]
pub enum StartupError {
    /// TLS/SSL initialization errors
    #[error("Tls initialization: {0}")]
    Tls(Str),
    /// Server is already running
    #[error("Server is already running")]
    AlreadyRunning,
}

impl StartupError {
    /// Create a new StartupError::tls variant
    pub fn init(field: impl Into<Str>) -> Self {
        Self::Tls(field.into())
    }
}

/// Host related errors.'
///
/// These errors occur when working with virtual hosts,
/// such as missing handlers or configuration issues.
#[derive(Debug, Clone, Error, PartialEq)]
pub enum HostError {
    /// Initialization error
    #[error("Initialization error: {0}")]
    Init(Str),
    /// No virtual hosts have been added to the server
    #[error("No hosts")]
    NoHosts,
    /// Handler errors
    #[error("Runtime error: {0}")]
    Runtime(RuntimeError),
}

impl HostError {
    /// Create a new HostError::init variant
    pub fn init(field: impl Into<Str>) -> Self {
        Self::Init(field.into())
    }
}

/// Runtime related errors.
///
/// These errors occur when working with hooks.
#[derive(Debug, Clone, Error, PartialEq)]
pub struct RuntimeError {
    message: Str,
    status_code: StatusCode,
    headers: Option<HeaderMap>,
}

impl RuntimeError {
    /// Create a new RuntimeError
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            headers: None,
        }
    }

    /// Add status code to error
    pub fn with_status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    /// Add headers to error
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Return response status code
    pub fn status_code(&self) -> &StatusCode {
        &self.status_code
    }

    /// Return response headers
    pub fn headers(&self) -> Option<&HeaderMap> {
        self.headers
            .as_ref()
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.message))
    }
}

impl Into<VetisError> for RuntimeError {
    fn into(self) -> VetisError {
        VetisError::Host(HostError::Runtime(self))
    }
}
