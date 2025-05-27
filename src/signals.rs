//! Signal handling for graceful process management.
//!
//! This module provides comprehensive signal handling capabilities for process management,
//! including sending signals to processes, setting up signal handlers for graceful shutdown,
//! and managing process termination with configurable timeouts.
//!
//! ## Features
//!
//! - **Signal Sending** - Send various signals (SIGTERM, SIGKILL, SIGINT, etc.) to processes
//! - **Graceful Shutdown** - Attempt graceful termination before force killing
//! - **Signal Handlers** - Setup handlers for daemon shutdown signals
//! - **Cross-platform Support** - Works on Unix-like systems with proper error handling
//! - **Timeout Management** - Configurable timeouts for graceful shutdown attempts
//!
//! ## Examples
//!
//! ### Basic Signal Handling
//!
//! ```rust
//! use pmdaemon::signals::{SignalHandler, ProcessSignal};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let handler = SignalHandler::new();
//!
//! // Send SIGTERM to a process
//! handler.send_signal(1234, ProcessSignal::Term)?;
//!
//! // Graceful shutdown with timeout
//! handler.graceful_shutdown(1234, 5000).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Setting up Signal Handlers
//!
//! ```rust,no_run
//! use pmdaemon::signals::SignalHandler;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let handler = SignalHandler::new();
//!
//! // Setup handlers for graceful daemon shutdown
//! handler.setup_handlers().await?;
//! # Ok(())
//! # }
//! ```

use crate::error::{Error, Result};
#[cfg(unix)]
use nix::sys::signal::{Signal, kill};
#[cfg(unix)]
use nix::unistd::Pid;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::signal;
use tracing::{debug, info, warn};

/// Signal handler for process management.
///
/// Provides functionality for sending signals to processes, setting up signal handlers
/// for graceful shutdown, and managing process termination with configurable timeouts.
///
/// # Examples
///
/// ```rust
/// use pmdaemon::signals::{SignalHandler, ProcessSignal};
///
/// let handler = SignalHandler::new();
/// ```
pub struct SignalHandler {
    /// Flag to indicate if shutdown has been requested
    shutdown_requested: Arc<AtomicBool>,
}

impl SignalHandler {
    /// Create a new signal handler.
    ///
    /// Initializes a new signal handler instance with default settings.
    /// The handler can be used to send signals to processes and manage
    /// graceful shutdown procedures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::signals::SignalHandler;
    ///
    /// let handler = SignalHandler::new();
    /// ```
    pub fn new() -> Self {
        Self {
            shutdown_requested: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Check if shutdown has been requested.
    ///
    /// Returns `true` if a shutdown signal (SIGTERM or SIGINT) has been received
    /// and the signal handler has been triggered.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::signals::SignalHandler;
    ///
    /// let handler = SignalHandler::new();
    /// if handler.is_shutdown_requested() {
    ///     println!("Shutdown requested, cleaning up...");
    /// }
    /// ```
    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::Relaxed)
    }

    /// Reset the shutdown flag.
    ///
    /// Clears the shutdown requested flag, allowing the handler to be reused.
    /// This is primarily useful for testing scenarios.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use pmdaemon::signals::SignalHandler;
    ///
    /// let handler = SignalHandler::new();
    /// handler.reset_shutdown_flag();
    /// assert!(!handler.is_shutdown_requested());
    /// ```
    pub fn reset_shutdown_flag(&self) {
        self.shutdown_requested.store(false, Ordering::Relaxed);
    }

