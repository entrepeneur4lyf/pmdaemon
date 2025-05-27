# Security

This comprehensive security guide covers best practices for securing PMDaemon deployments, protecting sensitive data, and implementing defense-in-depth strategies for production environments.

## System Security

### User and Permission Management

#### Dedicated User Account

Create a dedicated user for PMDaemon operations:

```bash
# Create pmdaemon user
sudo useradd -r -s /bin/false -d /var/lib/pmdaemon pmdaemon

# Create necessary directories
sudo mkdir -p /var/lib/pmdaemon/{logs,pids,config}
sudo mkdir -p /var/log/pmdaemon
sudo mkdir -p /etc/pmdaemon

# Set ownership
sudo chown -R pmdaemon:pmdaemon /var/lib/pmdaemon
sudo chown -R pmdaemon:pmdaemon /var/log/pmdaemon
sudo chown -R pmdaemon:pmdaemon /etc/pmdaemon

# Set permissions
sudo chmod 750 /var/lib/pmdaemon
sudo chmod 750 /var/log/pmdaemon
sudo chmod 750 /etc/pmdaemon
```

#### File Permissions

```bash
# Secure configuration files
sudo chmod 600 /etc/pmdaemon/ecosystem.json
sudo chmod 600 /etc/pmdaemon/.env

# Secure log directories
sudo chmod 750 /var/log/pmdaemon
sudo chmod 640 /var/log/pmdaemon/*.log

# Secure PID files
sudo chmod 644 /var/lib/pmdaemon/pids/*.pid
```

#### Process Isolation

```json
{
  "name": "secure-web-app",
  "script": "node",
  "args": ["server.js"],
  "user": "webapp",
  "group": "webapp",
  "cwd": "/opt/webapp",
  "env": {
    "NODE_ENV": "production"
  },
  "umask": "0027"
}
```

### Network Security

#### Firewall Configuration

```bash
# UFW (Ubuntu/Debian)
sudo ufw default deny incoming
sudo ufw default allow outgoing

# Allow SSH
sudo ufw allow ssh

# Allow application ports
sudo ufw allow 3000:3003/tcp comment "Web application"
sudo ufw allow 8000:8003/tcp comment "API services"

# Allow PMDaemon web interface (restrict to specific IPs)
sudo ufw allow from 192.168.1.0/24 to any port 9615 comment "PMDaemon web interface"

sudo ufw enable
```

#### Network Binding

```bash
# Bind to localhost only for security
pmdaemon web --host 127.0.0.1 --port 9615

# Use reverse proxy for external access
# nginx configuration for secure proxy
```

**Nginx reverse proxy configuration:**
```nginx
server {
    listen 443 ssl http2;
    server_name pmdaemon.yourdomain.com;
    
    ssl_certificate /etc/ssl/certs/pmdaemon.crt;
    ssl_certificate_key /etc/ssl/private/pmdaemon.key;
    
    # Security headers
    add_header X-Frame-Options DENY;
    add_header X-Content-Type-Options nosniff;
    add_header X-XSS-Protection "1; mode=block";
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains";
    
    # Client certificate authentication (optional)
    ssl_client_certificate /etc/ssl/certs/ca.crt;
    ssl_verify_client on;
    
    location / {
        proxy_pass http://127.0.0.1:9615;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # WebSocket support
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

## Configuration Security

### Environment Variable Protection

#### Secure Environment Files

```bash
# Create secure environment file
sudo touch /etc/pmdaemon/secrets.env
sudo chmod 600 /etc/pmdaemon/secrets.env
sudo chown pmdaemon:pmdaemon /etc/pmdaemon/secrets.env

