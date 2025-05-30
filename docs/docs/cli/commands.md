# CLI Commands Reference

PMDaemon provides a comprehensive command-line interface with PM2-compatible commands plus enhanced features. This reference covers all available commands with examples and options.

## Global Options

These options work with all commands:

```bash
pmdaemon [GLOBAL_OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

| Option | Description | Example |
|--------|-------------|---------|
| `--verbose`, `-v` | Enable verbose logging | `pmdaemon --verbose start app.js` |
| `--config`, `-c` | Configuration file path | `pmdaemon --config ecosystem.json start` |
| `--help`, `-h` | Show help information | `pmdaemon --help` |
| `--version`, `-V` | Show version information | `pmdaemon --version` |

## Process Management Commands

### `start` - Start a Process

Start a new process with optional configuration.

```bash
pmdaemon start [SCRIPT] [OPTIONS]
```

#### Basic Usage

```bash
# Start a simple process
pmdaemon start "node server.js" --name web-app

# Start from config file
pmdaemon --config ecosystem.json start

# Start specific app from config
pmdaemon --config ecosystem.json start --name api-server
```

#### Options

| Option | Type | Description | Example |
|--------|------|-------------|---------|
| `--name`, `-n` | String | Process name (required) | `--name web-app` |
| `--instances`, `-i` | Number | Number of instances | `--instances 4` |
| `--port`, `-p` | String | Port configuration | `--port 3000-3003` |
| `--cwd` | Path | Working directory | `--cwd /path/to/app` |
| `--env`, `-e` | String | Environment variables | `--env NODE_ENV=production` |
| `--max-memory` | String | Memory limit | `--max-memory 512M` |
| `--max-restarts` | Number | Maximum restart attempts | `--max-restarts 10` |
| `--min-uptime` | Duration | Minimum uptime before stable | `--min-uptime 5s` |
| `--restart-delay` | Duration | Delay between restarts | `--restart-delay 2s` |
| `--kill-timeout` | Duration | Graceful shutdown timeout | `--kill-timeout 30s` |

| `--wait-ready` | Flag | Block until process is healthy | `--wait-ready` |
| `--wait-timeout` | Duration | Timeout for blocking start | `--wait-timeout 60s` |
| `--out-file` | Path | Output log file | `--out-file /var/log/app.out` |
| `--error-file` | Path | Error log file | `--error-file /var/log/app.err` |
| `--log-file` | Path | Combined log file | `--log-file /var/log/app.log` |
| `--pid-file` | Path | PID file location | `--pid-file /var/run/app.pid` |

#### Advanced Examples

```bash
# Production cluster with health checks
pmdaemon start "node api.js" \
  --name prod-api \
  --instances 4 \
  --port auto:3000-3100 \
  --max-memory 1G \
  --wait-ready \
  --env NODE_ENV=production

# Python microservice
pmdaemon start "python -m uvicorn main:app" \
  --name python-api \
  --port 8000 \
  --max-memory 512M \
  --cwd /path/to/api

# Background worker with custom restart behavior
pmdaemon start "python worker.py" \
  --name background-worker \
  --max-restarts 5 \
  --min-uptime 10s \
  --restart-delay 5s \
  --max-memory 256M
```

### `stop` - Stop a Process

Stop one or more running processes.

```bash
pmdaemon stop <IDENTIFIER>
```

#### Examples

```bash
# Stop by name
pmdaemon stop web-app

# Stop by ID
pmdaemon stop 0

# Stop all processes
pmdaemon stop all
```

### `restart` - Restart a Process

Restart a process with optional port override.

```bash
pmdaemon restart <IDENTIFIER> [OPTIONS]
```

#### Options

| Option | Type | Description | Example |
|--------|------|-------------|---------|
| `--port`, `-p` | String | Override port configuration | `--port 3001` |

#### Examples

```bash
# Standard restart
pmdaemon restart web-app

# Restart with new port (doesn't modify saved config)
pmdaemon restart web-app --port 3001

# Restart with port range for clustering
pmdaemon restart web-app --port 4000-4003

# Restart all processes
pmdaemon restart all
```

### `reload` - Graceful Reload

Perform zero-downtime restart using graceful reload.

```bash
pmdaemon reload <IDENTIFIER> [OPTIONS]
```

#### Options

| Option | Type | Description | Example |
|--------|------|-------------|---------|
| `--port`, `-p` | String | Override port configuration | `--port 3001` |

#### Examples

```bash
# Graceful reload
pmdaemon reload web-app

