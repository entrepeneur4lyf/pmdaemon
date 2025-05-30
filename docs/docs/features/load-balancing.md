# Clustering & Process Distribution

PMDaemon provides **clustering capabilities** that allow you to run multiple instances of your application for improved performance and reliability. While PMDaemon handles process management and port distribution, external load balancing is handled by reverse proxies or load balancers.

## Overview

PMDaemon's clustering features include:

- **🚀 Multiple Process Instances** - Run N copies of your application
- **🔌 Automatic Port Distribution** - Each instance gets its own port
- **📊 Process Management** - Start, stop, restart all instances together
- **💾 Shared Configuration** - Single config manages all instances
- **🔄 Individual Instance Control** - Manage instances independently

> **Note:** PMDaemon focuses on **process management** rather than traffic load balancing. For HTTP/TCP load balancing, use a reverse proxy like Nginx, HAProxy, or a cloud load balancer.

## Clustering Configuration

### Basic Clustering

Run multiple instances of the same application:

```bash
# CLI - Start 4 instances
pmdaemon start "node server.js" --name web-app --instances 4

# Each instance will be named: web-app-0, web-app-1, web-app-2, web-app-3
```

```json
{
  "name": "web-app",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "exec_mode": "cluster"
}
```

### Port Distribution

PMDaemon automatically assigns ports to cluster instances:

```json
{
  "name": "api-cluster",
  "script": "node",
  "args": ["api.js"],
  "instances": 3,
  "port": "3000-3002"
}
```

**Result:**
- `api-cluster-0` → Port 3000
- `api-cluster-1` → Port 3001  
- `api-cluster-2` → Port 3002

### Auto Port Assignment

Let PMDaemon find available ports automatically:

```json
{
  "name": "worker-cluster",
  "script": "python",
  "args": ["worker.py"],
  "instances": 5,
  "port": "auto:8000-8100"
}
```

PMDaemon will assign the first 5 available ports in the range 8000-8100.

## Environment Variables

Each cluster instance receives automatic environment variables:

- **`PORT`** - The assigned port number
- **`PM2_INSTANCE_ID`** - Instance number (0, 1, 2, ...)
- **`NODE_APP_INSTANCE`** - Node.js compatible instance ID

```javascript
// In your application
const port = process.env.PORT || 3000;
const instanceId = process.env.PM2_INSTANCE_ID || 0;

console.log(`Instance ${instanceId} starting on port ${port}`);
```

## Cluster Management

### Start All Instances

```bash
pmdaemon start ecosystem.json  # Starts all configured instances
```

### Individual Instance Control

```bash
# Stop specific instance
pmdaemon stop web-app-1

# Restart specific instance  
pmdaemon restart web-app-2

# View all instances
pmdaemon list
```

### Cluster-wide Operations

```bash
# Stop all instances of an app
pmdaemon stop web-app

# Restart entire cluster
pmdaemon restart web-app

# Delete cluster
pmdaemon delete web-app
```

## Load Balancing with External Tools

Since PMDaemon handles **process management**, use these tools for **traffic load balancing**:

### Nginx Configuration

```nginx
upstream app_backend {
    server 127.0.0.1:3000;
    server 127.0.0.1:3001;
    server 127.0.0.1:3002;
    server 127.0.0.1:3003;
}

server {
    listen 80;
    server_name example.com;
    
    location / {
        proxy_pass http://app_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### HAProxy Configuration

```
backend app_servers
    balance roundrobin
    server app1 127.0.0.1:3000 check
    server app2 127.0.0.1:3001 check
    server app3 127.0.0.1:3002 check
    server app4 127.0.0.1:3003 check

frontend app_frontend
    bind *:80
    default_backend app_servers
```

### Node.js Cluster Integration

For Node.js applications, you can combine PMDaemon clustering with Node's built-in cluster module:

```javascript
// server.js
const cluster = require('cluster');
const numCPUs = require('os').cpus().length;

