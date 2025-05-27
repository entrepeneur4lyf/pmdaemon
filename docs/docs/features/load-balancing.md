# Load Balancing

PMDaemon provides built-in load balancing capabilities for distributing traffic across multiple application instances, ensuring high availability and optimal resource utilization.

## Load Balancing Strategies

### 1. Round Robin (Default)

Distributes requests evenly across all healthy instances in order.

```json
{
  "apps": [
    {
      "name": "web-app",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "load_balancer": {
        "strategy": "round_robin",
        "health_check": true
      }
    }
  ]
}
```

### 2. Least Connections

Routes requests to the instance with the fewest active connections.

```json
{
  "apps": [
    {
      "name": "api-server",
      "script": "api.js",
      "instances": 3,
      "exec_mode": "cluster",
      "load_balancer": {
        "strategy": "least_connections",
        "connection_tracking": true
      }
    }
  ]
}
```

### 3. Weighted Round Robin

Assigns different weights to instances based on their capacity.

```json
{
  "apps": [
    {
      "name": "high-capacity-server",
      "script": "server.js",
      "instances": 2,
      "exec_mode": "cluster",
      "weight": 3,
      "load_balancer": {
        "strategy": "weighted_round_robin"
      }
    },
    {
      "name": "standard-server",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "weight": 1,
      "load_balancer": {
        "strategy": "weighted_round_robin"
      }
    }
  ]
}
```

### 4. IP Hash

Routes requests based on client IP to ensure session affinity.

```json
{
  "apps": [
    {
      "name": "session-aware-app",
      "script": "app.js",
      "instances": 3,
      "exec_mode": "cluster",
      "load_balancer": {
        "strategy": "ip_hash",
        "hash_key": "client_ip",
        "session_affinity": true
      }
    }
  ]
}
```

## Advanced Load Balancing

### 1. Geographic Load Balancing

Route traffic based on client location for optimal latency.

```json
{
  "apps": [
    {
      "name": "us-east-servers",
      "script": "server.js",
      "instances": 3,
      "exec_mode": "cluster",
      "region": "us-east-1",
      "load_balancer": {
        "strategy": "geographic",
        "primary_region": true,
        "fallback_regions": ["us-west-1", "eu-west-1"]
      }
    },
    {
      "name": "us-west-servers",
      "script": "server.js",
      "instances": 2,
      "exec_mode": "cluster",
      "region": "us-west-1",
      "load_balancer": {
        "strategy": "geographic",
        "primary_region": false
      }
    }
  ]
}
```

### 2. Health-Aware Load Balancing

Automatically exclude unhealthy instances from load balancing.

```json
{
  "apps": [
    {
      "name": "resilient-app",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "health_check": {
        "enabled": true,
        "url": "http://localhost:{{PORT}}/health",
        "interval": 30,
        "timeout": 10,
        "retries": 3,
        "failure_threshold": 3
      },
      "load_balancer": {
        "strategy": "health_aware_round_robin",
        "exclude_unhealthy": true,
        "quarantine_duration": 300
      }
    }
  ]
}
```

### 3. Performance-Based Load Balancing

Route traffic based on instance performance metrics.

```json
{
  "apps": [
    {
      "name": "adaptive-app",
      "script": "server.js",
      "instances": 5,
      "exec_mode": "cluster",
      "load_balancer": {
        "strategy": "performance_based",
        "metrics": {
          "response_time": 0.4,
          "cpu_usage": 0.3,
          "memory_usage": 0.2,
          "error_rate": 0.1
        },
        "update_interval": 60
      },
      "monitoring": {
        "enabled": true,
        "metrics_collection": true
      }
    }
  ]
}
```

## HTTP Load Balancing

### 1. Built-in HTTP Load Balancer

PMDaemon includes a built-in HTTP load balancer for web applications.

```json
{
  "apps": [
    {
      "name": "web-servers",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 3000
      }
    }
  ],
  "load_balancer": {
    "enabled": true,
    "port": 8080,
    "strategy": "round_robin",
    "backend_ports": [3000, 3001, 3002, 3003],
    "health_check": {
      "enabled": true,
      "path": "/health",
      "interval": 30
    }
  }
}
```

