# Advanced Configuration

This guide covers advanced PMDaemon configuration patterns, complex scenarios, and sophisticated deployment strategies. Learn how to leverage PMDaemon's full potential for enterprise-grade process management.

## Multi-Environment Configuration

### Environment-Specific Configurations

Create separate configuration files for different environments:

**ecosystem.development.json:**
```json
{
  "apps": [
    {
      "name": "api-dev",
      "script": "npm",
      "args": ["run", "dev"],
      "instances": 1,
      "port": "3000",
      "env": {
        "NODE_ENV": "development",
        "DEBUG": "*",
        "LOG_LEVEL": "debug"
      },
      "watch": true,
      "ignore_watch": ["node_modules", "logs"],
      "max_restarts": 100,
      "min_uptime": "1s"
    }
  ]
}
```

**ecosystem.production.json:**
```json
{
  "apps": [
    {
      "name": "api-prod",
      "script": "node",
      "args": ["dist/server.js"],
      "instances": 4,
      "port": "3000-3003",
      "env": {
        "NODE_ENV": "production",
        "LOG_LEVEL": "info"
      },
      "max_memory_restart": "1G",
      "max_restarts": 5,
      "min_uptime": "10s",
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health",
        "timeout": 10,
        "interval": 30
      }
    }
  ]
}
```

### Environment Variable Substitution

Use environment variables in configuration files:

```json
{
  "apps": [
    {
      "name": "${APP_NAME:-api-service}",
      "script": "node",
      "args": ["server.js"],
      "instances": "${INSTANCES:-2}",
      "port": "${PORT_RANGE:-3000-3001}",
      "env": {
        "NODE_ENV": "${NODE_ENV:-production}",
        "DATABASE_URL": "${DATABASE_URL}",
        "REDIS_URL": "${REDIS_URL:-redis://localhost:6379}"
      },
      "max_memory_restart": "${MAX_MEMORY:-512M}"
    }
  ]
}
```

**Usage:**
```bash
export APP_NAME=user-service
export INSTANCES=4
export DATABASE_URL=postgres://prod-server/userdb
pmdaemon --config ecosystem.json start
```

## Complex Clustering Patterns

### Heterogeneous Clustering

Different instance types with specialized roles:

```json
{
  "apps": [
    {
      "name": "api-master",
      "script": "node",
      "args": ["server.js", "--role=master"],
      "instances": 1,
      "port": "3000",
      "env": {
        "ROLE": "master",
        "ENABLE_CRON": "true",
        "ENABLE_ADMIN": "true"
      }
    },
    {
      "name": "api-workers",
      "script": "node", 
      "args": ["server.js", "--role=worker"],
      "instances": 3,
      "port": "3001-3003",
      "env": {
        "ROLE": "worker",
        "ENABLE_CRON": "false",
        "ENABLE_ADMIN": "false"
      }
    }
  ]
}
```

### Load-Balanced Service Groups

```json
{
  "apps": [
    {
      "name": "frontend-servers",
      "script": "node",
      "args": ["frontend.js"],
      "instances": 2,
      "port": "8080-8081",
      "env": {
        "SERVICE_TYPE": "frontend",
        "BACKEND_URL": "http://localhost:3000"
      }
    },
    {
      "name": "api-servers",
      "script": "node",
      "args": ["api.js"],
      "instances": 4,
      "port": "3000-3003",
      "env": {
        "SERVICE_TYPE": "api",
        "DATABASE_POOL_SIZE": "25"
      }
    },
    {
      "name": "worker-processes",
      "script": "python",
      "args": ["worker.py"],
      "instances": 2,
      "env": {
        "SERVICE_TYPE": "worker",
        "QUEUE_NAME": "default"
      }
    }
  ]
}
```

## Advanced Port Management

### Dynamic Port Allocation

```json
{
  "apps": [
    {
      "name": "microservice-a",
      "script": "node",
      "args": ["service-a.js"],
      "instances": 2,
      "port": "auto:8000-8100",
      "env": {
        "SERVICE_NAME": "service-a"
      }
    },
    {
      "name": "microservice-b",
      "script": "node",
      "args": ["service-b.js"],
      "instances": 3,
      "port": "auto:8000-8100",
      "env": {
        "SERVICE_NAME": "service-b"
      }
    }
  ]
}
```

### Port Ranges with Load Balancer Integration

```json
{
  "apps": [
    {
      "name": "web-tier",
      "script": "node",
      "args": ["web.js"],
      "instances": 4,
      "port": "8080-8083",
      "env": {
        "TIER": "web",
        "UPSTREAM_SERVERS": "localhost:3000,localhost:3001,localhost:3002,localhost:3003"
      }
    },
    {
      "name": "api-tier",
      "script": "node",
      "args": ["api.js"],
      "instances": 4,
      "port": "3000-3003",
      "env": {
        "TIER": "api",
        "DATABASE_POOL_SIZE": "10"
      }
    }
  ]
}
```

## Sophisticated Health Checks

### Multi-Stage Health Checks

