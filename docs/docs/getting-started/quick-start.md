# Quick Start

Get up and running with PMDaemon in just 5 minutes! This guide covers the essential commands and features you need to start managing processes.

## Basic Process Management

### Start Your First Process

```bash
# Start a simple process
pmdaemon start "node server.js" --name my-app

# Start with custom options
pmdaemon start "python app.py" \
  --name python-api \
  --port 8000 \
  --max-memory 512M \
  --env NODE_ENV=production
```

### View Running Processes

```bash
# List all processes
pmdaemon list
```

You'll see a beautiful table with:
- **ID** - Process identifier
- **Name** - Process name
- **Status** - Current state (🟢 Online, 🔴 Stopped, etc.)
- **PID** - System process ID
- **Port** - Assigned port(s)
- **CPU** - CPU usage percentage
- **Memory** - Memory usage (RSS)
- **Uptime** - How long the process has been running
- **Restarts** - Number of restarts

### Control Processes

```bash
# Stop a process
pmdaemon stop my-app

# Restart a process
pmdaemon restart my-app

# Graceful reload (zero-downtime)
pmdaemon reload my-app

# Delete a process (stops if running)
pmdaemon delete my-app
```

## Advanced Features

### Clustering with Port Management

PMDaemon's **advanced port management** goes beyond PM2:

```bash
# Start 4 instances with consecutive ports
pmdaemon start "node server.js" \
  --name web-cluster \
  --instances 4 \
  --port 3000-3003

# Auto-assign ports from a range
pmdaemon start "python worker.py" \
  --name workers \
  --instances 3 \
  --port auto:5000-5100
```

Each instance gets:
- **Unique port** from the specified range
- **Environment variables**: `PORT`, `PM2_INSTANCE_ID`, `NODE_APP_INSTANCE`
- **Automatic load balancing**

### Health Checks & Blocking Start

Ensure your processes are ready before continuing:

```bash
# Start with HTTP health check
pmdaemon start "node api.js" \
  --name api-server \
  --port 3000 \
  --health-check-url http://localhost:3000/health \
  --wait-ready

# Start with script-based health check
pmdaemon start "python worker.py" \
  --name background-worker \
  --health-check-script ./health-check.sh \
  --wait-timeout 30s \
  --wait-ready
```

The `--wait-ready` flag blocks until health checks pass, perfect for deployment scripts!

### Real-time Monitoring

```bash
# Real-time monitoring (default 1-second updates)
pmdaemon monit

# Custom update interval
pmdaemon monit --interval 5

# View process logs
pmdaemon logs my-app

# Follow logs in real-time
pmdaemon logs my-app --follow
```

### Bulk Operations

PMDaemon supports powerful bulk operations:

```bash
# Delete all processes
pmdaemon delete all --force

# Delete only stopped processes
pmdaemon delete stopped --status --force

# Delete only errored processes
pmdaemon delete errored --status
```

## Configuration Files

For complex setups, use ecosystem configuration files:

### Create ecosystem.json

```json
{
  "apps": [
    {
      "name": "web-app",
      "script": "node",
      "args": ["server.js"],
      "instances": 2,
      "port": "3000-3001",
      "max_memory_restart": "512M",
      "env": {
        "NODE_ENV": "production"
      }
    },
    {
      "name": "api-service",
      "script": "python",
      "args": ["-m", "uvicorn", "main:app"],
      "port": "8000",
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:8000/health",
        "timeout": 5,
        "interval": 30,
        "retries": 3
      }
    }
  ]
}
```

### Use Configuration File

```bash
# Start all apps from config
pmdaemon --config ecosystem.json start

# Start specific app
pmdaemon --config ecosystem.json start --name web-app
```

## Web API & Remote Monitoring

Start the web server for remote access:

```bash
# Start web API server
pmdaemon web --port 9615 --host 0.0.0.0
```

Now you can:

### REST API
```bash
# List processes via API
curl http://localhost:9615/api/processes

# Get system metrics
curl http://localhost:9615/api/system

# Start a process via API
curl -X POST http://localhost:9615/api/processes \
  -H "Content-Type: application/json" \
  -d '{"name": "api-test", "script": "node", "args": ["app.js"]}'
```

### WebSocket (Real-time Updates)
```bash
# Connect to WebSocket for live updates
wscat -c ws://localhost:9615/ws
```

## Common Patterns

### Development Server
```bash
pmdaemon start "npm run dev" \
  --name dev-server \
  --env NODE_ENV=development \
  --port 3000
```

### Production API Cluster
```bash
pmdaemon start "node api.js" \
  --name prod-api \
  --instances 4 \
  --port auto:3000-3100 \
  --max-memory 1G \
  --health-check-url http://localhost:3000/health \
  --env NODE_ENV=production
```

### Python Microservice
```bash
pmdaemon start "python -m uvicorn main:app --host 0.0.0.0" \
  --name python-api \
  --port 8000 \
  --max-memory 512M \
  --health-check-url http://localhost:8000/docs
```

### Background Worker
```bash
pmdaemon start "python worker.py" \
  --name background-worker \
  --health-check-script ./worker-health.sh \
  --max-memory 256M
```

## Runtime Port Overrides

Change ports without modifying saved configuration:

```bash
# Start with default port from config
pmdaemon start "node server.js" --name web --port 3000

# Restart with different port (doesn't modify saved config)
pmdaemon restart web --port 3001

# Reload with port range for clustering
pmdaemon reload web --port 4000-4003
```

## Next Steps

Now that you're familiar with the basics:

1. **[Process Management](../features/process-management.md)** - Deep dive into lifecycle management
2. **[Port Management](../features/port-management.md)** - Master advanced port features
3. **[Health Checks](../features/health-checks.md)** - Set up robust monitoring
4. **[CLI Reference](../cli/commands.md)** - Complete command documentation
5. **[Configuration](../configuration/ecosystem-files.md)** - Advanced configuration options

## Need Help?

- **[Troubleshooting](../advanced/troubleshooting.md)** - Common issues and solutions
- **[Examples](../examples/use-cases.md)** - Real-world use cases
- **[GitHub Issues](https://github.com/entrepeneur4lyf/pmdaemon/issues)** - Report bugs or request features

---

**Congratulations!** 🎉 You're now ready to use PMDaemon for process management. The advanced features like port management, health checks, and real-time monitoring will help you build robust, production-ready applications.
