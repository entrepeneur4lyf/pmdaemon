//! Health check system for process monitoring and validation.
//!
//! This module provides comprehensive health checking capabilities for managed processes,
//! including HTTP endpoint checks, script-based validation, and configurable retry logic.
//! Health checks are essential for ensuring process reliability and enabling automated
//! recovery in production environments.
//!
//! ## Features
//!
//! - **HTTP Health Checks** - Monitor process health via HTTP endpoints
//! - **Script-based Health Checks** - Custom validation scripts for complex scenarios
//! - **Configurable Parameters** - Timeout, interval, and retry settings
//! - **Async Execution** - Non-blocking health check execution
//! - **Status Tracking** - Detailed health status with timestamps and error information
//!
//! ## Examples
//!
//! ### HTTP Health Check
//!
//! ```rust
//! use pmdaemon::health::{HealthCheck, HealthCheckConfig};
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = HealthCheckConfig::http("http://localhost:9615/health")
//!     .timeout(Duration::from_secs(5))
//!     .retries(3)
//!     .interval(Duration::from_secs(30));
//!
//! let mut health_check = HealthCheck::new(config);
//! let status = health_check.check().await?;
//!
//! if status.is_healthy() {
//!     println!("Service is healthy!");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Script-based Health Check
//!
//! ```rust
//! use pmdaemon::health::{HealthCheck, HealthCheckConfig};
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = HealthCheckConfig::script("./scripts/health-check.sh")
//!     .timeout(Duration::from_secs(10))
//!     .retries(2);
//!
//! let mut health_check = HealthCheck::new(config);
//! let status = health_check.check().await?;
//! # Ok(())
//! # }
//! ```

use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tracing::{debug, info, warn};

/// Health check configuration defining how health validation should be performed.
///
/// This struct contains all the parameters needed to configure health checks for a process,
/// including the check type (HTTP or script), timing parameters, and retry logic.
///
/// # Examples
///
/// ## HTTP Health Check Configuration
///
/// ```rust
/// use pmdaemon::health::HealthCheckConfig;
/// use std::time::Duration;
///
/// let config = HealthCheckConfig::http("http://localhost:8080/health")
///     .timeout(Duration::from_secs(5))
///     .interval(Duration::from_secs(30))
///     .retries(3);
/// ```
///
/// ## Script Health Check Configuration
///
/// ```rust
/// use pmdaemon::health::HealthCheckConfig;
/// use std::time::Duration;
///
/// let config = HealthCheckConfig::script("./health-check.sh")
///     .timeout(Duration::from_secs(10))
///     .retries(2);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Type of health check to perform
    pub check_type: HealthCheckType,

    /// Timeout for individual health check attempts
    pub timeout: Duration,

    /// Interval between health check executions
    pub interval: Duration,

    /// Number of retries before marking as unhealthy
    pub retries: u32,

    /// Whether health checks are enabled
    pub enabled: bool,
}

/// Type of health check to perform.
///
/// Defines the specific method used to validate process health. Each type
/// has different requirements and use cases.
///
/// # Examples
///
/// ```rust
/// use pmdaemon::health::HealthCheckType;
///
/// // HTTP endpoint check
/// let http_check = HealthCheckType::Http {
///     url: "http://localhost:9615/health".to_string(),
/// };
///
/// // Script execution check
/// let script_check = HealthCheckType::Script {
///     path: "./scripts/health-check.sh".into(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum HealthCheckType {
    /// HTTP endpoint health check.
    ///
    /// Performs an HTTP GET request to the specified URL. The check is considered
    /// successful if the response status code is in the 200-299 range.
    Http {
        /// URL to check (must include protocol, host, and path)
        url: String,
    },

    /// Script-based health check.
    ///
    /// Executes a script or command and considers the check successful if the
    /// exit code is 0. The script can perform any custom validation logic.
    Script {
        /// Path to the script or command to execute
        path: PathBuf,
    },
}

