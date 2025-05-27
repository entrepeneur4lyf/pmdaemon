# API Authentication

PMDaemon provides robust authentication mechanisms to secure API access. This guide covers all available authentication methods and their configuration.

## Authentication Methods

### Token-Based Authentication

Token-based authentication uses a shared secret to generate and validate access tokens.

#### Configuration
```toml
[security.auth]
method = "token"
secret_key = "your-256-bit-secret-key-here"
token_expiry = 3600  # Token lifetime in seconds
algorithm = "HS256"  # HMAC algorithm
```

#### Usage
```bash
# Get a token
curl -X POST http://localhost:3000/auth/token \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "your-password"}'

# Use the token
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/api/processes
```

#### Token Generation Example
```javascript
// Generate token programmatically
const token = await pmdaemon.auth.generateToken({
  userId: 'admin',
  permissions: ['process:read', 'process:write'],
  expiresIn: 3600
});
```

### JWT Authentication

JSON Web Token authentication provides stateless authentication with embedded claims.

#### Configuration
```toml
[security.auth]
method = "jwt"
jwt_secret = "your-jwt-secret-key"
jwt_algorithm = "HS256"  # or RS256 for asymmetric
issuer = "pmdaemon"
audience = "api-clients"
token_expiry = 7200

# For asymmetric JWT (RS256)
public_key_file = "/path/to/public.pem"
private_key_file = "/path/to/private.pem"
```

#### JWT Claims Structure
```json
{
  "sub": "user123",
  "iss": "pmdaemon",
  "aud": "api-clients",
  "exp": 1635724800,
  "iat": 1635721200,
  "roles": ["operator"],
  "permissions": ["process:read", "process:restart"]
}
```

#### Usage
```bash
# Login and get JWT
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "your-password"}'

# Use JWT in requests
curl -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  http://localhost:3000/api/processes
```

### Basic Authentication

Simple username/password authentication using HTTP Basic Auth.

#### Configuration
```toml
[security.auth]
method = "basic"
realm = "PMDaemon API"

[security.users.admin]
password_hash = "$2b$10$X8rqn5kG..."  # bcrypt hash
roles = ["admin"]

[security.users.operator]
password_hash = "$2b$10$Y9srm6hH..."
roles = ["operator"]
```

#### Usage
```bash
# Using curl with basic auth
curl -u admin:password http://localhost:3000/api/processes

# Using Authorization header
curl -H "Authorization: Basic YWRtaW46cGFzc3dvcmQ=" \
  http://localhost:3000/api/processes
```

### API Key Authentication

Long-lived API keys for service-to-service authentication.

#### Configuration
```toml
[security.auth]
method = "api_key"
header_name = "X-API-Key"  # or "Authorization"

[security.api_keys.service1]
key_hash = "$2b$10$..."
permissions = ["process:read"]
description = "Monitoring service"

[security.api_keys.deployment]
key_hash = "$2b$10$..."
permissions = ["process:*"]
description = "Deployment system"
```

#### Usage
```bash
# Using custom header
curl -H "X-API-Key: your-api-key" \
  http://localhost:3000/api/processes

# Using Authorization header
curl -H "Authorization: ApiKey your-api-key" \
  http://localhost:3000/api/processes
```

## Multi-Factor Authentication (MFA)

### TOTP (Time-based One-Time Password)

#### Configuration
```toml
[security.auth.mfa]
enabled = true
method = "totp"
issuer = "PMDaemon"
window = 1  # Allow 1 window tolerance
```

#### Setup Process
```bash
# Enable MFA for user
curl -X POST http://localhost:3000/auth/mfa/setup \
  -H "Authorization: Bearer token" \
  -d '{"method": "totp"}'

# Verify MFA setup
curl -X POST http://localhost:3000/auth/mfa/verify \
  -H "Authorization: Bearer token" \
  -d '{"code": "123456"}'
```

### SMS Authentication

#### Configuration
```toml
[security.auth.mfa]
enabled = true
method = "sms"
provider = "twilio"  # or "aws_sns"

[security.auth.mfa.sms.twilio]
account_sid = "your-twilio-sid"
auth_token = "your-twilio-token"
from_number = "+1234567890"
```

## OAuth 2.0 Integration

### OAuth Provider Configuration

#### GitHub OAuth
```toml
[security.auth.oauth.github]
enabled = true
client_id = "your-github-client-id"
client_secret = "your-github-client-secret"
redirect_uri = "http://localhost:3000/auth/github/callback"
scopes = ["user:email"]
```

#### Google OAuth
```toml
[security.auth.oauth.google]
enabled = true
client_id = "your-google-client-id"
client_secret = "your-google-client-secret"
redirect_uri = "http://localhost:3000/auth/google/callback"
scopes = ["openid", "profile", "email"]
```

