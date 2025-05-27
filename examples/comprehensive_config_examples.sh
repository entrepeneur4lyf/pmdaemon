#!/usr/bin/env bash
set -euo pipefail
set -euo pipefail
# PMDaemon Comprehensive Configuration Examples
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

echo "=== PMDaemon Configuration Examples ==="
echo "This script demonstrates all available CLI options and their defaults"
echo

# Basic process start (minimal configuration)
echo "1. Basic Process Start (minimal configuration)"
echo "pmdaemon start node server.js"
echo "# Uses defaults: instances=1, autorestart=true, max_restarts=16, etc."
echo

# Named process with explicit name
echo "2. Named Process"
echo "pmdaemon start --name my-web-server node server.js"
echo

# Multiple instances (cluster mode)
echo "3. Cluster Mode (multiple instances)"
echo "pmdaemon start --name web-cluster --instances 4 node server.js"
echo "# Automatically sets exec_mode to 'cluster' when instances > 1"
echo

# Working directory specification
echo "4. Custom Working Directory"
echo "pmdaemon start --name my-app --cwd /path/to/app node server.js"
echo

# Environment variables
echo "5. Environment Variables"
echo "pmdaemon start --name my-app --env NODE_ENV=production --env PORT=3000 --env DEBUG=true node server.js"
echo "# Multiple --env flags can be used"
echo "# Format: --env KEY=VALUE"
echo

# Memory limit configuration
echo "6. Memory Limit Configuration"
echo "pmdaemon start --name memory-limited --max-memory 512M node server.js"
echo "# Supported formats: 100K, 100KB, 100M, 100MB, 100G, 100GB, or raw bytes"
echo "# Process will be restarted if memory usage exceeds this limit"
echo

# Port configuration examples
echo "7. Port Configuration Examples"
echo "# Single port:"
echo "pmdaemon start --name single-port --port 3000 node server.js"
echo
echo "# Port range (for cluster mode):"
echo "pmdaemon start --name port-range --instances 4 --port 3000-3003 node server.js"
echo
echo "# Auto port assignment:"
echo "pmdaemon start --name auto-port --instances 3 --port auto:4000-4100 node server.js"
echo

# Complex example with all options
echo "8. Complex Configuration (all options)"
echo "pmdaemon start \\"
echo "  --name production-app \\"
echo "  --instances 4 \\"
echo "  --cwd /opt/myapp \\"
echo "  --env NODE_ENV=production \\"
echo "  --env DATABASE_URL=postgres://localhost/mydb \\"
echo "  --env REDIS_URL=redis://localhost:6379 \\"
echo "  --max-memory 1G \\"
echo "  --port auto:3000-3100 \\"
echo "  node \\"
echo "  server.js --cluster --verbose"
echo

# Process management commands
echo "9. Process Management Commands"
echo "# List all processes:"
echo "pmdaemon list"
echo
echo "# Stop a process:"
echo "pmdaemon stop my-app"
echo
echo "# Restart with port override:"
echo "pmdaemon restart my-app --port 4000"
echo
echo "# Graceful reload with port override:"
echo "pmdaemon reload my-app --port auto:5000-5100"
echo
echo "# Delete a single process:"
echo "pmdaemon delete my-app"
echo
echo "# Delete by status (with confirmation):"
echo "pmdaemon delete stopped --status"
echo
echo "# Delete all processes (with confirmation):"
echo "pmdaemon delete all"
echo
echo "# Force delete without confirmation:"
echo "pmdaemon delete all --force"
echo

# Monitoring commands
echo "10. Monitoring Commands"
echo "# Real-time monitoring (default 1-second intervals):"
echo "pmdaemon monit"
echo
echo "# Custom monitoring interval:"
echo "pmdaemon monit --interval 5"
echo
echo "# View logs (default 20 lines):"
echo "pmdaemon logs my-app"
echo
echo "# View more log lines:"
echo "pmdaemon logs my-app --lines 100"
echo
echo "# Follow logs in real-time:"
echo "pmdaemon logs my-app --follow"
echo
echo "# Process information:"
echo "pmdaemon info my-app"
echo

# Web monitoring server
echo "11. Web Monitoring Server"
echo "# Start web server (default port 9615, host 127.0.0.1):"
echo "pmdaemon web"
echo
echo "# Custom port and host:"
echo "pmdaemon web --port 8080 --host 0.0.0.0"
echo

# Global options
echo "12. Global Options"
echo "# Verbose logging:"
echo "pmdaemon --verbose start --name debug-app node server.js"
echo
# - Note: --config option is supported (configure via --config <file>)
echo

# Advanced examples with different applications
echo "13. Advanced Application Examples"
echo
echo "# Python application:"
echo "pmdaemon start --name python-api --instances 2 --port auto:8000-8100 --env PYTHONPATH=/opt/myapi python -m uvicorn main:app --host 0.0.0.0"
echo
echo "# Rust application:"
echo "pmdaemon start --name rust-service --max-memory 256M --port 9090 ./target/release/my-service --workers 4"
echo
echo "# Go application:"
echo "pmdaemon start --name go-microservice --instances 3 --port 6000-6002 --cwd /opt/go-app ./my-service -port"
echo
echo "# Static file server:"
echo "pmdaemon start --name static-server --port 8080 python -m http.server 8080"
echo

echo "=== Default Values Summary ==="
echo "instances: 1"
echo "autorestart: true"
echo "max_restarts: 16"
echo "min_uptime: 1000ms (1 second)"
echo "restart_delay: 0ms"
echo "kill_timeout: 1600ms (1.6 seconds)"
echo "exec_mode: fork (auto-cluster when instances > 1)"
echo "namespace: 'default'"
echo "watch: false (not implemented)"
echo "web_port: 9615"
echo "web_host: 127.0.0.1"
echo "monitor_interval: 1 second"
echo "log_lines: 20"
echo "max_memory_restart: none (unlimited)"
echo "port: none (no port assignment)"
echo "cwd: current directory"
echo "env: empty (inherits from parent)"
echo

echo "=== Notes ==="
echo "- Process names must be unique within a namespace"
echo "- PMDaemon automatically adds PORT, PM2_INSTANCE_ID, and NODE_APP_INSTANCE environment variables"
echo "- Cluster mode is automatically enabled when instances > 1"
echo "- Port ranges must have enough ports for all instances"
echo "- Auto port assignment finds the first available consecutive ports"
echo "- Memory limits trigger automatic restarts when exceeded"
echo "- Log files are auto-generated in the logs/ directory if not specified"
echo "- PID files are auto-generated in the pids/ directory if not specified"
