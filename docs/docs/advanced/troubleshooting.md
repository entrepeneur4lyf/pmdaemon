# Troubleshooting

This comprehensive troubleshooting guide helps you diagnose and resolve common issues with PMDaemon. From process startup failures to performance problems, find solutions to keep your applications running smoothly.

## Common Issues

### Process Won't Start

#### Symptoms
- Process shows "errored" status immediately
- "Failed to start process" error message
- Process starts then immediately crashes

#### Diagnosis Steps

1. **Check the script path:**
```bash
# Verify the script exists and is executable
ls -la /path/to/your/script
which node  # For Node.js scripts
which python  # For Python scripts
```

2. **Check permissions:**
```bash
# Ensure script is executable
chmod +x /path/to/your/script

# Check if PMDaemon can access the directory
ls -la /path/to/working/directory
```

3. **Test the script manually:**
```bash
# Run the script directly to see error messages
cd /path/to/working/directory
node server.js
python app.py
```

4. **Check PMDaemon logs:**
```bash
pmdaemon logs process-name --lines 50
```

#### Common Solutions

**Script not found:**
```bash
# Use absolute paths
pmdaemon start "/usr/bin/node" --name web-api --args "/full/path/to/server.js"

# Or set correct working directory
pmdaemon start "node server.js" --name web-api --cwd /path/to/app
```

**Permission denied:**
```bash
# Fix script permissions
chmod +x script.sh

# Run as different user (future feature)
pmdaemon start "node server.js" --name web-api --user app-user
```

**Missing dependencies:**
```bash
# Install dependencies first
cd /path/to/app
npm install  # Node.js
pip install -r requirements.txt  # Python

# Then start the process
pmdaemon start "node server.js" --name web-api --cwd /path/to/app
```

### Port Conflicts

#### Symptoms
- "Port already in use" error
- Process starts but can't bind to port
- Connection refused when accessing service

#### Diagnosis Steps

1. **Check what's using the port:**
```bash
# Linux/macOS
lsof -i :3000
netstat -tulpn | grep :3000

# Alternative
ss -tulpn | grep :3000
```

2. **Check PMDaemon port assignments:**
```bash
pmdaemon list  # Shows port assignments
```

#### Solutions

**Kill conflicting process:**
```bash
# Find and kill the process using the port
lsof -ti:3000 | xargs kill -9

# Or use fuser
fuser -k 3000/tcp
```

**Use different port:**
```bash
pmdaemon start "node server.js" --name web-api --port 3001
```

**Use auto port assignment:**
```bash
pmdaemon start "node server.js" --name web-api --port auto:3000-3100
```

### Health Checks Failing

#### Symptoms
- Process shows "unhealthy" status
- Health check timeouts
- Process restarts due to health failures

#### Diagnosis Steps

1. **Test health check manually:**
```bash
# For HTTP health checks
curl -v http://localhost:3000/health

# For script health checks
./health-check.sh
echo $?  # Should be 0 for healthy
```

2. **Check health check configuration:**
```bash
pmdaemon info process-name
```

3. **Review application logs:**
```bash
pmdaemon logs process-name --lines 100
```

#### Solutions

**Increase timeout:**
```bash
pmdaemon start "node server.js" --name web-api \
  --health-check-url http://localhost:3000/health \
  --health-check-timeout 30s
```

**Fix health endpoint:**
```javascript
// Node.js - Ensure health endpoint responds quickly
app.get('/health', (req, res) => {
  // Quick health check - avoid heavy operations
  res.status(200).json({ status: 'healthy', timestamp: new Date() });
});
```

**Adjust retry settings:**
```bash
pmdaemon start "node server.js" --name web-api \
  --health-check-url http://localhost:3000/health \
  --health-check-retries 5 \
  --health-check-interval 60s
```

### Memory Issues

#### Symptoms
- Process restarting due to memory limits
- "Memory limit exceeded" messages
- System running out of memory

#### Diagnosis Steps

1. **Check memory usage:**
```bash
# System memory
free -h
top
htop

# Process memory
pmdaemon list  # Shows memory usage
pmdaemon monit  # Real-time monitoring
```

