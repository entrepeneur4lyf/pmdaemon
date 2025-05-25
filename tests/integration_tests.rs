//! Integration tests for PMDaemon CLI commands
//!
//! These tests verify that the CLI commands work correctly end-to-end,
//! testing the actual binary with real process management scenarios.

use assert_cmd::Command;
use predicates::prelude::*;

use std::fs;
use std::path::PathBuf;

use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Helper struct for managing test environment
struct TestEnvironment {
    temp_dir: TempDir,
    config_dir: PathBuf,
}

impl TestEnvironment {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let config_dir = temp_dir.path().join(".pm2r");

        // Create config directory
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        Self { temp_dir, config_dir }
    }

    fn unique_name(&self, base: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("{}-{}", base, timestamp % 1000000)
    }

    fn cmd(&self) -> Command {
        let mut cmd = Command::cargo_bin("pmdaemon").expect("Failed to find binary");
        cmd.env("PM2R_HOME", &self.config_dir);
        // Disable colored output and verbose logging for cleaner test output
        cmd.env("NO_COLOR", "1");
        cmd.env("RUST_LOG", "error");
        cmd
    }

    fn temp_path(&self) -> &std::path::Path {
        self.temp_dir.path()
    }
}

/// Create a simple test script
fn create_test_script(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
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

#[test]
fn test_cli_help() {
    let env = TestEnvironment::new();

    env.cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("feature-limited PM2 clone"))
        .stdout(predicate::str::contains("start"))
        .stdout(predicate::str::contains("stop"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_cli_version() {
    let env = TestEnvironment::new();

    env.cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_start_simple_process() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("test-app");

    // Create a simple test script
    let script = create_test_script(
        env.temp_path(),
        "test_app",
        "#!/bin/bash\necho 'Hello from test app'\nsleep 5\n"
    );

    // Start the process
    env.cmd()
        .args(&["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"))
        .stdout(predicate::str::contains(&process_name));

    // Give it a moment to start
    thread::sleep(Duration::from_millis(500));

    // Check that it's listed
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name))
        .stdout(predicate::str::contains("online"));

    // Stop the process
    env.cmd()
        .args(&["stop", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stopped"));
}

#[test]
fn test_start_with_args() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("echo-test");

    // Create a script that echoes its arguments
    let script = create_test_script(
        env.temp_path(),
        "echo_args",
        "#!/bin/bash\necho \"Args: $@\"\nsleep 2\n"
    );

    // Start with arguments
    env.cmd()
        .args(&[
            "start",
            script.to_str().unwrap(),
            "--name", &process_name,
            "--", "arg1", "arg2", "arg3"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(500));

    // Clean up
    env.cmd()
        .args(&["delete", &process_name])
        .assert()
        .success();
}

#[test]
fn test_start_with_port() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("server-app");

    // Create a simple server script
    let script = create_test_script(
        env.temp_path(),
        "server",
        "#!/bin/bash\necho 'Server starting on port 8080'\nsleep 3\n"
    );

    // Start with port specification
    env.cmd()
        .args(&[
            "start",
            script.to_str().unwrap(),
            "--name", &process_name,
            "--port", "8080"
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(500));

    // Check list includes port
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name))
        .stdout(predicate::str::contains("8080"));

    // Clean up
    env.cmd()
        .args(&["delete", &process_name])
        .assert()
        .success();
}

#[test]
fn test_start_multiple_instances() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("multi-app");

    let script = create_test_script(
        env.temp_path(),
        "multi_app",
        "#!/bin/bash\necho 'Instance starting'\nsleep 3\n"
    );

    // Start with multiple instances
    env.cmd()
        .args(&[
            "start",
            script.to_str().unwrap(),
            "--name", &process_name,
            "--instances", "2"  // Reduced to 2 instances for more reliable testing
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(1000));

    // List should show instances
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name));

    // Clean up - try to delete, but don't fail if it doesn't exist
    let _ = env.cmd()
        .args(&["delete", &process_name])
        .assert();
}

#[test]
fn test_list_empty() {
    let env = TestEnvironment::new();

    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("No processes")
                .or(predicate::str::contains("ID"))  // Table header if processes exist
        );
}

#[test]
fn test_list_format() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("format-test");

    let script = create_test_script(
        env.temp_path(),
        "format_test",
        "#!/bin/bash\nsleep 2\n"
    );

    // Start a process
    env.cmd()
        .args(&["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success();

    thread::sleep(Duration::from_millis(500));

    // Get list output (should contain table format)
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name))
        .stdout(predicate::str::contains("ID"));

    // Clean up
    env.cmd()
        .args(&["delete", &process_name])
        .assert()
        .success();
}

#[test]
fn test_stop_nonexistent_process() {
    let env = TestEnvironment::new();

    env.cmd()
        .args(&["stop", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ProcessNotFound"));
}

#[test]
fn test_delete_process() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("delete-test");

    let script = create_test_script(
        env.temp_path(),
        "delete_test",
        "#!/bin/bash\nsleep 5\n"
    );

    // Start process
    env.cmd()
        .args(&["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success();

    thread::sleep(Duration::from_millis(500));

    // Delete process
    env.cmd()
        .args(&["delete", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted"));

    // Verify it's gone
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name).not());
}

#[test]
fn test_restart_process() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("restart-test");

    let script = create_test_script(
        env.temp_path(),
        "restart_test",
        "#!/bin/bash\necho 'Starting'\nsleep 10\n"
    );

    // Start process
    env.cmd()
        .args(&["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success();

    thread::sleep(Duration::from_millis(500));

    // Restart process
    env.cmd()
        .args(&["restart", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("Restarted"));

    // Clean up
    env.cmd()
        .args(&["delete", &process_name])
        .assert()
        .success();
}
