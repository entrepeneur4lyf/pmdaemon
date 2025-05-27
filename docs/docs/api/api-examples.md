# API Examples

This guide provides practical examples of using PMDaemon's REST and WebSocket APIs in various programming languages and scenarios. From simple process management to complex monitoring dashboards, these examples will help you integrate PMDaemon into your applications.

## REST API Examples

### JavaScript/Node.js

#### Basic Process Management

```javascript
const axios = require('axios');

class PMDaemonClient {
  constructor(baseURL = 'http://localhost:9615') {
    this.api = axios.create({
      baseURL,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }

  async listProcesses() {
    try {
      const response = await this.api.get('/api/processes');
      return response.data;
    } catch (error) {
      throw new Error(`Failed to list processes: ${error.message}`);
    }
  }

  async startProcess(config) {
    try {
      const response = await this.api.post('/api/processes', config);
      return response.data;
    } catch (error) {
      throw new Error(`Failed to start process: ${error.message}`);
    }
  }

  async stopProcess(nameOrId) {
    try {
      const response = await this.api.delete(`/api/processes/${nameOrId}`);
      return response.data;
    } catch (error) {
      throw new Error(`Failed to stop process: ${error.message}`);
    }
  }

  async restartProcess(nameOrId, config = {}) {
    try {
      const response = await this.api.post(`/api/processes/${nameOrId}/restart`, config);
      return response.data;
    } catch (error) {
      throw new Error(`Failed to restart process: ${error.message}`);
    }
  }

  async getProcessInfo(nameOrId) {
    try {
      const response = await this.api.get(`/api/processes/${nameOrId}`);
      return response.data;
    } catch (error) {
      throw new Error(`Failed to get process info: ${error.message}`);
    }
  }

  async getSystemMetrics() {
    try {
      const response = await this.api.get('/api/system');
      return response.data;
    } catch (error) {
      throw new Error(`Failed to get system metrics: ${error.message}`);
    }
  }

  async getProcessLogs(nameOrId, lines = 20, type = 'all') {
    try {
      const response = await this.api.get(`/api/logs/${nameOrId}`, {
        params: { lines, type }
      });
      return response.data;
    } catch (error) {
      throw new Error(`Failed to get process logs: ${error.message}`);
    }
  }
}

// Usage example
async function main() {
  const client = new PMDaemonClient();

  try {
    // Start a new process
    const startResult = await client.startProcess({
      name: 'test-api',
      script: 'node',
      args: ['server.js'],
      port: '3000',
      env: {
        NODE_ENV: 'production'
      }
    });
    console.log('Process started:', startResult);

    // Wait a moment for the process to start
    await new Promise(resolve => setTimeout(resolve, 2000));

    // Get process information
    const processInfo = await client.getProcessInfo('test-api');
    console.log('Process info:', processInfo);

    // Get system metrics
    const systemMetrics = await client.getSystemMetrics();
    console.log('System metrics:', systemMetrics);

    // Get process logs
    const logs = await client.getProcessLogs('test-api', 50);
    console.log('Process logs:', logs);

  } catch (error) {
    console.error('Error:', error.message);
  }
}

main();
```

#### Deployment Automation