### 2. Sticky Sessions

Maintain session affinity for stateful applications.

```json
{
  "load_balancer": {
    "enabled": true,
    "port": 8080,
    "strategy": "sticky_sessions",
    "session": {
      "cookie_name": "PMDAEMON_SESSION",
      "header_name": "X-Session-ID",
      "timeout": 3600
    },
    "backend_ports": [3000, 3001, 3002, 3003]
  }
}
```

### 3. SSL Termination

Handle SSL termination at the load balancer level.

```json
{
  "load_balancer": {
    "enabled": true,
    "port": 443,
    "ssl": {
      "enabled": true,
      "cert_file": "/etc/ssl/certs/server.crt",
      "key_file": "/etc/ssl/private/server.key",
      "protocols": ["TLSv1.2", "TLSv1.3"]
    },
    "backend_ports": [3000, 3001, 3002, 3003],
    "backend_protocol": "http"
  }
}
```

## TCP Load Balancing

### 1. TCP Stream Load Balancing

For non-HTTP protocols like databases or custom TCP services.

```json
{
  "apps": [
    {
      "name": "tcp-servers",
      "script": "tcp-server.js",
      "instances": 3,
      "exec_mode": "cluster",
      "increment_var": "PORT",
      "env": {
        "PORT": 5000,
        "PROTOCOL": "tcp"
      }
    }
  ],
  "load_balancer": {
    "enabled": true,
    "type": "tcp",
    "port": 4000,
    "strategy": "least_connections",
    "backend_ports": [5000, 5001, 5002]
  }
}
```

### 2. Connection Pooling

Manage persistent connections efficiently.

```json
{
  "load_balancer": {
    "type": "tcp",
    "port": 4000,
    "connection_pool": {
      "enabled": true,
      "max_connections": 1000,
      "idle_timeout": 300,
      "keep_alive": true
    },
    "backend_ports": [5000, 5001, 5002]
  }
}
```

## Load Balancer Configuration

### 1. Timeouts and Retries

```json
{
  "load_balancer": {
    "enabled": true,
    "port": 8080,
    "timeouts": {
      "connect": 5000,
      "read": 30000,
      "write": 30000,
      "idle": 60000
    },
    "retries": {
      "max_attempts": 3,
      "retry_delay": 1000,
      "backoff_factor": 2
    }
  }
}
```

### 2. Rate Limiting

Protect backend services from overload.

```json
{
  "load_balancer": {
    "enabled": true,
    "port": 8080,
    "rate_limiting": {
      "enabled": true,
      "requests_per_second": 100,
      "burst_size": 20,
      "per_ip_limit": 10
    }
  }
}
```

### 3. Circuit Breaker

Automatically handle failing backend services.

```json
{
  "load_balancer": {
    "enabled": true,
    "port": 8080,
    "circuit_breaker": {
      "enabled": true,
      "failure_threshold": 5,
      "recovery_timeout": 30000,
      "half_open_requests": 3
    }
  }
}
```

## Monitoring Load Balancing

### 1. Load Balancer Metrics

```bash
# View load balancer status
pmdaemon lb status

# Check backend health
pmdaemon lb health

# View traffic distribution
pmdaemon lb stats

# Monitor real-time metrics
pmdaemon lb monitor
```

### 2. Performance Metrics

```json
{
  "load_balancer": {
    "monitoring": {
      "enabled": true,
      "metrics": {
        "requests_per_second": true,
        "response_times": true,
        "error_rates": true,
        "backend_health": true,
        "connection_counts": true
      },
      "export": {
        "prometheus": true,
        "statsd": true
      }
    }
  }
}
```

### 3. Alerting

