# Monitoring

PMDaemon provides comprehensive **real-time monitoring** capabilities that give you deep insights into your processes and system performance. With configurable update intervals, beautiful table formatting, and detailed metrics collection, you can monitor everything from CPU usage to health check status.

## Overview

PMDaemon's monitoring system includes:

- **📊 Real-time metrics** - CPU, memory, uptime, restart count
- **🎨 Beautiful table formatting** - Professional display with color-coded status
- **⏱️ Configurable intervals** - Customize refresh rates from 1s to minutes
- **🔍 Process details** - PID, port assignments, health status
- **📈 System overview** - CPU, memory, load average, uptime
- **📱 Multiple interfaces** - CLI, Web API, WebSocket

## Real-time Monitoring

### Basic Monitoring

Start real-time monitoring with the `monit` command:

```bash
# Default monitoring (1-second updates)
pmdaemon monit

# Custom update interval
pmdaemon monit --interval 5

# Specific interval formats
pmdaemon monit --interval 2s    # 2 seconds
pmdaemon monit --interval 30s   # 30 seconds
pmdaemon monit --interval 1m    # 1 minute
```

### Monitoring Display

The monitoring interface shows a comprehensive table:

```
┌────┬─────────────┬────────┬─────┬──────┬─────┬────────┬─────────┬──────────┬────────┐
│ ID │ Name        │ Status │ PID │ Port │ CPU │ Memory │ Uptime  │ Restarts │ Health │
├────┼─────────────┼────────┼─────┼──────┼─────┼────────┼─────────┼──────────┼────────┤
│ 0  │ web-api     │ 🟢     │ 123 │ 3000 │ 2.5%│ 45.2MB │ 2h 15m  │ 0        │ ✅     │
│ 1  │ worker      │ 🟢     │ 124 │ -    │ 1.8%│ 32.1MB │ 1h 30m  │ 1        │ ⚠️     │
│ 2  │ processor   │ 🟡     │ 125 │ 8000 │ 0.1%│ 28.5MB │ 45m     │ 2        │ ✅     │
└────┴─────────────┴────────┴─────┴──────┴─────┴────────┴─────────┴──────────┴────────┘
```
```
System Overview:
CPU Usage: 15.2% | Memory: 2.1GB/8.0GB (26%) | Load: 0.85 | Uptime: 5d 12h
```

### Status Indicators

**Process Status Colors:**
- 🟢 **Online** - Process running normally
- 🔴 **Stopped** - Process not running
- 🟡 **Starting** - Process starting up
- 🟠 **Stopping** - Process shutting down
- 🔵 **Restarting** - Process restarting
- ❌ **Errored** - Process crashed or failed

**Health Status Indicators:**
- ✅ **Healthy** - All health checks passing
- ⚠️ **Warning** - Some health checks failing
- ❌ **Unhealthy** - Health checks failed
- ❓ **Unknown** - No health checks configured

## Metrics Collection

### Process Metrics

PMDaemon collects detailed metrics for each process:

| Metric | Description | Source |
|--------|-------------|--------|
| **CPU Usage** | Percentage of CPU time used | `sysinfo` crate |
| **Memory (RSS)** | Resident Set Size in MB | `sysinfo` crate |
| **Process Uptime** | Time since process started | Process start time |
| **Restart Count** | Number of times restarted | PMDaemon tracking |
| **PID** | System process identifier | Process spawn |
| **Port** | Assigned port number(s) | PMDaemon port management |
| **Health Status** | Health check results | Health check system |

### System Metrics

System-wide metrics provide context:

| Metric | Description |
|--------|-------------|
| **System CPU** | Overall CPU usage percentage |
| **System Memory** | Total memory usage and available |
| **Load Average** | System load (1, 5, 15 minute averages) |
| **System Uptime** | How long the system has been running |

### Metric History

PMDaemon maintains metric history for analysis:

```bash
# View process information with metric history
pmdaemon info web-api
```

```yaml
Process Metrics:
  Current:
    CPU: 2.5%
    Memory: 45.2MB
    Uptime: 2h 15m
  Averages (last hour):
    CPU: 3.1%
    Memory: 42.8MB
  Peak Values:
    CPU: 15.2% (at 14:23)
    Memory: 67.1MB (at 13:45)
```

## Configurable Update Intervals

### Performance vs Responsiveness

Choose update intervals based on your needs:

```bash
# High-frequency monitoring (development/debugging)
pmdaemon monit --interval 1s    # Very responsive, higher CPU usage

# Balanced monitoring (general use)
pmdaemon monit --interval 2s    # Good balance (default)

# Low-frequency monitoring (production)
pmdaemon monit --interval 10s   # Lower overhead, less responsive

# Very low frequency (background monitoring)
pmdaemon monit --interval 1m    # Minimal overhead
```

### Library Usage

When using PMDaemon as a library, you can configure monitoring intervals:

```rust
use pmdaemon::ProcessManager;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ProcessManager::new().await?;
    
    // Start monitoring with custom interval
    manager.monitor_with_interval(Duration::from_secs(5)).await?;
    
    Ok(())
}
```

## Process Information

### Detailed Process View

Get comprehensive information about a specific process:

```bash
pmdaemon info web-api
```