    /// Send a signal to a process.
    ///
    /// Sends the specified signal to the process with the given PID. This method
    /// provides a safe wrapper around the underlying system signal functionality.
    ///
    /// # Arguments
    ///
    /// * `pid` - The process ID to send the signal to
    /// * `signal` - The type of signal to send
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the signal was sent successfully, or an error if the
    /// operation failed (e.g., process doesn't exist, permission denied).
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The process with the given PID doesn't exist
    /// - Permission is denied to send the signal
    /// - The signal type is invalid for the target process
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::signals::{SignalHandler, ProcessSignal};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let handler = SignalHandler::new();
    ///
    /// // Send SIGTERM to process 1234
    /// handler.send_signal(1234, ProcessSignal::Term)?;
    ///
    /// // Send SIGUSR1 to process 5678
    /// handler.send_signal(5678, ProcessSignal::Usr1)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(unix)]
    pub fn send_signal(&self, pid: u32, signal: ProcessSignal) -> Result<()> {
        let pid = Pid::from_raw(pid as i32);
        let signal = match signal {
            ProcessSignal::Term => Signal::SIGTERM,
            ProcessSignal::Kill => Signal::SIGKILL,
            ProcessSignal::Int => Signal::SIGINT,
            ProcessSignal::Quit => Signal::SIGQUIT,
            ProcessSignal::Usr1 => Signal::SIGUSR1,
            ProcessSignal::Usr2 => Signal::SIGUSR2,
        };

        debug!("Sending signal {} to PID {}", signal, pid);
        kill(pid, signal).map_err(|e| Error::signal(format!("Failed to send signal {} to PID {}: {}", signal, pid, e)))?;
        Ok(())
    }

