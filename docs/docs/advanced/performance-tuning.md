# Performance Tuning

This guide covers optimization strategies for PMDaemon to achieve maximum performance in production environments. Learn how to tune system resources, optimize process configurations, and scale effectively.

## System-Level Optimizations

### Operating System Tuning

#### File Descriptor Limits

PMDaemon manages multiple processes and connections, requiring adequate file descriptor limits:

```bash
# Check current limits
ulimit -n

# Increase for current session
ulimit -n 65536

# Permanent increase (add to /etc/security/limits.conf)
echo "* soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "* hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# For systemd services
sudo mkdir -p /etc/systemd/system/pmdaemon.service.d
echo "[Service]" | sudo tee /etc/systemd/system/pmdaemon.service.d/limits.conf
echo "LimitNOFILE=65536" | sudo tee -a /etc/systemd/system/pmdaemon.service.d/limits.conf
```

#### Memory Management

```bash
# Optimize memory overcommit
echo 1 | sudo tee /proc/sys/vm/overcommit_memory

# Adjust swappiness for better performance
echo 10 | sudo tee /proc/sys/vm/swappiness

# Increase shared memory limits
echo "kernel.shmmax = 68719476736" | sudo tee -a /etc/sysctl.conf
echo "kernel.shmall = 4294967296" | sudo tee -a /etc/sysctl.conf
```

#### Network Optimization

```bash
# Increase network buffer sizes
echo "net.core.rmem_max = 16777216" | sudo tee -a /etc/sysctl.conf
echo "net.core.wmem_max = 16777216" | sudo tee -a /etc/sysctl.conf
echo "net.ipv4.tcp_rmem = 4096 87380 16777216" | sudo tee -a /etc/sysctl.conf
echo "net.ipv4.tcp_wmem = 4096 65536 16777216" | sudo tee -a /etc/sysctl.conf

# Apply changes
sudo sysctl -p
```

### PMDaemon Configuration

#### Monitoring Intervals

Optimize monitoring frequency based on your needs:

```bash
# High-frequency monitoring (development)
pmdaemon monit --interval 1s

# Balanced monitoring (production)
pmdaemon monit --interval 5s

# Low-overhead monitoring (resource-constrained)
pmdaemon monit --interval 30s
```

#### Log Management

```json
{
  "name": "optimized-service",
  "script": "node",
  "args": ["server.js"],
  "log": {
    "out_file": "/dev/null",
    "error_file": "/var/log/app/error.log",
    "max_log_size": "50M",
    "max_log_files": 3,
    "compress_logs": true
  }
}
```

## Process-Level Optimizations

### Instance Scaling Strategies

#### CPU-Based Scaling

```bash
# Get CPU core count
CPU_CORES=$(nproc)

# Scale based on workload type
# CPU-intensive: 1 instance per core
pmdaemon start "node cpu-intensive.js" --name cpu-app --instances $CPU_CORES

# I/O-intensive: 2x cores
pmdaemon start "node io-intensive.js" --name io-app --instances $((CPU_CORES * 2))

# Mixed workload: 1.5x cores
pmdaemon start "node mixed-app.js" --name mixed-app --instances $((CPU_CORES * 3 / 2))
```

#### Memory-Aware Scaling

```bash
# Calculate optimal instances based on memory
TOTAL_MEMORY=$(free -m | awk 'NR==2{print $2}')
MEMORY_PER_INSTANCE=512  # MB
OPTIMAL_INSTANCES=$((TOTAL_MEMORY / MEMORY_PER_INSTANCE / 2))  # Leave 50% for system

pmdaemon start "node server.js" --name web-app \
  --instances $OPTIMAL_INSTANCES \
  --max-memory ${MEMORY_PER_INSTANCE}M
```

### Port Management Optimization

#### Efficient Port Allocation

```json
{
  "apps": [
    {
      "name": "web-cluster",
      "script": "node",
      "args": ["server.js"],
      "instances": 8,
      "port": "auto:8000-8100",
      "env": {
        "UV_THREADPOOL_SIZE": "4"
      }
    }
  ]
}
```

#### Load Balancer Integration

```nginx
# nginx.conf - Optimized upstream configuration
upstream app_cluster {
    least_conn;
    server localhost:8000 weight=1 max_fails=3 fail_timeout=30s;
    server localhost:8001 weight=1 max_fails=3 fail_timeout=30s;
    server localhost:8002 weight=1 max_fails=3 fail_timeout=30s;
    server localhost:8003 weight=1 max_fails=3 fail_timeout=30s;
    keepalive 32;
}

server {
    listen 80;
    
    location / {
        proxy_pass http://app_cluster;
        proxy_http_version 1.1;
        proxy_set_header Connection "";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_connect_timeout 5s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }
}
```

## Application-Specific Optimizations

### Node.js Applications

#### Memory and CPU Tuning