```javascript
const PMDaemonClient = require('./pmdaemon-client');

class DeploymentManager {
  constructor(apiUrl) {
    this.client = new PMDaemonClient(apiUrl);
  }

  async deployService(serviceName, config, healthCheckUrl) {
    console.log(`🚀 Deploying ${serviceName}...`);

    try {
      // Stop existing service if it exists
      try {
        await this.client.stopProcess(serviceName);
        console.log(`✅ Stopped existing ${serviceName}`);
        await this.waitForProcessStop(serviceName);
      } catch (error) {
        // Process might not exist, continue
      }

      // Start new service
      const startResult = await this.client.startProcess({
        name: serviceName,
        ...config
      });

      if (!startResult.success) {
        throw new Error(`Failed to start ${serviceName}: ${startResult.message}`);
      }

      // Wait for service to be healthy
      if (healthCheckUrl) {
        await this.waitForHealthy(serviceName, healthCheckUrl);
      }

      console.log(`✅ ${serviceName} deployed successfully`);
      return true;

    } catch (error) {
      console.error(`❌ Failed to deploy ${serviceName}:`, error.message);
      
      // Get logs for debugging
      try {
        const logs = await this.client.getProcessLogs(serviceName, 20);
        console.log('Recent logs:', logs.logs.map(log => log.message).join('\n'));
      } catch (logError) {
        console.error('Could not retrieve logs:', logError.message);
      }

      throw error;
    }
  }

  async waitForProcessStop(serviceName, timeout = 30000) {
    const startTime = Date.now();
    
    while (Date.now() - startTime < timeout) {
      try {
        await this.client.getProcessInfo(serviceName);
        await new Promise(resolve => setTimeout(resolve, 1000));
      } catch (error) {
        // Process not found, it's stopped
        return;
      }
    }
    
    throw new Error(`Process ${serviceName} did not stop within ${timeout}ms`);
  }

  async waitForHealthy(serviceName, healthCheckUrl, timeout = 60000) {
    const startTime = Date.now();
    
    while (Date.now() - startTime < timeout) {
      try {
        const processInfo = await this.client.getProcessInfo(serviceName);
        
        if (processInfo.status === 'online' && processInfo.health === 'healthy') {
          return;
        }
        
        await new Promise(resolve => setTimeout(resolve, 2000));
      } catch (error) {
        await new Promise(resolve => setTimeout(resolve, 2000));
      }
    }
    
    throw new Error(`Service ${serviceName} did not become healthy within ${timeout}ms`);
  }

  async blueGreenDeploy(serviceName, config, healthCheckUrl) {
    const currentColor = await this.getCurrentColor(serviceName);
    const newColor = currentColor === 'blue' ? 'green' : 'blue';
    const newServiceName = `${serviceName}-${newColor}`;

    console.log(`🔄 Blue-green deployment: ${currentColor} -> ${newColor}`);

    // Deploy to new color
    await this.deployService(newServiceName, config, healthCheckUrl);

    // Switch traffic (this would integrate with your load balancer)
    console.log(`🔀 Switching traffic to ${newColor}`);
    
    // Wait for traffic to drain
    await new Promise(resolve => setTimeout(resolve, 10000));

    // Stop old color
    if (currentColor) {
      const oldServiceName = `${serviceName}-${currentColor}`;
      await this.client.stopProcess(oldServiceName);
      console.log(`🛑 Stopped old deployment: ${oldServiceName}`);
    }

    console.log(`✅ Blue-green deployment complete`);
  }

  async getCurrentColor(serviceName) {
    try {
      const processes = await this.client.listProcesses();
      const blueExists = processes.processes.some(p => p.name === `${serviceName}-blue`);
      const greenExists = processes.processes.some(p => p.name === `${serviceName}-green`);
      
      if (blueExists) return 'blue';
      if (greenExists) return 'green';
      return null;
    } catch (error) {
      return null;
    }
  }
}

// Usage
async function deployExample() {
  const deployer = new DeploymentManager('http://localhost:9615');

  const serviceConfig = {
    script: 'node',
    args: ['dist/server.js'],
    instances: 2,
    port: '3000-3001',
    env: {
      NODE_ENV: 'production',
      DATABASE_URL: 'postgres://localhost/myapp'
    },
    health_check: {
      check_type: 'http',
      url: 'http://localhost:3000/health',
      enabled: true
    }
  };

  await deployer.blueGreenDeploy('web-api', serviceConfig, 'http://localhost:3000/health');
}

deployExample().catch(console.error);
```

### Python

#### Process Management with Error Handling

