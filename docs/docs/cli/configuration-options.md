# Configuration Options

This comprehensive reference covers all CLI configuration options available in PMDaemon. Each option includes its purpose, default value, accepted formats, and practical examples.

## Global Options

These options apply to all PMDaemon commands:

### `--config <path>`
**Description:** Path to configuration file  
**Default:** None  
**Format:** File path (JSON, YAML, or TOML)  
**Example:**
```bash
pmdaemon --config ecosystem.json start
pmdaemon --config /etc/pmdaemon/production.yaml start --name web-api
```

### `--verbose` / `-v`
**Description:** Enable verbose logging  
**Default:** `false`  
**Format:** Boolean flag  
**Example:**
```bash
pmdaemon --verbose start "node server.js" --name web-api
pmdaemon -v list
```

## Process Identification

### `--name <name>`
**Description:** Unique process name identifier  
**Default:** Required for most commands  
**Format:** String (alphanumeric, hyphens, underscores)  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api
pmdaemon start "python worker.py" --name background-worker
```

**Naming conventions:**
- Use descriptive names: `web-api`, `user-service`, `data-processor`
- Avoid spaces and special characters
- Use consistent naming patterns across your infrastructure

## Process Execution

### `--args <arguments>`
**Description:** Command line arguments for the process  
**Default:** Empty array  
**Format:** Space-separated arguments  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --args "--port 3000 --env production"
pmdaemon start "python app.py" --name api --args "-m uvicorn main:app --host 0.0.0.0"
```

### `--cwd <directory>`
**Description:** Working directory for the process  
**Default:** Current directory  
**Format:** Absolute or relative path  
**Example:**
```bash
pmdaemon start "npm start" --name frontend --cwd /var/www/frontend
pmdaemon start "./run.sh" --name service --cwd /opt/myservice
```

### `--instances <count>`
**Description:** Number of process instances (cluster mode)  
**Default:** `1`  
**Format:** Positive integer  
**Example:**
```bash
pmdaemon start "node server.js" --name web-cluster --instances 4
pmdaemon start "python worker.py" --name workers --instances 2
```

**Special values:**
- `max` - Use all available CPU cores
- `0` - Disable the process (useful for configuration files)

## Environment Configuration

### `--env <KEY=VALUE>`
**Description:** Set environment variables  
**Default:** Inherits from parent process  
**Format:** `KEY=VALUE` pairs (can be used multiple times)  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api \
  --env NODE_ENV=production \
  --env DATABASE_URL=postgres://localhost/mydb \
  --env API_KEY=secret123
```

### `--env-file <path>`
**Description:** Load environment variables from file  
**Default:** None  
**Format:** File path (.env format)  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --env-file .env.production
```

**.env file format:**
```bash
NODE_ENV=production
DATABASE_URL=postgres://localhost/mydb
API_KEY=secret123
PORT=3000
```

## Port Management

### `--port <port_config>`
**Description:** Port assignment configuration  
**Default:** No port assigned  
**Format:** Various formats supported  

**Single port:**
```bash
pmdaemon start "node server.js" --name web-api --port 3000
```

**Port range (for clusters):**
```bash
pmdaemon start "node server.js" --name web-cluster --instances 4 --port 3000-3003
```

**Auto-assignment:**
```bash
pmdaemon start "node worker.js" --name workers --instances 3 --port auto:5000-5100
```

**Environment variables set:**
- `PORT` - Assigned port number
- `PM2_INSTANCE_ID` - Instance ID (0-based)
- `NODE_APP_INSTANCE` - Instance ID (Node.js compatibility)

## Resource Limits

### `--max-memory <size>`
**Description:** Memory limit before automatic restart  
**Default:** Unlimited  
**Format:** Size with unit (K, M, G) or raw bytes  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --max-memory 512M
pmdaemon start "python app.py" --name api --max-memory 1G
pmdaemon start "java -jar app.jar" --name java-app --max-memory 2048M
```

**Supported formats:**
- `100K` or `100KB` - Kilobytes
- `512M` or `512MB` - Megabytes
- `2G` or `2GB` - Gigabytes
- `1073741824` - Raw bytes

## Process Control

### `--autorestart <boolean>`
**Description:** Automatically restart process on crash  
**Default:** `true`  
**Format:** `true` or `false`  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --autorestart true
pmdaemon start "python script.py" --name one-time --autorestart false
```