    /// Send a signal to a process (Windows implementation).
    ///
    /// On Windows, this provides limited signal functionality using process termination.
    /// Only Term and Kill signals are supported and both result in process termination.
    #[cfg(windows)]
    pub fn send_signal(&self, pid: u32, signal: ProcessSignal) -> Result<()> {
        use std::process::Command;

        debug!("Sending signal {} to PID {} (Windows)", signal, pid);

        match signal {
            ProcessSignal::Term | ProcessSignal::Kill => {
                // Use taskkill for process termination on Windows
                let output = Command::new("taskkill")
                    .args(&["/PID", &pid.to_string(), "/F"])
                    .output()
                    .map_err(|e| Error::signal(format!("Failed to execute taskkill: {}", e)))?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(Error::signal(format!("Failed to kill process {}: {}", pid, stderr)));
                }
                Ok(())
            }
            _ => {
                // Other signals are not supported on Windows
                Err(Error::signal(format!("Signal {} is not supported on Windows", signal)))
            }
        }
    }

    /// Setup signal handlers for graceful shutdown.
    ///
    /// Configures signal handlers to catch SIGTERM and SIGINT signals, allowing
    /// the daemon to perform graceful shutdown when these signals are received.
    /// The handlers set an internal flag that can be checked with `is_shutdown_requested()`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if signal handlers were set up successfully, or an error
    /// if the setup failed.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The system doesn't support the required signals
    /// - Permission is denied to set up signal handlers
    /// - The signal handling subsystem is unavailable
    ///
    /// # Platform Support
    ///
    /// This method is available on Unix-like systems and Windows with different implementations.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::signals::SignalHandler;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let handler = SignalHandler::new();
    ///
    /// // Setup signal handlers
    /// handler.setup_handlers().await?;
    ///
    /// // Later, check if shutdown was requested
    /// if handler.is_shutdown_requested() {
    ///     println!("Graceful shutdown requested");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(unix)]
    pub async fn setup_handlers(&self) -> Result<()> {
        info!("Setting up signal handlers for graceful shutdown (Unix)");

        let shutdown_flag = Arc::clone(&self.shutdown_requested);

        // Setup SIGTERM handler
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .map_err(|e| Error::signal(format!("Failed to setup SIGTERM handler: {}", e)))?;

        // Setup SIGINT handler
        let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())
            .map_err(|e| Error::signal(format!("Failed to setup SIGINT handler: {}", e)))?;

        let shutdown_flag_term = Arc::clone(&shutdown_flag);
        let shutdown_flag_int = Arc::clone(&shutdown_flag);

        tokio::spawn(async move {
            tokio::select! {
                _ = sigterm.recv() => {
                    info!("Received SIGTERM, initiating graceful shutdown");
                    shutdown_flag_term.store(true, Ordering::Relaxed);
                }
                _ = sigint.recv() => {
                    info!("Received SIGINT, initiating graceful shutdown");
                    shutdown_flag_int.store(true, Ordering::Relaxed);
                }
            }
        });

        debug!("Signal handlers setup completed");
        Ok(())
    }

    /// Setup signal handlers for graceful shutdown (Windows implementation).
    ///
    /// On Windows, this sets up handlers for Ctrl+C (SIGINT equivalent).
    #[cfg(windows)]
    pub async fn setup_handlers(&self) -> Result<()> {
        info!("Setting up signal handlers for graceful shutdown (Windows)");

        let shutdown_flag = Arc::clone(&self.shutdown_requested);

        // Setup Ctrl+C handler
        let ctrl_c = signal::ctrl_c();

        tokio::spawn(async move {
            ctrl_c.await.ok();
            info!("Received Ctrl+C, initiating graceful shutdown");
            shutdown_flag.store(true, Ordering::Relaxed);
        });

        debug!("Signal handlers setup completed");
        Ok(())
    }

    /// Gracefully shutdown a process.
    ///
    /// Attempts to gracefully terminate a process by first sending SIGTERM and waiting
    /// for the process to exit. If the process doesn't exit within the specified timeout,
    /// it will be forcefully killed with SIGKILL.
    ///
    /// # Arguments
    ///
    /// * `pid` - The process ID to shutdown
    /// * `timeout_ms` - Maximum time to wait for graceful shutdown in milliseconds
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the process was successfully terminated (either gracefully
    /// or forcefully), or an error if the operation failed.
    ///
    /// # Errors
    ///
    /// This method will return an error if:
    /// - The process with the given PID doesn't exist
    /// - Permission is denied to send signals to the process
    /// - The signal sending operation fails
    ///
    /// # Behavior
    ///
    /// 1. Sends SIGTERM to the process
    /// 2. Polls every 100ms to check if the process has exited
    /// 3. If the process exits within the timeout, returns successfully
    /// 4. If the timeout is reached, sends SIGKILL to force termination
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pmdaemon::signals::SignalHandler;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let handler = SignalHandler::new();
    ///
    /// // Try graceful shutdown with 5 second timeout
    /// handler.graceful_shutdown(1234, 5000).await?;
    ///
    /// // Quick shutdown with 1 second timeout
    /// handler.graceful_shutdown(5678, 1000).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(unix)]
    pub async fn graceful_shutdown(&self, pid: u32, timeout_ms: u64) -> Result<()> {
        debug!("Initiating graceful shutdown for PID {} with timeout {}ms (Unix)", pid, timeout_ms);

        // Send SIGTERM first
        self.send_signal(pid, ProcessSignal::Term)?;

        // Wait for process to exit or timeout
        let timeout = tokio::time::Duration::from_millis(timeout_ms);
        let start = tokio::time::Instant::now();
        let poll_interval = tokio::time::Duration::from_millis(100);

        while start.elapsed() < timeout {
            // Check if process still exists
            match kill(Pid::from_raw(pid as i32), None) {
                Ok(_) => {
                    // Process still exists, wait a bit more
                    debug!("Process {} still running, waiting...", pid);
                    tokio::time::sleep(poll_interval).await;
                }
                Err(_) => {
                    // Process no longer exists
                    info!("Process {} exited gracefully", pid);
                    return Ok(());
                }
            }
        }

        // Timeout reached, send SIGKILL
        warn!("Process {} did not exit gracefully within {}ms, sending SIGKILL", pid, timeout_ms);
        self.send_signal(pid, ProcessSignal::Kill)?;

        // Wait a bit longer for SIGKILL to take effect and verify
        let kill_timeout = tokio::time::Duration::from_millis(500);
        let kill_start = tokio::time::Instant::now();

        while kill_start.elapsed() < kill_timeout {
            match kill(Pid::from_raw(pid as i32), None) {
                Ok(_) => {
                    // Process still exists
                    tokio::time::sleep(poll_interval).await;
                }
                Err(_) => {
                    // Process terminated
                    info!("Process {} forcefully terminated", pid);
                    return Ok(());
                }
            }
        }

        // If we get here, even SIGKILL didn't work
        Err(Error::signal(format!("Failed to kill process {} even with SIGKILL", pid)))
    }

    /// Gracefully shutdown a process (Windows implementation).
    ///
    /// On Windows, this uses taskkill to terminate the process.
    #[cfg(windows)]
    pub async fn graceful_shutdown(&self, pid: u32, timeout_ms: u64) -> Result<()> {
        debug!("Initiating graceful shutdown for PID {} with timeout {}ms (Windows)", pid, timeout_ms);

        // On Windows, we'll try a graceful termination first, then force kill
        // First attempt: try normal termination
        if let Ok(_) = self.send_signal(pid, ProcessSignal::Term) {
            // Wait for process to exit
            let timeout = tokio::time::Duration::from_millis(timeout_ms);
            let start = tokio::time::Instant::now();
            let poll_interval = tokio::time::Duration::from_millis(100);

            while start.elapsed() < timeout {
                // Check if process still exists using tasklist
                let output = std::process::Command::new("tasklist")
                    .args(&["/FI", &format!("PID eq {}", pid)])
                    .output();

                if let Ok(output) = output {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if !stdout.contains(&pid.to_string()) {
                        info!("Process {} exited gracefully", pid);
                        return Ok(());
                    }
                }

                tokio::time::sleep(poll_interval).await;
            }
        }

        // Force kill if graceful didn't work
        warn!("Process {} did not exit gracefully within {}ms, force killing", pid, timeout_ms);
        self.send_signal(pid, ProcessSignal::Kill)?;

        info!("Process {} forcefully terminated", pid);
        Ok(())
    }
}