```json
{
  "name": "nodejs-optimized",
  "script": "node",
  "args": [
    "--max-old-space-size=1024",
    "--max-new-space-size=256",
    "--optimize-for-size",
    "server.js"
  ],
  "instances": 4,
  "env": {
    "NODE_ENV": "production",
    "UV_THREADPOOL_SIZE": "16",
    "NODE_OPTIONS": "--max-old-space-size=1024"
  },
  "max_memory_restart": "1200M"
}
```

#### Cluster Mode Optimization

```javascript
// server.js - Optimized cluster setup
const cluster = require('cluster');
const numCPUs = require('os').cpus().length;

if (cluster.isMaster) {
  // Master process
  console.log(`Master ${process.pid} is running`);
  
  // Fork workers
  for (let i = 0; i < numCPUs; i++) {
    cluster.fork();
  }
  
  cluster.on('exit', (worker, code, signal) => {
    console.log(`Worker ${worker.process.pid} died`);
    cluster.fork(); // Restart worker
  });
} else {
  // Worker process
  const express = require('express');
  const app = express();
  
  // Optimize Express
  app.set('trust proxy', true);
  app.disable('x-powered-by');
  
  // Connection pooling
  const pool = require('generic-pool');
  
  app.listen(process.env.PORT || 3000, () => {
    console.log(`Worker ${process.pid} started`);
  });
}
```

### Python Applications

#### WSGI Server Optimization

```json
{
  "name": "python-optimized",
  "script": "gunicorn",
  "args": [
    "--workers", "4",
    "--worker-class", "gevent",
    "--worker-connections", "1000",
    "--max-requests", "1000",
    "--max-requests-jitter", "100",
    "--preload",
    "--bind", "0.0.0.0:8000",
    "app:application"
  ],
  "env": {
    "PYTHONOPTIMIZE": "1",
    "PYTHONUNBUFFERED": "1"
  },
  "max_memory_restart": "512M"
}
```

#### Async Python Applications

```json
{
  "name": "fastapi-optimized",
  "script": "uvicorn",
  "args": [
    "main:app",
    "--host", "0.0.0.0",
    "--port", "8000",
    "--workers", "4",
    "--loop", "uvloop",
    "--http", "httptools"
  ],
  "env": {
    "PYTHONPATH": "/app",
    "PYTHONOPTIMIZE": "2"
  }
}
```

## Health Check Optimization

### Efficient Health Checks

```json
{
  "name": "optimized-health-checks",
  "script": "node",
  "args": ["server.js"],
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 5,
    "interval": 30,
    "retries": 2
  }
}
```

### Lightweight Health Endpoints

```javascript
// Optimized health check endpoint
app.get('/health', (req, res) => {
  // Quick checks only
  const health = {
    status: 'healthy',
    timestamp: Date.now(),
    uptime: process.uptime(),
    memory: process.memoryUsage().rss
  };
  
  res.json(health);
});

// Detailed health check (separate endpoint)
app.get('/health/detailed', async (req, res) => {
  try {
    // More comprehensive checks
    await checkDatabase();
    await checkRedis();
    await checkExternalServices();
    
    res.json({ status: 'healthy', checks: 'all_passed' });
  } catch (error) {
    res.status(503).json({ status: 'unhealthy', error: error.message });
  }
});
```

## Monitoring and Metrics Optimization

### Efficient Metrics Collection

```bash
# Reduce monitoring overhead
export PMDAEMON_METRICS_INTERVAL=10s
export PMDAEMON_METRICS_BUFFER_SIZE=1000

# Start with optimized monitoring
pmdaemon start "node server.js" --name web-app
```

### Custom Metrics Integration

```javascript
// Prometheus metrics integration
const prometheus = require('prom-client');

// Create custom metrics
const httpRequestDuration = new prometheus.Histogram({
  name: 'http_request_duration_seconds',
  help: 'Duration of HTTP requests in seconds',
  labelNames: ['method', 'route', 'status_code']
});

// Middleware for metrics collection
app.use((req, res, next) => {
  const start = Date.now();
  
  res.on('finish', () => {
    const duration = (Date.now() - start) / 1000;
    httpRequestDuration
      .labels(req.method, req.route?.path || req.path, res.statusCode)
      .observe(duration);
  });
  
  next();
});

// Metrics endpoint
app.get('/metrics', (req, res) => {
  res.set('Content-Type', prometheus.register.contentType);
  res.end(prometheus.register.metrics());
});
```

## Database and External Service Optimization

### Connection Pooling

```javascript
// Optimized database connection pooling
const { Pool } = require('pg');

const pool = new Pool({
  host: process.env.DB_HOST,
  database: process.env.DB_NAME,
  user: process.env.DB_USER,
  password: process.env.DB_PASSWORD,
  port: process.env.DB_PORT,
  max: 20, // Maximum connections
  min: 5,  // Minimum connections
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
  maxUses: 7500, // Close connection after 7500 uses
});

// Connection health check
setInterval(async () => {
  try {
    await pool.query('SELECT 1');
  } catch (error) {
    console.error('Database health check failed:', error);
  }
}, 30000);
```

