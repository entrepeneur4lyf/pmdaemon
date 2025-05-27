# WebSocket API

PMDaemon's **WebSocket API** provides real-time communication for monitoring process status, receiving live updates, and managing processes interactively. Built for high performance and low latency, it's perfect for dashboards, monitoring tools, and real-time applications.

## Overview

The WebSocket API offers:

- **⚡ Real-time updates** - Instant process status changes
- **📊 Live metrics** - CPU, memory, and system metrics streaming
- **🔄 Bidirectional communication** - Send commands and receive responses
- **📱 Multiple clients** - Support for concurrent connections
- **🎯 Event filtering** - Subscribe to specific event types

## Connection

### Basic Connection

```javascript
const ws = new WebSocket('ws://localhost:9615/ws');

ws.onopen = function() {
    console.log('Connected to PMDaemon WebSocket');
};

ws.onmessage = function(event) {
    const data = JSON.parse(event.data);
    console.log('Received:', data);
};

ws.onclose = function() {
    console.log('WebSocket connection closed');
};

ws.onerror = function(error) {
    console.error('WebSocket error:', error);
};
```

### Connection with Authentication (Future)

```javascript
const ws = new WebSocket('ws://localhost:9615/ws', {
    headers: {
        'Authorization': 'Bearer your-jwt-token' // Node only
    }
});
```

## Message Format

All WebSocket messages use JSON format:

```json
{
  "type": "message_type",
  "data": {
    // Message-specific data
  },
  "timestamp": "2024-01-15T14:30:25.123Z",
  "id": "unique-message-id"
}
```

## Incoming Message Types

### Process Status Updates

Sent when process status changes:

```json
{
  "type": "process_status",
  "data": {
    "id": 0,
    "name": "web-api",
    "status": "online",
    "pid": 1234,
    "port": 3000,
    "previous_status": "starting"
  },
  "timestamp": "2024-01-15T14:30:25.123Z"
}
```

**Status values:**
- `starting` - Process is starting up
- `online` - Process running normally
- `stopping` - Process shutting down
- `stopped` - Process not running
- `errored` - Process crashed or failed
- `restarting` - Process restarting

### Process Metrics Updates

Real-time metrics for all processes:

```json
{
  "type": "process_metrics",
  "data": {
    "processes": [
      {
        "id": 0,
        "name": "web-api",
        "cpu": 2.5,
        "memory": 47185920,
        "uptime": 7890000,
        "restarts": 0
      },
      {
        "id": 1,
        "name": "worker",
        "cpu": 1.2,
        "memory": 32145920,
        "uptime": 5430000,
        "restarts": 1
      }
    ]
  },
  "timestamp": "2024-01-15T14:30:25.123Z"
}
```

### System Metrics Updates

System-wide performance metrics:

```json
{
  "type": "system_metrics",
  "data": {
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
    }
  },
  "timestamp": "2024-01-15T14:30:25.123Z"
}
```

### Health Check Updates

Health status changes:

```json
{
  "type": "health_update",
  "data": {
    "process": "web-api",
    "status": "healthy",
    "previous_status": "unhealthy",
    "check_type": "http",
    "details": {
      "url": "http://localhost:3000/health",
      "response_time": 45,
      "status_code": 200
    }
  },
  "timestamp": "2024-01-15T14:30:25.123Z"
}
```

### Process Events

Lifecycle events for processes:

```json
{
  "type": "process_event",
  "data": {
    "event": "started",
    "process": "new-api",
    "details": {
      "pid": 5678,
      "port": 3001,
      "startup_time": 1250
    }
  },
  "timestamp": "2024-01-15T14:30:25.123Z"
}
```

**Event types:**
- `started` - Process successfully started
- `stopped` - Process stopped
- `crashed` - Process crashed unexpectedly
- `restarted` - Process restarted
- `health_check_failed` - Health check failed
- `memory_limit_exceeded` - Memory limit reached

### Log Messages

Real-time log streaming:

```json
{
  "type": "log_message",
  "data": {
    "process": "web-api",
    "instance": 0,
    "level": "info",
    "message": "Server started on port 3000",
    "stream": "stdout"
  },
  "timestamp": "2024-01-15T14:30:25.123Z"
}
```

## Outgoing Message Types

### Subscribe to Events

Subscribe to specific event types:

```json
{
  "type": "subscribe",
  "data": {
    "events": ["process_status", "process_metrics"],
    "processes": ["web-api", "worker"],
    "interval": 1000
  }
}
```

**Subscription options:**
- `events` - Array of event types to receive
- `processes` - Array of process names (optional, defaults to all)
- `interval` - Update interval in milliseconds (for metrics)

### Unsubscribe from Events

