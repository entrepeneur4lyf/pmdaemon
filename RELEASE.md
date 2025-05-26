# PMDaemon v0.1.1 - Enhanced Deletion, Health Checks & Blocking Start 🚀🗑️🏥

**Release Date:** January 26, 2025

We are excited to announce PMDaemon v0.1.1 - a major feature update that adds powerful **Enhanced Delete Operations**, comprehensive **Health Check functionality**, and **Blocking Start capabilities**, making PMDaemon even more robust and user-friendly for production deployments.

## 🎉 What's New in v0.1.1

This release introduces advanced health check capabilities, blocking start functionality, and significantly enhanced delete operations, addressing key user requests for production process management. PMDaemon now provides comprehensive monitoring and control that goes far beyond what's available in PM2.

## ✨ New Features in v0.1.1

### 🏥 Health Checks & Monitoring
- **HTTP Health Checks** - Monitor process health via HTTP endpoints with configurable timeouts
  ```bash
  pmdaemon start app.js --health-check-url http://localhost:9615/health
  ```
- **Script-based Health Checks** - Custom health check scripts for complex validation
  ```bash
  pmdaemon start worker.js --health-check-script ./health-check.sh
  ```
- **Configurable Health Parameters** - Timeout, interval, and retry settings
  ```bash
  pmdaemon start api.js --health-check-timeout 5s --health-check-retries 3
  ```

### 🚦 Blocking Start Command
- **Wait for Ready** - Block start command until processes are healthy
  ```bash
  pmdaemon start app.js --health-check-url http://localhost:9615/health --wait-ready
  ```
- **Configurable Wait Timeout** - Set maximum wait time for process readiness
  ```bash
  pmdaemon start app.js --wait-ready --wait-timeout 30s
  ```
- **Progress Indicators** - Real-time status updates during health check waiting
- **Script-friendly** - Perfect for deployment scripts that need to wait for services

### 🗑️ Enhanced Delete Operations
PMDaemon v0.1.1 introduces powerful new delete capabilities that go far beyond basic process removal:

#### **Bulk Deletion**
- **`pmdaemon delete all`** - Delete all processes at once
- **Safety confirmations** - Interactive prompts prevent accidental bulk deletions
- **Force flag** - `--force` / `-f` skips confirmations for automation scenarios

#### **Status-Based Deletion**
- **`pmdaemon delete <status> --status`** - Delete processes by their current state
- **Supported statuses:**
  - `starting` - Processes currently starting up
  - `online` - Running processes
  - `stopping` - Processes currently shutting down
  - `stopped` - Processes that have exited
  - `errored` - Processes that crashed or failed
  - `restarting` - Processes currently restarting

#### **Safe Process Shutdown**
- **Automatic process stopping** - All delete operations now properly stop running processes before deletion
- **Graceful shutdown** - Uses existing `process.stop()` method for proper process termination
- **Error resilience** - Continues with deletion even if process stopping fails (with warning)
- **Resource cleanup** - Proper cleanup of files, configurations, and port allocations

#### **Enhanced Safety & User Experience for Deletion**
- **Interactive Confirmations**
  ```bash
  # Safe bulk operations with confirmation
  $ pmdaemon delete all
  Are you sure you want to delete ALL processes? (y/N):

  # Skip confirmations for automation
  $ pmdaemon delete all --force
  Stopped and deleted 5 processes
  ```
- **Clear Feedback**
  - **Detailed reporting** - Shows exactly how many processes were stopped vs. deleted
  - **Enhanced CLI output** - Updated messages clearly indicate "Stopped and deleted" behavior
  - **Comprehensive logging** - Detailed logs show the complete deletion process

#### **Usage Examples for Deletion**
##### **Basic Delete Operations**
```bash
# Delete single process (stops if running)
pmdaemon delete myapp

# Delete all processes with confirmation
pmdaemon delete all

# Delete all processes without confirmation
pmdaemon delete all --force
```
##### **Status-Based Deletion**
```bash
# Delete all stopped processes
pmdaemon delete stopped --status

# Delete all errored processes without confirmation
pmdaemon delete errored --status --force

# Delete all running processes (useful for cleanup)
pmdaemon delete online --status
```

