//! Process configuration types and builders.
//!
//! This module provides the core configuration types for defining how processes should be
//! started, monitored, and managed. It includes support for advanced features like port
//! management, clustering, memory limits, and environment variable injection.
//!
//! ## Key Types
//!
//! - [`ProcessConfig`] - Main configuration struct for a process
//! - [`ProcessConfigBuilder`] - Builder pattern for creating configurations
//! - [`PortConfig`] - Port allocation strategies (single, range, auto-assignment)
//! - [`ExecMode`] - Execution mode (fork vs cluster)
//!
//! ## Examples
//!
//! ### Basic Configuration
//!
//! ```rust
//! use pmdaemon::config::ProcessConfig;
//!
//! let config = ProcessConfig::builder()
//!     .name("my-app")
//!     .script("node")
//!     .args(vec!["server.js"])
//!     .build()
//!     .unwrap();
//! ```
//!
//! ### Advanced Configuration with Port Management
//!
//! ```rust
//! use pmdaemon::config::{ProcessConfig, PortConfig};
//!
//! let config = ProcessConfig::builder()
//!     .name("web-server")
//!     .script("node")
//!     .args(vec!["app.js"])
//!     .instances(4)
//!     .port(PortConfig::Range(3000, 3003))
//!     .env("NODE_ENV", "production")
//!     .max_memory_restart(512 * 1024 * 1024) // 512MB
//!     .build()
//!     .unwrap();
//! ```

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Parse memory string (e.g., "100M", "1G", "512K") to bytes
pub fn parse_memory_string(memory_str: &str) -> Result<u64> {
    let memory_str = memory_str.trim().to_uppercase();

    if memory_str.is_empty() {
        return Err(Error::config("Memory string cannot be empty"));
    }

    let (number_part, unit) = if memory_str.ends_with("K") || memory_str.ends_with("KB") {
        let number_str = memory_str.trim_end_matches("KB").trim_end_matches("K");
        (number_str, 1024u64)
    } else if memory_str.ends_with("M") || memory_str.ends_with("MB") {
        let number_str = memory_str.trim_end_matches("MB").trim_end_matches("M");
        (number_str, 1024u64 * 1024)
    } else if memory_str.ends_with("G") || memory_str.ends_with("GB") {
        let number_str = memory_str.trim_end_matches("GB").trim_end_matches("G");
        (number_str, 1024u64 * 1024 * 1024)
    } else if memory_str.ends_with("B") {
        let number_str = memory_str.trim_end_matches("B");
        (number_str, 1u64)
    } else {
        // Assume bytes if no unit specified
        (memory_str.as_str(), 1u64)
    };

let number: f64 = number_part.parse()
         .map_err(|_| Error::config(format!("Invalid memory number: {}", number_part)))?;

     if number < 0.0 {
         return Err(Error::config("Memory size cannot be negative"));
     }

    let bytes_f64 = number * unit as f64;
    if bytes_f64 > u64::MAX as f64 {
        return Err(Error::config("Memory size is too large"));
    }

    Ok(bytes_f64.round() as u64)
}

/// Format memory in human-readable format
pub fn format_memory(bytes: u64) -> String {
    if bytes == 0 {
        return "-".to_string();
    }

    let kb = bytes as f64 / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;

    if gb >= 1.0 {
        format!("{:.1}GB", gb)
    } else if mb >= 1.0 {
        format!("{:.1}MB", mb)
    } else if kb >= 1.0 {
        format!("{:.1}KB", kb)
    } else {
        format!("{}B", bytes)
    }
}

/// Wrapper for memory values that can be deserialized from either string or number
#[derive(Debug, Clone)]
pub struct MemoryValue(pub Option<u64>);

impl Serialize for MemoryValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0 {
            Some(bytes) => serializer.serialize_str(&format_memory(bytes)),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for MemoryValue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MemoryValueHelper {
            String(String),
            Number(u64),
            Null,
        }

        match MemoryValueHelper::deserialize(deserializer)? {
            MemoryValueHelper::String(s) => {
                let bytes = parse_memory_string(&s).map_err(serde::de::Error::custom)?;
                Ok(MemoryValue(Some(bytes)))
            }
            MemoryValueHelper::Number(n) => Ok(MemoryValue(Some(n))),
            MemoryValueHelper::Null => Ok(MemoryValue(None)),
        }
    }
}

