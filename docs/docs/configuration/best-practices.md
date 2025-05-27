# Configuration Best Practices

This guide covers best practices for configuring PMDaemon ecosystem files, ensuring optimal performance, reliability, and maintainability across different environments.

## General Configuration Principles

### 1. Keep It Simple
- Start with minimal configuration and add complexity as needed
- Use defaults when possible
- Document any non-obvious configurations

### 2. Environment Parity
- Maintain similar configurations across environments
- Use environment variables for differences
- Test configurations in staging before production

### 3. Version Control
- Store all configuration files in version control
- Use meaningful commit messages for config changes
- Tag configurations with release versions

## File Organization

### Directory Structure
```
config/
├── ecosystem.json              # Main configuration
├── ecosystem.dev.json          # Development overrides
├── ecosystem.staging.json      # Staging configuration
├── ecosystem.prod.json         # Production configuration
├── schemas/
│   ├── base.schema.json        # Base validation schema
│   ├── dev.schema.json         # Development schema
│   └── prod.schema.json        # Production schema
└── templates/
    ├── web-app.json            # Web application template
    ├── worker.json             # Background worker template
    └── microservice.json       # Microservice template
```

### Naming Conventions
```json
{
  "apps": [
    {
      "name": "myapp-web-prod",        // Format: {app}-{type}-{env}
      "script": "dist/server.js",
      "log_file": "/var/log/myapp/web.log",
      "pid_file": "/var/run/myapp/web.pid"
    }
  ]
}
```

## Performance Optimization

### 1. Instance Management

**Development (Single Instance):**
```json
{
  "instances": 1,
  "exec_mode": "fork"
}
```

**Production (Cluster Mode):**
```json
{
  "instances": "max",              // Use all CPU cores
  "exec_mode": "cluster",
  "kill_timeout": 5000,           // Graceful shutdown
  "wait_ready": true              // Wait for ready signal
}
```

**Custom Scaling:**
```json
{
  "instances": 4,                 // Specific instance count
  "exec_mode": "cluster",
  "increment_var": "PORT",        // Auto-increment PORT
  "instance_var": "INSTANCE_ID"   // Inject instance ID
}
```

### 2. Memory Management

**Memory Limits:**
```json
{
  "max_memory_restart": "1G",     // Restart if memory exceeds 1GB
  "kill_timeout": 5000,           // Grace period for cleanup
  "max_restarts": 3,              // Limit restart loops
  "min_uptime": "10s"             // Minimum runtime before restart
}
```

**Memory Monitoring:**
```json
{
  "monitoring": {
    "memory_threshold": 85,       // Alert at 85% memory usage
    "cpu_threshold": 80,          // Alert at 80% CPU usage
    "check_interval": 30          // Check every 30 seconds
  }
}
```

### 3. Resource Optimization

**File System:**
```json
{
  "watch": false,                 // Disable in production
  "ignore_watch": [               // Ignore unnecessary files
    "node_modules",
    "*.log",
    ".git",
    "tmp"
  ],
  "watch_options": {
    "ignored": /node_modules|\.git/,
    "persistent": true,
    "ignoreInitial": true
  }
}
```

**Network Optimization:**
```json
{
  "port": {
    "start": 3000,
    "increment": 1,
    "max": 3010
  },
  "health_check": {
    "enabled": true,
    "url": "http://localhost:3000/health",
    "interval": 30,
    "timeout": 5,
    "retries": 3
  }
}
```

## Reliability Configuration

### 1. Restart Policies

**Development (Lenient):**
```json
{
  "max_restarts": 15,
  "min_uptime": "1s",
  "restart_delay": 100,
  "autorestart": true
}
```

**Production (Strict):**
```json
{
  "max_restarts": 3,
  "min_uptime": "10s",
  "restart_delay": 4000,
  "autorestart": true,
  "exponential_backoff_restart_delay": 150
}
```

### 2. Health Checks

**Basic Health Check:**
```json
{
  "health_check": {
    "enabled": true,
    "url": "http://localhost:3000/health",
    "interval": 30,
    "timeout": 10,
    "retries": 3
  }
}
```

**Advanced Health Check:**
```json
{
  "health_check": {
    "enabled": true,
    "url": "http://localhost:3000/health",
    "interval": 30,
    "timeout": 10,
    "retries": 3,
    "headers": {
      "Authorization": "Bearer ${HEALTH_CHECK_TOKEN}"
    },
    "expected_status": [200, 201],
    "expected_body": "OK"
  }
}
```

### 3. Error Handling

**Graceful Shutdown:**
```json
{
  "kill_timeout": 5000,           // 5 seconds for graceful shutdown
  "shutdown_with_message": true,  // Send shutdown message
  "wait_ready": true,             // Wait for ready signal
  "listen_timeout": 3000          // Timeout for listen signal
}
```

