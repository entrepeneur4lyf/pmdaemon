# Port Management

PMDaemon's advanced port management is one of its most innovative features, going far beyond what traditional process managers offer. This system provides automatic port allocation, conflict detection, and runtime flexibility that makes deploying clustered applications effortless.

## Overview

PMDaemon's port management system includes:

- **Port range distribution** - Automatically assign consecutive ports to cluster instances
- **Auto-assignment** - Find first available port in specified ranges
- **Built-in conflict detection** - Prevent port conflicts at the process manager level
- **Runtime port overrides** - Change ports during restart without modifying saved config
- **Port visibility** - Display assigned ports in process listings
- **Environment variable injection** - Automatic PORT environment variable

## Port Configuration Formats

### Single Port

Assign a specific port to a process:

```bash
# CLI
pmdaemon start "node server.js" --name web-app --port 3000

# Configuration file
{
  "name": "web-app",
  "script": "node",
  "args": ["server.js"],
  "port": "3000"
}
```

### Port Range Distribution

Automatically distribute consecutive ports to cluster instances:

```bash
# CLI - 4 instances get ports 3000, 3001, 3002, 3003
pmdaemon start "node server.js" \
  --name web-cluster \
  --instances 4 \
  --port 3000-3003

# Configuration file
{
  "name": "web-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "port": "3000-3003"
}
```

### Auto-assignment

Find the first available ports in a specified range:

```bash
# CLI - Find 3 available ports between 5000-5100
pmdaemon start "node worker.js" \
  --name workers \
  --instances 3 \
  --port auto:5000-5100

# Configuration file
{
  "name": "workers",
  "script": "node", 
  "args": ["worker.js"],
  "instances": 3,
  "port": "auto:5000-5100"
}
```

## How Port Distribution Works

### Cluster Port Assignment

When you specify a port range for clustered instances:

1. **Parse range** - Extract start and end ports from range string
2. **Validate range** - Ensure range has enough ports for instances
3. **Check availability** - Verify ports are not in use
4. **Assign sequentially** - Assign ports in order to instances
5. **Set environment** - Set PORT environment variable for each instance

```bash
# Example: 4 instances with port range 8000-8003
pmdaemon start "node api.js" --instances 4 --port 8000-8003

# Results in:
# Instance 0: PORT=8000, PM2_INSTANCE_ID=0
# Instance 1: PORT=8001, PM2_INSTANCE_ID=1  
# Instance 2: PORT=8002, PM2_INSTANCE_ID=2
# Instance 3: PORT=8003, PM2_INSTANCE_ID=3
```

### Auto-assignment Algorithm

For auto-assignment (`auto:start-end`):

1. **Scan range** - Check each port in the specified range
2. **Test availability** - Attempt to bind to each port
3. **Collect available** - Build list of available ports
4. **Assign needed** - Take first N available ports for N instances
5. **Handle insufficient** - Error if not enough ports available

```bash
# Example: Need 3 ports from range 5000-5010
pmdaemon start "python worker.py" --instances 3 --port auto:5000-5010

# If ports 5000, 5002, 5005 are available:
# Instance 0: PORT=5000
# Instance 1: PORT=5002  
# Instance 2: PORT=5005
```

## Conflict Detection

PMDaemon includes built-in port conflict detection:

### Process Manager Level

- **Track assigned ports** - Maintain registry of ports assigned to processes
- **Prevent double assignment** - Block assignment of already-used ports
- **Cross-instance awareness** - Detect conflicts across different process groups

### System Level

- **Port availability check** - Test if ports are actually available on the system
- **Bind testing** - Attempt to bind to ports before assignment
- **Error reporting** - Clear error messages for port conflicts

```bash
# Example conflict detection
pmdaemon start "node app1.js" --name app1 --port 3000
pmdaemon start "node app2.js" --name app2 --port 3000
# Error: Port 3000 is already assigned to process 'app1'
```

## Runtime Port Overrides

Change ports during restart without modifying saved configuration:

### Restart with New Port

```bash
# Original configuration uses port 3000
pmdaemon start "node server.js" --name web-app --port 3000

# Restart with different port (doesn't modify saved config)
pmdaemon restart web-app --port 3001

# Configuration file still shows port 3000, but process runs on 3001
```

### Reload with Port Range

```bash
# Original single instance
pmdaemon start "node api.js" --name api --port 8000

# Graceful reload with clustering and port range
pmdaemon reload api --port 4000-4003
# Now running 4 instances on ports 4000-4003
```

### Use Cases for Runtime Overrides

1. **Blue-green deployments** - Switch between port sets
2. **Emergency port changes** - Quick response to port conflicts
3. **Testing different configurations** - Try new port setups without permanent changes
4. **Load balancer updates** - Change ports to match load balancer configuration

## Environment Variable Injection

PMDaemon automatically injects port-related environment variables:

### Standard Variables

- **`PORT`** - The assigned port number
- **`PM2_INSTANCE_ID`** - Instance ID (0-based)
- **`NODE_APP_INSTANCE`** - Instance ID (Node.js compatibility)

### Example Usage in Applications

**Node.js:**
```javascript
const port = process.env.PORT || 3000;
const instanceId = process.env.PM2_INSTANCE_ID || 0;

app.listen(port, () => {
  console.log(`Instance ${instanceId} listening on port ${port}`);
});
```

**Python:**
```python
import os

port = int(os.environ.get('PORT', 8000))
instance_id = os.environ.get('PM2_INSTANCE_ID', '0')

print(f"Instance {instance_id} starting on port {port}")
app.run(host='0.0.0.0', port=port)
```

