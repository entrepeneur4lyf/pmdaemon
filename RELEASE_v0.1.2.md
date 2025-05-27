# PMDaemon v0.1.2 - Ecosystem Configuration Files & Multi-App Management 🚀📁

**Release Date:** May 27, 2025

We are excited to announce PMDaemon v0.1.2 - a major productivity update that introduces **Ecosystem Configuration File Support**, enabling seamless management of multiple applications through JSON, YAML, and TOML configuration files. This release brings PMDaemon closer to PM2's ecosystem functionality while maintaining its superior port management and monitoring capabilities.

## 🎉 What's New in v0.1.2

This release focuses on developer productivity and deployment automation by introducing comprehensive ecosystem configuration support. PMDaemon now allows you to define and manage complex multi-application setups through simple configuration files, making it ideal for microservices, development environments, and production deployments.

## ✨ New Features in v0.1.2

### 📁 Ecosystem Configuration Files
- **Multi-Format Support** - JSON, YAML, and TOML configuration files
  ```bash
  pmdaemon --config ecosystem.json start
  pmdaemon --config ecosystem.yaml start
  pmdaemon --config ecosystem.toml start
  ```
- **Full Feature Parity** - All CLI options available in config files
- **App-Specific Targeting** - Start specific applications from config files
  ```bash
  pmdaemon --config ecosystem.json start --name web-server
  ```

### 🎯 Advanced Configuration Management
- **Comprehensive Field Support** - All process options configurable via files:
  - Process lifecycle (name, script, args, instances)
  - Port management (single, range, auto-assignment)
  - Environment variables and working directories
  - Memory limits and restart policies
  - Health checks and monitoring settings
  - Log file paths and PID management
- **Environment-Specific Configs** - Separate config files for different environments
- **Validation & Error Handling** - Detailed error messages for configuration issues

### 🔧 Enhanced CLI Integration
- **Global Config Flag** - `--config <file>` works with all commands
- **Backward Compatibility** - Existing CLI workflows unchanged
- **Mixed Usage** - Combine config files with individual CLI commands
- **Comprehensive Help** - Detailed configuration documentation and examples

### 📋 Configuration File Structure
**ecosystem.json example:**
```json
{
  "apps": [
    {
      "name": "web-server",
      "script": "node",
      "args": ["server.js"],
      "instances": 2,
      "port": "3000-3001",
      "env": {
        "NODE_ENV": "production"
      },
      "health_check_url": "http://localhost:3000/health",
      "max_memory_restart": "512M"
    },
    {
      "name": "api-service",
      "script": "python",
      "args": ["api.py"],
      "cwd": "/path/to/api",
      "autorestart": true,
      "max_restarts": 10
    }
  ]
}
```

**ecosystem.yaml example:**
```yaml
apps:
  - name: worker-queue
    script: node
    args: [worker.js]
    instances: 4
    port: auto:4000-4100
    env:
      REDIS_URL: redis://localhost:6379
      NODE_ENV: production
    health_check_script: ./scripts/health-check.sh
    wait_ready: true
```

### 🛡️ Robust Error Handling & Validation
- **File Format Validation** - Clear error messages for malformed configs
- **Required Field Validation** - Immediate feedback on missing required fields
- **Duplicate Detection** - Prevent duplicate app names across configurations
- **App Resolution** - Helpful suggestions when targeting non-existent apps
- **Schema Validation** - Comprehensive validation of all configuration fields

## 🔄 All Previous Features (v0.1.0 & v0.1.1)

### Enhanced Delete Operations (v0.1.1)
- **Bulk deletion** with `delete all` command and safety confirmations
- **Status-based deletion** (`delete <status> --status`) with force flags
- **Safe process shutdown** with proper lifecycle management

### Health Checks & Monitoring (v0.1.1)
- **HTTP Health Checks** with configurable endpoints and timeouts
- **Script-based Health Checks** for custom validation logic
- **Blocking Start Command** with `--wait-ready` for deployment scripts
- **Real-time health monitoring** integrated with web API and WebSocket

### Core Process Management (v0.1.0)
- **Complete lifecycle management** - Start, stop, restart, reload, delete
- **Advanced clustering** with automatic load balancing
- **Auto-restart mechanisms** with configurable limits and strategies
- **Graceful shutdown** with proper signal handling