# Example secure environment file
cat > /etc/pmdaemon/secrets.env << 'EOF'
DATABASE_PASSWORD=super_secure_password_123
JWT_SECRET=your_jwt_secret_key_here
API_KEY=your_api_key_here
REDIS_PASSWORD=redis_password_here
EOF
```

#### Configuration with Secrets

```json
{
  "name": "secure-api",
  "script": "node",
  "args": ["server.js"],
  "env_file": "/etc/pmdaemon/secrets.env",
  "env": {
    "NODE_ENV": "production",
    "LOG_LEVEL": "info"
  }
}
```

#### External Secret Management

```bash
# Using HashiCorp Vault
export VAULT_ADDR="https://vault.company.com"
export VAULT_TOKEN="your-vault-token"

# Retrieve secrets and start process
DATABASE_URL=$(vault kv get -field=url secret/database)
API_KEY=$(vault kv get -field=key secret/api)

pmdaemon start "node server.js" --name secure-api \
  --env DATABASE_URL="$DATABASE_URL" \
  --env API_KEY="$API_KEY"
```

### Configuration Validation

```bash
# Validate configuration security
pmdaemon validate ecosystem.json --security-check

# Check for common security issues
pmdaemon security-audit ecosystem.json
```

## Application Security

### Secure Process Configuration

#### Resource Limits

```json
{
  "name": "hardened-service",
  "script": "node",
  "args": ["--max-old-space-size=512", "server.js"],
  "max_memory_restart": "512M",
  "max_restarts": 3,
  "min_uptime": "30s",
  "kill_timeout": "10s",
  "env": {
    "NODE_ENV": "production",
    "NODE_OPTIONS": "--max-old-space-size=512"
  }
}
```

#### Security Headers and Middleware

```javascript
// Express.js security middleware
const helmet = require('helmet');
const rateLimit = require('express-rate-limit');

app.use(helmet({
  contentSecurityPolicy: {
    directives: {
      defaultSrc: ["'self'"],
      styleSrc: ["'self'", "'unsafe-inline'"],
      scriptSrc: ["'self'"],
      imgSrc: ["'self'", "data:", "https:"]
    }
  },
  hsts: {
    maxAge: 31536000,
    includeSubDomains: true,
    preload: true
  }
}));

// Rate limiting
const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // limit each IP to 100 requests per windowMs
  message: "Too many requests from this IP"
});

app.use('/api/', limiter);
```

### Input Validation and Sanitization

```javascript
// Input validation middleware
const { body, validationResult } = require('express-validator');

app.post('/api/users',
  body('email').isEmail().normalizeEmail(),
  body('password').isLength({ min: 8 }).matches(/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/),
  (req, res) => {
    const errors = validationResult(req);
    if (!errors.isEmpty()) {
      return res.status(400).json({ errors: errors.array() });
    }
    
    // Process validated input
  }
);
```

## Monitoring and Logging Security

### Secure Logging

```json
{
  "name": "secure-logging-app",
  "script": "node",
  "args": ["server.js"],
  "out_file": "/var/log/pmdaemon/app.log",
  "error_file": "/var/log/pmdaemon/app-error.log",
  "log": {
    "log_date_format": "YYYY-MM-DD HH:mm:ss Z",
    "merge_logs": false,
    "log_type": "json"
  },
  "env": {
    "LOG_LEVEL": "info",
    "AUDIT_LOG": "true"
  }
}
```

#### Log Sanitization

```javascript
// Secure logging configuration
const winston = require('winston');

