# Clustering Examples

This guide demonstrates how to implement clustering and load balancing with PMDaemon, including port management strategies for multi-instance applications.

## Basic Clustering Setup

### Simple Cluster Configuration

```json
{
  "apps": [
    {
      "name": "web-cluster",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "env": {
        "NODE_ENV": "production",
        "PORT": 3000
      }
    }
  ]
}
```

### Auto-Scaling Based on CPU Cores

```json
{
  "apps": [
    {
      "name": "auto-cluster",
      "script": "app.js",
      "instances": "max",           // Use all available CPU cores
      "exec_mode": "cluster",
      "max_memory_restart": "1G",
      "env": {
        "NODE_ENV": "production"
      }
    }
  ]
}
```

## Advanced Port Management

### 1. Port Increment Strategy

```json
{
  "apps": [
    {
      "name": "multi-port-app",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "increment_var": "PORT",      // Auto-increment PORT for each instance
      "env": {
        "PORT": 3000                // Starting port: 3000, 3001, 3002, 3003
      }
    }
  ]
}
```

### 2. Dynamic Port Allocation

```json
{
  "apps": [
    {
      "name": "dynamic-ports",
      "script": "server.js",
      "instances": 3,
      "exec_mode": "cluster",
      "port": {
        "start": 3000,
        "increment": 1,
        "max": 3010
      },
      "env": {
        "NODE_ENV": "production"
      }
    }
  ]
}
```

### 3. Port Range with Load Balancer

```json
{
  "apps": [
    {
      "name": "backend-cluster",
      "script": "api-server.js",
      "instances": 6,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 4000,              // Ports: 4000-4005
        "LOAD_BALANCER": "http://localhost:8080"
      }
    },
    {
      "name": "load-balancer",
      "script": "nginx-proxy.js",
      "instances": 1,
      "env": {
        "PORT": 8080,
        "BACKEND_PORTS": "4000,4001,4002,4003,4004,4005"
      }
    }
  ]
}
```

## Microservices Architecture

### Multi-Service Cluster

```json
{
  "apps": [
    {
      "name": "user-service",
      "script": "services/user/server.js",
      "instances": 2,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3001,
        "SERVICE_NAME": "user-service",
        "DATABASE_URL": "mongodb://localhost:27017/users"
      }
    },
    {
      "name": "order-service",
      "script": "services/order/server.js",
      "instances": 3,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3100,
        "SERVICE_NAME": "order-service",
        "DATABASE_URL": "mongodb://localhost:27017/orders"
      }
    },
    {
      "name": "notification-service",
      "script": "services/notification/server.js",
      "instances": 1,
      "env": {
        "PORT": 3200,
        "SERVICE_NAME": "notification-service",
        "REDIS_URL": "redis://localhost:6379"
      }
    },
    {
      "name": "api-gateway",
      "script": "gateway/server.js",
      "instances": 2,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 8080,
        "USER_SERVICE": "http://localhost:3001",
        "ORDER_SERVICE": "http://localhost:3100",
        "NOTIFICATION_SERVICE": "http://localhost:3200"
      }
    }
  ]
}
```

## Load Balancing Strategies

### 1. Round Robin with Health Checks

```json
{
  "apps": [
    {
      "name": "web-app",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3000
      },
      "health_check": {
        "enabled": true,
        "url": "http://localhost:{{PORT}}/health",
        "interval": 30,
        "timeout": 10,
        "retries": 3
      }
    }
  ]
}
```

### 2. Weighted Load Balancing

```json
{
  "apps": [
    {
      "name": "high-priority-instance",
      "script": "server.js",
      "instances": 2,
      "exec_mode": "cluster",
      "weight": 3,                  // Higher weight for more traffic
      "increment_var": "PORT",
      "env": {
        "PORT": 3000,
        "PRIORITY": "high"
      }
    },
    {
      "name": "standard-instance",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "weight": 1,                  // Standard weight
      "increment_var": "PORT",
      "env": {
        "PORT": 3010,
        "PRIORITY": "standard"
      }
    }
  ]
}
```

### 3. Geographic Load Balancing

