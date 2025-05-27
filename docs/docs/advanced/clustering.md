# Clustering

PMDaemon's advanced clustering capabilities enable you to scale applications horizontally across multiple instances with intelligent load distribution, automatic failover, and sophisticated process management strategies.

## Overview

PMDaemon clustering provides:

- **🔄 Automatic load balancing** - Distribute traffic across instances
- **📈 Horizontal scaling** - Add/remove instances dynamically
- **🛡️ Fault tolerance** - Automatic failover and recovery
- **🎯 Smart port allocation** - Automatic port distribution
- **📊 Instance monitoring** - Per-instance health and metrics

## Basic Clustering

### Simple Cluster Setup

```bash
# Start a basic cluster with 4 instances
pmdaemon start "node server.js" --name web-cluster \
  --instances 4 \
  --port 3000-3003
```

**What happens:**
- 4 Node.js processes start
- Ports 3000, 3001, 3002, 3003 are assigned
- Each instance gets `PM2_INSTANCE_ID` environment variable
- Load balancing can be configured externally

### Configuration File Clustering

```json
{
  "name": "web-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "exec_mode": "cluster",
  "port": "3000-3003",
  "env": {
    "NODE_ENV": "production"
  },
  "instance_var": "INSTANCE_ID",
  "merge_logs": true
}
```

## Advanced Clustering Patterns

### CPU-Based Auto Scaling

```json
{
  "name": "auto-scaled-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": "max",
  "port": "auto:3000-3100",
  "env": {
    "NODE_ENV": "production",
    "CLUSTER_MODE": "auto"
  }
}
```

**"max" instances:**
- Uses all available CPU cores
- Automatically adjusts to system capacity
- Optimal for CPU-bound applications

### Heterogeneous Clustering

Different instance types with specialized roles:

```json
{
  "apps": [
    {
      "name": "master-instance",
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
      "name": "worker-instances",
      "script": "node",
      "args": ["server.js", "--role=worker"],
      "instances": 6,
      "port": "3001-3006",
      "env": {
        "ROLE": "worker",
        "ENABLE_CRON": "false",
        "ENABLE_ADMIN": "false"
      }
    }
  ]
}
```

### Multi-Tier Clustering

```json
{
  "apps": [
    {
      "name": "frontend-tier",
      "script": "node",
      "args": ["frontend.js"],
      "instances": 3,
      "port": "8080-8082",
      "env": {
        "TIER": "frontend",
        "BACKEND_URLS": "http://localhost:3000,http://localhost:3001,http://localhost:3002"
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
    },
    {
      "name": "worker-tier",
      "script": "python",
      "args": ["worker.py"],
      "instances": 2,
      "env": {
        "TIER": "worker",
        "QUEUE_NAME": "default"
      }
    }
  ]
}
```

## Instance Management

### Dynamic Scaling

```bash
# Scale up to 8 instances
pmdaemon scale web-cluster 8

# Scale down to 2 instances
pmdaemon scale web-cluster 2

# Auto-scale based on CPU usage
pmdaemon autoscale web-cluster --min 2 --max 10 --cpu-threshold 70
```

### Instance-Specific Operations

```bash
# Restart specific instance
pmdaemon restart web-cluster --instance 2

# Stop specific instance
pmdaemon stop web-cluster --instance 1

# Get instance-specific logs
pmdaemon logs web-cluster --instance 0 --lines 50
```

### Rolling Updates

```bash
# Rolling restart (one instance at a time)
pmdaemon restart web-cluster --rolling

# Rolling update with new configuration
pmdaemon reload web-cluster --rolling --instances 6
```

## Load Balancing Integration

### Nginx Load Balancer

```nginx
upstream web_cluster {
    least_conn;
    server localhost:3000 weight=1 max_fails=3 fail_timeout=30s;
    server localhost:3001 weight=1 max_fails=3 fail_timeout=30s;
    server localhost:3002 weight=1 max_fails=3 fail_timeout=30s;
    server localhost:3003 weight=1 max_fails=3 fail_timeout=30s;
    
    # Health check
    keepalive 32;
}

server {
    listen 80;
    server_name myapp.com;
    
    location / {
        proxy_pass http://web_cluster;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Connection settings
        proxy_connect_timeout 5s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
        
        # Health check
        proxy_next_upstream error timeout invalid_header http_500 http_502 http_503;
    }
    
    # Health check endpoint
    location /health {
        access_log off;
        return 200 "healthy\n";
        add_header Content-Type text/plain;
    }
}
```

### HAProxy Configuration