# Reload with new port configuration
pmdaemon reload web-app --port 4000-4003
```

### `delete` - Delete Processes

Delete processes with enhanced bulk and status-based operations.

```bash
pmdaemon delete <IDENTIFIER> [OPTIONS]
```

#### Options

| Option | Type | Description | Example |
|--------|------|-------------|---------|
| `--status` | Flag | Delete by status | `--status` |
| `--force`, `-f` | Flag | Skip confirmation prompts | `--force` |

#### Examples

```bash
# Delete single process
pmdaemon delete web-app

# Delete all processes
pmdaemon delete all --force

# Delete by status
pmdaemon delete stopped --status
pmdaemon delete errored --status --force
pmdaemon delete starting --status

# Interactive deletion (with confirmation)
pmdaemon delete all
```

#### Valid Status Values

- `starting` - Processes currently starting up
- `online` - Running processes  
- `stopping` - Processes currently shutting down
- `stopped` - Processes that have exited
- `errored` - Processes that crashed or failed
- `restarting` - Processes currently restarting

## Monitoring Commands

### `list` - List Processes

Display all processes with detailed information.

```bash
pmdaemon list
```

#### Output Columns

- **ID** - Process identifier
- **Name** - Process name
- **Status** - Current state with color coding
- **PID** - System process ID
- **Port** - Assigned port(s)
- **CPU** - CPU usage percentage
- **Memory** - Memory usage (RSS)
- **Uptime** - How long the process has been running
- **Restarts** - Number of restarts

#### Example Output

```
┌────┬─────────────┬────────┬───────┬──────┬─────────┬──────────┬─────────┬──────────┐
│ ID │ Name        │ Status │ PID   │ Port │ CPU (%) │ Memory   │ Uptime  │ Restarts │
├────┼─────────────┼────────┼───────┼──────┼─────────┼──────────┼─────────┼──────────┤
│ 0  │ web-app     │ 🟢 Online │ 1234  │ 3000 │ 2.5     │ 45.2 MB  │ 2h 15m  │ 0        │
│ 1  │ api-server  │ 🟢 Online │ 1235  │ 8000 │ 1.8     │ 32.1 MB  │ 1h 45m  │ 1        │
│ 2  │ worker      │ 🔴 Stopped │ -     │ -    │ -       │ -        │ -       │ 3        │
└────┴─────────────┴────────┴───────┴──────┴─────────┴──────────┴─────────┴──────────┘
```

### `monit` - Real-time Monitoring

Display real-time process monitoring with system metrics.

```bash
pmdaemon monit [OPTIONS]
```

#### Options

| Option | Type | Description | Example |
|--------|------|-------------|---------|
| `--interval`, `-i` | Number | Update interval in seconds | `--interval 5` |

#### Examples

```bash
# Default monitoring (1-second updates)
pmdaemon monit

# Custom update interval
pmdaemon monit --interval 5

# Fast updates for debugging
pmdaemon monit --interval 0.5
```

#### Features

- **Real-time updates** with configurable intervals
- **Color-coded status indicators**
- **System overview** (CPU, memory, load average)
- **Process metrics** (CPU, memory, uptime)
- **Beautiful table formatting**
- **Keyboard shortcuts** (Ctrl+C to exit)

### `logs` - View Process Logs

View and follow process logs with filtering options.

```bash
pmdaemon logs <IDENTIFIER> [OPTIONS]
```

#### Options

| Option | Type | Description | Example |
|--------|------|-------------|---------|
| `--lines`, `-n` | Number | Number of lines to show | `--lines 100` |
| `--follow`, `-f` | Flag | Follow logs in real-time | `--follow` |
| `--error` | Flag | Show only error logs | `--error` |

#### Examples

```bash
# View recent logs (default: 20 lines)
pmdaemon logs web-app

# View more lines
pmdaemon logs web-app --lines 100

# Follow logs in real-time
pmdaemon logs web-app --follow

# View only error logs
pmdaemon logs web-app --error

