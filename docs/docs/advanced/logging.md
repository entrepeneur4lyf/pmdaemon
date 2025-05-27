# Logging

PMDaemon provides comprehensive logging capabilities for process output, system events, and operational monitoring. This guide covers advanced logging configuration, log management, and integration with external logging systems.

## Overview

PMDaemon logging features:

- **📝 Process output capture** - Stdout and stderr logging
- **🔄 Automatic log rotation** - Prevent disk space issues
- **📊 Structured logging** - JSON and custom formats
- **🎯 Log filtering** - Filter by level, process, or pattern
- **📤 External integration** - Syslog, ELK stack, and more

## Basic Logging Configuration

### Default Logging

```bash
# Basic process with default logging
pmdaemon start "node server.js" --name web-app

# Logs are automatically created:
# ~/.pmdaemon/logs/web-app-0-out.log (stdout)
# ~/.pmdaemon/logs/web-app-0-err.log (stderr)
# ~/.pmdaemon/logs/web-app-0.pid (process ID)
```

### Custom Log Files

```bash
# Specify custom log file locations
pmdaemon start "node server.js" --name web-app \
  --out-file /var/log/myapp/web-app.out \
  --error-file /var/log/myapp/web-app.err \
  --pid-file /var/run/myapp/web-app.pid
```

### Configuration File Logging

```json
{
  "name": "web-app",
  "script": "node",
  "args": ["server.js"],
  "out_file": "/var/log/myapp/web-app.out",
  "error_file": "/var/log/myapp/web-app.err",
  "log_file": "/var/log/myapp/web-app.log",
  "pid_file": "/var/run/myapp/web-app.pid",
  "log_date_format": "YYYY-MM-DD HH:mm:ss Z",
  "merge_logs": true
}
```

## Advanced Logging Configuration

### Structured Logging

```json
{
  "name": "structured-app",
  "script": "node",
  "args": ["server.js"],
  "log": {
    "out_file": "/var/log/myapp/app.json",
    "error_file": "/var/log/myapp/app-error.json",
    "log_type": "json",
    "log_date_format": "YYYY-MM-DD HH:mm:ss.SSS Z",
    "merge_logs": false,
    "max_log_size": "100M",
    "max_log_files": 10,
    "compress_logs": true
  }
}
```

### Log Rotation

```json
{
  "name": "rotating-logs-app",
  "script": "node",
  "args": ["server.js"],
  "log": {
    "out_file": "/var/log/myapp/app.log",
    "error_file": "/var/log/myapp/app-error.log",
    "max_log_size": "50M",
    "max_log_files": 5,
    "compress_logs": true,
    "rotation_schedule": "daily"
  }
}
```

### Environment-Specific Logging

```json
{
  "apps": [
    {
      "name": "app-development",
      "script": "node",
      "args": ["server.js"],
      "env": {
        "NODE_ENV": "development",
        "LOG_LEVEL": "debug"
      },
      "out_file": "/dev/stdout",
      "error_file": "/dev/stderr"
    },
    {
      "name": "app-production",
      "script": "node",
      "args": ["server.js"],
      "env": {
        "NODE_ENV": "production",
        "LOG_LEVEL": "info"
      },
      "out_file": "/var/log/myapp/app.log",
      "error_file": "/var/log/myapp/app-error.log",
      "log": {
        "max_log_size": "100M",
        "max_log_files": 10,
        "compress_logs": true
      }
    }
  ]
}
```

## Application-Level Logging

### Node.js Structured Logging

