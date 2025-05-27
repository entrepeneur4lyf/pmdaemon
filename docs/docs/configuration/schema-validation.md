# Schema Validation

PMDaemon provides comprehensive configuration validation to ensure your process configurations are correct before deployment. This prevents runtime errors and helps catch configuration issues early in the development cycle.

## Overview

PMDaemon's validation system includes:

- **🔍 Schema validation** - Validates configuration structure and types
- **📋 Required field checking** - Ensures all mandatory fields are present
- **🎯 Value range validation** - Checks numeric ranges and constraints
- **🔗 Dependency validation** - Validates relationships between fields
- **⚠️ Warning detection** - Identifies potential configuration issues

## Validation Commands

### Validate Configuration Files

```bash
# Validate a configuration file
pmdaemon validate ecosystem.json

# Validate with detailed output
pmdaemon validate ecosystem.json --verbose

# Validate multiple files
pmdaemon validate ecosystem.json ecosystem.staging.json
```

### Validate Before Starting

```bash
# Validate and start if valid
pmdaemon start --config ecosystem.json --validate

# CLI validation (automatic)
pmdaemon start "node server.js" --name web-api --port invalid
# Error: Invalid port format 'invalid'
```

## Configuration Schema

### Basic Process Schema

```json
{
  "$schema": "https://pmdaemon.io/schema/process.json",
  "type": "object",
  "required": ["name", "script"],
  "properties": {
    "name": {
      "type": "string",
      "pattern": "^[a-zA-Z0-9_-]+$",
      "minLength": 1,
      "maxLength": 50
    },
    "script": {
      "type": "string",
      "minLength": 1
    },
    "args": {
      "type": "array",
      "items": {
        "type": "string"
      }
    },
    "instances": {
      "oneOf": [
        {
          "type": "integer",
          "minimum": 0
        },
        {
          "type": "string",
          "enum": ["max"]
        }
      ]
    }
  }
}
```

### Port Configuration Schema

```json
{
  "port": {
    "oneOf": [
      {
        "type": "string",
        "pattern": "^\\d+$",
        "description": "Single port number"
      },
      {
        "type": "string",
        "pattern": "^\\d+-\\d+$",
        "description": "Port range (e.g., 3000-3003)"
      },
      {
        "type": "string",
        "pattern": "^auto:\\d+-\\d+$",
        "description": "Auto-assign from range"
      },
      {
        "type": "integer",
        "minimum": 1,
        "maximum": 65535
      }
    ]
  }
}
```

### Health Check Schema

```json
{
  "health_check": {
    "type": "object",
    "required": ["check_type"],
    "properties": {
      "check_type": {
        "type": "string",
        "enum": ["http", "script"]
      },
      "url": {
        "type": "string",
        "format": "uri",
        "description": "Required for HTTP health checks"
      },
      "script": {
        "type": "string",
        "description": "Required for script health checks"
      },
      "timeout": {
        "type": "integer",
        "minimum": 1,
        "maximum": 300,
        "default": 30
      },
      "interval": {
        "type": "integer",
        "minimum": 5,
        "maximum": 3600,
        "default": 60
      },
      "retries": {
        "type": "integer",
        "minimum": 1,
        "maximum": 10,
        "default": 3
      },
      "enabled": {
        "type": "boolean",
        "default": false,
        "description": "Whether health checks are enabled"
      }
    },
    "if": {
      "properties": {
        "check_type": {
          "const": "http"
        }
      }
    },
    "then": {
      "required": ["url"]
    },
    "else": {
      "required": ["script"]
    }
  }
}
```

## Validation Rules

### Required Fields

```json
// ✅ Valid: All required fields present
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"]
}

// ❌ Invalid: Missing required field 'name'
{
  "script": "node",
  "args": ["server.js"]
}
```

### Name Validation

```json
// ✅ Valid names
{
  "name": "web-api"
}
{
  "name": "user_service"
}
{
  "name": "worker-123"
}

// ❌ Invalid names
{
  "name": "web api"  // Spaces not allowed
}
{
  "name": "service@prod"  // Special characters not allowed
}
{
  "name": ""  // Empty string not allowed
}
```