```haproxy
global
    daemon
    maxconn 4096

defaults
    mode http
    timeout connect 5000ms
    timeout client 50000ms
    timeout server 50000ms
    option httpchk GET /health

frontend web_frontend
    bind *:80
    default_backend web_cluster

backend web_cluster
    balance roundrobin
    option httpchk GET /health HTTP/1.1\r\nHost:\ localhost
    
    server web1 localhost:3000 check inter 5s fall 3 rise 2
    server web2 localhost:3001 check inter 5s fall 3 rise 2
    server web3 localhost:3002 check inter 5s fall 3 rise 2
    server web4 localhost:3003 check inter 5s fall 3 rise 2

# Statistics interface
listen stats
    bind *:8404
    stats enable
    stats uri /stats
    stats refresh 30s
```

## Application-Level Clustering

### Node.js Cluster Integration

```javascript
// server.js - Cluster-aware application
const cluster = require('cluster');
const express = require('express');
const os = require('os');

const app = express();
const port = process.env.PORT || 3000;
const instanceId = process.env.PM2_INSTANCE_ID || 0;

// Instance-specific configuration
app.locals.instanceId = instanceId;
app.locals.workerId = cluster.worker?.id || 'master';

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    instance: instanceId,
    worker: app.locals.workerId,
    uptime: process.uptime(),
    memory: process.memoryUsage(),
    timestamp: new Date().toISOString()
  });
});

// Instance-specific behavior
if (instanceId == 0) {
  // Master instance handles cron jobs
  const cron = require('node-cron');
  cron.schedule('0 * * * *', () => {
    console.log('Running hourly task on master instance');
    // Perform scheduled tasks
  });
}

// Graceful shutdown
process.on('SIGTERM', () => {
  console.log(`Instance ${instanceId} received SIGTERM, shutting down gracefully`);
  server.close(() => {
    console.log(`Instance ${instanceId} shut down complete`);
    process.exit(0);
  });
});

const server = app.listen(port, () => {
  console.log(`Instance ${instanceId} listening on port ${port}`);
});
```

### Session Affinity

```javascript
// Session store for cluster
const session = require('express-session');
const RedisStore = require('connect-redis')(session);
const redis = require('redis');

const redisClient = redis.createClient({
  host: 'localhost',
  port: 6379
});

app.use(session({
  store: new RedisStore({ client: redisClient }),
  secret: process.env.SESSION_SECRET,
  resave: false,
  saveUninitialized: false,
  cookie: {
    secure: process.env.NODE_ENV === 'production',
    maxAge: 24 * 60 * 60 * 1000 // 24 hours
  }
}));
```

## Monitoring Clusters

### Cluster Metrics

```bash
# Monitor all instances
pmdaemon monit web-cluster

# Get cluster statistics
pmdaemon stats web-cluster

# View cluster topology
pmdaemon topology web-cluster
```

### Per-Instance Monitoring

```bash
# Monitor specific instance
pmdaemon monit web-cluster --instance 2

# Get instance metrics
pmdaemon info web-cluster --instance 1

# Compare instance performance
pmdaemon compare web-cluster
```

### Cluster Health Dashboard

```javascript
// cluster-dashboard.js
const PMDaemonClient = require('./pmdaemon-client');

class ClusterDashboard {
  constructor() {
    this.client = new PMDaemonClient();
  }

  async getClusterStatus(clusterName) {
    const processes = await this.client.listProcesses();
    const clusterProcesses = processes.processes.filter(p => 
      p.name.startsWith(clusterName)
    );

    const status = {
      name: clusterName,
      totalInstances: clusterProcesses.length,
      healthyInstances: clusterProcesses.filter(p => p.health === 'healthy').length,
      onlineInstances: clusterProcesses.filter(p => p.status === 'online').length,
      totalCpu: clusterProcesses.reduce((sum, p) => sum + p.cpu, 0),
      totalMemory: clusterProcesses.reduce((sum, p) => sum + p.memory, 0),
      instances: clusterProcesses.map(p => ({
        id: p.id,
        port: p.port,
        status: p.status,
        health: p.health,
        cpu: p.cpu,
        memory: p.memory,
        uptime: p.uptime,
        restarts: p.restarts
      }))
    };

    return status;
  }

  async autoScale(clusterName, options = {}) {
    const {
      minInstances = 2,
      maxInstances = 10,
      cpuThreshold = 70,
      memoryThreshold = 80
    } = options;

    const status = await this.getClusterStatus(clusterName);
    const avgCpu = status.totalCpu / status.totalInstances;
    const avgMemory = status.totalMemory / status.totalInstances / 1024 / 1024; // MB

    let targetInstances = status.totalInstances;

    // Scale up conditions
    if (avgCpu > cpuThreshold && status.totalInstances < maxInstances) {
      targetInstances = Math.min(maxInstances, status.totalInstances + 1);
      console.log(`🔼 Scaling up ${clusterName}: CPU ${avgCpu}% > ${cpuThreshold}%`);
    }
    
    // Scale down conditions
    if (avgCpu < cpuThreshold * 0.5 && status.totalInstances > minInstances) {
      targetInstances = Math.max(minInstances, status.totalInstances - 1);
      console.log(`🔽 Scaling down ${clusterName}: CPU ${avgCpu}% < ${cpuThreshold * 0.5}%`);
    }

    if (targetInstances !== status.totalInstances) {
      await this.client.scaleCluster(clusterName, targetInstances);
      console.log(`📊 Scaled ${clusterName} to ${targetInstances} instances`);
    }

    return targetInstances;
  }
}
```

