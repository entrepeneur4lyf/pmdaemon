# Error Handling

PMDaemon's REST API provides comprehensive error handling with detailed error messages, status codes, and troubleshooting guidance.

## Error Response Format

All API errors follow a consistent JSON format:

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error description",
    "details": {
      "field": "Additional context",
      "suggestion": "Recommended action"
    },
    "timestamp": "2025-05-27T10:30:00Z"
  }
}
```

## HTTP Status Codes

### 2xx Success
- **200 OK** - Request successful
- **201 Created** - Resource created successfully
- **204 No Content** - Request successful, no content returned

### 4xx Client Errors
- **400 Bad Request** - Invalid request syntax or parameters
- **401 Unauthorized** - Authentication required
- **403 Forbidden** - Insufficient permissions
- **404 Not Found** - Resource not found
- **409 Conflict** - Resource conflict (e.g., process already exists)
- **422 Unprocessable Entity** - Valid syntax but semantic errors

### 5xx Server Errors
- **500 Internal Server Error** - Unexpected server error
- **503 Service Unavailable** - Server temporarily unavailable

## Common Error Codes

### Process Management Errors

#### PROCESS_NOT_FOUND
```json
{
  "success": false,
  "error": {
    "code": "PROCESS_NOT_FOUND",
    "message": "Process with ID 'web-app' not found",
    "details": {
      "process_id": "web-app",
      "suggestion": "Check 'pmdaemon list' for available processes"
    }
  }
}
```

#### PROCESS_ALREADY_RUNNING
```json
{
  "success": false,
  "error": {
    "code": "PROCESS_ALREADY_RUNNING",
    "message": "Process 'web-app' is already running",
    "details": {
      "process_id": "web-app",
      "pid": 12345,
      "suggestion": "Use restart endpoint to restart the process"
    }
  }
}
```

#### PROCESS_START_FAILED
```json
{
  "success": false,
  "error": {
    "code": "PROCESS_START_FAILED",
    "message": "Failed to start process 'web-app'",
    "details": {
      "process_id": "web-app",
      "reason": "Command not found: node",
      "suggestion": "Verify the command and working directory"
    }
  }
}
```

### Configuration Errors

#### INVALID_CONFIG
```json
{
  "success": false,
  "error": {
    "code": "INVALID_CONFIG",
    "message": "Invalid ecosystem configuration",
    "details": {
      "field": "apps[0].script",
      "value": null,
      "suggestion": "Script field is required for each application"
    }
  }
}
```

#### CONFIG_PARSE_ERROR
```json
{
  "success": false,
  "error": {
    "code": "CONFIG_PARSE_ERROR",
    "message": "Failed to parse configuration file",
    "details": {
      "file": "ecosystem.json",
      "line": 15,
      "suggestion": "Check JSON syntax near line 15"
    }
  }
}
```

### Resource Errors

#### PORT_IN_USE
```json
{
  "success": false,
  "error": {
    "code": "PORT_IN_USE",
    "message": "Port 3000 is already in use",
    "details": {
      "port": 3000,
      "suggestion": "Use a different port or stop the conflicting process"
    }
  }
}
```

#### INSUFFICIENT_RESOURCES
```json
{
  "success": false,
  "error": {
    "code": "INSUFFICIENT_RESOURCES",
    "message": "Insufficient system resources",
    "details": {
      "resource": "memory",
      "available": "512MB",
      "required": "1GB",
      "suggestion": "Free up memory or reduce process requirements"
    }
  }
}
```

## Error Handling Best Practices

### 1. Check Response Status
Always check the HTTP status code before processing the response:

```javascript
const response = await fetch('/api/processes');
if (!response.ok) {
  const error = await response.json();
  console.error('API Error:', error.error.message);
  return;
}
```

### 2. Handle Specific Error Codes
Implement specific handling for known error codes:

```javascript
async function startProcess(processId) {
  try {
    const response = await fetch(`/api/processes/${processId}/start`, {
      method: 'POST'
    });

    if (!response.ok) {
      const error = await response.json();

      switch (error.error.code) {
        case 'PROCESS_ALREADY_RUNNING':
          console.log('Process is already running');
          break;
        case 'PROCESS_NOT_FOUND':
          console.error('Process not found:', processId);
          break;
        case 'PROCESS_START_FAILED':
          console.error('Start failed:', error.error.details.reason);
          break;
        default:
          console.error('Unknown error:', error.error.message);
      }
    }
  } catch (err) {
    console.error('Network error:', err);
  }
}
```

### 3. Implement Retry Logic
For transient errors, implement exponential backoff:

```javascript
async function apiCallWithRetry(url, options, maxRetries = 3) {
  for (let i = 0; i < maxRetries; i++) {
    try {
      const response = await fetch(url, options);

      if (response.ok) {
        return response;
      }

      // Don't retry client errors (4xx)
      if (response.status >= 400 && response.status < 500) {
        throw new Error(`Client error: ${response.status}`);
      }

      // Retry server errors (5xx) with backoff
      if (i < maxRetries - 1) {
        await new Promise(resolve => setTimeout(resolve, Math.pow(2, i) * 1000));
      }
    } catch (err) {
      if (i === maxRetries - 1) throw err;
    }
  }
}
```

### 4. Log Errors Appropriately
Log errors with context for debugging:

```javascript
function logApiError(endpoint, error, context = {}) {
  console.error('API Error:', {
    endpoint,
    code: error.error.code,
    message: error.error.message,
    details: error.error.details,
    context,
    timestamp: new Date().toISOString()
  });
}
```

## Troubleshooting Guide

### Connection Issues
- **Cannot connect to API**: Verify PMDaemon web server is running
- **CORS errors**: Check if requests are from allowed origins
- **Timeout errors**: Increase request timeout or check server load

### Authentication Issues
- **401 Unauthorized**: Verify API key or authentication token
- **403 Forbidden**: Check user permissions for the requested operation

### Process Issues
- **Process won't start**: Check command, working directory, and environment
- **Process keeps crashing**: Review logs and error output
- **Resource conflicts**: Check port availability and system resources

### Configuration Issues
- **Config validation fails**: Verify JSON/YAML/TOML syntax
- **Missing required fields**: Check against the ecosystem schema
- **Environment variables**: Ensure all required env vars are set

## Error Recovery Strategies

### Automatic Recovery
PMDaemon includes built-in recovery mechanisms:
- **Auto-restart**: Crashed processes are automatically restarted
- **Health checks**: Failed health checks trigger restarts
- **Resource monitoring**: Processes exceeding limits are managed

### Manual Recovery
For manual intervention:
1. Check process status: `GET /api/processes`
2. Review logs: `GET /api/processes/{id}/logs`
3. Restart if needed: `POST /api/processes/{id}/restart`
4. Update configuration if required

### Prevention
- Use health checks to detect issues early
- Set appropriate resource limits
- Monitor system metrics
- Implement proper logging and alerting

## Related Documentation

- **[REST API Reference](./rest-api.md)** - Complete API documentation
- **[WebSocket API](./websocket-api.md)** - Real-time events and monitoring
- **[Authentication](./authentication.md)** - API security and authentication
