# PMDaemon Configuration File Usage

PMDaemon now supports ecosystem configuration files in JSON, YAML, and TOML formats, similar to PM2's ecosystem.config.js.

## Quick Start

### 1. Create a Configuration File

Choose any format you prefer:

**ecosystem.json** (JSON format):
```json
{
  "apps": [
    {
      "name": "my-web-app",
      "script": "node",
      "args": ["server.js"],
      "instances": 2,
      "port": "3000-3001",
      "env": {
        "NODE_ENV": "production"
      }
    }
  ]
}
```

**ecosystem.yaml** (YAML format):
```yaml
apps:
  - name: my-web-app
    script: node
    args:
      - server.js
    instances: 2
    port: "3000-3001"
    env:
      NODE_ENV: production
```

**ecosystem.toml** (TOML format):
```toml
[[apps]]
name = "my-web-app"
script = "node"
args = ["server.js"]
instances = 2
port = "3000-3001"

[apps.env]
NODE_ENV = "production"
```

### 2. Use the Configuration File

```bash
# Start all apps from config file
pmdaemon --config ecosystem.json start

# Start specific app from config file
pmdaemon --config ecosystem.json start --name my-web-app

# Works with any format
pmdaemon --config ecosystem.yaml start
pmdaemon --config ecosystem.toml start
```

## Configuration File Structure

All formats support the same configuration options as the CLI:

### Required Fields
- `name` - Unique process name
- `script` - Command or executable to run

### Optional Fields
- `args` - Array of command line arguments
- `instances` - Number of instances (default: 1)
- `port` - Port configuration (single, range, or auto)
- `cwd` - Working directory
- `env` - Environment variables object
- `max_memory_restart` - Memory limit (e.g., "512M", "1G")
- `autorestart` - Auto restart on crash (default: true)
- `max_restarts` - Maximum restart attempts (default: 16)
- `min_uptime` - Minimum uptime before stable (default: 1000ms)
- `restart_delay` - Delay between restarts (default: 0ms)
- `kill_timeout` - Graceful shutdown timeout (default: 1600ms)
- `namespace` - Process namespace (default: "default")
- `out_file` - Output log file path
- `error_file` - Error log file path
- `log_file` - Combined log file path
- `pid_file` - PID file path

## Usage Examples

### Start All Apps
```bash
# Start all apps defined in the config file
pmdaemon --config ecosystem.json start
```

### Start Specific App
```bash
# Start only the "web-server" app from the config
pmdaemon --config ecosystem.json start --name web-server
```

### Mixed Usage
```bash
# You can still use CLI for individual processes
pmdaemon start node app.js --name standalone-app

# And config files for complex setups
pmdaemon --config production.json start
```

## Error Handling

PMDaemon provides detailed error messages for configuration issues:

### File Not Found
```bash
$ pmdaemon --config missing.json start
Error: Failed to read config file 'missing.json': No such file or directory
```

### Invalid JSON/YAML/TOML
```bash
$ pmdaemon --config bad.json start
Error: Failed to parse JSON config file 'bad.json': expected `,` or `}` at line 5 column 3
```

### Missing Required Fields
```bash
$ pmdaemon --config incomplete.json start
Error: App 0 validation failed: Process name cannot be empty
```

### Duplicate App Names
```bash
$ pmdaemon --config duplicate.json start
Error: Duplicate app name: 'web-server'
```

### App Not Found
```bash
$ pmdaemon --config ecosystem.json start --name nonexistent
Error: App 'nonexistent' not found in config file. Available apps: web-server, api-service, worker
```

## Best Practices

### 1. Environment-Specific Configs
Create separate config files for different environments:
```bash
pmdaemon --config development.json start
pmdaemon --config staging.yaml start
pmdaemon --config production.toml start
```

### 2. Validation Before Deployment
Test your config files locally before deploying:
```bash
# This will validate the config and show any errors
pmdaemon --config ecosystem.json start --name test-app
```

### 3. Version Control
Keep your config files in version control alongside your application code.

### 4. Documentation
Document your apps within the config files using comments (YAML/TOML) or descriptive names.

## Format Comparison

| Feature | JSON | YAML | TOML |
|---------|------|------|------|
| Comments | ❌ | ✅ | ✅ |
| Readability | Good | Excellent | Good |
| Parsing Speed | Fast | Medium | Medium |
| Ecosystem Support | Excellent | Excellent | Good |

Choose the format that best fits your team's preferences and existing tooling.

## Migration from PM2

If you're migrating from PM2, you can convert your `ecosystem.config.js`:

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

Note: PMDaemon doesn't support environment-specific configs in a single file yet. Use separate config files for different environments.

## Troubleshooting

### Common Issues

1. **Script not found**: Ensure the script path is correct and executable
2. **Port conflicts**: Check that port ranges don't overlap between apps
3. **Permission errors**: Ensure PMDaemon has permission to read the config file
4. **Working directory**: Use absolute paths or ensure relative paths are correct

### Debug Mode
Use verbose logging to troubleshoot issues:
```bash
pmdaemon --verbose --config ecosystem.json start
```

This implementation provides full ecosystem configuration support while maintaining backward compatibility with existing CLI usage.
