# REST API Reference

PMDaemon provides a comprehensive REST API for remote process management and monitoring. The API is PM2-compatible and includes additional features unique to PMDaemon.

## Getting Started

### Start the Web Server

```bash
# Start with default settings (localhost:9615)
pmdaemon web

# Custom host and port
pmdaemon web --host 0.0.0.0 --port 8080
```

### Base URL

```
http://localhost:9615
```

### Authentication

Currently, PMDaemon API doesn't require authentication. For production use, consider:
- Running behind a reverse proxy with authentication
- Using firewall rules to restrict access
- Binding to localhost only (`--host 127.0.0.1`)

## API Endpoints

### Root Endpoint

**GET** `/`

Get API information and available endpoints.

#### Response

```json
{
  "name": "PMDaemon",
  "version": "0.1.1",
  "description": "A feature-rich PM2 clone in Rust with advanced capabilities",
  "status": "running",
  "endpoints": {
    "processes": "/api/processes",
    "system": "/api/system",
    "status": "/api/status",
    "websocket": "/ws"
  }
}
```

## Process Management

### List Processes

**GET** `/api/processes`

Get list of all processes with their current status.

#### Query Parameters

| Parameter | Type | Description | Example |
|-----------|------|-------------|---------|
| `status` | String | Filter by process status | `?status=online` |
| `name` | String | Filter by process name | `?name=web-app` |

#### Response

```json
[
  {
    "id": 0,
    "name": "web-app",
    "status": "online",
    "pid": 1234,
    "port": "3000",
    "cpu_usage": 2.5,
    "memory_usage": 47448064,
    "uptime": "2h 15m 30s",
    "restarts": 0,
    "health_status": "healthy"
  },
  {
    "id": 1,
    "name": "api-server",
    "status": "online",
    "pid": 1235,
    "port": "8000",
    "cpu_usage": 1.8,
    "memory_usage": 33554432,
    "uptime": "1h 45m 12s",
    "restarts": 1,
    "health_status": "healthy"
  }
]
```

### Get Process Details

**GET** `/api/processes/{id}`

Get detailed information about a specific process.

#### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | String | Process ID or name |

#### Response

```json
{
  "id": 0,
  "name": "web-app",
  "status": "online",
  "pid": 1234,
  "port": "3000",
  "cpu_usage": 2.5,
  "memory_usage": 47448064,
  "uptime": "2h 15m 30s",
  "restarts": 0,
  "config": {
    "script": "node",
    "args": ["server.js"],
    "instances": 1,
    "max_memory_restart": 536870912,
    "port": "3000",
    "env": {
      "NODE_ENV": "production",
      "PORT": "3000"
    }
  },
  "health_check": {
    "enabled": true,
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "status": "healthy",
    "last_check": "2024-01-15T10:30:00Z",
    "response_time": 45
  },
  "logs": {
    "out_file": "/home/user/.local/share/pmdaemon/logs/web-app-0.out",
    "error_file": "/home/user/.local/share/pmdaemon/logs/web-app-0.err"
  }
}
```

### Start Process

**POST** `/api/processes`

Start a new process.

#### Request Body

```json
{
  "name": "new-app",
  "script": "node",
  "args": ["server.js"],
  "instances": 1,
  "port": "3000",
  "max_memory_restart": "512M",
  "cwd": "/path/to/app",
  "env": {
    "NODE_ENV": "production"
  },
  "health_check": {
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "timeout": 5,
    "interval": 30,
    "retries": 3
  }
}
```

#### Response

```json
{
  "success": true,
  "message": "Process started successfully",
  "process": {
    "id": 2,
    "name": "new-app",
    "status": "starting",
    "pid": null
  }
}
```

### Stop Process

**POST** `/api/processes/{id}/stop`

Stop a running process.

#### Response

```json
{
  "success": true,
  "message": "Process stopped successfully"
}
```

### Restart Process

**POST** `/api/processes/{id}/restart`

Restart a process with optional port override.

#### Request Body (Optional)

```json
{
  "port": "3001"
}
```

#### Response

```json
{
  "success": true,
  "message": "Process restarted successfully"
}
```

### Reload Process

**POST** `/api/processes/{id}/reload`

Gracefully reload a process (zero-downtime restart).

#### Request Body (Optional)

```json
{
  "port": "4000-4003"
}
```

#### Response

```json
{
  "success": true,
  "message": "Process reloaded successfully"
}
```

### Delete Process

**DELETE** `/api/processes/{id}`

Delete a process (stops if running).

#### Response

```json
{
  "success": true,
  "message": "Process deleted successfully"
}
```

### Get Process Logs

**GET** `/api/processes/{id}/logs`

Get process logs with optional filtering.

#### Query Parameters

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `lines` | Number | Number of lines to return | 20 |
| `type` | String | Log type (`out`, `error`, `all`) | `all` |

#### Response