## Advanced Examples

### Microservices with Different Port Ranges

```json
{
  "apps": [
    {
      "name": "api-gateway",
      "script": "node",
      "args": ["gateway.js"],
      "port": "8080"
    },
    {
      "name": "user-service",
      "script": "node",
      "args": ["user-service.js"],
      "instances": 2,
      "port": "8001-8002"
    },
    {
      "name": "order-service", 
      "script": "python",
      "args": ["-m", "uvicorn", "main:app"],
      "instances": 3,
      "port": "auto:8010-8020"
    }
  ]
}
```

### Development vs Production Ports

**Development:**
```json
{
  "name": "dev-api",
  "script": "npm",
  "args": ["run", "dev"],
  "port": "3000"
}
```

**Production:**
```json
{
  "name": "prod-api",
  "script": "node",
  "args": ["dist/server.js"],
  "instances": 4,
  "port": "auto:3000-3100"
}
```

### Load Balancer Integration

```bash
# Start services with known port ranges for load balancer
pmdaemon start "node api.js" \
  --name api-cluster \
  --instances 4 \
  --port 8000-8003

# Load balancer can be configured to route to 8000-8003
# nginx upstream example:
# upstream api_backend {
#   server 127.0.0.1:8000;
#   server 127.0.0.1:8001;
#   server 127.0.0.1:8002;
#   server 127.0.0.1:8003;
# }
```

## Port Monitoring

### View Assigned Ports

```bash
# List command shows assigned ports
pmdaemon list
```

Output includes port information:
```
┌────┬─────────────┬────────┬───────┬──────┬─────────┬──────────┬─────────┬──────────┐
│ ID │ Name        │ Status │ PID   │ Port │ CPU (%) │ Memory   │ Uptime  │ Restarts │
├────┼─────────────┼────────┼───────┼──────┼─────────┼──────────┼─────────┼──────────┤
│ 0  │ web-cluster │ 🟢 Online │ 1234  │ 3000 │ 2.5     │ 45.2 MB  │ 2h 15m  │ 0        │
│ 1  │ web-cluster │ 🟢 Online │ 1235  │ 3001 │ 1.8     │ 32.1 MB  │ 2h 15m  │ 0        │
│ 2  │ api-service │ 🟢 Online │ 1236  │ 8000 │ 3.2     │ 28.5 MB  │ 1h 30m  │ 0        │
└────┴─────────────┴────────┴───────┴──────┴─────────┴──────────┴─────────┴──────────┘
```

### Process Information

```bash
# Get detailed port information
pmdaemon info web-cluster
```

Shows port configuration and assignment details.

## Best Practices

### 1. Use Port Ranges for Clusters

```bash
# Good: Explicit port range
pmdaemon start "node server.js" --instances 4 --port 3000-3003

# Avoid: No port specification for clusters
pmdaemon start "node server.js" --instances 4  # Ports not managed
```

### 2. Reserve Port Ranges

Plan your port allocation to avoid conflicts:

```bash
# Development: 3000-3999
# Staging: 4000-4999  
# Production: 5000-5999
# Monitoring: 9000-9999
```

### 3. Use Auto-assignment for Dynamic Scaling

```bash
# Good for dynamic environments
pmdaemon start "node worker.js" --instances 3 --port auto:5000-5100

# Better than fixed ranges when scaling up/down
```

### 4. Document Port Assignments

Keep track of port assignments in your documentation:

```yaml
# ports.yaml
services:
  api-gateway: 8080
  user-service: 8001-8002
  order-service: 8010-8020 (auto)
  monitoring: 9615
```

## Troubleshooting

### Common Issues

1. **Port already in use**
   ```bash
   Error: Port 3000 is already in use by another process
   
   # Solution: Use different port or auto-assignment
   pmdaemon start app.js --port auto:3000-3100
   ```

2. **Not enough ports in range**
   ```bash
   Error: Port range 3000-3001 has only 2 ports but 4 instances requested
   
   # Solution: Expand range
   pmdaemon start app.js --instances 4 --port 3000-3003
   ```

3. **Port conflicts between processes**
   ```bash
   Error: Port 8000 is already assigned to process 'api-v1'
   
   # Solution: Use different port or stop conflicting process
   pmdaemon stop api-v1
   pmdaemon start app.js --port 8000
   ```

### Debugging Port Issues

```bash
# Check what's using a port
netstat -tulpn | grep :3000
lsof -i :3000

# Check PMDaemon port assignments
pmdaemon list

# View detailed process information
pmdaemon info process-name
```

## Comparison with Other Process Managers

| Feature | PMDaemon | PM2 | Forever | Supervisor |
|---------|----------|-----|---------|------------|
| Port range distribution | ✅ | ❌ | ❌ | ❌ |
| Auto port assignment | ✅ | ❌ | ❌ | ❌ |
| Runtime port override | ✅ | ❌ | ❌ | ❌ |
| Conflict detection | ✅ | ❌ | ❌ | ❌ |
| Port visibility | ✅ | ❌ | ❌ | ❌ |
| Environment injection | ✅ | ✅ | ❌ | ❌ |

## Next Steps

- **[Health Checks](./health-checks.md)** - Monitor process health
- **[Monitoring](./monitoring.md)** - Real-time process monitoring  
- **[Configuration](./configuration.md)** - Advanced configuration options
- **[Examples](../examples/clustering.md)** - Clustering examples with port management

---

PMDaemon's port management system eliminates the complexity of manual port coordination in clustered deployments, making it easy to scale applications while avoiding conflicts and maintaining clear port visibility.