```json
{
  "apps": [
    {
      "name": "us-east-cluster",
      "script": "server.js",
      "instances": 3,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3000,
        "REGION": "us-east-1",
        "DATABASE_URL": "mongodb://us-east-db:27017/app"
      }
    },
    {
      "name": "us-west-cluster",
      "script": "server.js",
      "instances": 3,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3100,
        "REGION": "us-west-1",
        "DATABASE_URL": "mongodb://us-west-db:27017/app"
      }
    },
    {
      "name": "eu-cluster",
      "script": "server.js",
      "instances": 2,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3200,
        "REGION": "eu-west-1",
        "DATABASE_URL": "mongodb://eu-db:27017/app"
      }
    }
  ]
}
```

## High Availability Patterns

### 1. Blue-Green Deployment

```json
{
  "apps": [
    {
      "name": "blue-deployment",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3000,
        "DEPLOYMENT": "blue",
        "DATABASE_URL": "mongodb://localhost:27017/app_blue"
      },
      "enabled": true
    },
    {
      "name": "green-deployment",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3100,
        "DEPLOYMENT": "green",
        "DATABASE_URL": "mongodb://localhost:27017/app_green"
      },
      "enabled": false              // Start disabled for deployment
    }
  ]
}
```

### 2. Canary Deployment

```json
{
  "apps": [
    {
      "name": "stable-version",
      "script": "server.js",
      "instances": 9,               // 90% of traffic
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3000,
        "VERSION": "v1.0.0"
      }
    },
    {
      "name": "canary-version",
      "script": "server-new.js",
      "instances": 1,               // 10% of traffic
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3010,
        "VERSION": "v1.1.0"
      }
    }
  ]
}
```

### 3. Circuit Breaker Pattern

```json
{
  "apps": [
    {
      "name": "primary-service",
      "script": "primary-server.js",
      "instances": 3,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3000,
        "CIRCUIT_BREAKER": "enabled",
        "FALLBACK_SERVICE": "http://localhost:3100"
      },
      "health_check": {
        "enabled": true,
        "url": "http://localhost:{{PORT}}/health",
        "interval": 10,
        "timeout": 5,
        "retries": 3
      }
    },
    {
      "name": "fallback-service",
      "script": "fallback-server.js",
      "instances": 2,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3100,
        "SERVICE_TYPE": "fallback"
      }
    }
  ]
}
```

## Container-Based Clustering

### 1. Docker Swarm Integration

```json
{
  "apps": [
    {
      "name": "swarm-app",
      "script": "server.js",
      "instances": "max",
      "exec_mode": "cluster",
      "env": {
        "PORT": 3000,
        "SWARM_NODE_ID": "${HOSTNAME}",
        "SWARM_SERVICE": "web-app"
      },
      "docker": {
        "image": "myapp:latest",
        "network": "overlay",
        "replicas": 6
      }
    }
  ]
}
```

### 2. Kubernetes-Style Deployment

```json
{
  "apps": [
    {
      "name": "k8s-style-app",
      "script": "server.js",
      "instances": 3,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3000,
        "POD_NAME": "${HOSTNAME}",
        "NAMESPACE": "production"
      },
      "labels": {
        "app": "web-server",
        "version": "v1.0.0",
        "environment": "production"
      },
      "health_check": {
        "enabled": true,
        "url": "http://localhost:{{PORT}}/readiness",
        "interval": 30
      }
    }
  ]
}
```

## Monitoring Clustered Applications

### 1. Centralized Logging

```json
{
  "apps": [
    {
      "name": "clustered-app",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "instance_var": "INSTANCE_ID",
      "env": {
        "PORT": 3000
      },
      "log_file": "/var/log/app/app-{{INSTANCE_ID}}.log",
      "error_file": "/var/log/app/error-{{INSTANCE_ID}}.log",
      "merge_logs": true,
      "log_type": "json"
    }
  ]
}
```

### 2. Metrics Collection

```json
{
  "apps": [
    {
      "name": "metrics-app",
      "script": "server.js",
      "instances": 3,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3000,
        "METRICS_PORT": 9090
      },
      "monitoring": {
        "enabled": true,
        "port": 9615,
        "metrics": {
          "cpu": true,
          "memory": true,
          "network": true,
          "custom": true
        }
      }
    }
  ]
}
```

## Performance Optimization

### 1. CPU-Optimized Clustering

