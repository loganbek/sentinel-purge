//! SentinelPurge Main Binary
//!
//! Advanced, cross-platform APT removal tool with comprehensive stealth capabilities.

use clap::{Arg, Command};
use sentinel_purge::{init_with_config, SentinelConfig};
use sentinel_purge::stealth::{init_stealth, StealthController};
use std::process;
use std::sync::Arc;
use tokio::signal;
use tracing::{info, error, warn};

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let matches = Command::new("SentinelPurge")
        .version(sentinel_purge::VERSION)
        .about("Advanced, cross-platform APT removal tool with stealth capabilities")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
        )
        .arg(
            Arg::new("stealth-mode")
                .short('s')
                .long("stealth")
                .value_name("MODE")
                .help("Stealth operation mode (silent, hibernation, mimicry, ghost, adaptive)")
                .default_value("silent")
        )
        .arg(
            Arg::new("log-level")
                .short('l')
                .long("log-level")
                .value_name("LEVEL")
                .help("Log level (trace, debug, info, warn, error)")
                .default_value("info")
        )
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .help("Run as daemon/service")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Load configuration
    let config = if let Some(config_path) = matches.get_one::<String>("config") {
        match SentinelConfig::from_file(config_path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to load configuration: {}", e);
                process::exit(1);
            }
        }
    } else {
        // Try environment variables, fallback to default
        SentinelConfig::from_env().unwrap_or_default()
    };

    // Override log level if specified
    let mut final_config = config;
    if let Some(log_level) = matches.get_one::<String>("log-level") {
        final_config.log_level = log_level.clone();
    }

    // Override stealth mode if specified
    if let Some(stealth_mode) = matches.get_one::<String>("stealth-mode") {
        final_config.stealth.mode = match stealth_mode.as_str() {
            "silent" => sentinel_purge::config::StealthMode::Silent,
            "hibernation" => sentinel_purge::config::StealthMode::Hibernation,
            "mimicry" => sentinel_purge::config::StealthMode::Mimicry,
            "ghost" => sentinel_purge::config::StealthMode::Ghost,
            "adaptive" => sentinel_purge::config::StealthMode::Adaptive,
            _ => {
                eprintln!("Invalid stealth mode: {}", stealth_mode);
                process::exit(1);
            }
        };
    }

    // Initialize SentinelPurge
    if let Err(e) = init_with_config(final_config.clone()).await {
        eprintln!("Failed to initialize SentinelPurge: {}", e);
        process::exit(1);
    }

    info!("SentinelPurge {} starting", sentinel_purge::VERSION);

    // Initialize stealth subsystem
    let stealth_controller = match init_stealth(&final_config).await {
        Ok(controller) => controller,
        Err(e) => {
            error!("Failed to initialize stealth subsystem: {}", e);
            process::exit(1);
        }
    };

    // Start stealth operations
    if let Err(e) = stealth_controller.start().await {
        error!("Failed to start stealth operations: {}", e);
        process::exit(1);
    }

    info!("Stealth operations started successfully");

    // Wrap stealth controller in Arc for sharing
    let stealth_controller = Arc::new(stealth_controller);

    // Set up signal handlers for graceful shutdown
    let stealth_controller_shutdown = Arc::clone(&stealth_controller);
    tokio::spawn(async move {
        if let Err(e) = signal::ctrl_c().await {
            error!("Failed to listen for shutdown signal: {}", e);
            return;
        }

        info!("Received shutdown signal, cleaning up...");
        
        if let Err(e) = stealth_controller_shutdown.stop().await {
            error!("Failed to stop stealth operations: {}", e);
        }

        info!("Shutdown completed");
        process::exit(0);
    });

    // Main operation loop
    if matches.get_flag("daemon") {
        info!("Running in daemon mode");
        run_daemon_mode(&stealth_controller).await;
    } else {
        info!("Running in interactive mode");
        run_interactive_mode(&stealth_controller).await;
    }
}