const logger = winston.createLogger({
  level: 'info',
  format: winston.format.combine(
    winston.format.timestamp(),
    winston.format.errors({ stack: true }),
    winston.format.json(),
    winston.format.printf(info => {
      // Remove sensitive data from logs
      const sanitized = { ...info };
      delete sanitized.password;
      delete sanitized.token;
      delete sanitized.apiKey;
      
      // Mask credit card numbers
      if (sanitized.message) {
        sanitized.message = sanitized.message.replace(/\d{4}-?\d{4}-?\d{4}-?\d{4}/g, '****-****-****-****');
      }
      
      return JSON.stringify(sanitized);
    })
  ),
  transports: [
    new winston.transports.File({ filename: '/var/log/app/error.log', level: 'error' }),
    new winston.transports.File({ filename: '/var/log/app/combined.log' })
  ]
});
```

### Security Monitoring

```bash
# Monitor for security events
tail -f /var/log/pmdaemon/*.log | grep -E "(failed|error|unauthorized|forbidden)"

# Set up log rotation with compression
sudo tee /etc/logrotate.d/pmdaemon << 'EOF'
/var/log/pmdaemon/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 640 pmdaemon pmdaemon
    postrotate
        systemctl reload pmdaemon || true
    endscript
}
EOF
```

## Health Check Security

### Secure Health Endpoints

```javascript
// Secure health check endpoint
app.get('/health', (req, res) => {
  // Basic health check - no sensitive information
  res.json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    uptime: process.uptime()
  });
});

// Detailed health check with authentication
app.get('/health/detailed', authenticateToken, async (req, res) => {
  try {
    const health = {
      status: 'healthy',
      timestamp: new Date().toISOString(),
      uptime: process.uptime(),
      memory: process.memoryUsage(),
      database: await checkDatabase(),
      redis: await checkRedis()
    };
    
    res.json(health);
  } catch (error) {
    res.status(503).json({
      status: 'unhealthy',
      error: 'Service unavailable'
    });
  }
});

function authenticateToken(req, res, next) {
  const authHeader = req.headers['authorization'];
  const token = authHeader && authHeader.split(' ')[1];
  
  if (!token) {
    return res.sendStatus(401);
  }
  
  jwt.verify(token, process.env.JWT_SECRET, (err, user) => {
    if (err) return res.sendStatus(403);
    req.user = user;
    next();
  });
}
```

### Health Check Authentication

```json
{
  "name": "secure-health-checks",
  "script": "node",
  "args": ["server.js"],
  "health_check": {
    "check_type": "script",
    "script": "./scripts/secure-health-check.sh",
    "timeout": 10,
    "interval": 30
  }
}
```

**secure-health-check.sh:**
```bash
#!/bin/bash

# Authenticate health check request
TOKEN=$(cat /etc/pmdaemon/health-token)

# Make authenticated request
RESPONSE=$(curl -s -H "Authorization: Bearer $TOKEN" \
  -w "%{http_code}" \
  http://localhost:3000/health/detailed)

HTTP_CODE="${RESPONSE: -3}"

if [ "$HTTP_CODE" = "200" ]; then
    echo "Health check passed"
    exit 0
else
    echo "Health check failed with code $HTTP_CODE"
    exit 1
fi
```

## Container Security

### Secure Docker Configuration

```dockerfile
# Use non-root user
FROM node:16-alpine

# Create app user
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nodejs -u 1001

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production && npm cache clean --force

# Copy application code
COPY --chown=nodejs:nodejs . .

# Switch to non-root user
USER nodejs

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:3000/health || exit 1

# Start application
CMD ["node", "server.js"]
```

### Container Runtime Security

```bash
# Run container with security options
docker run -d \
  --name secure-app \
  --user 1001:1001 \
  --read-only \
  --tmpfs /tmp \
  --tmpfs /var/run \
  --cap-drop ALL \
  --cap-add NET_BIND_SERVICE \
  --security-opt no-new-privileges \
  --security-opt seccomp=seccomp-profile.json \
  myapp:latest
```

## API Security

### Authentication and Authorization

```javascript
// JWT-based authentication
const jwt = require('jsonwebtoken');
const bcrypt = require('bcrypt');

// Login endpoint
app.post('/auth/login', async (req, res) => {
  try {
    const { username, password } = req.body;
    
    // Validate user credentials
    const user = await User.findOne({ username });
    if (!user || !await bcrypt.compare(password, user.passwordHash)) {
      return res.status(401).json({ error: 'Invalid credentials' });
    }
    
    // Generate JWT token
    const token = jwt.sign(
      { userId: user.id, username: user.username },
      process.env.JWT_SECRET,
      { expiresIn: '1h' }
    );
    
    res.json({ token });
  } catch (error) {
    res.status(500).json({ error: 'Authentication failed' });
  }
});

// Protected route middleware
function authenticateToken(req, res, next) {
  const authHeader = req.headers['authorization'];
  const token = authHeader && authHeader.split(' ')[1];
  
  if (!token) {
    return res.sendStatus(401);
  }
  
  jwt.verify(token, process.env.JWT_SECRET, (err, user) => {
    if (err) return res.sendStatus(403);
    req.user = user;
    next();
  });
}

// Apply authentication to PMDaemon API routes
app.use('/api/processes', authenticateToken);
app.use('/api/system', authenticateToken);
```

### API Rate Limiting

```javascript
const rateLimit = require('express-rate-limit');
const RedisStore = require('rate-limit-redis');
const Redis = require('ioredis');

const redisClient = new Redis(process.env.REDIS_URL);

// Create rate limiter
const apiLimiter = rateLimit({
  store: new RedisStore({
    sendCommand: (...args) => redisClient.call(...args),
  }),
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // limit each IP to 100 requests per windowMs
  message: {
    error: 'Too many requests, please try again later.'
  },
  standardHeaders: true,
  legacyHeaders: false,
});

// Apply rate limiting
app.use('/api/', apiLimiter);
```

## Compliance and Auditing

### Audit Logging

```javascript
// Audit logging middleware
function auditLog(action) {
  return (req, res, next) => {
    const audit = {
      timestamp: new Date().toISOString(),
      action: action,
      user: req.user?.username || 'anonymous',
      ip: req.ip,
      userAgent: req.get('User-Agent'),
      resource: req.originalUrl,
      method: req.method
    };
    
    // Log to audit file
    fs.appendFileSync('/var/log/pmdaemon/audit.log', JSON.stringify(audit) + '\n');
    
    next();
  };
}

// Apply audit logging to sensitive operations
app.post('/api/processes', auditLog('process_start'));
app.delete('/api/processes/:id', auditLog('process_stop'));
app.post('/api/processes/:id/restart', auditLog('process_restart'));
```

### Security Scanning

```bash
#!/bin/bash
# security-scan.sh

echo "🔍 Running security scan..."

# Check for known vulnerabilities
npm audit --audit-level moderate

# Scan Docker images
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  aquasec/trivy image myapp:latest

# Check file permissions
find /etc/pmdaemon -type f -perm /o+w -exec ls -la {} \;

# Check for exposed secrets
grep -r "password\|secret\|key" /etc/pmdaemon/ --exclude="*.log"

echo "✅ Security scan complete"
```

## Best Practices Summary

### 1. Principle of Least Privilege

```bash
# Run processes with minimal permissions
pmdaemon start "node server.js" --name web-app \
  --user webapp \
  --group webapp \
  --cwd /opt/webapp
```

### 2. Secure Configuration Management

```bash
# Use secure environment files
pmdaemon start "node server.js" --name api \
  --env-file /etc/pmdaemon/secrets.env
```

### 3. Network Security

```bash
# Bind to localhost and use reverse proxy
pmdaemon web --host 127.0.0.1 --port 9615
```

### 4. Regular Security Updates

```bash
# Keep PMDaemon updated
cargo install pmdaemon --force

# Update system packages
sudo apt update && sudo apt upgrade -y
```

### 5. Monitor and Audit

```bash
# Enable audit logging
export PMDAEMON_AUDIT_LOG=true
pmdaemon start "node server.js" --name web-app
```

## Next Steps

- **[Performance Tuning](./performance-tuning.md)** - Optimize secure configurations
- **[Logging](./logging.md)** - Secure logging practices
- **[Troubleshooting](./troubleshooting.md)** - Security issue diagnosis
- **[Deployment Examples](../examples/deployment-examples.md)** - Secure deployment patterns
