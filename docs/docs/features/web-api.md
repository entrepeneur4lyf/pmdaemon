# Web API

PMDaemon provides a **comprehensive REST API** and **real-time WebSocket interface** for remote process management and monitoring. Built on the high-performance Axum web framework, it offers PM2-compatible endpoints with additional advanced features.

## Overview

The Web API provides:

- **🌐 REST API** - Full process management via HTTP
- **⚡ WebSocket support** - Real-time updates and monitoring
- **🔒 Security features** - CORS support and security headers
- **📊 System metrics** - CPU, memory, load average monitoring
- **📝 Log access** - Retrieve and stream process logs
- **🔄 PM2 compatibility** - Compatible JSON response format

## Starting the Web Server

### Basic Usage

```bash
# Start on default port (9615)
pmdaemon web

# Custom port and host
pmdaemon web --port 8080 --host 0.0.0.0

# Bind to all interfaces for remote access
pmdaemon web --port 9615 --host 0.0.0.0
```

### Configuration Options

| Option | Default | Description |
|--------|---------|-------------|
| `--port` | `9615` | Port to bind the web server |
| `--host` | `127.0.0.1` | Host address to bind |

### Security Considerations

```bash
# Local development (secure)
pmdaemon web --host 127.0.0.1

# Production with reverse proxy (recommended)
pmdaemon web --host 127.0.0.1 --port 9615
# Use nginx/Apache as reverse proxy with SSL

# Direct remote access (use with caution)
pmdaemon web --host 0.0.0.0 --port 9615
# Consider firewall rules and authentication
```

## REST API Endpoints

### Process Management

#### List All Processes

```http
GET /api/processes
```

**Response:**
```json
{
  "processes": [
    {
      "id": 0,
      "name": "web-api",
      "status": "online",
      "pid": 1234,
      "port": 3000,
      "cpu": 2.5,
      "memory": 47185920,
      "uptime": 7890000,
      "restarts": 0,
      "health": "healthy"
    }
  ],
  "total": 1
}
```

#### Get Process Details

```http
GET /api/processes/{name_or_id}
```

**Example:**
```bash
curl http://localhost:9615/api/processes/web-api
```

**Response:**
```json
{
  "id": 0,
  "name": "web-api",
  "status": "online",
  "pid": 1234,
  "port": 3000,
  "cpu": 2.5,
  "memory": 47185920,
  "uptime": 7890000,
  "restarts": 0,
  "health": "healthy",
  "config": {
    "script": "node",
    "args": ["server.js"],
    "cwd": "/app",
    "env": {
      "NODE_ENV": "production",
      "PORT": "3000"
    }
  }
}
```

#### Start a Process

```http
POST /api/processes
```

**Request Body:**
```json
{
  "name": "new-api",
  "script": "node",
  "args": ["app.js"],
  "port": "3001",
  "instances": 1,
  "env": {
    "NODE_ENV": "production"
  }
}
```

**Response:**
```json
{
  "success": true,
  "message": "Process 'new-api' started successfully",
  "process": {
    "id": 1,
    "name": "new-api",
    "status": "starting",
    "pid": null,
    "port": 3001
  }
}
```

#### Stop a Process

```http
DELETE /api/processes/{name_or_id}
```

**Example:**
```bash
curl -X DELETE http://localhost:9615/api/processes/web-api
```

**Response:**
```json
{
  "success": true,
  "message": "Process 'web-api' stopped successfully"
}
```

#### Restart a Process

```http
POST /api/processes/{name_or_id}/restart
```

**Optional Request Body:**
```json
{
  "port": "3002",
  "instances": 2
}
```

**Response:**
```json
{
  "success": true,
  "message": "Process 'web-api' restarted successfully"
}
```

### System Information

#### Get System Metrics

```http
GET /api/system
```

**Response:**
```json
{
  "cpu": {
    "usage": 15.2,
    "cores": 8
  },
  "memory": {
    "total": 8589934592,
    "used": 2147483648,
    "available": 6442450944,
    "usage_percent": 25.0
  },
  "load": {
    "one": 0.85,
    "five": 1.2,
    "fifteen": 0.9
  },
  "uptime": 432000,
  "timestamp": "2024-01-15T14:30:25Z"
}
```

### Log Management

#### Get Process Logs

```http
GET /api/logs/{name_or_id}
```

**Query Parameters:**
- `lines` - Number of lines to retrieve (default: 20)
- `type` - Log type: `stdout`, `stderr`, or `all` (default: all)

**Example:**
```bash
curl "http://localhost:9615/api/logs/web-api?lines=50&type=stdout"
```

**Response:**
```json
{
  "logs": [
    {
      "timestamp": "2024-01-15T14:30:25Z",
      "type": "stdout",
      "message": "Server started on port 3000"
    },
    {
      "timestamp": "2024-01-15T14:30:26Z", 
      "type": "stdout",
      "message": "Database connected successfully"
    }
  ],
  "total_lines": 50,
  "process": "web-api"
}
```

### Health Checks

#### Get Process Health Status

```http
GET /api/processes/{name_or_id}/health
```

**Response:**
```json
{
  "process": "web-api",
  "health": {
    "status": "healthy",
    "last_check": "2024-01-15T14:30:25Z",
    "check_type": "http",
    "url": "http://localhost:3000/health",
    "success_rate": 98.5,
    "total_checks": 200,
    "successful_checks": 197
  }
}
```

## WebSocket API

### Connection

Connect to the WebSocket endpoint for real-time updates:

```javascript
const ws = new WebSocket('ws://localhost:9615/ws');

ws.onopen = function() {
    console.log('Connected to PMDaemon WebSocket');
};

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    console.log('Received update:', data);
};
```