/// Run in daemon/service mode
async fn run_daemon_mode(stealth_controller: &Arc<StealthController>) {
    info!("SentinelPurge daemon started");
    
    let config = SentinelConfig::default(); // Create config locally
    
    // Main daemon loop
    loop {
        // Check stealth status
        let metrics = stealth_controller.get_metrics().await;
        
        // Log periodic status (but not too frequently to avoid detection)
        tokio::time::sleep(tokio::time::Duration::from_secs(300)).await; // 5 minutes
        
        if !stealth_controller.is_active().await {
            warn!("Stealth controller is not active, attempting restart");
            if let Err(e) = stealth_controller.start().await {
                error!("Failed to restart stealth controller: {}", e);
            }
        }
        
        // Adaptive behavior based on metrics
        if let Err(e) = stealth_controller.adapt_behavior().await {
            error!("Failed to adapt behavior: {}", e);
        }
        
        // Check resource usage
        if !metrics.is_within_resource_limits(&config) {
            warn!("Resource usage exceeds limits, triggering evasion");
            if let Err(e) = stealth_controller.trigger_evasion().await {
                error!("Failed to trigger evasion: {}", e);
            }
        }
    }
}

/// Run in interactive mode
async fn run_interactive_mode(stealth_controller: &Arc<StealthController>) {
    info!("SentinelPurge interactive mode started");
    
    println!("SentinelPurge {} - Interactive Mode", sentinel_purge::VERSION);
    println!("Type 'help' for available commands");
    
    loop {
        print!("> ");
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        
        let command = input.trim();
        match command {
            "help" => show_help(),
            "status" => show_status(stealth_controller).await,
            "metrics" => show_metrics(stealth_controller).await,
            "evasion" => trigger_evasion(stealth_controller).await,
            "sleep" => enter_sleep_mode(stealth_controller).await,
            "adapt" => adapt_behavior(stealth_controller).await,
            "quit" | "exit" => break,
            "" => continue,
            _ => println!("Unknown command: {}. Type 'help' for available commands.", command),
        }
    }
    
    info!("Interactive mode exiting");
}

fn show_help() {
    println!("Available commands:");
    println!("  help     - Show this help message");
    println!("  status   - Show stealth controller status");
    println!("  metrics  - Show detailed stealth metrics");
    println!("  evasion  - Trigger immediate evasion response");
    println!("  sleep    - Enter sleep mode");
    println!("  adapt    - Adapt behavior based on environment");
    println!("  quit     - Exit SentinelPurge");
}

async fn show_status(stealth_controller: &Arc<StealthController>) {
    let is_active = stealth_controller.is_active().await;
    let metrics = stealth_controller.get_metrics().await;
    
    println!("Stealth Controller Status:");
    println!("  Active: {}", is_active);
    println!("  Mode: {:?}", metrics.status);
    println!("  CPU Usage: {:.2}%", metrics.cpu_usage);
    println!("  Memory Usage: {} MB", metrics.memory_usage_mb);
    println!("  Evasion Success Rate: {:.1}%", metrics.evasion_success_rate() * 100.0);
}

async fn show_metrics(stealth_controller: &Arc<StealthController>) {
    let metrics = stealth_controller.get_metrics().await;
    
    println!("Detailed Stealth Metrics:");
    println!("  Status: {:?}", metrics.status);
    println!("  Resource Usage:");
    println!("    CPU: {:.2}%", metrics.cpu_usage);
    println!("    Memory: {} MB", metrics.memory_usage_mb);
    println!("  Evasion Metrics:");
    println!("    Attempts: {}", metrics.evasion_attempts);
    println!("    Successes: {}", metrics.successful_evasions);
    println!("    Success Rate: {:.1}%", metrics.evasion_success_rate() * 100.0);
    println!("  Sleep Metrics:");
    println!("    Cycles Completed: {}", metrics.sleep_cycles_completed);
    println!("    Total Sleep Time: {} seconds", metrics.total_sleep_time_secs);
    println!("  Identity Changes: {}", metrics.identity_changes);
}

async fn trigger_evasion(stealth_controller: &Arc<StealthController>) {
    println!("Triggering evasion response...");
    match stealth_controller.trigger_evasion().await {
        Ok(()) => println!("Evasion response completed successfully"),
        Err(e) => println!("Evasion response failed: {}", e),
    }
}

async fn enter_sleep_mode(stealth_controller: &Arc<StealthController>) {
    println!("Entering sleep mode for 60 seconds...");
    let duration = tokio::time::Duration::from_secs(60);
    match stealth_controller.enter_sleep_mode(Some(duration)).await {
        Ok(()) => println!("Sleep mode completed"),
        Err(e) => println!("Sleep mode failed: {}", e),
    }
}

async fn adapt_behavior(stealth_controller: &Arc<StealthController>) {
    println!("Adapting behavior based on environment...");
    match stealth_controller.adapt_behavior().await {
        Ok(()) => println!("Behavior adaptation completed"),
        Err(e) => println!("Behavior adaptation failed: {}", e),
    }
}