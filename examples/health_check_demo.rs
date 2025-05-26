//! Health Check Demo
//!
//! This example demonstrates how to use the health check system in PMDaemon.
//! It shows both HTTP and script-based health checks with different configurations.

use pmdaemon::{HealthCheck, HealthCheckConfig, ProcessConfig, Result};
use std::time::Duration;
use tokio::time::sleep;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init();

    println!("🏥 PMDaemon Health Check Demo");
    println!("==============================\n");

    // Demo 1: HTTP Health Check
    demo_http_health_check().await?;

    // Demo 2: Script Health Check
    demo_script_health_check().await?;

    // Demo 3: Process with Health Check Configuration
    demo_process_with_health_check().await?;

    println!("✅ All health check demos completed successfully!");
    Ok(())
}

/// Demonstrates HTTP health check functionality
async fn demo_http_health_check() -> Result<()> {
    println!("📡 Demo 1: HTTP Health Check");
    println!("-----------------------------");

    // Create an HTTP health check configuration
    let config = HealthCheckConfig::http("https://httpbin.org/status/200")
        .timeout(Duration::from_secs(10))
        .interval(Duration::from_secs(30))
        .retries(3)
        .enabled(true);

    let mut health_check = HealthCheck::new(config);

    println!("🔍 Performing HTTP health check to: https://httpbin.org/status/200");

    match health_check.check().await {
        Ok(status) => {
            println!("✅ Health check completed!");
            println!("   State: {:?}", status.state);
            println!("   Total checks: {}", status.total_checks);
            println!("   Consecutive failures: {}", status.consecutive_failures);

            if let Some(last_check) = status.last_check {
                println!("   Last check: {}", last_check.format("%Y-%m-%d %H:%M:%S UTC"));
            }

            if status.is_healthy() {
                println!("   🟢 Service is healthy!");
            } else {
                println!("   🔴 Service is unhealthy!");
                if let Some(error) = &status.error_message {
                    println!("   Error: {}", error);
                }
            }
        }
        Err(e) => {
            println!("❌ Health check failed: {}", e);
        }
    }

    println!();
    Ok(())
}

/// Demonstrates script-based health check functionality
async fn demo_script_health_check() -> Result<()> {
    println!("📜 Demo 2: Script Health Check");
    println!("-------------------------------");

    // Create a script health check configuration
    // Use a simple command that should work on most systems
    let script_path = if cfg!(windows) { "echo" } else { "true" };

    let config = HealthCheckConfig::script(script_path)
        .timeout(Duration::from_secs(5))
        .interval(Duration::from_secs(15))
        .retries(2)
        .enabled(true);

    let mut health_check = HealthCheck::new(config);

    println!("🔍 Performing script health check: {}", script_path);

    match health_check.check().await {
        Ok(status) => {
            println!("✅ Health check completed!");
            println!("   State: {:?}", status.state);
            println!("   Total checks: {}", status.total_checks);

            if status.is_healthy() {
                println!("   🟢 Script executed successfully!");
            } else {
                println!("   🔴 Script failed!");
                if let Some(error) = &status.error_message {
                    println!("   Error: {}", error);
                }
            }
        }
        Err(e) => {
            println!("❌ Health check failed: {}", e);
        }
    }

    // Demo retry logic with a failing command
    println!("\n🔄 Testing retry logic with failing command...");

    let failing_script = if cfg!(windows) { "exit" } else { "false" };
    let failing_config = HealthCheckConfig::script(failing_script)
        .timeout(Duration::from_secs(2))
        .retries(3)
        .enabled(true);

    let mut failing_health_check = HealthCheck::new(failing_config);

    for attempt in 1..=3 {
        println!("   Attempt {}/3...", attempt);
        match failing_health_check.check().await {
            Ok(status) => {
                println!("   Consecutive failures: {}", status.consecutive_failures);
                if status.consecutive_failures >= 3 {
                    println!("   🔴 Health check marked as unhealthy after {} failures", status.consecutive_failures);
                    break;
                }
            }
            Err(e) => {
                println!("   Error: {}", e);
            }
        }
        sleep(Duration::from_millis(500)).await;
    }

    println!();
    Ok(())
}

/// Demonstrates integrating health checks with process configuration
async fn demo_process_with_health_check() -> Result<()> {
    println!("⚙️  Demo 3: Process with Health Check");
    println!("-------------------------------------");

    // Create a health check configuration
    let health_config = HealthCheckConfig::script("echo")
        .timeout(Duration::from_secs(5))
        .interval(Duration::from_secs(10))
        .retries(2)
        .enabled(true);

    // Create a process configuration with health check
    let process_config = ProcessConfig::builder()
        .name("demo-process")
        .script("echo")
        .args(vec!["Hello from PMDaemon!"])
        .health_check(health_config)
        .build()?;

    println!("📋 Process Configuration:");
    println!("   Name: {}", process_config.name);
    println!("   Script: {}", process_config.script);
    println!("   Args: {:?}", process_config.args);

    if let Some(health_check) = &process_config.health_check {
        println!("   Health Check: Enabled");
        println!("   Health Check Type: {:?}", health_check.check_type);
        println!("   Timeout: {:?}", health_check.timeout);
        println!("   Retries: {}", health_check.retries);
    } else {
        println!("   Health Check: Disabled");
    }

    println!("\n🏗️  This process configuration can be used with ProcessManager");
    println!("   to start a process with automatic health monitoring!");

    println!();
    Ok(())
}
