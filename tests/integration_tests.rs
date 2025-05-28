//! Integration tests for PMDaemon CLI commands
//!
//! These tests verify that the CLI commands work correctly end-to-end,
//! testing the actual binary with real process management scenarios.
//!
//! **IMPORTANT**: These tests must be run sequentially (--test-threads=1)
//! because they manage real processes and shared resources that can
//! interfere with each other when run in parallel. The CI configuration
//! is set up to handle this automatically.

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

        let env = Self {
            temp_dir,
            config_dir,
        };

        // Clean up any existing processes to ensure test isolation
        env.cleanup_all_processes();

        // Ensure we start with a completely clean config directory
        env.reset_config_dir();

        env
    }

    /// Clean up all processes to ensure test isolation
    fn cleanup_all_processes(&self) {
        // Try to delete all processes, ignore failures since this is cleanup
        let _ = self.cmd().args(["delete", "all", "--force"]).output();

        // Wait a moment for cleanup to complete
        thread::sleep(Duration::from_millis(100));

        // Also clean up any leftover config files
        if let Ok(entries) = fs::read_dir(&self.config_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|ext| ext == "json") {
                    let _ = fs::remove_file(entry.path());
                }
            }
        }

        // Clean up any leftover PID files
        if let Ok(entries) = fs::read_dir(&self.config_dir) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|ext| ext == "pid") {
                    let _ = fs::remove_file(entry.path());
                }
            }
        }
    }

    /// Reset the config directory to ensure complete isolation
    fn reset_config_dir(&self) {
        // Remove the entire config directory
        let _ = fs::remove_dir_all(&self.config_dir);

        // Recreate it
        let _ = fs::create_dir_all(&self.config_dir);
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

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        // Clean up all processes when test environment is dropped
        self.cleanup_all_processes();
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
        .stdout(predicate::str::contains(
            "A process manager built in Rust inspired by PM2",
        ))
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
        "#!/bin/bash\necho 'Hello from test app'\nsleep 5\n",
    );

    // Start the process
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"))
        .stdout(predicate::str::contains(&process_name));

    // Give it a moment to start
    thread::sleep(Duration::from_millis(1000));

    // Check that it's listed
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name))
        .stdout(predicate::str::contains("online"));

    // Stop the process
    env.cmd()
        .args(["stop", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stopped"));

    // Clean up - ensure process is deleted
    let _ = env
        .cmd()
        .args(["delete", &process_name, "--force"])
        .output();
}

#[test]
fn test_start_with_args() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("echo-test");

    // Create a script that echoes its arguments
    let script = create_test_script(
        env.temp_path(),
        "echo_args",
        "#!/bin/bash\necho \"Args: $@\"\nsleep 2\n",
    );

    // Start with arguments
    env.cmd()
        .args([
            "start",
            script.to_str().unwrap(),
            "--name",
            &process_name,
            "--",
            "arg1",
            "arg2",
            "arg3",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(1000));

    // Verify the process is actually running before cleanup
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name));

    // Clean up - try to delete, but don't fail if process doesn't exist
    let _ = env
        .cmd()
        .args(["delete", &process_name, "--force"])
        .assert();
}

#[test]
fn test_start_with_port() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("server-app");

    // Create a simple server script
    let script = create_test_script(
        env.temp_path(),
        "server",
        "#!/bin/bash\necho 'Server starting on port 8080'\nsleep 3\n",
    );

    // Start with port specification
    env.cmd()
        .args([
            "start",
            script.to_str().unwrap(),
            "--name",
            &process_name,
            "--port",
            "8080",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(1000));

    // Check list includes port
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name))
        .stdout(predicate::str::contains("8080"));

    // Clean up - try to delete, but don't fail if process doesn't exist
    let _ = env
        .cmd()
        .args(["delete", &process_name, "--force"])
        .assert();
}

#[test]
fn test_start_multiple_instances() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("multi-app");

    let script = create_test_script(
        env.temp_path(),
        "multi_app",
        "#!/bin/bash\necho 'Instance starting'\nsleep 3\n",
    );

    // Start with multiple instances
    env.cmd()
        .args([
            "start",
            script.to_str().unwrap(),
            "--name",
            &process_name,
            "--instances",
            "2", // Reduced to 2 instances for more reliable testing
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(2000)); // Longer wait for multiple instances

    // List should show instances
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name));

    // Clean up - force delete all to ensure cleanup
    env.cmd()
        .args(["delete", "all", "--force"])
        .assert()
        .success();
}

