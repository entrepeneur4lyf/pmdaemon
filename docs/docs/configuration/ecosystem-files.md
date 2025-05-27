# Ecosystem Configuration Files

PMDaemon supports ecosystem configuration files in multiple formats (JSON, YAML, TOML), allowing you to define complex application setups in a declarative way. This is similar to PM2's ecosystem.config.js but with enhanced features and multiple format support.

## Supported Formats

PMDaemon supports three configuration formats:

| Format | Extension | Pros | Cons |
|--------|-----------|------|------|
| **JSON** | `.json` | Fast parsing, wide support | No comments, strict syntax |
| **YAML** | `.yaml`, `.yml` | Human-readable, supports comments | Slower parsing, indentation-sensitive |
| **TOML** | `.toml` | Comments, clear syntax | Less common, moderate parsing speed |

## Basic Structure

All formats follow the same structure with an `apps` array containing process configurations:

### JSON Format

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
      }
    }
  ]
}
```

### YAML Format

```yaml
# PMDaemon ecosystem configuration
apps:
  - name: web-server
    script: node
    args:
      - server.js
    instances: 2
    port: "3000-3001"
    env:
      NODE_ENV: production
```

### TOML Format

```toml
# PMDaemon ecosystem configuration

[[apps]]
name = "web-server"
script = "node"
args = ["server.js"]
instances = 2
port = "3000-3001"

[apps.env]
NODE_ENV = "production"
```

## Configuration Options

### Required Fields

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `name` | String | Unique process name | `"web-server"` |
| `script` | String | Command or executable | `"node"`, `"python"`, `"./app"` |

### Optional Fields

| Field | Type | Description | Default | Example |
|-------|------|-------------|---------|---------|
| `args` | Array | Command arguments | `[]` | `["server.js", "--port", "3000"]` |
| `instances` | Number | Number of instances | `1` | `4` |
| `port` | String | Port configuration | `null` | `"3000"`, `"3000-3003"`, `"auto:5000-5100"` |
| `cwd` | String | Working directory | Current dir | `"/path/to/app"` |
| `env` | Object | Environment variables | `{}` | `{"NODE_ENV": "production"}` |
| `max_memory_restart` | String/Number | Memory limit | `null` | `"512M"`, `536870912` |
| `autorestart` | Boolean | Auto-restart on crash | `true` | `false` |
| `max_restarts` | Number | Maximum restart attempts | `16` | `10` |
| `min_uptime` | String | Minimum uptime before stable | `"1000ms"` | `"5s"` |
| `restart_delay` | String | Delay between restarts | `"0ms"` | `"2s"` |
| `kill_timeout` | String | Graceful shutdown timeout | `"1600ms"` | `"30s"` |
| `namespace` | String | Process namespace | `"default"` | `"production"` |
| `out_file` | String | Output log file | Auto-generated | `"/var/log/app.out"` |
| `error_file` | String | Error log file | Auto-generated | `"/var/log/app.err"` |
| `log_file` | String | Combined log file | `null` | `"/var/log/app.log"` |
| `pid_file` | String | PID file location | Auto-generated | `"/var/run/app.pid"` |

### Health Check Configuration

```json
{
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 5,
    "interval": 30,
    "retries": 3,
    "enabled": true
  }
}
```

| Field | Type | Description | Default | Example |
|-------|------|-------------|---------|---------|
| `check_type` | String | Type of health check | `null` | `"http"`, `"script"` |
| `url` | String | HTTP endpoint (for http type) | `null` | `"http://localhost:3000/health"` |
| `script` | String | Script path (for script type) | `null` | `"./health-check.sh"` |
| `timeout` | Number | Timeout in seconds | `30` | `10` |
| `interval` | Number | Check interval in seconds | `60` | `30` |
| `retries` | Number | Number of retries | `3` | `5` |
| `enabled` | Boolean | Enable health checks | `true` | `false` |

## Complete Examples

### Web Application Cluster

```json
{
  "apps": [
    {
      "name": "web-cluster",
      "script": "node",
      "args": ["server.js"],
      "instances": 4,
      "port": "3000-3003",
      "max_memory_restart": "1G",
      "env": {
        "NODE_ENV": "production",
        "DATABASE_URL": "postgres://localhost/myapp"
      },
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health",
        "timeout": 10,
        "interval": 30,
        "retries": 3
      },
      "out_file": "/var/log/web-cluster.out",
      "error_file": "/var/log/web-cluster.err"
    }
  ]
}
```

### Microservices Setup

```yaml
# microservices.yaml
apps:
  # API Gateway
  - name: api-gateway
    script: node
    args: [gateway.js]
    port: "8080"
    max_memory_restart: "512M"
    env:
      NODE_ENV: production
      LOG_LEVEL: info
    health_check:
      check_type: http
      url: http://localhost:8080/health
      timeout: 5
      interval: 30

  # User Service
  - name: user-service
    script: node
    args: [user-service.js]
    instances: 2
    port: "8001-8002"
    max_memory_restart: "256M"
    env:
      NODE_ENV: production
      DATABASE_URL: postgres://localhost/users
    health_check:
      check_type: http
      url: http://localhost:8001/health
      timeout: 5
      interval: 30

  # Order Service
  - name: order-service
    script: python
    args: [-m, uvicorn, main:app, --host, "0.0.0.0"]
    instances: 2
    port: "8003-8004"
    max_memory_restart: "512M"
    cwd: /path/to/order-service
    env:
      PYTHONPATH: /path/to/order-service
      DATABASE_URL: postgres://localhost/orders
    health_check:
      check_type: http
      url: http://localhost:8003/docs
      timeout: 10
      interval: 60

  # Background Worker
  - name: background-worker
    script: python
    args: [worker.py]
    max_memory_restart: "256M"
    max_restarts: 5
    restart_delay: "5s"
    env:
      REDIS_URL: redis://localhost:6379
      QUEUE_NAME: background_tasks
    health_check:
      check_type: script
      script: ./worker-health.sh
      timeout: 10
      interval: 60
