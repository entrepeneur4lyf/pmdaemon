# Environment Variables

PMDaemon uses environment variables for system-level configuration and automatically injects process-specific variables into managed processes. This reference covers both PMDaemon's own environment variables and those it provides to your applications.

## PMDaemon System Variables

These environment variables configure PMDaemon's behavior:

### `PMDAEMON_CONFIG_DIR`
**Description:** Directory for PMDaemon configuration files
**Default:** `~/.pmdaemon` (Linux/macOS), `%APPDATA%\pmdaemon` (Windows)
**Example:**
```bash
export PMDAEMON_CONFIG_DIR=/etc/pmdaemon
pmdaemon start "node server.js" --name web-api
```

### `PMDAEMON_LOG_DIR`
**Description:** Directory for PMDaemon log files
**Default:** `~/.pmdaemon/logs`
**Example:**
```bash
export PMDAEMON_LOG_DIR=/var/log/pmdaemon
pmdaemon start "node server.js" --name web-api
```

### `PMDAEMON_PID_DIR`
**Description:** Directory for process PID files
**Default:** `~/.pmdaemon/pids`
**Example:**
```bash
export PMDAEMON_PID_DIR=/var/run/pmdaemon
pmdaemon start "node server.js" --name web-api
```

### `PMDAEMON_LOG_LEVEL`
**Description:** Logging level for PMDaemon
**Default:** `info`
**Values:** `error`, `warn`, `info`, `debug`, `trace`
**Example:**
```bash
export PMDAEMON_LOG_LEVEL=debug
pmdaemon start "node server.js" --name web-api
```

### `PMDAEMON_WEB_PORT`
**Description:** Default port for web monitoring server
**Default:** `9615`
**Example:**
```bash
export PMDAEMON_WEB_PORT=8080
pmdaemon web  # Starts on port 8080
```

### `PMDAEMON_WEB_HOST`
**Description:** Default host for web monitoring server
**Default:** `127.0.0.1`
**Example:**
```bash
export PMDAEMON_WEB_HOST=0.0.0.0
pmdaemon web  # Binds to all interfaces
```

## Process Default Variables

Set default values for process configuration:

### `PMDAEMON_DEFAULT_INSTANCES`
**Description:** Default number of instances for new processes
**Default:** `1`
**Example:**
```bash
export PMDAEMON_DEFAULT_INSTANCES=2
pmdaemon start "node server.js" --name web-api  # Starts 2 instances
```

### `PMDAEMON_DEFAULT_MAX_MEMORY`
**Description:** Default memory limit for new processes
**Default:** None (unlimited)
**Example:**
```bash
export PMDAEMON_DEFAULT_MAX_MEMORY=512M
pmdaemon start "node server.js" --name web-api  # 512MB limit
```

### `PMDAEMON_DEFAULT_AUTORESTART`
**Description:** Default auto-restart setting
**Default:** `true`
**Example:**
```bash
export PMDAEMON_DEFAULT_AUTORESTART=false
pmdaemon start "python script.py" --name one-time  # No auto-restart
```

### `PMDAEMON_DEFAULT_MAX_RESTARTS`
**Description:** Default maximum restart attempts
**Default:** `16`
**Example:**
```bash
export PMDAEMON_DEFAULT_MAX_RESTARTS=5
pmdaemon start "node server.js" --name web-api  # Max 5 restarts
```

## Process-Injected Variables

PMDaemon automatically injects these variables into managed processes:

### Core Process Variables

#### `PORT`
**Description:** Assigned port number for the process
**Set when:** Process has port configuration
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --port 3000
# Process receives: PORT=3000
```

**Application usage:**
```javascript
// Node.js
const port = process.env.PORT || 3000;
app.listen(port);
```

```python
# Python
import os
port = int(os.environ.get('PORT', 8000))
app.run(port=port)
```

#### `PM2_INSTANCE_ID`
**Description:** Zero-based instance identifier for clustering
**Set when:** Process runs in cluster mode (instances > 1)
**Example:**
```bash
pmdaemon start "node server.js" --name web-cluster --instances 3 --port 3000-3002
# Instance 0: PM2_INSTANCE_ID=0, PORT=3000
# Instance 1: PM2_INSTANCE_ID=1, PORT=3001
# Instance 2: PM2_INSTANCE_ID=2, PORT=3002
```

**Application usage:**
```javascript
// Node.js - Instance-specific behavior
const instanceId = parseInt(process.env.PM2_INSTANCE_ID || '0');
console.log(`Worker ${instanceId} starting...`);

