# Integration Examples

This guide provides practical examples of integrating PMDaemon with popular frameworks, platforms, and tools. Learn how to seamlessly incorporate PMDaemon into your existing development and deployment workflows.

## Framework Integration

### Express.js Integration

**Scenario:** Integrate PMDaemon with an Express.js application for development and production.

**package.json scripts:**
```json
{
  "scripts": {
    "dev": "pmdaemon start 'npm run dev:server' --name dev-server --watch",
    "dev:server": "nodemon --exec 'node --inspect' server.js",
    "start": "pmdaemon start 'node server.js' --name web-app --instances 4 --port 3000-3003",
    "stop": "pmdaemon stop web-app",
    "restart": "pmdaemon restart web-app",
    "logs": "pmdaemon logs web-app --follow",
    "monit": "pmdaemon monit"
  }
}
```

**Development setup:**
```javascript
// server.js
const express = require('express');
const app = express();

// PMDaemon integration middleware
app.use((req, res, next) => {
  // Add instance information to responses
  res.setHeader('X-Instance-ID', process.env.PM2_INSTANCE_ID || '0');
  res.setHeader('X-Process-ID', process.pid);
  next();
});

// Health check endpoint for PMDaemon
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    instance: process.env.PM2_INSTANCE_ID || '0',
    pid: process.pid,
    uptime: process.uptime(),
    memory: process.memoryUsage()
  });
});

const port = process.env.PORT || 3000;
app.listen(port, () => {
  console.log(`Server running on port ${port}, instance ${process.env.PM2_INSTANCE_ID || '0'}`);
});
```

### Next.js Integration

**ecosystem.json:**
```json
{
  "apps": [
    {
      "name": "nextjs-app",
      "script": "npm",
      "args": ["start"],
      "instances": 2,
      "port": "3000-3001",
      "env": {
        "NODE_ENV": "production",
        "PORT": "3000"
      },
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/api/health"
      }
    }
  ]
}
```

**pages/api/health.js:**
```javascript
export default function handler(req, res) {
  res.status(200).json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    instance: process.env.PM2_INSTANCE_ID || '0'
  });
}
```

### FastAPI Integration

**main.py:**
```python
from fastapi import FastAPI
import os
import psutil
import time

app = FastAPI()

start_time = time.time()

@app.get("/health")
async def health_check():
    return {
        "status": "healthy",
        "instance": os.getenv("PM2_INSTANCE_ID", "0"),
        "pid": os.getpid(),
        "uptime": time.time() - start_time,
        "memory": psutil.Process().memory_info().rss
    }

@app.get("/")
async def root():
    return {
        "message": "Hello World",
        "instance": os.getenv("PM2_INSTANCE_ID", "0")
    }
```

**ecosystem.yaml:**
```yaml
apps:
  - name: fastapi-app
    script: uvicorn
    args: [main:app, --host, "0.0.0.0", --port, "8000"]
    instances: 3
    port: "8000-8002"
    env:
      PYTHONPATH: /app
    health_check:
      check_type: http
      url: http://localhost:8000/health
```

## CI/CD Integration

### GitHub Actions

**.github/workflows/deploy.yml:**
```yaml
name: Deploy with PMDaemon

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        cache: 'npm'
    
    - name: Install dependencies
      run: npm ci
    
    - name: Build application
      run: npm run build
    
    - name: Install PMDaemon
      run: |
        curl -sSL https://install.pmdaemon.io | bash
        echo "$HOME/.cargo/bin" >> $GITHUB_PATH
    
    - name: Deploy to staging
      run: |
        pmdaemon --config ecosystem.staging.json start --wait-ready
        
    - name: Run health checks
      run: |
        sleep 10
        curl -f http://localhost:3000/health
        
    - name: Deploy to production
      if: success()
      run: |
        pmdaemon --config ecosystem.production.json start --wait-ready
        
    - name: Cleanup on failure
      if: failure()
      run: |
        pmdaemon delete all --force
```

### GitLab CI

**.gitlab-ci.yml:**
```yaml
stages:
  - build
  - test
  - deploy

variables:
  NODE_VERSION: "18"

build:
  stage: build
  image: node:${NODE_VERSION}
  script:
    - npm ci
    - npm run build
  artifacts:
    paths:
      - dist/
    expire_in: 1 hour

test:
  stage: test
  image: node:${NODE_VERSION}
  script:
    - npm ci
    - npm test

deploy_staging:
  stage: deploy
  image: ubuntu:22.04
  before_script:
    - apt-get update && apt-get install -y curl
    - curl -sSL https://install.pmdaemon.io | bash
    - export PATH="$HOME/.cargo/bin:$PATH"
  script:
    - pmdaemon --config ecosystem.staging.json start --wait-ready
    - curl -f http://localhost:3000/health
  environment:
    name: staging
    url: https://staging.myapp.com
  only:
    - develop

deploy_production:
  stage: deploy
  image: ubuntu:22.04
  before_script:
    - apt-get update && apt-get install -y curl
    - curl -sSL https://install.pmdaemon.io | bash
    - export PATH="$HOME/.cargo/bin:$PATH"
  script:
    - pmdaemon --config ecosystem.production.json start --wait-ready
    - curl -f http://localhost:3000/health
  environment:
    name: production
    url: https://myapp.com
  only:
    - main
  when: manual
```