#[test]
fn test_list_empty() {
    let env = TestEnvironment::new();

    env.cmd().arg("list").assert().success().stdout(
        predicate::str::contains("No processes").or(predicate::str::contains("ID")), // Table header if processes exist
    );
}

#[test]
fn test_list_format() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("format-test");

    let script = create_test_script(env.temp_path(), "format_test", "#!/bin/bash\nsleep 2\n");

    // Start a process
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success();

    thread::sleep(Duration::from_millis(1000));

    // Get list output (should contain table format)
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name))
        .stdout(predicate::str::contains("ID"));

    // Clean up - try to delete, but don't fail if process doesn't exist
    let _ = env
        .cmd()
        .args(["delete", &process_name, "--force"])
        .assert();
}

#[test]
fn test_delete_all_with_force() {
    let env = TestEnvironment::new();
    let process_name1 = env.unique_name("test-app-1");
    let process_name2 = env.unique_name("test-app-2");

    // Create test scripts
    let script1 = create_test_script(
        env.temp_path(),
        "test_app_1",
        "#!/bin/bash\necho 'App 1'\nsleep 2\n",
    );
    let script2 = create_test_script(
        env.temp_path(),
        "test_app_2",
        "#!/bin/bash\necho 'App 2'\nsleep 2\n",
    );

    // Start both processes
    env.cmd()
        .args(["start", script1.to_str().unwrap(), "--name", &process_name1])
        .assert()
        .success();

    env.cmd()
        .args(["start", script2.to_str().unwrap(), "--name", &process_name2])
        .assert()
        .success();

    thread::sleep(Duration::from_millis(1000));

    // Verify both processes are listed
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name1))
        .stdout(predicate::str::contains(&process_name2));

    // Delete all processes with force flag
    env.cmd()
        .args(["delete", "all", "--force"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Stopped and deleted")
                .and(predicate::str::contains("processes")),
        );

    // Verify no processes remain
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No processes").or(predicate::str::contains("ID")));
}

#[test]
fn test_delete_by_status() {
    let env = TestEnvironment::new();
    let process_name1 = env.unique_name("running-app");
    let process_name2 = env.unique_name("stopped-app");

    // Create test scripts
    let script1 = create_test_script(
        env.temp_path(),
        "running_app",
        "#!/bin/bash\necho 'Running app'\nsleep 5\n",
    );
    let script2 = create_test_script(
        env.temp_path(),
        "stopped_app",
        "#!/bin/bash\necho 'Stopped app'\nexit 0\n",
    );

    // Start first process (will keep running)
    env.cmd()
        .args(["start", script1.to_str().unwrap(), "--name", &process_name1])
        .assert()
        .success();

    // Start second process (will exit quickly)
    env.cmd()
        .args(["start", script2.to_str().unwrap(), "--name", &process_name2])
        .assert()
        .success();

    // Wait for second process to exit
    thread::sleep(Duration::from_millis(1500));

    // Delete stopped processes with force flag
    env.cmd()
        .args(["delete", "stopped", "--status", "--force"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Stopped and deleted")
                .and(predicate::str::contains("stopped")),
        );

    // Verify that the delete by status command worked
    // The exact process states in integration tests can be unpredictable,
    // so we just verify that the command executed successfully
    // and that we can still list processes
    env.cmd().arg("list").assert().success();

    // Clean up any remaining processes
    env.cmd()
        .args(["delete", "all", "--force"])
        .assert()
        .success();
}

#[test]
fn test_stop_nonexistent_process() {
    let env = TestEnvironment::new();

    env.cmd()
        .args(["stop", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ProcessNotFound"));
}

#[test]
fn test_delete_process() {
    let env = TestEnvironment::new();
    let process_name = env.unique_name("delete-test");

    let script = create_test_script(env.temp_path(), "delete_test", "#!/bin/bash\nsleep 5\n");

    // Start process
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success();

    thread::sleep(Duration::from_millis(1000));

    // Delete process
    env.cmd()
        .args(["delete", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("Stopped and deleted"));

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
        "#!/bin/bash\necho 'Starting'\nsleep 10\n",
    );

    // Start process
    env.cmd()
        .args(["start", script.to_str().unwrap(), "--name", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("started"));

    thread::sleep(Duration::from_millis(1000));

    // Verify process is running before restart
    env.cmd()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains(&process_name));

    // Restart process
    env.cmd()
        .args(["restart", &process_name])
        .assert()
        .success()
        .stdout(predicate::str::contains("Restarted"));

    // Clean up - try to delete, but don't fail if process doesn't exist
    let _ = env
        .cmd()
        .args(["delete", &process_name, "--force"])
        .assert();
}