```json
{
  "type": "unsubscribe",
  "data": {
    "events": ["system_metrics"]
  }
}
```

### Process Commands

Send process management commands:

```json
{
  "type": "command",
  "data": {
    "action": "start",
    "process": "new-service",
    "config": {
      "script": "node",
      "args": ["server.js"],
      "port": "3002"
    }
  }
}
```

**Available actions:**
- `start` - Start a new process
- `stop` - Stop a process
- `restart` - Restart a process
- `delete` - Delete a process
- `reload` - Reload process configuration

### Request Process Information

```json
{
  "type": "get_info",
  "data": {
    "process": "web-api"
  }
}
```

### Request System Information

```json
{
  "type": "get_system",
  "data": {}
}
```

## Client Examples

### JavaScript/Browser

```javascript
class PMDaemonWebSocket {
  constructor(url = 'ws://localhost:9615/ws') {
    this.url = url;
    this.ws = null;
    this.listeners = new Map();
  }

  connect() {
    this.ws = new WebSocket(this.url);
    
    this.ws.onopen = () => {
      console.log('Connected to PMDaemon');
      this.emit('connected');
    };

    this.ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      this.emit(message.type, message.data);
    };

    this.ws.onclose = () => {
      console.log('Disconnected from PMDaemon');
      this.emit('disconnected');
    };

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      this.emit('error', error);
    };
  }

  subscribe(events, processes = null, interval = 1000) {
    this.send('subscribe', {
      events,
      processes,
      interval
    });
  }

  startProcess(name, config) {
    this.send('command', {
      action: 'start',
      process: name,
      config
    });
  }

  stopProcess(name) {
    this.send('command', {
      action: 'stop',
      process: name
    });
  }

  send(type, data) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ type, data }));
    }
  }

  on(event, callback) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event).push(callback);
  }

  emit(event, data) {
    if (this.listeners.has(event)) {
      this.listeners.get(event).forEach(callback => callback(data));
    }
  }
}

// Usage
const pmdaemon = new PMDaemonWebSocket();

pmdaemon.on('connected', () => {
  // Subscribe to process status and metrics
  pmdaemon.subscribe(['process_status', 'process_metrics']);
});

pmdaemon.on('process_status', (data) => {
  console.log(`Process ${data.name} is now ${data.status}`);
});

pmdaemon.on('process_metrics', (data) => {
  data.processes.forEach(process => {
    console.log(`${process.name}: CPU ${process.cpu}%, Memory ${process.memory}`);
  });
});

pmdaemon.connect();
```

### Node.js

```javascript
const WebSocket = require('ws');

class PMDaemonClient {
  constructor(url = 'ws://localhost:9615/ws') {
    this.url = url;
    this.ws = null;
  }

  async connect() {
    return new Promise((resolve, reject) => {
      this.ws = new WebSocket(this.url);
      
      this.ws.on('open', () => {
        console.log('Connected to PMDaemon');
        resolve();
      });

      this.ws.on('message', (data) => {
        const message = JSON.parse(data.toString());
        this.handleMessage(message);
      });

      this.ws.on('close', () => {
        console.log('Disconnected from PMDaemon');
      });

      this.ws.on('error', (error) => {
        console.error('WebSocket error:', error);
        reject(error);
      });
    });
  }

  handleMessage(message) {
    switch (message.type) {
      case 'process_status':
        console.log(`Process ${message.data.name} status: ${message.data.status}`);
        break;
      case 'process_metrics':
        this.updateMetrics(message.data.processes);
        break;
      case 'system_metrics':
        this.updateSystemMetrics(message.data);
        break;
    }
  }

  subscribe(events, processes = null) {
    this.send('subscribe', { events, processes });
  }

  send(type, data) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ type, data }));
    }
  }
}

// Usage
async function main() {
  const client = new PMDaemonClient();
  await client.connect();
  
  // Subscribe to all events
  client.subscribe(['process_status', 'process_metrics', 'system_metrics']);
}

main().catch(console.error);
```

### Python