### Message Types

#### Process Status Updates

```json
{
  "type": "process_update",
  "data": {
    "id": 0,
    "name": "web-api",
    "status": "online",
    "pid": 1234,
    "cpu": 2.5,
    "memory": 47185920,
    "timestamp": "2024-01-15T14:30:25Z"
  }
}
```

#### System Metrics Updates

```json
{
  "type": "system_update",
  "data": {
    "cpu": 15.2,
    "memory": {
      "usage_percent": 25.0,
      "used": 2147483648
    },
    "load": {
      "one": 0.85
    },
    "timestamp": "2024-01-15T14:30:25Z"
  }
}
```

#### Health Check Updates

```json
{
  "type": "health_update",
  "data": {
    "process": "web-api",
    "status": "healthy",
    "timestamp": "2024-01-15T14:30:25Z"
  }
}
```

#### Process Events

```json
{
  "type": "process_event",
  "data": {
    "event": "started",
    "process": "new-api",
    "timestamp": "2024-01-15T14:30:25Z"
  }
}
```

## Client Examples

### JavaScript/Node.js

```javascript
// REST API client
const axios = require('axios');

class PMDaemonClient {
  constructor(baseURL = 'http://localhost:9615') {
    this.api = axios.create({ baseURL });
  }

  async listProcesses() {
    const response = await this.api.get('/api/processes');
    return response.data;
  }

  async startProcess(config) {
    const response = await this.api.post('/api/processes', config);
    return response.data;
  }

  async stopProcess(nameOrId) {
    const response = await this.api.delete(`/api/processes/${nameOrId}`);
    return response.data;
  }

  async getSystemMetrics() {
    const response = await this.api.get('/api/system');
    return response.data;
  }
}

// Usage
const client = new PMDaemonClient();
const processes = await client.listProcesses();
console.log(processes);
```

### Python

```python
import requests
import websocket
import json

class PMDaemonClient:
    def __init__(self, base_url='http://localhost:9615'):
        self.base_url = base_url

    def list_processes(self):
        response = requests.get(f'{self.base_url}/api/processes')
        return response.json()

    def start_process(self, config):
        response = requests.post(f'{self.base_url}/api/processes', json=config)
        return response.json()

    def stop_process(self, name_or_id):
        response = requests.delete(f'{self.base_url}/api/processes/{name_or_id}')
        return response.json()

    def get_system_metrics(self):
        response = requests.get(f'{self.base_url}/api/system')
        return response.json()

# WebSocket client
def on_message(ws, message):
    data = json.loads(message)
    print(f"Received: {data}")

def on_error(ws, error):
    print(f"Error: {error}")

def on_close(ws, close_status_code, close_msg):
    print("Connection closed")

# Connect to WebSocket
ws = websocket.WebSocketApp("ws://localhost:9615/ws",
                          on_message=on_message,
                          on_error=on_error,
                          on_close=on_close)
ws.run_forever()
```

### cURL Examples

```bash
# List all processes
curl http://localhost:9615/api/processes

# Start a new process
curl -X POST http://localhost:9615/api/processes \
  -H "Content-Type: application/json" \
  -d '{
    "name": "test-api",
    "script": "node",
    "args": ["server.js"],
    "port": "3000"
  }'

# Get system metrics
curl http://localhost:9615/api/system

# Get process logs
curl "http://localhost:9615/api/logs/test-api?lines=100"

# Stop a process
curl -X DELETE http://localhost:9615/api/processes/test-api
```

## Error Handling

### HTTP Status Codes

| Code | Meaning | Description |
|------|---------|-------------|
| 200 | OK | Request successful |
| 201 | Created | Process created successfully |
| 400 | Bad Request | Invalid request parameters |
| 404 | Not Found | Process not found |
| 409 | Conflict | Process name already exists |
| 500 | Internal Server Error | Server error |

### Error Response Format

```json
{
  "success": false,
  "error": {
    "code": "PROCESS_NOT_FOUND",
    "message": "Process 'nonexistent' not found",
    "details": {
      "requested_process": "nonexistent",
      "available_processes": ["web-api", "worker"]
    }
  }
}
```

## Security Features

### CORS Support

PMDaemon includes built-in CORS support for web applications:

```http
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, DELETE, OPTIONS
Access-Control-Allow-Headers: Content-Type, Authorization
```

### Security Headers

Standard security headers are included:

```http
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
X-XSS-Protection: 1; mode=block
```

### Authentication (Future)

Authentication features planned for future releases:
- API key authentication
- JWT token support
- Role-based access control

## Performance

### Benchmarks

The Axum-based web server provides excellent performance:
- **Concurrent connections**: 10,000+
- **Request throughput**: 50,000+ req/s
- **Memory usage**: Low overhead
- **WebSocket connections**: 1,000+ concurrent

### Optimization Tips

1. **Use WebSocket for real-time data**:
   ```javascript
   // Efficient for live updates
   const ws = new WebSocket('ws://localhost:9615/ws');
   ```

2. **Batch API requests**:
   ```javascript
   // Get all data in one request
   const [processes, system] = await Promise.all([
     client.listProcesses(),
     client.getSystemMetrics()
   ]);
   ```

3. **Limit log retrieval**:
   ```bash
   # Don't retrieve too many log lines
   curl "http://localhost:9615/api/logs/app?lines=100"
   ```

## Next Steps

- **[WebSocket API](../api/websocket-api.md)** - Detailed WebSocket documentation
- **[API Examples](../api/api-examples.md)** - More client examples
- **[Security](../advanced/security.md)** - Security best practices
- **[Integration Examples](../examples/integration-examples.md)** - Framework integration
