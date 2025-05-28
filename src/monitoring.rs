//! Process monitoring and system metrics

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sysinfo::{CpuExt, Pid, ProcessExt, System, SystemExt};
use tracing::{debug, warn};

/// System-wide monitoring data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Total system CPU usage percentage
    pub cpu_usage: f32,
    /// Total system memory usage in bytes
    pub memory_usage: u64,
    /// Total system memory available in bytes
    pub memory_total: u64,
    /// Memory usage percentage
    pub memory_percent: f32,
    /// Memory used (alias for memory_usage for compatibility)
    pub memory_used: u64,
    /// System load average (1, 5, 15 minutes)
    pub load_average: [f32; 3],
    /// System uptime in seconds
    pub uptime: u64,
    /// Timestamp of measurement
    pub timestamp: DateTime<Utc>,
}

/// Process-specific monitoring data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringData {
    /// Process CPU usage percentage
    pub cpu_usage: f32,
    /// Process memory usage in bytes
    pub memory_usage: u64,
    /// Process uptime in seconds
    pub uptime: Option<u64>,
    /// Number of file descriptors open
    pub open_files: Option<u32>,
    /// Timestamp of measurement
    pub timestamp: DateTime<Utc>,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            memory_total: 0,
            memory_percent: 0.0,
            memory_used: 0,
            load_average: [0.0, 0.0, 0.0],
            uptime: 0,
            timestamp: Utc::now(),
        }
    }
}

impl Default for MonitoringData {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            uptime: None,
            open_files: None,
            timestamp: Utc::now(),
        }
    }
}

/// Monitor for collecting system and process metrics
pub struct Monitor {
    /// System information collector
    system: System,
    /// Cache for process monitoring data
    process_cache: HashMap<u32, MonitoringData>,
}

impl Monitor {
    /// Create a new monitor
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            system,
            process_cache: HashMap::new(),
        }
    }

    /// Get current system metrics
    pub async fn get_system_metrics(&mut self) -> SystemMetrics {
        self.system.refresh_system();

        let load_avg = self.system.load_average();
        let memory_used = self.system.used_memory();
        let memory_total = self.system.total_memory();
        let memory_percent = if memory_total > 0 {
            (memory_used as f64 / memory_total as f64 * 100.0) as f32
        } else {
            0.0
        };

        // Handle edge cases where CPU usage might be negative or NaN (especially on macOS)
        let raw_cpu_usage = self.system.global_cpu_info().cpu_usage();
        let cpu_usage = if raw_cpu_usage.is_finite() && raw_cpu_usage >= 0.0 {
            raw_cpu_usage
        } else {
            0.0 // Default to 0.0 for invalid values
        };

        SystemMetrics {
            cpu_usage,
            memory_usage: memory_used,
            memory_total,
            memory_percent,
            memory_used,
            load_average: [
                load_avg.one as f32,
                load_avg.five as f32,
                load_avg.fifteen as f32,
            ],
            uptime: self.system.uptime(),
            timestamp: Utc::now(),
        }
    }

    /// Get monitoring data for a specific process
    pub async fn get_process_metrics(&mut self, pid: u32) -> MonitoringData {
        self.system.refresh_process(Pid::from(pid as usize));

        if let Some(process) = self.system.process(Pid::from(pid as usize)) {
            let uptime = process.run_time();

            // Handle edge cases where CPU usage might be negative or NaN
            let raw_cpu_usage = process.cpu_usage();
            let cpu_usage = if raw_cpu_usage.is_finite() && raw_cpu_usage >= 0.0 {
                raw_cpu_usage
            } else {
                0.0 // Default to 0.0 for invalid values
            };

            let monitoring_data = MonitoringData {
                cpu_usage,
                memory_usage: process.memory(),
                uptime: Some(uptime),
                open_files: None, // sysinfo doesn't provide this directly
                timestamp: Utc::now(),
            };

            // Cache the data
            self.process_cache.insert(pid, monitoring_data.clone());

            monitoring_data
        } else {
            warn!("Process with PID {} not found", pid);
            MonitoringData::default()
        }
    }

    /// Update monitoring data for multiple processes
    pub async fn update_process_metrics(&mut self, pids: &[u32]) -> HashMap<u32, MonitoringData> {
        let mut results = HashMap::new();

        // Refresh all processes at once for efficiency
        self.system.refresh_processes();

        for &pid in pids {
            if let Some(process) = self.system.process(Pid::from(pid as usize)) {
                let uptime = process.run_time();

                // Handle edge cases where CPU usage might be negative or NaN
                let raw_cpu_usage = process.cpu_usage();
                let cpu_usage = if raw_cpu_usage.is_finite() && raw_cpu_usage >= 0.0 {
                    raw_cpu_usage
                } else {
                    0.0 // Default to 0.0 for invalid values
                };

                let monitoring_data = MonitoringData {
                    cpu_usage,
                    memory_usage: process.memory(),
                    uptime: Some(uptime),
                    open_files: None,
                    timestamp: Utc::now(),
                };

                results.insert(pid, monitoring_data.clone());
                self.process_cache.insert(pid, monitoring_data);
            } else {
                debug!(
                    "Process with PID {} not found during monitoring update",
                    pid
                );
            }
        }

        results
    }

    /// Get cached monitoring data for a process
    pub fn get_cached_metrics(&self, pid: u32) -> Option<&MonitoringData> {
        self.process_cache.get(&pid)
    }

    /// Clear cache for a specific process
    pub fn clear_process_cache(&mut self, pid: u32) {
        self.process_cache.remove(&pid);
    }

    /// Check if a process is still running
    pub async fn is_process_running(&mut self, pid: u32) -> bool {
        self.system.refresh_process(Pid::from(pid as usize));
        self.system.process(Pid::from(pid as usize)).is_some()
    }
}