/// Serde module for memory value serialization/deserialization
mod memory_value_serde {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(bytes) => serializer.serialize_str(&format_memory(*bytes)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum MemoryHelper {
            String(String),
            Number(u64),
            Null,
        }

        match Option::<MemoryHelper>::deserialize(deserializer)? {
            Some(MemoryHelper::String(s)) => {
                let bytes = parse_memory_string(&s).map_err(serde::de::Error::custom)?;
                Ok(Some(bytes))
            }
            Some(MemoryHelper::Number(n)) => Ok(Some(n)),
            Some(MemoryHelper::Null) | None => Ok(None),
        }
    }
}

/// Process configuration defining how a process should be started and managed.
///
/// This is the main configuration struct that defines all aspects of process execution,
/// monitoring, and lifecycle management. It supports advanced features like clustering,
/// port management, memory limits, and environment variable injection.
///
/// # Examples
///
/// ## Basic Configuration
///
/// ```rust
/// use pmdaemon::config::ProcessConfig;
///
/// let config = ProcessConfig::builder()
///     .name("my-app")
///     .script("node")
///     .args(vec!["server.js"])
///     .build()
///     .unwrap();
/// ```
///
/// ## Cluster Configuration with Port Range
///
/// ```rust
/// use pmdaemon::config::{ProcessConfig, PortConfig};
///
/// let config = ProcessConfig::builder()
///     .name("web-cluster")
///     .script("node")
///     .args(vec!["app.js"])
///     .instances(4)
///     .port(PortConfig::Range(3000, 3003))
///     .max_memory_restart(512 * 1024 * 1024) // 512MB
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProcessConfig {
    /// Process name (required) - must be unique within a namespace
    pub name: String,

    /// Script or command to execute (required) - path to executable or command name
    pub script: String,

    /// Command line arguments passed to the script
    pub args: Vec<String>,

    /// Working directory for process execution (defaults to current directory)
    pub cwd: Option<PathBuf>,

    /// Environment variables injected into the process
    ///
    /// Note: PMDaemon automatically adds PORT, PM2_INSTANCE_ID, and NODE_APP_INSTANCE
    /// variables for clustering and port management.
    pub env: HashMap<String, String>,

    /// Number of instances to run (default: 1)
    ///
    /// When > 1, enables cluster mode with automatic load balancing.
    /// Each instance gets a unique PM2_INSTANCE_ID and NODE_APP_INSTANCE.
    pub instances: u32,

    /// Execution mode (fork or cluster)
    ///
    /// Automatically set to Cluster when instances > 1.
    pub exec_mode: ExecMode,

    /// Auto restart on crash (default: true)
    ///
    /// When enabled, processes are automatically restarted when they exit unexpectedly.
    pub autorestart: bool,

    /// Maximum number of restart attempts (default: 16)
    ///
    /// After this many restarts, auto-restart is disabled for the process.
    pub max_restarts: u32,

    /// Minimum uptime before considering stable (ms, default: 1000)
    ///
    /// Process must run for this duration before restart counter is reset.
    pub min_uptime: u64,

    /// Restart delay (ms, default: 0)
    ///
    /// Delay between process exit and restart attempt.
    pub restart_delay: u64,

    /// Kill timeout (ms, default: 1600)
    ///
    /// Time to wait for graceful shutdown (SIGTERM) before force kill (SIGKILL).
    pub kill_timeout: u64,

    /// Maximum memory before restart (bytes, optional)
    ///
    /// When set, process is automatically restarted if memory usage exceeds this limit.
    /// This is a unique feature beyond standard PM2 capabilities.
    /// Can be specified as a string (e.g., "512M", "1G") or as raw bytes.
    #[serde(with = "memory_value_serde")]
    pub max_memory_restart: Option<u64>,

    /// Output log file path (auto-generated if not specified)
    pub out_file: Option<PathBuf>,

    /// Error log file path (auto-generated if not specified)
    pub error_file: Option<PathBuf>,

    /// Combined log file path (auto-generated if not specified)
    pub log_file: Option<PathBuf>,

    /// PID file path (auto-generated if not specified)
    pub pid_file: Option<PathBuf>,

    /// Watch for file changes and restart (not yet implemented)
    pub watch: bool,

    /// Files/directories to ignore when watching (not yet implemented)
    pub ignore_watch: Vec<String>,

    /// User to run the process as (not yet implemented)
    pub user: Option<String>,

    /// Group to run the process as (not yet implemented)
    pub group: Option<String>,

    /// Namespace for process grouping (default: "default")
    ///
    /// Allows logical grouping of processes for management operations.
    pub namespace: String,

    /// Port configuration for the process
    ///
    /// Supports single ports, port ranges, and auto-assignment.
    /// This is an innovative feature that provides advanced port management
    /// capabilities beyond standard PM2.
    pub port: Option<PortConfig>,

    /// Health check configuration for the process
    ///
    /// Enables monitoring of process health through HTTP endpoints or custom scripts.
    /// Health checks can trigger automatic restarts when processes become unhealthy.
    pub health_check: Option<crate::health::HealthCheckConfig>,
}

