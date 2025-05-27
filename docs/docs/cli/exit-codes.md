# Exit Codes

PMDaemon uses standardized exit codes to indicate the success or failure of operations. Understanding these codes is essential for automation, scripting, and troubleshooting.

## Standard Exit Codes

### Success Codes

#### `0` - Success
**Description:** Command completed successfully  
**When returned:** All operations completed without errors  
**Example:**
```bash
pmdaemon start "node server.js" --name web-api
echo $?  # Output: 0

pmdaemon stop web-api
echo $?  # Output: 0
```

### General Error Codes

#### `1` - General Error
**Description:** Generic error or unspecified failure  
**When returned:** Unexpected errors, general failures  
**Common causes:**
- Invalid command syntax
- Unexpected runtime errors
- Configuration parsing errors

**Example:**
```bash
pmdaemon invalid-command
echo $?  # Output: 1

pmdaemon start
echo $?  # Output: 1 (missing required arguments)
```

#### `2` - Invalid Arguments
**Description:** Invalid command line arguments or options  
**When returned:** Incorrect CLI usage, invalid parameters  
**Common causes:**
- Missing required arguments
- Invalid option values
- Conflicting options

**Example:**
```bash
pmdaemon start --name web-api  # Missing script
echo $?  # Output: 2

pmdaemon start "node server.js" --instances -1  # Invalid instance count
echo $?  # Output: 2
```

## Process Management Error Codes

#### `10` - Process Not Found
**Description:** Specified process does not exist  
**When returned:** Operations on non-existent processes  
**Common causes:**
- Typo in process name
- Process was already deleted
- Wrong namespace

**Example:**
```bash
pmdaemon stop nonexistent-process
echo $?  # Output: 10

pmdaemon restart invalid-name
echo $?  # Output: 10
```

#### `11` - Process Already Exists
**Description:** Attempting to create a process with an existing name  
**When returned:** Starting a process with a duplicate name  
**Common causes:**
- Process name conflict
- Attempting to start already running process

**Example:**
```bash
pmdaemon start "node server.js" --name web-api
pmdaemon start "python app.py" --name web-api  # Same name
echo $?  # Output: 11
```

#### `12` - Process Start Failed
**Description:** Failed to start the specified process  
**When returned:** Process startup failures  
**Common causes:**
- Invalid script path
- Permission issues
- Resource constraints
- Port conflicts

**Example:**
```bash
pmdaemon start "nonexistent-script.js" --name test
echo $?  # Output: 12

pmdaemon start "node server.js" --name web --port 80  # Permission denied
echo $?  # Output: 12
```

#### `13` - Process Stop Failed
**Description:** Failed to stop the specified process  
**When returned:** Process shutdown failures  
**Common causes:**
- Process not responding to signals
- Permission issues
- Process already stopped

**Example:**
```bash
pmdaemon stop unresponsive-process
echo $?  # Output: 13
```

#### `14` - Process Restart Failed
**Description:** Failed to restart the specified process  
**When returned:** Process restart failures  
**Common causes:**
- Stop operation failed
- Start operation failed
- Configuration changes invalid

**Example:**
```bash
pmdaemon restart broken-process
echo $?  # Output: 14
```

## Configuration Error Codes

#### `20` - Configuration File Not Found
**Description:** Specified configuration file does not exist  
**When returned:** Using `--config` with invalid file path  
**Common causes:**
- Incorrect file path
- File permissions
- File deleted

**Example:**
```bash
pmdaemon --config nonexistent.json start
echo $?  # Output: 20
```

#### `21` - Configuration Parse Error
**Description:** Configuration file contains invalid syntax  
**When returned:** Malformed JSON, YAML, or TOML files  
**Common causes:**
- Syntax errors in configuration
- Invalid JSON/YAML/TOML format
- Encoding issues

**Example:**
```bash
# ecosystem.json contains invalid JSON
pmdaemon --config ecosystem.json start
echo $?  # Output: 21
```

#### `22` - Configuration Validation Error
**Description:** Configuration contains invalid values  
**When returned:** Valid syntax but invalid configuration values  
**Common causes:**
- Invalid port numbers
- Invalid memory formats
- Missing required fields

**Example:**
```bash
# Configuration has invalid port range
pmdaemon start "node server.js" --name web --instances 4 --port 3000-3001
echo $?  # Output: 22
```

