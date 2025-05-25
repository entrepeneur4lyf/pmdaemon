# PMDaemon v0.1.0 - Initial Release 🚀

We are excited to announce the first release of PMDaemon - a high-performance process manager built in Rust, inspired by PM2 with innovative features that exceed the original.

## 🎉 Highlights

PMDaemon brings modern process management to Rust with production-ready features and performance benefits. This initial release includes all core PM2 functionality plus several innovative features not found in the original PM2.

## ✨ Key Features

### Core Process Management
- **Complete lifecycle management** - Start, stop, restart, reload, and delete processes
- **Clustering support** - Run multiple instances with automatic load balancing
- **Auto-restart on crash** - Configurable restart limits and strategies
- **Graceful shutdown** - Proper signal handling (SIGTERM/SIGINT)
- **Configuration persistence** - Process configs saved/restored between sessions
- **Multi-session support** - Processes persist across CLI sessions

### 🌟 Innovative Features (Beyond PM2)
- **Advanced Port Management**
  - Port range distribution for clusters (`--port 3000-3003`)
  - Auto-assignment from ranges (`--port auto:5000-5100`)
  - Built-in conflict detection
  - Runtime port overrides without config changes
  - Port visibility in process listings
- **Memory Limit Enforcement** - Automatic restart when exceeding limits (`--max-memory 100M`)
- **WebSocket Support** - Real-time process updates and monitoring
- **Enhanced CLI Display** - Color-coded statuses and formatted tables

### Monitoring & Logging
- **Real-time monitoring** - CPU, memory, uptime tracking
- **System metrics** - Load average, total memory usage
- **Log management** - Separate stdout/stderr files
- **PID file tracking** - Reliable process discovery

### Web API & Integration
- **REST API** - Full process management via HTTP
- **PM2-compatible responses** - Drop-in replacement potential
- **WebSocket endpoint** - Live status updates
- **CORS support** - Production-ready security headers

## 📊 Project Stats

- **158 tests** (120 unit + 11 integration + 8 e2e + 19 doc tests)
- **7 completed development phases**
- **100% core feature coverage**
- **Production-ready** stability

## 🚀 Quick Start

```bash
# Install via Cargo
cargo install pmdaemon

# Start a process
pmdaemon start app.js --name myapp

# Start a cluster with port distribution
pmdaemon start server.js --instances 4 --port 3000-3003

# Monitor processes
pmdaemon monit

# Start web API
pmdaemon web --port 9615
```

## 📦 What's Included

- ✅ All PM2 core commands (start, stop, restart, reload, delete, list, logs, monit)
- ✅ Process clustering with load balancing
- ✅ Environment variable management
- ✅ Working directory configuration
- ✅ Auto-restart with memory limits
- ✅ Real-time monitoring with formatted output
- ✅ Web API with WebSocket support
- ✅ Comprehensive error handling
- ✅ Cross-platform support (Linux, macOS, Windows)

## 🔧 Technical Details

- Built with Rust for performance and memory safety
- Async/await architecture using Tokio
- Web server powered by Axum
- System monitoring via sysinfo
- Comprehensive test coverage

## 🙏 Acknowledgments

This project was inspired by the excellent [PM2](https://pm2.keymetrics.io/) process manager. While PMDaemon aims to provide similar functionality, it leverages Rust's performance and safety benefits while adding innovative features for modern deployment scenarios.

## 📝 Notes

This is our initial release. We've thoroughly tested all features, but if you encounter any issues, please report them on our [GitHub repository](https://github.com/entrepeneur4lyf/pmdaemon).

## 🚀 Get Started

```bash
cargo install pmdaemon
pmdaemon --help
```

Thank you for trying PMDaemon! We're excited to see how you use it in your projects.

---

**Full Changelog**: https://github.com/entrepeneur4lyf/pmdaemon/commits/v0.1.0