## Container Integration

### Docker Integration

**Dockerfile:**
```dockerfile
FROM node:18-alpine

# Install PMDaemon
RUN apk add --no-cache curl
RUN curl -sSL https://install.pmdaemon.io | sh
ENV PATH="/root/.cargo/bin:$PATH"

WORKDIR /app

# Copy package files
COPY package*.json ./
RUN npm ci --only=production

# Copy application code
COPY . .

# Create non-root user
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nodejs -u 1001
USER nodejs

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Start with PMDaemon
CMD ["pmdaemon", "start", "node", "server.js", "--name", "app", "--health-check-url", "http://localhost:3000/health"]
```

### Docker Compose

**docker-compose.yml:**
```yaml
version: '3.8'

services:
  app:
    build: .
    ports:
      - "3000-3003:3000-3003"
      - "9615:9615"
    environment:
      - NODE_ENV=production
      - DATABASE_URL=postgres://postgres:password@db:5432/myapp
      - REDIS_URL=redis://redis:6379
    volumes:
      - ./ecosystem.docker.json:/app/ecosystem.json
    command: pmdaemon --config ecosystem.json start
    depends_on:
      - db
      - redis
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  db:
    image: postgres:13
    environment:
      POSTGRES_DB: myapp
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:6-alpine
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

## Monitoring Integration

### Prometheus Integration

**prometheus-exporter.js:**
```javascript
const express = require('express');
const client = require('prom-client');
const PMDaemonClient = require('./pmdaemon-client');

const app = express();
const pmdaemon = new PMDaemonClient();

// Create metrics
const processCount = new client.Gauge({
  name: 'pmdaemon_processes_total',
  help: 'Total number of processes managed by PMDaemon',
  labelNames: ['status']
});

const processCpu = new client.Gauge({
  name: 'pmdaemon_process_cpu_percent',
  help: 'CPU usage percentage for each process',
  labelNames: ['process_name', 'instance']
});

const processMemory = new client.Gauge({
  name: 'pmdaemon_process_memory_bytes',
  help: 'Memory usage in bytes for each process',
  labelNames: ['process_name', 'instance']
});

// Update metrics every 10 seconds
setInterval(async () => {
  try {
    const processes = await pmdaemon.listProcesses();
    
    // Reset gauges
    processCount.reset();
    processCpu.reset();
    processMemory.reset();
    
    // Count processes by status
    const statusCounts = {};
    processes.processes.forEach(proc => {
      statusCounts[proc.status] = (statusCounts[proc.status] || 0) + 1;
      
      // Set individual process metrics
      processCpu.set(
        { process_name: proc.name, instance: proc.instance || '0' },
        proc.cpu || 0
      );
      
      processMemory.set(
        { process_name: proc.name, instance: proc.instance || '0' },
        proc.memory || 0
      );
    });
    
    // Set status counts
    Object.entries(statusCounts).forEach(([status, count]) => {
      processCount.set({ status }, count);
    });
    
  } catch (error) {
    console.error('Failed to update metrics:', error);
  }
}, 10000);

// Metrics endpoint
app.get('/metrics', (req, res) => {
  res.set('Content-Type', client.register.contentType);
  res.end(client.register.metrics());
});

app.listen(9090, () => {
  console.log('Prometheus exporter listening on port 9090');
});
```

### Grafana Dashboard

**grafana-dashboard.json:**
```json
{
  "dashboard": {
    "title": "PMDaemon Monitoring",
    "panels": [
      {
        "title": "Process Count by Status",
        "type": "stat",
        "targets": [
          {
            "expr": "pmdaemon_processes_total",
            "legendFormat": "{{status}}"
          }
        ]
      },
      {
        "title": "CPU Usage by Process",
        "type": "graph",
        "targets": [
          {
            "expr": "pmdaemon_process_cpu_percent",
            "legendFormat": "{{process_name}}-{{instance}}"
          }
        ]
      },
      {
        "title": "Memory Usage by Process",
        "type": "graph",
        "targets": [
          {
            "expr": "pmdaemon_process_memory_bytes / 1024 / 1024",
            "legendFormat": "{{process_name}}-{{instance}}"
          }
        ]
      }
    ]
  }
}
```

## Load Balancer Integration

### Nginx with Dynamic Upstream

**nginx.conf:**
```nginx
upstream backend {
    least_conn;
    
    # PMDaemon managed processes
    server localhost:3000 max_fails=3 fail_timeout=30s;
    server localhost:3001 max_fails=3 fail_timeout=30s;
    server localhost:3002 max_fails=3 fail_timeout=30s;
    server localhost:3003 max_fails=3 fail_timeout=30s;
}