```python
import requests
import time
import json
from typing import Dict, List, Optional

class PMDaemonClient:
    def __init__(self, base_url: str = "http://localhost:9615"):
        self.base_url = base_url.rstrip('/')
        self.session = requests.Session()
        self.session.headers.update({
            'Content-Type': 'application/json'
        })

    def _request(self, method: str, endpoint: str, **kwargs) -> Dict:
        url = f"{self.base_url}{endpoint}"
        try:
            response = self.session.request(method, url, **kwargs)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            raise Exception(f"API request failed: {e}")

    def list_processes(self) -> Dict:
        return self._request('GET', '/api/processes')

    def start_process(self, config: Dict) -> Dict:
        return self._request('POST', '/api/processes', json=config)

    def stop_process(self, name_or_id: str) -> Dict:
        return self._request('DELETE', f'/api/processes/{name_or_id}')

    def restart_process(self, name_or_id: str, config: Optional[Dict] = None) -> Dict:
        return self._request('POST', f'/api/processes/{name_or_id}/restart', 
                           json=config or {})

    def get_process_info(self, name_or_id: str) -> Dict:
        return self._request('GET', f'/api/processes/{name_or_id}')

    def get_system_metrics(self) -> Dict:
        return self._request('GET', '/api/system')

    def get_process_logs(self, name_or_id: str, lines: int = 20, 
                        log_type: str = 'all') -> Dict:
        params = {'lines': lines, 'type': log_type}
        return self._request('GET', f'/api/logs/{name_or_id}', params=params)

    def wait_for_process_status(self, name_or_id: str, expected_status: str, 
                               timeout: int = 30) -> bool:
        """Wait for a process to reach a specific status"""
        start_time = time.time()
        
        while time.time() - start_time < timeout:
            try:
                info = self.get_process_info(name_or_id)
                if info.get('status') == expected_status:
                    return True
            except Exception:
                pass
            time.sleep(1)
        
        return False

    def wait_for_healthy(self, name_or_id: str, timeout: int = 60) -> bool:
        """Wait for a process to become healthy"""
        start_time = time.time()
        
        while time.time() - start_time < timeout:
            try:
                info = self.get_process_info(name_or_id)
                if (info.get('status') == 'online' and 
                    info.get('health') == 'healthy'):
                    return True
            except Exception:
                pass
            time.sleep(2)
        
        return False

class ProcessMonitor:
    def __init__(self, client: PMDaemonClient):
        self.client = client

    def check_process_health(self, process_name: str) -> Dict:
        """Check the health of a specific process"""
        try:
            info = self.client.get_process_info(process_name)
            
            health_status = {
                'name': process_name,
                'status': info.get('status'),
                'health': info.get('health'),
                'cpu': info.get('cpu'),
                'memory': info.get('memory'),
                'uptime': info.get('uptime'),
                'restarts': info.get('restarts'),
                'healthy': info.get('status') == 'online' and info.get('health') == 'healthy'
            }
            
            return health_status
            
        except Exception as e:
            return {
                'name': process_name,
                'status': 'unknown',
                'healthy': False,
                'error': str(e)
            }

    def monitor_processes(self, process_names: List[str]) -> Dict:
        """Monitor multiple processes and return their health status"""
        results = {}
        
        for process_name in process_names:
            results[process_name] = self.check_process_health(process_name)
        
        return results

    def restart_unhealthy_processes(self, process_names: List[str]) -> Dict:
        """Restart any unhealthy processes"""
        results = {}
        
        for process_name in process_names:
            health = self.check_process_health(process_name)
            
            if not health.get('healthy', False):
                print(f"🔄 Restarting unhealthy process: {process_name}")
                try:
                    restart_result = self.client.restart_process(process_name)
                    
                    # Wait for process to become healthy
                    if self.client.wait_for_healthy(process_name, timeout=60):
                        results[process_name] = {'status': 'restarted', 'healthy': True}
                        print(f"✅ {process_name} restarted successfully")
                    else:
                        results[process_name] = {'status': 'restart_failed', 'healthy': False}
                        print(f"❌ {process_name} restart failed")
                        
                except Exception as e:
                    results[process_name] = {'status': 'error', 'error': str(e)}
                    print(f"❌ Error restarting {process_name}: {e}")
            else:
                results[process_name] = {'status': 'healthy', 'healthy': True}
        
        return results

# Usage example
def main():
    client = PMDaemonClient()
    monitor = ProcessMonitor(client)

    # Deploy a new service
    service_config = {
        'name': 'python-api',
        'script': 'python',
        'args': ['-m', 'uvicorn', 'main:app', '--host', '0.0.0.0', '--port', '8000'],
        'port': '8000',
        'env': {
            'PYTHONPATH': '/app',
            'DATABASE_URL': 'postgres://localhost/myapp'
        },
        'health_check': {
            'check_type': 'http',
            'url': 'http://localhost:8000/health',
            'timeout': 10,
            'interval': 30,
            'enabled': True
        }
    }

    try:
        # Start the service
        print("🚀 Starting Python API service...")
        result = client.start_process(service_config)
        print(f"Start result: {result}")

        # Wait for it to become healthy
        if client.wait_for_healthy('python-api', timeout=60):
            print("✅ Service is healthy")
        else:
            print("❌ Service failed to become healthy")
            logs = client.get_process_logs('python-api', lines=20)
            print("Recent logs:")
            for log in logs.get('logs', []):
                print(f"  {log.get('message', '')}")

        # Monitor the service
        health_status = monitor.check_process_health('python-api')
        print(f"Health status: {health_status}")

    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
```

### Go

#### Simple Process Management