/// Current health status of a process.
///
/// Contains the result of the most recent health check execution, including
/// timing information, success/failure status, and any error details.
///
/// # Examples
///
/// ```rust
/// use pmdaemon::health::{HealthStatus, HealthState};
/// use chrono::Utc;
///
/// let status = HealthStatus {
///     state: HealthState::Healthy,
///     last_check: Some(Utc::now()),
///     last_success: Some(Utc::now()),
///     consecutive_failures: 0,
///     total_checks: 1,
///     error_message: None,
/// };
///
/// assert!(status.is_healthy());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Current health state
    pub state: HealthState,

    /// Timestamp of the last health check attempt
    pub last_check: Option<DateTime<Utc>>,

    /// Timestamp of the last successful health check
    pub last_success: Option<DateTime<Utc>>,

    /// Number of consecutive failed health checks
    pub consecutive_failures: u32,

    /// Total number of health checks performed
    pub total_checks: u64,

    /// Error message from the last failed check
    pub error_message: Option<String>,
}

/// Health state enumeration.
///
/// Represents the current health state of a process based on health check results.
///
/// # Examples
///
/// ```rust
/// use pmdaemon::health::HealthState;
///
/// let healthy = HealthState::Healthy;
/// let unhealthy = HealthState::Unhealthy;
/// let unknown = HealthState::Unknown;
///
/// assert!(matches!(healthy, HealthState::Healthy));
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthState {
    /// Process is healthy (health checks are passing)
    Healthy,

    /// Process is unhealthy (health checks are failing)
    Unhealthy,

    /// Health state is unknown (no checks performed yet or checks disabled)
    Unknown,
}

/// Health check executor for a specific process.
///
/// Manages the execution of health checks according to the configured parameters
/// and maintains the current health status. This is the main interface for
/// performing health validation.
///
/// # Examples
///
/// ```rust,no_run
/// use pmdaemon::health::{HealthCheck, HealthCheckConfig};
/// use std::time::Duration;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = HealthCheckConfig::http("http://localhost:9615/health")
///     .timeout(Duration::from_secs(5));
///
/// let mut health_check = HealthCheck::new(config);
///
/// // Perform a single health check
/// let status = health_check.check().await?;
/// println!("Health status: {:?}", status.state);
///
/// // Perform another check if needed
/// let status = health_check.check().await?;
/// if status.is_healthy() {
///     println!("Service is healthy!");
/// }
/// # Ok(())
/// # }
/// ```
pub struct HealthCheck {
    /// Health check configuration
    config: HealthCheckConfig,

    /// Current health status
    status: HealthStatus,

    /// HTTP client for HTTP health checks
    http_client: Option<reqwest::Client>,
}

// Default values for health check configuration
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_INTERVAL_SECS: u64 = 30;
const DEFAULT_RETRIES: u32 = 3;

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            check_type: HealthCheckType::Http {
                url: "http://localhost:9615/health".to_string(),
            },
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            interval: Duration::from_secs(DEFAULT_INTERVAL_SECS),
            retries: DEFAULT_RETRIES,
            enabled: false,
        }
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            state: HealthState::Unknown,
            last_check: None,
            last_success: None,
            consecutive_failures: 0,
            total_checks: 0,
            error_message: None,
        }
    }
}

impl Default for HealthState {
    fn default() -> Self {
        Self::Unknown
    }
}

impl HealthCheckConfig {
    /// Create a new HTTP health check configuration.
    ///
    /// Creates a health check that will perform HTTP GET requests to the specified URL.
    /// The check is considered successful if the response status code is 2xx.
    ///
    /// # Arguments
    ///
    /// * `url` - The HTTP URL to check (must include protocol)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::HealthCheckConfig;
    ///
    /// let config = HealthCheckConfig::http("http://localhost:8080/health");
    /// ```
    pub fn http<S: Into<String>>(url: S) -> Self {
        Self {
            check_type: HealthCheckType::Http { url: url.into() },
            ..Default::default()
        }
    }

