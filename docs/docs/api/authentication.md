# API Authentication

PMDaemon provides API key authentication to secure your process management API from unauthorized access.

## Overview

PMDaemon uses a simple but effective API key authentication system:
- **Optional by default** - API runs without authentication if no key is set
- **Global API key** - Single key protects all API endpoints  
- **Multiple header formats** - Flexible authentication methods
- **Public endpoints** - Root and WebSocket endpoints remain public

## Configuration

### Enable Authentication

Start PMDaemon with an API key to enable authentication:

```bash
# Start web server with API key authentication
pmdaemon web --api-key "your-secret-api-key"

# Custom host/port with authentication
pmdaemon web --host 0.0.0.0 --port 8080 --api-key "your-secret-key"
```

### Environment Variable

You can also use an environment variable:

```bash
export PMDAEMON_API_KEY="your-secret-api-key"
pmdaemon web
```

## Using Authentication

When an API key is configured, all API endpoints require authentication except:
- Root endpoint: `GET /`
- WebSocket endpoint: `GET /ws`

### Authentication Headers

PMDaemon accepts three authentication header formats:

#### 1. Bearer Token (Recommended)
```bash
curl -H "Authorization: Bearer your-secret-api-key" \
     http://localhost:9615/api/processes
```

#### 2. X-API-Key Header
```bash
curl -H "X-API-Key: your-secret-api-key" \
     http://localhost:9615/api/processes
```

#### 3. ApiKey Authorization
```bash
curl -H "Authorization: ApiKey your-secret-api-key" \
     http://localhost:9615/api/processes
```

## Examples

### JavaScript/Node.js

```javascript
const API_KEY = 'your-secret-api-key';
const BASE_URL = 'http://localhost:9615';

// Using fetch with Bearer token
const response = await fetch(`${BASE_URL}/api/processes`, {
  headers: {
    'Authorization': `Bearer ${API_KEY}`
  }
});

// Using fetch with X-API-Key header
const response2 = await fetch(`${BASE_URL}/api/processes`, {
  headers: {
    'X-API-Key': API_KEY
  }
});

const processes = await response.json();
```

### Python

```python
import requests

API_KEY = 'your-secret-api-key'
BASE_URL = 'http://localhost:9615'

# Using Authorization header
headers = {'Authorization': f'Bearer {API_KEY}'}
response = requests.get(f'{BASE_URL}/api/processes', headers=headers)

# Using X-API-Key header
headers = {'X-API-Key': API_KEY}
response = requests.get(f'{BASE_URL}/api/processes', headers=headers)

processes = response.json()
```

### cURL Examples

```bash
# List all processes
curl -H "Authorization: Bearer your-api-key" \
     http://localhost:9615/api/processes

# Get specific process
curl -H "X-API-Key: your-api-key" \
     http://localhost:9615/api/processes/my-app

# Stop a process
curl -X POST \
     -H "Authorization: Bearer your-api-key" \
     http://localhost:9615/api/processes/my-app/stop

# Get system information
curl -H "X-API-Key: your-api-key" \
     http://localhost:9615/api/system
```

## Error Responses

### 401 Unauthorized

When authentication fails, the API returns a `401 Unauthorized` status:

```json
{
  "error": "Unauthorized",
  "message": "Invalid or missing API key"
}
```

Common causes:
- Missing authentication header
- Incorrect API key
- Wrong header format

### 403 Forbidden

If authentication succeeds but access is denied:

```json
{
  "error": "Forbidden", 
  "message": "Access denied"
}
```

## Security Best Practices

### API Key Management

1. **Use strong keys**: Generate random, long API keys
   ```bash
   # Generate a secure API key
   openssl rand -hex 32
   ```

2. **Keep keys secret**: Never expose API keys in:
   - Client-side code
   - Version control
   - Log files
   - Error messages

3. **Rotate keys regularly**: Change API keys periodically

### Network Security

1. **Use HTTPS**: Always use HTTPS in production
   ```bash
   # Behind reverse proxy with SSL
   pmdaemon web --host 127.0.0.1 --api-key "$API_KEY"
   ```

2. **Restrict access**: Limit network access
   ```bash
   # Bind to localhost only
   pmdaemon web --host 127.0.0.1 --api-key "$API_KEY"
   
   # Use firewall rules
   iptables -A INPUT -p tcp --dport 9615 -s 10.0.0.0/8 -j ACCEPT
   ```

3. **Monitor access**: Log and monitor API usage

### Deployment Recommendations

#### Development
```bash
# No authentication for local development
pmdaemon web --host 127.0.0.1
```

#### Staging/Production
```bash
# Always use authentication
pmdaemon web --host 127.0.0.1 --api-key "$API_KEY"
```

#### Docker
```dockerfile
ENV PMDAEMON_API_KEY=your-secret-key
EXPOSE 9615
CMD ["pmdaemon", "web", "--host", "0.0.0.0"]
```

## Troubleshooting

### Connection Refused
```bash
# Check if server is running
curl http://localhost:9615/

# Check logs
pmdaemon web --api-key "key" --verbose
```

### Authentication Failures
```bash
# Test without authentication (if no key set)
curl http://localhost:9615/api/processes

# Test with authentication
curl -H "X-API-Key: your-key" http://localhost:9615/api/processes

# Verify key format
echo "API_KEY: $API_KEY"
```

### Common Issues

1. **Missing headers**: Ensure authentication header is included
2. **Wrong endpoint**: Authentication not required for `/` and `/ws`
3. **Key format**: Check for extra spaces or special characters
4. **Server configuration**: Verify API key was set when starting server

## Migration Guide

### From Unauthenticated to Authenticated

1. **Update clients** to include authentication headers
2. **Start server** with API key
3. **Test endpoints** with new authentication
4. **Update monitoring** tools and scripts

### Example Migration

Before:
```bash
# Server
pmdaemon web

# Client
curl http://localhost:9615/api/processes
```

After:
```bash
# Server  
pmdaemon web --api-key "secure-key-123"

# Client
curl -H "Authorization: Bearer secure-key-123" \
     http://localhost:9615/api/processes
```

## Next Steps

- **[REST API Reference](./rest-api.md)** - Complete API documentation
- **[WebSocket API](./websocket-api.md)** - Real-time updates
- **[Error Handling](./error-handling.md)** - Handle API errors
- **[Security Guide](../advanced/security.md)** - Advanced security topics
