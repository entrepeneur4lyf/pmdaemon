//! Process management types and utilities

use crate::config::ProcessConfig;
use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::fs::File;
use tokio::process::{Child, Command};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Unique identifier for a process
pub type ProcessId = Uuid;

/// Current state of a process
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessState {
    /// Process is starting up
    Starting,
    /// Process is running normally
    Online,
    /// Process is stopping
    Stopping,
    /// Process has stopped
    Stopped,
    /// Process has errored
    Errored,
    /// Process is being restarted
    Restarting,
}

impl std::fmt::Display for ProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessState::Starting => write!(f, "starting"),
            ProcessState::Online => write!(f, "online"),
            ProcessState::Stopping => write!(f, "stopping"),
            ProcessState::Stopped => write!(f, "stopped"),
            ProcessState::Errored => write!(f, "errored"),
            ProcessState::Restarting => write!(f, "restarting"),
        }
    }
}

/// Process status information for external consumption.
///
/// This struct represents the current status and metrics of a managed process.
/// It's designed to be serializable for API responses and contains all the
/// information needed for monitoring and display purposes.
///
/// # Examples
///
/// ```rust
/// use pmdaemon::process::{ProcessStatus, ProcessState};
/// use uuid::Uuid;
///
/// // ProcessStatus is typically obtained from Process::status()
/// // This example shows the structure for documentation purposes
/// let status = ProcessStatus {
///     id: Uuid::new_v4(),
///     name: "my-app".to_string(),
///     state: ProcessState::Online,
///     pid: Some(1234),
///     uptime: None,
///     restarts: 0,
///     cpu_usage: 15.5,
///     memory_usage: 128 * 1024 * 1024, // 128MB
///     exit_code: None,
///     error: None,
///     namespace: "default".to_string(),
///     instance: None,
///     assigned_port: Some(3000),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStatus {
    /// Unique process identifier (UUID)
    pub id: ProcessId,

    /// Human-readable process name
    pub name: String,

    /// Current process state (starting, online, stopped, etc.)
    pub state: ProcessState,

    /// System process ID (PID) if the process is currently running
    pub pid: Option<u32>,

    /// Process start time (UTC) for uptime calculation
    pub uptime: Option<DateTime<Utc>>,

    /// Total number of times this process has been restarted
    pub restarts: u32,

    /// Current CPU usage as a percentage (0.0-100.0)
    pub cpu_usage: f32,

    /// Current memory usage in bytes
    pub memory_usage: u64,

    /// Exit code from the last process termination
    pub exit_code: Option<i32>,

    /// Error message if the process is in an error state
    pub error: Option<String>,

    /// Namespace for logical process grouping
    pub namespace: String,

    /// Instance number for cluster mode (0-based)
    pub instance: Option<u32>,

    /// Port assigned to this process (if any)
    pub assigned_port: Option<u16>,
}

/// Internal process representation for lifecycle management.
///
/// This struct represents a managed process with full lifecycle control capabilities.
/// It contains both the configuration and runtime state, including the actual system
/// process handle for direct control operations.
///
/// # Examples
///
/// ```rust,no_run
/// use pmdaemon::{Process, ProcessConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ProcessConfig::builder()
///     .name("my-app")
///     .script("node")
///     .args(vec!["server.js"])
///     .build()?;
///
/// let mut process = Process::new(config);
///
/// // Start the process
/// process.start().await?;
///
/// // Check if it's running
/// if process.is_running() {
///     println!("Process is running with PID: {:?}", process.pid());
/// }
///
/// // Get status for monitoring
/// let status = process.status();
/// println!("Process {} is {}", status.name, status.state);
///
/// // Stop the process
/// process.stop().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Process {
    /// Unique identifier for this process instance
    pub id: ProcessId,

    /// Process configuration defining how it should run
    pub config: ProcessConfig,

    /// Current process state (starting, online, stopped, etc.)
    pub state: ProcessState,

    /// System process handle for direct control (if running)
    pub child: Option<Child>,

    /// Timestamp when the process was last started
    pub started_at: Option<DateTime<Utc>>,

    /// Total number of times this process has been restarted
    pub restarts: u32,

    /// Exit code from the last process termination
    pub exit_code: Option<i32>,

    /// Error message if the process is in an error state
    pub error: Option<String>,

    /// Instance number for cluster mode (0-based)
    pub instance: Option<u32>,

    /// Port assigned to this process by the port manager
    pub assigned_port: Option<u16>,

    /// Real-time monitoring data (CPU, memory, etc.)
    pub monitoring: ProcessMonitoring,
}

