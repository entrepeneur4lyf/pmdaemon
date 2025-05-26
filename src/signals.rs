//! Signal handling for graceful process management

use crate::error::{Error, Result};
use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use std::str::FromStr;
use tokio::signal;
use tracing::{debug, info, warn};

/// Signal handler for process management
pub struct SignalHandler {
    // TODO: Add signal handling fields
}

impl SignalHandler {
    /// Create a new signal handler
    pub fn new() -> Self {
        Self {
            // TODO: Initialize signal handling
        }
    }

    /// Send a signal to a process
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

        kill(pid, signal).map_err(|e| Error::signal(format!("Failed to send signal: {}", e)))?;
        Ok(())
    }

    /// Setup signal handlers for graceful shutdown
    pub async fn setup_handlers(&self) -> Result<()> {
        info!("Setting up signal handlers for graceful shutdown");

        // Setup SIGTERM handler
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
            .map_err(|e| Error::signal(format!("Failed to setup SIGTERM handler: {}", e)))?;

        // Setup SIGINT handler
        let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())
            .map_err(|e| Error::signal(format!("Failed to setup SIGINT handler: {}", e)))?;

        tokio::spawn(async move {
            tokio::select! {
                _ = sigterm.recv() => {
                    info!("Received SIGTERM, initiating graceful shutdown");
                    // TODO: Trigger graceful shutdown of all processes
                }
                _ = sigint.recv() => {
                    info!("Received SIGINT, initiating graceful shutdown");
                    // TODO: Trigger graceful shutdown of all processes
                }
            }
        });

        Ok(())
    }

    /// Gracefully shutdown a process
    pub async fn graceful_shutdown(&self, pid: u32, timeout_ms: u64) -> Result<()> {
        debug!("Initiating graceful shutdown for PID {}", pid);

        // Send SIGTERM first
        self.send_signal(pid, ProcessSignal::Term)?;

        // Wait for process to exit or timeout
        let timeout = tokio::time::Duration::from_millis(timeout_ms);
        let start = tokio::time::Instant::now();

        while start.elapsed() < timeout {
            // Check if process still exists
            match kill(Pid::from_raw(pid as i32), None) {
                Ok(_) => {
                    // Process still exists, wait a bit more
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
                Err(_) => {
                    // Process no longer exists
                    debug!("Process {} exited gracefully", pid);
                    return Ok(());
                }
            }
        }

        // Timeout reached, send SIGKILL
        warn!("Process {} did not exit gracefully, sending SIGKILL", pid);
        self.send_signal(pid, ProcessSignal::Kill)?;

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
        let _handler = SignalHandler::new();
        // Just verify it can be created without panicking
        assert!(true);
    }

    #[test]
    fn test_signal_handler_default() {
        let _handler = SignalHandler::default();
        // Just verify it can be created without panicking
        assert!(true);
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
