# Monitoring Overview

PMDaemon provides comprehensive monitoring capabilities to help you track the health and performance of your applications and the daemon itself.

## Built-in Monitoring Features

### Process Metrics
- **CPU Usage**: Real-time CPU utilization per process
- **Memory Usage**: RAM consumption tracking with leak detection
- **Uptime**: Process runtime and restart statistics
- **Status**: Current process state (running, stopped, errored, etc.)

### System Health Checks
- **Health Endpoints**: Configurable HTTP health check endpoints
- **Custom Health Scripts**: Execute custom validation scripts
- **Resource Limits**: Monitor and enforce CPU/memory limits
- **Auto-restart**: Automatic restart on failure or resource exhaustion

### Log Monitoring
- **Centralized Logging**: Unified log collection and rotation
- **Log Levels**: Configurable log levels (error, warn, info, debug)
- **Error Tracking**: Automatic error detection and alerting
- **Log Rotation**: Automatic log file rotation and cleanup

## Monitoring Endpoints

PMDaemon exposes several monitoring endpoints through its REST API:

### Health Check Endpoint
```bash
GET /health
```

Returns the overall health status of the daemon and all managed processes.

### Process Metrics
```bash
GET /api/processes/{id}/metrics
```

Returns detailed metrics for a specific process including:
- CPU usage percentage
- Memory usage (RSS, heap, etc.)
- Process uptime
- Restart count
- Error count

### System Stats
```bash
GET /api/system/stats
```

Returns system-wide statistics:
- Total CPU usage
- Available memory
- Disk usage
- Network statistics

## External Monitoring Integration

### Prometheus Integration
PMDaemon can expose metrics in Prometheus format for integration with monitoring stacks:

```toml
[monitoring.prometheus]
enabled = true
port = 9090
endpoint = "/metrics"
```

### Custom Webhooks
Configure webhooks to send alerts to external systems:

```toml
[monitoring.webhooks]
enabled = true
url = "https://your-monitoring-system.com/webhook"
events = ["process_crash", "high_memory", "restart_loop"]
```

### Log Forwarding
Forward logs to external logging systems:

```toml
[monitoring.logging]
forward_to = "syslog"
syslog_address = "localhost:514"
format = "json"
```

## Alerting

### Built-in Alerts
- Process crashes
- High memory usage
- CPU threshold breaches
- Restart loops
- Health check failures

### Alert Configuration
```toml
[monitoring.alerts]
cpu_threshold = 80  # Percentage
memory_threshold = "1GB"
restart_threshold = 5  # restarts in 10 minutes
```

### Notification Channels
- Email notifications
- Slack integration
- Discord webhooks
- Custom HTTP endpoints

## Performance Monitoring

### Real-time Metrics
Monitor performance metrics in real-time through the web dashboard or CLI:

```bash
# View real-time process metrics
pmdaemon monitor

# Show detailed process info
pmdaemon show <process_name>

# Display system overview
pmdaemon status
```

### Historical Data
PMDaemon can store historical performance data for trend analysis:

```toml
[monitoring.history]
enabled = true
retention_days = 30
storage_path = "/var/lib/pmdaemon/metrics"
```

## Dashboard Integration

### Web Dashboard
Access the built-in web dashboard at `http://localhost:3000` (configurable) to view:
- Process status overview
- Real-time metrics graphs
- Log viewer
- Alert history

### Third-party Dashboards
Integrate with popular monitoring dashboards:
- Grafana (via Prometheus)
- DataDog (via StatsD)
- New Relic (via HTTP API)

## Monitoring Best Practices

1. **Set Appropriate Thresholds**: Configure CPU and memory thresholds based on your application's normal behavior
2. **Enable Health Checks**: Implement proper health check endpoints in your applications
3. **Monitor Resource Trends**: Watch for gradual increases that might indicate memory leaks
4. **Set Up Alerts**: Configure notifications for critical events
5. **Regular Log Review**: Periodically review logs for patterns and issues

## Troubleshooting Monitoring Issues

### Common Problems
- High monitoring overhead
- Missing metrics
- False positive alerts
- Dashboard connection issues

### Solutions
- Adjust monitoring intervals
- Check endpoint configurations
- Review alert thresholds
- Verify network connectivity

For more detailed monitoring configuration, see our [Configuration Guide](../configuration/overview.md).
