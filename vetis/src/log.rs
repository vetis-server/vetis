use std::sync::Arc;

use crate::{VetisResult, errors::VetisError};
use crossfire::AsyncTxTrait;
use log::Level;
use logforth_core::DispatchBuilder;

#[typetag::serde(tag = "dest")]
/// A trait which describe log configuration
pub trait LogConfig {
    /// Sets the log level
    ///
    /// # Arguments
    ///
    /// * `value` - The log level to set
    fn log_level(&mut self, level: Level);
    /// Allow clone any type which implements
    /// LogConfig
    ///
    /// # Returns
    ///
    /// - `Box<dyn PathConfig>` - Clone of boxed log config
    fn boxed_clone(&self) -> Box<dyn LogConfig>;
    /// Allow clone any type which implements
    /// LogConfig
    ///
    /// # Returns
    ///
    /// - `DispatchBuilder<true>` - Clone of boxed log config
    fn into_builder(&self, target: &str, input: DispatchBuilder<false>) -> DispatchBuilder<true>;
}

impl Clone for Box<dyn LogConfig> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}

/// LogMessage type
pub struct LogMessage {
    level: Arc<str>,
    target: Arc<str>,
    message: Arc<str>,
}

impl LogMessage {
    /// Create a new LogMessage builder
    pub fn info(target: &str, message: &str) -> LogMessage {
        LogMessage { target: target.into(), level: "INFO".into(), message: message.into() }
    }

    /// Create a new LogMessage builder
    pub fn error(target: &str, message: &str) -> LogMessage {
        LogMessage { target: target.into(), level: "ERROR".into(), message: message.into() }
    }

    /// Create a new LogMessage builder
    pub fn debug(target: &str, message: &str) -> LogMessage {
        LogMessage { target: target.into(), level: "DEBUG".into(), message: message.into() }
    }

    /// Returns target
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns level
    pub fn level(&self) -> &str {
        &self.level
    }

    /// Returns message
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Logger type
pub struct Logger<S> {
    sender: S,
}

impl<S> Logger<S>
where
    S: AsyncTxTrait<LogMessage>,
{
    /// Initialize a new logger
    pub fn new(sender: S) -> Self {
        Self { sender }
    }

    /// Info log message
    pub async fn info(&self, target: &str, message: &str) -> VetisResult<()> {
        self.sender
            .send(LogMessage::info(target, message))
            .await
            .map_err(|e| VetisError::Log(e.to_string()))
    }

    /// Error log message
    pub async fn error(&self, target: &str, message: &str) -> VetisResult<()> {
        self.sender
            .send(LogMessage::error(target, message))
            .await
            .map_err(|e| VetisError::Log(e.to_string()))
    }

    /// Debug log message
    pub async fn debug(&self, target: &str, message: &str) -> VetisResult<()> {
        self.sender
            .send(LogMessage::debug(target, message))
            .await
            .map_err(|e| VetisError::Log(e.to_string()))
    }
}

impl<S> Clone for Logger<S>
where
    S: AsyncTxTrait<LogMessage> + Clone,
{
    fn clone(&self) -> Self {
        Logger { sender: self.sender.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.sender
            .clone_from(&source.sender);
    }
}

/// Log an error
pub fn log_error<'a>(e: &'a VetisError) {
    eprintln!("Log error: {:?}", e);
}

#[macro_export]
/// This macro is used to simplify call to log messages with debug log level
/// It is intended to be use ONLY after vetis::Logger initialization, and it
/// always takes logger as first argument
macro_rules! debug {
    ($logger:expr, $message:expr) => {
        if let Some(ref logger) = $logger {
            let _ = logger.debug("vetis", $message).await.inspect_err($crate::log::log_error);
        }
    };

    ($logger:expr, $message:literal, $($args:tt)*) => {
        if let Some(ref logger) = $logger {
            let _ = logger.debug("vetis", &std::format!($message, $($args)*)).await.inspect_err($crate::log::log_error);
        }
    };

    ($logger:expr, target: $target:expr, $message:literal) => {
        if let Some(ref logger) = $logger {
            let _ = logger.debug($target, &std::format!($message)).await.inspect_err($crate::log::log_error);
        }
    };

    ($logger:expr, target: $target:expr, $message:literal, $($args:tt)*) => {
        if let Some(ref logger) = $logger {
            let _ = logger.debug($target, &std::format!($message, $($args)*)).await.inspect_err($crate::log::log_error);
        }
    };
}

#[macro_export]
/// This macro is used to simplify call to log messages with info log level
/// It is intended to be use ONLY after vetis::Logger initialization, and it
/// always takes logger as first argument
macro_rules! info {
    ($logger:expr, $message:expr) => {
        if let Some(logger) = $logger {
            let _ = logger.info("vetis", $message).await.inspect_err($crate::log::log_error);
        }
    };

    ($logger:expr, $message:literal, $($args:tt)*) => {
        if let Some(logger) = $logger {
            let _ = logger.info("vetis", &std::format!($message, $($args)*)).await.inspect_err($crate::log::log_error);
        }
    };

    ($logger:expr, target: $target:expr, $message:literal) => {
        if let Some(ref logger) = $logger {
            let _ = logger.info($target, &std::format!($message)).await.inspect_err($crate::log::log_error);
        }
    };

    ($logger:expr, target: $target:expr, $message:literal, $($args:tt)*) => {
        if let Some(ref logger) = $logger {
            let _ = logger.info($target, &std::format!($message, $($args)*)).await.inspect_err($crate::log::log_error);
        }
    };
}

#[macro_export]
/// This macro is used to simplify call to log messages with error log level
/// It is intended to be use ONLY after vetis::Logger initialization, and it
/// always takes logger as first argument
macro_rules! error {
    ($logger:expr, $message:expr) => {
        if let Some(logger) = $logger {
            let _ = logger.error("vetis", $message).await.inspect_err($crate::log::log_error);
        }
    };

    ($logger:expr, $message:literal, $($args:tt)*) => {
        if let Some(ref logger) = $logger {
            let _ = logger.error("vetis", &std::format!($message, $($args)*)).await.inspect_err($crate::log::log_error);
        }
    };

    ($logger:expr, target: $target:expr, $message:literal) => {
        if let Some(ref logger) = $logger {
            let _ = logger.error($target, &std::format!($message)).await.inspect_err($crate::log::log_error);
        }
    };

    ($logger:expr, target: $target:expr, $message:literal, $($args:tt)*) => {
        if let Some(ref logger) = $logger {
            let _ = logger.error($target, &std::format!($message, $($args)*)).await.inspect_err($crate::log::log_error);
        }
    };
}