#### **Performance & Reliability for Deletion**
- **Optimized Operations** - Optimized for handling large numbers of processes
- **Minimal resource usage** - Careful memory management during bulk operations
- **Concurrent processing** - Parallel deletion where safe and beneficial
- **Error Handling**
  - **Graceful degradation** - Operations continue even if individual processes fail to stop
  - **Detailed error reporting** - Clear error messages for troubleshooting
  - **Recovery mechanisms** - Automatic cleanup even after partial failures

### 🔄 Enhanced Auto-restart
- **Health-based Restart** - Automatic restart when health checks fail
- **Integration with Monitoring** - Health status visible in process listings and web API
- **WebSocket Health Updates** - Real-time health status changes via WebSocket

## 🔄 All Previous Features (v0.1.0)

### Core Process Management
- **Complete lifecycle management** - Start, stop, restart, reload, and delete processes
- **Clustering support** - Run multiple instances with automatic load balancing
- **Auto-restart on crash** - Configurable restart limits and strategies
- **Graceful shutdown** - Proper signal handling (SIGTERM/SIGINT)
- **Configuration persistence** - Process configs saved/restored between sessions

### 🌟 Innovative Port Management (Beyond PM2)
- **Port range distribution** for clusters (`--port 3000-3003`)
- **Auto-assignment** from ranges (`--port auto:5000-5100`)
- **Built-in conflict detection** and runtime port overrides
- **Port visibility** in process listings

### Monitoring & Web API
- **Real-time monitoring** - CPU, memory, uptime tracking
- **Memory limit enforcement** - Automatic restart when exceeding limits
- **REST API** - Full process management via HTTP with PM2-compatible responses
- **WebSocket support** - Real-time process updates and monitoring

## 📊 Project Stats & Quality

- **223 total tests** (up from 158 prior to v0.1.1 delete/health check enhancements)
  - Comprehensive health check coverage
  - New test suites for delete operations:
    - Bulk deletion functionality
    - Status-based deletion
    - Process lifecycle management (for deletion)
    - Error handling and edge cases in deletion
- **End-to-end tests** verify complete delete workflows and health check integrations.
- **Safety testing** ensures confirmations work correctly.
- **Error path testing** validates graceful failure handling.
- **8 completed development phases** (including health checks and enhanced delete)
- **100% core feature coverage** plus advanced health monitoring and deletion
- **Production-ready** stability with enhanced reliability features

## 🆚 Comparison with PM2

PMDaemon v0.1.1 now offers several capabilities, particularly in deletion, that PM2 lacks:

| Feature (Deletion)      | PMDaemon v0.1.1 | PM2 |
|-------------------------|:---------------:|:---:|
| Bulk deletion (delete all)| ✅              | ❌  |
| Status-based deletion   | ✅              | ❌  |
| Safe process shutdown   | ✅              | ❌  |
| Interactive confirmations| ✅              | ❌  |
| Force flag for automation| ✅              | ❌  |

## 🚀 Quick Start

```bash
# Install via Cargo
cargo install pmdaemon

# Start a process
pmdaemon start app.js --name myapp

# Start a cluster with port distribution
pmdaemon start server.js --instances 4 --port 3000-3003

# Start with health checks and wait for ready
pmdaemon start app.js --health-check-url http://localhost:9615/health --wait-ready

# Monitor processes (now shows health status)
pmdaemon monit

# Start web API
pmdaemon web --port 9615

# For delete command examples, see "Enhanced Delete Operations" section above.
```

## 📦 What's Included in v0.1.1