/// Real-time monitoring data for a process.
///
/// This struct contains the current resource usage metrics for a process,
/// updated periodically by the monitoring system. It's used internally
/// for tracking performance and enforcing resource limits.
///
/// # Examples
///
/// ```rust
/// use pmdaemon::process::ProcessMonitoring;
/// use chrono::Utc;
///
/// let mut monitoring = ProcessMonitoring::default();
///
/// // Update with current metrics
/// monitoring.cpu_usage = 25.5;
/// monitoring.memory_usage = 128 * 1024 * 1024; // 128MB
/// monitoring.last_update = Some(Utc::now());
/// ```
#[derive(Debug, Default)]
pub struct ProcessMonitoring {
    /// Current CPU usage as a percentage (0.0-100.0)
    pub cpu_usage: f32,

    /// Current memory usage in bytes
    pub memory_usage: u64,

    /// Timestamp of the last monitoring update
    pub last_update: Option<DateTime<Utc>>,
}

impl Process {
    /// Create a new process instance from configuration.
    ///
    /// Creates a new process in the `Stopped` state with a unique ID.
    /// The process is not started automatically - call `start()` to begin execution.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::{Process, ProcessConfig};
    ///
    /// let config = ProcessConfig::builder()
    ///     .name("my-app")
    ///     .script("node")
    ///     .args(vec!["server.js"])
    ///     .build()
    ///     .unwrap();
    ///
    /// let process = Process::new(config);
    /// assert_eq!(process.state, pmdaemon::ProcessState::Stopped);
    /// ```
    pub fn new(config: ProcessConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            config,
            state: ProcessState::Stopped,
            child: None,
            started_at: None,
            restarts: 0,
            exit_code: None,
            error: None,
            instance: None,
            assigned_port: None,
            monitoring: ProcessMonitoring::default(),
        }
    }

    /// Get current process status for monitoring and API responses.
    ///
    /// Returns a `ProcessStatus` struct containing all current process information
    /// including state, resource usage, and metadata. This is the primary method
    /// for obtaining process information for external consumption.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::{Process, ProcessConfig, ProcessState};
    ///
    /// let config = ProcessConfig::builder()
    ///     .name("my-app")
    ///     .script("echo")
    ///     .build()
    ///     .unwrap();
    ///
    /// let process = Process::new(config);
    /// let status = process.status();
    ///
    /// assert_eq!(status.name, "my-app");
    /// assert_eq!(status.state, ProcessState::Stopped);
    /// assert_eq!(status.restarts, 0);
    /// ```
    pub fn status(&self) -> ProcessStatus {
        ProcessStatus {
            id: self.id,
            name: self.config.name.clone(),
            state: self.state,
            pid: self.child.as_ref().and_then(|c| c.id()),
            uptime: self.started_at,
            restarts: self.restarts,
            cpu_usage: self.monitoring.cpu_usage,
            memory_usage: self.monitoring.memory_usage,
            exit_code: self.exit_code,
            error: self.error.clone(),
            namespace: self.config.namespace.clone(),
            instance: self.instance,
            assigned_port: self.assigned_port,
        }
    }

    /// Check if process is running
    pub fn is_running(&self) -> bool {
        matches!(self.state, ProcessState::Online | ProcessState::Starting)
    }

    /// Set process state
    pub fn set_state(&mut self, state: ProcessState) {
        self.state = state;
        if state == ProcessState::Online && self.started_at.is_none() {
            self.started_at = Some(Utc::now());
        }
    }

    /// Start the process
    pub async fn start(&mut self) -> Result<()> {
        self.start_with_logs(None, None).await
    }

    /// Start the process with optional log file redirection.
    ///
    /// This is the core method for starting a process. It spawns the configured
    /// command with all specified arguments, environment variables, and working
    /// directory. Output can be redirected to log files or captured for processing.
    ///
    /// # Arguments
    ///
    /// * `out_log` - Optional path for stdout redirection
    /// * `err_log` - Optional path for stderr redirection
    ///
    /// # Process Lifecycle
    ///
    /// 1. Validates the process is not already running
    /// 2. Sets state to `Starting`
    /// 3. Configures the command with args, environment, and working directory
    /// 4. Sets up stdio redirection (files or pipes)
    /// 5. Spawns the process
    /// 6. Sets state to `Online` on success or `Errored` on failure
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::{Process, ProcessConfig};
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ProcessConfig::builder()
    ///     .name("my-app")
    ///     .script("node")
    ///     .args(vec!["server.js"])
    ///     .build()?;
    ///
    /// let mut process = Process::new(config);
    ///
    /// // Start with log redirection
    /// let out_log = Some(PathBuf::from("/var/log/myapp-out.log"));
    /// let err_log = Some(PathBuf::from("/var/log/myapp-err.log"));
    /// process.start_with_logs(out_log, err_log).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Process is already running
    /// - Command/script is not found
    /// - Log files cannot be created
    /// - Process spawn fails for any reason
    pub async fn start_with_logs(
        &mut self,
        out_log: Option<PathBuf>,
        err_log: Option<PathBuf>,
    ) -> Result<()> {
        if self.is_running() {
            return Err(Error::ProcessAlreadyRunning(self.config.name.clone()));
        }

        info!("Starting process: {}", self.config.name);
        self.set_state(ProcessState::Starting);

        // Prepare command
        let mut cmd = Command::new(&self.config.script);

        // Add arguments
        if !self.config.args.is_empty() {
            cmd.args(&self.config.args);
        }

        // Set working directory
        if let Some(cwd) = &self.config.cwd {
            cmd.current_dir(cwd);
        }

        // Set environment variables
        if !self.config.env.is_empty() {
            for (key, value) in &self.config.env {
                cmd.env(key, value);
            }
        }

        // Configure stdio with log file redirection
        if let Some(out_path) = out_log {
            // Create/open stdout log file
            let stdout_file = File::create(&out_path)
                .await
                .map_err(|e| Error::config(format!("Failed to create stdout log file: {}", e)))?;
            cmd.stdout(stdout_file.into_std().await);
            debug!("Redirecting stdout to: {:?}", out_path);
        } else {
            cmd.stdout(Stdio::piped());
        }

        if let Some(err_path) = err_log {
            // Create/open stderr log file
            let stderr_file = File::create(&err_path)
                .await
                .map_err(|e| Error::config(format!("Failed to create stderr log file: {}", e)))?;
            cmd.stderr(stderr_file.into_std().await);
            debug!("Redirecting stderr to: {:?}", err_path);
        } else {
            cmd.stderr(Stdio::piped());
        }

        cmd.stdin(Stdio::null());

        // Configure process to run independently (detached from parent)
        #[cfg(unix)]
        {
            #[allow(unused_imports)]
            use std::os::unix::process::CommandExt;
            cmd.process_group(0); // Create new process group to detach from parent
        }

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x00000008); // CREATE_NO_WINDOW flag to detach
        }

        // Spawn the process
        match cmd.spawn() {
            Ok(child) => {
                info!(
                    "Process {} started with PID: {}",
                    self.config.name,
                    child.id().unwrap_or(0)
                );
                // Store the child process handle
                self.child = Some(child);
                self.set_state(ProcessState::Online);
                self.error = None;

                // Note: Process is now detached and will continue running independently
                debug!(
                    "Process {} detached and running independently",
                    self.config.name
                );
                Ok(())
            }
            Err(e) => {
                error!("Failed to start process {}: {}", self.config.name, e);
                self.set_state(ProcessState::Errored);
                self.error = Some(format!("Failed to start: {}", e));
                Err(Error::ProcessStartFailed {
                    name: self.config.name.clone(),
                    reason: e.to_string(),
                })
            }
        }
    }

    /// Stop the process gracefully
    pub async fn stop(&mut self) -> Result<()> {
        if !self.is_running() {
            return Ok(());
        }

        info!("Stopping process: {}", self.config.name);
        self.set_state(ProcessState::Stopping);

        if let Some(mut child) = self.child.take() {
            // Try graceful shutdown first
            #[cfg(unix)]
            {
                use nix::sys::signal::{self, Signal};
                use nix::unistd::Pid;

                if let Some(pid) = child.id() {
                    let pid = Pid::from_raw(pid as i32);
                    if let Err(e) = signal::kill(pid, Signal::SIGTERM) {
                        warn!(
                            "Failed to send SIGTERM to process {}: {}",
                            self.config.name, e
                        );
                    } else {
                        debug!("Sent SIGTERM to process {}", self.config.name);
                    }
                }
            }

            // Wait for graceful shutdown with timeout
            let timeout = tokio::time::Duration::from_secs(10);
            match tokio::time::timeout(timeout, child.wait()).await {
                Ok(Ok(exit_status)) => {
                    info!(
                        "Process {} stopped gracefully with exit code: {:?}",
                        self.config.name,
                        exit_status.code()
                    );
                    self.exit_code = exit_status.code();
                }
                Ok(Err(e)) => {
                    error!(
                        "Error waiting for process {} to stop: {}",
                        self.config.name, e
                    );
                    return Err(Error::ProcessStopFailed {
                        name: self.config.name.clone(),
                        reason: e.to_string(),
                    });
                }
                Err(_) => {
                    warn!(
                        "Process {} did not stop gracefully, killing forcefully",
                        self.config.name
                    );
                    if let Err(e) = child.kill().await {
                        error!("Failed to kill process {}: {}", self.config.name, e);
                        return Err(Error::ProcessStopFailed {
                            name: self.config.name.clone(),
                            reason: e.to_string(),
                        });
                    }
                    if let Ok(exit_status) = child.wait().await {
                        self.exit_code = exit_status.code();
                    }
                }
            }
        }

        self.set_state(ProcessState::Stopped);
        self.started_at = None;
        Ok(())
    }

    /// Restart the process
    pub async fn restart(&mut self) -> Result<()> {
        info!("Restarting process: {}", self.config.name);
        self.set_state(ProcessState::Restarting);

        // Stop if running
        if self.is_running() {
            self.stop().await?;
        }

        // Increment restart counter
        self.restarts += 1;

        // Start again
        self.start().await
    }

    /// Check if the process is still alive
    pub async fn check_status(&mut self) -> Result<bool> {
        if let Some(child) = &mut self.child {
            match child.try_wait() {
                Ok(Some(exit_status)) => {
                    // Process has exited
                    info!(
                        "Process {} exited with status: {:?}",
                        self.config.name,
                        exit_status.code()
                    );
                    self.exit_code = exit_status.code();
                    self.set_state(ProcessState::Stopped);
                    self.child = None;
                    self.started_at = None;
                    Ok(false)
                }
                Ok(None) => {
                    // Process is still running
                    Ok(true)
                }
                Err(e) => {
                    error!("Error checking process {} status: {}", self.config.name, e);
                    self.set_state(ProcessState::Errored);
                    self.error = Some(format!("Status check failed: {}", e));
                    self.child = None;
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }

    /// Get the process PID if running
    pub fn pid(&self) -> Option<u32> {
        self.child.as_ref().and_then(|c| c.id())
    }

    /// Set the assigned port for this process
    pub fn set_assigned_port(&mut self, port: Option<u16>) {
        self.assigned_port = port;
    }

    /// Set the instance number for this process
    pub fn set_instance(&mut self, instance: Option<u32>) {
        self.instance = instance;
    }

    /// Update monitoring data
    pub fn update_monitoring(&mut self, cpu_usage: f32, memory_usage: u64) {
        self.monitoring.cpu_usage = cpu_usage;
        self.monitoring.memory_usage = memory_usage;
        self.monitoring.last_update = Some(Utc::now());
    }

    /// Get uptime in seconds
    pub fn uptime_seconds(&self) -> Option<i64> {
        self.started_at
            .map(|start| (Utc::now() - start).num_seconds())
    }

    /// Check if process should be auto-restarted
    pub fn should_auto_restart(&self) -> bool {
        self.config.autorestart
            && self.state == ProcessState::Stopped
            && self.config.max_restarts > self.restarts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ProcessConfig;
    use pretty_assertions::assert_eq;

    fn create_test_config() -> ProcessConfig {
        ProcessConfig::builder()
            .name("test-process")
            .script("echo")
            .args(vec!["hello", "world"])
            .build()
            .unwrap()
    }

    #[test]
    fn test_process_state_display() {
        assert_eq!(ProcessState::Starting.to_string(), "starting");
        assert_eq!(ProcessState::Online.to_string(), "online");
        assert_eq!(ProcessState::Stopping.to_string(), "stopping");
        assert_eq!(ProcessState::Stopped.to_string(), "stopped");
        assert_eq!(ProcessState::Errored.to_string(), "errored");
        assert_eq!(ProcessState::Restarting.to_string(), "restarting");
    }

    #[test]
    fn test_process_state_serialization() {
        let state = ProcessState::Online;
        let serialized = serde_json::to_string(&state).unwrap();
        assert_eq!(serialized, "\"online\"");

        let deserialized: ProcessState = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ProcessState::Online);
    }

    #[test]
    fn test_process_monitoring_default() {
        let monitoring = ProcessMonitoring::default();
        assert_eq!(monitoring.cpu_usage, 0.0);
        assert_eq!(monitoring.memory_usage, 0);
        assert!(monitoring.last_update.is_none());
    }

    #[test]
    fn test_process_new() {
        let config = create_test_config();
        let process = Process::new(config.clone());

        assert_eq!(process.config.name, "test-process");
        assert_eq!(process.config.script, "echo");
        assert_eq!(process.config.args, vec!["hello", "world"]);
        assert_eq!(process.state, ProcessState::Stopped);
        assert!(process.child.is_none());
        assert!(process.started_at.is_none());
        assert_eq!(process.restarts, 0);
        assert!(process.exit_code.is_none());
        assert!(process.error.is_none());
        assert!(process.instance.is_none());
        assert!(process.assigned_port.is_none());
    }

    #[test]
    fn test_process_status() {
        let config = create_test_config();
        let mut process = Process::new(config);

        // Set some test data
        process.set_state(ProcessState::Online);
        process.restarts = 5;
        process.update_monitoring(25.5, 1024 * 1024);
        process.set_assigned_port(Some(8080));
        process.set_instance(Some(1));

        let status = process.status();
        assert_eq!(status.name, "test-process");
        assert_eq!(status.state, ProcessState::Online);
        assert_eq!(status.restarts, 5);
        assert_eq!(status.cpu_usage, 25.5);
        assert_eq!(status.memory_usage, 1024 * 1024);
        assert_eq!(status.assigned_port, Some(8080));
        assert_eq!(status.instance, Some(1));
        assert_eq!(status.namespace, "default");
    }

    #[test]
    fn test_process_is_running() {
        let config = create_test_config();
        let mut process = Process::new(config);

        assert!(!process.is_running());

        process.set_state(ProcessState::Starting);
        assert!(process.is_running());

        process.set_state(ProcessState::Online);
        assert!(process.is_running());

        process.set_state(ProcessState::Stopping);
        assert!(!process.is_running());

        process.set_state(ProcessState::Stopped);
        assert!(!process.is_running());

        process.set_state(ProcessState::Errored);
        assert!(!process.is_running());

        process.set_state(ProcessState::Restarting);
        assert!(!process.is_running());
    }

    #[test]
    fn test_process_set_state() {
        let config = create_test_config();
        let mut process = Process::new(config);

        assert!(process.started_at.is_none());

        process.set_state(ProcessState::Online);
        assert_eq!(process.state, ProcessState::Online);
        assert!(process.started_at.is_some());

        // Setting state again shouldn't change started_at
        let original_start = process.started_at;
        process.set_state(ProcessState::Online);
        assert_eq!(process.started_at, original_start);

        process.set_state(ProcessState::Stopped);
        assert_eq!(process.state, ProcessState::Stopped);
        // started_at should remain unchanged when setting to stopped
        assert_eq!(process.started_at, original_start);
    }

    #[test]
    fn test_process_set_assigned_port() {
        let config = create_test_config();
        let mut process = Process::new(config);

        assert!(process.assigned_port.is_none());

        process.set_assigned_port(Some(8080));
        assert_eq!(process.assigned_port, Some(8080));

        process.set_assigned_port(None);
        assert!(process.assigned_port.is_none());
    }

    #[test]
    fn test_process_set_instance() {
        let config = create_test_config();
        let mut process = Process::new(config);

        assert!(process.instance.is_none());

        process.set_instance(Some(1));
        assert_eq!(process.instance, Some(1));

        process.set_instance(None);
        assert!(process.instance.is_none());
    }

    #[test]
    fn test_process_update_monitoring() {
        let config = create_test_config();
        let mut process = Process::new(config);

        assert_eq!(process.monitoring.cpu_usage, 0.0);
        assert_eq!(process.monitoring.memory_usage, 0);
        assert!(process.monitoring.last_update.is_none());

        process.update_monitoring(50.5, 2048);
        assert_eq!(process.monitoring.cpu_usage, 50.5);
        assert_eq!(process.monitoring.memory_usage, 2048);
        assert!(process.monitoring.last_update.is_some());
    }

    #[test]
    fn test_process_uptime_seconds() {
        let config = create_test_config();
        let mut process = Process::new(config);

        assert!(process.uptime_seconds().is_none());

        process.set_state(ProcessState::Online);
        let uptime = process.uptime_seconds();
        assert!(uptime.is_some());
        assert!(uptime.unwrap() >= 0);
    }

    #[test]
    fn test_process_should_auto_restart() {
        let mut config = create_test_config();
        config.autorestart = true;
        config.max_restarts = 5;

        let mut process = Process::new(config);
        process.set_state(ProcessState::Stopped);

        // Should auto-restart when stopped and under restart limit
        assert!(process.should_auto_restart());

        // Should not auto-restart when at restart limit
        process.restarts = 5;
        assert!(!process.should_auto_restart());

        // Should not auto-restart when autorestart is disabled
        process.restarts = 0;
        process.config.autorestart = false;
        assert!(!process.should_auto_restart());

        // Should not auto-restart when not stopped
        process.config.autorestart = true;
        process.set_state(ProcessState::Online);
        assert!(!process.should_auto_restart());
    }

    #[test]
    fn test_process_pid() {
        let config = create_test_config();
        let process = Process::new(config);

        // Process without child should return None
        assert!(process.pid().is_none());
    }

    #[tokio::test]
    async fn test_process_start_already_running() {
        let config = create_test_config();
        let mut process = Process::new(config);

        // Simulate running state
        process.set_state(ProcessState::Online);

        let result = process.start().await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error::ProcessAlreadyRunning(_)
        ));
    }

    #[tokio::test]
    async fn test_process_start_invalid_command() {
        let config = ProcessConfig::builder()
            .name("test-invalid")
            .script("this-command-does-not-exist-12345")
            .build()
            .unwrap();

        let mut process = Process::new(config);
        let result = process.start().await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            Error::ProcessStartFailed { .. }
        ));
        assert_eq!(process.state, ProcessState::Errored);
        assert!(process.error.is_some());
    }

    #[tokio::test]
    async fn test_process_stop_not_running() {
        let config = create_test_config();
        let mut process = Process::new(config);

        // Process is not running, stop should succeed
        let result = process.stop().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_process_check_status_no_child() {
        let config = create_test_config();
        let mut process = Process::new(config);

        let result = process.check_status().await;
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should return false when no child
    }

    #[test]
    fn test_process_status_serialization() {
        let config = create_test_config();
        let process = Process::new(config);
        let status = process.status();

        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: ProcessStatus = serde_json::from_str(&serialized).unwrap();

        assert_eq!(status.name, deserialized.name);
        assert_eq!(status.state, deserialized.state);
        assert_eq!(status.restarts, deserialized.restarts);
    }
}
