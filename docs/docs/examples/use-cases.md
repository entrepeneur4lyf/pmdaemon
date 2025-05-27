# Real-World Use Cases

This guide showcases practical, real-world scenarios where PMDaemon excels, demonstrating how its advanced features solve common deployment and management challenges.

## Web Application Deployment

### Scenario: E-commerce Platform

Deploy a Node.js e-commerce platform with multiple services, load balancing, and health monitoring.

#### Requirements
- **Frontend**: React app served by Express
- **API**: Node.js REST API with database
- **Workers**: Background job processing
- **Monitoring**: Health checks and real-time monitoring
- **Scaling**: Auto-scaling based on load

#### Configuration

```json
{
  "apps": [
    {
      "name": "ecommerce-frontend",
      "script": "node",
      "args": ["frontend-server.js"],
      "instances": 2,
      "port": "3000-3001",
      "max_memory_restart": "512M",
      "env": {
        "NODE_ENV": "production",
        "API_URL": "http://localhost:4000"
      },
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health",
        "timeout": 5,
        "interval": 30,
        "retries": 3,
        "enabled": true
      }
    },
    {
      "name": "ecommerce-api",
      "script": "node",
      "args": ["api-server.js"],
      "instances": 4,
      "port": "4000-4003",
      "max_memory_restart": "1G",
      "env": {
        "NODE_ENV": "production",
        "DATABASE_URL": "postgres://localhost/ecommerce",
        "REDIS_URL": "redis://localhost:6379"
      },
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:4000/api/health",
        "timeout": 10,
        "interval": 30,
        "retries": 3,
        "enabled": true
      }
    },
    {
      "name": "order-processor",
      "script": "node",
      "args": ["workers/order-processor.js"],
      "instances": 2,
      "max_memory_restart": "256M",
      "env": {
        "NODE_ENV": "production",
        "QUEUE_NAME": "order_processing",
        "REDIS_URL": "redis://localhost:6379"
      },
      "health_check": {
        "check_type": "script",
        "script": "./scripts/worker-health.sh",
        "timeout": 15,
        "interval": 60,
        "retries": 2,
        "enabled": true
      }
    },
    {
      "name": "email-worker",
      "script": "node",
      "args": ["workers/email-worker.js"],
      "max_memory_restart": "128M",
      "env": {
        "NODE_ENV": "production",
        "SMTP_HOST": "smtp.example.com",
        "QUEUE_NAME": "email_queue"
      }
    }
  ]
}
```

#### Deployment Script

```bash
#!/bin/bash
# deploy-ecommerce.sh

set -e

echo "🚀 Deploying E-commerce Platform..."

# Stop existing processes
pmdaemon delete all --force 2>/dev/null || true

# Start services with health check waiting
echo "📦 Starting API services..."
pmdaemon --config ecommerce.json start --name ecommerce-api --wait-ready

echo "🌐 Starting frontend..."
pmdaemon --config ecommerce.json start --name ecommerce-frontend --wait-ready

echo "⚙️ Starting background workers..."
pmdaemon --config ecommerce.json start --name order-processor
pmdaemon --config ecommerce.json start --name email-worker

echo "✅ Deployment complete!"
echo "🔍 Monitoring: pmdaemon monit"
echo "📊 Web UI: pmdaemon web --host 0.0.0.0"

# Show status
pmdaemon list
```

## Microservices Architecture

### Scenario: Financial Services Platform

Deploy a microservices-based financial platform with strict health monitoring and auto-scaling.

#### Architecture
- **API Gateway**: Route requests to services
- **User Service**: Authentication and user management
- **Transaction Service**: Payment processing
- **Notification Service**: Email/SMS notifications
- **Audit Service**: Compliance and logging

#### Configuration