```javascript
// logger.js - Winston configuration
const winston = require('winston');
const path = require('path');

const logFormat = winston.format.combine(
  winston.format.timestamp({
    format: 'YYYY-MM-DD HH:mm:ss.SSS Z'
  }),
  winston.format.errors({ stack: true }),
  winston.format.json(),
  winston.format.printf(info => {
    const { timestamp, level, message, ...meta } = info;
    
    const logEntry = {
      timestamp,
      level,
      message,
      service: 'web-app',
      instance: process.env.PM2_INSTANCE_ID || 0,
      pid: process.pid,
      ...meta
    };
    
    return JSON.stringify(logEntry);
  })
);

const logger = winston.createLogger({
  level: process.env.LOG_LEVEL || 'info',
  format: logFormat,
  defaultMeta: {
    service: 'web-app',
    version: process.env.APP_VERSION || '1.0.0'
  },
  transports: [
    new winston.transports.File({
      filename: '/var/log/myapp/error.log',
      level: 'error',
      maxsize: 50 * 1024 * 1024, // 50MB
      maxFiles: 5,
      tailable: true
    }),
    new winston.transports.File({
      filename: '/var/log/myapp/combined.log',
      maxsize: 100 * 1024 * 1024, // 100MB
      maxFiles: 10,
      tailable: true
    })
  ]
});

// Add console transport for development
if (process.env.NODE_ENV !== 'production') {
  logger.add(new winston.transports.Console({
    format: winston.format.combine(
      winston.format.colorize(),
      winston.format.simple()
    )
  }));
}

module.exports = logger;
```

### Request Logging Middleware

```javascript
// middleware/logging.js
const logger = require('../logger');

function requestLogger(req, res, next) {
  const start = Date.now();
  
  // Log request
  logger.info('Request started', {
    method: req.method,
    url: req.url,
    userAgent: req.get('User-Agent'),
    ip: req.ip,
    requestId: req.id
  });
  
  // Log response
  res.on('finish', () => {
    const duration = Date.now() - start;
    
    logger.info('Request completed', {
      method: req.method,
      url: req.url,
      statusCode: res.statusCode,
      duration,
      requestId: req.id
    });
  });
  
  next();
}

module.exports = requestLogger;
```

### Error Logging

```javascript
// middleware/errorHandler.js
const logger = require('../logger');

function errorHandler(err, req, res, next) {
  // Log error with context
  logger.error('Unhandled error', {
    error: {
      message: err.message,
      stack: err.stack,
      name: err.name
    },
    request: {
      method: req.method,
      url: req.url,
      headers: req.headers,
      body: req.body,
      params: req.params,
      query: req.query
    },
    user: req.user?.id,
    requestId: req.id
  });
  
  // Send error response
  res.status(err.status || 500).json({
    error: 'Internal server error',
    requestId: req.id
  });
}

module.exports = errorHandler;
```

## Log Management

### Viewing Logs

```bash
# View recent logs
pmdaemon logs web-app

# View specific number of lines
pmdaemon logs web-app --lines 100

# Follow logs in real-time
pmdaemon logs web-app --follow

# View logs with timestamps
pmdaemon logs web-app --timestamps

# View only error logs
pmdaemon logs web-app --error

# View logs from specific instance
pmdaemon logs web-app --instance 2
```

### Log Filtering

```bash
# Filter logs by pattern
pmdaemon logs web-app --grep "ERROR"

# Filter by log level
pmdaemon logs web-app --level error

# Filter by time range
pmdaemon logs web-app --since "2024-01-15 10:00:00"
pmdaemon logs web-app --until "2024-01-15 12:00:00"

# Combine filters
pmdaemon logs web-app --grep "database" --level error --lines 50
```

### Log Rotation with Logrotate

```bash
# Create logrotate configuration
sudo tee /etc/logrotate.d/pmdaemon << 'EOF'
/var/log/myapp/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 pmdaemon pmdaemon
    postrotate
        # Send USR1 signal to reload logs
        pkill -USR1 -f pmdaemon || true
    endscript
}

/var/log/myapp/*.json {
    daily
    rotate 7
    compress
    delaycompress
    missingok
    notifempty
    create 644 pmdaemon pmdaemon
    copytruncate
}
EOF

# Test logrotate configuration
sudo logrotate -d /etc/logrotate.d/pmdaemon

# Force rotation (for testing)
sudo logrotate -f /etc/logrotate.d/pmdaemon
```

## External Logging Integration

### Syslog Integration

```bash
# Send logs to syslog
pmdaemon start "node server.js | logger -t web-app" --name web-app
```