/// Port configuration strategies for process management.
///
/// PMDaemon provides advanced port management capabilities that go beyond standard PM2.
/// This enum defines different strategies for port allocation and assignment.
///
/// # Examples
///
/// ```rust
/// use pmdaemon::config::PortConfig;
///
/// // Single port assignment
/// let single = PortConfig::Single(3000);
///
/// // Port range for cluster distribution
/// let range = PortConfig::Range(4000, 4003); // Ports 4000, 4001, 4002, 4003
///
/// // Auto-assignment from available ports in range
/// let auto = PortConfig::Auto(5000, 5100); // Find first available port 5000-5100
///
/// // Parse from string (useful for CLI)
/// let parsed = PortConfig::parse("auto:6000-6010").unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortConfig {
    /// Single port assignment.
    ///
    /// Assigns a specific port to the process. For cluster mode with multiple instances,
    /// only the first instance gets this port, others get no port assignment.
    Single(u16),

    /// Port range distribution (start, end) - inclusive.
    ///
    /// For cluster mode, distributes consecutive ports from the range to each instance.
    /// For example, Range(3000, 3003) with 4 instances assigns ports 3000, 3001, 3002, 3003.
    /// Fails if there aren't enough ports in the range for all instances.
    Range(u16, u16),

    /// Auto-assignment from range (start, end) - inclusive.
    ///
    /// Automatically finds the first available port(s) within the specified range.
    /// For cluster mode, finds consecutive available ports for all instances.
    /// This prevents port conflicts and is ideal for dynamic environments.
    Auto(u16, u16),
}

impl PortConfig {
    /// Parse port configuration from string representation.
    ///
    /// Supports multiple formats for flexible CLI and configuration usage:
    /// - `"3000"` → `PortConfig::Single(3000)`
    /// - `"3000-3010"` → `PortConfig::Range(3000, 3010)`
    /// - `"auto:3000-3100"` → `PortConfig::Auto(3000, 3100)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::config::PortConfig;
    ///
    /// let single = PortConfig::parse("8080").unwrap();
    /// let range = PortConfig::parse("4000-4003").unwrap();
    /// let auto = PortConfig::parse("auto:5000-5100").unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Port numbers are invalid (not u16)
    /// - Range format is incorrect
    /// - Start port is greater than end port
    pub fn parse(port_str: &str) -> Result<Self> {
        let port_str = port_str.trim();

        if port_str.starts_with("auto:") {
            let range_str = port_str.strip_prefix("auto:").unwrap();
            let (start, end) = Self::parse_range(range_str)?;
            Ok(PortConfig::Auto(start, end))
        } else if port_str.contains('-') {
            let (start, end) = Self::parse_range(port_str)?;
            Ok(PortConfig::Range(start, end))
        } else {
            let port: u16 = port_str.parse()
                .map_err(|_| Error::config(format!("Invalid port number: {}", port_str)))?;
            Ok(PortConfig::Single(port))
        }
    }

    fn parse_range(range_str: &str) -> Result<(u16, u16)> {
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            return Err(Error::config(format!("Invalid port range format: {}", range_str)));
        }