## Fault Tolerance

### Automatic Failover

```json
{
  "name": "fault-tolerant-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "port": "3000-3003",
  "autorestart": true,
  "max_restarts": 5,
  "min_uptime": "10s",
  "restart_delay": "2s",
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:${PORT}/health",
    "timeout": 5,
    "interval": 15,
    "retries": 3
  }
}
```

### Circuit Breaker Pattern

```javascript
// circuit-breaker.js
const CircuitBreaker = require('opossum');

const options = {
  timeout: 3000,
  errorThresholdPercentage: 50,
  resetTimeout: 30000,
  rollingCountTimeout: 10000,
  rollingCountBuckets: 10
};

// Create circuit breaker for database calls
const dbBreaker = new CircuitBreaker(callDatabase, options);

dbBreaker.on('open', () => {
  console.log('Circuit breaker opened - database calls failing');
});

dbBreaker.on('halfOpen', () => {
  console.log('Circuit breaker half-open - testing database');
});

dbBreaker.on('close', () => {
  console.log('Circuit breaker closed - database calls healthy');
});

async function callDatabase(query) {
  // Database call implementation
  return await db.query(query);
}

// Use circuit breaker in application
app.get('/api/data', async (req, res) => {
  try {
    const data = await dbBreaker.fire(req.query.sql);
    res.json(data);
  } catch (error) {
    if (dbBreaker.opened) {
      res.status(503).json({ error: 'Service temporarily unavailable' });
    } else {
      res.status(500).json({ error: 'Database error' });
    }
  }
});
```

## Performance Optimization

### Cluster-Specific Tuning

```json
{
  "name": "optimized-cluster",
  "script": "node",
  "args": [
    "--max-old-space-size=512",
    "--optimize-for-size",
    "server.js"
  ],
  "instances": 8,
  "port": "auto:3000-3100",
  "env": {
    "NODE_ENV": "production",
    "UV_THREADPOOL_SIZE": "4",
    "CLUSTER_WORKER_SIZE": "1"
  },
  "max_memory_restart": "600M",
  "instance_var": "WORKER_ID"
}
```

### Resource Allocation

```bash
# CPU affinity for instances
pmdaemon start "node server.js" --name cpu-cluster \
  --instances 4 \
  --cpu-affinity "0,1,2,3"

# Memory limits per instance
pmdaemon start "node server.js" --name memory-cluster \
  --instances 4 \
  --max-memory 512M \
  --port 3000-3003
```

## Best Practices

### 1. Right-Size Your Clusters

```bash
# Start with conservative instance count
pmdaemon start "node server.js" --name web-cluster --instances 2

# Monitor and scale based on metrics
pmdaemon monit web-cluster
pmdaemon scale web-cluster 4  # Scale up if needed
```

### 2. Use Health Checks

```json
{
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:${PORT}/health",
    "timeout": 5,
    "interval": 30,
    "retries": 2
  }
}
```

### 3. Implement Graceful Shutdown

```javascript
process.on('SIGTERM', async () => {
  console.log('Received SIGTERM, shutting down gracefully');
  
  // Stop accepting new requests
  server.close();
  
  // Finish existing requests
  await finishPendingRequests();
  
  // Close database connections
  await db.close();
  
  process.exit(0);
});
```

### 4. Monitor Cluster Health

```bash
# Set up monitoring
pmdaemon start "node monitor.js" --name cluster-monitor \
  --env CLUSTER_NAME=web-cluster \
  --env CHECK_INTERVAL=30s
```

### 5. Use Load Balancer Health Checks

```nginx
# Configure health checks in load balancer
location /health {
    proxy_pass http://web_cluster;
    proxy_connect_timeout 1s;
    proxy_send_timeout 1s;
    proxy_read_timeout 1s;
}
```

## Next Steps

- **[Performance Tuning](./performance-tuning.md)** - Optimize cluster performance
- **[Security](./security.md)** - Secure cluster deployments
- **[Monitoring](../features/monitoring.md)** - Advanced cluster monitoring
- **[Deployment Examples](../examples/deployment-examples.md)** - Production cluster patterns