```

### Development vs Production

#### development.json
```json
{
  "apps": [
    {
      "name": "dev-server",
      "script": "npm",
      "args": ["run", "dev"],
      "port": "3000",
      "env": {
        "NODE_ENV": "development",
        "DEBUG": "*",
        "DATABASE_URL": "postgres://localhost/myapp_dev"
      },
      "autorestart": false,
      "max_restarts": 0
    }
  ]
}
```

#### production.json
```json
{
  "apps": [
    {
      "name": "prod-server",
      "script": "node",
      "args": ["dist/server.js"],
      "instances": 4,
      "port": "3000-3003",
      "max_memory_restart": "1G",
      "env": {
        "NODE_ENV": "production",
        "DATABASE_URL": "postgres://prod-db:5432/myapp"
      },
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health",
        "timeout": 10,
        "interval": 30,
        "retries": 3
      },
      "out_file": "/var/log/myapp/server.out",
      "error_file": "/var/log/myapp/server.err"
    }
  ]
}
```

## Using Configuration Files

### Start All Apps

```bash
# Start all apps from config file
pmdaemon --config ecosystem.json start
pmdaemon --config ecosystem.yaml start
pmdaemon --config ecosystem.toml start
```

### Start Specific App

```bash
# Start only the specified app
pmdaemon --config ecosystem.json start --name web-server
```

### Environment-Specific Configs

```bash
# Use different configs for different environments
pmdaemon --config development.json start
pmdaemon --config staging.yaml start
pmdaemon --config production.toml start
```

## Advanced Features

### Port Configuration

PMDaemon's advanced port management works seamlessly with config files:

```json
{
  "apps": [
    {
      "name": "web-app",
      "port": "3000",           // Single port
      "instances": 1
    },
    {
      "name": "api-cluster", 
      "port": "4000-4003",      // Port range
      "instances": 4
    },
    {
      "name": "workers",
      "port": "auto:5000-5100", // Auto-assignment
      "instances": 3
    }
  ]
}
```

### Memory Formats

Multiple memory format options:

```json
{
  "apps": [
    {
      "name": "app1",
      "max_memory_restart": "512M"     // String format
    },
    {
      "name": "app2", 
      "max_memory_restart": 536870912  // Bytes (number)
    },
    {
      "name": "app3",
      "max_memory_restart": "1G"       // Gigabytes
    }
  ]
}
```

### Complex Health Checks

```json
{
  "apps": [
    {
      "name": "api-server",
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/api/health",
        "timeout": 15,
        "interval": 45,
        "retries": 5,
        "enabled": true
      }
    },
    {
      "name": "worker",
      "health_check": {
        "check_type": "script",
        "script": "./scripts/worker-health.sh",
        "timeout": 30,
        "interval": 120,
        "retries": 2,
        "enabled": true
      }
    }
  ]
}
```

## Validation and Error Handling

PMDaemon provides detailed validation and error messages:

### Common Validation Errors

```bash
# Missing required fields
Error: App 0 validation failed: Process name cannot be empty