        let start: u16 = parts[0].trim().parse()
            .map_err(|_| Error::config(format!("Invalid start port: {}", parts[0])))?;
        let end: u16 = parts[1].trim().parse()
            .map_err(|_| Error::config(format!("Invalid end port: {}", parts[1])))?;

        if start > end {
            return Err(Error::config("Start port must be less than or equal to end port"));
        }

        Ok((start, end))
    }

    /// Get all possible ports from this configuration.
    ///
    /// Returns a vector of all ports that could be assigned based on this configuration.
    /// For Single, returns a single-element vector. For Range and Auto, returns all
    /// ports in the range (inclusive).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::config::PortConfig;
    ///
    /// assert_eq!(PortConfig::Single(8080).get_ports(), vec![8080]);
    /// assert_eq!(PortConfig::Range(3000, 3002).get_ports(), vec![3000, 3001, 3002]);
    /// assert_eq!(PortConfig::Auto(4000, 4001).get_ports(), vec![4000, 4001]);
    /// ```
    pub fn get_ports(&self) -> Vec<u16> {
        match self {
            PortConfig::Single(port) => vec![*port],
            PortConfig::Range(start, end) => (*start..=*end).collect(),
            PortConfig::Auto(start, end) => (*start..=*end).collect(),
        }
    }

    /// Check if this configuration uses auto-assignment.
    ///
    /// Returns `true` only for `PortConfig::Auto` variants, which automatically
    /// find available ports to prevent conflicts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::config::PortConfig;
    ///
    /// assert!(!PortConfig::Single(8080).is_auto());
    /// assert!(!PortConfig::Range(3000, 3005).is_auto());
    /// assert!(PortConfig::Auto(4000, 4010).is_auto());
    /// ```
    pub fn is_auto(&self) -> bool {
        matches!(self, PortConfig::Auto(_, _))
    }
}

impl std::fmt::Display for PortConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortConfig::Single(port) => write!(f, "{}", port),
            PortConfig::Range(start, end) => write!(f, "{}-{}", start, end),
            PortConfig::Auto(start, end) => write!(f, "auto:{}-{}", start, end),
        }
    }
}

impl Serialize for PortConfig {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PortConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PortConfig::parse(&s).map_err(serde::de::Error::custom)
    }
}

/// Execution mode for processes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExecMode {
    /// Fork mode - single process
    Fork,
    /// Cluster mode - multiple processes with load balancing
    Cluster,
}

impl Default for ExecMode {
    fn default() -> Self {
        Self::Fork
    }
}

impl Default for ProcessConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            script: String::new(),
            args: Vec::new(),
            cwd: None,
            env: HashMap::new(),
            instances: 1,
            exec_mode: ExecMode::Fork,
            autorestart: true,
            max_restarts: crate::DEFAULT_MAX_RESTARTS,
            min_uptime: crate::DEFAULT_MIN_UPTIME,
            restart_delay: crate::DEFAULT_RESTART_DELAY,
            kill_timeout: crate::DEFAULT_KILL_TIMEOUT,
            max_memory_restart: None,
            out_file: None,
            error_file: None,
            log_file: None,
            pid_file: None,
            watch: false,
            ignore_watch: Vec::new(),
            user: None,
            group: None,
            namespace: "default".to_string(),
            port: None,
            health_check: None,
        }
    }
}

/// Builder for ProcessConfig
#[derive(Debug, Default)]
pub struct ProcessConfigBuilder {
    config: ProcessConfig,
}

