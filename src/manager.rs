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

use crate::error::{Error, Result};
use crate::process::{Process, ProcessId, ProcessStatus};
use crate::config::{ProcessConfig, PortConfig};
use crate::monitoring::Monitor;
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
            fs::create_dir_all(&config_dir).await
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
    fn get_config_dir() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| Error::config("Could not determine home directory"))?;
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
            fs::create_dir_all(&logs_dir).await
                .map_err(|e| Error::config(format!("Failed to create logs directory: {}", e)))?;
        }
        Ok(())
    }

    /// Save PID file for a process
    async fn save_pid_file(&self, process_name: &str, pid: u32) -> Result<()> {
        let pid_dir = self.get_pid_dir();
        if !pid_dir.exists() {
            fs::create_dir_all(&pid_dir).await
                .map_err(|e| Error::config(format!("Failed to create PID directory: {}", e)))?;
        }

        let pid_file = pid_dir.join(format!("{}.pid", process_name));
        fs::write(&pid_file, pid.to_string()).await
            .map_err(|e| Error::config(format!("Failed to write PID file: {}", e)))?;

        debug!("Saved PID file for process {}: {}", process_name, pid);
        Ok(())
    }

    /// Remove PID file for a process
    async fn remove_pid_file(&self, process_name: &str) -> Result<()> {
        let pid_file = self.get_pid_dir().join(format!("{}.pid", process_name));
        if pid_file.exists() {
            fs::remove_file(&pid_file).await
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

        let pid_content = fs::read_to_string(&pid_file).await
            .map_err(|e| Error::config(format!("Failed to read PID file: {}", e)))?;

        let pid = pid_content.trim().parse::<u32>()
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
            let assigned_port = self.allocate_port(port_config, &process.config.name).await?;
            process.assigned_port = Some(assigned_port);

            // Add PORT environment variable
            process.config.env.insert("PORT".to_string(), assigned_port.to_string());
        }

        // Ensure logs directory exists
        self.ensure_logs_dir().await?;

        // Get log file paths
        let (out_log, err_log, _combined_log) = self.get_log_paths(&process.config.name);

        // Start the process with log redirection
        process.start_with_logs(Some(out_log), Some(err_log)).await?;

        // Save PID file if process started successfully
        if let Some(pid) = process.pid() {
            self.save_pid_file(&process.config.name, pid).await?;
        }

        // Save configuration to disk
        self.save_process_config(&process).await?;

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
            instance_config.env.insert("PM2_INSTANCE_ID".to_string(), i.to_string());
            instance_config.env.insert("NODE_APP_INSTANCE".to_string(), i.to_string());

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

        info!("Started cluster '{}' with {} instances", config.name, config.instances);
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
    pub async fn restart_with_port(&mut self, identifier: &str, port_override: Option<PortConfig>) -> Result<()> {
        let process_id = self.resolve_identifier(identifier).await?;

        let mut processes = self.processes.write().await;
        if let Some(process) = processes.get_mut(&process_id) {
            // Handle port deallocation and reallocation if there's an override
            if let Some(new_port_config) = port_override {
                // Deallocate current port if any
                if let Some(current_port_config) = &process.config.port {
                    self.deallocate_ports(current_port_config, process.assigned_port).await;
                }

                // Allocate new port
                let assigned_port = self.allocate_port(&new_port_config, &process.config.name).await?;
                process.assigned_port = Some(assigned_port);

                // Update environment variable
                process.config.env.insert("PORT".to_string(), assigned_port.to_string());

                info!("Restarting {} with new port: {}", process.config.name, assigned_port);
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
    pub async fn reload_with_port(&mut self, identifier: &str, port_override: Option<PortConfig>) -> Result<()> {
        // For now, reload is the same as restart with port override
        self.restart_with_port(identifier, port_override).await
    }

    /// Delete a process
    pub async fn delete(&mut self, identifier: &str) -> Result<()> {
        let process_id = self.resolve_identifier(identifier).await?;

        // Remove from maps
        let mut processes = self.processes.write().await;
        let mut name_map = self.name_to_id.write().await;

        if let Some(process) = processes.remove(&process_id) {
            let process_name = process.config.name.clone();
            let port_config = process.config.port.clone();
            let assigned_port = process.assigned_port;
            name_map.remove(&process_name);

            // Remove configuration, PID, and log files
            drop(processes); // Release lock before async operation
            drop(name_map);

            // Deallocate ports
            if let Some(port_config) = port_config {
                self.deallocate_ports(&port_config, assigned_port).await;
            }

            self.remove_process_config(&process_name).await?;
            self.remove_pid_file(&process_name).await?;
            self.remove_log_files(&process_name).await?;
        }

        Ok(())
    }

    /// List all processes
    pub async fn list(&self) -> Result<Vec<ProcessStatus>> {
        let processes = self.processes.read().await;
        Ok(processes.values().map(|p| p.status()).collect())
    }

    /// Monitor processes in real-time
    pub async fn monitor(&self) -> Result<()> {
        // TODO: Implement real-time monitoring
        println!("Monitoring not yet implemented");
        Ok(())
    }

    /// Get process logs
    pub async fn get_logs(&self, identifier: &str, lines: usize) -> Result<String> {
        let _process_id = self.resolve_identifier(identifier).await?;

        // TODO: Implement log retrieval
        Ok(format!("Logs for {} (last {} lines) - not yet implemented", identifier, lines))
    }

    /// Follow process logs
    pub async fn follow_logs(&self, identifier: &str) -> Result<()> {
        let _process_id = self.resolve_identifier(identifier).await?;

        // TODO: Implement log following
        println!("Log following for {} - not yet implemented", identifier);
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
                                debug!("Process {} memory usage: {}MB / {}MB",
                                       process_name, memory_mb, limit_mb);
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
                        warn!("Process {} has died, scheduling restart", process.config.name);
                        to_restart.push(*process_id);
                    }
                }
                Err(e) => {
                    error!("Error checking process {} status: {}", process.config.name, e);
                }
            }
        }

        // Restart processes that need it (memory limit exceeded or crashed)
        for process_id in to_restart {
            if let Some(process) = processes.get_mut(&process_id) {
                let restart_reason = if process.is_running() { "memory limit exceeded" } else { "process crashed" };
                info!("Auto-restarting process {} ({})", process.config.name, restart_reason);

                if let Err(e) = process.restart().await {
                    error!("Failed to auto-restart process {}: {}", process.config.name, e);
                } else {
                    // Update PID file for restarted process
                    if let Some(new_pid) = process.pid() {
                        let process_name = process.config.name.clone();
                        drop(processes); // Release lock before async operation
                        if let Err(e) = self.save_pid_file(&process_name, new_pid).await {
                            warn!("Failed to update PID file after restart: {}", e);
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
        let pids: Vec<u32> = processes
            .values()
            .filter_map(|p| p.pid())
            .collect();

        if !pids.is_empty() {
            let monitoring_data = monitor.update_process_metrics(&pids).await;
            debug!("Updated monitoring data for {} processes", monitoring_data.len());
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
        let config_file = self.config_dir.join(format!("{}.json", process.config.name));
        let config_json = serde_json::to_string_pretty(&process.config)
            .map_err(|e| Error::config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&config_file, config_json).await
            .map_err(|e| Error::config(format!("Failed to write config file: {}", e)))?;

        debug!("Saved configuration for process: {}", process.config.name);
        Ok(())
    }

    /// Load all process configurations from disk
    async fn load_processes(&mut self) -> Result<()> {
        let mut entries = fs::read_dir(&self.config_dir).await
            .map_err(|e| Error::config(format!("Failed to read config directory: {}", e)))?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| Error::config(format!("Failed to read directory entry: {}", e)))? {

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Err(e) = self.load_process_config(&path).await {
                    warn!("Failed to load process config from {:?}: {}", path, e);
                }
            }
        }

        info!("Loaded {} process configurations", self.processes.read().await.len());
        Ok(())
    }

    /// Load a single process configuration from disk
    async fn load_process_config(&mut self, config_path: &PathBuf) -> Result<()> {
        let config_content = fs::read_to_string(config_path).await
            .map_err(|e| Error::config(format!("Failed to read config file: {}", e)))?;

        let config: ProcessConfig = serde_json::from_str(&config_content)
            .map_err(|e| Error::config(format!("Failed to parse config file: {}", e)))?;

        // Create process but don't start it automatically
        let mut process = Process::new(config.clone());
        let process_id = process.id;

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
                            debug!("Restored port allocation {} for running process {}", port, config.name);
                        }
                        PortConfig::Range(start, _end) => {
                            // For ranges, we assume the first port was assigned
                            process.assigned_port = Some(*start);
                            let mut allocated_ports = self.allocated_ports.write().await;
                            allocated_ports.insert(*start);
                            debug!("Restored port allocation {} for running process {}", start, config.name);
                        }
                        PortConfig::Auto(_, _) => {
                            // For auto ports, we can't easily restore the exact port
                            // This is a limitation - in a real implementation, we'd save the assigned port
                            debug!("Cannot restore auto-assigned port for process {}", config.name);
                        }
                    }
                }

                // Note: We can't restore the actual Child handle, but we can track the PID
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
            fs::remove_file(&config_file).await
                .map_err(|e| Error::config(format!("Failed to remove config file: {}", e)))?;
            debug!("Removed configuration file for process: {}", process_name);
        }
        Ok(())
    }

    /// Read log files for a process
    pub async fn read_logs(&self, process_name: &str, lines: Option<usize>, follow: bool) -> Result<()> {
        let (out_log, err_log, _combined_log) = self.get_log_paths(process_name);

        if follow {
            // TODO: Implement log following (tail -f functionality)
            println!("Log following not yet implemented");
            return Ok(());
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
            fs::write(&out_log, "").await
                .map_err(|e| Error::config(format!("Failed to clear stdout log: {}", e)))?;
        }

        if err_log.exists() {
            fs::write(&err_log, "").await
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
                        return Err(Error::config(format!("Port {} in range {}-{} is already in use", port, start, end)));
                    }
                }
                // Allocate all ports in the range
                for port in &ports {
                    allocated_ports.insert(*port);
                }
                info!("Allocated port range {}-{} to process {}", start, end, process_name);
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
                Err(Error::config(format!("No available ports in range {}-{}", start, end)))
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
    use crate::config::{ProcessConfig, PortConfig};
    use pretty_assertions::assert_eq;
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
        assert!(path.to_string_lossy().contains(".pm2r"));
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
        assert!(port1 >= 8000 && port1 <= 8002);

        // Allocate second port
        let result2 = manager.allocate_port(&port_config, "test-process2").await;
        assert!(result2.is_ok());
        let port2 = result2.unwrap();
        assert!(port2 >= 8000 && port2 <= 8002);
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
        manager.allocate_port(&port_config, "test-process").await.unwrap();
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
        manager.allocate_port(&port_config, "test-process").await.unwrap();

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
}