```go
package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "io"
    "net/http"
    "time"
)

type PMDaemonClient struct {
    BaseURL string
    Client  *http.Client
}

type ProcessConfig struct {
    Name      string            `json:"name"`
    Script    string            `json:"script"`
    Args      []string          `json:"args,omitempty"`
    Port      string            `json:"port,omitempty"`
    Instances int               `json:"instances,omitempty"`
    Env       map[string]string `json:"env,omitempty"`
}

type ProcessInfo struct {
    ID       int    `json:"id"`
    Name     string `json:"name"`
    Status   string `json:"status"`
    PID      int    `json:"pid"`
    Port     int    `json:"port"`
    CPU      float64 `json:"cpu"`
    Memory   int64  `json:"memory"`
    Uptime   int64  `json:"uptime"`
    Restarts int    `json:"restarts"`
    Health   string `json:"health"`
}

type ProcessListResponse struct {
    Processes []ProcessInfo `json:"processes"`
    Total     int           `json:"total"`
}

func NewPMDaemonClient(baseURL string) *PMDaemonClient {
    return &PMDaemonClient{
        BaseURL: baseURL,
        Client: &http.Client{
            Timeout: 30 * time.Second,
        },
    }
}

func (c *PMDaemonClient) request(method, endpoint string, body interface{}) (*http.Response, error) {
    var reqBody io.Reader
    
    if body != nil {
        jsonData, err := json.Marshal(body)
        if err != nil {
            return nil, err
        }
        reqBody = bytes.NewBuffer(jsonData)
    }
    
    req, err := http.NewRequest(method, c.BaseURL+endpoint, reqBody)
    if err != nil {
        return nil, err
    }
    
    req.Header.Set("Content-Type", "application/json")
    
    return c.Client.Do(req)
}

func (c *PMDaemonClient) ListProcesses() (*ProcessListResponse, error) {
    resp, err := c.request("GET", "/api/processes", nil)
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()
    
    var result ProcessListResponse
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return nil, err
    }
    
    return &result, nil
}

func (c *PMDaemonClient) StartProcess(config ProcessConfig) error {
    resp, err := c.request("POST", "/api/processes", config)
    if err != nil {
        return err
    }
    defer resp.Body.Close()
    
    if resp.StatusCode != http.StatusOK && resp.StatusCode != http.StatusCreated {
        return fmt.Errorf("failed to start process: status %d", resp.StatusCode)
    }
    
    return nil
}

func (c *PMDaemonClient) StopProcess(nameOrID string) error {
    resp, err := c.request("DELETE", "/api/processes/"+nameOrID, nil)
    if err != nil {
        return err
    }
    defer resp.Body.Close()
    
    if resp.StatusCode != http.StatusOK {
        return fmt.Errorf("failed to stop process: status %d", resp.StatusCode)
    }
    
    return nil
}

func (c *PMDaemonClient) GetProcessInfo(nameOrID string) (*ProcessInfo, error) {
    resp, err := c.request("GET", "/api/processes/"+nameOrID, nil)
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()
    
    var result ProcessInfo
    if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
        return nil, err
    }
    
    return &result, nil
}

func (c *PMDaemonClient) WaitForHealthy(nameOrID string, timeout time.Duration) error {
    start := time.Now()
    
    for time.Since(start) < timeout {
        info, err := c.GetProcessInfo(nameOrID)
        if err == nil && info.Status == "online" && info.Health == "healthy" {
            return nil
        }
        
        time.Sleep(2 * time.Second)
    }
    
    return fmt.Errorf("process %s did not become healthy within %v", nameOrID, timeout)
}

func main() {
    client := NewPMDaemonClient("http://localhost:9615")
    
    // Start a new process
    config := ProcessConfig{
        Name:   "go-api",
        Script: "go",
        Args:   []string{"run", "main.go"},
        Port:   "8080",
        Env: map[string]string{
            "GO_ENV": "production",
        },
    }
    
    fmt.Println("🚀 Starting Go API service...")
    if err := client.StartProcess(config); err != nil {
        fmt.Printf("❌ Failed to start process: %v\n", err)
        return
    }
    
    // Wait for it to become healthy
    fmt.Println("⏳ Waiting for service to become healthy...")
    if err := client.WaitForHealthy("go-api", 60*time.Second); err != nil {
        fmt.Printf("❌ Service failed to become healthy: %v\n", err)
        return
    }
    
    fmt.Println("✅ Service is healthy!")
    
    // List all processes
    processes, err := client.ListProcesses()
    if err != nil {
        fmt.Printf("❌ Failed to list processes: %v\n", err)
        return
    }
    
    fmt.Printf("📋 Found %d processes:\n", processes.Total)
    for _, proc := range processes.Processes {
        fmt.Printf("  - %s (PID: %d, Status: %s, Health: %s)\n", 
                   proc.Name, proc.PID, proc.Status, proc.Health)
    }
}
```