impl ProcessConfigBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the process name
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.config.name = name.into();
        self
    }

    /// Set the script/command to execute
    pub fn script<S: Into<String>>(mut self, script: S) -> Self {
        self.config.script = script.into();
        self
    }

    /// Set command line arguments
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.config.args = args.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set working directory
    pub fn cwd<P: Into<PathBuf>>(mut self, cwd: P) -> Self {
        self.config.cwd = Some(cwd.into());
        self
    }

    /// Set environment variables
    pub fn env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.config.env.insert(key.into(), value.into());
        self
    }

    /// Set number of instances
    pub fn instances(mut self, instances: u32) -> Self {
        self.config.instances = instances;
        if instances > 1 {
            self.config.exec_mode = ExecMode::Cluster;
        }
        self
    }

    /// Set maximum memory before restart (in bytes)
    pub fn max_memory_restart(mut self, max_memory: u64) -> Self {
        self.config.max_memory_restart = Some(max_memory);
        self
    }

    /// Set port configuration
    pub fn port(mut self, port_config: PortConfig) -> Self {
        self.config.port = Some(port_config);
        self
    }

    /// Set health check configuration
    pub fn health_check(mut self, health_check_config: crate::health::HealthCheckConfig) -> Self {
        self.config.health_check = Some(health_check_config);
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<ProcessConfig> {
        if self.config.name.is_empty() {
            return Err(Error::config("Process name is required"));
        }
        if self.config.script.is_empty() {
            return Err(Error::config("Script/command is required"));
        }
        Ok(self.config)
    }
}

