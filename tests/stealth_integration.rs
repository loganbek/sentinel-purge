//! Integration tests for SentinelPurge stealth components

use sentinel_purge::{
    init_with_config, SentinelConfig, 
    stealth::{init_stealth, StealthStatus}
};

#[tokio::test]
async fn test_stealth_initialization() {
    // Test basic initialization
    let config = SentinelConfig::default();
    
    // Initialize SentinelPurge
    init_with_config(config.clone()).await.expect("Failed to initialize SentinelPurge");
    
    // Initialize stealth subsystem
    let stealth_controller = init_stealth(&config).await
        .expect("Failed to initialize stealth subsystem");
    
    // Check initial state
    assert!(!stealth_controller.is_active().await);
    
    let metrics = stealth_controller.get_metrics().await;
    assert!(matches!(metrics.status, StealthStatus::Inactive));
}

#[tokio::test]
async fn test_stealth_start_stop() {
    let config = SentinelConfig::default();
    init_with_config(config.clone()).await.expect("Failed to initialize SentinelPurge");
    
    let stealth_controller = init_stealth(&config).await
        .expect("Failed to initialize stealth subsystem");
    
    // Start stealth operations
    stealth_controller.start().await.expect("Failed to start stealth operations");
    assert!(stealth_controller.is_active().await);
    
    let metrics = stealth_controller.get_metrics().await;
    assert!(!matches!(metrics.status, StealthStatus::Inactive));
    
    // Stop stealth operations
    stealth_controller.stop().await.expect("Failed to stop stealth operations");
    assert!(!stealth_controller.is_active().await);
}

#[tokio::test]
async fn test_stealth_configuration() {
    let mut config = SentinelConfig::default();
    
    // Test different stealth modes
    config.stealth.mode = sentinel_purge::config::StealthMode::Silent;
    let stealth_controller = init_stealth(&config).await
        .expect("Failed to initialize with silent mode");
    
    config.stealth.mode = sentinel_purge::config::StealthMode::Ghost;
    let stealth_controller = init_stealth(&config).await
        .expect("Failed to initialize with ghost mode");
    
    config.stealth.mode = sentinel_purge::config::StealthMode::Adaptive;
    let stealth_controller = init_stealth(&config).await
        .expect("Failed to initialize with adaptive mode");
}

#[tokio::test]
async fn test_resource_limits() {
    let config = SentinelConfig::default();
    init_with_config(config.clone()).await.expect("Failed to initialize SentinelPurge");
    
    let stealth_controller = init_stealth(&config).await
        .expect("Failed to initialize stealth subsystem");
    
    let metrics = stealth_controller.get_metrics().await;
    
    // Test resource limit checking
    assert!(metrics.is_within_resource_limits(&config));
    
    // Test evasion success rate calculation
    let success_rate = metrics.evasion_success_rate();
    assert!(success_rate >= 0.0 && success_rate <= 1.0);
}

#[tokio::test]
async fn test_stealth_metrics() {
    let config = SentinelConfig::default();
    init_with_config(config.clone()).await.expect("Failed to initialize SentinelPurge");
    
    let stealth_controller = init_stealth(&config).await
        .expect("Failed to initialize stealth subsystem");
    
    stealth_controller.start().await.expect("Failed to start stealth operations");
    
    // Give some time for metrics to be updated
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let metrics = stealth_controller.get_metrics().await;
    
    // Basic metrics validation
    assert!(metrics.cpu_usage >= 0.0);
    assert!(metrics.memory_usage_mb >= 0);
    assert!(metrics.evasion_attempts >= 0);
    assert!(metrics.successful_evasions >= 0);
    assert!(metrics.sleep_cycles_completed >= 0);
    assert!(metrics.total_sleep_time_secs >= 0);
    assert!(metrics.identity_changes >= 0);
    
    stealth_controller.stop().await.expect("Failed to stop stealth operations");
}

#[tokio::test]
async fn test_evasion_trigger() {
    let config = SentinelConfig::default();
    init_with_config(config.clone()).await.expect("Failed to initialize SentinelPurge");
    
    let stealth_controller = init_stealth(&config).await
        .expect("Failed to initialize stealth subsystem");
    
    stealth_controller.start().await.expect("Failed to start stealth operations");
    
    // Trigger evasion response
    stealth_controller.trigger_evasion().await
        .expect("Failed to trigger evasion");
    
    let metrics = stealth_controller.get_metrics().await;
    assert!(metrics.evasion_attempts > 0);
    
    stealth_controller.stop().await.expect("Failed to stop stealth operations");
}

#[tokio::test]
async fn test_config_validation() {
    let mut config = SentinelConfig::default();
    
    // Test valid configuration
    assert!(config.validate().is_ok());
    
    // Test invalid CPU usage
    config.stealth.max_cpu_usage = 150.0; // > 100%
    assert!(config.validate().is_err());
    
    // Reset and test invalid memory
    config = SentinelConfig::default();
    config.stealth.max_memory_mb = 0;
    assert!(config.validate().is_err());
    
    // Reset and test invalid sleep configuration
    config = SentinelConfig::default();
    config.sleep.min_sleep_secs = 1000;
    config.sleep.max_sleep_secs = 500; // min > max
    assert!(config.validate().is_err());
}