```yaml
# financial-platform.yaml
apps:
  # API Gateway
  - name: api-gateway
    script: node
    args: [gateway/server.js]
    port: "8080"
    max_memory_restart: "512M"
    env:
      NODE_ENV: production
      LOG_LEVEL: info
      RATE_LIMIT: "1000"
    health_check:
      check_type: http
      url: http://localhost:8080/health
      timeout: 5
      interval: 15
      retries: 3
      enabled: true

  # User Service
  - name: user-service
    script: node
    args: [services/user/server.js]
    instances: 3
    port: "8001-8003"
    max_memory_restart: "256M"
    env:
      NODE_ENV: production
      DATABASE_URL: postgres://localhost/users
      JWT_SECRET: ${JWT_SECRET}
    health_check:
      check_type: http
      url: http://localhost:8001/health
      timeout: 5
      interval: 30
      enabled: true

  # Transaction Service (Critical)
  - name: transaction-service
    script: node
    args: [services/transaction/server.js]
    instances: 4
    port: "8004-8007"
    max_memory_restart: "512M"
    max_restarts: 3
    min_uptime: "30s"
    env:
      NODE_ENV: production
      DATABASE_URL: postgres://localhost/transactions
      ENCRYPTION_KEY: ${ENCRYPTION_KEY}
    health_check:
      check_type: http
      url: http://localhost:8004/health
      timeout: 3
      interval: 10
      retries: 5
      enabled: true

  # Notification Service
  - name: notification-service
    script: python
    args: [-m, uvicorn, main:app, --host, "0.0.0.0"]
    instances: 2
    port: "8008-8009"
    max_memory_restart: "256M"
    cwd: /app/services/notification
    env:
      PYTHONPATH: /app/services/notification
      REDIS_URL: redis://localhost:6379
      EMAIL_PROVIDER: sendgrid
    health_check:
      check_type: http
      url: http://localhost:8008/health
      timeout: 10
      interval: 30
      enabled: true

  # Audit Service
  - name: audit-service
    script: java
    args: [-jar, audit-service.jar]
    port: "8010"
    max_memory_restart: "1G"
    cwd: /app/services/audit
    env:
      SPRING_PROFILES_ACTIVE: production
      DATABASE_URL: postgres://localhost/audit
    health_check:
      check_type: http
      url: http://localhost:8010/actuator/health
      timeout: 15
      interval: 60
      enabled: true
```

#### Monitoring Setup

```bash
#!/bin/bash
# monitor-financial-platform.sh

# Start monitoring dashboard
pmdaemon web --host 0.0.0.0 --port 9615 &

# Real-time monitoring with fast updates for critical services
pmdaemon monit --interval 1 &

# Log monitoring for transaction service
pmdaemon logs transaction-service --follow &

echo "🔍 Monitoring started:"
echo "  - Web Dashboard: http://localhost:9615"
echo "  - Real-time CLI: pmdaemon monit"
echo "  - Transaction Logs: Following in background"
```

## Development Environment

### Scenario: Full-Stack Development Setup

Set up a complete development environment with hot-reloading, database, and external services.

#### Services
- **Frontend**: React development server
- **Backend**: Node.js API with hot-reload
- **Database**: PostgreSQL
- **Redis**: Caching and sessions
- **Worker**: Background job processing

#### Configuration

```json
{
  "apps": [
    {
      "name": "frontend-dev",
      "script": "npm",
      "args": ["run", "dev"],
      "port": "3000",
      "cwd": "./frontend",
      "env": {
        "NODE_ENV": "development",
        "REACT_APP_API_URL": "http://localhost:4000"
      },
      "autorestart": false,
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000",
        "timeout": 10,
        "interval": 30,
        "enabled": true
      }
    },
    {
      "name": "backend-dev",
      "script": "npm",
      "args": ["run", "dev"],
      "port": "4000",
      "cwd": "./backend",
      "env": {
        "NODE_ENV": "development",
        "DATABASE_URL": "postgres://localhost/myapp_dev",
        "REDIS_URL": "redis://localhost:6379"
      },
      "autorestart": false,
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:4000/health",
        "timeout": 5,
        "interval": 30,
        "enabled": true
      }
    },
    {
      "name": "postgres-dev",
      "script": "docker",
      "args": [
        "run", "--rm", "--name", "postgres-dev",
        "-p", "5432:5432",
        "-e", "POSTGRES_DB=myapp_dev",
        "-e", "POSTGRES_USER=dev",
        "-e", "POSTGRES_PASSWORD=dev123",
        "postgres:14"
      ],
      "autorestart": true,
      "health_check": {
        "check_type": "script",
        "script": "./scripts/postgres-health.sh",
        "timeout": 10,
        "interval": 30,
        "enabled": true
      }
    },
    {
      "name": "redis-dev",
      "script": "docker",
      "args": [
        "run", "--rm", "--name", "redis-dev",
        "-p", "6379:6379",
        "redis:7-alpine"
      ],
      "autorestart": true,
      "health_check": {
        "check_type": "script",
        "script": "./scripts/redis-health.sh",
        "timeout": 5,
        "interval": 30,
        "enabled": true
      }
    }
  ]
}
```