server {
    listen 80;
    server_name myapp.com;
    
    # Health check endpoint
    location /health {
        access_log off;
        proxy_pass http://backend;
        proxy_connect_timeout 1s;
        proxy_send_timeout 1s;
        proxy_read_timeout 1s;
    }
    
    # Main application
    location / {
        proxy_pass http://backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Health check for upstream
        proxy_next_upstream error timeout invalid_header http_500 http_502 http_503;
    }
}
```

### Consul Service Discovery

**consul-integration.js:**
```javascript
const consul = require('consul')();
const PMDaemonClient = require('./pmdaemon-client');

class ConsulIntegration {
  constructor() {
    this.pmdaemon = new PMDaemonClient();
    this.serviceName = 'web-app';
  }

  async registerServices() {
    const processes = await this.pmdaemon.listProcesses();
    
    for (const process of processes.processes) {
      if (process.name.startsWith(this.serviceName) && process.status === 'online') {
        await consul.agent.service.register({
          id: `${process.name}-${process.instance}`,
          name: this.serviceName,
          port: process.port,
          address: 'localhost',
          check: {
            http: `http://localhost:${process.port}/health`,
            interval: '30s',
            timeout: '10s'
          }
        });
        
        console.log(`Registered ${process.name} with Consul`);
      }
    }
  }

  async deregisterServices() {
    const services = await consul.agent.service.list();
    
    for (const [serviceId, service] of Object.entries(services)) {
      if (service.Service === this.serviceName) {
        await consul.agent.service.deregister(serviceId);
        console.log(`Deregistered ${serviceId} from Consul`);
      }
    }
  }

  async syncServices() {
    await this.deregisterServices();
    await this.registerServices();
  }
}

// Auto-sync every 60 seconds
const integration = new ConsulIntegration();
setInterval(() => {
  integration.syncServices().catch(console.error);
}, 60000);

// Initial sync
integration.syncServices().catch(console.error);
```

## Database Integration

### Connection Pool Management

**database.js:**
```javascript
const { Pool } = require('pg');

class DatabaseManager {
  constructor() {
    const instanceId = parseInt(process.env.PM2_INSTANCE_ID || '0');
    const totalInstances = parseInt(process.env.PM2_INSTANCES || '1');
    
    // Distribute connection pool across instances
    const totalConnections = 100;
    const connectionsPerInstance = Math.ceil(totalConnections / totalInstances);
    
    this.pool = new Pool({
      host: process.env.DB_HOST,
      database: process.env.DB_NAME,
      user: process.env.DB_USER,
      password: process.env.DB_PASSWORD,
      port: process.env.DB_PORT,
      max: connectionsPerInstance,
      min: Math.ceil(connectionsPerInstance / 4),
      idleTimeoutMillis: 30000,
      connectionTimeoutMillis: 2000,
    });
    
    console.log(`Instance ${instanceId}: Database pool configured with ${connectionsPerInstance} connections`);
  }

  async query(text, params) {
    const client = await this.pool.connect();
    try {
      const result = await client.query(text, params);
      return result;
    } finally {
      client.release();
    }
  }

  async healthCheck() {
    try {
      await this.query('SELECT 1');
      return true;
    } catch (error) {
      console.error('Database health check failed:', error);
      return false;
    }
  }

  async close() {
    await this.pool.end();
  }
}

module.exports = DatabaseManager;
```

## Best Practices

### 1. Environment-Specific Configuration

```bash
# Development
pmdaemon --config ecosystem.dev.json start

# Staging
pmdaemon --config ecosystem.staging.json start

# Production
pmdaemon --config ecosystem.production.json start
```

### 2. Health Check Integration

```javascript
// Always implement health checks
app.get('/health', async (req, res) => {
  const checks = {
    database: await checkDatabase(),
    redis: await checkRedis(),
    external_api: await checkExternalAPI()
  };
  
  const healthy = Object.values(checks).every(check => check);
  
  res.status(healthy ? 200 : 503).json({
    status: healthy ? 'healthy' : 'unhealthy',
    checks,
    instance: process.env.PM2_INSTANCE_ID
  });
});
```

### 3. Graceful Shutdown

```javascript
// Handle shutdown signals
process.on('SIGTERM', async () => {
  console.log('Received SIGTERM, shutting down gracefully');
  
  // Stop accepting new requests
  server.close();
  
  // Close database connections
  await database.close();
  
  // Exit
  process.exit(0);
});
```

### 4. Monitoring Integration

```bash
# Start monitoring alongside your application
pmdaemon start "node prometheus-exporter.js" --name metrics-exporter --port 9090
pmdaemon start "node server.js" --name web-app --instances 4 --port 3000-3003
```

## Next Steps

- **[Deployment Examples](./deployment-examples.md)** - Production deployment patterns
- **[API Examples](../api/api-examples.md)** - API integration examples
- **[Performance Tuning](../advanced/performance-tuning.md)** - Optimization strategies
- **[Monitoring](../features/monitoring.md)** - Advanced monitoring setup