// Instance-specific configuration
if (instanceId === 0) {
  // Master instance - handle cron jobs
  startCronJobs();
}
```

#### `NODE_APP_INSTANCE`
**Description:** Instance identifier for Node.js compatibility
**Set when:** Process runs in cluster mode
**Value:** Same as `PM2_INSTANCE_ID`
**Example:**
```bash
pmdaemon start "node server.js" --name web-cluster --instances 2
# Instance 0: NODE_APP_INSTANCE=0
# Instance 1: NODE_APP_INSTANCE=1
```

### Process Metadata Variables

#### `PMDAEMON_PROCESS_NAME`
**Description:** Process name as defined in PMDaemon
**Always set:** Yes
**Example:**
```bash
pmdaemon start "node server.js" --name web-api
# Process receives: PMDAEMON_PROCESS_NAME=web-api
```

#### `PMDAEMON_PROCESS_ID`
**Description:** PMDaemon internal process ID
**Always set:** Yes
**Example:**
```bash
# Process receives: PMDAEMON_PROCESS_ID=0
```

#### `PMDAEMON_NAMESPACE`
**Description:** Process namespace
**Set when:** Namespace is specified
**Example:**
```bash
pmdaemon start "node server.js" --name web-api --namespace production
# Process receives: PMDAEMON_NAMESPACE=production
```

#### `PMDAEMON_VERSION`
**Description:** PMDaemon version managing the process
**Always set:** Yes
**Example:**
```bash
# Process receives: PMDAEMON_VERSION=0.1.1
```

## Custom Environment Variables

### Setting via CLI

```bash
# Single variable
pmdaemon start "node server.js" --name web-api --env NODE_ENV=production

# Multiple variables
pmdaemon start "node server.js" --name web-api \
  --env NODE_ENV=production \
  --env DATABASE_URL=postgres://localhost/mydb \
  --env API_KEY=secret123
```

### Setting via Environment File

Create a `.env` file:
```bash
# .env.production
NODE_ENV=production
DATABASE_URL=postgres://localhost/mydb
API_KEY=secret123
PORT=3000
DEBUG=app:*
```

Use with PMDaemon:
```bash
pmdaemon start "node server.js" --name web-api --env-file .env.production
```

### Setting via Configuration File

**JSON format:**
```json
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "env": {
    "NODE_ENV": "production",
    "DATABASE_URL": "postgres://localhost/mydb",
    "API_KEY": "secret123"
  }
}
```

**YAML format:**
```yaml
name: web-api
script: node
args: [server.js]
env:
  NODE_ENV: production
  DATABASE_URL: postgres://localhost/mydb
  API_KEY: secret123
```

## Environment Variable Precedence

When the same variable is defined in multiple places, PMDaemon uses this precedence:

1. **CLI `--env` flags** (highest priority)
2. **Environment file (`--env-file`)**
3. **Configuration file `env` section**
4. **PMDaemon-injected variables**
5. **Parent process environment** (lowest priority)

**Example:**
```bash
# Parent process has NODE_ENV=development
# Configuration file has NODE_ENV=staging
# CLI override takes precedence
pmdaemon start "node server.js" --name web-api --env NODE_ENV=production
# Process receives: NODE_ENV=production
```

## Environment Variable Patterns

### Development vs Production

**Development:**
```bash
# .env.development
NODE_ENV=development
DEBUG=*
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost/mydb_dev
```

**Production:**
```bash
# .env.production
NODE_ENV=production
DEBUG=app:error
LOG_LEVEL=info
DATABASE_URL=postgres://prod-server/mydb
```

### Instance-Specific Configuration

```javascript
// Node.js - Different behavior per instance
const instanceId = parseInt(process.env.PM2_INSTANCE_ID || '0');