# Duplicate names
Error: Duplicate app name: 'web-server'

# Invalid port format
Error: Invalid port configuration: 'invalid-port'

# Invalid memory format
Error: Invalid memory format: '512X'
```

### File Format Errors

```bash
# JSON syntax error
Error: Failed to parse JSON config file 'ecosystem.json': expected `,` or `}` at line 5 column 3

# YAML syntax error
Error: Failed to parse YAML config file 'ecosystem.yaml': invalid indentation at line 8

# File not found
Error: Failed to read config file 'missing.json': No such file or directory
```

## Best Practices

### 1. Use Descriptive Names

```json
{
  "apps": [
    {
      "name": "api-server-prod",     // Good: descriptive
      "name": "web-frontend-v2",     // Good: includes version
      "name": "app"                  // Bad: too generic
    }
  ]
}
```

### 2. Set Resource Limits

```json
{
  "apps": [
    {
      "name": "memory-intensive-app",
      "max_memory_restart": "2G",
      "max_restarts": 5,
      "min_uptime": "10s"
    }
  ]
}
```

### 3. Use Health Checks for Critical Services

```json
{
  "apps": [
    {
      "name": "critical-api",
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health",
        "timeout": 10,
        "interval": 30,
        "retries": 3
      }
    }
  ]
}
```

### 4. Environment-Specific Configurations

```bash
# Separate files for different environments
ecosystem.development.json
ecosystem.staging.yaml
ecosystem.production.toml
```

### 5. Version Control

- **Include in repository** - Keep config files with your code
- **Use comments** (YAML/TOML) - Document complex configurations
- **Validate before deployment** - Test configs locally first

## Migration from PM2

### Converting PM2 ecosystem.config.js

**PM2 Format:**
```javascript
module.exports = {
  apps: [{
    name: 'my-app',
    script: 'server.js',
    instances: 4,
    env: {
      NODE_ENV: 'development'
    },
    env_production: {
      NODE_ENV: 'production'
    }
  }]
};
```

**PMDaemon Format:**
```json
{
  "apps": [{
    "name": "my-app",
    "script": "node",
    "args": ["server.js"],
    "instances": 4,
    "env": {
      "NODE_ENV": "production"
    }
  }]
}
```

### Key Differences

1. **Script separation** - PMDaemon separates executable from arguments
2. **Environment configs** - Use separate files instead of env_production
3. **Port strings** - PMDaemon uses strings for advanced port features
4. **Health checks** - PMDaemon adds health check configuration

## Next Steps

- **[Schema Validation](./schema-validation.md)** - IDE integration and validation
- **[Environment-Specific Configs](./environment-specific.md)** - Managing different environments
- **[Best Practices](./best-practices.md)** - Configuration best practices
- **[CLI Commands](../cli/commands.md)** - Using configs with CLI commands

---

Ecosystem configuration files provide a powerful way to manage complex application deployments with PMDaemon's advanced features like port management, health checks, and clustering.
