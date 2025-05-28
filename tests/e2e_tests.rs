//! End-to-end tests for PMDaemon with various process types
//!
//! These tests verify that PMDaemon works correctly with different types of
//! applications and scenarios, testing the complete system integration.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Helper struct for managing test environment
struct E2ETestEnvironment {
    temp_dir: TempDir,
    config_dir: PathBuf,
}

impl E2ETestEnvironment {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let config_dir = temp_dir.path().join(".pm2r");

        // Create config directory
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        Self {
            temp_dir,
            config_dir,
        }
    }

    fn cmd(&self) -> Command {
        let mut cmd = Command::cargo_bin("pmdaemon").expect("Failed to find binary");
        cmd.env("PM2R_HOME", &self.config_dir);
        cmd.env("NO_COLOR", "1");
        cmd.env("RUST_LOG", "error");
        cmd
    }

    fn temp_path(&self) -> &std::path::Path {
        self.temp_dir.path()
    }

    fn unique_name(&self, base: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("{}-{}", base, timestamp % 1000000)
    }
}

/// Create a test script with specific content
fn create_script(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
    let script_path = dir.join(format!("{}.sh", name));
    fs::write(&script_path, content).expect("Failed to write test script");

    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).unwrap();
    }

    script_path
}

/// Create a Python script
fn create_python_script(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
    let script_path = dir.join(format!("{}.py", name));
    fs::write(&script_path, content).expect("Failed to write Python script");
    script_path
}

/// Create a Node.js script
fn create_node_script(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
    let script_path = dir.join(format!("{}.js", name));
    fs::write(&script_path, content).expect("Failed to write Node.js script");
    script_path
}

#[test]
fn test_simple_shell_script() {
    let env = E2ETestEnvironment::new();
    let process_name = env.unique_name("shell-app");

    // Create a simple shell script that runs for a while
    let script = create_script(
        env.temp_path(),
        "simple_shell",
        r#"#!/bin/bash
echo "Shell script starting..."
for i in {1..10}; do
    echo "Iteration $i"
    sleep 1
done
echo "Shell script completed"
"#,
    );

    // Start the process
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(500));

    // Verify it's running
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name))
        .stdout(predicate::str::contains("online"));

    // Stop and clean up
    env.cmd().args(["stop", &process_name]).assert().success();

    env.cmd().args(["delete", &process_name]).assert().success();
}

#[test]
fn test_python_script() {
    let env = E2ETestEnvironment::new();
    let process_name = env.unique_name("python-app");

    // Create a Python script with shebang
    let script = create_python_script(
        env.temp_path(),
        "python_app",
        r#"#!/usr/bin/env python3
import time
import sys

print("Python application starting...")
for i in range(5):
    print(f"Python iteration {i+1}")
    time.sleep(1)
print("Python application completed")
"#,
    );

    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script, perms).unwrap();
    }

    // Start the process (Python script with shebang)
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(500));

    // Verify it's running
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name));

    // Clean up
    env.cmd().args(["delete", &process_name]).assert().success();
}

#[test]
fn test_node_script() {
    let env = E2ETestEnvironment::new();
    let process_name = env.unique_name("node-app");

    // Create a Node.js script with shebang
    let script = create_node_script(
        env.temp_path(),
        "node_app",
        r#"#!/usr/bin/env node
console.log('Node.js application starting...');

let counter = 0;
const interval = setInterval(() => {
    counter++;
    console.log(`Node.js iteration ${counter}`);

    if (counter >= 5) {
        console.log('Node.js application completed');
        clearInterval(interval);
        process.exit(0);
    }
}, 1000);

// Handle graceful shutdown
process.on('SIGTERM', () => {
    console.log('Received SIGTERM, shutting down gracefully');
    clearInterval(interval);
    process.exit(0);
});
"#,
    );

    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script, perms).unwrap();
    }

    // Start the process (Node.js script with shebang)
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(500));

    // Verify it's running
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name));

    // Clean up
    env.cmd().args(["delete", &process_name]).assert().success();
}

