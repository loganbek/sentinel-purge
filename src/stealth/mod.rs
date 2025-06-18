//! # Stealth Module
//!
//! Advanced stealth capabilities for SentinelPurge, providing covert operations
//! and anti-detection features for threat hunting and remediation.
//!
//! ## Core Components
//!
//! - **Controller**: Central management of all covert operations
//! - **Identity**: Process identity and disguise operations
//! - **Sleep**: Dormancy periods and awakening conditions
//! - **Evasion**: Anti-analysis and detection evasion capabilities
//! - **Communication**: Steganography and covert communications

pub mod controller;
pub mod identity;
pub mod sleep;
pub mod evasion;
pub mod communication;
pub mod platform;

pub use controller::StealthController;
pub use identity::IdentityManager;
pub use sleep::SleepScheduler;
pub use evasion::EvasionEngine;
pub use communication::CommunicationSteganography;

use crate::config::SentinelConfig;
use crate::error::Result;

/// Initialize the stealth subsystem with configuration
pub async fn init_stealth(config: &SentinelConfig) -> Result<StealthController> {
    tracing::info!("Initializing stealth subsystem");
    
    if !config.stealth.enabled {
        tracing::warn!("Stealth mode is disabled in configuration");
    }
    
    StealthController::new(config.clone()).await
}

/// Stealth operation status
#[derive(Debug, Clone)]
pub enum StealthStatus {
    /// Stealth mode is inactive
    Inactive,
    /// Operating in silent mode
    Silent,
    /// In hibernation/sleep mode
    Hibernating,
    /// Mimicking legitimate processes
    Mimicking,
    /// Ghost mode - advanced evasion
    Ghost,
    /// Adaptive mode - dynamic behavior
    Adaptive,
}

/// Stealth metrics for monitoring and assessment
#[derive(Debug, Clone)]
pub struct StealthMetrics {
    /// Current stealth status
    pub status: StealthStatus,
    /// Resource usage metrics
    pub cpu_usage: f32,
    pub memory_usage_mb: u64,
    /// Detection evasion metrics
    pub evasion_attempts: u64,
    pub successful_evasions: u64,
    /// Sleep cycle metrics
    pub sleep_cycles_completed: u64,
    pub total_sleep_time_secs: u64,
    /// Identity changes
    pub identity_changes: u64,
}

impl Default for StealthMetrics {
    fn default() -> Self {
        Self {
            status: StealthStatus::Inactive,
            cpu_usage: 0.0,
            memory_usage_mb: 0,
            evasion_attempts: 0,
            successful_evasions: 0,
            sleep_cycles_completed: 0,
            total_sleep_time_secs: 0,
            identity_changes: 0,
        }
    }
}

impl StealthMetrics {
    /// Calculate evasion success rate
    pub fn evasion_success_rate(&self) -> f32 {
        if self.evasion_attempts == 0 {
            return 1.0; // No attempts means perfect success
        }
        self.successful_evasions as f32 / self.evasion_attempts as f32
    }

    /// Check if resource usage is within limits
    pub fn is_within_resource_limits(&self, config: &SentinelConfig) -> bool {
        self.cpu_usage <= config.stealth.max_cpu_usage
            && self.memory_usage_mb <= config.stealth.max_memory_mb
    }
}