**Error Recovery:**
```json
{
  "max_restarts": 3,
  "min_uptime": "10s",
  "restart_delay": 4000,
  "autorestart": true,
  "crash_action": "restart",      // Action on crash: restart|stop|ignore
  "pmx": false                    // Disable PMX if not needed
}
```

## Security Best Practices

### 1. Environment Variables

**Secure Configuration:**
```json
{
  "env": {
    "NODE_ENV": "production",
    "PORT": "3000",
    "DATABASE_URL": "${DATABASE_URL}",
    "JWT_SECRET": "${JWT_SECRET}",
    "API_KEY": "${API_KEY}"
  },
  "env_file": ".env.production"
}
```

**Secret Management:**
```json
{
  "env": {
    "DATABASE_PASSWORD_FILE": "/run/secrets/db_password",
    "JWT_SECRET_FILE": "/run/secrets/jwt_secret"
  }
}
```

### 2. User and Permissions

**User Configuration:**
```json
{
  "user": "appuser",              // Run as specific user
  "group": "appgroup",            // Run as specific group
  "uid": 1000,                    // Specific UID
  "gid": 1000                     // Specific GID
}
```

**File Permissions:**
```json
{
  "cwd": "/app",
  "log_file": "/var/log/app/app.log",
  "pid_file": "/var/run/app/app.pid",
  "umask": "0022"                 // Set file creation mask
}
```

### 3. Network Security

**Binding Configuration:**
```json
{
  "env": {
    "HOST": "127.0.0.1",          // Bind to localhost only
    "PORT": "3000"
  },
  "instances": 1,                 // Single instance for security
  "exec_mode": "fork"
}
```

## Logging Best Practices

### 1. Log Configuration

**Development Logging:**
```json
{
  "log_file": "logs/app.log",
  "error_file": "logs/error.log",
  "out_file": "logs/out.log",
  "log_date_format": "YYYY-MM-DD HH:mm:ss Z",
  "merge_logs": false,
  "log_type": "json"
}
```

**Production Logging:**
```json
{
  "log_file": "/var/log/myapp/app.log",
  "error_file": "/var/log/myapp/error.log",
  "out_file": "/var/log/myapp/out.log",
  "log_date_format": "YYYY-MM-DD HH:mm:ss Z",
  "merge_logs": true,
  "log_type": "json",
  "log_options": {
    "max_size": "10m",
    "max_files": 5,
    "tailable": true,
    "zippedArchive": true
  }
}
```

### 2. Log Rotation

```json
{
  "log_file": "/var/log/myapp/app.log",
  "log_options": {
    "max_size": "50m",            // Rotate when file reaches 50MB
    "max_files": 10,              // Keep 10 old files
    "tailable": true,             // Keep watching newest file
    "zippedArchive": true         // Compress old files
  }
}
```

## Monitoring Configuration

### 1. Basic Monitoring

```json
{
  "monitoring": {
    "http": true,                 // Enable HTTP monitoring
    "https": false,
    "port": 9615,
    "host": "localhost"
  }
}
```

### 2. Advanced Monitoring

```json
{
  "monitoring": {
    "enabled": true,
    "port": 9615,
    "host": "0.0.0.0",
    "auth": {
      "username": "admin",
      "password": "${MONITORING_PASSWORD}"
    },
    "metrics": {
      "cpu": true,
      "memory": true,
      "disk": true,
      "network": true
    },
    "alerts": {
      "cpu_threshold": 80,
      "memory_threshold": 85,
      "disk_threshold": 90
    }
  }
}
```

## Configuration Templates

### 1. Web Application Template

```json
{
  "apps": [
    {
      "name": "${APP_NAME}-web",
      "script": "${SCRIPT_PATH}",
      "cwd": "${APP_ROOT}",
      "instances": "${INSTANCES:-1}",
      "exec_mode": "${EXEC_MODE:-fork}",
      "env": {
        "NODE_ENV": "${NODE_ENV}",
        "PORT": "${PORT:-3000}",
        "HOST": "${HOST:-localhost}"
      },
      "max_memory_restart": "${MAX_MEMORY:-500M}",
      "max_restarts": "${MAX_RESTARTS:-3}",
      "min_uptime": "${MIN_UPTIME:-10s}",
      "health_check": {
        "enabled": true,
        "url": "http://${HOST:-localhost}:${PORT:-3000}/health",
        "interval": 30
      },
      "log_file": "${LOG_DIR}/${APP_NAME}.log",
      "error_file": "${LOG_DIR}/${APP_NAME}-error.log"
    }
  ]
}
```

### 2. Worker Template