- ✅ All PM2 core commands (start, stop, restart, reload, delete, list, logs, monit)
- ✅ **NEW:** HTTP and script-based health checks
- ✅ **NEW:** Blocking start command with `--wait-ready`
- ✅ **NEW:** Configurable health check parameters (timeout, interval, retries)
- ✅ **NEW:** Health status integration in monitoring and web API
- ✅ **NEW:** Bulk deletion (`pmdaemon delete all`)
- ✅ **NEW:** Status-based deletion (`pmdaemon delete <status> --status`)
- ✅ **NEW:** Safe process shutdown (automatic stop before delete)
- ✅ **NEW:** Interactive confirmations for delete operations
- ✅ **NEW:** Force flag (`--force`) for delete automation
- ✅ Process clustering with load balancing
- ✅ Advanced port management (ranges, auto-assignment, conflict detection)
- ✅ Environment variable management and working directory configuration
- ✅ Auto-restart with memory limits and health-based restart
- ✅ Real-time monitoring with formatted output and health status
- ✅ Web API with WebSocket support and health endpoints
- ✅ Comprehensive error handling and cross-platform support

## 🔧 Technical Details

- Built with Rust for performance and memory safety
- Async/await architecture using Tokio
- HTTP health checks powered by reqwest
- Web server powered by Axum with health endpoints
- System monitoring via sysinfo with health status integration
- Comprehensive test coverage including health check and deletion scenarios

## 📝 Upgrade Notes & Backward Compatibility

### **For Existing Users (Upgrading from v0.1.0)**
- No migration required for basic functionality! All existing commands work as before.
- **Enhanced behavior:** Single process deletion (`pmdaemon delete myapp`) now includes automatic stopping for safety.
- New health check and advanced delete features are opt-in via command line flags.
- No breaking changes to existing configurations or APIs. Library API maintains full backward compatibility.
- Health check dependencies are automatically included.

### **For Automation Scripts (Regarding Deletion)**
If your scripts use `pmdaemon delete all` or status-based deletion, consider adding the `--force` flag to skip new interactive confirmation prompts:
```bash
# Old way (single delete, still works, now safer)
pmdaemon delete myapp

# New way for automation with bulk delete
pmdaemon delete all --force

# New way for automation with status-based delete
pmdaemon delete errored --status --force
```

## 🔮 What's Next

This release significantly enhances PMDaemon's production readiness. Future releases will build on these capabilities with:
- **From Health Checks & Blocking Start:**
  - Enhanced health check visualization in the web interface
  - Custom health check plugins and extensibility
  - Advanced deployment automation features
  - Performance optimizations and benchmarks
- **From Enhanced Delete Operations:**
  - Conditional deletion - Delete based on resource usage, uptime, etc.
  - Batch operations - Apply operations to multiple named processes
  - Advanced filtering - More sophisticated process selection criteria

## 🙏 Acknowledgments

This project was inspired by the excellent [PM2](https://pm2.keymetrics.io/) process manager. While PMDaemon aims to provide similar functionality, it leverages Rust's performance and safety benefits while adding innovative features for modern deployment scenarios.
Special thanks to the community for feedback on process management workflows, safety requirements for deletion, and health monitoring needs. This release directly addresses user requests for more powerful, safer, and observable process management.

## 🚀 Get Started

```bash
cargo install pmdaemon
pmdaemon --help
```

Thank you for using PMDaemon! The new enhanced deletion, health check, and blocking start features make it even more powerful for production deployments.

---

**Download PMDaemon v0.1.1:**
- [GitHub Releases](https://github.com/entrepeneur4lyf/pmdaemon/releases/tag/v0.1.1)
- `cargo install pmdaemon`

**Full Changelog:** [v0.1.0...v0.1.1](https://github.com/entrepeneur4lyf/pmdaemon/compare/v0.1.0...v0.1.1)
**Previous Release (v0.1.0):** [https://github.com/entrepeneur4lyf/pmdaemon/releases/tag/v0.1.0](https://github.com/entrepeneur4lyf/pmdaemon/releases/tag/v0.1.0)