```json
{
  "load_balancer": {
    "alerts": {
      "enabled": true,
      "rules": [
        {
          "name": "high_error_rate",
          "condition": "error_rate > 5%",
          "duration": "5m",
          "action": "webhook",
          "url": "https://alerts.example.com/webhook"
        },
        {
          "name": "backend_down",
          "condition": "healthy_backends < 2",
          "duration": "1m",
          "action": "email",
          "recipients": ["admin@example.com"]
        }
      ]
    }
  }
}
```

## External Load Balancer Integration

### 1. NGINX Integration

```nginx
upstream pmdaemon_backend {
    # PMDaemon managed instances
    server 127.0.0.1:3000;
    server 127.0.0.1:3001;
    server 127.0.0.1:3002;
    server 127.0.0.1:3003;
}

server {
    listen 80;
    server_name example.com;

    location / {
        proxy_pass http://pmdaemon_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # Health check endpoint
    location /lb-health {
        access_log off;
        return 200 "healthy\n";
        add_header Content-Type text/plain;
    }
}
```

### 2. HAProxy Integration

```
# /etc/haproxy/haproxy.cfg
global
    daemon
    maxconn 256

defaults
    mode http
    timeout connect 5000ms
    timeout client 50000ms
    timeout server 50000ms

frontend web_frontend
    bind *:80
    default_backend pmdaemon_servers

backend pmdaemon_servers
    balance roundrobin
    option httpchk GET /health
    server web1 127.0.0.1:3000 check
    server web2 127.0.0.1:3001 check
    server web3 127.0.0.1:3002 check
    server web4 127.0.0.1:3003 check
```

### 3. Cloud Load Balancer Integration

**AWS Application Load Balancer:**
```json
{
  "apps": [
    {
      "name": "aws-web-app",
      "script": "server.js",
      "instances": 4,
      "exec_mode": "cluster",
      "env": {
        "PORT": 3000,
        "AWS_REGION": "us-east-1"
      },
      "aws": {
        "alb": {
          "target_group_arn": "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/my-targets/73e2d6bc24d8a067",
          "health_check_path": "/health",
          "deregistration_delay": 30
        }
      }
    }
  ]
}
```

## Load Balancing Best Practices

### 1. Health Check Strategy
- Implement comprehensive health checks
- Use different endpoints for different types of checks
- Set appropriate timeouts and retry logic
- Monitor health check performance

### 2. Session Management
- Use sticky sessions sparingly
- Prefer stateless application design
- Implement session storage for stateful apps
- Handle session failover gracefully

### 3. Performance Optimization
- Monitor response times across instances
- Adjust weights based on instance capacity
- Use connection pooling for efficiency
- Implement proper caching strategies

### 4. Failure Handling
- Configure circuit breakers for resilience
- Implement graceful degradation
- Use multiple availability zones
- Plan for disaster recovery

### 5. Security Considerations
- Implement rate limiting
- Use SSL termination appropriately
- Protect against DDoS attacks
- Monitor for suspicious traffic patterns

## Troubleshooting Load Balancing

### Common Issues

1. **Uneven Traffic Distribution**
   - Check instance health status
   - Verify load balancing strategy
   - Monitor instance performance

2. **Session Affinity Problems**
   - Validate sticky session configuration
   - Check session storage backend
   - Monitor session timeouts

3. **Health Check Failures**
   - Verify health check endpoints
   - Check network connectivity
   - Review timeout settings

4. **Performance Degradation**
   - Monitor backend response times
   - Check resource utilization
   - Verify load balancer capacity

### Debugging Commands

```bash
# Check load balancer configuration
pmdaemon lb config

# View backend instance status
pmdaemon lb backends

# Monitor traffic distribution
pmdaemon lb traffic

# Test health checks
pmdaemon lb health-check --test

# View detailed metrics
pmdaemon lb metrics --detailed
```

## Related Documentation

- **[Clustering Examples](../examples/clustering.md)** - Clustering configuration examples
- **[Port Management](./port-management.md)** - Advanced port management
- **[Monitoring](../monitoring/overview.md)** - Monitoring and alerting
- **[Performance Optimization](../performance/optimization.md)** - Performance tuning