2. **Check memory limits:**
```bash
pmdaemon info process-name
```

#### Solutions

**Increase memory limit:**
```bash
pmdaemon restart process-name --max-memory 1G
```

**Optimize application memory usage:**
```javascript
// Node.js - Increase heap size
pmdaemon start "node --max-old-space-size=1024 server.js" --name web-api

// Monitor memory usage in application
setInterval(() => {
  const usage = process.memoryUsage();
  console.log('Memory usage:', {
    rss: Math.round(usage.rss / 1024 / 1024) + 'MB',
    heapUsed: Math.round(usage.heapUsed / 1024 / 1024) + 'MB'
  });
}, 60000);
```

**Scale horizontally instead:**
```bash
# Instead of one large instance
pmdaemon delete memory-hungry-app

# Use multiple smaller instances
pmdaemon start "node server.js" --name web-cluster \
  --instances 4 --max-memory 512M --port 3000-3003
```

### Performance Problems

#### Symptoms
- High CPU usage
- Slow response times
- Process becoming unresponsive

#### Diagnosis Steps

1. **Monitor system resources:**
```bash
# Real-time monitoring
pmdaemon monit

# System monitoring
top
htop
iostat 1
```

2. **Check process configuration:**
```bash
pmdaemon info process-name
```

3. **Profile the application:**
```bash
# Node.js profiling
pmdaemon start "node --inspect server.js" --name web-api

# Python profiling
pmdaemon start "python -m cProfile app.py" --name api
```

#### Solutions

**Scale with clustering:**
```bash
pmdaemon restart web-api --instances 4 --port 3000-3003
```

**Optimize application code:**
```javascript
// Node.js - Use clustering
const cluster = require('cluster');
const numCPUs = require('os').cpus().length;

if (cluster.isMaster) {
  for (let i = 0; i < numCPUs; i++) {
    cluster.fork();
  }
} else {
  // Worker process
  require('./server');
}
```

**Adjust process priorities:**
```bash
# Lower priority for background tasks
nice -n 10 pmdaemon start "python worker.py" --name background-worker
```

### Log Issues

#### Symptoms
- Missing log files
- Log files growing too large
- Cannot access log files

#### Diagnosis Steps

1. **Check log file locations:**
```bash
pmdaemon info process-name | grep -A 10 "Log Files"
```

2. **Check file permissions:**
```bash
ls -la /path/to/log/files
```

3. **Check disk space:**
```bash
df -h
```

#### Solutions

**Fix log file permissions:**
```bash
# Create log directory
sudo mkdir -p /var/log/pmdaemon
sudo chown $USER:$USER /var/log/pmdaemon

# Restart with correct log path
pmdaemon restart process-name --out-file /var/log/pmdaemon/app.out
```

**Implement log rotation:**
```bash
# Use logrotate
sudo tee /etc/logrotate.d/pmdaemon << EOF
/var/log/pmdaemon/*.log {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    copytruncate
}
EOF
```

**Redirect logs to syslog:**
```bash
pmdaemon start "node server.js | logger -t web-api" --name web-api
```

## Advanced Troubleshooting

### Debug Mode

Enable verbose logging for detailed troubleshooting:

```bash
# Enable debug logging
export PMDAEMON_LOG_LEVEL=debug
pmdaemon --verbose start "node server.js" --name web-api

# Check debug logs
pmdaemon logs pmdaemon-daemon --lines 100
```

### Process Tracing

Trace system calls and signals:

```bash
# Linux - trace process system calls
strace -p $(pmdaemon info web-api | grep PID | awk '{print $2}')

# macOS - trace process
dtruss -p $(pmdaemon info web-api | grep PID | awk '{print $2}')
```

### Network Debugging

Debug network-related issues:

```bash
# Check network connections
netstat -tulpn | grep $(pmdaemon info web-api | grep PID | awk '{print $2}')

# Monitor network traffic
tcpdump -i any port 3000

# Test connectivity
telnet localhost 3000
nc -zv localhost 3000
```

### Core Dumps

Enable core dumps for crash analysis:

