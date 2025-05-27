# Deployment Examples

This guide provides real-world deployment examples for PMDaemon across different environments and use cases. From simple single-server deployments to complex multi-tier architectures, these examples will help you deploy your applications effectively.

## Single Server Deployments

### Basic Web Application

**Scenario:** Deploy a Node.js web application with health checks and monitoring.

**ecosystem.json:**
```json
{
  "apps": [
    {
      "name": "web-app",
      "script": "node",
      "args": ["dist/server.js"],
      "instances": 2,
      "port": "3000-3001",
      "cwd": "/var/www/myapp",
      "env": {
        "NODE_ENV": "production",
        "DATABASE_URL": "postgres://localhost/myapp",
        "REDIS_URL": "redis://localhost:6379"
      },
      "max_memory_restart": "512M",
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

**Deployment script:**
```bash
#!/bin/bash
# deploy.sh

set -e

echo "🚀 Deploying web application..."

# Stop existing processes
pmdaemon delete web-app 2>/dev/null || true

# Pull latest code
cd /var/www/myapp
git pull origin main

# Install dependencies and build
npm ci --production
npm run build

# Start with health check validation
pmdaemon --config ecosystem.json start --wait-ready

echo "✅ Deployment complete!"
echo "🌐 Application available at http://localhost:3000"
```

### Python API Service

**Scenario:** Deploy a FastAPI service with worker processes.

**ecosystem.yaml:**
```yaml
apps:
  - name: api-server
    script: python
    args: ["-m", "uvicorn", "main:app", "--host", "0.0.0.0"]
    instances: 4
    port: "8000-8003"
    cwd: /opt/api-service
    env:
      PYTHONPATH: /opt/api-service
      DATABASE_URL: postgresql://localhost/apidb
      REDIS_URL: redis://localhost:6379
    max_memory_restart: 256M
    health_check:
      check_type: http
      url: http://localhost:8000/health
      timeout: 15
      interval: 45

  - name: worker-processes
    script: python
    args: ["-m", "celery", "worker", "-A", "tasks"]
    instances: 2
    cwd: /opt/api-service
    env:
      PYTHONPATH: /opt/api-service
      CELERY_BROKER_URL: redis://localhost:6379/0
      CELERY_RESULT_BACKEND: redis://localhost:6379/0
    max_memory_restart: 512M
    health_check:
      check_type: script
      script: ./scripts/worker-health.sh
      timeout: 10
      interval: 60
```

**Health check script (worker-health.sh):**
```bash
#!/bin/bash

# Check if Celery worker is responding
python -c "
from celery import Celery
app = Celery('tasks', broker='redis://localhost:6379/0')
result = app.control.ping()
if not result:
    exit(1)
print('Worker is healthy')
"
```

## Multi-Tier Architecture

### Complete Web Stack

**Scenario:** Deploy a full web stack with frontend, API, workers, and database.

**ecosystem.production.json:**
```json
{
  "apps": [
    {
      "name": "nginx-proxy",
      "script": "nginx",
      "args": ["-g", "daemon off;"],
      "autorestart": true,
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:80/health",
        "timeout": 5,
        "interval": 30
      }
    },
    {
      "name": "frontend-servers",
      "script": "node",
      "args": ["dist/server.js"],
      "instances": 2,
      "port": "3000-3001",
      "cwd": "/var/www/frontend",
      "env": {
        "NODE_ENV": "production",
        "API_URL": "http://localhost:8000"
      },
      "max_memory_restart": "256M",
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health"
      }
    },
    {
      "name": "api-servers",
      "script": "node",
      "args": ["dist/api.js"],
      "instances": 4,
      "port": "8000-8003",
      "cwd": "/var/www/api",
      "env": {
        "NODE_ENV": "production",
        "DATABASE_URL": "postgres://db-server/myapp",
        "REDIS_URL": "redis://cache-server:6379",
        "JWT_SECRET": "${JWT_SECRET}"
      },
      "max_memory_restart": "512M",
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:8000/api/health",
        "timeout": 10,
        "interval": 30
      }
    },
    {
      "name": "background-workers",
      "script": "node",
      "args": ["dist/worker.js"],
      "instances": 3,
      "cwd": "/var/www/workers",
      "env": {
        "NODE_ENV": "production",
        "REDIS_URL": "redis://cache-server:6379",
        "EMAIL_SERVICE_URL": "https://api.sendgrid.com"
      },
      "max_memory_restart": "256M",
      "health_check": {
        "check_type": "script",
        "script": "./health-checks/worker.sh",
        "timeout": 15,
        "interval": 60
      }
    }
  ]
}
```

**Nginx configuration (/etc/nginx/sites-available/myapp):**
```nginx
upstream frontend {
    server localhost:3000;
    server localhost:3001;
}

upstream api {
    server localhost:8000;
    server localhost:8001;
    server localhost:8002;
    server localhost:8003;
}

