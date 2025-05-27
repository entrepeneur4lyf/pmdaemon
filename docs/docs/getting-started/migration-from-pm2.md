# Migration from PM2

Migrating from PM2 to PMDaemon is straightforward thanks to PMDaemon's PM2-compatible command interface. This guide helps you transition smoothly while taking advantage of PMDaemon's enhanced features.

## Command Compatibility

PMDaemon provides a **PM2-compatible CLI**, so most of your existing commands work unchanged:

### Direct Command Mapping

| PM2 Command | PMDaemon Command | Status |
|-------------|------------------|---------|
| `pm2 start app.js` | `pmdaemon start app.js` | ✅ Compatible |
| `pm2 stop app` | `pmdaemon stop app` | ✅ Compatible |
| `pm2 restart app` | `pmdaemon restart app` | ✅ Compatible |
| `pm2 reload app` | `pmdaemon reload app` | ✅ Compatible |
| `pm2 delete app` | `pmdaemon delete app` | ✅ Enhanced |
| `pm2 list` | `pmdaemon list` | ✅ Enhanced |
| `pm2 monit` | `pmdaemon monit` | ✅ Enhanced |
| `pm2 logs app` | `pmdaemon logs app` | ✅ Enhanced |
| `pm2 describe app` | `pmdaemon info app` | ✅ Compatible |

### Enhanced Commands

PMDaemon extends PM2 commands with additional features:

```bash
# PM2 style (works in PMDaemon)
pmdaemon start app.js --name myapp --instances 4

# PMDaemon enhancements
pmdaemon start app.js --name myapp --instances 4 \
  --port 3000-3003 \                    # Port range distribution
  --health-check-url http://localhost:3000/health \  # Health checks
  --wait-ready                           # Blocking start
```

## Configuration Migration

### PM2 Ecosystem Files

**PM2 ecosystem.config.js:**
```javascript
module.exports = {
  apps: [{
    name: 'my-app',
    script: 'server.js',
    instances: 4,
    env: {
      NODE_ENV: 'development'
    },
    env_production: {
      NODE_ENV: 'production'
    }
  }]
};
```

**PMDaemon ecosystem.json:**
```json
{
  "apps": [{
    "name": "my-app",
    "script": "node",
    "args": ["server.js"],
    "instances": 4,
    "env": {
      "NODE_ENV": "production"
    }
  }]
}
```

### Key Differences

1. **Script field**: PMDaemon separates the executable from arguments
   - PM2: `"script": "server.js"`
   - PMDaemon: `"script": "node", "args": ["server.js"]`

2. **Environment configs**: PMDaemon uses separate files for different environments
   - PM2: `env_production` in same file
   - PMDaemon: Separate `production.json`, `development.json` files

3. **Port configuration**: PMDaemon uses strings for advanced port features
   - PM2: `"port": 3000`
   - PMDaemon: `"port": "3000"` or `"port": "3000-3003"`

## Migration Steps

### Step 1: Install PMDaemon

```bash
cargo install pmdaemon
```

### Step 2: Stop PM2 Processes

```bash
# List current PM2 processes
pm2 list

# Stop all PM2 processes
pm2 stop all

# Save PM2 configuration for reference
pm2 save
```

### Step 3: Convert Configuration

Use this script to convert PM2 ecosystem files:

```bash
#!/bin/bash
# convert-pm2-config.sh

# Check if ecosystem.config.js exists
if [ ! -f "./ecosystem.config.js" ]; then
  echo "Error: ecosystem.config.js not found in current directory"
  exit 1
fi

# Convert PM2 ecosystem.config.js to PMDaemon ecosystem.json
node -e "
try {
const pm2Config = require('./ecosystem.config.js');
if (!pm2Config.apps || !Array.isArray(pm2Config.apps)) {
  throw new Error('Invalid ecosystem.config.js: apps array not found');
}
const pmdConfig = {
  apps: pm2Config.apps.map(app => ({
    name: app.name,
    script: app.interpreter || 'node',
    args: [app.script, ...(app.args || [])],
    instances: app.instances || 1,
    port: app.env?.PORT ? String(app.env.PORT) : undefined,
    max_memory_restart: app.max_memory_restart,
    cwd: app.cwd,
    env: app.env_production || app.env || {},
    autorestart: app.autorestart !== false,
    max_restarts: app.max_restarts || 16
  }))
};
console.log(JSON.stringify(pmdConfig, null, 2));
} catch (error) {
  console.error('Conversion failed:', error.message);
  process.exit(1);
}
" > ecosystem.json

### Step 4: Start with PMDaemon

```bash
# Start all apps from converted config
pmdaemon --config ecosystem.json start

# Or start individual processes
pmdaemon start "node server.js" --name my-app --instances 4
```

### Step 5: Verify Migration

```bash
# Check processes are running
pmdaemon list

