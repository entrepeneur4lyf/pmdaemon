# PMDaemon Comprehensive Configuration Examples (PowerShell)
# This script demonstrates all available configuration options with their defaults
#
# Default Values Reference:
# - instances: 1
# - autorestart: true
# - max_restarts: 16
# - min_uptime: 1000ms (1 second)
# - restart_delay: 0ms
# - kill_timeout: 1600ms (1.6 seconds)
# - exec_mode: fork (automatically becomes cluster when instances > 1)
# - namespace: "default"
# - watch: false (not yet implemented)
# - web_port: 9615
# - monitor_interval: 1 second
# - log_lines: 20

Write-Host "=== PMDaemon Configuration Examples (PowerShell) ===" -ForegroundColor Green
Write-Host "This script demonstrates all available CLI options and their defaults"
Write-Host

# Basic process start (minimal configuration)
Write-Host "1. Basic Process Start (minimal configuration)" -ForegroundColor Yellow
Write-Host "pmdaemon start node server.js"
Write-Host "# Uses defaults: instances=1, autorestart=true, max_restarts=16, etc." -ForegroundColor Gray
Write-Host

# Named process with explicit name
Write-Host "2. Named Process" -ForegroundColor Yellow
Write-Host "pmdaemon start --name my-web-server node server.js"
Write-Host

# Multiple instances (cluster mode)
Write-Host "3. Cluster Mode (multiple instances)" -ForegroundColor Yellow
Write-Host "pmdaemon start --name web-cluster --instances 4 node server.js"
Write-Host "# Automatically sets exec_mode to 'cluster' when instances > 1" -ForegroundColor Gray
Write-Host

# Working directory specification
Write-Host "4. Custom Working Directory" -ForegroundColor Yellow
Write-Host "pmdaemon start --name my-app --cwd C:\path\to\app node server.js"
Write-Host

# Environment variables
Write-Host "5. Environment Variables" -ForegroundColor Yellow
Write-Host "pmdaemon start --name my-app --env NODE_ENV=production --env PORT=3000 --env DEBUG=true node server.js"
Write-Host "# Multiple --env flags can be used" -ForegroundColor Gray
Write-Host "# Format: --env KEY=VALUE" -ForegroundColor Gray
Write-Host

# Memory limit configuration
Write-Host "6. Memory Limit Configuration" -ForegroundColor Yellow
Write-Host "pmdaemon start --name memory-limited --max-memory 512M node server.js"
Write-Host "# Supported formats: 100K, 100KB, 100M, 100MB, 100G, 100GB, or raw bytes" -ForegroundColor Gray
Write-Host "# Process will be restarted if memory usage exceeds this limit" -ForegroundColor Gray
Write-Host

# Port configuration examples
Write-Host "7. Port Configuration Examples" -ForegroundColor Yellow
Write-Host "# Single port:"
Write-Host "pmdaemon start --name single-port --port 3000 node server.js"
Write-Host
Write-Host "# Port range (for cluster mode):"
Write-Host "pmdaemon start --name port-range --instances 4 --port 3000-3003 node server.js"
Write-Host
Write-Host "# Auto port assignment:"
Write-Host "pmdaemon start --name auto-port --instances 3 --port auto:4000-4100 node server.js"
Write-Host

