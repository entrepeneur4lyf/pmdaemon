# Environment-Specific Configuration

PMDaemon supports environment-specific configurations to manage different deployment stages (development, staging, production) with tailored settings for each environment.

## Environment Configuration Strategies

### 1. Multiple Config Files

Create separate ecosystem files for each environment:

```bash
ecosystem.dev.json      # Development
ecosystem.staging.json  # Staging
ecosystem.prod.json     # Production
```

**Development (ecosystem.dev.json):**
```json
{
  "apps": [
    {
      "name": "web-app-dev",
      "script": "npm",
      "args": ["run", "dev"],
      "cwd": "/app",
      "env": {
        "NODE_ENV": "development",
        "PORT": "3000",
        "DEBUG": "*",
        "DB_URL": "mongodb://localhost:27017/myapp_dev"
      },
      "instances": 1,
      "watch": true,
      "ignore_watch": ["node_modules", "logs"],
      "max_restarts": 10,
      "min_uptime": "1s"
    }
  ]
}
```

**Production (ecosystem.prod.json):**
```json
{
  "apps": [
    {
      "name": "web-app-prod",
      "script": "dist/server.js",
      "cwd": "/app",
      "env": {
        "NODE_ENV": "production",
        "PORT": "3000"
      },
      "instances": "max",
      "exec_mode": "cluster",
      "watch": false,
      "max_restarts": 3,
      "min_uptime": "10s",
      "max_memory_restart": "1G",
      "health_check": {
        "enabled": true,
        "url": "http://localhost:3000/health",
        "interval": 30,
        "timeout": 10,
        "retries": 3
      }
    }
  ]
}
```

### 2. Environment Variables Override

Use a single config file with environment variable substitution:

```json
{
  "apps": [
    {
      "name": "web-app",
      "script": "${APP_SCRIPT:-server.js}",
      "instances": "${APP_INSTANCES:-1}",
      "env": {
        "NODE_ENV": "${NODE_ENV:-development}",
        "PORT": "${PORT:-3000}",
        "DB_URL": "${DATABASE_URL}",
        "REDIS_URL": "${REDIS_URL}"
      },
      "watch": "${WATCH_FILES:-false}",
      "max_memory_restart": "${MAX_MEMORY:-500M}"
    }
  ]
}
```

**Development (.env.dev):**
```bash
NODE_ENV=development
APP_SCRIPT=npm run dev
APP_INSTANCES=1
PORT=3000
WATCH_FILES=true
MAX_MEMORY=1G
DATABASE_URL=mongodb://localhost:27017/myapp_dev
REDIS_URL=redis://localhost:6379
```

**Production (.env.prod):**
```bash
NODE_ENV=production
APP_SCRIPT=dist/server.js
APP_INSTANCES=max
PORT=3000
WATCH_FILES=false
MAX_MEMORY=2G
DATABASE_URL=mongodb://prod-cluster:27017/myapp
REDIS_URL=redis://prod-redis:6379
```

### 3. Conditional Configuration

Use environment-based conditional logic:

```json
{
  "apps": [
    {
      "name": "web-app",
      "script": "server.js",
      "env": {
        "NODE_ENV": "development"
      },
      "env_development": {
        "NODE_ENV": "development",
        "PORT": "3000",
        "DEBUG": "*",
        "DB_URL": "mongodb://localhost:27017/myapp_dev"
      },
      "env_staging": {
        "NODE_ENV": "staging",
        "PORT": "3000",
        "DB_URL": "mongodb://staging-db:27017/myapp_staging"
      },
      "env_production": {
        "NODE_ENV": "production",
        "PORT": "3000",
        "DB_URL": "mongodb://prod-cluster:27017/myapp"
      }
    }
  ]
}
```

## Environment-Specific Settings

### Development Environment

**Characteristics:**
- Fast iteration and debugging
- File watching enabled
- Detailed logging
- Relaxed restart policies

**Configuration:**
```json
{
  "apps": [
    {
      "name": "dev-app",
      "script": "npm run dev",
      "watch": true,
      "ignore_watch": ["node_modules", "*.log"],
      "env": {
        "NODE_ENV": "development",
        "DEBUG": "*",
        "LOG_LEVEL": "debug"
      },
      "instances": 1,
      "max_restarts": 50,
      "min_uptime": "1s",
      "restart_delay": 100
    }
  ]
}
```

### Staging Environment

**Characteristics:**
- Production-like setup
- Testing and validation
- Moderate monitoring
- Controlled resource usage

**Configuration:**
```json
{
  "apps": [
    {
      "name": "staging-app",
      "script": "dist/server.js",
      "instances": 2,
      "exec_mode": "cluster",
      "env": {
        "NODE_ENV": "staging",
        "LOG_LEVEL": "info"
      },
      "max_restarts": 5,
      "min_uptime": "5s",
      "max_memory_restart": "800M",
      "health_check": {
        "enabled": true,
        "url": "http://localhost:3000/health",
        "interval": 60
      }
    }
  ]
}
```

### Production Environment

**Characteristics:**
- Maximum performance and reliability
- Comprehensive monitoring
- Strict resource limits
- Minimal restarts

**Configuration:**
```json
{
  "apps": [
    {
      "name": "prod-app",
      "script": "dist/server.js",
      "instances": "max",
      "exec_mode": "cluster",
      "env": {
        "NODE_ENV": "production",
        "LOG_LEVEL": "warn"
      },
      "max_restarts": 3,
      "min_uptime": "10s",
      "max_memory_restart": "1G",
      "kill_timeout": 5000,
      "health_check": {
        "enabled": true,
        "url": "http://localhost:3000/health",
        "interval": 30,
        "timeout": 5,
        "retries": 3
      },
      "monitoring": {
        "cpu_threshold": 80,
        "memory_threshold": 85
      }
    }
  ]
}
```

