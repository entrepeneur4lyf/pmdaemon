# Process Configuration

PMDaemon provides comprehensive process configuration options that allow you to define every aspect of how your applications run. This guide covers individual process configuration in detail, from basic settings to advanced clustering and health checks.

## Configuration Overview

Process configuration in PMDaemon can be specified through:

- **CLI arguments** - Direct command-line configuration
- **Configuration files** - JSON, YAML, or TOML format
- **Environment variables** - System-level defaults
- **Runtime overrides** - Temporary configuration changes

## Basic Process Configuration

### Required Fields

Every process must have these essential fields:

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"]
}
```

| Field | Description | Example |
|-------|-------------|---------|
| `name` | Unique process identifier | `"web-api"`, `"worker-service"` |
| `script` | Command or executable to run | `"node"`, `"python"`, `"./app"` |
| `args` | Command line arguments | `["server.js"]`, `["-m", "uvicorn", "main:app"]` |

### Basic Example

```bash
# CLI
pmdaemon start "node server.js" --name web-api

# Configuration file
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"]
}
```

## Process Execution Settings

### Working Directory

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "cwd": "/var/www/myapp"
}
```

**CLI equivalent:**
```bash
pmdaemon start "node server.js" --name web-api --cwd /var/www/myapp
```

### Environment Variables

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "env": {
    "NODE_ENV": "production",
    "DATABASE_URL": "postgres://localhost/mydb",
    "API_KEY": "secret123"
  }
}
```

**CLI equivalent:**
```bash
pmdaemon start "node server.js" --name web-api \
  --env NODE_ENV=production \
  --env DATABASE_URL=postgres://localhost/mydb \
  --env API_KEY=secret123
```

### User and Group (Future Feature)

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "user": "www-data",
  "group": "www-data"
}
```

## Clustering Configuration

### Basic Clustering

```json
{
  "name": "web-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "exec_mode": "cluster"
}
```

**CLI equivalent:**
```bash
pmdaemon start "node server.js" --name web-cluster --instances 4
```

### Advanced Clustering

```json
{
  "name": "web-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "exec_mode": "cluster",
  "instance_var": "INSTANCE_ID",
  "merge_logs": true
}
```

**Special instance values:**
- `"max"` - Use all available CPU cores
- `0` - Disable the process
- Positive integer - Specific number of instances

## Port Management

### Single Port

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "port": "3000"
}
```

### Port Range for Clustering

```json
{
  "name": "web-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "port": "3000-3003"
}
```

### Auto Port Assignment

```json
{
  "name": "workers",
  "script": "python",
  "args": ["worker.py"],
  "instances": 3,
  "port": "auto:5000-5100"
}
```

**Port configuration formats:**
- `"3000"` - Single port
- `"3000-3003"` - Port range (4 ports)
- `"auto:5000-5100"` - Auto-assign from range

## Resource Limits

### Memory Limits

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "max_memory_restart": "512M"
}
```

**Memory format examples:**
- `"100K"` or `"100KB"` - Kilobytes
- `"512M"` or `"512MB"` - Megabytes
- `"2G"` or `"2GB"` - Gigabytes
- `1073741824` - Raw bytes (number)

### CPU Limits (Future Feature)

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "max_cpu_percent": 80
}
```

## Process Control Settings

### Auto-restart Configuration

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "autorestart": true,
  "max_restarts": 10,
  "min_uptime": "5s",
  "restart_delay": "1s"
}
```

| Field | Default | Description |
|-------|---------|-------------|
| `autorestart` | `true` | Restart on crash |
| `max_restarts` | `16` | Maximum restart attempts |
| `min_uptime` | `"1000ms"` | Minimum uptime before stable |
| `restart_delay` | `"0ms"` | Delay between restarts |

