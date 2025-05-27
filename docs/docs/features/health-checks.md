# Health Checks

PMDaemon's **health check system** provides robust monitoring and validation of your processes, ensuring they're not just running but actually functioning correctly. This goes beyond basic process monitoring to verify that your applications are healthy and ready to serve traffic.

## Overview

Health checks in PMDaemon offer:

- **🏥 HTTP health checks** - Monitor web services via HTTP endpoints
- **📜 Script-based health checks** - Custom validation logic for any application type
- **⏱️ Configurable parameters** - Timeout, interval, and retry settings
- **🚦 Blocking start command** - Wait for processes to be healthy before continuing
- **🔄 Auto-restart on failure** - Automatic restart when health checks fail
- **📊 Health status integration** - Visible in all monitoring interfaces

## Health Check Types

### HTTP Health Checks

Monitor web services by making HTTP requests to specific endpoints:

```bash
# Basic HTTP health check
pmdaemon start "node server.js" \
  --name web-api \
  --port 3000 \
  --health-check-url http://localhost:3000/health

# With custom parameters
pmdaemon start "python api.py" \
  --name python-api \
  --port 8000 \
  --health-check-url http://localhost:8000/status \
  --health-check-timeout 10s \
  --health-check-interval 30s \
  --health-check-retries 3
```

**How it works:**
1. PMDaemon makes HTTP GET requests to the specified URL
2. Considers 2xx status codes as healthy
3. Retries on failure according to retry settings
4. Marks process as unhealthy after max retries exceeded

### Script-based Health Checks

Run custom scripts for complex health validation:

```bash
# Basic script health check
pmdaemon start "python worker.py" \
  --name background-worker \
  --health-check-script ./health-check.sh

# With custom parameters
pmdaemon start "node processor.js" \
  --name data-processor \
  --health-check-script ./scripts/check-processor.py \
  --health-check-timeout 15s \
  --health-check-interval 60s \
  --health-check-retries 2
```

**How it works:**
1. PMDaemon executes the specified script/command
2. Exit code 0 indicates healthy, non-zero indicates unhealthy
3. Script output is captured for debugging
4. Retries on failure according to retry settings

## Configuration Parameters

### Timeout Settings

Control how long to wait for health check responses:

```bash
# Short timeout for fast services
--health-check-timeout 5s

# Longer timeout for complex checks
--health-check-timeout 30s

# Very long timeout for heavy operations
--health-check-timeout 2m
```

**Supported formats:**
- `5s` - 5 seconds
- `30s` - 30 seconds  
- `2m` - 2 minutes
- `1h` - 1 hour

### Interval Settings

Configure how often health checks run:

```bash
# Frequent checks for critical services
--health-check-interval 10s

# Standard interval for most services
--health-check-interval 30s

# Less frequent for stable services
--health-check-interval 5m
```

### Retry Settings

Set how many times to retry failed health checks:

```bash
# Conservative - fail fast
--health-check-retries 1

# Balanced - allow for temporary issues
--health-check-retries 3

# Aggressive - very tolerant of failures
--health-check-retries 5
```

## Blocking Start Command

The `--wait-ready` flag makes the start command wait until health checks pass:

```bash
# Wait for HTTP service to be ready
pmdaemon start "node api.js" \
  --name api-service \
  --port 3000 \
  --health-check-url http://localhost:3000/health \
  --wait-ready

# Wait with custom timeout
pmdaemon start "python worker.py" \
  --name worker \
  --health-check-script ./health.sh \
  --wait-ready \
  --wait-timeout 60s
```

**Perfect for deployment scripts:**
```bash
#!/bin/bash
# Deploy script that waits for services

echo "Starting API service..."
pmdaemon start "node api.js" \
  --name api \
  --health-check-url http://localhost:3000/health \
  --wait-ready

echo "API is ready! Starting worker..."
pmdaemon start "python worker.py" \
  --name worker \
  --health-check-script ./worker-health.sh \
  --wait-ready

echo "All services are healthy and ready!"
```

## Health Check Examples

### Web API Health Check