### OAuth Flow Example
```bash
# Initiate OAuth flow
curl http://localhost:3000/auth/github

# Handle callback (automatic)
# User is redirected with authorization code

# Exchange code for token (handled by PMDaemon)
# User receives JWT or session token
```

## LDAP/Active Directory Integration

### LDAP Configuration
```toml
[security.auth.ldap]
enabled = true
url = "ldap://ldap.company.com:389"
bind_dn = "cn=admin,dc=company,dc=com"
bind_password = "admin-password"
search_base = "ou=users,dc=company,dc=com"
search_filter = "(uid={{username}})"
attributes = ["uid", "mail", "memberOf"]

# TLS Configuration
tls = true
ca_cert_file = "/path/to/ca.pem"
```

### User Mapping
```toml
[security.auth.ldap.mapping]
username_attribute = "uid"
email_attribute = "mail"
groups_attribute = "memberOf"

# Map LDAP groups to PMDaemon roles
[security.auth.ldap.group_mapping]
"CN=Administrators,OU=Groups,DC=company,DC=com" = "admin"
"CN=Operators,OU=Groups,DC=company,DC=com" = "operator"
```

## Session Management

### Session Configuration
```toml
[security.sessions]
enabled = true
store = "memory"  # or "redis", "file"
secret = "session-secret-key"
cookie_name = "pmdaemon_session"
max_age = 86400  # 24 hours
secure = true  # HTTPS only
http_only = true
same_site = "strict"
```

### Redis Session Store
```toml
[security.sessions.redis]
host = "localhost"
port = 6379
password = "redis-password"
db = 0
key_prefix = "pmd:session:"
```

## Rate Limiting

### Authentication Rate Limiting
```toml
[security.rate_limiting.auth]
enabled = true
max_attempts = 5
window_ms = 300000  # 5 minutes
block_duration = 900  # 15 minutes
```

### API Rate Limiting
```toml
[security.rate_limiting.api]
enabled = true
requests_per_minute = 60
burst = 10
skip_successful_requests = true
```

## Security Headers

### Authentication Security Headers
```toml
[security.headers]
# Prevent credential stuffing
strict_transport_security = "max-age=31536000; includeSubDomains"
x_frame_options = "DENY"
x_content_type_options = "nosniff"
x_xss_protection = "1; mode=block"
```

## Token Refresh and Revocation

### Token Refresh
```bash
# Refresh expired token
curl -X POST http://localhost:3000/auth/refresh \
  -H "Authorization: Bearer expired-token"
```

### Token Revocation
```bash
# Revoke token
curl -X POST http://localhost:3000/auth/revoke \
  -H "Authorization: Bearer token-to-revoke"

# Revoke all user tokens
curl -X POST http://localhost:3000/auth/revoke-all \
  -H "Authorization: Bearer admin-token" \
  -d '{"user_id": "target-user"}'
```

## Authentication Middleware

### Custom Authentication Middleware
```javascript
// Custom authentication plugin
class CustomAuthPlugin {
  async authenticate(request) {
    const token = request.headers.authorization;
    // Custom validation logic
    return {
      valid: true,
      user: { id: 'user123', roles: ['operator'] }
    };
  }
}

// Register plugin
pmdaemon.auth.registerPlugin('custom', new CustomAuthPlugin());
```

## Error Handling

### Authentication Errors
```json
{
  "error": "authentication_failed",
  "message": "Invalid credentials",
  "code": 401,
  "details": {
    "reason": "invalid_password",
    "attempts_remaining": 2
  }
}
```

### Common Error Codes
- `401` - Unauthorized (invalid credentials)
- `403` - Forbidden (insufficient permissions)
- `429` - Too Many Requests (rate limited)
- `422` - Unprocessable Entity (MFA required)

## Security Best Practices

### Token Security
1. **Use HTTPS**: Always use TLS in production
2. **Short Expiry**: Use short token lifetimes with refresh tokens
3. **Secure Storage**: Store tokens securely on client side
4. **Rotation**: Regularly rotate secrets and keys

### Password Security
1. **Strong Passwords**: Enforce password complexity
2. **Hashing**: Use bcrypt or similar for password hashing
3. **Salting**: Always use salt with password hashes
4. **Two-Factor**: Enable MFA for privileged accounts

### API Security
1. **Rate Limiting**: Implement aggressive rate limiting for auth endpoints
2. **Monitoring**: Monitor failed authentication attempts
3. **Audit Logging**: Log all authentication events
4. **Principle of Least Privilege**: Grant minimal necessary permissions

## Troubleshooting

### Common Issues
- Token expiration handling
- CORS configuration for web clients
- Clock skew with JWT timestamps
- LDAP connection timeouts

### Debug Configuration
```toml
[security.auth]
debug = true  # Enable debug logging
log_level = "debug"
```

For more security configurations, see our [Security Overview](../security/overview.md) guide.
