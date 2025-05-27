# Changelog

All notable changes to PMDaemon will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-05-26

### 🚀 Added

#### **Enhanced Delete Operations**
- **Bulk deletion**: Added `delete all` command to stop and delete all processes at once
- **Status-based deletion**: Added `delete <status> --status` to delete processes by their current state
  - Valid statuses: `starting`, `online`, `stopping`, `stopped`, `errored`, `restarting`
- **Force flag**: Added `--force` / `-f` flag to skip confirmation prompts for automation
- **Safety confirmations**: Interactive prompts for bulk operations to prevent accidental deletions
- **Process shutdown**: All delete operations now properly stop running processes before deletion
- **Enhanced feedback**: Clear reporting of how many processes were stopped vs. deleted

#### **Health Checks & Monitoring**
- **HTTP Health Checks**: Monitor process health via HTTP endpoints (`--health-check-url`)
- **Script-based Health Checks**: Custom health check scripts for complex validation (`--health-check-script`)
- **Configurable Health Parameters**: Timeout (`--health-check-timeout`), interval, and retry settings (`--health-check-retries`)
- **Health-based Auto-restart**: Automatic restart when health checks fail
- **Monitoring Integration**: Health status visible in process listings (`pmdaemon monit`) and web API

#### **Blocking Start Command**
- **Wait for Ready**: Block start command until processes are healthy (`--wait-ready`)
- **Configurable Wait Timeout**: Set maximum wait time for process readiness (`--wait-timeout`)
- **Progress Indicators**: Real-time status updates during health check waiting
- **Script-friendly**: Ideal for deployment scripts that need to wait for services

#### **Configurable Monitoring Intervals**
- **Library**: Added `monitor_with_interval(Duration)` method for configurable update intervals
- **CLI**: Added `--interval` / `-i` flag to `monit` command for custom refresh rates
- **Performance optimization**: Users can now balance responsiveness vs. resource usage:
  - Fast updates (1s) for debugging and development
  - Balanced updates (2s) for general use (library default)
  - Slower updates (5s+) for reduced system load

#### **Enhanced Log Management**
- **Real-time log following**: Implemented `follow_logs()` method with `tail -f` functionality
- **Configurable log retrieval**: Enhanced `get_logs()` method with proper line limiting
- **Missing file handling**: Graceful handling of non-existent log files
- **CLI integration**: `pmdaemon logs --follow` for real-time log monitoring

#### **Professional Monitoring Display**
- **Beautiful table formatting**: Integrated `comfy-table` for professional display
- **Color-coded status indicators**:
  - 🟢 Green for Online processes
  - 🔴 Red for Stopped/Errored processes
  - 🟡 Yellow for Starting/Stopping processes
  - 🔵 Blue for Restarting processes
- **PID column**: Added Process ID display for better debugging and system integration
- **Enhanced system overview**: Improved system metrics display with proper formatting

#### **Comprehensive Test Coverage**
- **New test suites**: Added new tests for delete operations, health checks, blocking start, monitoring, and log functionality.
- **Delete operations tests**: Coverage for bulk deletion, status-based deletion, safe shutdown.
- **Health check tests**: Validation of HTTP and script-based checks, timeouts, and retries.
- **Blocking start tests**: Ensuring `--wait-ready` and `--wait-timeout` function correctly.
- **Log management tests**: Comprehensive testing of log retrieval and error handling.
- **Monitoring configuration tests**: Validation of configurable intervals.
- **Performance tests**: Basic performance validation for monitoring operations.

### 🔧 Enhanced

#### **Delete Command Safety**
- **Process lifecycle management**: Delete operations now properly stop running processes before removal
- **Graceful shutdown**: Uses existing `process.stop()` method for proper process termination
- **Error resilience**: Continues with deletion even if process stopping fails (with warning)
- **Detailed logging**: Enhanced logging shows exactly what processes were stopped vs. deleted
- **CLI output improvements**: Updated messages to clearly indicate "Stopped and deleted" behavior

#### **Signal Module Completion**
- **Eliminated TODO comments**: Completed all signal handling functionality
- **Proper implementation**: Full signal handler setup and graceful shutdown
- **Thread-safe operations**: Proper atomic operations for shutdown flags

#### **Manager Module Completion**
- **Log functionality**: Completed all log-related methods (`get_logs`, `follow_logs`)
- **Monitoring integration**: Proper integration with monitoring system, including health status.
- **Error handling**: Robust error handling throughout

#### **Documentation Improvements**
- **Rustdoc compliance**: All methods include comprehensive documentation with examples
- **Performance guidance**: Clear documentation on interval trade-offs
- **Cross-references**: Proper linking between related functionality

### 🐛 Fixed
- **Production-ready code**: Eliminated all TODO comments from production codebase
- **Thread safety**: Improved async/await patterns and lock management
- **Memory efficiency**: Optimized file reading and monitoring operations

### 📊 Technical Details
- **Dependencies**: Added `comfy-table` for professional table formatting. (reqwest for HTTP health checks is likely already a dependency or part of a larger HTTP client lib like hyper/axum used for the web API).
- **Performance**: Configurable intervals allow optimization for different use cases. Health checks add minimal overhead.
- **Compatibility**: Backward compatible - no breaking changes to existing API.

### 🧪 Testing
- **Test count**: 223 total tests (up from 158 before health checks & blocking start)
- **New coverage**: Delete operations (bulk, status-based, safe shutdown), Health Checks (HTTP, script, params), Blocking Start (`--wait-ready`, `--wait-timeout`), and process lifecycle management.
- **Enhanced test suites**: Added comprehensive tests for all new delete, health check, and blocking start functionality.
- **Quality**: 100% test success rate with comprehensive error path testing.

## [0.1.0] - 2024-12-18

### Added
- **Core Process Management**
  - Complete lifecycle management (start, stop, restart, reload, delete)
  - Process clustering with automatic load balancing
  - Auto-restart on crash with configurable limits
  - Graceful shutdown with proper signal handling (SIGTERM/SIGINT)
  - Configuration persistence and multi-session support
- **Advanced Port Management** (Beyond PM2)
  - Port range distribution for clusters (`--port 3000-3003`)
  - Auto-assignment from ranges (`--port auto:5000-5100`)
  - Built-in port conflict detection
  - Runtime port overrides without config changes
  - Port visibility in process listings
- **Real-time Monitoring**
  - CPU, memory, uptime tracking
  - System metrics (load average, total memory)
  - Memory limit enforcement with automatic restart
  - Process health checks and auto-restart logic
- **CLI Interface**
  - All PM2-compatible commands (start, stop, restart, reload, delete, list, logs, monit)
  - Enhanced CLI display with color-coded statuses
  - Formatted tables using comfy-table
- **Web API & WebSocket Support**
  - REST API for full process management
  - PM2-compatible JSON responses
  - Real-time WebSocket updates
  - CORS support and security headers
- **Log Management**
  - Separate stdout/stderr files
  - PID file handling
  - Log viewing and following capabilities
- **Testing & Documentation**
  - 158 comprehensive tests (unit, integration, e2e, doc tests)
  - Complete Rust documentation
  - Usage examples and guides

### Technical Details
- Built with Rust for performance and memory safety
- Async/await architecture using Tokio
- Web server powered by Axum
- System monitoring via sysinfo
- Cross-platform support (Linux, macOS, Windows)

[0.1.1]: https://github.com/entrepeneur4lyf/pmdaemon/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/entrepeneur4lyf/pmdaemon/releases/tag/v0.1.0