# Complex example with all options
Write-Host "8. Complex Configuration (all options)" -ForegroundColor Yellow
Write-Host @"
pmdaemon start `
  --name production-app `
  --instances 4 `
  --cwd C:\opt\myapp `
  --env NODE_ENV=production `
  --env DATABASE_URL=postgres://localhost/mydb `
  --env REDIS_URL=redis://localhost:6379 `
  --max-memory 1G `
  --port auto:3000-3100 `
  node `
  server.js --cluster --verbose
"@
Write-Host

# Process management commands
Write-Host "9. Process Management Commands" -ForegroundColor Yellow
Write-Host "# List all processes:"
Write-Host "pmdaemon list"
Write-Host
Write-Host "# Stop a process:"
Write-Host "pmdaemon stop my-app"
Write-Host
Write-Host "# Restart with port override:"
Write-Host "pmdaemon restart my-app --port 4000"
Write-Host
Write-Host "# Graceful reload with port override:"
Write-Host "pmdaemon reload my-app --port auto:5000-5100"
Write-Host
Write-Host "# Delete a single process:"
Write-Host "pmdaemon delete my-app"
Write-Host
Write-Host "# Delete by status (with confirmation):"
Write-Host "pmdaemon delete stopped --status"
Write-Host
Write-Host "# Delete all processes (with confirmation):"
Write-Host "pmdaemon delete all"
Write-Host
Write-Host "# Force delete without confirmation:"
Write-Host "pmdaemon delete all --force"
Write-Host

# Monitoring commands
Write-Host "10. Monitoring Commands" -ForegroundColor Yellow
Write-Host "# Real-time monitoring (default 1-second intervals):"
Write-Host "pmdaemon monit"
Write-Host
Write-Host "# Custom monitoring interval:"
Write-Host "pmdaemon monit --interval 5"
Write-Host
Write-Host "# View logs (default 20 lines):"
Write-Host "pmdaemon logs my-app"
Write-Host
Write-Host "# View more log lines:"
Write-Host "pmdaemon logs my-app --lines 100"
Write-Host
Write-Host "# Follow logs in real-time:"
Write-Host "pmdaemon logs my-app --follow"
Write-Host
Write-Host "# Process information:"
Write-Host "pmdaemon info my-app"
Write-Host

# Web monitoring server
Write-Host "11. Web Monitoring Server" -ForegroundColor Yellow
Write-Host "# Start web server (default port 9615, host 127.0.0.1):"
Write-Host "pmdaemon web"
Write-Host
Write-Host "# Custom port and host:"
Write-Host "pmdaemon web --port 8080 --host 0.0.0.0"
Write-Host

# Global options
Write-Host "12. Global Options" -ForegroundColor Yellow
Write-Host "# Verbose logging:"
Write-Host "pmdaemon --verbose start --name debug-app node server.js"
Write-Host
Write-Host "# Note: --config option exists but is not yet implemented"
Write-Host

# Advanced examples with different applications
Write-Host "13. Advanced Application Examples" -ForegroundColor Yellow
Write-Host
Write-Host "# Python application:"
Write-Host "pmdaemon start --name python-api --instances 2 --port auto:8000-8100 --env PYTHONPATH=C:\opt\myapi python -m uvicorn main:app --host 0.0.0.0"
Write-Host
Write-Host "# .NET application:"
Write-Host "pmdaemon start --name dotnet-api --max-memory 256M --port 9090 dotnet MyService.dll --urls http://0.0.0.0:9090"
Write-Host
Write-Host "# PowerShell script:"
Write-Host "pmdaemon start --name ps-service --instances 2 --port 6000-6001 --cwd C:\opt\ps-app powershell -File service.ps1"
Write-Host
Write-Host "# Static file server (Python):"
Write-Host "pmdaemon start --name static-server --port 8080 python -m http.server 8080"
Write-Host

Write-Host "=== Default Values Summary ===" -ForegroundColor Cyan
Write-Host "instances: 1"
Write-Host "autorestart: true"
Write-Host "max_restarts: 16"
Write-Host "min_uptime: 1000ms (1 second)"
Write-Host "restart_delay: 0ms"
Write-Host "kill_timeout: 1600ms (1.6 seconds)"
Write-Host "exec_mode: fork (auto-cluster when instances > 1)"
Write-Host "namespace: 'default'"
Write-Host "watch: false (not implemented)"
Write-Host "web_port: 9615"
Write-Host "web_host: 127.0.0.1"
Write-Host "monitor_interval: 1 second"
Write-Host "log_lines: 20"
Write-Host "max_memory_restart: none (unlimited)"
Write-Host "port: none (no port assignment)"
Write-Host "cwd: current directory"
Write-Host "env: empty (inherits from parent)"
Write-Host

Write-Host "=== PowerShell-Specific Notes ===" -ForegroundColor Cyan
Write-Host "- Use backticks (`) for line continuation in PowerShell"
Write-Host "- Use double quotes for paths with spaces: --cwd `"C:\Program Files\MyApp`""
Write-Host "- Environment variables can reference PowerShell variables: --env PATH=`$env:PATH"
Write-Host "- Use Start-Process for background execution if needed"
Write-Host

Write-Host "=== General Notes ===" -ForegroundColor Cyan
Write-Host "- Process names must be unique within a namespace"
Write-Host "- PMDaemon automatically adds PORT, PM2_INSTANCE_ID, and NODE_APP_INSTANCE environment variables"
Write-Host "- Cluster mode is automatically enabled when instances > 1"
Write-Host "- Port ranges must have enough ports for all instances"
Write-Host "- Auto port assignment finds the first available consecutive ports"
Write-Host "- Memory limits trigger automatic restarts when exceeded"
Write-Host "- Log files are auto-generated in the logs/ directory if not specified"
Write-Host "- PID files are auto-generated in the pids/ directory if not specified"
