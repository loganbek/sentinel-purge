//! Error handling for SentinelPurge
//!
//! This module provides comprehensive error types and handling for the entire
//! SentinelPurge system, with a focus on security and non-disclosure of sensitive
//! information in error messages.

use thiserror::Error;

/// Main error type for SentinelPurge operations
#[derive(Error, Debug)]
pub enum SentinelError {
    /// I/O operation errors
    #[error("I/O operation failed")]
    Io(#[from] std::io::Error),

    /// Serialization/deserialization errors
    #[error("Serialization error")]
    Serde(#[from] serde_json::Error),

    /// Configuration errors
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Stealth operation errors
    #[error("Stealth operation failed")]
    Stealth { operation: String },

    /// Platform-specific operation errors
    #[error("Platform operation not supported")]
    PlatformNotSupported,

    /// Permission/privilege errors
    #[error("Insufficient privileges")]
    InsufficientPrivileges,

    /// Process operation errors
    #[error("Process operation failed")]
    ProcessOperation,

    /// Memory operation errors
    #[error("Memory operation failed")]
    MemoryOperation,

    /// Network operation errors
    #[error("Network operation failed")]
    NetworkOperation,

    /// Generic internal error (avoid exposing internal details)
    #[error("Internal operation failed")]
    Internal,
}

/// Result type alias for SentinelPurge operations
pub type Result<T> = std::result::Result<T, SentinelError>;

impl SentinelError {
    /// Create a configuration error with a message
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a stealth operation error
    pub fn stealth<S: Into<String>>(operation: S) -> Self {
        Self::Stealth {
            operation: operation.into(),
        }
    }

    /// Check if the error is related to insufficient privileges
    pub fn is_privilege_error(&self) -> bool {
        matches!(self, Self::InsufficientPrivileges)
    }

    /// Check if the error is platform-related
    pub fn is_platform_error(&self) -> bool {
        matches!(self, Self::PlatformNotSupported)
    }
}