```javascript
// Application syslog integration
const winston = require('winston');
require('winston-syslog').Syslog;

const logger = winston.createLogger({
  transports: [
    new winston.transports.Syslog({
      host: 'localhost',
      port: 514,
      protocol: 'udp4',
      facility: 'local0',
      app_name: 'web-app',
      eol: '\n'
    })
  ]
});
```

### ELK Stack Integration

#### Filebeat Configuration

```yaml
# filebeat.yml
filebeat.inputs:
- type: log
  enabled: true
  paths:
    - /var/log/myapp/*.json
  json.keys_under_root: true
  json.add_error_key: true
  fields:
    service: myapp
    environment: production
  fields_under_root: true

output.elasticsearch:
  hosts: ["elasticsearch:9200"]
  index: "myapp-logs-%{+yyyy.MM.dd}"

setup.template.name: "myapp-logs"
setup.template.pattern: "myapp-logs-*"

logging.level: info
logging.to_files: true
logging.files:
  path: /var/log/filebeat
  name: filebeat
  keepfiles: 7
  permissions: 0644
```

#### Logstash Configuration

```ruby
# logstash.conf
input {
  beats {
    port => 5044
  }
}

filter {
  if [service] == "myapp" {
    # Parse timestamp
    date {
      match => [ "timestamp", "yyyy-MM-dd HH:mm:ss.SSS Z" ]
    }
    
    # Add computed fields
    mutate {
      add_field => { "log_processed_at" => "%{@timestamp}" }
    }
    
    # Parse error stack traces
    if [level] == "error" and [error][stack] {
      mutate {
        add_field => { "error_parsed" => "true" }
      }
    }
  }
}

output {
  elasticsearch {
    hosts => ["elasticsearch:9200"]
    index => "myapp-logs-%{+YYYY.MM.dd}"
  }
}
```

### Fluentd Integration

```ruby
# fluent.conf
<source>
  @type tail
  path /var/log/myapp/*.json
  pos_file /var/log/fluentd/myapp.log.pos
  tag myapp.logs
  format json
  time_key timestamp
  time_format %Y-%m-%d %H:%M:%S.%L %z
</source>

<filter myapp.logs>
  @type record_transformer
  <record>
    hostname "#{Socket.gethostname}"
    environment production
  </record>
</filter>

<match myapp.logs>
  @type elasticsearch
  host elasticsearch
  port 9200
  index_name myapp-logs
  type_name _doc
  logstash_format true
  logstash_prefix myapp-logs
  logstash_dateformat %Y.%m.%d
  include_tag_key true
  tag_key @log_name
  flush_interval 1s
</match>
```

## Monitoring and Alerting

### Log-Based Monitoring

```bash
#!/bin/bash
# log-monitor.sh

LOG_FILE="/var/log/myapp/app.log"
ERROR_THRESHOLD=10
TIME_WINDOW=300  # 5 minutes

# Count errors in the last 5 minutes
ERROR_COUNT=$(tail -n 1000 "$LOG_FILE" | \
  jq -r --arg since "$(date -d '5 minutes ago' '+%Y-%m-%d %H:%M:%S')" \
  'select(.timestamp > $since and .level == "error")' | \
  wc -l)

if [ "$ERROR_COUNT" -gt "$ERROR_THRESHOLD" ]; then
  echo "ALERT: $ERROR_COUNT errors in the last $TIME_WINDOW seconds"
  # Send alert (email, Slack, etc.)
  curl -X POST -H 'Content-type: application/json' \
    --data "{\"text\":\"High error rate: $ERROR_COUNT errors in 5 minutes\"}" \
    "$SLACK_WEBHOOK_URL"
fi
```

### Log Analysis Scripts