## Deployment Strategies

### 1. Environment-Specific Deployment

```bash
# Development
pmdaemon start ecosystem.dev.json

# Staging
pmdaemon start ecosystem.staging.json

# Production
pmdaemon start ecosystem.prod.json
```

### 2. Environment Variable Deployment

```bash
# Development
NODE_ENV=development pmdaemon start ecosystem.json

# Production
NODE_ENV=production pmdaemon start ecosystem.json
```

### 3. Docker-Based Deployment

**Dockerfile:**
```dockerfile
FROM node:18-alpine

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

# Install PMDaemon
RUN cargo install pmdaemon

COPY ecosystem.json ./

CMD ["pmdaemon", "start", "ecosystem.json"]
```

**docker-compose.yml:**
```yaml
version: '3.8'
services:
  app-dev:
    build: .
    environment:
      - NODE_ENV=development
      - PORT=3000
      - DATABASE_URL=mongodb://mongo:27017/myapp_dev
    volumes:
      - .:/app
      - /app/node_modules
    ports:
      - "3000:3000"
    depends_on:
      - mongo

  app-prod:
    build: .
    environment:
      - NODE_ENV=production
      - PORT=3000
      - DATABASE_URL=mongodb://mongo:27017/myapp
    ports:
      - "3000:3000"
    depends_on:
      - mongo
    restart: unless-stopped
```

## Environment Variable Management

### 1. Environment Files

**.env.development:**
```bash
NODE_ENV=development
DEBUG=*
DB_HOST=localhost
DB_PORT=5432
DB_NAME=myapp_dev
REDIS_HOST=localhost
REDIS_PORT=6379
LOG_LEVEL=debug
```

**.env.production:**
```bash
NODE_ENV=production
DB_HOST=prod-db-cluster
DB_PORT=5432
DB_NAME=myapp
REDIS_HOST=prod-redis
REDIS_PORT=6379
LOG_LEVEL=error
```

### 2. Secure Secrets Management

For sensitive data, use external secret management:

```json
{
  "apps": [
    {
      "name": "secure-app",
      "script": "server.js",
      "env": {
        "NODE_ENV": "production",
        "DB_PASSWORD_FILE": "/run/secrets/db_password",
        "JWT_SECRET_FILE": "/run/secrets/jwt_secret"
      }
    }
  ]
}
```

### 3. Dynamic Configuration Loading

```javascript
// config.js
const fs = require('fs');

function loadConfig() {
  const env = process.env.NODE_ENV || 'development';

  // Load base config
  const baseConfig = require('./config.base.json');

  // Load environment-specific config
  const envConfig = require(`./config.${env}.json`);

  // Load secrets from files
  if (process.env.DB_PASSWORD_FILE) {
    envConfig.database.password = fs.readFileSync(
      process.env.DB_PASSWORD_FILE,
      'utf8'
    ).trim();
  }

  return { ...baseConfig, ...envConfig };
}

module.exports = loadConfig();
```

## Configuration Validation

### Environment-Specific Schemas

**development.schema.json:**
```json
{
  "type": "object",
  "required": ["name", "script"],
  "properties": {
    "watch": { "type": "boolean", "default": true },
    "instances": { "type": "number", "maximum": 4 },
    "max_restarts": { "type": "number", "minimum": 10 }
  }
}
```

**production.schema.json:**
```json
{
  "type": "object",
  "required": ["name", "script", "health_check"],
  "properties": {
    "watch": { "type": "boolean", "const": false },
    "instances": { "type": ["number", "string"] },
    "max_restarts": { "type": "number", "maximum": 5 },
    "health_check": { "type": "object", "required": ["enabled"] }
  }
}
```

### Validation Scripts

```bash
#!/bin/bash
# validate-config.sh

ENV=${1:-development}
CONFIG_FILE="ecosystem.${ENV}.json"

echo "Validating configuration for environment: $ENV"

# Validate JSON syntax
if ! jq empty "$CONFIG_FILE" 2>/dev/null; then
  echo "Error: Invalid JSON syntax in $CONFIG_FILE"
  exit 1
fi

# Validate against schema
if ! pmdaemon validate "$CONFIG_FILE" --schema="${ENV}.schema.json"; then
  echo "Error: Configuration validation failed"
  exit 1
fi

echo "Configuration is valid for $ENV environment"
```

## Best Practices

### 1. Environment Isolation
- Use separate databases for each environment
- Isolate network configurations
- Use different service accounts/credentials

### 2. Configuration Management
- Version control all configuration files
- Use environment-specific validation
- Document environment differences

### 3. Security Considerations
- Never commit secrets to version control
- Use secret management systems in production
- Rotate credentials regularly

### 4. Testing Strategies
- Test configurations in staging before production
- Validate environment parity
- Use automated deployment pipelines

### 5. Monitoring and Observability
- Environment-specific monitoring dashboards
- Different alerting thresholds per environment
- Comprehensive logging strategies

## Related Documentation

- **[Ecosystem Files](./ecosystem-files.md)** - Complete ecosystem configuration guide
- **[Best Practices](./best-practices.md)** - Configuration best practices
- **[Security](../security/overview.md)** - Security considerations
