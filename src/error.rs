//! Error types for PM2 Rust

/// Result type alias for PM2 Rust operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for PM2 Rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO operation failed
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Process operation failed
    #[error("Process error: {message}")]
    Process {
        /// Error message describing the process operation failure
        message: String,
    },

    /// Process not found
    #[error("Process not found: {identifier}")]
    ProcessNotFound {
        /// Process identifier (name or ID) that was not found
        identifier: String,
    },

    /// Process already exists
    #[error("Process already exists: {name}")]
    ProcessAlreadyExists {
        /// Name of the process that already exists
        name: String,
    },

    /// Process already running
    #[error("Process already running: {0}")]
    ProcessAlreadyRunning(String),

    /// Process start failed
    #[error("Failed to start process {name}: {reason}")]
    ProcessStartFailed {
        /// Name of the process that failed to start
        name: String,
        /// Reason for the start failure
        reason: String,
    },

    /// Process stop failed
    #[error("Failed to stop process {name}: {reason}")]
    ProcessStopFailed {
        /// Name of the process that failed to stop
        name: String,
        /// Reason for the stop failure
        reason: String,
    },

    /// Configuration error
    #[error("Configuration error: {message}")]
    Config {
        /// Configuration error message
        message: String,
    },

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// TOML parsing error
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// System error (nix)
    #[error("System error: {0}")]
    #[cfg(unix)]
    System(#[from] nix::Error),

    /// Signal handling error
    #[error("Signal error: {message}")]
    Signal {
        /// Signal handling error message
        message: String,
    },

    /// Monitoring error
    #[error("Monitoring error: {message}")]
    Monitoring {
        /// Monitoring system error message
        message: String,
    },

    /// Web server error
    #[error("Web server error: {message}")]
    WebServer {
        /// Web server error message
        message: String,
    },

    /// Timeout error
    #[error("Operation timed out: {operation}")]
    Timeout {
        /// Operation that timed out
        operation: String,
    },

    /// Permission denied
    #[error("Permission denied: {message}")]
    PermissionDenied {
        /// Permission denied error message
        message: String,
    },

    /// Invalid argument
    #[error("Invalid argument: {message}")]
    InvalidArgument {
        /// Invalid argument error message
        message: String,
    },

    /// Resource not available
    #[error("Resource not available: {resource}")]
    ResourceNotAvailable {
        /// Resource that is not available
        resource: String,
    },

    /// Internal error
    #[error("Internal error: {message}")]
    Internal {
        /// Internal error message
        message: String,
    },

    /// Health check error
    #[error("Health check error: {message}")]
    HealthCheck {
        /// Health check error message
        message: String,
    },
}

impl Error {
    /// Create a new process error
    pub fn process<S: Into<String>>(message: S) -> Self {
        Self::Process {
            message: message.into(),
        }
    }

    /// Create a new process not found error
    pub fn process_not_found<S: Into<String>>(identifier: S) -> Self {
        Self::ProcessNotFound {
            identifier: identifier.into(),
        }
    }