```json
{
  "name": "complex-service",
  "script": "node",
  "args": ["service.js"],
  "health_check": {
    "check_type": "script",
    "script": "./health-checks/comprehensive.sh",
    "timeout": 30,
    "interval": 45,
    "retries": 3
  }
}
```

**comprehensive.sh:**
```bash
#!/bin/bash

# Stage 1: Basic process check
if ! pgrep -f "node service.js" > /dev/null; then
    echo "Process not running"
    exit 1
fi

# Stage 2: HTTP endpoint check
if ! curl -f -s http://localhost:3000/health > /dev/null; then
    echo "HTTP health check failed"
    exit 1
fi

# Stage 3: Database connectivity
if ! node -e "require('./db').testConnection()" 2>/dev/null; then
    echo "Database connection failed"
    exit 1
fi

# Stage 4: External service dependencies
if ! curl -f -s http://external-api/status > /dev/null; then
    echo "External service unavailable"
    exit 1
fi

# Stage 5: Resource utilization check
MEMORY_USAGE=$(ps -o pid,vsz,comm -p $(pgrep -f "node service.js") | tail -1 | awk '{print $2}')
if [ "$MEMORY_USAGE" -gt 1048576 ]; then  # 1GB in KB
    echo "Memory usage too high: ${MEMORY_USAGE}KB"
    exit 1
fi

echo "All health checks passed"
exit 0
```

### Conditional Health Checks

```json
{
  "name": "adaptive-service",
  "script": "node",
  "args": ["service.js"],
  "env": {
    "HEALTH_CHECK_MODE": "comprehensive"
  },
  "health_check": {
    "check_type": "script",
    "script": "./health-checks/adaptive.sh",
    "timeout": 20,
    "interval": 30
  }
}
```

**adaptive.sh:**
```bash
#!/bin/bash

MODE=${HEALTH_CHECK_MODE:-basic}

case $MODE in
    "basic")
        curl -f -s http://localhost:3000/ping > /dev/null
        ;;
    "standard")
        curl -f -s http://localhost:3000/health > /dev/null
        ;;
    "comprehensive")
        # Full health check suite
        ./health-checks/comprehensive.sh
        ;;
    *)
        echo "Unknown health check mode: $MODE"
        exit 1
        ;;
esac
```

## Resource Management Strategies

### Memory-Aware Scaling

```json
{
  "apps": [
    {
      "name": "memory-intensive-service",
      "script": "python",
      "args": ["ml_service.py"],
      "instances": 2,
      "max_memory_restart": "2G",
      "env": {
        "MEMORY_LIMIT": "2048",
        "BATCH_SIZE": "100"
      }
    },
    {
      "name": "lightweight-service",
      "script": "node",
      "args": ["api.js"],
      "instances": 8,
      "max_memory_restart": "256M",
      "env": {
        "MEMORY_LIMIT": "256",
        "CACHE_SIZE": "50"
      }
    }
  ]
}
```

### CPU-Aware Instance Distribution

```json
{
  "apps": [
    {
      "name": "cpu-bound-service",
      "script": "python",
      "args": ["compute.py"],
      "instances": "max",
      "env": {
        "WORKER_TYPE": "cpu_intensive",
        "THREAD_POOL_SIZE": "1"
      }
    },
    {
      "name": "io-bound-service",
      "script": "node",
      "args": ["io_service.js"],
      "instances": 4,
      "env": {
        "WORKER_TYPE": "io_intensive",
        "CONNECTION_POOL_SIZE": "100"
      }
    }
  ]
}
```

## Advanced Logging Patterns

### Structured Logging Configuration

```json
{
  "name": "structured-service",
  "script": "node",
  "args": ["service.js"],
  "env": {
    "LOG_FORMAT": "json",
    "LOG_LEVEL": "info"
  },
  "out_file": "/var/log/services/structured-service.json",
  "error_file": "/var/log/services/structured-service-error.json",
  "log_date_format": "YYYY-MM-DD HH:mm:ss.SSS Z"
}
```

### Log Rotation and Management

```json
{
  "name": "high-volume-service",
  "script": "node",
  "args": ["service.js"],
  "log": {
    "out_file": "/var/log/services/high-volume.out",
    "error_file": "/var/log/services/high-volume.err",
    "max_log_size": "100M",
    "max_log_files": 10,
    "compress_logs": true
  }
}
```

## Service Dependencies and Orchestration

### Dependency-Aware Startup

```json
{
  "apps": [
    {
      "name": "database",
      "script": "postgres",
      "args": ["-D", "/var/lib/postgresql/data"],
      "autorestart": true,
      "health_check": {
        "check_type": "script",
        "script": "./health-checks/postgres.sh",
        "timeout": 10,
        "interval": 30
      }
    },
    {
      "name": "redis",
      "script": "redis-server",
      "args": ["/etc/redis/redis.conf"],
      "autorestart": true,
      "health_check": {
        "check_type": "script",
        "script": "./health-checks/redis.sh",
        "timeout": 5,
        "interval": 30
      }
    },
    {
      "name": "api-service",
      "script": "node",
      "args": ["api.js"],
      "instances": 2,
      "port": "3000-3001",
      "depends_on": ["database", "redis"],
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health",
        "timeout": 10,
        "interval": 30
      }
    }
  ]
}
```