```json
{
  "apps": [
    {
      "name": "cpu-intensive-app",
      "script": "cpu-worker.js",
      "instances": "max",           // Use all CPU cores
      "exec_mode": "cluster",
      "node_args": ["--optimize-for-size"],
      "env": {
        "UV_THREADPOOL_SIZE": 16,   // Increase thread pool
        "NODE_OPTIONS": "--max-old-space-size=2048"
      },
      "max_memory_restart": "2G"
    }
  ]
}
```

### 2. I/O-Optimized Clustering

```json
{
  "apps": [
    {
      "name": "io-intensive-app",
      "script": "io-worker.js",
      "instances": 2,               // Fewer instances for I/O bound
      "exec_mode": "cluster",
      "env": {
        "UV_THREADPOOL_SIZE": 32,   // Large thread pool for I/O
        "DATABASE_POOL_SIZE": 20
      }
    }
  ]
}
```

### 3. Memory-Optimized Clustering

```json
{
  "apps": [
    {
      "name": "memory-efficient-app",
      "script": "server.js",
      "instances": 8,               // More instances, less memory each
      "exec_mode": "cluster",
      "max_memory_restart": "512M", // Smaller memory limit per instance
      "node_args": ["--max-old-space-size=400"],
      "env": {
        "NODE_OPTIONS": "--optimize-for-size"
      }
    }
  ]
}
```

## Troubleshooting Clustered Applications

### 1. Port Conflict Resolution

```bash
# Check port availability
pmdaemon ports --check --range 3000-3010

# Start with automatic port detection
pmdaemon start ecosystem.json --auto-ports

# Force port allocation
pmdaemon start ecosystem.json --force-ports
```

### 2. Load Distribution Analysis

```bash
# Monitor cluster load distribution
pmdaemon monitor --cluster web-cluster

# Check instance health
pmdaemon health --all-instances

# View port allocation
pmdaemon list --show-ports
```

### 3. Performance Monitoring

```bash
# Monitor cluster performance
pmdaemon monitor --cpu --memory --cluster

# Check load balancing effectiveness
pmdaemon stats --load-balance

# Analyze instance utilization
pmdaemon analyze --cluster-efficiency
```

## Deployment Scripts

### 1. Cluster Deployment Script

```bash
#!/bin/bash
# deploy-cluster.sh

set -e

CONFIG_FILE="ecosystem.cluster.json"
APP_NAME="web-cluster"

echo "Deploying cluster: $APP_NAME"

# Validate configuration
pmdaemon validate "$CONFIG_FILE"

# Check port availability
pmdaemon ports --check --range 3000-3010

# Deploy cluster
pmdaemon start "$CONFIG_FILE"

# Wait for all instances to be ready
sleep 10

# Verify cluster health
pmdaemon health --app "$APP_NAME" --all-instances

# Test load distribution
for i in {1..10}; do
  curl -s "http://localhost:8080/health" > /dev/null
  echo "Health check $i: OK"
done

echo "Cluster deployment completed successfully"
```

### 2. Rolling Update Script

```bash
#!/bin/bash
# rolling-update.sh

set -e

APP_NAME="web-cluster"
NEW_VERSION="$1"

if [ -z "$NEW_VERSION" ]; then
  echo "Usage: $0 <new-version>"
  exit 1
fi

echo "Performing rolling update to version: $NEW_VERSION"

# Get current instances
INSTANCES=$(pmdaemon list --app "$APP_NAME" --format json | jq -r '.[] | .instance_id')

# Update instances one by one
for instance in $INSTANCES; do
  echo "Updating instance: $instance"

  # Stop instance
  pmdaemon stop "$APP_NAME" --instance "$instance"

  # Update code (deployment specific)
  deploy_new_version "$NEW_VERSION"

  # Start instance
  pmdaemon start "$APP_NAME" --instance "$instance"

  # Wait for health check
  sleep 30

  # Verify instance health
  if ! pmdaemon health "$APP_NAME" --instance "$instance"; then
    echo "Health check failed for instance: $instance"
    exit 1
  fi

  echo "Instance $instance updated successfully"
done

echo "Rolling update completed successfully"
```

## Related Documentation

- **[Port Management](../features/port-management.md)** - Advanced port management features
- **[Load Balancing](../features/load-balancing.md)** - Load balancing strategies
- **[Monitoring](../monitoring/overview.md)** - Monitoring clustered applications
- **[Performance](../performance/optimization.md)** - Performance optimization techniques