if (cluster.isMaster && process.env.PM2_INSTANCE_ID === '0') {
    // Only fork from the first PMDaemon instance
    for (let i = 0; i < numCPUs; i++) {
        cluster.fork();
    }
} else {
    // Worker process or other PMDaemon instances
    require('./app.js');
}
```

## Monitoring Clusters

### Process List View

```bash
pmdaemon list
```

```
│ ID │ Name        │ Status │ PID   │ Port │ CPU (%) │ Memory   │ Uptime  │ Restarts │
├────┼─────────────┼────────┼───────┼──────┼─────────┼──────────┼─────────┼──────────┤
│ 1  │ web-app-0   │ online │ 1234  │ 3000 │ 15.2    │ 125.4MB  │ 2h 15m  │ 0        │
│ 2  │ web-app-1   │ online │ 1235  │ 3001 │ 12.8    │ 118.7MB  │ 2h 15m  │ 0        │
│ 3  │ web-app-2   │ online │ 1236  │ 3002 │ 18.5    │ 132.1MB  │ 2h 15m  │ 0        │
│ 4  │ web-app-3   │ online │ 1237  │ 3003 │ 14.1    │ 127.9MB  │ 2h 15m  │ 0        │
```

### Real-time Monitoring

```bash
pmdaemon monit --interval 2
```

Monitor all cluster instances with real-time CPU, memory, and status updates.

### API Access

```bash
# List all instances via API
curl http://localhost:9615/api/processes

# Get specific instance
curl http://localhost:9615/api/processes/web-app-1
```

## Best Practices

### 1. Instance Count

```bash
# Match CPU cores for CPU-bound apps
pmdaemon start "node cpu-heavy.js" --instances $(nproc)

# Use fewer instances for I/O-bound apps
pmdaemon start "node io-app.js" --instances 2
```

### 2. Resource Limits

```json
{
  "name": "memory-limited-cluster",
  "script": "node",
  "args": ["app.js"],
  "instances": 4,
  "max_memory_restart": "512M",
  "port": "auto:4000-4100"
}
```

### 3. Health Checks

```json
{
  "name": "health-checked-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 3,
  "port": "5000-5002",
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:{PORT}/health",
    "timeout": 5,
    "interval": 30,
    "retries": 3,
    "enabled": true
  }
}
```

### 4. Graceful Shutdowns

```json
{
  "name": "graceful-cluster",
  "script": "node",
  "args": ["app.js"],
  "instances": 4,
  "kill_timeout": 5000,
  "restart_delay": 1000
}
```

## Troubleshooting

### Port Conflicts

```bash
# Check port allocation
pmdaemon list

# Use auto-assignment to avoid conflicts
# Change from: "port": "3000-3003"
# To: "port": "auto:3000-3100"
```

### Instance Failures

```bash
# Check logs for failed instances
pmdaemon logs web-app-1 --lines 50

# Restart individual instances
pmdaemon restart web-app-1
```

### Memory Issues

```bash
# Monitor memory usage
pmdaemon monit

# Set memory limits
pmdaemon start "node app.js" --max-memory 256M --instances 4
```

## Integration Examples

### Docker Compose

```yaml
version: '3.8'
services:
  app:
    build: .
    command: pmdaemon start ecosystem.json
    ports:
      - "3000-3003:3000-3003"
    environment:
      - NODE_ENV=production
  
  nginx:
    image: nginx
    ports:
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
    depends_on:
      - app
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: pmdaemon-app
spec:
  replicas: 2
  selector:
    matchLabels:
      app: pmdaemon-app
  template:
    metadata:
      labels:
        app: pmdaemon-app
    spec:
      containers:
      - name: app
        image: my-app:latest
        command: ["pmdaemon", "start", "ecosystem.json"]
        ports:
        - containerPort: 3000
        - containerPort: 3001
        - containerPort: 3002
        - containerPort: 3003
```

## Future Roadmap

PMDaemon focuses on **process management excellence**. For advanced load balancing features, we recommend:

- **Traffic Load Balancing**: Nginx, HAProxy, Envoy, or cloud load balancers
- **Service Discovery**: Consul, etcd, or Kubernetes services  
- **Circuit Breakers**: Application-level libraries or service mesh
- **SSL Termination**: Reverse proxy or CDN solutions

This separation of concerns allows PMDaemon to excel at process management while leveraging mature, battle-tested tools for traffic distribution.

---

**Next Steps:**
- **[Port Management](./port-management.md)** - Advanced port allocation strategies
- **[Health Checks](./health-checks.md)** - Application health monitoring
- **[Monitoring](./monitoring.md)** - Process and system monitoring
- **[Configuration](./configuration.md)** - Ecosystem file configuration
