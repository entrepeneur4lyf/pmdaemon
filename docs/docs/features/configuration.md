# Configuration

PMDaemon provides flexible **configuration options** that allow you to customize every aspect of process management. From basic process settings to advanced clustering and health checks, PMDaemon's configuration system is designed for both simplicity and power.

## Overview

PMDaemon supports configuration through:

- **🖥️ CLI arguments** - Direct command-line configuration
- **📄 Configuration files** - JSON, YAML, and TOML formats
- **🔧 Environment variables** - System-level configuration
- **⚙️ Runtime overrides** - Temporary configuration changes
- **🎯 Default values** - Sensible defaults for all options

## Configuration Methods

### 1. CLI Arguments (Primary)

The most common way to configure processes:

```bash
pmdaemon start "node server.js" \
  --name web-api \
  --instances 2 \
  --port 3000-3001 \
  --max-memory 512M \
  --env NODE_ENV=production \
  --health-check-url http://localhost:3000/health \
  --wait-ready
```

### 2. Configuration Files

Define complex configurations in files:

**ecosystem.json:**
```json
{
  "apps": [
    {
      "name": "web-api",
      "script": "node",
      "args": ["server.js"],
      "instances": 2,
      "port": "3000-3001",
      "max_memory_restart": "512M",
      "env": {
        "NODE_ENV": "production"
      },
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health",
        "timeout": 5,
        "interval": 30,
        "retries": 3,
        "enabled": true
      }
    }
  ]
}
```

**ecosystem.yaml:**
```yaml
apps:
  - name: web-api
    script: node
    args: [server.js]
    instances: 2
    port: "3000-3001"
    max_memory_restart: 512M
    env:
      NODE_ENV: production
    health_check:
      check_type: http
      url: http://localhost:3000/health
      timeout: 5
      interval: 30
      retries: 3
      enabled: true
```

### 3. Environment Variables

System-level configuration:

```bash
# Set default values
export PMDAEMON_DEFAULT_INSTANCES=2
export PMDAEMON_DEFAULT_MAX_MEMORY=512M
export PMDAEMON_LOG_LEVEL=info

# Use in commands
pmdaemon start "node server.js" --name web-api
```

## Core Configuration Options

### Basic Process Settings

| Option | CLI Flag | Default | Description |
|--------|----------|---------|-------------|
| **Name** | `--name` | Required | Unique process identifier |
| **Script** | First argument | Required | Command or script to execute |
| **Arguments** | `--args` | `[]` | Command line arguments |
| **Working Directory** | `--cwd` | Current dir | Process working directory |
| **Instances** | `--instances` | `1` | Number of process instances |

```bash
# Basic configuration
pmdaemon start "python app.py" \
  --name python-api \
  --args "--port 8000 --workers 4" \
  --cwd /app \
  --instances 2
```

### Environment Variables

| Option | CLI Flag | Default | Description |
|--------|----------|---------|-------------|
| **Environment** | `--env` | Inherited | Environment variables |
| **Environment File** | `--env-file` | None | Load environment from file |

```bash
# Set environment variables
pmdaemon start "node server.js" \
  --name web-api \
  --env NODE_ENV=production \
  --env DATABASE_URL=postgres://localhost/mydb \
  --env-file .env.production
```

### Resource Limits

| Option | CLI Flag | Default | Description |
|--------|----------|---------|-------------|
| **Memory Limit** | `--max-memory` | Unlimited | Memory limit before restart |
| **CPU Limit** | `--max-cpu` | Unlimited | CPU limit (future feature) |

```bash
# Set resource limits
pmdaemon start "node server.js" \
  --name web-api \
  --max-memory 1G
```

**Memory format examples:**
- `100K` or `100KB` - Kilobytes
- `512M` or `512MB` - Megabytes
- `2G` or `2GB` - Gigabytes
- `1073741824` - Raw bytes

### Port Management

| Option | CLI Flag | Default | Description |
|--------|----------|---------|-------------|
| **Port** | `--port` | None | Port assignment |
| **Port Range** | `--port` | None | Port range for clusters |
| **Auto Port** | `--port` | None | Auto-assign from range |