impl ProcessConfig {
    /// Create a new builder
    pub fn builder() -> ProcessConfigBuilder {
        ProcessConfigBuilder::new()
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::config("Process name cannot be empty"));
        }
        if self.script.is_empty() {
            return Err(Error::config("Script/command cannot be empty"));
        }
        if self.instances == 0 {
            return Err(Error::config("Number of instances must be greater than 0"));
        }
        Ok(())
    }

    /// Get the effective working directory
    pub fn effective_cwd(&self) -> PathBuf {
        self.cwd.clone().unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        })
    }

    /// Check if this configuration uses clustering
    pub fn is_cluster_mode(&self) -> bool {
        self.instances > 1 || self.exec_mode == ExecMode::Cluster
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn test_port_config_parse_single() {
        let port = PortConfig::parse("8080").unwrap();
        assert_eq!(port, PortConfig::Single(8080));
    }

    #[test]
    fn test_port_config_parse_range() {
        let port = PortConfig::parse("3000-3005").unwrap();
        assert_eq!(port, PortConfig::Range(3000, 3005));
    }

    #[test]
    fn test_port_config_parse_auto() {
        let port = PortConfig::parse("auto:4000-4010").unwrap();
        assert_eq!(port, PortConfig::Auto(4000, 4010));
    }

    #[test]
    fn test_port_config_parse_invalid() {
        assert!(PortConfig::parse("invalid").is_err());
        assert!(PortConfig::parse("3000-").is_err());
        assert!(PortConfig::parse("-3000").is_err());
        assert!(PortConfig::parse("3000-2000").is_err());
    }

    #[test]
    fn test_port_config_get_ports() {
        let single = PortConfig::Single(8080);
        assert_eq!(single.get_ports(), vec![8080]);

        let range = PortConfig::Range(3000, 3002);
        assert_eq!(range.get_ports(), vec![3000, 3001, 3002]);

        let auto = PortConfig::Auto(4000, 4001);
        assert_eq!(auto.get_ports(), vec![4000, 4001]);
    }

    #[test]
    fn test_port_config_is_auto() {
        assert!(!PortConfig::Single(8080).is_auto());
        assert!(!PortConfig::Range(3000, 3005).is_auto());
        assert!(PortConfig::Auto(4000, 4010).is_auto());
    }

    #[test]
    fn test_port_config_display() {
        assert_eq!(PortConfig::Single(8080).to_string(), "8080");
        assert_eq!(PortConfig::Range(3000, 3005).to_string(), "3000-3005");
        assert_eq!(PortConfig::Auto(4000, 4010).to_string(), "auto:4000-4010");
    }

    #[test]
    fn test_exec_mode_default() {
        assert_eq!(ExecMode::default(), ExecMode::Fork);
    }

    #[test]
    fn test_process_config_default() {
        let config = ProcessConfig::default();
        assert_eq!(config.name, "");
        assert_eq!(config.script, "");
        assert_eq!(config.instances, 1);
        assert_eq!(config.exec_mode, ExecMode::Fork);
        assert!(config.autorestart);
        assert_eq!(config.namespace, "default");
        assert!(config.port.is_none());
    }

    #[test]
    fn test_process_config_builder_basic() {
        let config = ProcessConfig::builder()
            .name("test-app")
            .script("/usr/bin/node")
            .build()
            .unwrap();

        assert_eq!(config.name, "test-app");
        assert_eq!(config.script, "/usr/bin/node");
    }

    #[test]
    fn test_process_config_builder_with_args() {
        let config = ProcessConfig::builder()
            .name("test-app")
            .script("node")
            .args(vec!["app.js", "--port", "3000"])
            .build()
            .unwrap();

        assert_eq!(config.args, vec!["app.js", "--port", "3000"]);
    }

    #[test]
    fn test_process_config_builder_with_env() {
        let config = ProcessConfig::builder()
            .name("test-app")
            .script("node")
            .env("NODE_ENV", "production")
            .env("PORT", "3000")
            .build()
            .unwrap();

        let mut expected_env = HashMap::new();
        expected_env.insert("NODE_ENV".to_string(), "production".to_string());
        expected_env.insert("PORT".to_string(), "3000".to_string());
        assert_eq!(config.env, expected_env);
    }

    #[test]
    fn test_process_config_builder_with_instances() {
        let config = ProcessConfig::builder()
            .name("test-app")
            .script("node")
            .instances(4)
            .build()
            .unwrap();

        assert_eq!(config.instances, 4);
        assert_eq!(config.exec_mode, ExecMode::Cluster);
    }

    #[test]
    fn test_process_config_builder_with_port() {
        let config = ProcessConfig::builder()
            .name("test-app")
            .script("node")
            .port(PortConfig::Single(8080))
            .build()
            .unwrap();

        assert_eq!(config.port, Some(PortConfig::Single(8080)));
    }

    #[test]
    fn test_process_config_builder_validation_empty_name() {
        let result = ProcessConfig::builder()
            .script("node")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name is required"));
    }

    #[test]
    fn test_process_config_builder_validation_empty_script() {
        let result = ProcessConfig::builder()
            .name("test-app")
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Script/command is required"));
    }

    #[test]
    fn test_process_config_validate() {
        let mut config = ProcessConfig::default();

        // Empty name should fail
        assert!(config.validate().is_err());

        config.name = "test".to_string();
        // Empty script should fail
        assert!(config.validate().is_err());

        config.script = "node".to_string();
        // Valid config should pass
        assert!(config.validate().is_ok());

        // Zero instances should fail
        config.instances = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_process_config_effective_cwd() {
        let mut config = ProcessConfig::default();

        // Should return current directory when cwd is None
        let cwd = config.effective_cwd();
        assert!(!cwd.as_os_str().is_empty());

        // Should return specified directory when cwd is Some
        config.cwd = Some(PathBuf::from("/tmp"));
        assert_eq!(config.effective_cwd(), PathBuf::from("/tmp"));
    }

    #[test]
    fn test_process_config_is_cluster_mode() {
        let mut config = ProcessConfig::default();

        // Single instance fork mode should not be cluster
        assert!(!config.is_cluster_mode());

        // Multiple instances should be cluster
        config.instances = 4;
        assert!(config.is_cluster_mode());

        // Explicit cluster mode should be cluster
        config.instances = 1;
        config.exec_mode = ExecMode::Cluster;
        assert!(config.is_cluster_mode());
    }
}

/// Ecosystem configuration for loading multiple apps from a config file.
///
/// This structure represents the top-level configuration file format that can contain
/// multiple application definitions, similar to PM2's ecosystem.config.js.
///
/// # Examples
///
/// ```rust
/// use pmdaemon::config::{EcosystemConfig, ProcessConfig};
///
/// let ecosystem = EcosystemConfig {
///     apps: vec![
///         ProcessConfig::builder()
///             .name("web-app")
///             .script("node")
///             .args(vec!["server.js"])
///             .build()
///             .unwrap(),
///         ProcessConfig::builder()
///             .name("api-service")
///             .script("python")
///             .args(vec!["-m", "uvicorn", "main:app"])
///             .build()
///             .unwrap(),
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemConfig {
    /// List of application configurations
    pub apps: Vec<ProcessConfig>,
}

impl EcosystemConfig {
    /// Load ecosystem configuration from a file.
    ///
    /// Supports JSON, YAML, and TOML formats based on file extension.
    /// Falls back to JSON parsing if extension is not recognized.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::config::EcosystemConfig;
    /// use std::path::Path;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // Load from JSON
    /// let config = EcosystemConfig::from_file(Path::new("ecosystem.json")).await?;
    ///
    /// // Load from YAML
    /// let config = EcosystemConfig::from_file(Path::new("ecosystem.yaml")).await?;
    ///
    /// // Load from TOML
    /// let config = EcosystemConfig::from_file(Path::new("ecosystem.toml")).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File cannot be read
    /// - File content is not valid for the detected format
    /// - Configuration validation fails
    pub async fn from_file(path: &std::path::Path) -> Result<Self> {
       // Check file size before reading
       let metadata = tokio::fs::metadata(path)
           .await
           .map_err(|e| Error::config(format!("Failed to access config file '{}': {}", path.display(), e)))?;
       
       const MAX_CONFIG_SIZE: u64 = 10 * 1024 * 1024; // 10MB limit
       if metadata.len() > MAX_CONFIG_SIZE {
           return Err(Error::config(format!("Config file '{}' is too large ({}MB). Maximum allowed: {}MB", 
               path.display(), metadata.len() / 1024 / 1024, MAX_CONFIG_SIZE / 1024 / 1024)));
       }
       
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| Error::config(format!("Failed to read config file '{}': {}", path.display(), e)))?;

        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("json")
            .to_lowercase();

        let config: EcosystemConfig = match extension.as_str() {
            "yaml" | "yml" => {
                serde_yaml::from_str(&content)
                    .map_err(|e| Error::config(format!("Failed to parse YAML config file '{}': {}", path.display(), e)))?
            }
            "toml" => {
                toml::from_str(&content)
                    .map_err(|e| Error::config(format!("Failed to parse TOML config file '{}': {}", path.display(), e)))?
            }
            "json" | _ => {
                serde_json::from_str(&content)
                    .map_err(|e| Error::config(format!("Failed to parse JSON config file '{}': {}", path.display(), e)))?
            }
        };

        // Validate all app configurations
        config.validate()?;

        Ok(config)
    }

    /// Validate the ecosystem configuration.
    ///
    /// Checks that all app configurations are valid and that app names are unique.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any app configuration is invalid
    /// - App names are not unique
    /// - No apps are defined
    pub fn validate(&self) -> Result<()> {
        if self.apps.is_empty() {
            return Err(Error::config("Ecosystem configuration must contain at least one app"));
        }

        // Validate each app configuration
        for (index, app) in self.apps.iter().enumerate() {
            app.validate()
                .map_err(|e| Error::config(format!("App {} validation failed: {}", index, e)))?;
        }

        // Check for duplicate app names
        let mut names = std::collections::HashSet::new();
        for app in &self.apps {
            if !names.insert(&app.name) {
                return Err(Error::config(format!("Duplicate app name: '{}'", app.name)));
            }
        }

        Ok(())
    }

    /// Get a specific app configuration by name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::config::{EcosystemConfig, ProcessConfig};
    ///
    /// let ecosystem = EcosystemConfig {
    ///     apps: vec![
    ///         ProcessConfig::builder()
    ///             .name("web-app")
    ///             .script("node")
    ///             .build()
    ///             .unwrap(),
    ///     ],
    /// };
    ///
    /// let app = ecosystem.get_app("web-app");
    /// assert!(app.is_some());
    /// assert_eq!(app.unwrap().name, "web-app");
    /// ```
    pub fn get_app(&self, name: &str) -> Option<&ProcessConfig> {
        self.apps.iter().find(|app| app.name == name)
    }

    /// Get all app names in the ecosystem.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::config::{EcosystemConfig, ProcessConfig};
    ///
    /// let ecosystem = EcosystemConfig {
    ///     apps: vec![
    ///         ProcessConfig::builder()
    ///             .name("web-app")
    ///             .script("node")
    ///             .build()
    ///             .unwrap(),
    ///         ProcessConfig::builder()
    ///             .name("api-service")
    ///             .script("python")
    ///             .build()
    ///             .unwrap(),
    ///     ],
    /// };
    ///
    /// let names = ecosystem.app_names();
    /// assert_eq!(names, vec!["web-app", "api-service"]);
    /// ```
    pub fn app_names(&self) -> Vec<&str> {
        self.apps.iter().map(|app| app.name.as_str()).collect()
    }
}