/// Process signals that can be sent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessSignal {
    /// SIGTERM - Graceful termination
    Term,
    /// SIGKILL - Force kill
    Kill,
    /// SIGINT - Interrupt
    Int,
    /// SIGQUIT - Quit
    Quit,
    /// SIGUSR1 - User signal 1
    Usr1,
    /// SIGUSR2 - User signal 2
    Usr2,
}

impl Default for SignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ProcessSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessSignal::Term => write!(f, "SIGTERM"),
            ProcessSignal::Kill => write!(f, "SIGKILL"),
            ProcessSignal::Int => write!(f, "SIGINT"),
            ProcessSignal::Quit => write!(f, "SIGQUIT"),
            ProcessSignal::Usr1 => write!(f, "SIGUSR1"),
            ProcessSignal::Usr2 => write!(f, "SIGUSR2"),
        }
    }
}

impl FromStr for ProcessSignal {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TERM" | "SIGTERM" => Ok(ProcessSignal::Term),
            "KILL" | "SIGKILL" => Ok(ProcessSignal::Kill),
            "INT" | "SIGINT" => Ok(ProcessSignal::Int),
            "QUIT" | "SIGQUIT" => Ok(ProcessSignal::Quit),
            "USR1" | "SIGUSR1" => Ok(ProcessSignal::Usr1),
            "USR2" | "SIGUSR2" => Ok(ProcessSignal::Usr2),
            _ => Err(format!("Invalid signal: {}", s)),
        }
    }
}

impl ProcessSignal {
    /// Get all available signals
    pub fn all() -> Vec<ProcessSignal> {
        vec![
            ProcessSignal::Term,
            ProcessSignal::Kill,
            ProcessSignal::Int,
            ProcessSignal::Quit,
            ProcessSignal::Usr1,
            ProcessSignal::Usr2,
        ]
    }



    /// Check if this is a termination signal
    pub fn is_termination_signal(&self) -> bool {
        matches!(self, ProcessSignal::Term | ProcessSignal::Kill | ProcessSignal::Int | ProcessSignal::Quit)
    }