```bash
# Enable core dumps
ulimit -c unlimited

# Set core dump pattern
echo '/tmp/core.%e.%p' | sudo tee /proc/sys/kernel/core_pattern

# Restart process
pmdaemon restart process-name
```

## Environment-Specific Issues

### Docker Containers

**Issue: Process not starting in container**
```dockerfile
# Ensure proper signal handling
FROM node:16
COPY . /app
WORKDIR /app
CMD ["pmdaemon", "start", "node", "server.js", "--name", "web-api"]
```

**Issue: Health checks failing in container**
```bash
# Use container-appropriate health check URLs
pmdaemon start "node server.js" --name web-api \
  --health-check-url http://0.0.0.0:3000/health
```

### Systemd Integration

**Issue: PMDaemon not starting on boot**
```ini
# /etc/systemd/system/pmdaemon.service
[Unit]
Description=PMDaemon Process Manager
After=network.target

[Service]
Type=forking
User=pmdaemon
ExecStart=/usr/local/bin/pmdaemon start --config /etc/pmdaemon/ecosystem.json
ExecReload=/usr/local/bin/pmdaemon reload all
Restart=always

[Install]
WantedBy=multi-user.target
```

### Cloud Deployments

**Issue: Processes failing in cloud environments**
```bash
# Handle cloud metadata services
pmdaemon start "node server.js" --name web-api \
  --env CLOUD_PROVIDER=aws \
  --env INSTANCE_METADATA_URL=http://169.254.169.254
```

## Diagnostic Commands

### System Information

```bash
# PMDaemon version and system info
pmdaemon --version
pmdaemon system-info

# Process information
pmdaemon info process-name
pmdaemon describe process-name

# System resources
pmdaemon system-resources
```

### Log Analysis

```bash
# Recent logs with timestamps
pmdaemon logs process-name --lines 100 --timestamps

# Follow logs in real-time
pmdaemon logs process-name --follow

# Search logs for errors
pmdaemon logs process-name --lines 1000 | grep -i error
```

### Performance Analysis

```bash
# Real-time monitoring
pmdaemon monit --interval 1s

# Process tree
pmdaemon tree

# Resource usage history
pmdaemon stats process-name
```

## Getting Help

### Community Support

1. **GitHub Issues**: Report bugs and request features
2. **Documentation**: Check the latest documentation
3. **Examples**: Review example configurations

### Providing Debug Information

When reporting issues, include:

```bash
# System information
uname -a
pmdaemon --version

# Process configuration
pmdaemon info process-name

# Recent logs
pmdaemon logs process-name --lines 50

# System resources
free -h
df -h
```

### Creating Minimal Reproduction

```bash
# Create minimal test case
echo 'console.log("Hello World"); setInterval(() => {}, 1000);' > test.js
pmdaemon start "node test.js" --name test-process

# Test with minimal configuration
pmdaemon start "node test.js" --name test --port 3000 --health-check-url http://localhost:3000/health
```

## Prevention Strategies

### Monitoring Setup

```bash
# Set up comprehensive monitoring
pmdaemon start "node server.js" --name web-api \
  --health-check-url http://localhost:3000/health \
  --health-check-interval 30s \
  --max-memory 512M \
  --max-restarts 5
```

### Automated Testing

```bash
#!/bin/bash
# deployment-test.sh

# Start service
pmdaemon start "node server.js" --name test-api --port 3000 --wait-ready

# Test health endpoint
if curl -f http://localhost:3000/health; then
    echo "✅ Service is healthy"
else
    echo "❌ Service health check failed"
    pmdaemon logs test-api --lines 20
    exit 1
fi

# Cleanup
pmdaemon delete test-api
```

### Configuration Validation

```bash
# Validate configuration before deployment
pmdaemon validate ecosystem.json --strict

# Test configuration in staging
pmdaemon --config ecosystem.staging.json start
```

## Next Steps

- **[Performance Tuning](./performance-tuning.md)** - Optimize PMDaemon performance
- **[Security](./security.md)** - Security best practices
- **[Logging](./logging.md)** - Advanced logging configuration
- **[Examples](../examples/deployment-examples.md)** - Real-world deployment examples