server {
    listen 80;
    server_name myapp.com;

    location /health {
        return 200 "healthy\n";
        add_header Content-Type text/plain;
    }

    location /api/ {
        proxy_pass http://api;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    location / {
        proxy_pass http://frontend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## Microservices Architecture

### Service Mesh Deployment

**Scenario:** Deploy multiple microservices with service discovery.

**services/user-service.json:**
```json
{
  "name": "user-service",
  "script": "node",
  "args": ["dist/server.js"],
  "instances": 3,
  "port": "auto:9000-9100",
  "namespace": "microservices",
  "env": {
    "SERVICE_NAME": "user-service",
    "SERVICE_VERSION": "1.2.0",
    "DATABASE_URL": "postgres://db-cluster/users",
    "CONSUL_URL": "http://consul:8500"
  },
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:${PORT}/health",
    "timeout": 10,
    "interval": 30
  }
}
```

**services/order-service.json:**
```json
{
  "name": "order-service",
  "script": "python",
  "args": ["-m", "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "${PORT}"],
  "instances": 2,
  "port": "auto:9000-9100",
  "namespace": "microservices",
  "env": {
    "SERVICE_NAME": "order-service",
    "SERVICE_VERSION": "2.1.0",
    "DATABASE_URL": "postgres://db-cluster/orders",
    "USER_SERVICE_URL": "http://user-service:9001"
  },
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:${PORT}/health",
    "timeout": 15,
    "interval": 45
  }
}
```

**Deployment script for microservices:**
```bash
#!/bin/bash
# deploy-microservices.sh

set -e

SERVICES=("user-service" "order-service" "payment-service" "notification-service")

echo "🚀 Deploying microservices..."

for service in "${SERVICES[@]}"; do
    echo "📦 Deploying $service..."
    
    # Stop existing service
    pmdaemon delete "$service" --namespace microservices 2>/dev/null || true
    
    # Deploy new version
    pmdaemon --config "services/${service}.json" start --wait-ready
    
    # Verify deployment
    if pmdaemon info "$service" --namespace microservices | grep -q "online"; then
        echo "✅ $service deployed successfully"
    else
        echo "❌ $service deployment failed"
        pmdaemon logs "$service" --namespace microservices --lines 20
        exit 1
    fi
done

echo "🎉 All microservices deployed successfully!"
```

## Container Deployments

### Docker Compose Integration

**docker-compose.yml:**
```yaml
version: '3.8'

services:
  pmdaemon:
    image: pmdaemon:latest
    volumes:
      - ./ecosystem.docker.json:/app/ecosystem.json
      - ./app:/app/src
      - logs:/app/logs
    ports:
      - "3000-3003:3000-3003"
      - "9615:9615"
    environment:
      - NODE_ENV=production
      - DATABASE_URL=postgres://db:5432/myapp
    command: pmdaemon --config ecosystem.json start
    depends_on:
      - db
      - redis

  db:
    image: postgres:13
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: app
      POSTGRES_PASSWORD: secret
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:6-alpine
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
  logs:
```

**ecosystem.docker.json:**
```json
{
  "apps": [
    {
      "name": "web-app",
      "script": "node",
      "args": ["src/server.js"],
      "instances": 4,
      "port": "3000-3003",
      "env": {
        "NODE_ENV": "production",
        "DATABASE_URL": "postgres://db:5432/myapp",
        "REDIS_URL": "redis://redis:6379"
      },
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

### Kubernetes Deployment

**k8s/pmdaemon-deployment.yaml:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pmdaemon-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: pmdaemon-app
  template:
    metadata:
      labels:
        app: pmdaemon-app
    spec:
      containers:
      - name: pmdaemon
        image: pmdaemon:latest
        ports:
        - containerPort: 3000
        - containerPort: 9615
        env:
        - name: NODE_ENV
          value: "production"
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: app-secrets
              key: database-url
        volumeMounts:
        - name: config
          mountPath: /app/ecosystem.json
          subPath: ecosystem.json
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: pmdaemon-config
```

## Cloud Platform Deployments

### AWS EC2 with Auto Scaling

**user-data.sh (EC2 launch script):**
```bash
#!/bin/bash

# Install PMDaemon
curl -sSL https://install.pmdaemon.io | bash

# Download application
aws s3 cp s3://my-app-bucket/latest.tar.gz /tmp/
tar -xzf /tmp/latest.tar.gz -C /opt/

# Create ecosystem configuration
cat > /opt/myapp/ecosystem.json << EOF
{
  "apps": [
    {
      "name": "web-app",
      "script": "node",
      "args": ["server.js"],
      "instances": "max",
      "port": "auto:3000-3100",
      "cwd": "/opt/myapp",
      "env": {
        "NODE_ENV": "production",
        "AWS_REGION": "us-east-1"
      },
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health"
      }
    }
  ]
}
EOF

# Start application
cd /opt/myapp
pmdaemon --config ecosystem.json start --wait-ready

# Register with load balancer
aws elbv2 register-targets --target-group-arn $TARGET_GROUP_ARN \
  --targets Id=$(curl -s http://169.254.169.254/latest/meta-data/instance-id),Port=3000
```

### Google Cloud Platform

**startup-script.sh:**
```bash
#!/bin/bash

# Install PMDaemon
curl -sSL https://install.pmdaemon.io | bash

# Get application from Cloud Storage
gsutil cp gs://my-app-bucket/app.tar.gz /tmp/
tar -xzf /tmp/app.tar.gz -C /opt/

# Configure for GCP
export DATABASE_URL=$(gcloud secrets versions access latest --secret="database-url")
export REDIS_URL=$(gcloud secrets versions access latest --secret="redis-url")

# Start application
cd /opt/myapp
pmdaemon start "node server.js" --name web-app \
  --instances 4 --port 8080 \
  --env NODE_ENV=production \
  --env DATABASE_URL="$DATABASE_URL" \
  --env REDIS_URL="$REDIS_URL" \
  --health-check-url http://localhost:8080/health \
  --wait-ready
```

## Blue-Green Deployments

### Zero-Downtime Deployment

**blue-green-deploy.sh:**
```bash
#!/bin/bash

set -e

CURRENT_COLOR=$(pmdaemon list --namespace production | grep -q "web-app-blue" && echo "blue" || echo "green")
NEW_COLOR=$([ "$CURRENT_COLOR" = "blue" ] && echo "green" || echo "blue")

echo "🔄 Current deployment: $CURRENT_COLOR"
echo "🚀 Deploying to: $NEW_COLOR"

# Deploy to new color
pmdaemon start "node server.js" \
  --name "web-app-$NEW_COLOR" \
  --namespace production \
  --instances 4 \
  --port "auto:3000-3100" \
  --health-check-url "http://localhost:\${PORT}/health" \
  --wait-ready

# Wait for health checks to stabilize
sleep 30

# Verify new deployment
if pmdaemon info "web-app-$NEW_COLOR" --namespace production | grep -q "healthy"; then
    echo "✅ New deployment is healthy"
    
    # Update load balancer (example with nginx)
    sed -i "s/web-app-$CURRENT_COLOR/web-app-$NEW_COLOR/g" /etc/nginx/sites-available/myapp
    nginx -s reload
    
    # Wait for traffic to drain
    sleep 60
    
    # Stop old deployment
    pmdaemon delete "web-app-$CURRENT_COLOR" --namespace production
    
    echo "🎉 Blue-green deployment complete!"
else
    echo "❌ New deployment failed health checks"
    pmdaemon delete "web-app-$NEW_COLOR" --namespace production
    exit 1
fi
```

## Monitoring and Alerting

### Comprehensive Monitoring Setup

**monitoring.json:**
```json
{
  "apps": [
    {
      "name": "prometheus-exporter",
      "script": "node",
      "args": ["monitoring/prometheus-exporter.js"],
      "port": "9090",
      "env": {
        "PMDAEMON_API_URL": "http://localhost:9615"
      }
    },
    {
      "name": "log-aggregator",
      "script": "node",
      "args": ["monitoring/log-aggregator.js"],
      "env": {
        "ELASTICSEARCH_URL": "http://elasticsearch:9200"
      }
    },
    {
      "name": "alerting-service",
      "script": "python",
      "args": ["monitoring/alerts.py"],
      "env": {
        "SLACK_WEBHOOK_URL": "${SLACK_WEBHOOK_URL}",
        "PAGERDUTY_API_KEY": "${PAGERDUTY_API_KEY}"
      },
      "health_check": {
        "check_type": "script",
        "script": "./monitoring/health-check.sh"
      }
    }
  ]
}
```

**Prometheus configuration (prometheus.yml):**
```yaml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'pmdaemon'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: /metrics
    scrape_interval: 5s
```

## Best Practices Summary

### 1. Use Configuration Files

```bash
# Good: Use configuration files for complex deployments
pmdaemon --config ecosystem.production.json start

# Avoid: Long CLI commands in production
```

### 2. Implement Health Checks

```bash
# Always include health checks for web services
{
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 10,
    "interval": 30
  }
}
```

### 3. Use Namespaces for Organization

```bash
# Organize by environment
pmdaemon start app.js --name api --namespace production
pmdaemon start app.js --name api --namespace staging
```

### 4. Automate Deployments

```bash
# Use deployment scripts with validation
pmdaemon validate ecosystem.json --strict
pmdaemon --config ecosystem.json start --wait-ready
```

### 5. Monitor Resource Usage

```bash
# Set appropriate resource limits
{
  "max_memory_restart": "512M",
  "instances": 4
}
```

## Next Steps

- **[Integration Examples](./integration-examples.md)** - Framework-specific examples
- **[Performance Tuning](../advanced/performance-tuning.md)** - Optimization strategies
- **[Security](../advanced/security.md)** - Security best practices
- **[Troubleshooting](../advanced/troubleshooting.md)** - Common issues and solutions