#### Development Scripts

```bash
#!/bin/bash
# dev-start.sh

echo "🚀 Starting development environment..."

# Start infrastructure first
echo "📦 Starting databases..."
pmdaemon --config dev.json start --name postgres-dev --wait-ready
pmdaemon --config dev.json start --name redis-dev --wait-ready

# Start application services
echo "🔧 Starting backend..."
pmdaemon --config dev.json start --name backend-dev --wait-ready

echo "🌐 Starting frontend..."
pmdaemon --config dev.json start --name frontend-dev --wait-ready

echo "✅ Development environment ready!"
echo "  - Frontend: http://localhost:3000"
echo "  - Backend: http://localhost:4000"
echo "  - Monitor: pmdaemon monit"

# Open browser
open http://localhost:3000
```

## CI/CD Pipeline Integration

### Scenario: Automated Deployment Pipeline

Integrate PMDaemon with CI/CD for automated testing and deployment.

#### GitHub Actions Workflow

```yaml
# .github/workflows/deploy.yml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Install PMDaemon
      run: |
        curl -sSL https://github.com/entrepeneur4lyf/pmdaemon/releases/latest/download/pmdaemon-linux-x86_64.tar.gz | tar -xz
        sudo mv pmdaemon /usr/local/bin/

    - name: Deploy Application
      run: |
        # Copy config to server
        scp production.json user@server:/app/

        # Deploy with health check waiting
        ssh user@server << 'EOF'
          cd /app

          # Graceful deployment
          pmdaemon --config production.json start --name api-server --wait-ready
          pmdaemon --config production.json start --name frontend --wait-ready

          # Verify deployment
          pmdaemon list

          # Run health checks
          curl -f http://localhost:3000/health
          curl -f http://localhost:4000/api/health
        EOF
```

#### Deployment Script with Rollback

```bash
#!/bin/bash
# deploy-with-rollback.sh

set -e

BACKUP_CONFIG="/tmp/pmdaemon-backup-$(date +%s).json"
NEW_CONFIG="production.json"

echo "🚀 Starting deployment..."

# Backup current configuration
echo "💾 Backing up current configuration..."
pmdaemon list --json > "$BACKUP_CONFIG"

# Function to rollback on failure
rollback() {
    echo "❌ Deployment failed, rolling back..."
    pmdaemon delete all --force
    pmdaemon --config "$BACKUP_CONFIG" start
    exit 1
}

# Set trap for rollback
trap rollback ERR

# Deploy new configuration
echo "📦 Deploying new configuration..."
pmdaemon --config "$NEW_CONFIG" start --wait-ready

# Verify deployment
echo "🔍 Verifying deployment..."
sleep 10

# Health check all services
for service in api-server frontend worker; do
    if ! pmdaemon info "$service" | grep -q "online"; then
        echo "❌ Service $service is not healthy"
        rollback
    fi
done

# Cleanup old processes if everything is healthy
echo "🧹 Cleaning up old processes..."
# Implementation depends on your deployment strategy

echo "✅ Deployment successful!"
echo "📊 Monitor: pmdaemon web --host 0.0.0.0"
```

## Container Orchestration

### Scenario: Docker Swarm Alternative

Use PMDaemon as a lightweight alternative to Docker Swarm for container orchestration.

#### Configuration