### `--max-restarts <count>`
**Description:** Maximum number of restart attempts  
**Default:** `16`  
**Format:** Positive integer or `unlimited`  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --max-restarts 10
pmdaemon start "python worker.py" --name worker --max-restarts unlimited
```

### `--min-uptime <duration>`
**Description:** Minimum uptime before considering process stable  
**Default:** `1000ms`  
**Format:** Duration with unit (ms, s, m, h)  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --min-uptime 5s
pmdaemon start "java -jar app.jar" --name java-app --min-uptime 30s
```

### `--restart-delay <duration>`
**Description:** Delay between process exit and restart  
**Default:** `0ms`  
**Format:** Duration with unit (ms, s, m, h)  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --restart-delay 1s
pmdaemon start "python worker.py" --name worker --restart-delay 5s
```

### `--kill-timeout <duration>`
**Description:** Time to wait for graceful shutdown before force kill  
**Default:** `1600ms`  
**Format:** Duration with unit (ms, s, m, h)  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --kill-timeout 30s
pmdaemon start "python app.py" --name api --kill-timeout 10s
```

## Logging Configuration

### `--out-file <path>`
**Description:** File path for stdout logs  
**Default:** Auto-generated (`{name}-{instance}-out.log`)  
**Format:** File path  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --out-file /var/log/web-api.out
```

### `--error-file <path>`
**Description:** File path for stderr logs  
**Default:** Auto-generated (`{name}-{instance}-err.log`)  
**Format:** File path  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --error-file /var/log/web-api.err
```

### `--log-file <path>`
**Description:** File path for combined stdout/stderr logs  
**Default:** Auto-generated (`{name}-{instance}.log`)  
**Format:** File path  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --log-file /var/log/web-api.log
```

### `--pid-file <path>`
**Description:** File path for process ID file  
**Default:** Auto-generated (`{name}-{instance}.pid`)  
**Format:** File path  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --pid-file /var/run/web-api.pid
```

## Health Checks

### `--health-check-url <url>`
**Description:** HTTP endpoint for health checks  
**Default:** None  
**Format:** HTTP/HTTPS URL  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api \
  --port 3000 \
  --health-check-url http://localhost:3000/health
```

### `--health-check-script <path>`
**Description:** Script to run for health validation  
**Default:** None  
**Format:** File path to executable script  
**Example:**
```bash
pmdaemon start "python worker.py" --name worker \
  --health-check-script ./scripts/health-check.sh
```

### `--health-check-timeout <duration>`
**Description:** Timeout for individual health checks  
**Default:** `30s`  
**Format:** Duration with unit (s, m, h)  
**Example:**
```bash
pmdaemon start "node api.js" --name api \
  --health-check-url http://localhost:3000/health \
  --health-check-timeout 10s
```

### `--health-check-interval <duration>`
**Description:** Interval between health checks  
**Default:** `60s`  
**Format:** Duration with unit (s, m, h)  
**Example:**
```bash
pmdaemon start "node api.js" --name api \
  --health-check-url http://localhost:3000/health \
  --health-check-interval 30s
```

### `--health-check-retries <count>`
**Description:** Number of retries before marking unhealthy  
**Default:** `3`  
**Format:** Positive integer  
**Example:**
```bash
pmdaemon start "node api.js" --name api \
  --health-check-url http://localhost:3000/health \
  --health-check-retries 5
```

## Blocking Start Options

### `--wait-ready`
**Description:** Block start command until health checks pass  
**Default:** `false`  
**Format:** Boolean flag  
**Example:**
```bash
pmdaemon start "node api.js" --name api \
  --health-check-url http://localhost:3000/health \
  --wait-ready
```

### `--wait-timeout <duration>`
**Description:** Maximum time to wait for process to be ready  
**Default:** `30s`  
**Format:** Duration with unit (s, m, h)  
**Example:**
```bash
pmdaemon start "java -jar app.jar" --name java-app \
  --health-check-url http://localhost:8080/health \
  --wait-ready \
  --wait-timeout 120s