# Compare with PM2 output
pm2 list
```

## Feature Comparison

### What PMDaemon Has That PM2 Doesn't

| Feature | PMDaemon | PM2 | Description |
|---------|----------|-----|-------------|
| **Port Range Distribution** | ✅ | ❌ | `--port 3000-3003` distributes consecutive ports |
| **Auto Port Assignment** | ✅ | ❌ | `--port auto:5000-5100` finds available ports |
| **Runtime Port Override** | ✅ | ❌ | Change ports during restart without config changes |
| **HTTP Health Checks** | ✅ | ❌ | Monitor endpoints with configurable timeouts |
| **Script Health Checks** | ✅ | ❌ | Custom health validation scripts |
| **Blocking Start** | ✅ | ❌ | Wait for processes to be ready (`--wait-ready`) |
| **Bulk Delete Operations** | ✅ | ❌ | `delete all`, `delete stopped --status` |
| **Configurable Monitor Intervals** | ✅ | ❌ | `monit --interval 5` for custom refresh rates |
| **Real-time Log Following** | ✅ | ❌ | `logs --follow` with proper `tail -f` behavior |
| **Professional Table Display** | ✅ | ❌ | Color-coded status, PID column, beautiful formatting |
| **WebSocket Real-time Updates** | ✅ | ❌ | Live process status via WebSocket |

### What's Different

| Feature | PM2 | PMDaemon | Notes |
|---------|-----|----------|-------|
| **Ecosystem Files** | JavaScript | JSON/YAML/TOML | PMDaemon supports multiple formats |
| **Environment Configs** | Single file | Separate files | Use different files for dev/prod |
| **Port Configuration** | Numbers | Strings | PMDaemon: `"3000"`, `"3000-3003"`, `"auto:5000-5100"` |
| **Memory Limits** | Various formats | Standardized | PMDaemon: `"512M"`, `"1G"`, `"100K"` |
| **Log Management** | Built-in rotation | External tools | Use `logrotate` or similar |

## Common Migration Issues

### 1. Port Configuration

**Problem**: PM2 uses numbers, PMDaemon uses strings
```javascript
// PM2
{ port: 3000 }

// PMDaemon
{ "port": "3000" }
```

**Solution**: Convert port numbers to strings in your config files.

### 2. Script vs Command

**Problem**: PM2 combines script and interpreter
```javascript
// PM2
{ script: "server.js" }

// PMDaemon
{ "script": "node", "args": ["server.js"] }
```

**Solution**: Separate the interpreter from the script file.

### 3. Environment-specific Configs

**Problem**: PM2 supports `env_production` in same file
```javascript
// PM2
{
  env: { NODE_ENV: 'development' },
  env_production: { NODE_ENV: 'production' }
}
```

**Solution**: Create separate config files:
```bash
# development.json
{ "apps": [{ "env": { "NODE_ENV": "development" } }] }

# production.json  
{ "apps": [{ "env": { "NODE_ENV": "production" } }] }
```

### 4. Memory Format

**Problem**: PM2 accepts various memory formats
```javascript
// PM2 (various formats)
{ max_memory_restart: "1024M" }
{ max_memory_restart: "1G" }
```

**Solution**: Use PMDaemon's standardized format:
```json
{ "max_memory_restart": "1G" }
```

## Migration Checklist

- [ ] **Install PMDaemon**: `cargo install pmdaemon`
- [ ] **Backup PM2 config**: `pm2 save`
- [ ] **Stop PM2 processes**: `pm2 stop all`
- [ ] **Convert ecosystem files**: Use conversion script or manual conversion
- [ ] **Update port configurations**: Change numbers to strings
- [ ] **Separate environment configs**: Create dev/prod specific files
- [ ] **Test PMDaemon startup**: `pmdaemon --config ecosystem.json start`
- [ ] **Verify process status**: `pmdaemon list`
- [ ] **Test health checks**: Add health check configurations
- [ ] **Update deployment scripts**: Replace `pm2` commands with `pmdaemon`
- [ ] **Configure monitoring**: Set up `pmdaemon monit` or web API
- [ ] **Update documentation**: Update team docs with new commands

## Taking Advantage of New Features

Once migrated, enhance your setup with PMDaemon's unique features:

### 1. Add Port Management
```bash
# Before (PM2)
pm2 start server.js --instances 4

# After (PMDaemon with port ranges)
pmdaemon start server.js --instances 4 --port 3000-3003
```

### 2. Add Health Checks
```json
{
  "apps": [{
    "name": "api-server",
    "script": "node",
    "args": ["server.js"],
    "health_check": {
      "check_type": "http",
      "url": "http://localhost:3000/health",
      "timeout": 5,
      "interval": 30,
      "retries": 3
    }
  }]
}
```

### 3. Use Blocking Start in Deployment
```bash
# Wait for service to be ready before continuing
pmdaemon start server.js --health-check-url http://localhost:3000/health --wait-ready
echo "Service is ready for traffic!"
```

### 4. Enhanced Monitoring
```bash
# Real-time monitoring with custom intervals
pmdaemon monit --interval 2

# Follow logs in real-time
pmdaemon logs api-server --follow
```

## Getting Help

- **[Troubleshooting Guide](../advanced/troubleshooting.md)** - Common issues and solutions
- **[GitHub Issues](https://github.com/entrepeneur4lyf/pmdaemon/issues)** - Report migration problems
- **[Examples](../examples/use-cases.md)** - See real-world migration examples

---

**Welcome to PMDaemon!** 🎉 You now have access to advanced features that go beyond what PM2 offers, while maintaining the familiar interface you're used to.