```json
{
  "logs": [
    {
      "timestamp": "2024-01-15T10:30:00Z",
      "type": "out",
      "message": "Server started on port 3000"
    },
    {
      "timestamp": "2024-01-15T10:30:01Z",
      "type": "out",
      "message": "Database connected successfully"
    }
  ],
  "total_lines": 156
}
```

## System Information

### System Metrics

**GET** `/api/system`

Get system metrics and resource usage.

#### Response

```json
{
  "hostname": "server-01",
  "platform": "linux",
  "arch": "x86_64",
  "uptime": "5d 12h 30m",
  "cpu": {
    "cores": 8,
    "usage": 15.2,
    "load_average": [1.2, 1.5, 1.8]
  },
  "memory": {
    "total": 16777216000,
    "used": 8388608000,
    "free": 8388608000,
    "usage_percent": 50.0
  },
  "disk": {
    "total": 1000000000000,
    "used": 500000000000,
    "free": 500000000000,
    "usage_percent": 50.0
  },
  "processes": {
    "total": 3,
    "online": 2,
    "stopped": 1,
    "errored": 0
  }
}
```

### Status Check

<div class="api-endpoint">
  <span class="api-method get">GET</span>
  <span class="api-path">/api/status</span>
</div>

Simple health check endpoint for load balancers.

#### Response

```json
{
  "status": "ok",
  "timestamp": "2024-01-15T10:30:00Z",
  "version": "0.1.1"
}
```

## Error Responses

All endpoints return consistent error responses:

### Error Format

```json
{
  "success": false,
  "error": {
    "code": "PROCESS_NOT_FOUND",
    "message": "Process with ID 'invalid-id' not found",
    "details": {
      "requested_id": "invalid-id",
      "available_processes": ["web-app", "api-server"]
    }
  }
}
```

### HTTP Status Codes

| Code | Description | Example |
|------|-------------|---------|
| 200 | Success | Request completed successfully |
| 201 | Created | Process started successfully |
| 400 | Bad Request | Invalid request parameters |
| 404 | Not Found | Process not found |
| 409 | Conflict | Process name already exists |
| 500 | Internal Error | Server error |

### Common Error Codes

| Code | Description |
|------|-------------|
| `PROCESS_NOT_FOUND` | Specified process doesn't exist |
| `PROCESS_ALREADY_EXISTS` | Process name already in use |
| `INVALID_CONFIGURATION` | Invalid process configuration |
| `OPERATION_FAILED` | Process operation failed |
| `PERMISSION_DENIED` | Insufficient permissions |

## Rate Limiting

The API includes basic rate limiting:
- **100 requests per minute** per IP address
- **Burst limit**: 20 requests in 10 seconds
- **Headers included** in responses:
  - `X-RateLimit-Limit`: Request limit
  - `X-RateLimit-Remaining`: Remaining requests
  - `X-RateLimit-Reset`: Reset timestamp

## CORS Support

The API includes CORS headers for browser access:
- **Allowed Origins**: Configurable (default: all)
- **Allowed Methods**: GET, POST, PUT, DELETE, OPTIONS
- **Allowed Headers**: Content-Type, Authorization

## Examples

### Using curl

```bash
# List all processes
curl http://localhost:9615/api/processes

# Start a new process
curl -X POST http://localhost:9615/api/processes \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test-app",
    "script": "node",
    "args": ["server.js"],
    "port": "3000"
  }'

# Get process details
curl http://localhost:9615/api/processes/test-app

# Stop a process
curl -X POST http://localhost:9615/api/processes/test-app/stop

# Get system metrics
curl http://localhost:9615/api/system
```

### Using JavaScript (fetch)

```javascript
// List processes
const processes = await fetch('http://localhost:9615/api/processes')
  .then(res => res.json());

// Start a process
const result = await fetch('http://localhost:9615/api/processes', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    name: 'web-app',
    script: 'node',
    args: ['server.js'],
    port: '3000'
  })
}).then(res => res.json());

// Get system metrics
const system = await fetch('http://localhost:9615/api/system')
  .then(res => res.json());
```

### Using Python (requests)

```python
import requests

# List processes
response = requests.get('http://localhost:9615/api/processes')
processes = response.json()

# Start a process
process_config = {
    'name': 'python-app',
    'script': 'python',
    'args': ['app.py'],
    'port': '8000'
}
response = requests.post(
    'http://localhost:9615/api/processes',
    json=process_config
)
result = response.json()
```

## Next Steps

- **[WebSocket API](./websocket-api.md)** - Real-time updates via WebSocket
- **[Library Usage](./library-usage.md)** - Using PMDaemon as a Rust library
- **[Error Handling](./error-handling.md)** - Comprehensive error handling guide

---

The REST API provides full programmatic access to PMDaemon's process management capabilities, making it easy to integrate with monitoring systems, deployment tools, and custom applications.