```bash
# Port configuration examples
pmdaemon start "node server.js" --port 3000                    # Single port
pmdaemon start "node server.js" --instances 4 --port 3000-3003 # Port range
pmdaemon start "node server.js" --instances 3 --port auto:5000-5100 # Auto-assign
```

### Process Control

| Option | CLI Flag | Default | Description |
|--------|----------|---------|-------------|
| **Auto Restart** | `--autorestart` | `true` | Restart on crash |
| **Max Restarts** | `--max-restarts` | `16` | Maximum restart attempts |
| **Min Uptime** | `--min-uptime` | `1000ms` | Minimum uptime before stable |
| **Restart Delay** | `--restart-delay` | `0ms` | Delay between restarts |
| **Kill Timeout** | `--kill-timeout` | `1600ms` | Graceful shutdown timeout |

```bash
# Process control configuration
pmdaemon start "node server.js" \
  --name web-api \
  --autorestart true \
  --max-restarts 10 \
  --min-uptime 5s \
  --restart-delay 1s \
  --kill-timeout 30s
```

### Logging Configuration

| Option | CLI Flag | Default | Description |
|--------|----------|---------|-------------|
| **Output File** | `--out-file` | Auto-generated | Stdout log file |
| **Error File** | `--error-file` | Auto-generated | Stderr log file |
| **Log File** | `--log-file` | Auto-generated | Combined log file |
| **PID File** | `--pid-file` | Auto-generated | Process ID file |

```bash
# Logging configuration
pmdaemon start "node server.js" \
  --name web-api \
  --out-file /var/log/web-api.out \
  --error-file /var/log/web-api.err \
  --pid-file /var/run/web-api.pid
```

### Health Checks

| Option | CLI Flag | Default | Description |
|--------|----------|---------|-------------|
| **Health Check URL** | `--health-check-url` | None | HTTP health check endpoint |
| **Health Check Script** | `--health-check-script` | None | Script-based health check |
| **Health Check Timeout** | `--health-check-timeout` | `30s` | Health check timeout |
| **Health Check Interval** | `--health-check-interval` | `60s` | Health check frequency |
| **Health Check Retries** | `--health-check-retries` | `3` | Retries before failure |
| **Wait Ready** | `--wait-ready` | `false` | Block start until healthy |
| **Wait Timeout** | `--wait-timeout` | `30s` | Blocking start timeout |

```bash
# Health check configuration
pmdaemon start "node api.js" \
  --name api-service \
  --port 3000 \
  --health-check-url http://localhost:3000/health \
  --health-check-timeout 10s \
  --health-check-interval 30s \
  --health-check-retries 5 \
  --wait-ready \
  --wait-timeout 60s
```

## Advanced Configuration

### Clustering Configuration

```bash
# Advanced clustering setup
pmdaemon start "node server.js" \
  --name web-cluster \
  --instances 4 \
  --port 3000-3003 \
  --exec-mode cluster \
  --instance-var INSTANCE_ID \
  --merge-logs true
```

### Namespace Configuration

```bash
# Organize processes by namespace
pmdaemon start "node api.js" \
  --name api \
  --namespace production

pmdaemon start "node worker.js" \
  --name worker \
  --namespace production

# List processes by namespace
pmdaemon list --namespace production
```

### Watch Mode (Future Feature)

```bash
# File watching for auto-restart
pmdaemon start "node server.js" \
  --name dev-server \
  --watch true \
  --watch-delay 1000 \
  --ignore-watch "node_modules logs *.log"
```

## Configuration File Formats

### JSON Configuration

```json
{
  "apps": [
    {
      "name": "web-api",
      "script": "node",
      "args": ["server.js"],
      "instances": 2,
      "port": "3000-3001",
      "cwd": "/app",
      "env": {
        "NODE_ENV": "production",
        "PORT": "3000"
      },
      "max_memory_restart": "512M",
      "autorestart": true,
      "max_restarts": 10,
      "min_uptime": "5s",
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health",
        "timeout": 10,
        "interval": 30,
        "retries": 3,
        "enabled": true
      },
      "log": {
        "out_file": "/var/log/web-api.out",
        "error_file": "/var/log/web-api.err",
        "pid_file": "/var/run/web-api.pid"
      }
    }
  ]
}
```

### YAML Configuration