**Application code (Node.js):**
```javascript
// server.js
const express = require('express');
const app = express();

// Health check endpoint
app.get('/health', (req, res) => {
  // Check database connection, external services, etc.
  const isHealthy = checkDatabase() && checkRedis();
  
  if (isHealthy) {
    res.status(200).json({ status: 'healthy', timestamp: new Date() });
  } else {
    res.status(503).json({ status: 'unhealthy', timestamp: new Date() });
  }
});

app.listen(3000);
```

**PMDaemon configuration:**
```bash
pmdaemon start "node server.js" \
  --name web-api \
  --port 3000 \
  --health-check-url http://localhost:3000/health \
  --health-check-timeout 5s \
  --health-check-interval 30s \
  --health-check-retries 3 \
  --wait-ready
```

### Database Worker Health Check

**Health check script:**
```bash
#!/bin/bash
# worker-health.sh

# Check if worker process is responding
if ! pgrep -f "python worker.py" > /dev/null; then
    echo "Worker process not found"
    exit 1
fi

# Check if worker can connect to database
if ! python -c "import psycopg2; psycopg2.connect('host=localhost dbname=mydb')" 2>/dev/null; then
    echo "Cannot connect to database"
    exit 1
fi

# Check if worker queue is not too backed up
QUEUE_SIZE=$(redis-cli llen worker_queue)
if [ "$QUEUE_SIZE" -gt 1000 ]; then
    echo "Queue too large: $QUEUE_SIZE items"
    exit 1
fi

echo "Worker is healthy"
exit 0
```

**PMDaemon configuration:**
```bash
pmdaemon start "python worker.py" \
  --name db-worker \
  --health-check-script ./worker-health.sh \
  --health-check-timeout 10s \
  --health-check-interval 60s \
  --health-check-retries 2
```

### Microservice Health Check

**Python FastAPI with health endpoint:**
```python
# main.py
from fastapi import FastAPI, HTTPException
import asyncio
import aioredis

app = FastAPI()

@app.get("/health")
async def health_check():
    try:
        # Check Redis connection
        redis = await aioredis.from_url("redis://localhost")
        await redis.ping()
        await redis.close()
        
        # Check other dependencies...
        
        return {"status": "healthy", "checks": {"redis": "ok"}}
    except Exception as e:
        raise HTTPException(status_code=503, detail=f"Unhealthy: {str(e)}")
```

**PMDaemon configuration:**
```bash
pmdaemon start "python -m uvicorn main:app --host 0.0.0.0 --port 8000" \
  --name microservice \
  --port 8000 \
  --health-check-url http://localhost:8000/health \
  --health-check-timeout 15s \
  --health-check-interval 45s \
  --wait-ready
```

## Health Status Integration

Health status is visible throughout PMDaemon's interfaces:

### Process List
```bash
pmdaemon list
```

```
┌────┬─────────────┬────────┬─────┬──────┬─────┬────────┬─────────┬──────────┬────────┐
│ ID │ Name        │ Status │ PID │ Port │ CPU │ Memory │ Uptime  │ Restarts │ Health │
├────┼─────────────┼────────┼─────┼──────┼─────┼────────┼─────────┼──────────┼────────┤
│ 0  │ web-api     │ 🟢     │ 123 │ 3000 │ 2%  │ 45MB   │ 2h 15m  │ 0        │ ✅     │
│ 1  │ worker      │ 🟢     │ 124 │ -    │ 1%  │ 32MB   │ 1h 30m  │ 1        │ ⚠️     │
│ 2  │ processor   │ 🔴     │ -   │ -    │ -   │ -      │ -       │ 3        │ ❌     │
└────┴─────────────┴────────┴─────┴──────┴─────┴────────┴─────────┴──────────┴────────┘
```

**Health indicators:**
- ✅ Healthy - All health checks passing
- ⚠️ Warning - Some health checks failing but within retry limits
- ❌ Unhealthy - Health checks failed, process may be restarted
- ❓ Unknown - Health checks not configured or not yet run

### Real-time Monitoring
```bash
pmdaemon monit
```
Shows live health status updates with color-coded indicators.