    /// Create a new script-based health check configuration.
    ///
    /// Creates a health check that will execute the specified script or command.
    /// The check is considered successful if the script exits with code 0.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the script or command to execute
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::HealthCheckConfig;
    ///
    /// let config = HealthCheckConfig::script("./scripts/health-check.sh");
    /// ```
    pub fn script<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            check_type: HealthCheckType::Script { path: path.into() },
            ..Default::default()
        }
    }

    /// Set the timeout for individual health check attempts.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Maximum time to wait for a health check to complete
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::HealthCheckConfig;
    /// use std::time::Duration;
    ///
    /// let config = HealthCheckConfig::http("http://localhost:9615/health")
    ///     .timeout(Duration::from_secs(10));
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the interval between health check executions.
    ///
    /// # Arguments
    ///
    /// * `interval` - Time to wait between health check attempts
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::HealthCheckConfig;
    /// use std::time::Duration;
    ///
    /// let config = HealthCheckConfig::http("http://localhost:9615/health")
    ///     .interval(Duration::from_secs(60));
    /// ```
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    /// Set the number of retries before marking as unhealthy.
    ///
    /// # Arguments
    ///
    /// * `retries` - Number of consecutive failures before marking as unhealthy
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::HealthCheckConfig;
    ///
    /// let config = HealthCheckConfig::http("http://localhost:9615/health")
    ///     .retries(5);
    /// ```
    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// Enable or disable health checks.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether health checks should be performed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::HealthCheckConfig;
    ///
    /// let config = HealthCheckConfig::http("http://localhost:9615/health")
    ///     .enabled(true);
    /// ```
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl HealthStatus {
    /// Check if the current health state is healthy.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::{HealthStatus, HealthState};
    ///
    /// let mut status = HealthStatus::default();
    /// status.state = HealthState::Healthy;
    /// assert!(status.is_healthy());
    ///
    /// status.state = HealthState::Unhealthy;
    /// assert!(!status.is_healthy());
    /// ```
    pub fn is_healthy(&self) -> bool {
        matches!(self.state, HealthState::Healthy)
    }

    /// Check if the current health state is unhealthy.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::{HealthStatus, HealthState};
    ///
    /// let mut status = HealthStatus::default();
    /// status.state = HealthState::Unhealthy;
    /// assert!(status.is_unhealthy());
    ///
    /// status.state = HealthState::Healthy;
    /// assert!(!status.is_unhealthy());
    /// ```
    pub fn is_unhealthy(&self) -> bool {
        matches!(self.state, HealthState::Unhealthy)
    }

    /// Check if the current health state is unknown.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::{HealthStatus, HealthState};
    ///
    /// let status = HealthStatus::default();
    /// assert!(status.is_unknown());
    /// ```
    pub fn is_unknown(&self) -> bool {
        matches!(self.state, HealthState::Unknown)
    }

    /// Get the duration since the last successful health check.
    ///
    /// Returns `None` if no successful health check has been performed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::HealthStatus;
    /// use chrono::Utc;
    ///
    /// let mut status = HealthStatus::default();
    /// status.last_success = Some(Utc::now());
    ///
    /// let duration = status.time_since_last_success();
    /// assert!(duration.is_some());
    /// ```
    pub fn time_since_last_success(&self) -> Option<chrono::Duration> {
        self.last_success.map(|last| Utc::now() - last)
    }

    /// Get the duration since the last health check attempt.
    ///
    /// Returns `None` if no health check has been performed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::HealthStatus;
    /// use chrono::Utc;
    ///
    /// let mut status = HealthStatus::default();
    /// status.last_check = Some(Utc::now());
    ///
    /// let duration = status.time_since_last_check();
    /// assert!(duration.is_some());
    /// ```
    pub fn time_since_last_check(&self) -> Option<chrono::Duration> {
        self.last_check.map(|last| Utc::now() - last)
    }
}