```python
#!/usr/bin/env python3
# log-analyzer.py

import json
import sys
from datetime import datetime, timedelta
from collections import defaultdict

def analyze_logs(log_file, hours=1):
    """Analyze logs for the last N hours"""
    
    cutoff_time = datetime.now() - timedelta(hours=hours)
    
    stats = {
        'total_requests': 0,
        'error_count': 0,
        'status_codes': defaultdict(int),
        'response_times': [],
        'error_messages': defaultdict(int)
    }
    
    with open(log_file, 'r') as f:
        for line in f:
            try:
                log_entry = json.loads(line)
                
                # Parse timestamp
                log_time = datetime.fromisoformat(
                    log_entry['timestamp'].replace('Z', '+00:00')
                )
                
                if log_time < cutoff_time:
                    continue
                
                # Count requests
                if 'Request completed' in log_entry.get('message', ''):
                    stats['total_requests'] += 1
                    
                    if 'statusCode' in log_entry:
                        stats['status_codes'][log_entry['statusCode']] += 1
                    
                    if 'duration' in log_entry:
                        stats['response_times'].append(log_entry['duration'])
                
                # Count errors
                if log_entry.get('level') == 'error':
                    stats['error_count'] += 1
                    error_msg = log_entry.get('message', 'Unknown error')
                    stats['error_messages'][error_msg] += 1
                    
            except (json.JSONDecodeError, KeyError, ValueError):
                continue
    
    # Calculate averages
    if stats['response_times']:
        stats['avg_response_time'] = sum(stats['response_times']) / len(stats['response_times'])
        stats['max_response_time'] = max(stats['response_times'])
    
    return stats

if __name__ == '__main__':
    log_file = sys.argv[1] if len(sys.argv) > 1 else '/var/log/myapp/app.log'
    hours = int(sys.argv[2]) if len(sys.argv) > 2 else 1
    
    stats = analyze_logs(log_file, hours)
    
    print(f"Log Analysis (Last {hours} hour(s)):")
    print(f"Total Requests: {stats['total_requests']}")
    print(f"Error Count: {stats['error_count']}")
    print(f"Error Rate: {stats['error_count'] / max(stats['total_requests'], 1) * 100:.2f}%")
    
    if stats.get('avg_response_time'):
        print(f"Avg Response Time: {stats['avg_response_time']:.2f}ms")
        print(f"Max Response Time: {stats['max_response_time']:.2f}ms")
    
    print("\nStatus Code Distribution:")
    for code, count in sorted(stats['status_codes'].items()):
        print(f"  {code}: {count}")
    
    if stats['error_messages']:
        print("\nTop Error Messages:")
        for msg, count in sorted(stats['error_messages'].items(), 
                                key=lambda x: x[1], reverse=True)[:5]:
            print(f"  {count}x: {msg}")
```

## Best Practices

### 1. Use Structured Logging

```javascript
// Good: Structured logging
logger.info('User login', {
  userId: user.id,
  email: user.email,
  ip: req.ip,
  userAgent: req.get('User-Agent')
});

// Avoid: Unstructured logging
console.log(`User ${user.email} logged in from ${req.ip}`);
```

### 2. Set Appropriate Log Levels

```javascript
// Development
logger.level = 'debug';

// Production
logger.level = 'info';

// Critical systems
logger.level = 'warn';
```

### 3. Implement Log Rotation

```json
{
  "log": {
    "max_log_size": "100M",
    "max_log_files": 10,
    "compress_logs": true
  }
}
```

### 4. Sanitize Sensitive Data

```javascript
function sanitizeLog(data) {
  const sanitized = { ...data };
  
  // Remove sensitive fields
  delete sanitized.password;
  delete sanitized.token;
  delete sanitized.creditCard;
  
  // Mask email addresses
  if (sanitized.email) {
    sanitized.email = sanitized.email.replace(/(.{2}).*@/, '$1***@');
  }
  
  return sanitized;
}

logger.info('User data', sanitizeLog(userData));
```

### 5. Monitor Log Health

```bash
# Check log file sizes
du -sh /var/log/myapp/*

# Check log rotation
ls -la /var/log/myapp/*.gz

# Monitor disk space
df -h /var/log
```

## Next Steps

- **[Monitoring](../features/monitoring.md)** - Real-time monitoring integration
- **[Security](./security.md)** - Secure logging practices
- **[Performance Tuning](./performance-tuning.md)** - Optimize logging performance
- **[Troubleshooting](./troubleshooting.md)** - Debug using logs
