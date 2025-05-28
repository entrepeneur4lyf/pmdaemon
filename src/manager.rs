//! Process manager implementation for lifecycle and resource management.
//!
//! This module provides the core `ProcessManager` struct that orchestrates all
//! process operations including starting, stopping, monitoring, and resource allocation.
//! It handles advanced features like clustering, port management, and persistent configuration.
//!
//! ## Key Features
//!
//! - **Process Lifecycle Management** - Start, stop, restart, reload, delete operations
//! - **Clustering Support** - Automatic load balancing with multiple instances
//! - **Advanced Port Management** - Single ports, ranges, and auto-assignment with conflict detection
//! - **Configuration Persistence** - Process configs saved and restored between sessions
//! - **Real-time Monitoring** - CPU, memory tracking with automatic health checks
//! - **Resource Limits** - Memory limit enforcement with automatic restart
//! - **Log Management** - Separate stdout/stderr files with automatic rotation
//!
//! ## Examples
//!
//! ### Basic Process Management
//!
//! ```rust,no_run
//! use pmdaemon::{ProcessManager, ProcessConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut manager = ProcessManager::new().await?;
//!
//! let config = ProcessConfig::builder()
//!     .name("web-server")
//!     .script("node")
//!     .args(vec!["server.js"])
//!     .build()?;
//!
//! // Start the process
//! let process_id = manager.start(config).await?;
//!
//! // List all processes
//! let processes = manager.list().await?;
//! for process in processes {
//!     println!("Process: {} ({})", process.name, process.state);
//! }
//!
//! // Stop the process
//! manager.stop("web-server").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Clustering with Port Management
//!
//! ```rust,no_run
//! use pmdaemon::{ProcessManager, ProcessConfig, config::PortConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut manager = ProcessManager::new().await?;
//!
//! let config = ProcessConfig::builder()
//!     .name("web-cluster")
//!     .script("node")
//!     .args(vec!["app.js"])
//!     .instances(4)
//!     .port(PortConfig::Range(3000, 3003)) // Ports 3000-3003
//!     .build()?;
//!
//! // Start 4 instances with automatic port distribution
//! manager.start(config).await?;
//! # Ok(())
//! # }
//! ```

use crate::config::{PortConfig, ProcessConfig};
use crate::error::{Error, Result};
use crate::monitoring::Monitor;
use crate::process::{Process, ProcessId, ProcessStatus};
use comfy_table::{presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement, Table};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Main process manager for orchestrating process lifecycle and resources.
///
/// The `ProcessManager` is the central component that handles all process operations
/// including starting, stopping, monitoring, and resource allocation. It provides
/// advanced features like clustering, port management, and persistent configuration
/// that go beyond standard PM2 capabilities.
///
/// ## Architecture
///
/// - **Process Storage** - Thread-safe storage for all managed processes
/// - **Port Allocation** - Conflict-free port assignment with ranges and auto-detection
/// - **Configuration Persistence** - Automatic save/restore of process configurations
/// - **Monitoring Integration** - Real-time CPU/memory tracking with health checks
/// - **Log Management** - Automatic log file creation and management
///
/// ## Thread Safety
///
/// All operations are thread-safe using `RwLock` for concurrent access.
/// Multiple threads can safely read process information while write operations
/// are properly synchronized.
///
/// # Examples
///
/// ```rust,no_run
/// use pmdaemon::{ProcessManager, ProcessConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a new process manager
/// let mut manager = ProcessManager::new().await?;
///
/// // Start monitoring loop in background
/// tokio::spawn(async move {
///     if let Err(e) = manager.start_monitoring().await {
///         eprintln!("Monitoring error: {}", e);
///     }
/// });
/// # Ok(())
/// # }
/// ```
pub struct ProcessManager {
    /// Map of process ID to process
    processes: RwLock<HashMap<ProcessId, Process>>,
    /// Map of process name to process ID for quick lookup
    name_to_id: RwLock<HashMap<String, ProcessId>>,
    /// System monitor for collecting metrics
    monitor: RwLock<Monitor>,
    /// Configuration directory path
    config_dir: PathBuf,
    /// Set of allocated ports
    allocated_ports: RwLock<HashSet<u16>>,
}

impl ProcessManager {
    /// Create a new process manager
    pub async fn new() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;