## WebSocket API Examples

### Real-time Dashboard

```javascript
// dashboard.js
class PMDaemonDashboard {
  constructor(wsUrl = 'ws://localhost:9615/ws') {
    this.wsUrl = wsUrl;
    this.ws = null;
    this.processes = new Map();
    this.systemMetrics = {};
  }

  connect() {
    this.ws = new WebSocket(this.wsUrl);
    
    this.ws.onopen = () => {
      console.log('Connected to PMDaemon WebSocket');
      this.subscribe();
    };

    this.ws.onmessage = (event) => {
      const message = JSON.parse(event.data);
      this.handleMessage(message);
    };

    this.ws.onclose = () => {
      console.log('WebSocket connection closed');
      // Attempt to reconnect after 5 seconds
      setTimeout(() => this.connect(), 5000);
    };

    this.ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };
  }

  subscribe() {
    this.send('subscribe', {
      events: ['process_status', 'process_metrics', 'system_metrics', 'health_update'],
      interval: 1000
    });
  }

  send(type, data) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ type, data }));
    }
  }

  handleMessage(message) {
    switch (message.type) {
      case 'process_status':
        this.updateProcessStatus(message.data);
        break;
      case 'process_metrics':
        this.updateProcessMetrics(message.data);
        break;
      case 'system_metrics':
        this.updateSystemMetrics(message.data);
        break;
      case 'health_update':
        this.updateProcessHealth(message.data);
        break;
    }
  }

  updateProcessStatus(data) {
    const process = this.processes.get(data.name) || {};
    process.status = data.status;
    process.pid = data.pid;
    process.port = data.port;
    this.processes.set(data.name, process);
    this.renderProcesses();
  }

  updateProcessMetrics(data) {
    data.processes.forEach(proc => {
      const process = this.processes.get(proc.name) || {};
      Object.assign(process, proc);
      this.processes.set(proc.name, process);
    });
    this.renderProcesses();
  }

  updateSystemMetrics(data) {
    this.systemMetrics = data;
    this.renderSystemMetrics();
  }

  updateProcessHealth(data) {
    const process = this.processes.get(data.process) || {};
    process.health = data.status;
    this.processes.set(data.process, process);
    this.renderProcesses();
  }

  renderProcesses() {
    const container = document.getElementById('processes');
    container.innerHTML = Array.from(this.processes.values()).map(proc => `
      <div class="process-card ${proc.status} ${proc.health}">
        <h3>${proc.name}</h3>
        <div class="status">Status: ${proc.status}</div>
        <div class="health">Health: ${proc.health || 'unknown'}</div>
        <div class="metrics">
          <span>CPU: ${proc.cpu?.toFixed(1) || 0}%</span>
          <span>Memory: ${proc.memory ? (proc.memory / 1024 / 1024).toFixed(1) : 0}MB</span>
          <span>PID: ${proc.pid || 'N/A'}</span>
          <span>Port: ${proc.port || 'N/A'}</span>
        </div>
        <div class="actions">
          <button onclick="dashboard.restartProcess('${proc.name}')">Restart</button>
          <button onclick="dashboard.stopProcess('${proc.name}')">Stop</button>
        </div>
      </div>
    `).join('');
  }

  renderSystemMetrics() {
    const container = document.getElementById('system-metrics');
    const metrics = this.systemMetrics;
    
    container.innerHTML = `
      <div class="metric">
        <h4>CPU Usage</h4>
        <div class="value">${metrics.cpu?.usage?.toFixed(1) || 0}%</div>
      </div>
      <div class="metric">
        <h4>Memory Usage</h4>
        <div class="value">${metrics.memory?.usage_percent?.toFixed(1) || 0}%</div>
      </div>
      <div class="metric">
        <h4>Load Average</h4>
        <div class="value">${metrics.load?.one?.toFixed(2) || 0}</div>
      </div>
    `;
  }

  restartProcess(name) {
    this.send('command', {
      action: 'restart',
      process: name
    });
  }

  stopProcess(name) {
    this.send('command', {
      action: 'stop',
      process: name
    });
  }
}

// Initialize dashboard
const dashboard = new PMDaemonDashboard();
dashboard.connect();
```

## Next Steps

- **[Library Usage](./library-usage.md)** - Using PMDaemon as a Rust library
- **[WebSocket API](./websocket-api.md)** - Real-time WebSocket communication
- **[REST API](./rest-api.md)** - Complete HTTP API reference
- **[Integration Examples](../examples/integration-examples.md)** - Framework-specific integration