### Redis Optimization

```javascript
// Optimized Redis configuration
const Redis = require('ioredis');

const redis = new Redis({
  host: process.env.REDIS_HOST,
  port: process.env.REDIS_PORT,
  maxRetriesPerRequest: 3,
  retryDelayOnFailover: 100,
  enableReadyCheck: false,
  maxLoadingTimeout: 1000,
  lazyConnect: true,
  keepAlive: 30000,
  family: 4,
  connectTimeout: 10000,
  commandTimeout: 5000,
});

// Connection pooling for Redis
const cluster = new Redis.Cluster([
  { host: 'redis-1', port: 6379 },
  { host: 'redis-2', port: 6379 },
  { host: 'redis-3', port: 6379 }
], {
  enableOfflineQueue: false,
  redisOptions: {
    password: process.env.REDIS_PASSWORD
  }
});
```

## Performance Monitoring

### Benchmarking Setup

```bash
#!/bin/bash
# benchmark.sh - Performance testing script

echo "🚀 Starting performance benchmark..."

# Start optimized configuration
pmdaemon --config ecosystem.optimized.json start

# Wait for services to be ready
sleep 30

# Run load tests
echo "📊 Running load tests..."

# HTTP load test
ab -n 10000 -c 100 http://localhost:3000/ > results/ab-results.txt

# WebSocket load test
node websocket-load-test.js > results/ws-results.txt

# Database load test
pgbench -c 10 -j 2 -t 1000 myapp_db > results/db-results.txt

echo "✅ Benchmark complete. Results in results/ directory."
```

### Performance Metrics Dashboard

```javascript
// performance-dashboard.js
const PMDaemonClient = require('./pmdaemon-client');

class PerformanceDashboard {
  constructor() {
    this.client = new PMDaemonClient();
    this.metrics = {
      requests_per_second: 0,
      average_response_time: 0,
      error_rate: 0,
      cpu_usage: 0,
      memory_usage: 0
    };
  }

  async collectMetrics() {
    const processes = await this.client.listProcesses();
    const systemMetrics = await this.client.getSystemMetrics();
    
    // Calculate aggregate metrics
    this.metrics.cpu_usage = systemMetrics.cpu.usage;
    this.metrics.memory_usage = systemMetrics.memory.usage_percent;
    
    // Process-specific metrics
    processes.processes.forEach(proc => {
      console.log(`${proc.name}: CPU ${proc.cpu}%, Memory ${proc.memory}MB`);
    });
    
    return this.metrics;
  }

  async optimizeBasedOnMetrics() {
    const metrics = await this.collectMetrics();
    
    if (metrics.cpu_usage > 80) {
      console.log('🔥 High CPU usage detected, scaling up...');
      // Scale up instances
    }
    
    if (metrics.memory_usage > 85) {
      console.log('💾 High memory usage detected, optimizing...');
      // Restart memory-heavy processes
    }
    
    if (metrics.error_rate > 5) {
      console.log('❌ High error rate detected, investigating...');
      // Check logs and restart failing processes
    }
  }
}
```

## Best Practices Summary

### 1. Right-Size Your Instances

```bash
# CPU-bound applications
pmdaemon start "cpu-app" --instances $(nproc)

# I/O-bound applications  
pmdaemon start "io-app" --instances $(($(nproc) * 2))

# Memory-bound applications
pmdaemon start "memory-app" --instances 2 --max-memory 2G
```

### 2. Optimize Health Checks

```bash
# Frequent but lightweight health checks
pmdaemon start "node server.js" --name web-api \
  --health-check-url http://localhost:3000/ping \
  --health-check-timeout 2s \
  --health-check-interval 15s
```

### 3. Use Efficient Logging

```bash
# Structured logging with rotation
pmdaemon start "node server.js" --name web-api \
  --out-file /var/log/app/web-api.json \
  --error-file /var/log/app/web-api-error.log
```

### 4. Monitor Resource Usage

```bash
# Set appropriate limits
pmdaemon start "node server.js" --name web-api \
  --max-memory 512M \
  --max-restarts 5 \
  --min-uptime 10s
```

### 5. Optimize for Your Workload

```bash
# Web servers: Focus on concurrency
pmdaemon start "node web.js" --instances 8 --port auto:3000-3100

# Workers: Focus on throughput
pmdaemon start "python worker.py" --instances 4 --max-memory 1G

# APIs: Balance both
pmdaemon start "node api.js" --instances 6 --port 8000-8005 --max-memory 512M
```

## Next Steps

- **[Security](./security.md)** - Security optimization and hardening
- **[Clustering](./clustering.md)** - Advanced clustering strategies
- **[Logging](./logging.md)** - Optimized logging configuration
- **[Troubleshooting](./troubleshooting.md)** - Performance issue diagnosis