```yaml
Process Information:
  Basic:
    Name: web-api
    ID: 0
    Status: Online
    PID: 1234
    Port: 3000
    
  Configuration:
    Script: node
    Args: ["server.js"]
    Working Directory: /app
    Environment: NODE_ENV=production
    
  Runtime:
    CPU Usage: 2.5%
    Memory Usage: 45.2MB
    Uptime: 2h 15m 30s
    Restart Count: 0
    
  Health Check:
    Type: HTTP
    URL: http://localhost:3000/health
    Status: Healthy
    Last Check: 2024-01-15 14:30:25
    Success Rate: 98.5%
    
  Logs:
    Stdout: /var/log/pmdaemon/web-api-0-out.log
    Stderr: /var/log/pmdaemon/web-api-0-err.log
    PID File: /var/run/pmdaemon/web-api-0.pid
```

### Process List

View all processes with key metrics:

```bash
pmdaemon list
```

The list command provides a snapshot of all processes with their current status and key metrics.

## Log Monitoring

### View Process Logs

```bash
# View recent logs
pmdaemon logs web-api

# View specific number of lines
pmdaemon logs web-api --lines 50

# Follow logs in real-time
pmdaemon logs web-api --follow

# View logs with timestamps
pmdaemon logs web-api --timestamps
```

### Log File Management

PMDaemon automatically manages log files:

- **Stdout logs**: `{name}-{instance}-out.log`
- **Stderr logs**: `{name}-{instance}-err.log`
- **Automatic rotation**: Prevents logs from growing too large
- **Structured naming**: Easy to identify logs for specific processes

## Web-based Monitoring

### Web Interface

Start the web monitoring server:

```bash
# Start web server on default port (9615)
pmdaemon web

# Custom port and host
pmdaemon web --port 8080 --host 0.0.0.0
```

Access the web interface at `http://localhost:9615` for:
- Real-time process monitoring
- Interactive process management
- System metrics dashboard
- Log viewing and searching

### REST API Monitoring

Get monitoring data via REST API:

```bash
# Get all processes with metrics
curl http://localhost:9615/api/processes

# Get system metrics
curl http://localhost:9615/api/system

# Get specific process information
curl http://localhost:9615/api/processes/web-api

# Get process logs
curl http://localhost:9615/api/logs/web-api?lines=100
```

### WebSocket Real-time Updates

Connect to WebSocket for live updates:

```bash
# Connect with wscat
wscat -c ws://localhost:9615/ws
```

Receive real-time updates for:
- Process status changes
- Metric updates
- Health check results
- System metrics

## Performance Considerations

### Monitoring Overhead

Different intervals have different performance impacts:

| Interval | CPU Impact | Memory Impact | Use Case |
|----------|------------|---------------|----------|
| 1s | High | Low | Development, debugging |
| 2s | Medium | Low | General monitoring |
| 5s | Low | Low | Production monitoring |
| 30s+ | Minimal | Minimal | Background monitoring |

### Optimization Tips

1. **Use appropriate intervals**:
   ```bash
   # Development
   pmdaemon monit --interval 1s
   
   # Production
   pmdaemon monit --interval 5s
   ```

2. **Monitor selectively**:
   ```bash
   # Monitor specific processes only
   pmdaemon info critical-service
   ```

3. **Use web interface for continuous monitoring**:
   ```bash
   # Start web server for dashboard
   pmdaemon web --port 9615
   ```

## Alerting and Notifications

### Built-in Alerting

PMDaemon provides built-in alerting through:

- **Health check failures** - Automatic restart on health failures
- **Memory limit exceeded** - Restart when memory limits are breached
- **Process crashes** - Automatic restart with configurable limits

### Custom Alerting

Integrate with external alerting systems:

```bash
# Monitor via script
#!/bin/bash
while true; do
    STATUS=$(pmdaemon list --format json | jq -r '.processes[] | select(.name=="critical-service") | .status')
    if [ "$STATUS" != "online" ]; then
        # Send alert (email, Slack, etc.)
        send_alert "Critical service is down: $STATUS"
    fi
    sleep 30
done
```

## Monitoring Best Practices

### 1. Choose Appropriate Intervals

```bash
# Critical services - frequent monitoring
pmdaemon monit --interval 2s

# Background services - less frequent
pmdaemon monit --interval 30s
```

### 2. Monitor Key Metrics

Focus on metrics that matter:
- **CPU usage** - Detect performance issues
- **Memory usage** - Prevent memory leaks
- **Restart count** - Identify unstable processes
- **Health status** - Ensure functionality

### 3. Set Up Proper Logging

```bash
# Ensure logs are captured
pmdaemon start "node server.js" \
  --name web-api \
  --out-file /var/log/web-api.out \
  --error-file /var/log/web-api.err
```

### 4. Use Health Checks

```bash
# Combine monitoring with health checks
pmdaemon start "node api.js" \
  --name api \
  --health-check-url http://localhost:3000/health \
  --health-check-interval 30s
```

## Troubleshooting Monitoring

### High CPU Usage in Monitoring

```bash
# Reduce monitoring frequency
pmdaemon monit --interval 10s

# Check system load
pmdaemon monit  # Look at system CPU usage
```

### Missing Metrics

```bash
# Verify process is running
pmdaemon list

# Check process permissions
pmdaemon info process-name

# Restart PMDaemon if needed
pmdaemon restart process-name
```

### Inaccurate Metrics

```bash
# Verify system tools are available
which ps htop

# Check PMDaemon version
pmdaemon --version

# Review logs for errors
pmdaemon logs process-name
```

## Next Steps

- **[Health Checks](./health-checks.md)** - Set up health monitoring
- **[Web API](./web-api.md)** - Access monitoring via API
- **[Performance Tuning](../advanced/performance-tuning.md)** - Optimize monitoring performance
- **[Troubleshooting](../advanced/troubleshooting.md)** - Solve monitoring issues