### Instance Validation

```json
// ✅ Valid instances
{
  "instances": 1
}
{
  "instances": 4
}
{
  "instances": "max"
}

// ❌ Invalid instances
{
  "instances": 0  // Must be positive
}
{
  "instances": -1  // Negative not allowed
}
{
  "instances": "invalid"  // Invalid string value
}
```

### Port Validation

```json
// ✅ Valid port configurations
{
  "port": "3000"
}
{
  "port": 3000
}
{
  "port": "3000-3003"
}
{
  "port": "auto:5000-5100"
}

// ❌ Invalid port configurations
{
  "port": "0"  // Port 0 not allowed
}
{
  "port": "70000"  // Port > 65535
}
{
  "port": "3000-2999"  // Invalid range (start > end)
}
{
  "port": "invalid"  // Invalid format
}
```

### Memory Validation

```json
// ✅ Valid memory formats
{
  "max_memory_restart": "512M"
}
{
  "max_memory_restart": "1G"
}
{
  "max_memory_restart": "100KB"
}

// ❌ Invalid memory formats
{
  "max_memory_restart": "invalid"
}
{
  "max_memory_restart": "512"  // Missing unit
}
{
  "max_memory_restart": "-1M"  // Negative not allowed
}
```

## Cross-Field Validation

### Port Range vs Instances

```json
// ❌ Invalid: Need 4 ports but only 2 in range
{
  "name": "web-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "port": "3000-3001"
}

// ✅ Valid: 4 ports for 4 instances
{
  "name": "web-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "port": "3000-3003"
}
```

### Health Check Dependencies

```json
// ❌ Invalid: HTTP health check without URL
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "health_check": {
    "check_type": "http",
    "timeout": 10
  }
}

// ✅ Valid: HTTP health check with URL
{
  "name": "web-api",
  "script": "node",
  "args": ["server.js"],
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 10
  }
}
```

## Validation Examples

### Complete Valid Configuration

```json
{
  "apps": [
    {
      "name": "production-api",
      "script": "node",
      "args": ["dist/server.js"],
      "instances": 4,
      "port": "3000-3003",
      "cwd": "/var/www/api",
      "env": {
        "NODE_ENV": "production",
        "DATABASE_URL": "postgres://localhost/mydb"
      },
      "max_memory_restart": "1G",
      "autorestart": true,
      "max_restarts": 5,
      "min_uptime": "10s",
      "restart_delay": "2s",
      "kill_timeout": "30s",
      "health_check": {
        "check_type": "http",
        "url": "http://localhost:3000/health",
        "timeout": 10,
        "interval": 30,
        "retries": 3,
        "enabled": true
      }
    }
  ]
}
```

### Common Validation Errors

#### Missing Required Fields

```json
// Error: Missing required field 'name'
{
  "script": "node",
  "args": ["server.js"]
}
```

**Error message:**
```
Validation Error: Missing required field 'name'
  at apps[0]
  Required fields: name, script
```

#### Invalid Port Range

```json
{
  "name": "web-cluster",
  "script": "node",
  "args": ["server.js"],
  "instances": 4,
  "port": "3000-3001"
}
```

**Error message:**
```
Validation Error: Port range insufficient for instances
  at apps[0].port
  Need 4 ports but range 3000-3001 only provides 2 ports
  Suggestion: Use port range "3000-3003" or reduce instances to 2
```

#### Invalid Health Check Configuration

```json
{
  "name": "api",
  "script": "node",
  "args": ["api.js"],
  "health_check": {
    "check_type": "http",
    "timeout": 10
  }
}
```

**Error message:**
```
Validation Error: Missing required field for HTTP health check
  at apps[0].health_check
  HTTP health checks require 'url' field
  Example: "url": "http://localhost:3000/health"
```

## Validation Warnings

PMDaemon also provides warnings for potentially problematic configurations:

### Performance Warnings