### Process Information
```bash
pmdaemon info web-api
```

```yaml
Process Information:
  Name: web-api
  Status: Online
  PID: 1234
  Port: 3000
  Health Check:
    Type: HTTP
    URL: http://localhost:3000/health
    Status: Healthy
    Last Check: 2024-01-15 14:30:25
    Success Rate: 98.5% (197/200)
    Timeout: 5s
    Interval: 30s
    Retries: 3
```

## Auto-restart on Health Failure

When health checks fail consistently, PMDaemon can automatically restart the process:

```bash
# Enable auto-restart on health failure (default behavior)
pmdaemon start "node api.js" \
  --name api \
  --health-check-url http://localhost:3000/health \
  --health-check-retries 3  # Restart after 3 consecutive failures
```

**Restart behavior:**
1. Health check fails
2. PMDaemon retries according to `--health-check-retries`
3. If all retries fail, process is marked as unhealthy
4. Process is automatically restarted
5. Health checks resume after restart

## Web API Integration

Health status is available via the REST API and WebSocket:

### REST API
```bash
# Get all processes with health status
curl http://localhost:9615/api/processes

# Get specific process health
curl http://localhost:9615/api/processes/web-api/health
```

### WebSocket Updates
```bash
# Connect to WebSocket for real-time health updates
wscat -c ws://localhost:9615/ws
```

Health status changes are broadcast in real-time to connected clients.

## Best Practices

### 1. Design Proper Health Endpoints

```javascript
// Good: Comprehensive health check
app.get('/health', async (req, res) => {
  const checks = {
    database: await checkDatabase(),
    redis: await checkRedis(),
    external_api: await checkExternalAPI(),
    disk_space: checkDiskSpace()
  };
  
  const isHealthy = Object.values(checks).every(check => check.healthy);
  
  res.status(isHealthy ? 200 : 503).json({
    status: isHealthy ? 'healthy' : 'unhealthy',
    checks,
    timestamp: new Date()
  });
});

// Avoid: Simple always-healthy endpoint
app.get('/health', (req, res) => {
  res.json({ status: 'ok' });  // Not useful
});
```

### 2. Set Appropriate Timeouts

```bash
# Fast web APIs
--health-check-timeout 5s --health-check-interval 30s

# Database operations
--health-check-timeout 15s --health-check-interval 60s

# Heavy batch processing
--health-check-timeout 30s --health-check-interval 300s
```

### 3. Use Blocking Start for Dependencies

```bash
# Start database first and wait
pmdaemon start "postgres" --name db --wait-ready

# Then start API that depends on database
pmdaemon start "node api.js" \
  --name api \
  --health-check-url http://localhost:3000/health \
  --wait-ready
```

### 4. Monitor Health Check Performance

```bash
# View health check statistics
pmdaemon info process-name

# Monitor for patterns in health failures
pmdaemon logs process-name | grep "health"
```

## Troubleshooting

### Health Checks Always Failing

```bash
# Check if health endpoint is accessible
curl http://localhost:3000/health

# Verify health check script manually
./health-check.sh
echo $?  # Should be 0 for healthy

# Check PMDaemon logs
pmdaemon logs process-name
```

### Blocking Start Timing Out

```bash
# Increase wait timeout
pmdaemon start app.js \
  --health-check-url http://localhost:3000/health \
  --wait-ready \
  --wait-timeout 120s  # Increase from default 30s

# Check what's preventing health checks from passing
curl -v http://localhost:3000/health
```

### False Positive Health Failures

```bash
# Increase retry count for flaky services
--health-check-retries 5

# Increase timeout for slow responses
--health-check-timeout 30s

# Reduce check frequency
--health-check-interval 120s
```

## Next Steps

- **[Monitoring](./monitoring.md)** - Real-time process monitoring
- **[Web API](./web-api.md)** - Access health status via API
- **[Deployment Examples](../examples/deployment-examples.md)** - Production deployment patterns
- **[Troubleshooting](../advanced/troubleshooting.md)** - Common issues and solutions