### 🌟 Innovative Port Management (Beyond PM2)
- **Port range distribution** for clusters (`--port 3000-3003`)
- **Auto-assignment** from ranges (`--port auto:5000-5100`)
- **Built-in conflict detection** and runtime port overrides
- **Port visibility** in process listings and monitoring

### Advanced Monitoring & Web API
- **Real-time monitoring** with configurable intervals
- **Memory limit enforcement** with automatic restart capabilities
- **REST API** with PM2-compatible responses
- **WebSocket support** for real-time updates and live monitoring

## 📊 Project Stats & Quality

- **267 total tests** (comprehensive ecosystem config coverage)
  - Configuration parsing and validation tests
  - Multi-format file support testing
  - Error handling and edge case coverage
  - End-to-end ecosystem workflow tests
- **9 completed development phases** (including ecosystem configuration)
- **100% feature coverage** with advanced multi-app management
- **Production-ready** stability with comprehensive configuration support

## 🆚 Comparison with PM2

PMDaemon v0.1.2 now matches and exceeds PM2's ecosystem capabilities:

| Feature                 | PMDaemon v0.1.2 | PM2 |
|-------------------------|:---------------:|:---:|
| Ecosystem config files | ✅              | ✅  |
| Multiple config formats| ✅ (JSON/YAML/TOML) | ❌ (JS only) |
| Port range distribution | ✅              | ❌  |
| Auto port assignment   | ✅              | ❌  |
| Built-in health checks | ✅              | ❌  |
| Blocking start command | ✅              | ❌  |
| Status-based deletion  | ✅              | ❌  |
| Real-time WebSocket API| ✅              | ❌  |
| Memory limit enforcement| ✅              | ✅  |
| Configuration validation| ✅              | ❌  |

## 🚀 Getting Started with Ecosystem Configs

### 1. Create your ecosystem file
```bash
# Create ecosystem.json
cat > ecosystem.json << 'EOF'
{
  "apps": [
    {
      "name": "web-app",
      "script": "node",
      "args": ["server.js"],
      "instances": 2,
      "port": "3000-3001"
    }
  ]
}
EOF
```

### 2. Start your applications
```bash
# Start all apps from config
pmdaemon --config ecosystem.json start

# Start specific app
pmdaemon --config ecosystem.json start --name web-app
```

### 3. Monitor and manage
```bash
# Standard commands work with config-started apps
pmdaemon list
pmdaemon monit
pmdaemon logs web-app
```

## 📚 Documentation & Examples

- **[CONFIG_USAGE.md](examples/CONFIG_USAGE.md)** - Comprehensive configuration guide
- **[Example configs](examples/)** - JSON, YAML, and TOML examples
- **[Schema validation](examples/SCHEMA_VALIDATION.md)** - Configuration validation guide

## 🛣️ Migration from PM2

PMDaemon v0.1.2 makes migration from PM2 straightforward:

**PM2 ecosystem.config.js:**
```javascript
module.exports = {
  apps: [{
    name: 'my-app',
    script: 'server.js',
    instances: 4,
    env: {
      NODE_ENV: 'development'
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
      "NODE_ENV": "development"
    }
  }]
}
```

## 🔮 What's Next

PMDaemon v0.1.2 establishes a solid foundation for ecosystem management. Future releases will focus on:
- **Advanced deployment features** - Blue-green deployments, rolling updates
- **Container integration** - Docker and Kubernetes support
- **Enhanced monitoring** - Metrics collection and alerting
- **Clustering improvements** - Advanced load balancing strategies

## 🙏 Community & Contributing

PMDaemon continues to grow with community input. This ecosystem configuration feature was driven by user requests for better multi-application management.

- **GitHub**: [https://github.com/entrepeneur4lyf/pmdaemon](https://github.com/entrepeneur4lyf/pmdaemon)
- **Documentation**: [https://entrepeneur4lyf.github.io/pmdaemon](https://entrepeneur4lyf.github.io/pmdaemon)
- **Issues & Feature Requests**: [GitHub Issues](https://github.com/entrepeneur4lyf/pmdaemon/issues)

---

**PMDaemon v0.1.2** - *Ecosystem Configuration Files & Multi-App Management*

*Bringing enterprise-grade process management to modern development workflows.*