// Instance 0 handles background tasks
if (instanceId === 0) {
  require('./background-tasks');
}

// All instances handle web requests
require('./web-server');
```

### Database Connection Pooling

```javascript
// Node.js - Scale connection pool with instances
const instanceId = parseInt(process.env.PM2_INSTANCE_ID || '0');
// Total instances can be tracked in your application logic
// Total instances can be tracked in your application logic
const totalInstances = process.env.INSTANCES || 1;

const poolSize = Math.ceil(100 / totalInstances); // Distribute 100 connections
const dbConfig = {
  host: process.env.DATABASE_HOST,
  database: process.env.DATABASE_NAME,
  pool: {
    min: 1,
    max: poolSize
  }
};
```

## Security Considerations

### Sensitive Variables

```bash
# Good: Use environment files for secrets
echo "API_KEY=secret123" > .env.production
chmod 600 .env.production
pmdaemon start "node api.js" --name api --env-file .env.production

# Avoid: Secrets in CLI (visible in process list)
pmdaemon start "node api.js" --name api --env API_KEY=secret123
```

### Variable Validation

```javascript
// Node.js - Validate required environment variables
const requiredVars = ['DATABASE_URL', 'API_KEY', 'JWT_SECRET'];
const missing = requiredVars.filter(varName => !process.env[varName]);

if (missing.length > 0) {
  console.error(`Missing required environment variables: ${missing.join(', ')}`);
  process.exit(1);
}
```

### Environment Isolation

```bash
# Use namespaces to isolate environments
pmdaemon start "node api.js" --name api --namespace production --env-file .env.prod
pmdaemon start "node api.js" --name api --namespace staging --env-file .env.staging
```

## Debugging Environment Variables

### View Process Environment

```bash
# View all environment variables for a process
pmdaemon info web-api

# View specific process details
pmdaemon describe web-api
```

### Log Environment Variables

```javascript
// Node.js - Log environment on startup
console.log('Environment variables:');
console.log('NODE_ENV:', process.env.NODE_ENV);
console.log('PORT:', process.env.PORT);
console.log('PM2_INSTANCE_ID:', process.env.PM2_INSTANCE_ID);
console.log('PMDAEMON_PROCESS_NAME:', process.env.PMDAEMON_PROCESS_NAME);
```

### Environment Variable Conflicts

```bash
# Check for conflicts
env | grep NODE_ENV  # Check system environment
pmdaemon info web-api | grep -A 20 "Environment"  # Check process environment
```

## Best Practices

### 1. Use Environment Files

```bash
# Good: Organized environment files
pmdaemon start "node api.js" --name api --env-file .env.production

# Avoid: Long CLI commands
pmdaemon start "node api.js" --name api --env VAR1=value1 --env VAR2=value2 --env VAR3=value3
```

### 2. Validate Required Variables

```javascript
// Good: Validate on startup
const config = {
  port: process.env.PORT || 3000,
  dbUrl: process.env.DATABASE_URL || (() => {
    throw new Error('DATABASE_URL is required');
  })(),
  apiKey: process.env.API_KEY || (() => {
    throw new Error('API_KEY is required');
  })()
};
```

### 3. Use Instance-Specific Logic

```javascript
// Good: Instance-aware code
const instanceId = parseInt(process.env.PM2_INSTANCE_ID || '0');

// Only instance 0 runs scheduled tasks
if (instanceId === 0) {
  startScheduledTasks();
}
```

### 4. Secure Sensitive Data

```bash
# Good: Secure file permissions
chmod 600 .env.production
pmdaemon start "node api.js" --name api --env-file .env.production

# Good: Use external secret management
pmdaemon start "node api.js" --name api --env DATABASE_URL="$(vault kv get -field=url secret/db)"
```

## Next Steps

- **[Configuration Options](./configuration-options.md)** - Complete CLI options
- **[Exit Codes](./exit-codes.md)** - Error code reference
- **[Security](../advanced/security.md)** - Security best practices
- **[Examples](../examples/deployment-examples.md)** - Real-world examples
