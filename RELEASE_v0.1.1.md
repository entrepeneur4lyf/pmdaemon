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
  pmdaemon start api.js --health-check-timeout 30s --health-check-retries 5
  ```

### 🚦 Blocking Start Command
- **Wait for Ready** - Block start command until processes are healthy
  ```bash
  pmdaemon start app.js --health-check-url http://localhost:9615/health --wait-ready
  ```
- **Configurable Wait Timeout** - Set maximum wait time for process readiness
  ```bash
  pmdaemon start worker.js --wait-ready --wait-timeout 60s
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

## 📊 Project Stats & Quality

- **223 total tests** (up from 158 prior to v0.1.1 delete/health check enhancements)
  - Comprehensive health check coverage
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
| Interactive confirmations| ✅              | ❌  |
| Force flag for automation| ✅              | ❌  |
| Safe process shutdown   | ✅              | ❌  |

| Feature (Health Checks) | PMDaemon v0.1.1 | PM2 |
|-------------------------|:---------------:|:---:|
| HTTP health checks      | ✅              | ❌  |
| Script-based health checks| ✅              | ❌  |
| Blocking start (wait-ready)| ✅              | ❌  |
| Health-based auto-restart| ✅              | ❌  |
| Configurable health params| ✅              | ❌  |