## Resource Error Codes

#### `30` - Port Conflict
**Description:** Requested port is already in use  
**When returned:** Port assignment conflicts  
**Common causes:**
- Port already used by another process
- Port already assigned to another PMDaemon process
- Insufficient ports in range

**Example:**
```bash
pmdaemon start "node server1.js" --name app1 --port 3000
pmdaemon start "node server2.js" --name app2 --port 3000  # Conflict
echo $?  # Output: 30
```

#### `31` - Memory Limit Exceeded
**Description:** Process exceeded memory limits  
**When returned:** Memory constraint violations  
**Common causes:**
- Process using too much memory
- System memory exhausted
- Invalid memory limit format

**Example:**
```bash
pmdaemon start "memory-hungry-app" --name app --max-memory 10M
# If app uses more than 10MB
echo $?  # Output: 31
```

#### `32` - File System Error
**Description:** File system operation failed  
**When returned:** File/directory access issues  
**Common causes:**
- Permission denied
- Disk space full
- Invalid file paths

**Example:**
```bash
pmdaemon start "node server.js" --name web --out-file /root/logs/app.log
echo $?  # Output: 32 (permission denied)
```

## Health Check Error Codes

#### `40` - Health Check Failed
**Description:** Process health checks are failing  
**When returned:** Health check validation failures  
**Common causes:**
- HTTP endpoint not responding
- Health check script returning non-zero
- Health check timeout

**Example:**
```bash
pmdaemon start "node api.js" --name api \
  --health-check-url http://localhost:3000/health \
  --wait-ready
# If health check fails
echo $?  # Output: 40
```

#### `41` - Health Check Timeout
**Description:** Health check timed out during blocking start  
**When returned:** `--wait-ready` timeout exceeded  
**Common causes:**
- Process taking too long to start
- Health check endpoint not available
- Network issues

**Example:**
```bash
pmdaemon start "slow-starting-app" --name app \
  --health-check-url http://localhost:3000/health \
  --wait-ready --wait-timeout 10s
# If app takes longer than 10s to be healthy
echo $?  # Output: 41
```

## Network Error Codes

#### `50` - Web Server Start Failed
**Description:** Failed to start web monitoring server  
**When returned:** Web server startup failures  
**Common causes:**
- Port already in use
- Permission denied
- Invalid host/port configuration

**Example:**
```bash
pmdaemon web --port 80  # Permission denied
echo $?  # Output: 50

pmdaemon web --port 3000  # Port in use
echo $?  # Output: 50
```

#### `51` - Network Connection Failed
**Description:** Network operation failed  
**When returned:** Network connectivity issues  
**Common causes:**
- Health check URL unreachable
- Web API connection failed
- DNS resolution failed

**Example:**
```bash
pmdaemon start "node api.js" --name api \
  --health-check-url http://unreachable-host/health
echo $?  # Output: 51
```

## Permission Error Codes

#### `60` - Permission Denied
**Description:** Insufficient permissions for operation  
**When returned:** Permission-related failures  
**Common causes:**
- Cannot write to log directory
- Cannot bind to privileged port
- Cannot execute script

**Example:**
```bash
pmdaemon start "node server.js" --name web --port 80
echo $?  # Output: 60 (need root for port 80)

pmdaemon start "script.sh" --name script --out-file /root/app.log
echo $?  # Output: 60 (cannot write to /root)
```

#### `61` - Access Denied
**Description:** Access denied to required resources  
**When returned:** Resource access failures  
**Common causes:**
- File/directory access denied
- Process access denied
- System resource access denied

## System Error Codes

#### `70` - System Resource Exhausted
**Description:** System resources exhausted  
**When returned:** System resource limitations  
**Common causes:**
- Out of memory
- Too many open files
- Process limit reached

**Example:**
```bash
# System out of memory
pmdaemon start "memory-intensive-app" --name app
echo $?  # Output: 70
```

#### `71` - System Call Failed
**Description:** System call operation failed  
**When returned:** Low-level system operation failures  
**Common causes:**
- Signal delivery failed
- Process creation failed
- System API errors

## Using Exit Codes in Scripts

### Basic Error Handling