```

## Monitoring Options

### `--interval <duration>`
**Description:** Update interval for monitoring commands  
**Default:** `1s` (for `monit` command)  
**Format:** Duration with unit (s, m, h)  
**Example:**
```bash
pmdaemon monit --interval 5s
pmdaemon monit --interval 30s
```

### `--lines <count>`
**Description:** Number of log lines to display  
**Default:** `20` (for `logs` command)  
**Format:** Positive integer  
**Example:**
```bash
pmdaemon logs web-api --lines 100
pmdaemon logs worker --lines 50
```

### `--follow`
**Description:** Follow logs in real-time  
**Default:** `false` (for `logs` command)  
**Format:** Boolean flag  
**Example:**
```bash
pmdaemon logs web-api --follow
pmdaemon logs worker --follow --lines 0  # Only new logs
```

## Delete Command Options

### `--status`
**Description:** Delete processes by status instead of name  
**Default:** `false`  
**Format:** Boolean flag  
**Example:**
```bash
pmdaemon delete stopped --status
pmdaemon delete errored --status
```

**Valid status values:**
- `starting` - Processes currently starting
- `online` - Running processes
- `stopping` - Processes shutting down
- `stopped` - Stopped processes
- `errored` - Failed processes
- `restarting` - Processes restarting

### `--force` / `-f`
**Description:** Skip confirmation prompts  
**Default:** `false`  
**Format:** Boolean flag  
**Example:**
```bash
pmdaemon delete all --force
pmdaemon delete stopped --status --force
```

## Web Server Options

### `--port <port>` (for `web` command)
**Description:** Port for web monitoring server  
**Default:** `9615`  
**Format:** Port number (1-65535)  
**Example:**
```bash
pmdaemon web --port 8080
pmdaemon web --port 9000
```

### `--host <address>` (for `web` command)
**Description:** Host address to bind web server  
**Default:** `127.0.0.1`  
**Format:** IP address or hostname  
**Example:**
```bash
pmdaemon web --host 0.0.0.0        # All interfaces
pmdaemon web --host 192.168.1.100  # Specific IP
```

## Advanced Options

### `--namespace <name>`
**Description:** Logical grouping for processes  
**Default:** `"default"`  
**Format:** String identifier  
**Example:**
```bash
pmdaemon start "node api.js" --name api --namespace production
pmdaemon start "node worker.js" --name worker --namespace production
pmdaemon list --namespace production
```

### `--exec-mode <mode>`
**Description:** Execution mode for the process  
**Default:** `fork` (auto-detects `cluster` when instances > 1)  
**Format:** `fork` or `cluster`  
**Example:**
```bash
pmdaemon start "node server.js" --name web --exec-mode cluster --instances 4
```

## Configuration File Options

When using configuration files, all CLI options can be specified in the file format:

**JSON format:**
```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "instances": 2,
  "port": "3000-3001",
  "max_memory_restart": "512M",
  "autorestart": true,
  "max_restarts": 10,
  "min_uptime": "5s",
  "restart_delay": "1s",
  "kill_timeout": "30s",
  "env": {
    "NODE_ENV": "production"
  },
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 10,
    "interval": 30,
    "retries": 3
  }
}
```

## Option Precedence

When the same option is specified in multiple places, PMDaemon uses this precedence order:

1. **CLI arguments** (highest priority)
2. **Environment variables**
3. **Configuration file**
4. **Default values** (lowest priority)

**Example:**
```bash
# Configuration file specifies port 3000
# CLI override takes precedence
pmdaemon --config ecosystem.json start --name web-api --port 3001
# Process will use port 3001, not 3000
```

## Validation Rules

PMDaemon validates all configuration options:

- **Required fields:** `name` and `script` are always required
- **Type validation:** Numeric options must be valid numbers
- **Range validation:** Ports must be 1-65535, instances must be positive
- **Format validation:** Memory sizes, durations, and URLs must be valid
- **Conflict detection:** Process names and ports must be unique

## Next Steps

- **[Commands](./commands.md)** - Complete command reference
- **[Environment Variables](./environment-variables.md)** - Environment configuration
- **[Exit Codes](./exit-codes.md)** - Error code reference
- **[Examples](../examples/deployment-examples.md)** - Real-world usage examples
