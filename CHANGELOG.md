# Changelog

All notable changes to PMDaemon will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2024-12-19

### Added
- **Health Check System** - Comprehensive health monitoring capabilities
  - HTTP health checks with configurable endpoints
  - Script-based health checks for custom validation
  - Configurable timeout, interval, and retry parameters
  - Health status integration in process listings and monitoring
- **Blocking Start Command** - Wait for processes to be healthy before returning
  - `--wait-ready` flag to block start until health checks pass
  - `--wait-timeout` option for configurable wait timeouts
  - Progress indicators during health check waiting
  - Perfect for deployment scripts that need to wait for services
- **Enhanced Auto-restart** - Health-based process restart capabilities
  - Automatic restart when health checks fail
  - Integration with existing memory limit restart functionality
- **Web API Health Endpoints** - Health status via REST API
  - Health check status in process information endpoints
  - Real-time health updates via WebSocket
- **New CLI Options**
  - `--health-check-url <url>` - HTTP endpoint for health checks
  - `--health-check-script <path>` - Script to run for health validation
  - `--health-check-timeout <time>` - Timeout for individual health checks
  - `--health-check-interval <time>` - Interval between health checks
  - `--health-check-retries <num>` - Number of retries before failure
  - `--wait-ready` - Block start until health checks pass
  - `--wait-timeout <time>` - Timeout for blocking start

### Changed
- Updated process monitoring to include health status
- Enhanced process listings to show health information
- Improved WebSocket API to broadcast health status changes
- Updated documentation with health check examples

### Dependencies
- Added `reqwest` for HTTP health check functionality

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