### Signal Handling

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "kill_timeout": "30s",
  "kill_retry_time": "100ms"
}
```

| Field | Default | Description |
|-------|---------|-------------|
| `kill_timeout` | `"1600ms"` | Graceful shutdown timeout |
| `kill_retry_time` | `"100ms"` | Time between kill attempts |

## Logging Configuration

### Basic Logging

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "out_file": "/var/log/web-api.out",
  "error_file": "/var/log/web-api.err",
  "log_file": "/var/log/web-api.log",
  "pid_file": "/var/run/web-api.pid"
}
```

### Advanced Logging

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "log": {
    "out_file": "/var/log/web-api.out",
    "error_file": "/var/log/web-api.err",
    "log_file": "/var/log/web-api.log",
    "pid_file": "/var/run/web-api.pid",
    "log_date_format": "YYYY-MM-DD HH:mm:ss Z",
    "merge_logs": true,
    "log_type": "json"
  }
}
```

**Default log file patterns:**
- Stdout: `{name}-{instance}-out.log`
- Stderr: `{name}-{instance}-err.log`
- Combined: `{name}-{instance}.log`
- PID: `{name}-{instance}.pid`

## Health Check Configuration

### HTTP Health Checks

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "port": "3000",
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 10,
    "interval": 30,
    "retries": 3,
    "enabled": true
  }
}
```

### Script-based Health Checks

```json
{
  "name": "worker",
  "script": "python",
  "args": ["worker.py"],
  "health_check": {
    "check_type": "script",
    "script": "./health-check.sh",
    "timeout": 15,
    "interval": 60,
    "retries": 2,
    "enabled": true
  }
}
```

**Health check fields:**
| Field | Default | Description |
|-------|---------|-------------|
| `check_type` | Required | `"http"` or `"script"` |
| `url` | Required for HTTP | Health check endpoint |
| `script` | Required for script | Script path to execute |
| `timeout` | `30` | Timeout in seconds |
| `interval` | `60` | Check interval in seconds |
| `retries` | `3` | Retries before failure |
| `enabled` | `true` | Enable/disable health checks |

## Watch Mode (Not Yet Implemented)

File watching is planned for a future release but not currently available:

```json
{
  "name": "dev-server", 
  "script": "node",
  "args": ["server.js"],
  "watch": false,  // Always false - not implemented
  "ignore_watch": []  // Not used
}
```

## Complete Configuration Example

### Production Web API

```json
{
  "name": "production-api",
  "script": "node",
  "args": ["dist/server.js"],
  "instances": 4,
  "exec_mode": "cluster",
  "port": "3000-3003",
  "cwd": "/var/www/api",
  "env": {
    "NODE_ENV": "production",
    "DATABASE_URL": "postgres://prod-server/mydb",
    "REDIS_URL": "redis://prod-redis:6379"
  },
  "max_memory_restart": "1G",
  "autorestart": true,
  "max_restarts": 5,
  "min_uptime": "10s",
  "restart_delay": "2s",
  "kill_timeout": "30s",
  "out_file": "/var/log/api/production-api.out",
  "error_file": "/var/log/api/production-api.err",
  "pid_file": "/var/run/api/production-api.pid",
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 10,
    "interval": 30,
    "retries": 3,
    "enabled": true
  }
}
```

### Background Worker

```json
{
  "name": "email-worker",
  "script": "python",
  "args": ["-m", "celery", "worker", "-A", "myapp"],
  "instances": 2,
  "cwd": "/var/www/workers",
  "env": {
    "CELERY_BROKER_URL": "redis://localhost:6379/0",
    "CELERY_RESULT_BACKEND": "redis://localhost:6379/0",
    "WORKER_TYPE": "email"
  },
  "max_memory_restart": "512M",
  "autorestart": true,
  "max_restarts": 10,
  "min_uptime": "5s",
  "restart_delay": "1s",
  "health_check": {
    "check_type": "script",
    "script": "./scripts/check-worker.sh",
    "timeout": 15,
    "interval": 60,
    "retries": 2,
    "enabled": true
  }
}
```

### Development Server

```json
{
  "name": "dev-server",
  "script": "npm",
  "args": ["run", "dev"],
  "cwd": "/home/user/myapp",
  "env": {
    "NODE_ENV": "development",
    "DEBUG": "*"
  },
  "autorestart": true,
  "max_restarts": 100,
  "min_uptime": "1s",
  "watch": true,
  "ignore_watch": [
    "node_modules",
    "dist",
    "*.log"
  ]
}
```

