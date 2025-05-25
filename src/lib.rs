//! # PMDaemon - Advanced Process Manager
//!
//! A feature-rich PM2 clone in Rust with advanced capabilities that exceed the original PM2.
//! PMDaemon is designed as a general-purpose process manager with innovative features for
//! modern application deployment and monitoring.
//!
//! ## Key Features
//!
//! ### Core Process Management
//! - **Process lifecycle management** - Start, stop, restart, reload, delete operations
//! - **Clustering support** - Run multiple instances with automatic load balancing
//! - **Auto-restart** - Automatic restart on crashes with configurable limits
//! - **Signal handling** - Graceful shutdown with SIGTERM/SIGINT and custom signals
//! - **Configuration persistence** - Process configs saved and restored between sessions
//!
//! ### Advanced Monitoring
//! - **Real-time monitoring** - CPU, memory, uptime tracking with system metrics
//! - **Memory limit enforcement** - Automatic restart when processes exceed memory limits
//! - **Process health checks** - Continuous monitoring with automatic failure detection
//! - **Log management** - Separate stdout/stderr files with viewing and following
//!
//! ### Innovative Port Management (Beyond PM2)
//! - **Port range distribution** - Automatically distribute consecutive ports to cluster instances
//! - **Auto-assignment from ranges** - Find first available port in specified range
//! - **Built-in conflict detection** - Prevent port conflicts at the process manager level
//! - **Runtime port overrides** - Change ports during restart without modifying saved config
//! - **Port visibility** - Display assigned ports in process listings
//!
//! ### Web API & Real-time Updates
//! - **Comprehensive REST API** - Full process management via HTTP with PM2-compatible responses
//! - **Real-time WebSocket updates** - Live process status and system metrics streaming
//! - **Production-ready web server** - Built on Axum with CORS and security headers
//!
//! ## Library Usage
//!
//! ### Basic Process Management
//!
//! ```rust,no_run
//! use pmdaemon::{ProcessManager, ProcessConfig};
//! use tokio;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut manager = ProcessManager::new().await?;
//!
//!     // Start a simple process
//!     let config = ProcessConfig::builder()
//!         .name("my-app")
//!         .script("node")
//!         .args(vec!["server.js"])
//!         .build()?;
//!
//!     let process_id = manager.start(config).await?;
//!     println!("Started process with ID: {}", process_id);
//!
//!     // List all processes
//!     let processes = manager.list().await?;
//!     for process in processes {
//!         println!("Process: {} ({})", process.name, process.state);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Advanced Features - Clustering with Port Management
//!
//! ```rust,no_run
//! use pmdaemon::{ProcessManager, ProcessConfig, config::PortConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut manager = ProcessManager::new().await?;
//!
//!     // Start a cluster with port range distribution
//!     let config = ProcessConfig::builder()
//!         .name("web-cluster")
//!         .script("node")
//!         .args(vec!["app.js"])
//!         .instances(4)
//!         .port(PortConfig::Range(3000, 3003)) // Ports 3000-3003
//!         .max_memory_restart(512 * 1024 * 1024) // 512MB limit
//!         .build()?;
//!
//!     manager.start(config).await?;
//!     println!("Started 4-instance cluster on ports 3000-3003");
//!
//!     Ok(())
//! }
//! ```
//!
//! ## CLI Usage
//!
//! ```bash
//! # Basic process management
//! pmdaemon start app.js --name myapp
//! pmdaemon stop myapp
//! pmdaemon restart myapp
//! pmdaemon delete myapp
//!
//! # Clustering with port management
//! pmdaemon start server.js --instances 4 --port 4000-4003
//! pmdaemon start worker.js --port auto:5000-5100
//!
//! # Runtime port override (without modifying saved config)
//! pmdaemon restart myapp --port 3001
//!
//! # Memory limits and monitoring
//! pmdaemon start app.js --max-memory 100M
//! pmdaemon list  # Shows ports, memory usage, CPU, etc.
//! pmdaemon monit # Real-time monitoring
//!
//! # Web API server for remote monitoring
//! pmdaemon web --port 9615 --host 127.0.0.1
//! ```

pub mod config;
pub mod error;
pub mod manager;
pub mod monitoring;
pub mod process;
pub mod signals;
pub mod web;

// Re-export main types for convenience
pub use config::{ProcessConfig, ProcessConfigBuilder};
pub use error::{Error, Result};
pub use manager::ProcessManager;
pub use monitoring::{MonitoringData, SystemMetrics};
pub use process::{Process, ProcessId, ProcessState, ProcessStatus};

/// Version of the PM2 Rust library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration directory name
pub const CONFIG_DIR: &str = ".pm2r";

/// Default log directory name
pub const LOG_DIR: &str = "logs";

/// Default PID directory name
pub const PID_DIR: &str = "pids";

/// Default web server port for monitoring API
pub const DEFAULT_WEB_PORT: u16 = 9615;

/// Default kill timeout in milliseconds
pub const DEFAULT_KILL_TIMEOUT: u64 = 1600;

/// Default restart delay in milliseconds
pub const DEFAULT_RESTART_DELAY: u64 = 0;

/// Maximum number of restart attempts
pub const DEFAULT_MAX_RESTARTS: u32 = 16;

/// Minimum uptime in milliseconds before considering a process stable
pub const DEFAULT_MIN_UPTIME: u64 = 1000;