### Graceful Shutdown Orchestration

```json
{
  "name": "graceful-service",
  "script": "node",
  "args": ["service.js"],
  "kill_timeout": "60s",
  "env": {
    "GRACEFUL_SHUTDOWN_TIMEOUT": "50000"
  }
}
```

**service.js:**
```javascript
process.on('SIGTERM', async () => {
  console.log('Received SIGTERM, starting graceful shutdown...');
  
  // Stop accepting new requests
  server.close();
  
  // Finish processing existing requests
  await finishPendingRequests();
  
  // Close database connections
  await database.close();
  
  // Clean up resources
  await cleanup();
  
  console.log('Graceful shutdown complete');
  process.exit(0);
});
```

## Configuration Inheritance and Composition

### Base Configuration Template

**base.json:**
```json
{
  "base_config": {
    "autorestart": true,
    "max_restarts": 10,
    "min_uptime": "5s",
    "restart_delay": "1s",
    "kill_timeout": "30s",
    "env": {
      "NODE_ENV": "production",
      "LOG_LEVEL": "info"
    }
  }
}
```

### Service-Specific Extensions

**web-service.json:**
```json
{
  "extends": "./base.json",
  "name": "web-service",
  "script": "node",
  "args": ["web.js"],
  "instances": 4,
  "port": "3000-3003",
  "env": {
    "SERVICE_TYPE": "web"
  },
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health"
  }
}
```

## Performance Optimization Patterns

### Instance-Specific Optimization

```json
{
  "apps": [
    {
      "name": "optimized-service",
      "script": "node",
      "args": ["--max-old-space-size=1024", "service.js"],
      "instances": 4,
      "env": {
        "UV_THREADPOOL_SIZE": "16",
        "NODE_OPTIONS": "--max-old-space-size=1024"
      }
    }
  ]
}
```

### Resource Pool Management

```json
{
  "name": "pooled-service",
  "script": "node",
  "args": ["service.js"],
  "instances": 4,
  "env": {
    "DB_POOL_SIZE": "25",
    "REDIS_POOL_SIZE": "10",
    "HTTP_KEEP_ALIVE_TIMEOUT": "5000"
  }
}
```

## Security Hardening

### Secure Process Configuration

```json
{
  "name": "secure-service",
  "script": "node",
  "args": ["service.js"],
  "user": "app-user",
  "group": "app-group",
  "env": {
    "NODE_ENV": "production",
    "SECURE_MODE": "true"
  },
  "cwd": "/opt/secure-app",
  "umask": "0027"
}
```

### Environment Variable Security

```json
{
  "name": "secure-api",
  "script": "node",
  "args": ["api.js"],
  "env_file": "/etc/secrets/api.env",
  "env": {
    "CONFIG_FILE": "/etc/app/config.json"
  }
}
```

**/etc/secrets/api.env:**
```bash
DATABASE_PASSWORD=secret123
JWT_SECRET=supersecret
API_KEY=private-key
```

## Monitoring and Observability

### Comprehensive Monitoring Setup

```json
{
  "name": "monitored-service",
  "script": "node",
  "args": ["service.js"],
  "env": {
    "METRICS_PORT": "9090",
    "TRACING_ENABLED": "true",
    "LOG_STRUCTURED": "true"
  },
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 10,
    "interval": 30
  }
}
```

## Best Practices for Advanced Configuration

### 1. Use Configuration Validation

```bash
# Validate configuration before deployment
pmdaemon --config ecosystem.json validate
```

### 2. Implement Gradual Rollouts

```json
{
  "apps": [
    {
      "name": "service-v1",
      "script": "node",
      "args": ["service-v1.js"],
      "instances": 3,
      "port": "3000-3002"
    },
    {
      "name": "service-v2",
      "script": "node", 
      "args": ["service-v2.js"],
      "instances": 1,
      "port": "3003"
    }
  ]
}
```

### 3. Use Namespace Organization

```json
{
  "apps": [
    {
      "name": "api",
      "namespace": "production",
      "script": "node",
      "args": ["api.js"]
    },
    {
      "name": "worker",
      "namespace": "production", 
      "script": "python",
      "args": ["worker.py"]
    }
  ]
}
```

### 4. Implement Circuit Breakers

```javascript
// In your application code
const CircuitBreaker = require('opossum');

const options = {
  timeout: 3000,
  errorThresholdPercentage: 50,
  resetTimeout: 30000
};

const breaker = new CircuitBreaker(callExternalService, options);
```

## Next Steps

- **[Schema Validation](./schema-validation.md)** - Configuration validation details
- **[Process Configuration](./process-configuration.md)** - Individual process settings
- **[Security](../advanced/security.md)** - Security best practices
- **[Performance Tuning](../advanced/performance-tuning.md)** - Optimization strategies