# Follow error logs
pmdaemon logs web-app --error --follow
```

### `info` - Process Information

Get detailed information about a specific process.

```bash
pmdaemon info <IDENTIFIER>
```

#### Example Output

```json
{
  "id": 0,
  "name": "web-app",
  "status": "online",
  "pid": 1234,
  "port": "3000",
  "cpu_usage": 2.5,
  "memory_usage": 47448064,
  "uptime": "2h 15m 30s",
  "restarts": 0,
  "config": {
    "script": "node",
    "args": ["server.js"],
    "instances": 1,
    "max_memory_restart": 536870912,
    "env": {
      "NODE_ENV": "production",
      "PORT": "3000"
    }
  },
  "health_check": {
    "enabled": true,
    "status": "healthy",
    "last_check": "2024-01-15T10:30:00Z"
  }
}
```

## Web API Commands

### `web` - Start Web Server

Start the web API server for remote monitoring and management.

```bash
pmdaemon web [OPTIONS]
```

#### Options

| Option | Type | Description | Default | Example |
|--------|------|-------------|---------|---------|
| `--port`, `-p` | Number | Web server port | 9615 | `--port 8080` |
| `--host`, `-h` | String | Bind address | 127.0.0.1 | `--host 0.0.0.0` |
| `--api-key` | String | API key for authentication | None | `--api-key "secret123"` |

#### Examples

```bash
# Start with default settings (no authentication)
pmdaemon web

# With API key authentication (recommended for production)
pmdaemon web --api-key "your-secret-api-key"

# Custom port and host with authentication
pmdaemon web --port 8080 --host 0.0.0.0 --api-key "$API_KEY"

# Environment variable for API key
export PMDAEMON_API_KEY="your-secret-key"
pmdaemon web --host 0.0.0.0
```

#### Available Endpoints

Once started, the web server provides:

- **REST API** at `http://host:port/api/`
- **WebSocket** at `ws://host:port/ws`
- **API documentation** at `http://host:port/`

## Configuration Commands

### Using Configuration Files

PMDaemon supports ecosystem configuration files in multiple formats:

```bash
# JSON format
pmdaemon --config ecosystem.json start

# YAML format  
pmdaemon --config ecosystem.yaml start

# TOML format
pmdaemon --config ecosystem.toml start
```

#### Start All Apps from Config

```bash
# Start all apps defined in config file
pmdaemon --config ecosystem.json start
```

#### Start Specific App from Config

```bash
# Start only the specified app
pmdaemon --config ecosystem.json start --name web-server
```

## Exit Codes

PMDaemon uses standard exit codes for scripting:

| Code | Meaning | Description |
|------|---------|-------------|
| 0 | Success | Command completed successfully |
| 1 | General Error | Command failed due to an error |
| 2 | Invalid Usage | Invalid command line arguments |
| 3 | Process Not Found | Specified process doesn't exist |
| 4 | Configuration Error | Invalid configuration file or options |
| 5 | Permission Error | Insufficient permissions |
| 130 | Interrupted | Command interrupted by user (Ctrl+C) |

## Command Aliases

Some commands have shorter aliases for convenience:

| Full Command | Alias | Example |
|--------------|-------|---------|
| `pmdaemon start` | `pmdaemon s` | `pmdaemon s app.js --name web` |
| `pmdaemon stop` | `pmdaemon st` | `pmdaemon st web` |
| `pmdaemon restart` | `pmdaemon r` | `pmdaemon r web` |
| `pmdaemon list` | `pmdaemon ls` | `pmdaemon ls` |
| `pmdaemon delete` | `pmdaemon del` | `pmdaemon del web` |

## Best Practices

### 1. Use Descriptive Names

```bash
# Good
pmdaemon start "node api.js" --name api-server-prod

# Avoid
pmdaemon start "node app.js" --name app
```

### 2. Set Resource Limits

```bash
pmdaemon start "node server.js" \
  --name web-app \
  --max-memory 1G \
  --max-restarts 10
```

### 3. Use Process Management for Critical Services

```bash
pmdaemon start "node api.js" \
  --name critical-api \
  --max-restarts 10 \
  --wait-ready
```

### 4. Use Configuration Files for Complex Setups

```bash
# Better than long command lines
pmdaemon --config production.json start
```

## Next Steps

- **[Configuration Options](./configuration-options.md)** - Detailed configuration reference
- **[Environment Variables](./environment-variables.md)** - Environment variable usage
- **[Examples](../examples/use-cases.md)** - Real-world usage examples

---

This CLI reference provides comprehensive coverage of all PMDaemon commands. For specific use cases and advanced configurations, see the examples and feature documentation.