    /// Check if this is a user signal
    pub fn is_user_signal(&self) -> bool {
        matches!(self, ProcessSignal::Usr1 | ProcessSignal::Usr2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;



    #[test]
    fn test_signal_handler_new() {
        let handler = SignalHandler::new();
        assert!(!handler.is_shutdown_requested());
    }

    #[test]
    fn test_signal_handler_default() {
        let handler = SignalHandler::default();
        assert!(!handler.is_shutdown_requested());
    }

    #[test]
    fn test_signal_handler_shutdown_flag() {
        let handler = SignalHandler::new();

        // Initially should not be shutdown requested
        assert!(!handler.is_shutdown_requested());

        // Simulate shutdown request
        handler.shutdown_requested.store(true, Ordering::Relaxed);
        assert!(handler.is_shutdown_requested());

        // Reset flag
        handler.reset_shutdown_flag();
        assert!(!handler.is_shutdown_requested());
    }

    #[test]
    fn test_process_signal_display() {
        assert_eq!(ProcessSignal::Term.to_string(), "SIGTERM");
        assert_eq!(ProcessSignal::Kill.to_string(), "SIGKILL");
        assert_eq!(ProcessSignal::Int.to_string(), "SIGINT");
        assert_eq!(ProcessSignal::Quit.to_string(), "SIGQUIT");
        assert_eq!(ProcessSignal::Usr1.to_string(), "SIGUSR1");
        assert_eq!(ProcessSignal::Usr2.to_string(), "SIGUSR2");
    }

    #[test]
    fn test_process_signal_debug() {
        let signal = ProcessSignal::Term;
        let debug_str = format!("{:?}", signal);
        assert!(debug_str.contains("Term"));
    }

    #[test]
    fn test_process_signal_clone() {
        let original = ProcessSignal::Term;
        let cloned = original.clone();
        assert_eq!(format!("{:?}", original), format!("{:?}", cloned));
    }

    #[test]
    fn test_process_signal_copy() {
        let original = ProcessSignal::Term;
        let copied = original;
        assert_eq!(format!("{:?}", original), format!("{:?}", copied));
    }

    #[test]
    fn test_process_signal_all() {
        let signals = ProcessSignal::all();
        assert_eq!(signals.len(), 6);
        assert!(signals.contains(&ProcessSignal::Term));
        assert!(signals.contains(&ProcessSignal::Kill));
        assert!(signals.contains(&ProcessSignal::Int));
        assert!(signals.contains(&ProcessSignal::Quit));
        assert!(signals.contains(&ProcessSignal::Usr1));
        assert!(signals.contains(&ProcessSignal::Usr2));
    }

    #[test]
    fn test_process_signal_from_str() {
        assert_eq!("TERM".parse::<ProcessSignal>(), Ok(ProcessSignal::Term));
        assert_eq!("SIGTERM".parse::<ProcessSignal>(), Ok(ProcessSignal::Term));
        assert_eq!("term".parse::<ProcessSignal>(), Ok(ProcessSignal::Term));
        assert_eq!("sigterm".parse::<ProcessSignal>(), Ok(ProcessSignal::Term));

        assert_eq!("KILL".parse::<ProcessSignal>(), Ok(ProcessSignal::Kill));
        assert_eq!("SIGKILL".parse::<ProcessSignal>(), Ok(ProcessSignal::Kill));

        assert_eq!("INT".parse::<ProcessSignal>(), Ok(ProcessSignal::Int));
        assert_eq!("SIGINT".parse::<ProcessSignal>(), Ok(ProcessSignal::Int));

        assert_eq!("QUIT".parse::<ProcessSignal>(), Ok(ProcessSignal::Quit));
        assert_eq!("SIGQUIT".parse::<ProcessSignal>(), Ok(ProcessSignal::Quit));

        assert_eq!("USR1".parse::<ProcessSignal>(), Ok(ProcessSignal::Usr1));
        assert_eq!("SIGUSR1".parse::<ProcessSignal>(), Ok(ProcessSignal::Usr1));

        assert_eq!("USR2".parse::<ProcessSignal>(), Ok(ProcessSignal::Usr2));
        assert_eq!("SIGUSR2".parse::<ProcessSignal>(), Ok(ProcessSignal::Usr2));

        assert!("INVALID".parse::<ProcessSignal>().is_err());
        assert!("".parse::<ProcessSignal>().is_err());
    }

    #[test]
    fn test_process_signal_is_termination_signal() {
        assert!(ProcessSignal::Term.is_termination_signal());
        assert!(ProcessSignal::Kill.is_termination_signal());
        assert!(ProcessSignal::Int.is_termination_signal());
        assert!(ProcessSignal::Quit.is_termination_signal());
        assert!(!ProcessSignal::Usr1.is_termination_signal());
        assert!(!ProcessSignal::Usr2.is_termination_signal());
    }

    #[test]
    fn test_process_signal_is_user_signal() {
        assert!(!ProcessSignal::Term.is_user_signal());
        assert!(!ProcessSignal::Kill.is_user_signal());
        assert!(!ProcessSignal::Int.is_user_signal());
        assert!(!ProcessSignal::Quit.is_user_signal());
        assert!(ProcessSignal::Usr1.is_user_signal());
        assert!(ProcessSignal::Usr2.is_user_signal());
    }

    #[tokio::test]
    async fn test_send_signal_to_nonexistent_process() {
        let handler = SignalHandler::new();
        let fake_pid = 999999u32; // Very unlikely to exist

        let result = handler.send_signal(fake_pid, ProcessSignal::Term);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to send signal"));
    }

    // Note: Commented out because sending signals to self can interfere with test runner
    // #[tokio::test]
    // async fn test_send_signal_to_self() {
    //     let handler = SignalHandler::new();
    //     let current_pid = std::process::id();
    //
    //     // Sending SIGUSR1 to self should work (it's a harmless signal)
    //     let result = handler.send_signal(current_pid, ProcessSignal::Usr1);
    //     assert!(result.is_ok());
    // }

    #[tokio::test]
    async fn test_graceful_shutdown_nonexistent_process() {
        let handler = SignalHandler::new();
        let fake_pid = 999999u32; // Very unlikely to exist

        let result = handler.graceful_shutdown(fake_pid, 100).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to send signal"));
    }

    // Note: Commented out because it can interfere with test environment
    // #[tokio::test]
    // async fn test_graceful_shutdown_with_sleep_process() {
    //     let handler = SignalHandler::new();
    //
    //     // Start a sleep process that we can control
    //     let mut child = Command::new("sleep")
    //         .arg("10")
    //         .stdout(Stdio::null())
    //         .stderr(Stdio::null())
    //         .spawn()
    //         .expect("Failed to start sleep process");
    //
    //     let pid = child.id();
    //
    //     // Test graceful shutdown with short timeout
    //     let result = handler.graceful_shutdown(pid, 500).await;
    //
    //     // Clean up - make sure the process is killed
    //     let _ = child.kill();
    //     let _ = child.wait();
    //
    //     // The result should be Ok (either graceful or force killed)
    //     assert!(result.is_ok());
    // }

    #[tokio::test]
    async fn test_setup_handlers() {
        let handler = SignalHandler::new();

        // This should not fail, but we can't easily test the actual signal handling
        // without sending signals to the test process
        let result = handler.setup_handlers().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_signal_equality() {
        assert_eq!(ProcessSignal::Term, ProcessSignal::Term);
        assert_ne!(ProcessSignal::Term, ProcessSignal::Kill);
        assert_ne!(ProcessSignal::Int, ProcessSignal::Quit);
        assert_eq!(ProcessSignal::Usr1, ProcessSignal::Usr1);
    }

    #[test]
    fn test_process_signal_comprehensive_coverage() {
        // Test all signals can be created and used
        let signals = vec![
            ProcessSignal::Term,
            ProcessSignal::Kill,
            ProcessSignal::Int,
            ProcessSignal::Quit,
            ProcessSignal::Usr1,
            ProcessSignal::Usr2,
        ];

        for signal in signals {
            // Test display
            let display_str = signal.to_string();
            assert!(!display_str.is_empty());
            assert!(display_str.starts_with("SIG"));

            // Test debug
            let debug_str = format!("{:?}", signal);
            assert!(!debug_str.is_empty());

            // Test categorization
            let is_term = signal.is_termination_signal();
            let is_user = signal.is_user_signal();
            // A signal should be either termination or user, but not both
            assert!(is_term || is_user);
            assert!(!(is_term && is_user));
        }
    }
}
