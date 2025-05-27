# PMDaemon Configuration Examples

This directory contains comprehensive examples demonstrating all available configuration options for PMDaemon, including their default values.

## Files Overview

### Shell Scripts
- **`comprehensive_config_examples.sh`** - Bash/Linux shell script with all CLI examples
- **`comprehensive_config_examples.ps1`** - PowerShell script with Windows-specific examples
- **`comprehensive_config_examples.bat`** - Windows batch file with CMD examples

### Configuration Files
- **Note**: PMDaemon currently does NOT support ecosystem config files like PM2
- The `--config` CLI option exists but is not yet implemented
- PMDaemon only saves individual process configs internally as JSON

### Existing Examples
- **`demo_delete_features.sh`** - Demonstrates delete command functionality
- **`health_check_demo.rs`** - Rust example for health check implementation
- **`test_delete_functionality.sh`** - Test script for delete operations

## Default Values Reference

| Option | Default Value | Description |
|--------|---------------|-------------|
| `instances` | `1` | Number of process instances |
| `autorestart` | `true` | Automatically restart on crash |
| `max_restarts` | `16` | Maximum restart attempts |
| `min_uptime` | `1000ms` | Minimum uptime before considering stable |
| `restart_delay` | `0ms` | Delay between exit and restart |
| `kill_timeout` | `1600ms` | Time to wait for graceful shutdown |
| `exec_mode` | `fork` | Execution mode (auto-cluster when instances > 1) |
| `namespace` | `"default"` | Process namespace for grouping |
| `watch` | `false` | File watching (not yet implemented) |
| `web_port` | `9615` | Web monitoring server port |
| `web_host` | `127.0.0.1` | Web monitoring server host |
| `monitor_interval` | `1` second | Real-time monitoring update interval |
| `log_lines` | `20` | Default number of log lines to show |
| `max_memory_restart` | `none` | Memory limit before restart (unlimited) |
| `port` | `none` | Port assignment (no port by default) |
| `cwd` | current directory | Working directory |
| `env` | empty | Environment variables (inherits from parent) |

## Configuration Options

### Basic Options
- **`name`** - Process name (required, must be unique)
- **`script`** - Command or script to execute (required)
- **`args`** - Command line arguments
- **`instances`** - Number of instances (enables cluster mode when > 1)
- **`cwd`** - Working directory

### Environment & Resources
- **`env`** - Environment variables (KEY=VALUE format)
- **`max_memory`** - Memory limit (100K, 512M, 1G, etc.)

### Port Management
- **`port`** - Port configuration:
  - Single port: `3000`
  - Port range: `3000-3003`
  - Auto assignment: `auto:4000-4100`

### Process Control
- **`autorestart`** - Auto restart on crash
- **`max_restarts`** - Maximum restart attempts
- **`min_uptime`** - Minimum uptime before stable
- **`restart_delay`** - Delay between restarts
- **`kill_timeout`** - Graceful shutdown timeout

### Logging & Monitoring
- **`out_file`** - Output log file path
- **`error_file`** - Error log file path
- **`log_file`** - Combined log file path
- **`pid_file`** - PID file path

### Health Checks
- **`health_check`** - Health check configuration:
  - HTTP checks: Monitor HTTP endpoints
  - Script checks: Run custom health check scripts

### Advanced Options
- **`namespace`** - Logical process grouping
- **`watch`** - File watching (not yet implemented)
- **`ignore_watch`** - Files to ignore when watching
- **`user`** - User to run as (not yet implemented)
- **`group`** - Group to run as (not yet implemented)

## CLI Commands

### Process Management
```bash
# Start processes
pmdaemon start <script> [options]

# Stop processes
pmdaemon stop <name|id>

# Restart processes
pmdaemon restart <name|id> [--port <port>]

# Graceful reload
pmdaemon reload <name|id> [--port <port>]

# Delete processes
pmdaemon delete <name|id|status|all> [--status] [--force]
```

### Monitoring
```bash
# List all processes
pmdaemon list

# Real-time monitoring
pmdaemon monit [--interval <seconds>]

# View logs
pmdaemon logs <name|id> [--lines <count>] [--follow]

# Process information
pmdaemon info <name|id>

# Web monitoring server
pmdaemon web [--port <port>] [--host <host>]
```