```json
{
  "name": "high-instance-service",
  "script": "node",
  "args": ["server.js"],
  "instances": 32  // Warning: High instance count
}
```

**Warning message:**
```
Warning: High instance count (32) may cause resource contention
  at apps[0].instances
  Consider reducing instances or increasing system resources
  Recommended: Use instances <= CPU cores (8)
```

### Memory Warnings

```json
{
  "name": "memory-service",
  "script": "node",
  "args": ["server.js"],
  "max_memory_restart": "8G"  // Warning: High memory limit
}
```

**Warning message:**
```
Warning: High memory limit (8G) may cause system instability
  at apps[0].max_memory_restart
  System has 8G total memory
  Recommendation: Use max_memory_restart <= 6G
```

### Health Check Warnings

```json
{
  "name": "api-service",
  "script": "node",
  "args": ["api.js"],
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 1  // Warning: Very short timeout
  }
}
```

**Warning message:**
```
Warning: Very short health check timeout (1s)
  at apps[0].health_check.timeout
  May cause false positives for slow-starting services
  Recommendation: Use timeout >= 5s
```

## Custom Validation Rules

### Environment-Specific Validation

```bash
# Validate for production environment
pmdaemon validate ecosystem.json --env production

# Validate for development environment
pmdaemon validate ecosystem.json --env development
```

### Strict Validation Mode

```bash
# Treat warnings as errors
pmdaemon validate ecosystem.json --strict

# Validate with custom schema
pmdaemon validate ecosystem.json --schema custom-schema.json
```

## Integration with CI/CD

### GitHub Actions

```yaml
name: Validate PMDaemon Configuration
on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install PMDaemon
        run: cargo install pmdaemon
      - name: Validate Configuration
        run: |
          pmdaemon validate ecosystem.json --strict
          pmdaemon validate ecosystem.production.json --strict
```

### GitLab CI

```yaml
validate-config:
  stage: validate
  script:
    - cargo install pmdaemon
    - pmdaemon validate ecosystem.json --strict
  only:
    changes:
      - ecosystem*.json
      - "**/*.pmdaemon.json"
```

## Best Practices

### 1. Validate Early and Often

```bash
# Validate during development
pmdaemon validate ecosystem.json

# Validate before deployment
pmdaemon validate ecosystem.production.json --strict
```

### 2. Use Schema Versioning

```json
{
  "$schema": "https://pmdaemon.io/schema/v1/process.json",
  "apps": [...]
}
```

### 3. Handle Validation in Scripts

```bash
#!/bin/bash

echo "Validating configuration..."
if pmdaemon validate ecosystem.json --strict; then
    echo "✅ Configuration valid"
    pmdaemon --config ecosystem.json start
else
    echo "❌ Configuration invalid"
    exit 1
fi
```

### 4. Use Environment-Specific Validation

```bash
# Different validation rules for different environments
pmdaemon validate ecosystem.dev.json --env development
pmdaemon validate ecosystem.prod.json --env production --strict
```

## Troubleshooting Validation Issues

### Common Issues and Solutions

#### Port Conflicts

```bash
# Check for port conflicts
pmdaemon validate ecosystem.json --check-ports

# Resolve by using auto-assignment
{
  "port": "auto:3000-3100"
}
```

#### Memory Limits

```bash
# Check system memory constraints
pmdaemon validate ecosystem.json --check-resources

# Adjust memory limits accordingly
{
  "max_memory_restart": "512M"  // Reduced from 2G
}
```

#### Health Check URLs

```bash
# Validate health check endpoints
pmdaemon validate ecosystem.json --check-health-urls

# Fix unreachable URLs
{
  "health_check": {
    "url": "http://localhost:3000/health"  // Fixed from external URL
  }
}
```

## Next Steps

- **[Process Configuration](./process-configuration.md)** - Individual process settings
- **[Advanced Configuration](./advanced-configuration.md)** - Complex scenarios
- **[Ecosystem Files](./ecosystem-files.md)** - Multi-process configurations
- **[Troubleshooting](../advanced/troubleshooting.md)** - Common configuration issues
