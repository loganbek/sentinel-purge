//! # SentinelPurge
//!
//! Advanced, cross-platform APT removal tool with comprehensive stealth capabilities.
//!
//! ## Core Modules
//!
//! - **stealth**: Covert operations and anti-detection features
//! - **scanner**: Threat detection and analysis engine
//! - **remediation**: Gradual threat removal capabilities
//! - **forensics**: System baseline and forensic analysis
//!
//! ## Security First
//!
//! This library prioritizes memory safety, secure error handling, and 
//! cryptographic best practices throughout all components.

pub mod stealth;
pub mod error;
pub mod config;

pub use error::{SentinelError, Result};
pub use config::SentinelConfig;

/// SentinelPurge version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the SentinelPurge runtime with default configuration
pub async fn init() -> Result<()> {
    // Only initialize tracing if it hasn't been initialized already
    if tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .is_err()
    {
        // Subscriber already set, just continue
    }
    
    tracing::info!("SentinelPurge {} initializing", VERSION);
    Ok(())
}

/// Initialize SentinelPurge with custom configuration
pub async fn init_with_config(config: SentinelConfig) -> Result<()> {
    // Only initialize tracing if it hasn't been initialized already
    if tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(&config.log_level))
        .try_init()
        .is_err()
    {
        // Subscriber already set, just continue
    }
    
    tracing::info!("SentinelPurge {} initializing with custom config", VERSION);
    Ok(())
}