### Global Options
```bash
# Verbose logging
pmdaemon --verbose <command>

# Custom config file
pmdaemon --config <path> <command>
```

## Memory Format Examples

PMDaemon supports various memory format specifications:

- **Kilobytes**: `100K`, `100KB`
- **Megabytes**: `512M`, `512MB`
- **Gigabytes**: `2G`, `2GB`
- **Bytes**: `1073741824` (raw bytes)

## Port Configuration Examples

### Single Port
```bash
pmdaemon start --port 3000 node server.js
```

### Port Range (Cluster Mode)
```bash
pmdaemon start --instances 4 --port 3000-3003 node server.js
```

### Auto Port Assignment
```bash
pmdaemon start --instances 3 --port auto:4000-4100 node server.js
```

## Environment Variables

PMDaemon automatically adds these environment variables:
- **`PORT`** - Assigned port number
- **`PM2_INSTANCE_ID`** - Instance ID (0-based)
- **`NODE_APP_INSTANCE`** - Instance ID (for Node.js compatibility)

## Health Check Examples

### HTTP Health Check
```json
{
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 30,
    "interval": 60,
    "retries": 3,
    "enabled": true
  }
}
```

### Script Health Check
```json
{
  "health_check": {
    "check_type": "script",
    "script": "./health_check.sh",
    "timeout": 10,
    "interval": 30,
    "retries": 2,
    "enabled": true
  }
}
```

## Platform-Specific Notes

### Linux/macOS (Bash)
- Use backslashes (`\`) for line continuation
- Use single or double quotes for paths with spaces
- Environment variables: `$VAR` or `${VAR}`

### Windows (PowerShell)
- Use backticks (`` ` ``) for line continuation
- Use double quotes for paths with spaces
- Environment variables: `$env:VAR`

### Windows (Batch)
- Use caret (`^`) for line continuation
- Use double quotes for paths with spaces
- Environment variables: `%VAR%`

## Usage Examples

### Development Server
```bash
pmdaemon start --name dev-server --env NODE_ENV=development node server.js
```

### Production Cluster
```bash
pmdaemon start \
  --name prod-api \
  --instances 4 \
  --port auto:3000-3100 \
  --max-memory 1G \
  --env NODE_ENV=production \
  node server.js
```

### Python API
```bash
pmdaemon start \
  --name python-api \
  --instances 2 \
  --port auto:8000-8100 \
  --max-memory 512M \
  python -m uvicorn main:app --host 0.0.0.0
```

### Static File Server
```bash
pmdaemon start --name static --port 8080 python -m http.server 8080
```

## Configuration File Support Status

**Important**: PMDaemon currently does NOT support ecosystem configuration files like PM2's `ecosystem.config.js`.

### What's Currently Supported:
- **CLI-only configuration**: All options must be passed via command line
- **Internal JSON storage**: PMDaemon saves individual process configs as JSON files internally
- **Process persistence**: Started processes are automatically restored on restart

### What's NOT Supported (Yet):
- **Ecosystem config files**: No support for `ecosystem.json` or `ecosystem.yaml`
- **`--config` option**: The CLI option exists but is not implemented
- **Bulk process definition**: Cannot define multiple apps in a single file

### Current Workflow:
```bash
# Start processes individually with CLI options
pmdaemon start --name web-app --instances 2 --port 3000-3001 --env NODE_ENV=production node server.js
pmdaemon start --name api-service --port 4000 --max-memory 512M python -m uvicorn main:app

# Processes are automatically saved and restored
pmdaemon list  # Shows all running processes
```

## Getting Started

### For CLI Usage:
1. Choose the appropriate shell script for your platform
2. Review the configuration options and defaults
3. Modify the examples to match your application requirements
4. Test with a simple application first

### Current Limitations:
1. **No ecosystem config files**: Must use CLI for each process
2. **No bulk operations**: Start processes one by one
3. **No config file templates**: Use the shell script examples instead

### General:
1. Test with a simple application first
2. Scale up to production configurations
3. Use the monitoring commands to verify everything works

For more information, see the main PMDaemon documentation.