```bash
#!/bin/bash

# Start a process and check result
pmdaemon start "node server.js" --name web-api --port 3000
if [ $? -eq 0 ]; then
    echo "✅ Process started successfully"
else
    echo "❌ Failed to start process"
    exit 1
fi
```

### Comprehensive Error Handling

```bash
#!/bin/bash

start_process() {
    local name=$1
    local script=$2
    local port=$3
    
    pmdaemon start "$script" --name "$name" --port "$port"
    local exit_code=$?
    
    case $exit_code in
        0)
            echo "✅ Process '$name' started successfully"
            ;;
        11)
            echo "⚠️  Process '$name' already exists, restarting..."
            pmdaemon restart "$name"
            ;;
        30)
            echo "❌ Port $port is already in use"
            return 1
            ;;
        12)
            echo "❌ Failed to start process '$name' - check script path"
            return 1
            ;;
        *)
            echo "❌ Unknown error (exit code: $exit_code)"
            return 1
            ;;
    esac
}

# Usage
start_process "web-api" "node server.js" 3000
start_process "worker" "python worker.py" 8000
```

### Deployment Script with Health Checks

```bash
#!/bin/bash

deploy_service() {
    local name=$1
    local script=$2
    local health_url=$3
    
    echo "Deploying $name..."
    
    pmdaemon start "$script" --name "$name" \
        --health-check-url "$health_url" \
        --wait-ready --wait-timeout 60s
    
    local exit_code=$?
    
    case $exit_code in
        0)
            echo "✅ $name deployed and healthy"
            ;;
        40)
            echo "❌ $name started but health checks failed"
            pmdaemon logs "$name" --lines 20
            return 1
            ;;
        41)
            echo "❌ $name health check timed out"
            pmdaemon logs "$name" --lines 20
            return 1
            ;;
        *)
            echo "❌ Failed to deploy $name (exit code: $exit_code)"
            return 1
            ;;
    esac
}

# Deploy services
deploy_service "api" "node api.js" "http://localhost:3000/health"
deploy_service "worker" "python worker.py" "http://localhost:8000/health"
```

### Monitoring Script

```bash
#!/bin/bash

check_process_health() {
    local name=$1
    
    pmdaemon info "$name" > /dev/null 2>&1
    local exit_code=$?
    
    case $exit_code in
        0)
            echo "✅ $name is running"
            ;;
        10)
            echo "❌ $name not found - restarting..."
            pmdaemon start "node server.js" --name "$name"
            ;;
        *)
            echo "⚠️  $name status unknown (exit code: $exit_code)"
            ;;
    esac
}

# Monitor critical processes
check_process_health "web-api"
check_process_health "worker"
check_process_health "database"
```

## Best Practices

### 1. Always Check Exit Codes

```bash
# Good: Check exit codes
pmdaemon start "node server.js" --name web-api
if [ $? -ne 0 ]; then
    echo "Failed to start process"
    exit 1
fi

# Avoid: Ignoring exit codes
pmdaemon start "node server.js" --name web-api
# Continue regardless of success/failure
```

### 2. Use Specific Error Handling

```bash
# Good: Handle specific error codes
case $? in
    0) echo "Success" ;;
    11) echo "Process already exists" ;;
    30) echo "Port conflict" ;;
    *) echo "Unknown error" ;;
esac

# Avoid: Generic error handling
if [ $? -ne 0 ]; then
    echo "Something went wrong"
fi
```

### 3. Log Exit Codes for Debugging

```bash
# Good: Log exit codes
pmdaemon start "node server.js" --name web-api
EXIT_CODE=$?
echo "$(date): pmdaemon start exited with code $EXIT_CODE" >> deployment.log
```

### 4. Use Exit Codes in CI/CD

```yaml
# GitHub Actions example
- name: Deploy application
  run: |
    pmdaemon start "node server.js" --name web-api \
      --health-check-url http://localhost:3000/health \
      --wait-ready
    if [ $? -ne 0 ]; then
      echo "Deployment failed"
      exit 1
    fi
```

## Next Steps

- **[Commands](./commands.md)** - Complete command reference
- **[Configuration Options](./configuration-options.md)** - CLI options reference
- **[Troubleshooting](../advanced/troubleshooting.md)** - Common issues and solutions
- **[Examples](../examples/deployment-examples.md)** - Real-world deployment scripts