```yaml
# container-orchestration.yaml
apps:
  # Load Balancer
  - name: nginx-lb
    script: docker
    args:
      - run
      - --rm
      - --name
      - nginx-lb
      - -p
      - "80:80"
      - -v
      - ./nginx.conf:/etc/nginx/nginx.conf
      - nginx:alpine
    health_check:
      check_type: http
      url: http://localhost:80/health
      timeout: 5
      interval: 30
      enabled: true

  # Web Application Containers
  - name: web-app
    script: docker
    args:
      - run
      - --rm
      - --name
      - web-app-${PM2_INSTANCE_ID}
      - -p
      - "${PORT}:3000"
      - myapp:latest
    instances: 3
    port: "8001-8003"
    health_check:
      check_type: http
      url: http://localhost:${PORT}/health
      timeout: 10
      interval: 30
      enabled: true

  # Database
  - name: postgres
    script: docker
    args:
      - run
      - --rm
      - --name
      - postgres-prod
      - -p
      - "5432:5432"
      - -e
      - POSTGRES_DB=myapp
      - -v
      - postgres-data:/var/lib/postgresql/data
      - postgres:14
    health_check:
      check_type: script
      script: ./scripts/postgres-health.sh
      timeout: 15
      interval: 60
      enabled: true

  # Redis Cache
  - name: redis
    script: docker
    args:
      - run
      - --rm
      - --name
      - redis-prod
      - -p
      - "6379:6379"
      - redis:7-alpine
    health_check:
      check_type: script
      script: ./scripts/redis-health.sh
      timeout: 5
      interval: 30
      enabled: true
```

## Multi-Language Application Stack

### Scenario: Polyglot Microservices

Deploy a multi-language application stack with different runtime requirements.

#### Configuration

```toml
# polyglot-stack.toml

# Node.js API Gateway
[[apps]]
name = "api-gateway"
script = "node"
args = ["gateway.js"]
port = "8080"
max_memory_restart = "512M"

[apps.env]
NODE_ENV = "production"

[apps.health_check]
check_type = "http"
url = "http://localhost:8080/health"
timeout = 5
interval = 30
enabled = true

# Python ML Service
[[apps]]
name = "ml-service"
script = "python"
args = ["-m", "uvicorn", "main:app", "--host", "0.0.0.0"]
port = "8001"
max_memory_restart = "2G"
cwd = "/app/ml-service"

[apps.env]
PYTHONPATH = "/app/ml-service"
MODEL_PATH = "/models/latest"

[apps.health_check]
check_type = "http"
url = "http://localhost:8001/health"
timeout = 15
interval = 60
enabled = true

# Go Analytics Service
[[apps]]
name = "analytics-service"
script = "./analytics-server"
port = "8002"
max_memory_restart = "1G"
cwd = "/app/analytics"

[apps.env]
GO_ENV = "production"
DATABASE_URL = "postgres://localhost/analytics"

[apps.health_check]
check_type = "http"
url = "http://localhost:8002/metrics"
timeout = 10
interval = 30
enabled = true

# Rust High-Performance Service
[[apps]]
name = "performance-service"
script = "./target/release/performance-server"
instances = 2
port = "8003-8004"
max_memory_restart = "256M"
cwd = "/app/performance-service"

[apps.env]
RUST_ENV = "production"
WORKER_THREADS = "4"

[apps.health_check]
check_type = "http"
url = "http://localhost:8003/status"
timeout = 3
interval = 15
enabled = true

# Java Spring Boot Service
[[apps]]
name = "legacy-service"
script = "java"
args = ["-Xmx1g", "-jar", "legacy-service.jar"]
port = "8005"
max_memory_restart = "1.5G"
cwd = "/app/legacy"

[apps.env]
SPRING_PROFILES_ACTIVE = "production"
JVM_OPTS = "-XX:+UseG1GC"

[apps.health_check]
check_type = "http"
url = "http://localhost:8005/actuator/health"
timeout = 20
interval = 60
enabled = true
```

## Best Practices Summary

### 1. Health Check Strategy
- **Critical services**: Fast, frequent checks (10-15s intervals)
- **Background services**: Slower checks (60s+ intervals)
- **Use blocking start** for deployment scripts
- **Script-based checks** for complex validation

### 2. Resource Management
- **Set memory limits** for all services
- **Use appropriate restart policies** based on service criticality
- **Monitor resource usage** with `pmdaemon monit`

### 3. Port Management
- **Use port ranges** for clustered services
- **Auto-assignment** for dynamic scaling
- **Runtime overrides** for blue-green deployments

### 4. Configuration Management
- **Environment-specific configs** for different stages
- **Version control** all configuration files
- **Validate configs** before deployment

### 5. Monitoring and Logging
- **Centralized logging** with proper log rotation
- **Real-time monitoring** during deployments
- **Web dashboard** for remote monitoring

---

These real-world use cases demonstrate PMDaemon's flexibility and advanced features in practical scenarios. The combination of health checks, port management, and robust process lifecycle management makes PMDaemon ideal for modern application deployment and management.
