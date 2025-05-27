---
id: schema-validation
title: Configuration Schema Validation
sidebar_label: Schema Validation
sidebar_position: 10
---
 # PMDaemon Configuration Schema Validation
 This document explains...
This document explains how to use schema validation with PMDaemon ecosystem configuration files.

## Schema File

The `ecosystem.schema.json` file provides a complete JSON Schema definition for PMDaemon configuration files. This schema:

- ✅ **Correctly defines string formats** for `port` and `max_memory_restart` fields
- ✅ **Supports all PMDaemon features** including port ranges, memory units, and health checks
- ✅ **Provides validation** for required fields and data types
- ✅ **Includes documentation** for all configuration options

## Using Schema Validation

### VS Code / IDE Setup

1. **JSON Files**: Add the schema reference at the top of your JSON config:
```json
{
  "$schema": "./ecosystem.schema.json",
  "apps": [
    // your app configurations
  ]
}
```

2. **YAML Files**: Add the schema comment at the top of your YAML config:
```yaml
# yaml-language-server: $schema=./ecosystem.schema.json
apps:
  - name: my-app
    # your app configuration
```

3. **TOML Files**: TOML schema validation is limited, but the configuration will still work with PMDaemon.

### Schema Validation Benefits

- **IntelliSense**: Auto-completion for configuration options
- **Error Detection**: Real-time validation of configuration syntax
- **Documentation**: Hover tooltips with field descriptions
- **Type Checking**: Ensures correct data types for all fields

## Important Notes

### String vs Number Fields

PMDaemon uses **string formats** for certain fields that other process managers might use numbers for:

#### Port Configuration
```yaml
# ✅ Correct PMDaemon format (strings)
port: "3000"           # Single port
port: "3000-3003"      # Port range  
port: "auto:4000-4100" # Auto assignment

# ❌ Generic PM2 format (numbers) - NOT supported
port: 3000             # PMDaemon expects strings
```

#### Memory Configuration
```yaml
# ✅ Correct PMDaemon format (strings with units)
max_memory_restart: "512M"  # 512 megabytes
max_memory_restart: "1G"    # 1 gigabyte
max_memory_restart: "100K"  # 100 kilobytes

# ✅ Also supported (raw bytes as number)
max_memory_restart: 536870912  # 512MB in bytes

# ❌ Generic format - NOT supported
max_memory_restart: "512MB"    # PMDaemon uses "M" not "MB"
```

### Why String Formats?

PMDaemon uses string formats for these fields because:

1. **Port Ranges**: `"3000-3003"` cannot be represented as a single number
2. **Auto Assignment**: `"auto:4000-4100"` requires string parsing
3. **Memory Units**: `"512M"` is more human-readable than `536870912`
4. **Consistency**: All port/memory configs use the same string-based approach

## Schema Validation Errors

If you see schema validation errors like:
```
Incorrect type. Expected "number".
```

This means your IDE is using a generic PM2 schema instead of the PMDaemon schema. To fix:

1. **Ensure the schema reference is correct** in your config file
2. **Use the PMDaemon schema file** (`ecosystem.schema.json`)
3. **Remember PMDaemon uses strings** for port and memory fields

## Example Valid Configurations

### JSON with Schema
```json
{
  "$schema": "./ecosystem.schema.json",
  "apps": [
    {
      "name": "web-app",
      "script": "node",
      "args": ["server.js"],
      "instances": 4,
      "port": "3000-3003",
      "max_memory_restart": "512M",
      "env": {
        "NODE_ENV": "production"
      }
    }
  ]
}
```

### YAML with Schema
```yaml
# yaml-language-server: $schema=./ecosystem.schema.json
apps:
  - name: api-service
    script: python
    args: ["-m", "uvicorn", "main:app"]
    instances: 2
    port: "auto:8000-8100"
    max_memory_restart: "1G"
    env:
      DATABASE_URL: postgres://localhost/db
```

## Testing Schema Validation

You can test that your configuration is valid by:

1. **Using the schema in your IDE** - errors will be highlighted
2. **Running PMDaemon** - it will validate and report errors
3. **Using a JSON Schema validator** online with the schema file

## Schema Updates

The schema file is maintained alongside PMDaemon development. When new features are added to PMDaemon, the schema will be updated to reflect them.

For the latest schema, always use the one included with your PMDaemon version in the `examples/` directory.