```python
import asyncio
import websockets
import json

class PMDaemonClient:
    def __init__(self, url="ws://localhost:9615/ws"):
        self.url = url
        self.websocket = None

    async def connect(self):
        self.websocket = await websockets.connect(self.url)
        print("Connected to PMDaemon")

    async def listen(self):
        async for message in self.websocket:
            data = json.loads(message)
            await self.handle_message(data)

    async def handle_message(self, message):
        msg_type = message.get('type')
        data = message.get('data')
        
        if msg_type == 'process_status':
            print(f"Process {data['name']} status: {data['status']}")
        elif msg_type == 'process_metrics':
            for process in data['processes']:
                print(f"{process['name']}: CPU {process['cpu']}%, Memory {process['memory']}")
        elif msg_type == 'system_metrics':
            cpu = data['cpu']['usage']
            memory = data['memory']['usage_percent']
            print(f"System: CPU {cpu}%, Memory {memory}%")

    async def subscribe(self, events, processes=None):
        message = {
            'type': 'subscribe',
            'data': {
                'events': events,
                'processes': processes
            }
        }
        await self.websocket.send(json.dumps(message))

    async def start_process(self, name, config):
        message = {
            'type': 'command',
            'data': {
                'action': 'start',
                'process': name,
                'config': config
            }
        }
        await self.websocket.send(json.dumps(message))

# Usage
async def main():
    client = PMDaemonClient()
    await client.connect()
    
    # Subscribe to events
    await client.subscribe(['process_status', 'process_metrics'])
    
    # Listen for messages
    await client.listen()

if __name__ == "__main__":
    asyncio.run(main())
```

## Real-time Dashboard Example

```html
<!DOCTYPE html>
<html>
<head>
    <title>PMDaemon Dashboard</title>
    <style>
        .process { margin: 10px; padding: 10px; border: 1px solid #ccc; }
        .online { background-color: #d4edda; }
        .stopped { background-color: #f8d7da; }
        .starting { background-color: #fff3cd; }
    </style>
</head>
<body>
    <h1>PMDaemon Real-time Dashboard</h1>
    <div id="processes"></div>
    <div id="system-metrics"></div>

    <script>
        const ws = new WebSocket('ws://localhost:9615/ws');
        const processesDiv = document.getElementById('processes');
        const systemDiv = document.getElementById('system-metrics');

        ws.onopen = function() {
            // Subscribe to all updates
            ws.send(JSON.stringify({
                type: 'subscribe',
                data: {
                    events: ['process_status', 'process_metrics', 'system_metrics'],
                    interval: 1000
                }
            }));
        };

        ws.onmessage = function(event) {
            const message = JSON.parse(event.data);
            
            switch (message.type) {
                case 'process_metrics':
                    updateProcesses(message.data.processes);
                    break;
                case 'system_metrics':
                    updateSystemMetrics(message.data);
                    break;
            }
        };

        function updateProcesses(processes) {
            processesDiv.innerHTML = processes.map(process => `
                <div class="process ${process.status}">
                    <h3>${process.name}</h3>
                    <p>Status: ${process.status}</p>
                    <p>CPU: ${process.cpu}%</p>
                    <p>Memory: ${(process.memory / 1024 / 1024).toFixed(1)}MB</p>
                    <p>Uptime: ${Math.floor(process.uptime / 1000)}s</p>
                </div>
            `).join('');
        }

        function updateSystemMetrics(metrics) {
            systemDiv.innerHTML = `
                <h3>System Metrics</h3>
                <p>CPU: ${metrics.cpu.usage}%</p>
                <p>Memory: ${metrics.memory.usage_percent}%</p>
                <p>Load: ${metrics.load.one}</p>
            `;
        }
    </script>
</body>
</html>
```

## Best Practices

### 1. Handle Connection Failures

```javascript
class RobustPMDaemonClient {
  constructor(url) {
    this.url = url;
    this.reconnectInterval = 5000;
    this.maxReconnectAttempts = 10;
    this.reconnectAttempts = 0;
  }

  connect() {
    this.ws = new WebSocket(this.url);
    
    this.ws.onopen = () => {
      this.reconnectAttempts = 0;
      this.onConnected();
    };

    this.ws.onclose = () => {
      this.reconnect();
    };
  }

  reconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      setTimeout(() => this.connect(), this.reconnectInterval);
    }
  }
}
```

### 2. Efficient Event Filtering

```javascript
// Subscribe only to needed events
ws.send(JSON.stringify({
  type: 'subscribe',
  data: {
    events: ['process_status'],  // Only status changes
    processes: ['critical-service'],  // Only critical processes
    interval: 5000  // Less frequent updates
  }
}));
```

### 3. Message Queuing

```javascript
class QueuedWebSocket {
  constructor(url) {
    this.url = url;
    this.messageQueue = [];
    this.connected = false;
  }

  send(message) {
    if (this.connected) {
      this.ws.send(JSON.stringify(message));
    } else {
      this.messageQueue.push(message);
    }
  }

  onConnected() {
    this.connected = true;
    // Send queued messages
    while (this.messageQueue.length > 0) {
      const message = this.messageQueue.shift();
      this.ws.send(JSON.stringify(message));
    }
  }
}
```

## Next Steps

- **[REST API](./rest-api.md)** - HTTP API reference
- **[API Examples](./api-examples.md)** - More client examples
- **[Library Usage](./library-usage.md)** - Using PMDaemon as a library
- **[Integration Examples](../examples/integration-examples.md)** - Framework integration