```yaml
apps:
  - name: web-api
    script: node
    args: [server.js]
    instances: 2
    port: "3000-3001"
    cwd: /app
    env:
      NODE_ENV: production
      PORT: "3000"
    max_memory_restart: 512M
    autorestart: true
    max_restarts: 10
    min_uptime: 5s
    health_check:
      check_type: http
      url: http://localhost:3000/health
      timeout: 10
      interval: 30
      retries: 3
      enabled: true
    log:
      out_file: /var/log/web-api.out
      error_file: /var/log/web-api.err
      pid_file: /var/run/web-api.pid
```

### TOML Configuration

```toml
[[apps]]
name = "web-api"
script = "node"
args = ["server.js"]
instances = 2
port = "3000-3001"
cwd = "/app"
max_memory_restart = "512M"
autorestart = true
max_restarts = 10
min_uptime = "5s"

[apps.env]
NODE_ENV = "production"
PORT = "3000"

[apps.health_check]
check_type = "http"
url = "http://localhost:3000/health"
timeout = 10
interval = 30
retries = 3
enabled = true

[apps.log]
out_file = "/var/log/web-api.out"
error_file = "/var/log/web-api.err"
pid_file = "/var/run/web-api.pid"
```

## Configuration Validation

PMDaemon validates all configuration options:

### Required Fields

```bash
# Error: Missing required fields
pmdaemon start --name web-api
# Error: Script is required

pmdaemon start "node server.js"
# Error: Name is required
```

### Value Validation

```bash
# Error: Invalid memory format
pmdaemon start "node server.js" --name web-api --max-memory "invalid"
# Error: Invalid memory format 'invalid'. Use formats like '512M', '1G', etc.

# Error: Invalid port range
pmdaemon start "node server.js" --name web-api --instances 4 --port 3000-3001
# Error: Port range 3000-3001 has only 2 ports but 4 instances requested
```

### Conflict Detection

```bash
# Error: Name conflict
pmdaemon start "node server.js" --name web-api
pmdaemon start "python app.py" --name web-api
# Error: Process with name 'web-api' already exists

# Error: Port conflict
pmdaemon start "node server.js" --name api1 --port 3000
pmdaemon start "python app.py" --name api2 --port 3000
# Error: Port 3000 is already assigned to process 'api1'
```

## Configuration Best Practices

### 1. Use Configuration Files for Complex Setups

```bash
# Good: Use config file for multiple processes
pmdaemon --config ecosystem.json start

# Avoid: Long CLI commands
pmdaemon start "node server.js" --name web-api --instances 4 --port 3000-3003 --max-memory 512M --env NODE_ENV=production --health-check-url http://localhost:3000/health --wait-ready
```

### 2. Set Appropriate Resource Limits

```bash
# Good: Set memory limits
pmdaemon start "node server.js" --name web-api --max-memory 512M

# Avoid: No limits (can cause system issues)
pmdaemon start "node server.js" --name web-api
```

### 3. Use Health Checks for Critical Services

```bash
# Good: Health checks for web services
pmdaemon start "node api.js" \
  --name api \
  --health-check-url http://localhost:3000/health

# Good: Script checks for workers
pmdaemon start "python worker.py" \
  --name worker \
  --health-check-script ./health-check.sh
```

### 4. Organize with Namespaces

```bash
# Good: Use namespaces for organization
pmdaemon start "node api.js" --name api --namespace production
pmdaemon start "node worker.js" --name worker --namespace production

# List by namespace
pmdaemon list --namespace production
```

## Runtime Configuration Changes

### Temporary Overrides

```bash
# Start with default configuration
pmdaemon start "node server.js" --name web-api --port 3000

# Restart with temporary port override
pmdaemon restart web-api --port 3001

# Configuration file still shows port 3000, but process runs on 3001
```

### Permanent Changes

```bash
# To permanently change configuration, delete and recreate
pmdaemon delete web-api
pmdaemon start "node server.js" --name web-api --port 3001
```

## Next Steps

- **[Process Configuration](../configuration/process-configuration.md)** - Detailed process options
- **[Advanced Configuration](../configuration/advanced-configuration.md)** - Complex scenarios
- **[Schema Validation](../configuration/schema-validation.md)** - Configuration validation
- **[Examples](../examples/deployment-examples.md)** - Real-world configurations