impl Default for Monitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::process;

    #[test]
    fn test_system_metrics_default() {
        let metrics = SystemMetrics::default();
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage, 0);
        assert_eq!(metrics.memory_total, 0);
        assert_eq!(metrics.load_average, [0.0, 0.0, 0.0]);
        assert_eq!(metrics.uptime, 0);
        assert!(metrics.timestamp <= Utc::now());
    }

    #[test]
    fn test_monitoring_data_default() {
        let data = MonitoringData::default();
        assert_eq!(data.cpu_usage, 0.0);
        assert_eq!(data.memory_usage, 0);
        assert!(data.uptime.is_none());
        assert!(data.open_files.is_none());
        assert!(data.timestamp <= Utc::now());
    }

    #[test]
    fn test_system_metrics_serialization() {
        let memory_usage = 1024 * 1024 * 1024; // 1GB
        let memory_total = 8 * 1024 * 1024 * 1024; // 8GB
        let memory_percent = (memory_usage as f64 / memory_total as f64 * 100.0) as f32;

        let metrics = SystemMetrics {
            cpu_usage: 25.5,
            memory_usage,
            memory_total,
            memory_percent,
            memory_used: memory_usage,
            load_average: [1.0, 1.5, 2.0],
            uptime: 3600, // 1 hour
            timestamp: Utc::now(),
        };

        let serialized = serde_json::to_string(&metrics).unwrap();
        let deserialized: SystemMetrics = serde_json::from_str(&serialized).unwrap();

        assert_eq!(metrics.cpu_usage, deserialized.cpu_usage);
        assert_eq!(metrics.memory_usage, deserialized.memory_usage);
        assert_eq!(metrics.memory_total, deserialized.memory_total);
        assert_eq!(metrics.load_average, deserialized.load_average);
        assert_eq!(metrics.uptime, deserialized.uptime);
    }

    #[test]
    fn test_monitoring_data_serialization() {
        let data = MonitoringData {
            cpu_usage: 15.2,
            memory_usage: 512 * 1024 * 1024, // 512MB
            uptime: Some(1800),              // 30 minutes
            open_files: Some(42),
            timestamp: Utc::now(),
        };

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: MonitoringData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(data.cpu_usage, deserialized.cpu_usage);
        assert_eq!(data.memory_usage, deserialized.memory_usage);
        assert_eq!(data.uptime, deserialized.uptime);
        assert_eq!(data.open_files, deserialized.open_files);
    }

    #[test]
    fn test_monitor_new() {
        let monitor = Monitor::new();
        assert!(monitor.process_cache.is_empty());
    }

    #[test]
    fn test_monitor_default() {
        let monitor = Monitor::default();
        assert!(monitor.process_cache.is_empty());
    }

    #[tokio::test]
    async fn test_get_system_metrics() {
        let mut monitor = Monitor::new();
        let metrics = monitor.get_system_metrics().await;

        // System metrics should have reasonable values
        assert!(metrics.cpu_usage >= 0.0);
        assert!(metrics.memory_usage > 0);
        assert!(metrics.memory_total > 0);
        assert!(metrics.memory_usage <= metrics.memory_total);
        assert!(metrics.uptime > 0);
        assert!(metrics.timestamp <= Utc::now());
    }

    #[tokio::test]
    async fn test_get_process_metrics_current_process() {
        let mut monitor = Monitor::new();
        let current_pid = process::id();

        let metrics = monitor.get_process_metrics(current_pid).await;

        // Current process should exist and have some metrics
        assert!(metrics.cpu_usage >= 0.0);
        assert!(metrics.memory_usage > 0);
        assert!(metrics.uptime.is_some());
        assert!(metrics.timestamp <= Utc::now());

        // Should be cached
        let cached = monitor.get_cached_metrics(current_pid);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().memory_usage, metrics.memory_usage);
    }

    #[tokio::test]
    async fn test_get_process_metrics_nonexistent() {
        let mut monitor = Monitor::new();
        let fake_pid = 999999u32; // Very unlikely to exist

        let metrics = monitor.get_process_metrics(fake_pid).await;

        // Should return default values for non-existent process
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage, 0);
        assert!(metrics.uptime.is_none());
    }

    #[tokio::test]
    async fn test_update_process_metrics() {
        let mut monitor = Monitor::new();
        let current_pid = process::id();
        let fake_pid = 999999u32;
        let pids = vec![current_pid, fake_pid];

        let results = monitor.update_process_metrics(&pids).await;

        // Should have metrics for current process but not fake process
        assert!(results.contains_key(&current_pid));
        assert!(!results.contains_key(&fake_pid));

        let current_metrics = results.get(&current_pid).unwrap();
        assert!(current_metrics.memory_usage > 0);
        assert!(current_metrics.uptime.is_some());
    }

    #[tokio::test]
    async fn test_is_process_running() {
        let mut monitor = Monitor::new();
        let current_pid = process::id();
        let fake_pid = 999999u32;

        // Current process should be running
        assert!(monitor.is_process_running(current_pid).await);

        // Fake process should not be running
        assert!(!monitor.is_process_running(fake_pid).await);
    }

    #[test]
    fn test_get_cached_metrics() {
        let mut monitor = Monitor::new();
        let pid = 12345u32;

        // Initially no cache
        assert!(monitor.get_cached_metrics(pid).is_none());

        // Add to cache
        let data = MonitoringData {
            cpu_usage: 25.0,
            memory_usage: 1024 * 1024,
            uptime: Some(3600),
            open_files: Some(10),
            timestamp: Utc::now(),
        };
        monitor.process_cache.insert(pid, data.clone());

        // Should be in cache now
        let cached = monitor.get_cached_metrics(pid);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().cpu_usage, 25.0);
        assert_eq!(cached.unwrap().memory_usage, 1024 * 1024);
    }

    #[test]
    fn test_clear_process_cache() {
        let mut monitor = Monitor::new();
        let pid = 12345u32;

        // Add to cache
        let data = MonitoringData::default();
        monitor.process_cache.insert(pid, data);
        assert!(monitor.get_cached_metrics(pid).is_some());

        // Clear cache
        monitor.clear_process_cache(pid);
        assert!(monitor.get_cached_metrics(pid).is_none());
    }

    #[test]
    fn test_monitoring_data_clone() {
        let original = MonitoringData {
            cpu_usage: 50.0,
            memory_usage: 2048,
            uptime: Some(7200),
            open_files: Some(20),
            timestamp: Utc::now(),
        };

        let cloned = original.clone();
        assert_eq!(original.cpu_usage, cloned.cpu_usage);
        assert_eq!(original.memory_usage, cloned.memory_usage);
        assert_eq!(original.uptime, cloned.uptime);
        assert_eq!(original.open_files, cloned.open_files);
    }

    #[test]
    fn test_system_metrics_clone() {
        let memory_usage = 4 * 1024 * 1024 * 1024;
        let memory_total = 16 * 1024 * 1024 * 1024;
        let memory_percent = (memory_usage as f64 / memory_total as f64 * 100.0) as f32;

        let original = SystemMetrics {
            cpu_usage: 75.0,
            memory_usage,
            memory_total,
            memory_percent,
            memory_used: memory_usage,
            load_average: [2.5, 2.0, 1.5],
            uptime: 86400,
            timestamp: Utc::now(),
        };

        let cloned = original.clone();
        assert_eq!(original.cpu_usage, cloned.cpu_usage);
        assert_eq!(original.memory_usage, cloned.memory_usage);
        assert_eq!(original.memory_total, cloned.memory_total);
        assert_eq!(original.load_average, cloned.load_average);
        assert_eq!(original.uptime, cloned.uptime);
    }

    #[test]
    fn test_monitoring_data_debug() {
        let data = MonitoringData {
            cpu_usage: 33.3,
            memory_usage: 1024,
            uptime: Some(1800),
            open_files: Some(5),
            timestamp: Utc::now(),
        };

        let debug_str = format!("{:?}", data);
        assert!(debug_str.contains("cpu_usage"));
        assert!(debug_str.contains("33.3"));
        assert!(debug_str.contains("memory_usage"));
        assert!(debug_str.contains("1024"));
    }

    #[test]
    fn test_system_metrics_debug() {
        let memory_usage = 2048;
        let memory_total = 8192;
        let memory_percent = (memory_usage as f64 / memory_total as f64 * 100.0) as f32;

        let metrics = SystemMetrics {
            cpu_usage: 45.6,
            memory_usage,
            memory_total,
            memory_percent,
            memory_used: memory_usage,
            load_average: [1.1, 1.2, 1.3],
            uptime: 3600,
            timestamp: Utc::now(),
        };

        let debug_str = format!("{:?}", metrics);
        assert!(debug_str.contains("cpu_usage"));
        assert!(debug_str.contains("45.6"));
        assert!(debug_str.contains("memory_total"));
        assert!(debug_str.contains("8192"));
    }
}
