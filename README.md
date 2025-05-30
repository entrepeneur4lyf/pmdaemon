<div style="background-color: #1b252f; color: white; padding: 20px; text-align: center; border-radius: 8px;">
  <img src="assets/logo.png" alt="PMDaemon Logo" style="height: 60px; display: block; margin: 0 auto 10px;">
  <h1 style="margin: 0; color: white;">PMDaemon - Advanced Process Manager</h1>
  <p style="margin: 5px 0 0; color: #cccccc;">A high-performance, cross-platform process manager built in Rust</p>
</div>

[![GitHub Stars](https://img.shields.io/github/stars/entrepeneur4lyf/pmdaemon?style=flat-square)](https://github.com/entrepeneur4lyf/pmdaemon/stargazers)
[![Crates.io Version](https://img.shields.io/crates/v/pmdaemon?style=flat-square)](https://crates.io/crates/pmdaemon)
[![Crates.io Downloads](https://img.shields.io/crates/d/pmdaemon?style=flat-square)](https://crates.io/crates/pmdaemon)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/entrepeneur4lyf/pmdaemon/ci.yml?branch=main&style=flat-square)](https://github.com/entrepeneur4lyf/pmdaemon/actions/workflows/ci.yml?query=branch%3Amain)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![Test Coverage](https://img.shields.io/badge/tests-272%20passing-brightgreen.svg?style=flat-square)](https://github.com/entrepeneur4lyf/pmdaemon)

A high-performance, **cross-platform** process manager built in Rust, inspired by PM2 with innovative features that exceed the original. PMDaemon runs natively on **Linux, Windows, and macOS** and is designed for modern application deployment with advanced port management, real-time monitoring, and production-ready web APIs.

## 📑 Table of Contents

- [Key Features](#-key-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Command Reference](#-command-reference)
- [Configuration Options](#-configuration-options)
- [Web API](#-web-api)
- [Monitoring](#-monitoring)
- [Comparison with PM2](#-comparison-with-pm2)
- [Library Usage](#-library-usage)
- [Testing](#-testing)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [License](#-license)

## The CLI ##
![PMDaemon CLI](assets/cli.png)

## 🚀 Key Features

### Core Process Management
- **Process Lifecycle** - Start, stop, restart, reload, and enhanced delete operations
- **Enhanced Delete Operations** - Bulk deletion, status-based deletion, and safe process shutdown
- **Clustering** - Run multiple instances with automatic load balancing
- **Auto-restart** - Automatic restart on crashes with configurable limits
- **Signal Handling** - Graceful shutdown with SIGTERM/SIGINT and custom signals
- **Configuration Persistence** - Process configs saved and restored between sessions
- **🆕 Ecosystem Config Files** - Manage multiple applications with JSON, YAML, or TOML config files

### Advanced Monitoring & Health Checks
- **Real-time Monitoring** - CPU, memory, uptime tracking with system metrics
- **Memory Limit Enforcement** - Automatic restart when processes exceed memory limits
- **HTTP Health Checks** - Monitor process health via HTTP endpoints with configurable timeouts
- **Script-based Health Checks** - Custom health check scripts for complex validation
- **Blocking Start Command** - Wait for processes to be healthy before returning from start command
- **Log Management** - Separate stdout/stderr files with viewing and following

### 🌟 Innovative Port Management (Beyond PM2)
- **Port Range Distribution** - Automatically distribute consecutive ports to cluster instances
- **Auto-assignment** - Find first available port in specified range
- **Conflict Detection** - Prevent port conflicts at the process manager level
- **Runtime Port Overrides** - Change ports during restart without modifying saved config
- **Port Visibility** - Display assigned ports in process listings

### Web API & Real-time Updates
- **REST API** - Full process management via HTTP with PM2-compatible responses
- **WebSocket Support** - Live process status and system metrics streaming
- **Production Web Server** - Built on Axum with CORS and security headers

### 🌍 Cross-Platform Support
- **Native Windows Support** - Full functionality on Windows with platform-optimized process management
- **Native macOS Support** - Complete support for both Intel and Apple Silicon Macs
- **Native Linux Support** - Optimized for Linux servers and development environments
- **Unified API** - Same commands and features work identically across all platforms
- **Platform-Specific Optimizations** - Tailored signal handling and process termination for each OS

## 📦 Installation

### From Source (All Platforms)
```bash
git clone https://github.com/entrepeneur4lyf/pmdaemon
cd pmdaemon
cargo build --release
```

**Linux/macOS:**
```bash
sudo cp target/release/pmdaemon /usr/local/bin/
```

**Windows:**
```cmd
copy target\release\pmdaemon.exe C:\Windows\System32\
```

### Using Cargo (All Platforms)
```bash
cargo install pmdaemon
```

### Pre-built Binaries
Download platform-specific binaries from [GitHub Releases](https://github.com/entrepeneur4lyf/pmdaemon/releases):
- **Linux**: `pmdaemon-linux-x86_64`
- **Windows**: `pmdaemon-windows-x86_64.exe`
- **macOS Intel**: `pmdaemon-macos-x86_64`
- **macOS Apple Silicon**: `pmdaemon-macos-aarch64`

## 🚀 Quick Start

> **Cross-Platform Note**: All commands below work identically on Linux, Windows, and macOS. PMDaemon automatically handles platform-specific differences internally.

### Basic Process Management
```bash
# Start a process
pmdaemon start app.js --name myapp

# List all processes
pmdaemon list

# Stop a process
pmdaemon stop myapp

# Restart a process
pmdaemon restart myapp

# Delete a process (stops if running)
pmdaemon delete myapp

# Delete all processes
pmdaemon delete all --force

# Delete processes by status
pmdaemon delete stopped --status --force
```

### Clustering with Port Management
```bash
# Start 4 instances with port range
pmdaemon start server.js --instances 4 --port 4000-4003

# Auto-assign ports from range
pmdaemon start worker.js --port auto:5000-5100

# Runtime port override (doesn't modify saved config)
pmdaemon restart myapp --port 3001
```

### Ecosystem Configuration Files
```bash
# Start all apps from config file (JSON, YAML, or TOML)
pmdaemon --config ecosystem.json start

# Start specific app from config file
pmdaemon --config ecosystem.yaml start --name my-web-app

# Example ecosystem.json
{
  "apps": [
    {
      "name": "web-server",
      "script": "node",
      "args": ["server.js"],
      "instances": 2,
      "port": "3000-3001",
      "env": {
        "NODE_ENV": "production"
      }
    }
  ]
}
```

### Memory Limits and Monitoring
```bash
# Set memory limit with auto-restart
pmdaemon start app.js --max-memory 100M

# Real-time monitoring with configurable intervals
pmdaemon monit --interval 2

# View logs
pmdaemon logs myapp

# Follow logs in real-time
pmdaemon logs myapp --follow
```

### Health Checks and Blocking Start
```bash
# Start with HTTP health check and wait for ready
pmdaemon start app.js --health-check-url http://localhost:9615/health --wait-ready

# Start with script-based health check
pmdaemon start worker.js --health-check-script ./health-check.sh --wait-ready

# Custom health check timeout
pmdaemon start api.js --health-check-url http://localhost:9615/status --wait-timeout 30s
```

### Web API Server
```bash
# Start web API server for remote monitoring (no authentication)
pmdaemon web --port 9615 --host 127.0.0.1

# Start with API key authentication (recommended for production)
pmdaemon web --api-key "your-secret-api-key"
```

## 📋 Command Reference

| Command     | Description                | Example                                |
|-------------|----------------------------|----------------------------------------|
| `start`     | Start a new process        | `pmdaemon start app.js --name myapp`  |
|             | Start from config file     | `pmdaemon --config ecosystem.json start` |
| `stop`      | Stop a process             | `pmdaemon stop myapp`                 |
| `restart`   | Restart a process          | `pmdaemon restart myapp`              |
| `reload`    | Graceful restart           | `pmdaemon reload myapp`               |
| `delete`    | Delete process(es)         | `pmdaemon delete myapp`               |
|             | Delete all processes       | `pmdaemon delete all --force`         |
|             | Delete by status           | `pmdaemon delete stopped --status`   |
| `list`      | List all processes         | `pmdaemon list`                       |
| `monit`     | Real-time monitoring       | `pmdaemon monit --interval 2`         |
| `logs`      | View/follow process logs   | `pmdaemon logs myapp --follow`        |
| `info`      | Process details            | `pmdaemon info myapp`                 |
| `web`       | Start web API server       | `pmdaemon web --port 9615`            |

## 🔧 Configuration Options

### Ecosystem Configuration Files

PMDaemon supports ecosystem configuration files in JSON, YAML, and TOML formats for managing multiple applications:

```bash
# Use ecosystem config file
pmdaemon --config ecosystem.json start

# Start specific app from config
pmdaemon --config ecosystem.yaml start --name web-server
```

**Example ecosystem.json:**
```json
{
  "apps": [
    {
      "name": "web-server",
      "script": "node",
      "args": ["server.js"],
      "instances": 2,
      "port": "3000-3001",
      "env": {
        "NODE_ENV": "production"
      },
      "health_check_url": "http://localhost:3000/health"
    },
    {
      "name": "api-service",
      "script": "python",
      "args": ["api.py"],
      "cwd": "/path/to/api",
      "max_memory_restart": "512M"
    }
  ]
}
```

**Supported config formats:**
- `ecosystem.json` - JSON format
- `ecosystem.yaml` / `ecosystem.yml` - YAML format
- `ecosystem.toml` - TOML format

See [CONFIG_USAGE.md](examples/CONFIG_USAGE.md) for detailed ecosystem configuration documentation.

### Process Configuration
```bash
pmdaemon start app.js \
  --name "my-app" \
  --instances 4 \
  --port 3000-3003 \
  --max-memory 512M \
  --env NODE_ENV=production \
  --cwd /path/to/app \
  --log-file /var/log/app.log \
  --health-check-url http://localhost:9615/health \
  --wait-ready
```

### Port Management Options

| Option                      | Description                            | Example                    |
|-----------------------------|----------------------------------------|----------------------------|
| `--port 3000`               | Single port assignment                 | Assigns port 3000          |
| `--port 3000-3005`          | Port range for clusters                | Distributes 3000-3005      |
| `--port auto:4000-4100`     | Auto-find available port               | First available in range   |

### Health Check Options

| Option                           | Description                            | Example                         |
|----------------------------------|----------------------------------------|---------------------------------|
| `--health-check-url <url>`       | HTTP endpoint for health checks        | `http://localhost:9615/health` |
| `--health-check-script <path>`   | Script to run for health validation    | `./scripts/health-check.sh`     |
| `--health-check-timeout <time>`  | Timeout for individual health checks   | `5s`, `30s`, `1m`               |
| `--health-check-interval <time>` | Interval between health checks         | `10s`, `30s`, `1m`              |
| `--health-check-retries <num>`   | Number of retries before failure       | `3`, `5`, `10`                  |
| `--wait-ready`                   | Block start until health checks pass   | Boolean flag                    |
| `--wait-timeout <time>`          | Timeout for blocking start             | `30s`, `1m`, `5m`               |

### Delete Options

| Option                      | Description                            | Example                         |
|-----------------------------|----------------------------------------|---------------------------------|
| `delete <name>`             | Delete single process by name/ID       | `pmdaemon delete myapp`         |
| `delete all`                | Delete all processes                   | `pmdaemon delete all`           |
| `delete <status> --status`  | Delete processes by status             | `pmdaemon delete stopped --status` |
| `--force` / `-f`            | Skip confirmation prompts             | `pmdaemon delete all --force`   |

**Valid statuses for `--status` flag:**
- `starting` - Processes currently starting up
- `online` - Running processes
- `stopping` - Processes currently shutting down
- `stopped` - Processes that have exited
- `errored` - Processes that crashed or failed
- `restarting` - Processes currently restarting

**Safety Features:**
- All delete operations automatically stop running processes before deletion
- Interactive confirmation prompts for bulk operations (unless `--force` is used)
- Graceful process shutdown with proper cleanup of files and ports
- Clear feedback showing how many processes were stopped vs. deleted

## 🌐 Web API

PMDaemon provides a comprehensive REST API with optional authentication:

### Security Features
- **API Key Authentication** - Secure your API with keys
- **Process Management Only** - Cannot create new processes for security
- **Production Ready** - Built with security best practices

### Endpoints

| Method   | Endpoint                    | Description                  | Auth Required |
|----------|----------------------------|------------------------------|---------------|
| `GET`    | `/api/processes`           | List all processes           | ✅            |
| `POST`   | `/api/processes/:id/start` | Start existing process       | ✅            |
| `DELETE` | `/api/processes/:id`       | Stop/delete a process        | ✅            |
| `GET`    | `/api/system`              | System metrics               | ✅            |
| `GET`    | `/api/logs/:id`            | Process logs                 | ✅            |
| `WS`     | `/ws`                      | Real-time updates            | ❌            |

### Example API Usage
```bash
# Start with authentication
pmdaemon web --api-key "your-secret-key"

# List processes (with API key)
curl -H "Authorization: Bearer your-secret-key" \
     http://localhost:9615/api/processes

# Start an existing process (processes created via CLI only)
curl -X POST \
     -H "Authorization: Bearer your-secret-key" \
     http://localhost:9615/api/processes/my-app/start

# WebSocket for real-time updates
wscat -c ws://localhost:9615/ws
```

## 📊 Monitoring

PMDaemon provides comprehensive monitoring capabilities:

### Real-time Metrics
- CPU usage percentage
- Memory usage (RSS)
- Process uptime
- Restart count
- Port assignments
- Process state
- Process ID (PID) for debugging

### Configurable Monitoring
- **Configurable update intervals** - Customize refresh rates (1s, 2s, 5s, etc.)
- **Beautiful table formatting** - Professional display using comfy-table
- **Color-coded status indicators** - Visual process state identification
- **System overview** - CPU, memory, load average, and uptime

### Log Management
- Separate stdout/stderr files
- **Real-time log following** - `tail -f` functionality for live log monitoring
- **Configurable log retrieval** - Get last N lines from log files
- **Missing file handling** - Graceful handling of non-existent log files
- HTTP log access via API

## 🆚 PMDaemon vs PM2

| Feature                          | PMDaemon | PM2 |
|----------------------------------|:--------:|:---:|
| Port range distribution          |    ✅    |  ❌  |
| Auto port assignment             |    ✅    |  ❌  |
| Runtime port override            |    ✅    |  ❌  |
| Built-in port conflict detection |    ✅    |  ❌  |
| HTTP health checks               |    ✅    |  ❌  |
| Script-based health checks       |    ✅    |  ❌  |
| Blocking start command           |    ✅    |  ❌  |
| **Configurable monitor intervals** |    ✅    |  ❌  |
| **Real-time log following**      |    ✅    |  ❌  |
| **Professional table formatting** |    ✅    |  ❌  |
| **PID display in monitoring**    |    ✅    |  ❌  |
| **Enhanced delete operations**   |    ✅    |  ❌  |
| **Bulk deletion (delete all)**   |    ✅    |  ❌  |
| **Status-based deletion**        |    ✅    |  ❌  |
| **Safe process shutdown**        |    ✅    |  ❌  |
| Memory limit enforcement         |    ✅    |  ✅  |
| WebSocket real-time updates      |    ✅    |  ❌  |
| **Native Windows support**       |    ✅    |  ❌  |
| **Native macOS support**         |    ✅    |  ❌  |
| **Cross-platform compatibility** |    ✅    |  ❌  |
| Rust performance                 |    ✅    |  ❌  |
| PM2-compatible API               |    ✅    |  ✅  |

## Process Flowchart

```mermaid
graph TD

    User["User<br>External Actor"]
    subgraph PMDaemon["PMDaemon Application<br>Rust"]
        CLI["CLI Entry Point<br>Rust"]

        subgraph WebLayer["Web Layer"]
            WebAPI["Web API Server<br>Axum / HTTP"]
            Auth["API Key Authentication<br>Middleware"]
            WebSocket["WebSocket Handler<br>Real-time Updates"]
        end

        subgraph CoreLayer["Core Management Layer"]
            ProcessManager["Process Manager<br>Core Orchestrator"]
            PortManager["Port Manager<br>Allocation & Conflicts"]
            ConfigService["Configuration Service<br>JSON/YAML/TOML"]
        end

        subgraph MonitoringLayer["Monitoring & Health Layer"]
            HealthMonitor["Health Monitor<br>HTTP & Script Checks"]
            SystemMonitor["System Monitor<br>CPU/Memory/Load"]
            ProcessMonitor["Process Monitor<br>Individual Process Metrics"]
        end

        subgraph ProcessLayer["Process Execution Layer"]
            ProcessExecution["Process Execution<br>Spawn & Control"]
            SignalHandler["Signal Handler<br>OS Signal Management"]
            LogManager["Log Manager<br>stdout/stderr Capture"]
        end

        %% Web Layer Connections
        WebAPI -->|Authenticates via| Auth
        Auth -->|Authorized requests to| ProcessManager
        WebAPI -->|Real-time updates via| WebSocket
        WebSocket -->|Broadcasts from| SystemMonitor

        %% CLI Connections
        CLI -->|Direct commands to| ProcessManager

        %% Core Layer Connections
        ProcessManager -->|Manages ports via| PortManager
        ProcessManager -->|Loads/saves config via| ConfigService
        ProcessManager -->|Controls processes via| ProcessExecution
        ProcessManager -->|Coordinates monitoring via| HealthMonitor
        ProcessManager -->|Gets metrics from| SystemMonitor

        %% Monitoring Connections
        HealthMonitor -->|Checks health of| ProcessExecution
        ProcessMonitor -->|Monitors individual| ProcessExecution
        SystemMonitor -->|Aggregates data from| ProcessMonitor

        %% Process Layer Connections
        ProcessExecution -->|Handles signals via| SignalHandler
        ProcessExecution -->|Captures logs via| LogManager
        ProcessExecution -->|Uses config from| ConfigService
        ProcessExecution -->|Reports to| ProcessMonitor
    end

    %% External Connections
    User -->|CLI Commands| CLI
    User -->|HTTP/WebSocket| WebAPI

    %% External Systems
    ProcessExecution -->|Spawns & Controls| SystemProcesses["System Processes<br>Managed Applications"]
    ConfigService -->|Persists to| ConfigFiles["Config Files<br>JSON/YAML/TOML"]
    LogManager -->|Writes to| LogFiles["Log Files<br>stdout/stderr"]
```

## 🔧 Library Usage

PMDaemon can also be used as a Rust library:

```rust
use pmdaemon::{ProcessManager, ProcessConfig, config::PortConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ProcessManager::new().await?;

    let config = ProcessConfig::builder()
        .name("web-cluster")
        .script("node")
        .args(vec!["app.js"])
        .instances(4)
        .port(PortConfig::Range(3000, 3003))
        .max_memory_restart(512 * 1024 * 1024) // 512MB
        .build()?;

    manager.start(config).await?;
    println!("Started 4-instance cluster on ports 3000-3003");

    Ok(())
}
```

## 🧪 Testing

PMDaemon has comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run with coverage
cargo test --all-features

# Run documentation tests
cargo test --doc

# Run integration tests
cargo test --test integration_tests

# Run end-to-end tests
cargo test --test e2e_tests
```

### Test Coverage
- **272 Total Tests**
  - 150 Unit tests (library)
  - 33 Unit tests (binary CLI)
  - 13 Integration tests (including config file functionality tests)
  - 8 End-to-end tests
  - 8 Configuration format tests
  - 60 Documentation tests

## 🗺️ Roadmap

### Completed Features ✅
- ✅ Core process management (Phase 1-3)
- ✅ CLI interface with all PM2-compatible commands (Phase 5)
- ✅ Advanced monitoring and logging (Phase 6)
- ✅ Web API and WebSocket support (Phase 7)
- ✅ Health checks and blocking start (Phase 8)
- ✅ Enhanced delete operations with bulk and status-based deletion
- ✅ Safe process shutdown and lifecycle management
- ✅ **Cross-platform support** - Native Windows, macOS, and Linux compatibility
- ✅ **Platform-specific optimizations** - Tailored signal handling for each OS
- ✅ Comprehensive test suite (Phase 10.1-10.3)

### In Progress 🚧
- 📝 Integration examples and documentation (Phase 9)
- 📝 API documentation beyond docs.rs
- 📝 Performance benchmarks vs PM2

### Future Enhancements 🔮
- 🎨 **v2.0** - Enhanced CLI with [ratatui](https://github.com/ratatui-org/ratatui) for interactive terminal UI
- 📊 Advanced metrics visualization
- 🔌 Plugin system for custom monitors
- 🌍 Distributed process management

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Guidelines
- Follow Rust best practices and idioms
- Add tests for new features
- Update documentation as needed
- Ensure all tests pass before submitting PR

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [PM2](https://pm2.keymetrics.io/) - The original Node.js process manager
- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Uses [Tokio](https://tokio.rs/) for async runtime
- Web server powered by [Axum](https://github.com/tokio-rs/axum)

---

**PMDaemon** - Process management, evolved. 🚀
