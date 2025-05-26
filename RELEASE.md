# PMDaemon v0.1.1 - Health Checks & Blocking Start 🏥

We are excited to announce PMDaemon v0.1.1 - a major feature update that adds comprehensive health check functionality and blocking start capabilities, making PMDaemon even more powerful for production deployments.

## 🎉 What's New in v0.1.1

This release introduces advanced health check capabilities and blocking start functionality, addressing one of the most requested features for production process management. PMDaemon now provides comprehensive health monitoring that goes far beyond what's available in PM2.

## ✨ New Features in v0.1.1

### 🏥 Health Checks & Monitoring
- **HTTP Health Checks** - Monitor process health via HTTP endpoints with configurable timeouts
  ```bash
  pmdaemon start app.js --health-check-url http://localhost:3000/health
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
  pmdaemon start app.js --health-check-url http://localhost:3000/health --wait-ready
  ```
- **Configurable Wait Timeout** - Set maximum wait time for process readiness
  ```bash
  pmdaemon start app.js --wait-ready --wait-timeout 30s
  ```
- **Progress Indicators** - Real-time status updates during health check waiting
- **Script-friendly** - Perfect for deployment scripts that need to wait for services

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

## 📊 Project Stats

- **180+ tests** with comprehensive health check coverage
- **8 completed development phases** (including health checks)
- **100% core feature coverage** plus advanced health monitoring
- **Production-ready** stability with enhanced reliability features

## 🚀 Quick Start

```bash
# Install via Cargo
cargo install pmdaemon

# Start a process
pmdaemon start app.js --name myapp

# Start a cluster with port distribution
pmdaemon start server.js --instances 4 --port 3000-3003

# Start with health checks and wait for ready
pmdaemon start app.js --health-check-url http://localhost:3000/health --wait-ready

# Monitor processes (now shows health status)
pmdaemon monit

# Start web API
pmdaemon web --port 9615
```

## 📦 What's Included in v0.1.1

- ✅ All PM2 core commands (start, stop, restart, reload, delete, list, logs, monit)
- ✅ **NEW:** HTTP and script-based health checks
- ✅ **NEW:** Blocking start command with `--wait-ready`
- ✅ **NEW:** Configurable health check parameters (timeout, interval, retries)
- ✅ **NEW:** Health status integration in monitoring and web API
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
- Comprehensive test coverage including health check scenarios

## 🙏 Acknowledgments

This project was inspired by the excellent [PM2](https://pm2.keymetrics.io/) process manager. While PMDaemon aims to provide similar functionality, it leverages Rust's performance and safety benefits while adding innovative features for modern deployment scenarios.

## 🆕 What's Next

This release significantly enhances PMDaemon's production readiness with health checks and blocking start functionality. Future releases will focus on:

- Enhanced health check visualization in the web interface
- Custom health check plugins and extensibility
- Advanced deployment automation features
- Performance optimizations and benchmarks

## 📝 Upgrade Notes

If upgrading from v0.1.0:
- All existing functionality remains unchanged
- New health check features are opt-in via command line flags
- No breaking changes to existing configurations or APIs
- Health check dependencies are automatically included

## 🚀 Get Started

```bash
cargo install pmdaemon
pmdaemon --help
```

Thank you for using PMDaemon! The new health check features make it even more powerful for production deployments.

---

**Full Changelog**: https://github.com/entrepeneur4lyf/pmdaemon/compare/v0.1.0...v0.1.1
**Previous Release**: https://github.com/entrepeneur4lyf/pmdaemon/releases/tag/v0.1.0