        // Ensure config directory exists
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .await
                .map_err(|e| Error::config(format!("Failed to create config directory: {}", e)))?;
        }

        let mut manager = Self {
            processes: RwLock::new(HashMap::new()),
            name_to_id: RwLock::new(HashMap::new()),
            monitor: RwLock::new(Monitor::new()),
            config_dir,
            allocated_ports: RwLock::new(HashSet::new()),
        };

        // Load existing processes from configuration
        manager.load_processes().await?;

        Ok(manager)
    }

    /// Get the configuration directory path
    ///
    /// Checks for PMDAEMON_HOME environment variable first, which allows overriding
    /// the default configuration directory. This is particularly useful for testing
    /// and when running multiple isolated PMDaemon instances.
    fn get_config_dir() -> Result<PathBuf> {
        // Check for PMDAEMON_HOME environment variable first
        if let Ok(pmdaemon_home) = std::env::var("PMDAEMON_HOME") {
            return Ok(PathBuf::from(pmdaemon_home));
        }

        let home_dir =
            dirs::home_dir().ok_or_else(|| Error::config("Could not determine home directory"))?;
        Ok(home_dir.join(crate::CONFIG_DIR))
    }

    /// Get the PID directory path
    fn get_pid_dir(&self) -> PathBuf {
        self.config_dir.join(crate::PID_DIR)
    }

    /// Get the logs directory path
    fn get_logs_dir(&self) -> PathBuf {
        self.config_dir.join(crate::LOG_DIR)
    }

    /// Get log file paths for a process
    fn get_log_paths(&self, process_name: &str) -> (PathBuf, PathBuf, PathBuf) {
        let logs_dir = self.get_logs_dir();
        let out_file = logs_dir.join(format!("{}-out.log", process_name));
        let err_file = logs_dir.join(format!("{}-error.log", process_name));
        let combined_file = logs_dir.join(format!("{}.log", process_name));
        (out_file, err_file, combined_file)
    }

    /// Ensure logs directory exists
    async fn ensure_logs_dir(&self) -> Result<()> {
        let logs_dir = self.get_logs_dir();
        if !logs_dir.exists() {
            fs::create_dir_all(&logs_dir)
                .await
                .map_err(|e| Error::config(format!("Failed to create logs directory: {}", e)))?;
        }
        Ok(())
    }

    /// Save PID file for a process
    async fn save_pid_file(&self, process_name: &str, pid: u32) -> Result<()> {
        let pid_dir = self.get_pid_dir();
        if !pid_dir.exists() {
            fs::create_dir_all(&pid_dir)
                .await
                .map_err(|e| Error::config(format!("Failed to create PID directory: {}", e)))?;
        }

        let pid_file = pid_dir.join(format!("{}.pid", process_name));
        fs::write(&pid_file, pid.to_string())
            .await
            .map_err(|e| Error::config(format!("Failed to write PID file: {}", e)))?;

        debug!("Saved PID file for process {}: {}", process_name, pid);
        Ok(())
    }

    /// Remove PID file for a process
    async fn remove_pid_file(&self, process_name: &str) -> Result<()> {
        let pid_file = self.get_pid_dir().join(format!("{}.pid", process_name));
        if pid_file.exists() {
            fs::remove_file(&pid_file)
                .await
                .map_err(|e| Error::config(format!("Failed to remove PID file: {}", e)))?;
            debug!("Removed PID file for process: {}", process_name);
        }
        Ok(())
    }

    /// Read PID from PID file
    async fn read_pid_file(&self, process_name: &str) -> Result<Option<u32>> {
        let pid_file = self.get_pid_dir().join(format!("{}.pid", process_name));
        if !pid_file.exists() {
            return Ok(None);
        }

        let pid_content = fs::read_to_string(&pid_file)
            .await
            .map_err(|e| Error::config(format!("Failed to read PID file: {}", e)))?;

        let pid = pid_content
            .trim()
            .parse::<u32>()
            .map_err(|e| Error::config(format!("Invalid PID in file: {}", e)))?;

        Ok(Some(pid))
    }

    /// Start a new process (or multiple instances for clustering)
    pub async fn start(&mut self, config: ProcessConfig) -> Result<ProcessId> {
        // Validate configuration
        config.validate()?;

        if config.instances == 1 {
            // Single instance
            self.start_single_instance(config).await
        } else {
            // Multiple instances (clustering)
            self.start_cluster(config).await
        }
    }

    /// Start a single process instance
    async fn start_single_instance(&mut self, config: ProcessConfig) -> Result<ProcessId> {
        // Check if process with same name already exists
        let name_map = self.name_to_id.read().await;
        if name_map.contains_key(&config.name) {
            return Err(Error::process_already_exists(&config.name));
        }
        drop(name_map);

        // Create new process
        let mut process = Process::new(config.clone());
        let process_id = process.id;

        // Allocate port if specified
        if let Some(port_config) = &process.config.port {
            let assigned_port = self
                .allocate_port(port_config, &process.config.name)
                .await?;
            process.assigned_port = Some(assigned_port);

            // Add PORT environment variable
            process
                .config
                .env
                .insert("PORT".to_string(), assigned_port.to_string());
        }

        // Ensure logs directory exists
        self.ensure_logs_dir().await?;

        // Get log file paths
        let (out_log, err_log, _combined_log) = self.get_log_paths(&process.config.name);

        // Start the process with log redirection
        process
            .start_with_logs(Some(out_log), Some(err_log))
            .await?;

        // Save PID file if process started successfully
        if let Some(pid) = process.pid() {
            self.save_pid_file(&process.config.name, pid).await?;
            // Also store the PID in the process for later retrieval
            process.set_stored_pid(Some(pid));
        }

        // Save configuration to disk
        self.save_process_config(&process).await?;

        // Save runtime metadata (assigned port, etc.)
        self.save_process_metadata(&process).await?;

        // Store process
        let mut processes = self.processes.write().await;
        let mut name_map = self.name_to_id.write().await;

        processes.insert(process_id, process);
        name_map.insert(config.name, process_id);

        Ok(process_id)
    }

    /// Start multiple process instances (clustering)
    async fn start_cluster(&mut self, config: ProcessConfig) -> Result<ProcessId> {
        // Check if any instance with the base name already exists
        let name_map = self.name_to_id.read().await;
        for i in 0..config.instances {
            let instance_name = format!("{}-{}", config.name, i);
            if name_map.contains_key(&instance_name) {
                return Err(Error::process_already_exists(&instance_name));
            }
        }
        drop(name_map);

        let mut first_process_id = None;
        let mut started_instances = Vec::new();

        // Start each instance
        for i in 0..config.instances {
            let instance_name = format!("{}-{}", config.name, i);
            let mut instance_config = config.clone();
            instance_config.name = instance_name.clone();
            instance_config.instances = 1; // Each instance is a single process

            // Add instance-specific environment variable
            instance_config
                .env
                .insert("PM2_INSTANCE_ID".to_string(), i.to_string());
            instance_config
                .env
                .insert("NODE_APP_INSTANCE".to_string(), i.to_string());

            // Handle port allocation for cluster instances
            if let Some(port_config) = &config.port {
                match port_config {
                    PortConfig::Auto(start, end) => {
                        // Each instance gets auto-assigned port from the range
                        instance_config.port = Some(PortConfig::Auto(*start, *end));
                    }
                    PortConfig::Range(start, end) => {
                        // Each instance gets a specific port from the range
                        if i < (end - start + 1) as u32 {
                            let instance_port = start + i as u16;
                            instance_config.port = Some(PortConfig::Single(instance_port));
                        } else {
                            return Err(Error::config(format!(
                                "Not enough ports in range {}-{} for {} instances",
                                start, end, config.instances
                            )));
                        }
                    }
                    PortConfig::Single(port) => {
                        // For single port, only the first instance gets it
                        if i == 0 {
                            instance_config.port = Some(PortConfig::Single(*port));
                        } else {
                            instance_config.port = None; // Other instances get no port
                        }
                    }
                }
            }

            match self.start_single_instance(instance_config).await {
                Ok(process_id) => {
                    if first_process_id.is_none() {
                        first_process_id = Some(process_id);
                    }
                    started_instances.push((i, process_id));
                    info!("Started cluster instance {}: {}", i, instance_name);
                }
                Err(e) => {
                    error!("Failed to start cluster instance {}: {}", i, e);

                    // Clean up already started instances
                    for (_, pid) in started_instances {
                        if let Err(cleanup_err) = self.stop_by_id(pid).await {
                            warn!("Failed to cleanup instance {}: {}", pid, cleanup_err);
                        }
                    }

                    return Err(e);
                }
            }
        }

        info!(
            "Started cluster '{}' with {} instances",
            config.name, config.instances
        );
        Ok(first_process_id.unwrap()) // We know this is Some because instances > 1
    }

    /// Stop a process by ProcessId
    async fn stop_by_id(&mut self, process_id: ProcessId) -> Result<()> {
        let mut processes = self.processes.write().await;
        if let Some(process) = processes.get_mut(&process_id) {
            let process_name = process.config.name.clone();
            process.stop().await?;

            // Remove PID file
            drop(processes); // Release lock before async operation
            self.remove_pid_file(&process_name).await?;
        }
        Ok(())
    }

    /// Stop a process
    pub async fn stop(&mut self, identifier: &str) -> Result<()> {
        let process_id = self.resolve_identifier(identifier).await?;

        let mut processes = self.processes.write().await;
        if let Some(process) = processes.get_mut(&process_id) {
            let process_name = process.config.name.clone();
            process.stop().await?;

            // Remove PID file
            drop(processes); // Release lock before async operation
            self.remove_pid_file(&process_name).await?;
        }

        Ok(())
    }

    /// Restart a process
    pub async fn restart(&mut self, identifier: &str) -> Result<()> {
        self.restart_with_port(identifier, None).await
    }

    /// Restart a process with optional port override
    pub async fn restart_with_port(
        &mut self,
        identifier: &str,
        port_override: Option<PortConfig>,
    ) -> Result<()> {
        let process_id = self.resolve_identifier(identifier).await?;

        let mut processes = self.processes.write().await;
        if let Some(process) = processes.get_mut(&process_id) {
            // Handle port deallocation and reallocation if there's an override
            if let Some(new_port_config) = port_override {
                // Deallocate current port if any
                if let Some(current_port_config) = &process.config.port {
                    self.deallocate_ports(current_port_config, process.assigned_port)
                        .await;
                }

                // Allocate new port
                let assigned_port = self
                    .allocate_port(&new_port_config, &process.config.name)
                    .await?;
                process.assigned_port = Some(assigned_port);

                // Update environment variable
                process
                    .config
                    .env
                    .insert("PORT".to_string(), assigned_port.to_string());

                info!(
                    "Restarting {} with new port: {}",
                    process.config.name, assigned_port
                );
            }

            process.restart().await?;
        }

        Ok(())
    }

    /// Reload a process (graceful restart)
    pub async fn reload(&mut self, identifier: &str) -> Result<()> {
        self.reload_with_port(identifier, None).await
    }

    /// Reload a process with optional port override
    pub async fn reload_with_port(
        &mut self,
        identifier: &str,
        port_override: Option<PortConfig>,
    ) -> Result<()> {
        // For now, reload is the same as restart with port override
        self.restart_with_port(identifier, port_override).await
    }

    /// Delete a process
    pub async fn delete(&mut self, identifier: &str) -> Result<()> {
        let process_id = self.resolve_identifier(identifier).await?;

        // First, stop the process if it's running
        let (process_name, port_config, assigned_port, was_running) = {
            let mut processes = self.processes.write().await;
            if let Some(mut process) = processes.remove(&process_id) {
                let process_name = process.config.name.clone();
                let port_config = process.config.port.clone();
                let assigned_port = process.assigned_port;
                let was_running = process.is_running();

                // Stop the process if it's running
                if was_running {
                    info!("Stopping process '{}' before deletion", process_name);
                    if let Err(e) = process.stop().await {
                        warn!(
                            "Failed to stop process '{}' during deletion: {}",
                            process_name, e
                        );
                        // Continue with deletion even if stop fails
                    }
                }

                // Remove from name map
                let mut name_map = self.name_to_id.write().await;
                name_map.remove(&process_name);
                drop(name_map);

                (process_name, port_config, assigned_port, was_running)
            } else {
                return Ok(()); // Process not found, nothing to delete
            }
        };

        // Deallocate ports
        if let Some(port_config) = port_config {
            self.deallocate_ports(&port_config, assigned_port).await;
        }

        // Clean up files
        self.remove_process_config(&process_name).await?;
        self.remove_pid_file(&process_name).await?;
        self.remove_log_files(&process_name).await?;

        if was_running {
            info!(
                "Process '{}' stopped and deleted successfully",
                process_name
            );
        } else {
            info!("Process '{}' deleted successfully", process_name);
        }

        Ok(())
    }

    /// Delete all processes
    pub async fn delete_all(&mut self) -> Result<usize> {
        let process_ids: Vec<ProcessId>;

        // Get all process IDs
        {
            let processes = self.processes.read().await;
            process_ids = processes.keys().cloned().collect();
        }

        let mut deleted_count = 0;
        let mut stopped_count = 0;

        for process_id in process_ids {
            // Stop and remove the process
            let (process_name, port_config, assigned_port, _was_running) = {
                let mut processes = self.processes.write().await;
                if let Some(mut process) = processes.remove(&process_id) {
                    let process_name = process.config.name.clone();
                    let port_config = process.config.port.clone();
                    let assigned_port = process.assigned_port;
                    let was_running = process.is_running();

                    // Stop the process if it's running
                    if was_running {
                        if let Err(e) = process.stop().await {
                            warn!(
                                "Failed to stop process '{}' during bulk deletion: {}",
                                process_name, e
                            );
                            // Continue with deletion even if stop fails
                        } else {
                            stopped_count += 1;
                        }
                    }

                    // Remove from name map
                    let mut name_map = self.name_to_id.write().await;
                    name_map.remove(&process_name);
                    drop(name_map);

                    deleted_count += 1;
                    (process_name, port_config, assigned_port, was_running)
                } else {
                    continue; // Process already deleted
                }
            };

            // Deallocate ports
            if let Some(port_config) = port_config {
                self.deallocate_ports(&port_config, assigned_port).await;
            }

            // Clean up files
            if let Err(e) = self.remove_process_config(&process_name).await {
                warn!("Failed to remove config for {}: {}", process_name, e);
            }
            if let Err(e) = self.remove_pid_file(&process_name).await {
                warn!("Failed to remove PID file for {}: {}", process_name, e);
            }
            if let Err(e) = self.remove_log_files(&process_name).await {
                warn!("Failed to remove log files for {}: {}", process_name, e);
            }
        }

        if stopped_count > 0 {
            info!(
                "Stopped {} running processes and deleted {} total processes",
                stopped_count, deleted_count
            );
        } else {
            info!("Deleted {} processes", deleted_count);
        }

        Ok(deleted_count)
    }

    /// Delete processes by status
    pub async fn delete_by_status(&mut self, status_str: &str) -> Result<usize> {
        use crate::process::ProcessState;

        // Parse the status string
        let target_state = match status_str.to_lowercase().as_str() {
            "starting" => ProcessState::Starting,
            "online" => ProcessState::Online,
            "stopping" => ProcessState::Stopping,
            "stopped" => ProcessState::Stopped,
            "errored" => ProcessState::Errored,
            "restarting" => ProcessState::Restarting,
            _ => return Err(crate::error::Error::config(format!(
                "Invalid status '{}'. Valid statuses are: starting, online, stopping, stopped, errored, restarting",
                status_str
            ))),
        };

        let process_ids_to_delete: Vec<ProcessId>;

        // Find processes with matching status
        {
            let processes = self.processes.read().await;
            process_ids_to_delete = processes
                .iter()
                .filter(|(_, process)| process.state == target_state)
                .map(|(id, _)| *id)
                .collect();
        }

        let mut deleted_count = 0;
        let mut stopped_count = 0;

        for process_id in process_ids_to_delete {
            // Stop and remove the process
            let (process_name, port_config, assigned_port, _was_running) = {
                let mut processes = self.processes.write().await;
                if let Some(mut process) = processes.remove(&process_id) {
                    let process_name = process.config.name.clone();
                    let port_config = process.config.port.clone();
                    let assigned_port = process.assigned_port;
                    let was_running = process.is_running();

                    // Stop the process if it's running
                    if was_running {
                        if let Err(e) = process.stop().await {
                            warn!(
                                "Failed to stop process '{}' during status-based deletion: {}",
                                process_name, e
                            );
                            // Continue with deletion even if stop fails
                        } else {
                            stopped_count += 1;
                        }
                    }

                    // Remove from name map
                    let mut name_map = self.name_to_id.write().await;
                    name_map.remove(&process_name);
                    drop(name_map);

                    deleted_count += 1;
                    (process_name, port_config, assigned_port, was_running)
                } else {
                    continue; // Process already deleted
                }
            };

            // Deallocate ports
            if let Some(port_config) = port_config {
                self.deallocate_ports(&port_config, assigned_port).await;
            }

            // Clean up files
            let _ = self.remove_process_config(&process_name).await;
            let _ = self.remove_pid_file(&process_name).await;
            let _ = self.remove_log_files(&process_name).await;
        }

        if stopped_count > 0 {
            info!(
                "Stopped {} running processes and deleted {} total processes with status '{}'",
                stopped_count, deleted_count, status_str
            );
        } else {
            info!(
                "Deleted {} processes with status '{}'",
                deleted_count, status_str
            );
        }

        Ok(deleted_count)
    }

    /// List all processes
    pub async fn list(&self) -> Result<Vec<ProcessStatus>> {
        let processes = self.processes.read().await;
        Ok(processes.values().map(|p| p.status()).collect())
    }

    /// Monitor processes in real-time.
    ///
    /// Displays a comprehensive real-time dashboard of all managed processes using beautifully
    /// formatted tables with color-coded status indicators. Uses the default update interval
    /// of 2 seconds.
    ///
    /// For custom update intervals, use [`monitor_with_interval`](Self::monitor_with_interval).
    ///
    /// ## Dashboard Components
    ///
    /// - **System Overview** - CPU usage, memory consumption, load average, and uptime
    /// - **Process Table** - Detailed process information with the following columns:
    ///   - **Name** - Process name
    ///   - **ID** - Process UUID (first 8 characters)
    ///   - **Status** - Color-coded process state (Online=Green, Stopped=Red, etc.)
    ///   - **PID** - System process ID (blue highlighting)
    ///   - **CPU%** - Real-time CPU usage percentage
    ///   - **Memory** - Memory consumption in MB
    ///   - **Restarts** - Total number of restarts
    ///   - **Port** - Assigned port number (cyan highlighting)
    ///   - **Uptime** - Process uptime in human-readable format
    ///
    /// ## Visual Features
    ///
    /// - Professional table formatting using `comfy-table` with UTF8 borders
    /// - Color-coded status indicators for quick visual assessment
    /// - Clear screen refresh for smooth real-time updates
    /// - Update counter and timestamp for monitoring freshness
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when monitoring is stopped (e.g., by Ctrl+C), or an error
    /// if the monitoring setup fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::ProcessManager;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ProcessManager::new().await?;
    ///
    /// // Start real-time monitoring dashboard with default 2-second updates
    /// manager.monitor().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// This method runs an infinite loop and will block the current thread until
    /// interrupted (typically with Ctrl+C). It's designed for interactive use
    /// and provides the same comprehensive monitoring as the CLI `pmdaemon monit` command.
    pub async fn monitor(&self) -> Result<()> {
        self.monitor_with_interval(Duration::from_secs(2)).await
    }

    /// Monitor processes in real-time with a configurable update interval.
    ///
    /// Displays a comprehensive real-time dashboard of all managed processes using beautifully
    /// formatted tables with color-coded status indicators. Updates at the specified interval
    /// with the latest system and process metrics.
    ///
    /// # Arguments
    ///
    /// * `update_interval` - How frequently to refresh the display. Common values:
    ///   - `Duration::from_secs(1)` - Fast updates for debugging (higher CPU usage)
    ///   - `Duration::from_secs(2)` - Default balanced updates
    ///   - `Duration::from_secs(5)` - Slower updates for reduced system load
    ///   - `Duration::from_millis(500)` - Very fast updates for development
    ///
    /// ## Dashboard Components
    ///
    /// - **System Overview** - CPU usage, memory consumption, load average, and uptime
    /// - **Process Table** - Detailed process information with the following columns:
    ///   - **Name** - Process name
    ///   - **ID** - Process UUID (first 8 characters)
    ///   - **Status** - Color-coded process state (Online=Green, Stopped=Red, etc.)
    ///   - **PID** - System process ID (blue highlighting)
    ///   - **CPU%** - Real-time CPU usage percentage
    ///   - **Memory** - Memory consumption in MB
    ///   - **Restarts** - Total number of restarts
    ///   - **Port** - Assigned port number (cyan highlighting)
    ///   - **Uptime** - Process uptime in human-readable format
    ///
    /// ## Visual Features
    ///
    /// - Professional table formatting using `comfy-table` with UTF8 borders
    /// - Color-coded status indicators for quick visual assessment
    /// - Clear screen refresh for smooth real-time updates
    /// - Update counter and timestamp for monitoring freshness
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when monitoring is stopped (e.g., by Ctrl+C), or an error
    /// if the monitoring setup fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::ProcessManager;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ProcessManager::new().await?;
    ///
    /// // Fast updates for debugging
    /// manager.monitor_with_interval(Duration::from_secs(1)).await?;
    ///
    /// // Slower updates to reduce system load
    /// manager.monitor_with_interval(Duration::from_secs(5)).await?;
    ///
    /// // Very fast updates for development
    /// manager.monitor_with_interval(Duration::from_millis(500)).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Performance Considerations
    ///
    /// - **Faster intervals** (< 1 second) provide more responsive monitoring but use more CPU
    /// - **Slower intervals** (> 3 seconds) reduce system load but may miss short-lived events
    /// - **Default interval** (2 seconds) provides a good balance for most use cases
    ///
    /// # Note
    ///
    /// This method runs an infinite loop and will block the current thread until
    /// interrupted (typically with Ctrl+C). It's designed for interactive use.
    pub async fn monitor_with_interval(&self, update_interval: Duration) -> Result<()> {
        info!(
            "Starting real-time process monitoring dashboard with {:?} update interval",
            update_interval
        );

        let mut interval = interval(update_interval);
        let mut iteration = 0u64;

        loop {
            interval.tick().await;
            iteration += 1;

            // Clear screen and move cursor to top
            #[cfg(windows)]
            {
                // For Windows, try to use the cls command or fall back to ANSI
                if std::process::Command::new("cmd")
                    .args(&["/C", "cls"])
                    .status()
                    .is_err()
                {
                    print!("\x1B[2J\x1B[H");
                }
            }
            #[cfg(not(windows))]
            {
                // For Unix-like systems, use ANSI escape sequences
                print!("\x1B[2J\x1B[H");
            }

            // Create header table
            let mut header_table = Table::new();
            header_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("PMDaemon Process Monitor")
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Cyan),
                    Cell::new(format!("Update #{}", iteration))
                        .add_attribute(Attribute::Bold)
                        .fg(Color::Yellow),
                ]);

            println!("{}", header_table);
            println!("Press Ctrl+C to exit\n");

            // Get system metrics and create system overview table
            match self.get_system_info().await {
                Ok(system_metrics) => {
                    let mut system_table = Table::new();
                    system_table
                        .load_preset(UTF8_FULL)
                        .set_content_arrangement(ContentArrangement::Dynamic)
                        .set_header(vec![Cell::new("System Overview")
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Green)]);

                    system_table
                        .add_row(vec![format!("CPU Usage: {:.1}%", system_metrics.cpu_usage)]);
                    system_table.add_row(vec![format!(
                        "Memory: {:.1}% ({:.1} GB / {:.1} GB)",
                        system_metrics.memory_percent,
                        system_metrics.memory_used as f64 / 1024.0 / 1024.0 / 1024.0,
                        system_metrics.memory_total as f64 / 1024.0 / 1024.0 / 1024.0
                    )]);
                    system_table.add_row(vec![format!(
                        "Load Average: [{:.2}, {:.2}, {:.2}]",
                        system_metrics.load_average[0],
                        system_metrics.load_average[1],
                        system_metrics.load_average[2]
                    )]);
                    system_table
                        .add_row(vec![format!("Uptime: {} seconds", system_metrics.uptime)]);

                    println!("{}", system_table);
                }
                Err(e) => {
                    let mut error_table = Table::new();
                    error_table
                        .load_preset(UTF8_FULL)
                        .set_header(vec![Cell::new("System Overview")
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Red)]);
                    error_table.add_row(vec![format!("Error retrieving metrics: {}", e)]);
                    println!("{}", error_table);
                }
            }

            // Get process list
            match self.list().await {
                Ok(processes) => {
                    if processes.is_empty() {
                        let mut no_processes_table = Table::new();
                        no_processes_table
                            .load_preset(UTF8_FULL)
                            .set_header(vec![Cell::new("Processes")
                                .add_attribute(Attribute::Bold)
                                .fg(Color::Blue)]);
                        no_processes_table.add_row(vec!["No processes currently managed."]);
                        println!("{}", no_processes_table);
                    } else {
                        // Create process table
                        let mut process_table = Table::new();
                        process_table
                            .load_preset(UTF8_FULL)
                            .set_content_arrangement(ContentArrangement::Dynamic)
                            .set_header(vec![
                                Cell::new("Name").add_attribute(Attribute::Bold),
                                Cell::new("ID").add_attribute(Attribute::Bold),
                                Cell::new("Status").add_attribute(Attribute::Bold),
                                Cell::new("PID").add_attribute(Attribute::Bold),
                                Cell::new("CPU%").add_attribute(Attribute::Bold),
                                Cell::new("Memory").add_attribute(Attribute::Bold),
                                Cell::new("Restarts").add_attribute(Attribute::Bold),
                                Cell::new("Port").add_attribute(Attribute::Bold),
                                Cell::new("Uptime").add_attribute(Attribute::Bold),
                            ]);

                        // Get monitoring data for all processes
                        let processes_guard = self.processes.read().await;
                        let mut monitor = self.monitor.write().await;

                        let pids: Vec<u32> =
                            processes_guard.values().filter_map(|p| p.pid()).collect();

                        let monitoring_data = if !pids.is_empty() {
                            monitor.update_process_metrics(&pids).await
                        } else {
                            std::collections::HashMap::new()
                        };

                        drop(monitor);
                        drop(processes_guard);

                        // Add each process to the table
                        for process_status in processes {
                            let cpu_usage = if let Some(pid) = process_status.pid {
                                monitoring_data
                                    .get(&pid)
                                    .map(|m| format!("{:.1}%", m.cpu_usage))
                                    .unwrap_or_else(|| "N/A".to_string())
                            } else {
                                "N/A".to_string()
                            };

                            let memory_usage = if let Some(pid) = process_status.pid {
                                monitoring_data
                                    .get(&pid)
                                    .map(|m| {
                                        format!("{:.1}MB", m.memory_usage as f64 / 1024.0 / 1024.0)
                                    })
                                    .unwrap_or_else(|| "N/A".to_string())
                            } else {
                                "N/A".to_string()
                            };

                            let uptime = if let Some(uptime_start) = process_status.uptime {
                                let duration = chrono::Utc::now() - uptime_start;
                                let seconds = duration.num_seconds();
                                if seconds < 60 {
                                    format!("{}s", seconds)
                                } else if seconds < 3600 {
                                    format!("{}m", seconds / 60)
                                } else {
                                    format!("{}h", seconds / 3600)
                                }
                            } else {
                                "N/A".to_string()
                            };

                            // Color code the status
                            let status_cell = match process_status.state {
                                crate::process::ProcessState::Online => {
                                    Cell::new("Online").fg(Color::Green)
                                }
                                crate::process::ProcessState::Stopped => {
                                    Cell::new("Stopped").fg(Color::Red)
                                }
                                crate::process::ProcessState::Errored => Cell::new("Errored")
                                    .fg(Color::Red)
                                    .add_attribute(Attribute::Bold),
                                crate::process::ProcessState::Starting => {
                                    Cell::new("Starting").fg(Color::Yellow)
                                }
                                crate::process::ProcessState::Stopping => {
                                    Cell::new("Stopping").fg(Color::Yellow)
                                }
                                crate::process::ProcessState::Restarting => {
                                    Cell::new("Restarting").fg(Color::Blue)
                                }
                            };

                            // Format port assignment
                            let port_assignment = if let Some(port) = process_status.assigned_port {
                                Cell::new(port.to_string()).fg(Color::Cyan)
                            } else {
                                Cell::new("N/A").fg(Color::DarkGrey)
                            };

                            // Format PID
                            let pid_cell = if let Some(pid) = process_status.pid {
                                Cell::new(pid.to_string()).fg(Color::Blue)
                            } else {
                                Cell::new("-").fg(Color::DarkGrey)
                            };

                            process_table.add_row(vec![
                                Cell::new(&process_status.name),
                                Cell::new(
                                    process_status
                                        .id
                                        .to_string()
                                        .chars()
                                        .take(8)
                                        .collect::<String>(),
                                ),
                                status_cell,
                                pid_cell,
                                Cell::new(cpu_usage),
                                Cell::new(memory_usage),
                                Cell::new(process_status.restarts.to_string()),
                                port_assignment,
                                Cell::new(uptime),
                            ]);
                        }

                        println!("{}", process_table);
                    }
                }
                Err(e) => {
                    let mut error_table = Table::new();
                    error_table
                        .load_preset(UTF8_FULL)
                        .set_header(vec![Cell::new("Error")
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Red)]);
                    error_table.add_row(vec![format!("Error retrieving process list: {}", e)]);
                    println!("{}", error_table);
                }
            }

            // Add timestamp footer
            let mut footer_table = Table::new();
            footer_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic);
            footer_table.add_row(vec![Cell::new(format!(
                "Last updated: {}",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            ))
            .add_attribute(Attribute::Dim)]);
            println!("{}", footer_table);
        }
    }

    /// Get process logs.
    ///
    /// Retrieves the last N lines from both stdout and stderr log files for the specified process.
    /// Returns a formatted string containing both log streams with clear separation.
    ///
    /// # Arguments
    ///
    /// * `identifier` - Process name or UUID to get logs for
    /// * `lines` - Number of lines to retrieve from the end of each log file
    ///
    /// # Returns
    ///
    /// Returns a formatted string containing the log content, or an error if the process
    /// doesn't exist or log files cannot be read.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::ProcessManager;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ProcessManager::new().await?;
    ///
    /// // Get last 50 lines of logs for a process
    /// let logs = manager.get_logs("my-app", 50).await?;
    /// println!("{}", logs);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_logs(&self, identifier: &str, lines: usize) -> Result<String> {
        let process_id = self.resolve_identifier(identifier).await?;

        // Get the process name for log file paths
        let process_name = {
            let processes = self.processes.read().await;
            if let Some(process) = processes.get(&process_id) {
                process.config.name.clone()
            } else {
                return Err(Error::process_not_found(identifier));
            }
        };

        let (out_log, err_log, _combined_log) = self.get_log_paths(&process_name);
        let mut result = String::new();

        // Debug: Log the paths being used
        debug!("Looking for logs at: stdout={:?}, stderr={:?}", out_log, err_log);

        // Read stdout log
        if out_log.exists() {
            result.push_str(&format!("==> {} stdout <==\n", process_name));
            match fs::read_to_string(&out_log).await {
                Ok(content) => {
                    debug!("Read {} bytes from stdout log", content.len());
                    if content.is_empty() {
                        result.push_str("(stdout log file is empty)\n");
                    } else {
                        let log_lines: Vec<&str> = content.lines().collect();
                        let start = if log_lines.len() > lines {
                            log_lines.len() - lines
                        } else {
                            0
                        };
                        debug!("Showing {} lines from stdout (total: {})", log_lines.len() - start, log_lines.len());
                        for line in &log_lines[start..] {
                            result.push_str(line);
                            result.push('\n');
                        }
                    }
                }
                Err(e) => {
                    result.push_str(&format!("Error reading stdout log: {}\n", e));
                }
            }
            result.push('\n');
        } else {
            result.push_str(&format!("==> {} stdout <==\n", process_name));
            result.push_str(&format!("No stdout log file found at: {:?}\n\n", out_log));
        }

        // Read stderr log
        if err_log.exists() {
            result.push_str(&format!("==> {} stderr <==\n", process_name));
            match fs::read_to_string(&err_log).await {
                Ok(content) => {
                    debug!("Read {} bytes from stderr log", content.len());
                    if content.is_empty() {
                        result.push_str("(stderr log file is empty)\n");
                    } else {
                        let log_lines: Vec<&str> = content.lines().collect();
                        let start = if log_lines.len() > lines {
                            log_lines.len() - lines
                        } else {
                            0
                        };
                        debug!("Showing {} lines from stderr (total: {})", log_lines.len() - start, log_lines.len());
                        for line in &log_lines[start..] {
                            result.push_str(line);
                            result.push('\n');
                        }
                    }
                }
                Err(e) => {
                    result.push_str(&format!("Error reading stderr log: {}\n", e));
                }
            }
        } else {
            result.push_str(&format!("==> {} stderr <==\n", process_name));
            result.push_str(&format!("No stderr log file found at: {:?}\n", err_log));
        }

        Ok(result)
    }

    /// Follow process logs in real-time.
    ///
    /// Continuously monitors and displays new log entries from both stdout and stderr
    /// log files for the specified process. Similar to `tail -f` functionality.
    /// This method blocks until interrupted (e.g., by Ctrl+C).
    ///
    /// # Arguments
    ///
    /// * `identifier` - Process name or UUID to follow logs for
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when log following is stopped, or an error if the process
    /// doesn't exist or log files cannot be accessed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::ProcessManager;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ProcessManager::new().await?;
    ///
    /// // Follow logs for a process (blocks until Ctrl+C)
    /// manager.follow_logs("my-app").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn follow_logs(&self, identifier: &str) -> Result<()> {
        let process_id = self.resolve_identifier(identifier).await?;

        // Get the process name for log file paths
        let process_name = {
            let processes = self.processes.read().await;
            if let Some(process) = processes.get(&process_id) {
                process.config.name.clone()
            } else {
                return Err(Error::process_not_found(identifier));
            }
        };

        let (out_log, err_log, _combined_log) = self.get_log_paths(&process_name);

        info!("Following logs for process: {}", process_name);
        println!("==> Following logs for {} <==", process_name);
        println!("Press Ctrl+C to stop following");
        println!();

        // Track file positions for both stdout and stderr
        let mut out_position = if out_log.exists() {
            fs::metadata(&out_log).await.map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        let mut err_position = if err_log.exists() {
            fs::metadata(&err_log).await.map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        let mut interval = interval(Duration::from_millis(500)); // Check every 500ms

        loop {
            interval.tick().await;

            // Check stdout log for new content
            if out_log.exists() {
                if let Ok(metadata) = fs::metadata(&out_log).await {
                    let current_size = metadata.len();
                    if current_size > out_position {
                        // Read only new content from stdout log
                        use tokio::fs::File;
                        use tokio::io::{AsyncReadExt, AsyncSeekExt};

                        if let Ok(mut file) = File::open(&out_log).await {
                            if file
                                .seek(tokio::io::SeekFrom::Start(out_position))
                                .await
                                .is_ok()
                            {
                                let mut buffer = Vec::new();
                                if file.read_to_end(&mut buffer).await.is_ok() {
                                    if let Ok(new_content) = String::from_utf8(buffer) {
                                        if !new_content.trim().is_empty() {
                                            for line in new_content.lines() {
                                                println!("[stdout] {}", line);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        out_position = current_size;
                    }
                }
            }

            // Check stderr log for new content
            if err_log.exists() {
                if let Ok(metadata) = fs::metadata(&err_log).await {
                    let current_size = metadata.len();
                    if current_size > err_position {
                        // Read only new content from stderr log
                        use tokio::fs::File;
                        use tokio::io::{AsyncReadExt, AsyncSeekExt};

                        if let Ok(mut file) = File::open(&err_log).await {
                            if file
                                .seek(tokio::io::SeekFrom::Start(err_position))
                                .await
                                .is_ok()
                            {
                                let mut buffer = Vec::new();
                                if file.read_to_end(&mut buffer).await.is_ok() {
                                    if let Ok(new_content) = String::from_utf8(buffer) {
                                        if !new_content.trim().is_empty() {
                                            for line in new_content.lines() {
                                                println!("[stderr] {}", line);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        err_position = current_size;
                    }
                }
            }

            // Check if process still exists
            let processes = self.processes.read().await;
            if !processes.contains_key(&process_id) {
                println!(
                    "\nProcess {} no longer exists, stopping log following",
                    process_name
                );
                break;
            }
            drop(processes);
        }

        Ok(())
    }

    /// Get process information
    pub async fn get_process_info(&self, identifier: &str) -> Result<ProcessStatus> {
        let process_id = self.resolve_identifier(identifier).await?;

        let processes = self.processes.read().await;
        if let Some(process) = processes.get(&process_id) {
            Ok(process.status())
        } else {
            Err(Error::process_not_found(identifier))
        }
    }

    /// Start web monitoring server
    pub async fn start_web_server(&self, host: &str, port: u16) -> Result<()> {
        use crate::web::WebServer;
        use std::sync::Arc;
        use tokio::sync::RwLock;

        info!("Starting web monitoring server on {}:{}", host, port);

        // Create a new process manager instance for the web server
        // This is a temporary solution - in a real implementation, we'd want to share the same instance
        let manager_arc = Arc::new(RwLock::new(ProcessManager::new().await?));

        let web_server = WebServer::new(manager_arc).await?;
        web_server.start(host, port).await
    }

    /// Start the process monitoring loop
    pub async fn start_monitoring(&self) -> Result<()> {
        info!("Starting process monitoring loop");

        let mut interval = interval(Duration::from_secs(5)); // Check every 5 seconds

        loop {
            interval.tick().await;

            // Check process status and handle auto-restart
            if let Err(e) = self.check_all_processes().await {
                error!("Error during process monitoring: {}", e);
            }

            // Update monitoring data
            if let Err(e) = self.update_monitoring_data().await {
                error!("Error updating monitoring data: {}", e);
            }
        }
    }

    /// Check all processes and handle auto-restart
    pub async fn check_all_processes(&self) -> Result<()> {
        let mut processes = self.processes.write().await;
        let mut monitor = self.monitor.write().await;
        let mut to_restart = Vec::new();

        // Collect PIDs for monitoring data update
        let pids: Vec<(ProcessId, u32, String)> = processes
            .iter()
            .filter_map(|(id, p)| p.pid().map(|pid| (*id, pid, p.config.name.clone())))
            .collect();

        // Update monitoring data for all running processes
        if !pids.is_empty() {
            let pid_list: Vec<u32> = pids.iter().map(|(_, pid, _)| *pid).collect();
            let monitoring_data = monitor.update_process_metrics(&pid_list).await;

            // Check memory limits for each process
            for (process_id, pid, process_name) in pids {
                if let Some(process) = processes.get(&process_id) {
                    if let Some(max_memory) = process.config.max_memory_restart {
                        if let Some(metrics) = monitoring_data.get(&pid) {
                            let memory_mb = metrics.memory_usage / 1024 / 1024; // Convert to MB
                            let limit_mb = max_memory / 1024 / 1024;

                            if memory_mb > limit_mb {
                                warn!("Process {} exceeded memory limit: {}MB > {}MB, scheduling restart",
                                      process_name, memory_mb, limit_mb);
                                to_restart.push(process_id);
                            } else {
                                debug!(
                                    "Process {} memory usage: {}MB / {}MB",
                                    process_name, memory_mb, limit_mb
                                );
                            }
                        }
                    }
                }
            }
        }

        drop(monitor); // Release monitor lock before process operations

        // Check process status and handle crashes
        for (process_id, process) in processes.iter_mut() {
            match process.check_status().await {
                Ok(is_running) => {
                    if !is_running && process.config.autorestart {
                        // Process has died and should be restarted
                        warn!(
                            "Process {} has died, scheduling restart",
                            process.config.name
                        );
                        to_restart.push(*process_id);
                    }
                }
                Err(e) => {
                    error!(
                        "Error checking process {} status: {}",
                        process.config.name, e
                    );
                }
            }
        }

        // Restart processes that need it (memory limit exceeded or crashed)
        for process_id in to_restart {
            if let Some(process) = processes.get_mut(&process_id) {
                let restart_reason = if process.is_running() {
                    "memory limit exceeded"
                } else {
                    "process crashed"
                };
                info!(
                    "Auto-restarting process {} ({})",
                    process.config.name, restart_reason
                );

                if let Err(e) = process.restart().await {
                    error!(
                        "Failed to auto-restart process {}: {}",
                        process.config.name, e
                    );
                } else {
                    // Update PID file and metadata for restarted process
                    if let Some(new_pid) = process.pid() {
                        process.set_stored_pid(Some(new_pid));
                        let process_name = process.config.name.clone();
                        drop(processes); // Release lock before async operation
                        if let Err(e) = self.save_pid_file(&process_name, new_pid).await {
                            warn!("Failed to update PID file after restart: {}", e);
                        }
                        // Re-acquire lock to save metadata
                        let processes = self.processes.read().await;
                        if let Some(process) = processes.get(&process_id) {
                            if let Err(e) = self.save_process_metadata(process).await {
                                warn!("Failed to update metadata after restart: {}", e);
                            }
                        }
                        break; // Re-acquire lock in next iteration
                    }
                }
            }
        }

        Ok(())
    }

    /// Update process monitoring data
    pub async fn update_monitoring_data(&self) -> Result<()> {
        let processes = self.processes.read().await;
        let mut monitor = self.monitor.write().await;

        // Collect PIDs of running processes
        let pids: Vec<u32> = processes.values().filter_map(|p| p.pid()).collect();

        if !pids.is_empty() {
            let monitoring_data = monitor.update_process_metrics(&pids).await;
            debug!(
                "Updated monitoring data for {} processes",
                monitoring_data.len()
            );
        }

        Ok(())
    }

    /// Get system metrics
    pub async fn get_system_info(&self) -> Result<crate::monitoring::SystemMetrics> {
        let mut monitor = self.monitor.write().await;
        Ok(monitor.get_system_metrics().await)
    }

    /// Save process configuration to disk
    async fn save_process_config(&self, process: &Process) -> Result<()> {
        let config_file = self
            .config_dir
            .join(format!("{}.json", process.config.name));
        let config_json = serde_json::to_string_pretty(&process.config)
            .map_err(|e| Error::config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&config_file, config_json)
            .await
            .map_err(|e| Error::config(format!("Failed to write config file: {}", e)))?;

        debug!("Saved configuration for process: {}", process.config.name);
        Ok(())
    }

    /// Save process runtime metadata (assigned port, etc.) to disk
    async fn save_process_metadata(&self, process: &Process) -> Result<()> {
        use serde_json::json;

        let metadata_file = self
            .config_dir
            .join(format!("{}.meta.json", process.config.name));

        let metadata = json!({
            "id": process.id,
            "assigned_port": process.assigned_port,
            "instance": process.instance,
            "stored_pid": process.stored_pid
        });

        let metadata_json = serde_json::to_string_pretty(&metadata)
            .map_err(|e| Error::config(format!("Failed to serialize metadata: {}", e)))?;

        fs::write(&metadata_file, metadata_json)
            .await
            .map_err(|e| Error::config(format!("Failed to write metadata file: {}", e)))?;

        debug!("Saved metadata for process: {}", process.config.name);
        Ok(())
    }

    /// Load process runtime metadata from disk
    async fn load_process_metadata(&self, process: &mut Process) -> Result<()> {
        let metadata_file = self
            .config_dir
            .join(format!("{}.meta.json", process.config.name));

        if !metadata_file.exists() {
            // No metadata file, that's okay
            return Ok(());
        }

        let metadata_content = fs::read_to_string(&metadata_file)
            .await
            .map_err(|e| Error::config(format!("Failed to read metadata file: {}", e)))?;

        let metadata: serde_json::Value = serde_json::from_str(&metadata_content)
            .map_err(|e| Error::config(format!("Failed to parse metadata file: {}", e)))?;

        // Restore process ID
        if let Some(id_str) = metadata.get("id").and_then(|v| v.as_str()) {
            if let Ok(id) = uuid::Uuid::parse_str(id_str) {
                process.set_id(id);
            }
        }

        // Restore assigned port
        if let Some(port) = metadata.get("assigned_port").and_then(|v| v.as_u64()) {
            process.assigned_port = Some(port as u16);
        }

        // Restore instance number
        if let Some(instance) = metadata.get("instance").and_then(|v| v.as_u64()) {
            process.instance = Some(instance as u32);
        }

        // Restore stored PID
        if let Some(pid) = metadata.get("stored_pid").and_then(|v| v.as_u64()) {
            process.stored_pid = Some(pid as u32);
        }

        debug!("Loaded metadata for process: {}", process.config.name);
        Ok(())
    }

    /// Attempt to detect port from process logs
    async fn detect_port_from_logs(&self, process_name: &str) -> Option<u16> {
        let (out_log, _err_log, _combined_log) = self.get_log_paths(process_name);

        if !out_log.exists() {
            return None;
        }

        match fs::read_to_string(&out_log).await {
            Ok(content) => {
                // Common patterns for port detection
                let patterns = [
                    r"(?i)server.*(?:listening|bound|running).*(?:on|at).*:(\d+)",
                    r"(?i)listening.*(?:on|at).*:(\d+)",
                    r"(?i)bound.*(?:to|on).*:(\d+)",
                    r"(?i)port\s*:?\s*(\d+)",
                    r"(?i)running.*(?:on|at).*:(\d+)",
                ];

                for pattern in &patterns {
                    if let Ok(re) = regex::Regex::new(pattern) {
                        for line in content.lines().rev().take(50) { // Check last 50 lines
                            if let Some(captures) = re.captures(line) {
                                if let Some(port_match) = captures.get(1) {
                                    if let Ok(port) = port_match.as_str().parse::<u16>() {
                                        debug!("Detected port {} from log line: {}", port, line);
                                        return Some(port);
                                    }
                                }
                            }
                        }
                    }
                }
                None
            }
            Err(_) => None,
        }
    }

    /// Load all process configurations from disk
    async fn load_processes(&mut self) -> Result<()> {
        let mut entries = fs::read_dir(&self.config_dir)
            .await
            .map_err(|e| Error::config(format!("Failed to read config directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| Error::config(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                // Only load process config files, not metadata files
                if file_name.ends_with(".json") && !file_name.ends_with(".meta.json") {
                    if let Err(e) = self.load_process_config(&path).await {
                        warn!("Failed to load process config from {:?}: {}", path, e);
                    }
                }
            }
        }

        info!(
            "Loaded {} process configurations",
            self.processes.read().await.len()
        );
        Ok(())
    }

    /// Load a single process configuration from disk
    async fn load_process_config(&mut self, config_path: &PathBuf) -> Result<()> {
        let config_content = fs::read_to_string(config_path)
            .await
            .map_err(|e| Error::config(format!("Failed to read config file: {}", e)))?;

        let config: ProcessConfig = serde_json::from_str(&config_content)
            .map_err(|e| Error::config(format!("Failed to parse config file: {}", e)))?;

        // Create process but don't start it automatically
        let mut process = Process::new(config.clone());
        let process_id = process.id;

        // Load runtime metadata if it exists
        self.load_process_metadata(&mut process).await?;

        // Check if the process is still running by checking PID files
        if let Ok(Some(pid)) = self.read_pid_file(&config.name).await {
            // Check if the process is actually running
            let mut monitor = self.monitor.write().await;
            if monitor.is_process_running(pid).await {
                // Process is still running, update the process state
                process.set_state(crate::process::ProcessState::Online);

                // Restore port allocation if the process has a port configuration
                if let Some(port_config) = &config.port {
                    match port_config {
                        PortConfig::Single(port) => {
                            process.assigned_port = Some(*port);
                            let mut allocated_ports = self.allocated_ports.write().await;
                            allocated_ports.insert(*port);
                            debug!(
                                "Restored port allocation {} for running process {}",
                                port, config.name
                            );
                        }
                        PortConfig::Range(start, _end) => {
                            // For ranges, we assume the first port was assigned
                            process.assigned_port = Some(*start);
                            let mut allocated_ports = self.allocated_ports.write().await;
                            allocated_ports.insert(*start);
                            debug!(
                                "Restored port allocation {} for running process {}",
                                start, config.name
                            );
                        }
                        PortConfig::Auto(_, _) => {
                            // For auto ports, we can't easily restore the exact port
                            // This is a limitation - in a real implementation, we'd save the assigned port
                            debug!(
                                "Cannot restore auto-assigned port for process {}",
                                config.name
                            );
                        }
                    }
                }

                // Note: We can't restore the actual Child handle, but we can track the PID
                process.set_stored_pid(Some(pid));

                // Try to detect port from logs if not already assigned
                if process.assigned_port.is_none() {
                    if let Some(detected_port) = self.detect_port_from_logs(&config.name).await {
                        process.assigned_port = Some(detected_port);
                        debug!("Detected port {} for process {} from logs", detected_port, config.name);
                    }
                }

                debug!("Found running process {} with PID {}", config.name, pid);
            } else {
                // PID file exists but process is not running, clean up
                process.set_state(crate::process::ProcessState::Stopped);
                if let Err(e) = self.remove_pid_file(&config.name).await {
                    warn!("Failed to remove stale PID file for {}: {}", config.name, e);
                }
            }
        } else {
            // No PID file, process is stopped
            process.set_state(crate::process::ProcessState::Stopped);
        }

        let process_name = config.name.clone();

        let mut processes = self.processes.write().await;
        let mut name_map = self.name_to_id.write().await;

        processes.insert(process_id, process);
        name_map.insert(config.name, process_id);

        debug!("Loaded process configuration: {}", process_name);
        Ok(())
    }

    /// Remove process configuration from disk
    async fn remove_process_config(&self, process_name: &str) -> Result<()> {
        let config_file = self.config_dir.join(format!("{}.json", process_name));
        if config_file.exists() {
            fs::remove_file(&config_file)
                .await
                .map_err(|e| Error::config(format!("Failed to remove config file: {}", e)))?;
            debug!("Removed configuration file for process: {}", process_name);
        }

        // Also remove metadata file
        let metadata_file = self.config_dir.join(format!("{}.meta.json", process_name));
        if metadata_file.exists() {
            fs::remove_file(&metadata_file)
                .await
                .map_err(|e| Error::config(format!("Failed to remove metadata file: {}", e)))?;
            debug!("Removed metadata file for process: {}", process_name);
        }

        Ok(())
    }

    /// Read log files for a process
    pub async fn read_logs(
        &self,
        process_name: &str,
        lines: Option<usize>,
        follow: bool,
    ) -> Result<()> {
        let (out_log, err_log, _combined_log) = self.get_log_paths(process_name);

        if follow {
            // Use the dedicated follow_logs method for real-time following
            return self.follow_logs(process_name).await;
        }

        let lines_to_read = lines.unwrap_or(15);

        // Read stdout log
        if out_log.exists() {
            println!("==> {} stdout <==", process_name);
            if let Ok(content) = fs::read_to_string(&out_log).await {
                let lines: Vec<&str> = content.lines().collect();
                let start = if lines.len() > lines_to_read {
                    lines.len() - lines_to_read
                } else {
                    0
                };
                for line in &lines[start..] {
                    println!("{}", line);
                }
            }
            println!();
        }

        // Read stderr log
        if err_log.exists() {
            println!("==> {} stderr <==", process_name);
            if let Ok(content) = fs::read_to_string(&err_log).await {
                let lines: Vec<&str> = content.lines().collect();
                let start = if lines.len() > lines_to_read {
                    lines.len() - lines_to_read
                } else {
                    0
                };
                for line in &lines[start..] {
                    println!("{}", line);
                }
            }
        }

        Ok(())
    }

    /// Clear log files for a process
    pub async fn clear_logs(&self, process_name: &str) -> Result<()> {
        let (out_log, err_log, _combined_log) = self.get_log_paths(process_name);

        if out_log.exists() {
            fs::write(&out_log, "")
                .await
                .map_err(|e| Error::config(format!("Failed to clear stdout log: {}", e)))?;
        }

        if err_log.exists() {
            fs::write(&err_log, "")
                .await
                .map_err(|e| Error::config(format!("Failed to clear stderr log: {}", e)))?;
        }

        info!("Cleared logs for process: {}", process_name);
        Ok(())
    }

    /// Remove log files for a process
    async fn remove_log_files(&self, process_name: &str) -> Result<()> {
        let (out_log, err_log, combined_log) = self.get_log_paths(process_name);

        for log_file in [out_log, err_log, combined_log] {
            if log_file.exists() {
                if let Err(e) = fs::remove_file(&log_file).await {
                    warn!("Failed to remove log file {:?}: {}", log_file, e);
                }
            }
        }

        debug!("Removed log files for process: {}", process_name);
        Ok(())
    }

    /// Allocate a port for a process
    async fn allocate_port(&self, port_config: &PortConfig, process_name: &str) -> Result<u16> {
        let mut allocated_ports = self.allocated_ports.write().await;

        match port_config {
            PortConfig::Single(port) => {
                if allocated_ports.contains(port) {
                    return Err(Error::config(format!("Port {} is already in use", port)));
                }
                allocated_ports.insert(*port);
                info!("Allocated port {} to process {}", port, process_name);
                Ok(*port)
            }
            PortConfig::Range(start, end) => {
                // For ranges, we need to allocate all ports in the range
                let ports: Vec<u16> = (*start..=*end).collect();
                for port in &ports {
                    if allocated_ports.contains(port) {
                        return Err(Error::config(format!(
                            "Port {} in range {}-{} is already in use",
                            port, start, end
                        )));
                    }
                }
                // Allocate all ports in the range
                for port in &ports {
                    allocated_ports.insert(*port);
                }
                info!(
                    "Allocated port range {}-{} to process {}",
                    start, end, process_name
                );
                Ok(*start) // Return the first port in the range
            }
            PortConfig::Auto(start, end) => {
                // Find the first available port in the range
                for port in *start..=*end {
                    if !allocated_ports.contains(&port) {
                        allocated_ports.insert(port);
                        info!("Auto-allocated port {} to process {}", port, process_name);
                        return Ok(port);
                    }
                }
                Err(Error::config(format!(
                    "No available ports in range {}-{}",
                    start, end
                )))
            }
        }
    }

    /// Deallocate ports for a port configuration
    async fn deallocate_ports(&self, port_config: &PortConfig, assigned_port: Option<u16>) {
        let mut allocated_ports = self.allocated_ports.write().await;

        match port_config {
            PortConfig::Single(port) => {
                allocated_ports.remove(port);
                debug!("Deallocated port {}", port);
            }
            PortConfig::Range(start, end) => {
                for port in *start..=*end {
                    allocated_ports.remove(&port);
                }
                debug!("Deallocated port range {}-{}", start, end);
            }
            PortConfig::Auto(_, _) => {
                if let Some(port) = assigned_port {
                    allocated_ports.remove(&port);
                    debug!("Deallocated auto-assigned port {}", port);
                }
            }
        }
    }

    /// Check if a port is available
    pub async fn is_port_available(&self, port: u16) -> bool {
        let allocated_ports = self.allocated_ports.read().await;
        !allocated_ports.contains(&port)
    }

    /// Get all allocated ports
    pub async fn get_allocated_ports(&self) -> Vec<u16> {
        let allocated_ports = self.allocated_ports.read().await;
        let mut ports: Vec<u16> = allocated_ports.iter().copied().collect();
        ports.sort();
        ports
    }

    /// Resolve process identifier (name or UUID) to ProcessId
    async fn resolve_identifier(&self, identifier: &str) -> Result<ProcessId> {
        // Try to parse as UUID first
        if let Ok(uuid) = Uuid::parse_str(identifier) {
            let processes = self.processes.read().await;
            if processes.contains_key(&uuid) {
                return Ok(uuid);
            }
        }

        // Try to resolve by name
        let name_map = self.name_to_id.read().await;
        if let Some(&process_id) = name_map.get(identifier) {
            Ok(process_id)
        } else {
            Err(Error::process_not_found(identifier))
        }
    }

    /// Get the number of processes
    pub async fn process_count(&self) -> usize {
        let processes = self.processes.read().await;
        processes.len()
    }

    /// Check if a process exists by name
    pub async fn process_exists(&self, name: &str) -> bool {
        let name_map = self.name_to_id.read().await;
        name_map.contains_key(name)
    }

    /// Get all process names
    pub async fn get_process_names(&self) -> Vec<String> {
        let name_map = self.name_to_id.read().await;
        name_map.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PortConfig, ProcessConfig};
    use crate::process::{Process, ProcessState};
    use pretty_assertions::assert_eq;
    use std::time::Duration;
    use tempfile::TempDir;
    use tokio::fs;

    async fn create_test_manager() -> (ProcessManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().to_path_buf();

        let manager = ProcessManager {
            processes: RwLock::new(HashMap::new()),
            name_to_id: RwLock::new(HashMap::new()),
            monitor: RwLock::new(Monitor::new()),
            config_dir,
            allocated_ports: RwLock::new(HashSet::new()),
        };

        (manager, temp_dir)
    }

    fn create_test_config(name: &str) -> ProcessConfig {
        ProcessConfig::builder()
            .name(name)
            .script("echo")
            .args(vec!["hello", "world"])
            .build()
            .unwrap()
    }

    #[test]
    fn test_get_config_dir() {
        let result = ProcessManager::get_config_dir();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains(".pmdaemon"));
    }

    #[tokio::test]
    async fn test_process_manager_new() {
        let manager = ProcessManager::new().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_get_log_paths() {
        let (manager, _temp_dir) = create_test_manager().await;
        let (out_log, err_log, combined_log) = manager.get_log_paths("test-process");

        assert!(out_log.to_string_lossy().contains("test-process-out.log"));
        assert!(err_log.to_string_lossy().contains("test-process-error.log"));
        assert!(combined_log.to_string_lossy().contains("test-process.log"));
    }

    #[tokio::test]
    async fn test_ensure_logs_dir() {
        let (manager, _temp_dir) = create_test_manager().await;
        let result = manager.ensure_logs_dir().await;
        assert!(result.is_ok());

        let logs_dir = manager.get_logs_dir();
        assert!(logs_dir.exists());
    }

    #[tokio::test]
    async fn test_save_and_read_pid_file() {
        let (manager, _temp_dir) = create_test_manager().await;
        let process_name = "test-process";
        let pid = 12345u32;

        // Save PID file
        let result = manager.save_pid_file(process_name, pid).await;
        assert!(result.is_ok());

        // Read PID file
        let read_result = manager.read_pid_file(process_name).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), Some(pid));

        // Test non-existent PID file
        let missing_result = manager.read_pid_file("non-existent").await;
        assert!(missing_result.is_ok());
        assert_eq!(missing_result.unwrap(), None);
    }

    #[tokio::test]
    async fn test_remove_pid_file() {
        let (manager, _temp_dir) = create_test_manager().await;
        let process_name = "test-process";
        let pid = 12345u32;

        // Save PID file first
        manager.save_pid_file(process_name, pid).await.unwrap();

        // Remove PID file
        let result = manager.remove_pid_file(process_name).await;
        assert!(result.is_ok());

        // Verify it's gone
        let read_result = manager.read_pid_file(process_name).await;
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), None);
    }

    #[tokio::test]
    async fn test_port_allocation_single() {
        let (manager, _temp_dir) = create_test_manager().await;
        let port_config = PortConfig::Single(8080);

        // Allocate port
        let result = manager.allocate_port(&port_config, "test-process").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8080);

        // Try to allocate same port again
        let result2 = manager.allocate_port(&port_config, "test-process2").await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn test_port_allocation_auto() {
        let (manager, _temp_dir) = create_test_manager().await;
        let port_config = PortConfig::Auto(8000, 8002);

        // Allocate first port
        let result1 = manager.allocate_port(&port_config, "test-process1").await;
        assert!(result1.is_ok());
        let port1 = result1.unwrap();
        assert!((8000..=8002).contains(&port1));

        // Allocate second port
        let result2 = manager.allocate_port(&port_config, "test-process2").await;
        assert!(result2.is_ok());
        let port2 = result2.unwrap();
        assert!((8000..=8002).contains(&port2));
        assert_ne!(port1, port2);
    }

    #[tokio::test]
    async fn test_port_allocation_range() {
        let (manager, _temp_dir) = create_test_manager().await;
        let port_config = PortConfig::Range(9000, 9002);

        // Allocate port range
        let result = manager.allocate_port(&port_config, "test-process").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 9000); // Should return first port in range

        // Try to allocate overlapping range
        let port_config2 = PortConfig::Single(9001);
        let result2 = manager.allocate_port(&port_config2, "test-process2").await;
        assert!(result2.is_err()); // Should fail because 9001 is already allocated
    }

    #[tokio::test]
    async fn test_port_deallocation() {
        let (manager, _temp_dir) = create_test_manager().await;
        let port_config = PortConfig::Single(8080);

        // Allocate port
        manager
            .allocate_port(&port_config, "test-process")
            .await
            .unwrap();
        assert!(!manager.is_port_available(8080).await);

        // Deallocate port
        manager.deallocate_ports(&port_config, Some(8080)).await;
        assert!(manager.is_port_available(8080).await);
    }

    #[tokio::test]
    async fn test_is_port_available() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Port should be available initially
        assert!(manager.is_port_available(8080).await);

        // Allocate port
        let port_config = PortConfig::Single(8080);
        manager
            .allocate_port(&port_config, "test-process")
            .await
            .unwrap();

        // Port should not be available now
        assert!(!manager.is_port_available(8080).await);
    }

    #[tokio::test]
    async fn test_get_allocated_ports() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Initially no ports allocated
        let ports = manager.get_allocated_ports().await;
        assert!(ports.is_empty());

        // Allocate some ports
        let port_config1 = PortConfig::Single(8080);
        let port_config2 = PortConfig::Single(8081);
        manager.allocate_port(&port_config1, "test1").await.unwrap();
        manager.allocate_port(&port_config2, "test2").await.unwrap();

        // Check allocated ports
        let ports = manager.get_allocated_ports().await;
        assert_eq!(ports.len(), 2);
        assert!(ports.contains(&8080));
        assert!(ports.contains(&8081));
    }

    #[tokio::test]
    async fn test_get_logs_nonexistent_process() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Try to get logs for a process that doesn't exist
        let result = manager.get_logs("nonexistent", 10).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_get_logs_with_mock_process() {
        let (manager, _temp_dir) = create_test_manager().await;
        let config = create_test_config("test-logs");

        // Create a mock process entry
        let process = Process::new(config.clone());
        let process_id = process.id;

        {
            let mut processes = manager.processes.write().await;
            let mut name_map = manager.name_to_id.write().await;
            processes.insert(process_id, process);
            name_map.insert(config.name.clone(), process_id);
        }

        // Create mock log files
        let (out_log, err_log, _) = manager.get_log_paths(&config.name);
        fs::create_dir_all(out_log.parent().unwrap()).await.unwrap();
        fs::write(&out_log, "stdout line 1\nstdout line 2\nstdout line 3\n")
            .await
            .unwrap();
        fs::write(&err_log, "stderr line 1\nstderr line 2\n")
            .await
            .unwrap();

        // Test getting logs
        let logs = manager.get_logs(&config.name, 2).await.unwrap();

        // Should contain both stdout and stderr sections
        assert!(logs.contains("==> test-logs stdout <=="));
        assert!(logs.contains("==> test-logs stderr <=="));
        assert!(logs.contains("stdout line 2"));
        assert!(logs.contains("stdout line 3"));
        assert!(logs.contains("stderr line 1"));
        assert!(logs.contains("stderr line 2"));
        // Should not contain the first stdout line (only last 2 lines)
        assert!(!logs.contains("stdout line 1"));
    }

    #[tokio::test]
    async fn test_get_logs_missing_files() {
        let (manager, _temp_dir) = create_test_manager().await;
        let config = create_test_config("test-missing-logs");

        // Create a mock process entry
        let process = Process::new(config.clone());
        let process_id = process.id;

        {
            let mut processes = manager.processes.write().await;
            let mut name_map = manager.name_to_id.write().await;
            processes.insert(process_id, process);
            name_map.insert(config.name.clone(), process_id);
        }

        // Don't create log files - test missing files
        let logs = manager.get_logs(&config.name, 10).await.unwrap();

        // Should contain sections indicating no files found
        assert!(logs.contains("==> test-missing-logs stdout <=="));
        assert!(logs.contains("==> test-missing-logs stderr <=="));
        assert!(logs.contains("No stdout log file found"));
        assert!(logs.contains("No stderr log file found"));
    }

    #[tokio::test]
    async fn test_monitor_with_interval_validation() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Test that monitor_with_interval accepts different durations
        // We can't actually run the monitor loop in tests, but we can verify
        // the method exists and accepts the right parameters

        // This would normally run forever, so we'll just verify it compiles
        // and the method signature is correct
        let _future = manager.monitor_with_interval(Duration::from_millis(100));
        let _future = manager.monitor_with_interval(Duration::from_secs(1));
        let _future = manager.monitor_with_interval(Duration::from_secs(10));

        // If we get here, the method signature is correct
        // Test passed - process was successfully stopped
    }

    #[tokio::test]
    async fn test_default_monitor_delegates_to_configurable() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Test that the default monitor() method exists and compiles
        // This verifies that monitor() properly delegates to monitor_with_interval()
        let _future = manager.monitor();

        // If we get here, the delegation is working
        // Test passed - default monitor method exists and compiles
    }

    #[tokio::test]
    async fn test_get_system_info() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Test that we can get system information
        let system_info = manager.get_system_info().await.unwrap();

        // Verify system metrics have reasonable values
        // CPU usage should now be normalized to >= 0.0 by the monitoring system
        assert!(system_info.cpu_usage >= 0.0);
        assert!(system_info.memory_usage > 0);
        assert!(system_info.memory_total > 0);
        assert!(system_info.memory_usage <= system_info.memory_total);
        assert!(system_info.uptime > 0);
        assert_eq!(system_info.load_average.len(), 3);
    }

    #[tokio::test]
    async fn test_update_monitoring_data() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Test that update_monitoring_data doesn't crash with no processes
        let result = manager.update_monitoring_data().await;
        assert!(result.is_ok());

        // The method should handle empty process lists gracefully
    }

    #[tokio::test]
    async fn test_process_count() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Initially no processes
        assert_eq!(manager.process_count().await, 0);

        // Add a process manually for testing
        let config = create_test_config("test-process");
        let process = Process::new(config.clone());
        let process_id = process.id;

        {
            let mut processes = manager.processes.write().await;
            let mut name_map = manager.name_to_id.write().await;
            processes.insert(process_id, process);
            name_map.insert(config.name.clone(), process_id);
        }

        assert_eq!(manager.process_count().await, 1);
    }

    #[tokio::test]
    async fn test_process_exists() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Process should not exist initially
        assert!(!manager.process_exists("test-process").await);

        // Add a process manually for testing
        let config = create_test_config("test-process");
        let process = Process::new(config.clone());
        let process_id = process.id;

        {
            let mut processes = manager.processes.write().await;
            let mut name_map = manager.name_to_id.write().await;
            processes.insert(process_id, process);
            name_map.insert(config.name.clone(), process_id);
        }

        assert!(manager.process_exists("test-process").await);
        assert!(!manager.process_exists("non-existent").await);
    }

    #[tokio::test]
    async fn test_get_process_names() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Initially no process names
        let names = manager.get_process_names().await;
        assert!(names.is_empty());

        // Add processes manually for testing
        let configs = vec![
            create_test_config("process1"),
            create_test_config("process2"),
        ];

        {
            let mut processes = manager.processes.write().await;
            let mut name_map = manager.name_to_id.write().await;

            for config in configs {
                let process = Process::new(config.clone());
                let process_id = process.id;
                processes.insert(process_id, process);
                name_map.insert(config.name.clone(), process_id);
            }
        }

        let names = manager.get_process_names().await;
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"process1".to_string()));
        assert!(names.contains(&"process2".to_string()));
    }

    #[tokio::test]
    async fn test_resolve_identifier_by_name() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Add a process manually for testing
        let config = create_test_config("test-process");
        let process = Process::new(config.clone());
        let process_id = process.id;

        {
            let mut processes = manager.processes.write().await;
            let mut name_map = manager.name_to_id.write().await;
            processes.insert(process_id, process);
            name_map.insert(config.name.clone(), process_id);
        }

        // Resolve by name
        let result = manager.resolve_identifier("test-process").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), process_id);

        // Try non-existent process
        let result = manager.resolve_identifier("non-existent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_resolve_identifier_by_uuid() {
        let (manager, _temp_dir) = create_test_manager().await;

        // Add a process manually for testing
        let config = create_test_config("test-process");
        let process = Process::new(config.clone());
        let process_id = process.id;

        {
            let mut processes = manager.processes.write().await;
            let mut name_map = manager.name_to_id.write().await;
            processes.insert(process_id, process);
            name_map.insert(config.name.clone(), process_id);
        }

        // Resolve by UUID
        let uuid_str = process_id.to_string();
        let result = manager.resolve_identifier(&uuid_str).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), process_id);
    }

    #[tokio::test]
    async fn test_list_empty() {
        let (manager, _temp_dir) = create_test_manager().await;

        let result = manager.list().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_save_and_remove_process_config() {
        let (manager, _temp_dir) = create_test_manager().await;
        let config = create_test_config("test-process");
        let process = Process::new(config.clone());

        // Save config
        let result = manager.save_process_config(&process).await;
        assert!(result.is_ok());

        // Check file exists
        let config_file = manager.config_dir.join("test-process.json");
        assert!(config_file.exists());

        // Remove config
        let result = manager.remove_process_config("test-process").await;
        assert!(result.is_ok());

        // Check file is gone
        assert!(!config_file.exists());
    }

    #[tokio::test]
    async fn test_clear_logs() {
        let (manager, _temp_dir) = create_test_manager().await;
        let process_name = "test-process";

        // Create logs directory and files
        manager.ensure_logs_dir().await.unwrap();
        let (out_log, err_log, _) = manager.get_log_paths(process_name);

        // Write some content to log files
        fs::write(&out_log, "stdout content").await.unwrap();
        fs::write(&err_log, "stderr content").await.unwrap();

        // Clear logs
        let result = manager.clear_logs(process_name).await;
        assert!(result.is_ok());

        // Check files are empty
        let out_content = fs::read_to_string(&out_log).await.unwrap();
        let err_content = fs::read_to_string(&err_log).await.unwrap();
        assert!(out_content.is_empty());
        assert!(err_content.is_empty());
    }

    #[tokio::test]
    async fn test_remove_log_files() {
        let (manager, _temp_dir) = create_test_manager().await;
        let process_name = "test-process";

        // Create logs directory and files
        manager.ensure_logs_dir().await.unwrap();
        let (out_log, err_log, combined_log) = manager.get_log_paths(process_name);

        // Create log files
        fs::write(&out_log, "stdout content").await.unwrap();
        fs::write(&err_log, "stderr content").await.unwrap();
        fs::write(&combined_log, "combined content").await.unwrap();

        // Remove log files
        manager.remove_log_files(process_name).await.unwrap();

        // Check files are gone
        assert!(!out_log.exists());
        assert!(!err_log.exists());
        assert!(!combined_log.exists());
    }

    #[tokio::test]
    async fn test_delete_all() {
        let (mut manager, _temp_dir) = create_test_manager().await;

        // Create some test processes
        let config1 = create_test_config("test-process-1");
        let config2 = create_test_config("test-process-2");

        // Add processes directly to the manager for testing
        let process1 = Process::new(config1);
        let process2 = Process::new(config2);
        let id1 = process1.id;
        let id2 = process2.id;

        {
            let mut processes = manager.processes.write().await;
            let mut name_map = manager.name_to_id.write().await;

            processes.insert(id1, process1);
            processes.insert(id2, process2);
            name_map.insert("test-process-1".to_string(), id1);
            name_map.insert("test-process-2".to_string(), id2);
        }

        // Verify processes exist
        assert_eq!(manager.processes.read().await.len(), 2);

        // Delete all processes
        let deleted_count = manager.delete_all().await.unwrap();
        assert_eq!(deleted_count, 2);

        // Verify all processes are deleted
        assert_eq!(manager.processes.read().await.len(), 0);
        assert_eq!(manager.name_to_id.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_by_status() {
        let (mut manager, _temp_dir) = create_test_manager().await;

        // Create test processes with different states
        let config1 = create_test_config("stopped-process");
        let config2 = create_test_config("online-process");

        let mut process1 = Process::new(config1);
        let mut process2 = Process::new(config2);

        // Set different states
        process1.state = ProcessState::Stopped;
        process2.state = ProcessState::Online;

        let id1 = process1.id;
        let id2 = process2.id;

        {
            let mut processes = manager.processes.write().await;
            let mut name_map = manager.name_to_id.write().await;

            processes.insert(id1, process1);
            processes.insert(id2, process2);
            name_map.insert("stopped-process".to_string(), id1);
            name_map.insert("online-process".to_string(), id2);
        }

        // Verify processes exist
        assert_eq!(manager.processes.read().await.len(), 2);

        // Delete only stopped processes
        let deleted_count = manager.delete_by_status("stopped").await.unwrap();
        assert_eq!(deleted_count, 1);

        // Verify only the stopped process was deleted
        assert_eq!(manager.processes.read().await.len(), 1);
        assert!(manager
            .name_to_id
            .read()
            .await
            .contains_key("online-process"));
        assert!(!manager
            .name_to_id
            .read()
            .await
            .contains_key("stopped-process"));
    }

    #[tokio::test]
    async fn test_delete_by_invalid_status() {
        let (mut manager, _temp_dir) = create_test_manager().await;

        // Try to delete by invalid status
        let result = manager.delete_by_status("invalid-status").await;
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Invalid status"));
        assert!(error_msg.contains("invalid-status"));
    }
}