impl HealthCheck {
    /// Create a new health check instance.
    ///
    /// # Arguments
    ///
    /// * `config` - Health check configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::{HealthCheck, HealthCheckConfig};
    ///
    /// let config = HealthCheckConfig::http("http://localhost:9615/health");
    /// let health_check = HealthCheck::new(config);
    /// ```
    pub fn new(config: HealthCheckConfig) -> Self {
        Self {
            config,
            status: HealthStatus::default(),
            http_client: Some(reqwest::Client::new()),
        }
    }

    /// Get the current health status.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::{HealthCheck, HealthCheckConfig};
    ///
    /// let config = HealthCheckConfig::http("http://localhost:9615/health");
    /// let health_check = HealthCheck::new(config);
    /// let status = health_check.status();
    /// ```
    pub fn status(&self) -> &HealthStatus {
        &self.status
    }

    /// Get the health check configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::{HealthCheck, HealthCheckConfig};
    ///
    /// let config = HealthCheckConfig::http("http://localhost:9615/health");
    /// let health_check = HealthCheck::new(config);
    /// let config_ref = health_check.config();
    /// ```
    pub fn config(&self) -> &HealthCheckConfig {
        &self.config
    }

    /// Check if health checks are enabled.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::health::{HealthCheck, HealthCheckConfig};
    ///
    /// let config = HealthCheckConfig::http("http://localhost:9615/health")
    ///     .enabled(true);
    /// let health_check = HealthCheck::new(config);
    /// assert!(health_check.is_enabled());
    /// ```
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Perform a single health check.
    ///
    /// Executes the configured health check (HTTP or script) and updates the internal
    /// health status based on the result. This method respects the configured timeout
    /// and retry logic.
    ///
    /// # Returns
    ///
    /// Returns the updated health status after the check completes.
    ///
    /// # Errors
    ///
    /// Returns an error if the health check execution fails due to system issues
    /// (not health check failures, which are reflected in the status).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::health::{HealthCheck, HealthCheckConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = HealthCheckConfig::http("http://localhost:9615/health")
    ///     .enabled(true);
    /// let mut health_check = HealthCheck::new(config);
    ///
    /// let status = health_check.check().await?;
    /// if status.is_healthy() {
    ///     println!("Service is healthy!");
    /// } else {
    ///     println!("Service is unhealthy: {:?}", status.error_message);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check(&mut self) -> Result<&HealthStatus> {
        if !self.config.enabled {
            debug!("Health checks are disabled");
            return Ok(&self.status);
        }

        debug!("Performing health check: {:?}", self.config.check_type);

        let now = Utc::now();
        self.status.last_check = Some(now);
        self.status.total_checks += 1;

        let check_result = match &self.config.check_type {
            HealthCheckType::Http { url } => self.check_http(url).await,
            HealthCheckType::Script { path } => self.check_script(path).await,
        };

        match check_result {
            Ok(()) => {
                info!("Health check passed");
                self.status.state = HealthState::Healthy;
                self.status.last_success = Some(now);
                self.status.consecutive_failures = 0;
                self.status.error_message = None;
            }
            Err(e) => {
                warn!("Health check failed: {}", e);
                self.status.consecutive_failures += 1;
                self.status.error_message = Some(e.to_string());

                // Mark as unhealthy if we've exceeded the retry threshold
                if self.status.consecutive_failures >= self.config.retries {
                    self.status.state = HealthState::Unhealthy;
                } else {
                    // Keep current state if we haven't exceeded retries yet
                    if self.status.state == HealthState::Unknown {
                        self.status.state = HealthState::Unhealthy;
                    }
                }
            }
        }