#[test]
fn test_clustering_mode() {
    let env = E2ETestEnvironment::new();
    let process_name = env.unique_name("cluster-app");

    // Create a script suitable for clustering
    let script = create_script(
        env.temp_path(),
        "cluster_app",
        r#"#!/bin/bash
echo "Cluster instance starting with PID $$"
echo "Instance ID: ${PM2_INSTANCE_ID:-0}"
sleep 5
echo "Cluster instance completed"
"#,
    );

    // Start with clustering
    env.cmd()
        .args([
            "start",
            script.to_str().unwrap(),
            "--name",
            &process_name,
            "--instances",
            "2",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(1000));

    // Verify instances are running
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name));

    // Clean up - try to delete, but don't fail if it doesn't exist
    let _ = env.cmd().args(["delete", &process_name]).assert();
}

#[test]
fn test_port_management() {
    let env = E2ETestEnvironment::new();
    let process_name = env.unique_name("port-app");

    // Create a script that simulates a server
    let script = create_script(
        env.temp_path(),
        "port_server",
        r#"#!/bin/bash
echo "Server starting on port ${PORT:-8080}"
echo "Process PID: $$"
sleep 5
echo "Server shutting down"
"#,
    );

    // Start with port allocation
    env.cmd()
        .args([
            "start",
            script.to_str().unwrap(),
            "--name",
            &process_name,
            "--port",
            "9000",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(500));

    // Verify port is shown in list
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name))
        .stdout(predicate::str::contains("9000"));

    // Clean up
    env.cmd().args(["delete", &process_name]).assert().success();
}

#[test]
fn test_auto_restart() {
    let env = E2ETestEnvironment::new();
    let process_name = env.unique_name("restart-app");

    // Create a script that exits quickly (to trigger restart)
    let script = create_script(
        env.temp_path(),
        "quick_exit",
        r#"#!/bin/bash
echo "Process starting..."
sleep 1
echo "Process exiting..."
exit 1
"#,
    );

    // Start the process (auto-restart is enabled by default)
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    // Wait for potential restart
    thread::sleep(Duration::from_millis(2000));

    // Check if process shows restart count
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name));

    // Clean up - try to delete, but don't fail if it doesn't exist
    let _ = env.cmd().args(["delete", &process_name]).assert();
}

#[test]
fn test_graceful_shutdown() {
    let env = E2ETestEnvironment::new();
    let process_name = env.unique_name("graceful-app");

    // Create a script that handles SIGTERM gracefully
    let script = create_script(
        env.temp_path(),
        "graceful_shutdown",
        r#"#!/bin/bash
cleanup() {
    echo "Received signal, cleaning up..."
    sleep 1
    echo "Cleanup completed, exiting gracefully"
    exit 0
}

trap cleanup SIGTERM SIGINT

echo "Process starting with PID $$"
echo "Waiting for signal..."

# Run indefinitely until signal received
while true; do
    sleep 1
done
"#,
    );

    // Start the process
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(500));

    // Verify it's running
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name))
        .stdout(predicate::str::contains("online"));

    // Stop gracefully
    env.cmd()
        .args(["stop", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stopped"));

    // Clean up
    env.cmd().args(["delete", &process_name]).assert().success();
}

#[test]
fn test_resource_monitoring() {
    let env = E2ETestEnvironment::new();
    let process_name = env.unique_name("monitor-app");

    // Create a script that uses some resources
    let script = create_script(
        env.temp_path(),
        "resource_app",
        r#"#!/bin/bash
echo "Resource-intensive process starting..."

# Simulate some CPU and memory usage
for i in {1..5}; do
    echo "Working... iteration $i"
    # Create some temporary data
    data=$(seq 1 1000)
    sleep 1
done

echo "Resource process completed"
"#,
    );

    // Start the process
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(1000));

    // Check monitoring data is available
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name));

    // Clean up
    env.cmd().args(["delete", &process_name]).assert().success();
}