    /// Create a new process already exists error
    pub fn process_already_exists<S: Into<String>>(name: S) -> Self {
        Self::ProcessAlreadyExists { name: name.into() }
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a new signal error
    pub fn signal<S: Into<String>>(message: S) -> Self {
        Self::Signal {
            message: message.into(),
        }
    }

    /// Create a new monitoring error
    pub fn monitoring<S: Into<String>>(message: S) -> Self {
        Self::Monitoring {
            message: message.into(),
        }
    }

    /// Create a new web server error
    pub fn web_server<S: Into<String>>(message: S) -> Self {
        Self::WebServer {
            message: message.into(),
        }
    }

    /// Create a new timeout error
    pub fn timeout<S: Into<String>>(operation: S) -> Self {
        Self::Timeout {
            operation: operation.into(),
        }
    }

    /// Create a new permission denied error
    pub fn permission_denied<S: Into<String>>(message: S) -> Self {
        Self::PermissionDenied {
            message: message.into(),
        }
    }

    /// Create a new invalid argument error
    pub fn invalid_argument<S: Into<String>>(message: S) -> Self {
        Self::InvalidArgument {
            message: message.into(),
        }
    }

    /// Create a new resource not available error
    pub fn resource_not_available<S: Into<String>>(resource: S) -> Self {
        Self::ResourceNotAvailable {
            resource: resource.into(),
        }
    }

    /// Create a new internal error
    pub fn internal<S: Into<String>>(message: S) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Create a new health check error
    pub fn health_check<S: Into<String>>(message: S) -> Self {
        Self::HealthCheck {
            message: message.into(),
        }
    }

    /// Check if this error is a process-related error
    pub fn is_process_error(&self) -> bool {
        matches!(
            self,
            Error::Process { .. }
                | Error::ProcessNotFound { .. }
                | Error::ProcessAlreadyExists { .. }
                | Error::ProcessAlreadyRunning(_)
                | Error::ProcessStartFailed { .. }
                | Error::ProcessStopFailed { .. }
        )
    }

    /// Check if this error is a configuration error
    pub fn is_config_error(&self) -> bool {
        matches!(self, Error::Config { .. })
    }

    /// Check if this error is a system error
    pub fn is_system_error(&self) -> bool {
        match self {
            Error::Io(_) => true,
            #[cfg(unix)]
            Error::System(_) => true,
            _ => false,
        }
    }

    /// Get the error category as a string
    pub fn category(&self) -> &'static str {
        match self {
            Error::Io(_) => "io",
            Error::Process { .. } => "process",
            Error::ProcessNotFound { .. } => "process_not_found",
            Error::ProcessAlreadyExists { .. } => "process_already_exists",
            Error::ProcessAlreadyRunning(_) => "process_already_running",
            Error::ProcessStartFailed { .. } => "process_start_failed",
            Error::ProcessStopFailed { .. } => "process_stop_failed",
            Error::Config { .. } => "config",
            Error::Serialization(_) => "serialization",
            Error::Toml(_) => "toml",
            #[cfg(unix)]
            Error::System(_) => "system",
            Error::Signal { .. } => "signal",
            Error::Monitoring { .. } => "monitoring",
            Error::WebServer { .. } => "web_server",
            Error::Timeout { .. } => "timeout",
            Error::PermissionDenied { .. } => "permission_denied",
            Error::InvalidArgument { .. } => "invalid_argument",
            Error::ResourceNotAvailable { .. } => "resource_not_available",
            Error::Internal { .. } => "internal",
            Error::HealthCheck { .. } => "health_check",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io;

    #[test]
    fn test_error_constructors() {
        let err = Error::process("test message");
        assert!(matches!(err, Error::Process { .. }));
        assert_eq!(err.to_string(), "Process error: test message");

        let err = Error::process_not_found("test-process");
        assert!(matches!(err, Error::ProcessNotFound { .. }));
        assert_eq!(err.to_string(), "Process not found: test-process");

        let err = Error::process_already_exists("test-process");
        assert!(matches!(err, Error::ProcessAlreadyExists { .. }));
        assert_eq!(err.to_string(), "Process already exists: test-process");

        let err = Error::config("invalid config");
        assert!(matches!(err, Error::Config { .. }));
        assert_eq!(err.to_string(), "Configuration error: invalid config");

        let err = Error::signal("signal failed");
        assert!(matches!(err, Error::Signal { .. }));
        assert_eq!(err.to_string(), "Signal error: signal failed");

        let err = Error::monitoring("monitoring failed");
        assert!(matches!(err, Error::Monitoring { .. }));
        assert_eq!(err.to_string(), "Monitoring error: monitoring failed");

        let err = Error::web_server("server failed");
        assert!(matches!(err, Error::WebServer { .. }));
        assert_eq!(err.to_string(), "Web server error: server failed");

        let err = Error::timeout("start process");
        assert!(matches!(err, Error::Timeout { .. }));
        assert_eq!(err.to_string(), "Operation timed out: start process");

        let err = Error::permission_denied("access denied");
        assert!(matches!(err, Error::PermissionDenied { .. }));
        assert_eq!(err.to_string(), "Permission denied: access denied");

        let err = Error::invalid_argument("bad arg");
        assert!(matches!(err, Error::InvalidArgument { .. }));
        assert_eq!(err.to_string(), "Invalid argument: bad arg");

        let err = Error::resource_not_available("port 8080");
        assert!(matches!(err, Error::ResourceNotAvailable { .. }));
        assert_eq!(err.to_string(), "Resource not available: port 8080");

        let err = Error::internal("internal failure");
        assert!(matches!(err, Error::Internal { .. }));
        assert_eq!(err.to_string(), "Internal error: internal failure");

        let err = Error::health_check("health check failed");
        assert!(matches!(err, Error::HealthCheck { .. }));
        assert_eq!(err.to_string(), "Health check error: health check failed");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn test_error_from_serde_json() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let err: Error = json_err.into();
        assert!(matches!(err, Error::Serialization(_)));
    }

    #[test]
    fn test_error_from_toml() {
        let toml_err = toml::from_str::<toml::Value>("invalid = toml = syntax").unwrap_err();
        let err: Error = toml_err.into();
        assert!(matches!(err, Error::Toml(_)));
    }

    #[test]
    fn test_error_is_process_error() {
        assert!(Error::process("test").is_process_error());
        assert!(Error::process_not_found("test").is_process_error());
        assert!(Error::process_already_exists("test").is_process_error());
        assert!(Error::ProcessAlreadyRunning("test".to_string()).is_process_error());
        assert!(Error::ProcessStartFailed {
            name: "test".to_string(),
            reason: "failed".to_string()
        }
        .is_process_error());
        assert!(Error::ProcessStopFailed {
            name: "test".to_string(),
            reason: "failed".to_string()
        }
        .is_process_error());

        assert!(!Error::config("test").is_process_error());
        assert!(!Error::signal("test").is_process_error());
    }

    #[test]
    fn test_error_is_config_error() {
        assert!(Error::config("test").is_config_error());
        assert!(!Error::process("test").is_config_error());
        assert!(!Error::signal("test").is_config_error());
    }

    #[test]
    fn test_error_is_system_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(err.is_system_error());

        assert!(!Error::config("test").is_system_error());
        assert!(!Error::process("test").is_system_error());
    }

    #[test]
    fn test_error_category() {
        assert_eq!(Error::process("test").category(), "process");
        assert_eq!(
            Error::process_not_found("test").category(),
            "process_not_found"
        );
        assert_eq!(
            Error::process_already_exists("test").category(),
            "process_already_exists"
        );
        assert_eq!(
            Error::ProcessAlreadyRunning("test".to_string()).category(),
            "process_already_running"
        );
        assert_eq!(
            Error::ProcessStartFailed {
                name: "test".to_string(),
                reason: "failed".to_string()
            }
            .category(),
            "process_start_failed"
        );
        assert_eq!(
            Error::ProcessStopFailed {
                name: "test".to_string(),
                reason: "failed".to_string()
            }
            .category(),
            "process_stop_failed"
        );
        assert_eq!(Error::config("test").category(), "config");
        assert_eq!(Error::signal("test").category(), "signal");
        assert_eq!(Error::monitoring("test").category(), "monitoring");
        assert_eq!(Error::web_server("test").category(), "web_server");
        assert_eq!(Error::timeout("test").category(), "timeout");
        assert_eq!(
            Error::permission_denied("test").category(),
            "permission_denied"
        );
        assert_eq!(
            Error::invalid_argument("test").category(),
            "invalid_argument"
        );
        assert_eq!(
            Error::resource_not_available("test").category(),
            "resource_not_available"
        );
        assert_eq!(Error::internal("test").category(), "internal");
        assert_eq!(Error::health_check("test").category(), "health_check");

        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert_eq!(err.category(), "io");

        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let err: Error = json_err.into();
        assert_eq!(err.category(), "serialization");

        let toml_err = toml::from_str::<toml::Value>("invalid = toml = syntax").unwrap_err();
        let err: Error = toml_err.into();
        assert_eq!(err.category(), "toml");
    }

    #[test]
    fn test_result_type_alias() {
        fn test_function() -> Result<String> {
            Ok("success".to_string())
        }

        fn test_function_error() -> Result<String> {
            Err(Error::process("test error"))
        }

        assert!(test_function().is_ok());
        assert!(test_function_error().is_err());
    }

    #[test]
    fn test_error_debug_format() {
        let err = Error::process("test message");
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Process"));
        assert!(debug_str.contains("test message"));
    }

    #[test]
    fn test_error_display_format() {
        let err = Error::ProcessStartFailed {
            name: "myapp".to_string(),
            reason: "command not found".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Failed to start process myapp: command not found"
        );

        let err = Error::ProcessStopFailed {
            name: "myapp".to_string(),
            reason: "process not responding".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Failed to stop process myapp: process not responding"
        );
    }
}