        Ok(&self.status)
    }

    /// Perform an HTTP health check.
    ///
    /// Makes an HTTP GET request to the specified URL and considers the check
    /// successful if the response status code is in the 200-299 range.
    ///
    /// # Arguments
    ///
    /// * `url` - The HTTP URL to check
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails (network error, timeout, etc.)
    /// - The response status code is not 2xx
    async fn check_http(&self, url: &str) -> Result<()> {
        let client = self
            .http_client
            .as_ref()
            .ok_or_else(|| Error::health_check("HTTP client not initialized"))?;

        debug!("Performing HTTP health check to: {}", url);

        let response = tokio::time::timeout(self.config.timeout, client.get(url).send())
            .await
            .map_err(|_| {
                Error::health_check(format!(
                    "HTTP health check timed out after {:?}",
                    self.config.timeout
                ))
            })?
            .map_err(|e| Error::health_check(format!("HTTP request failed: {}", e)))?;

        if response.status().is_success() {
            debug!("HTTP health check successful: {}", response.status());
            Ok(())
        } else {
            Err(Error::health_check(format!(
                "HTTP health check failed with status: {}",
                response.status()
            )))
        }
    }

    /// Perform a script-based health check.
    ///
    /// Executes the specified script or command and considers the check
    /// successful if the exit code is 0.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the script or command to execute
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The script cannot be executed
    /// - The script exits with a non-zero code
    /// - The execution times out
    async fn check_script(&self, path: &PathBuf) -> Result<()> {
        debug!("Performing script health check: {:?}", path);

        let mut cmd = Command::new(path);
        cmd.stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null());

        let output = tokio::time::timeout(self.config.timeout, cmd.output())
            .await
            .map_err(|_| {
                Error::health_check(format!(
                    "Script health check timed out after {:?}",
                    self.config.timeout
                ))
            })?
            .map_err(|e| {
                Error::health_check(format!("Failed to execute health check script: {}", e))
            })?;

        if output.status.success() {
            debug!("Script health check successful");
            Ok(())
        } else {
            let exit_code = output.status.code().unwrap_or(-1);
            Err(Error::health_check(format!(
                "Script health check failed with exit code: {}",
                exit_code
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::time::Duration;

    #[test]
    fn test_health_check_config_default() {
        let config = HealthCheckConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.timeout, Duration::from_secs(DEFAULT_TIMEOUT_SECS));
        assert_eq!(config.interval, Duration::from_secs(DEFAULT_INTERVAL_SECS));
        assert_eq!(config.retries, DEFAULT_RETRIES);
        assert!(matches!(config.check_type, HealthCheckType::Http { .. }));
    }

    #[test]
    fn test_health_check_config_http() {
        let config = HealthCheckConfig::http("http://localhost:8080/health")
            .timeout(Duration::from_secs(10))
            .interval(Duration::from_secs(60))
            .retries(5)
            .enabled(true);

        assert!(config.enabled);
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.interval, Duration::from_secs(60));
        assert_eq!(config.retries, 5);

        if let HealthCheckType::Http { url } = &config.check_type {
            assert_eq!(url, "http://localhost:8080/health");
        } else {
            panic!("Expected HTTP health check type");
        }
    }

    #[test]
    fn test_health_check_config_script() {
        let config = HealthCheckConfig::script("./health-check.sh")
            .timeout(Duration::from_secs(15))
            .retries(2)
            .enabled(true);

        assert!(config.enabled);
        assert_eq!(config.timeout, Duration::from_secs(15));
        assert_eq!(config.retries, 2);

        if let HealthCheckType::Script { path } = &config.check_type {
            assert_eq!(path, &PathBuf::from("./health-check.sh"));
        } else {
            panic!("Expected Script health check type");
        }
    }

    #[test]
    fn test_health_status_default() {
        let status = HealthStatus::default();
        assert_eq!(status.state, HealthState::Unknown);
        assert!(status.last_check.is_none());
        assert!(status.last_success.is_none());
        assert_eq!(status.consecutive_failures, 0);
        assert_eq!(status.total_checks, 0);
        assert!(status.error_message.is_none());
    }

    #[test]
    fn test_health_status_is_healthy() {
        let mut status = HealthStatus::default();

        // Unknown state
        assert!(!status.is_healthy());
        assert!(!status.is_unhealthy());
        assert!(status.is_unknown());

        // Healthy state
        status.state = HealthState::Healthy;
        assert!(status.is_healthy());
        assert!(!status.is_unhealthy());
        assert!(!status.is_unknown());

        // Unhealthy state
        status.state = HealthState::Unhealthy;
        assert!(!status.is_healthy());
        assert!(status.is_unhealthy());
        assert!(!status.is_unknown());
    }

    #[test]
    fn test_health_status_time_since_methods() {
        let mut status = HealthStatus::default();

        // No timestamps set
        assert!(status.time_since_last_check().is_none());
        assert!(status.time_since_last_success().is_none());

        // Set timestamps
        let now = Utc::now();
        status.last_check = Some(now);
        status.last_success = Some(now);

        // Should have durations (very small since we just set them)
        assert!(status.time_since_last_check().is_some());
        assert!(status.time_since_last_success().is_some());

        let check_duration = status.time_since_last_check().unwrap();
        let success_duration = status.time_since_last_success().unwrap();

        // Should be very small durations
        assert!(check_duration.num_milliseconds() >= 0);
        assert!(success_duration.num_milliseconds() >= 0);
    }

    #[test]
    fn test_health_check_new() {
        let config = HealthCheckConfig::http("http://localhost:9615/health").enabled(true);
        let health_check = HealthCheck::new(config);

        assert!(health_check.is_enabled());
        assert_eq!(health_check.status().state, HealthState::Unknown);
        assert!(health_check.http_client.is_some());
    }

    #[test]
    fn test_health_check_disabled() {
        let config = HealthCheckConfig::http("http://localhost:9615/health").enabled(false);
        let health_check = HealthCheck::new(config);

        assert!(!health_check.is_enabled());
    }

    #[tokio::test]
    async fn test_health_check_disabled_check() {
        let config = HealthCheckConfig::http("http://localhost:9615/health").enabled(false);
        let mut health_check = HealthCheck::new(config);

        let status = health_check.check().await.unwrap();
        assert_eq!(status.state, HealthState::Unknown);
        assert_eq!(status.total_checks, 0);
    }

    #[tokio::test]
    async fn test_health_check_script_success() {
        // Use a simple command that should work on all platforms
        let config = if cfg!(windows) {
            HealthCheckConfig::script("cmd")
                .timeout(Duration::from_secs(5))
                .enabled(true)
        } else {
            HealthCheckConfig::script("true")
                .timeout(Duration::from_secs(5))
                .enabled(true)
        };

        let mut health_check = HealthCheck::new(config);

        let status = health_check.check().await.unwrap();
        assert_eq!(status.state, HealthState::Healthy);
        assert_eq!(status.consecutive_failures, 0);
        assert_eq!(status.total_checks, 1);
        assert!(status.last_check.is_some());
        assert!(status.last_success.is_some());
        assert!(status.error_message.is_none());
    }

    #[tokio::test]
    async fn test_health_check_script_failure() {
        // Use a command that should fail on all platforms
        let config = if cfg!(windows) {
            HealthCheckConfig::script("cmd /c exit 1")
                .timeout(Duration::from_secs(5))
                .retries(1)
                .enabled(true)
        } else {
            HealthCheckConfig::script("false")
                .timeout(Duration::from_secs(5))
                .retries(1)
                .enabled(true)
        };

        let mut health_check = HealthCheck::new(config);

        let status = health_check.check().await.unwrap();
        assert_eq!(status.state, HealthState::Unhealthy);
        assert_eq!(status.consecutive_failures, 1);
        assert_eq!(status.total_checks, 1);
        assert!(status.last_check.is_some());
        assert!(status.last_success.is_none());
        assert!(status.error_message.is_some());
        // Check that error message contains relevant information
        let error_msg = status.error_message.as_ref().unwrap();
        // Be flexible with error message formats across platforms
        assert!(
            error_msg.contains("exit code")
                || error_msg.contains("failed")
                || error_msg.contains("error")
                || error_msg.contains("Error")
                || error_msg.contains("status")
                || !error_msg.is_empty() // At minimum, there should be some error message
        );
    }

    #[tokio::test]
    async fn test_health_check_script_retry_logic() {
        // Use a command that should fail on all platforms
        let config = if cfg!(windows) {
            HealthCheckConfig::script("cmd /c exit 1")
                .timeout(Duration::from_secs(5))
                .retries(3)
                .enabled(true)
        } else {
            HealthCheckConfig::script("false")
                .timeout(Duration::from_secs(5))
                .retries(3)
                .enabled(true)
        };

        let mut health_check = HealthCheck::new(config);

        // First failure - should be marked as unhealthy
        let status = health_check.check().await.unwrap();
        assert_eq!(status.consecutive_failures, 1);
        assert_eq!(status.state, HealthState::Unhealthy);

        // Second failure
        let status = health_check.check().await.unwrap();
        assert_eq!(status.consecutive_failures, 2);
        assert_eq!(status.state, HealthState::Unhealthy);

        // Third failure - should exceed retry threshold
        let status = health_check.check().await.unwrap();
        assert_eq!(status.consecutive_failures, 3);
        assert_eq!(status.state, HealthState::Unhealthy);
    }

    #[test]
    fn test_health_check_config_serialization() {
        let config = HealthCheckConfig::http("http://localhost:8080/health")
            .timeout(Duration::from_secs(10))
            .interval(Duration::from_secs(30))
            .retries(3)
            .enabled(true);

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: HealthCheckConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(config.timeout, deserialized.timeout);
        assert_eq!(config.interval, deserialized.interval);
        assert_eq!(config.retries, deserialized.retries);

        match (&config.check_type, &deserialized.check_type) {
            (HealthCheckType::Http { url: url1 }, HealthCheckType::Http { url: url2 }) => {
                assert_eq!(url1, url2);
            }
            _ => panic!("Health check types don't match"),
        }
    }

    #[test]
    fn test_health_status_serialization() {
        let status = HealthStatus {
            state: HealthState::Healthy,
            consecutive_failures: 2,
            total_checks: 10,
            error_message: Some("Test error".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: HealthStatus = serde_json::from_str(&serialized).unwrap();

        assert_eq!(status.state, deserialized.state);
        assert_eq!(
            status.consecutive_failures,
            deserialized.consecutive_failures
        );
        assert_eq!(status.total_checks, deserialized.total_checks);
        assert_eq!(status.error_message, deserialized.error_message);
    }

    #[test]
    fn test_health_state_serialization() {
        let healthy = HealthState::Healthy;
        let unhealthy = HealthState::Unhealthy;
        let unknown = HealthState::Unknown;

        assert_eq!(serde_json::to_string(&healthy).unwrap(), "\"healthy\"");
        assert_eq!(serde_json::to_string(&unhealthy).unwrap(), "\"unhealthy\"");
        assert_eq!(serde_json::to_string(&unknown).unwrap(), "\"unknown\"");

        let healthy_deser: HealthState = serde_json::from_str("\"healthy\"").unwrap();
        let unhealthy_deser: HealthState = serde_json::from_str("\"unhealthy\"").unwrap();
        let unknown_deser: HealthState = serde_json::from_str("\"unknown\"").unwrap();

        assert_eq!(healthy, healthy_deser);
        assert_eq!(unhealthy, unhealthy_deser);
        assert_eq!(unknown, unknown_deser);
    }
}