## Configuration Validation

PMDaemon validates all configuration fields:

### Required Field Validation

```json
{
  // Error: Missing required field 'name'
  "script": "node",
  "args": ["server.js"]
}
```

### Type Validation

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "instances": "invalid"  // Error: Must be number
}
```

### Value Range Validation

```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "instances": -1  // Error: Must be positive
}
```

### Port Range Validation

```json
{
  "name": "web-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "port": "3000-3001"  // Error: Need 4 ports but only 2 in range
}
```

## Configuration Best Practices

### 1. Use Descriptive Names

```json
// Good: Descriptive names
{
  "name": "user-service-api",
  "name": "email-notification-worker",
  "name": "payment-processor"
}

// Avoid: Generic names
{
  "name": "app1",
  "name": "worker",
  "name": "service"
}
```

### 2. Set Appropriate Resource Limits

```json
// Good: Set memory limits
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "max_memory_restart": "512M"
}

// Avoid: No limits (can cause system issues)
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"]
}
```

### 3. Configure Health Checks

```json
// Good: Health checks for web services
{
  "name": "api-service",
  "script": "node",
  "args": ["api.js"],
  "port": "3000",
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health"
  }
}

// Good: Script checks for workers
{
  "name": "background-worker",
  "script": "python",
  "args": ["worker.py"],
  "health_check": {
    "check_type": "script",
    "script": "./health-check.sh"
  }
}
```

### 4. Use Environment-Specific Configurations

```json
// Development
{
  "name": "dev-api",
  "script": "npm",
  "args": ["run", "dev"],
  "env": {
    "NODE_ENV": "development",
    "DEBUG": "*"
  },
  "watch": true
}

// Production
{
  "name": "prod-api",
  "script": "node",
  "args": ["dist/server.js"],
  "instances": 4,
  "env": {
    "NODE_ENV": "production"
  },
  "max_memory_restart": "1G"
}
```

### 5. Organize with Namespaces

```json
{
  "name": "api-service",
  "script": "node",
  "args": ["server.js"],
  "namespace": "production"
}
```

## Configuration Templates

### Web Application Template

```json
{
  "name": "{{APP_NAME}}",
  "script": "node",
  "args": ["server.js"],
  "instances": "{{INSTANCES:-2}}",
  "port": "{{PORT_RANGE:-3000-3001}}",
  "cwd": "{{APP_DIR}}",
  "env": {
    "NODE_ENV": "{{NODE_ENV:-production}}",
    "DATABASE_URL": "{{DATABASE_URL}}",
    "REDIS_URL": "{{REDIS_URL}}"
  },
  "max_memory_restart": "{{MAX_MEMORY:-512M}}",
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:{{HEALTH_PORT:-3000}}/health"
  }
}
```

### Worker Service Template

```json
{
  "name": "{{WORKER_NAME}}",
  "script": "python",
  "args": ["-m", "celery", "worker", "-A", "{{APP_MODULE}}"],
  "instances": "{{WORKER_INSTANCES:-1}}",
  "cwd": "{{WORKER_DIR}}",
  "env": {
    "CELERY_BROKER_URL": "{{BROKER_URL}}",
    "CELERY_RESULT_BACKEND": "{{RESULT_BACKEND}}"
  },
  "max_memory_restart": "{{WORKER_MEMORY:-256M}}",
  "health_check": {
    "check_type": "script",
    "script": "{{HEALTH_SCRIPT:-./health-check.sh}}"
  }
}
```

## Next Steps

- **[Advanced Configuration](./advanced-configuration.md)** - Complex scenarios and patterns
- **[Schema Validation](./schema-validation.md)** - Configuration validation details
- **[Ecosystem Files](./ecosystem-files.md)** - Multi-process configurations
- **[Examples](../examples/deployment-examples.md)** - Real-world configuration examples