```json
{
  "apps": [
    {
      "name": "${APP_NAME}-worker",
      "script": "${WORKER_SCRIPT}",
      "cwd": "${APP_ROOT}",
      "instances": "${WORKER_INSTANCES:-1}",
      "exec_mode": "fork",
      "env": {
        "NODE_ENV": "${NODE_ENV}",
        "WORKER_TYPE": "${WORKER_TYPE}",
        "QUEUE_URL": "${QUEUE_URL}"
      },
      "max_memory_restart": "${WORKER_MAX_MEMORY:-1G}",
      "max_restarts": 10,
      "min_uptime": "30s",
      "autorestart": true,
      "watch": false
    }
  ]
}
```

### 3. Microservice Template

```json
{
  "apps": [
    {
      "name": "${SERVICE_NAME}",
      "script": "dist/server.js",
      "cwd": "/app",
      "instances": "${INSTANCES:-max}",
      "exec_mode": "cluster",
      "env": {
        "NODE_ENV": "production",
        "PORT": "${PORT}",
        "SERVICE_NAME": "${SERVICE_NAME}",
        "DATABASE_URL": "${DATABASE_URL}",
        "REDIS_URL": "${REDIS_URL}"
      },
      "max_memory_restart": "1G",
      "max_restarts": 3,
      "min_uptime": "10s",
      "health_check": {
        "enabled": true,
        "url": "http://localhost:${PORT}/health",
        "interval": 30,
        "timeout": 10,
        "retries": 3
      },
      "log_file": "/var/log/${SERVICE_NAME}/${SERVICE_NAME}.log",
      "error_file": "/var/log/${SERVICE_NAME}/${SERVICE_NAME}-error.log"
    }
  ]
}
```

## Validation and Testing

### 1. Configuration Validation

```bash
# Validate configuration syntax
pmdaemon validate ecosystem.json

# Validate against schema
pmdaemon validate ecosystem.json --schema production.schema.json

# Test configuration without starting
pmdaemon start ecosystem.json --dry-run
```

### 2. Testing Scripts

```bash
#!/bin/bash
# test-config.sh

set -e

CONFIG_FILE=${1:-ecosystem.json}
ENV=${2:-development}

echo "Testing configuration: $CONFIG_FILE for environment: $ENV"

# Syntax validation
echo "Validating JSON syntax..."
jq empty "$CONFIG_FILE"

# Schema validation
echo "Validating against schema..."
pmdaemon validate "$CONFIG_FILE" --schema="${ENV}.schema.json"

# Dry run
echo "Testing configuration..."
pmdaemon start "$CONFIG_FILE" --dry-run

# Environment variable check
echo "Checking environment variables..."
node scripts/check-env.js "$CONFIG_FILE"

echo "Configuration test passed!"
```

## Common Pitfalls to Avoid

### 1. Configuration Issues
- **Don't hardcode environment-specific values**
- **Avoid overly complex configurations**
- **Don't ignore error handling configuration**
- **Avoid insufficient resource limits**

### 2. Performance Issues
- **Don't use file watching in production**
- **Avoid too many instances on small servers**
- **Don't ignore memory limits**
- **Avoid blocking startup procedures**

### 3. Security Issues
- **Never commit secrets to version control**
- **Don't run as root user**
- **Avoid exposing internal services**
- **Don't use weak authentication**

### 4. Monitoring Issues
- **Don't disable health checks in production**
- **Avoid insufficient logging**
- **Don't ignore resource monitoring**
- **Avoid missing alerting configuration**

## Migration Strategies

### 1. Configuration Updates

```bash
#!/bin/bash
# migrate-config.sh

OLD_CONFIG="ecosystem.old.json"
NEW_CONFIG="ecosystem.json"
BACKUP_DIR="config-backups"

# Create backup
mkdir -p "$BACKUP_DIR"
cp "$NEW_CONFIG" "$BACKUP_DIR/ecosystem-$(date +%Y%m%d-%H%M%S).json"

# Stop current processes
pmdaemon stop all

# Update configuration
cp "$OLD_CONFIG" "$NEW_CONFIG"

# Validate new configuration
pmdaemon validate "$NEW_CONFIG"

# Start with new configuration
pmdaemon start "$NEW_CONFIG"

echo "Configuration migration completed"
```

### 2. Rolling Updates

```bash
#!/bin/bash
# rolling-update.sh

NEW_CONFIG="ecosystem.new.json"

# Validate new configuration
pmdaemon validate "$NEW_CONFIG"

# Reload configuration
pmdaemon reload "$NEW_CONFIG"

# Verify all processes are running
pmdaemon status

echo "Rolling update completed"
```

## Related Documentation

- **[Ecosystem Files](./ecosystem-files.md)** - Complete ecosystem configuration reference
- **[Environment-Specific](./environment-specific.md)** - Environment-specific configurations
- **[Security](../security/overview.md)** - Security best practices
- **[Monitoring](../monitoring/overview.md)** - Monitoring and alerting setup
