@echo off
REM PMDaemon Comprehensive Configuration Examples (Windows Batch)
REM This script demonstrates all available configuration options with their defaults
REM
REM Default Values Reference:
REM - instances: 1
REM - autorestart: true
REM - max_restarts: 16
REM - min_uptime: 1000ms (1 second)
REM - restart_delay: 0ms
REM - kill_timeout: 1600ms (1.6 seconds)
REM - exec_mode: fork (automatically becomes cluster when instances > 1)
REM - namespace: "default"
REM - watch: false (not yet implemented)
REM - web_port: 9615
REM - monitor_interval: 1 second
REM - log_lines: 20

echo === PMDaemon Configuration Examples (Windows Batch) ===
echo This script demonstrates all available CLI options and their defaults
echo.

REM Basic process start (minimal configuration)
echo 1. Basic Process Start (minimal configuration)
echo pmdaemon start node server.js
echo REM Uses defaults: instances=1, autorestart=true, max_restarts=16, etc.
echo.

REM Named process with explicit name
echo 2. Named Process
echo pmdaemon start --name my-web-server node server.js
echo.

REM Multiple instances (cluster mode)
echo 3. Cluster Mode (multiple instances)
echo pmdaemon start --name web-cluster --instances 4 node server.js
echo REM Automatically sets exec_mode to 'cluster' when instances ^> 1
echo.

REM Working directory specification
echo 4. Custom Working Directory
echo pmdaemon start --name my-app --cwd C:\path\to\app node server.js
echo.

REM Environment variables
echo 5. Environment Variables
echo pmdaemon start --name my-app --env NODE_ENV=production --env PORT=3000 --env DEBUG=true node server.js
echo REM Multiple --env flags can be used
echo REM Format: --env KEY=VALUE
echo.

REM Memory limit configuration
echo 6. Memory Limit Configuration
echo pmdaemon start --name memory-limited --max-memory 512M node server.js
echo REM Supported formats: 100K, 100KB, 100M, 100MB, 100G, 100GB, or raw bytes
echo REM Process will be restarted if memory usage exceeds this limit
echo.

REM Port configuration examples
echo 7. Port Configuration Examples
echo REM Single port:
echo pmdaemon start --name single-port --port 3000 node server.js
echo.
echo REM Port range (for cluster mode):
echo pmdaemon start --name port-range --instances 4 --port 3000-3003 node server.js
echo.
echo REM Auto port assignment:
echo pmdaemon start --name auto-port --instances 3 --port auto:4000-4100 node server.js
echo.

REM Complex example with all options
echo 8. Complex Configuration (all options)
echo pmdaemon start ^
echo   --name production-app ^
echo   --instances 4 ^
echo   --cwd C:\opt\myapp ^
echo   --env NODE_ENV=production ^
echo   --env DATABASE_URL=postgres://localhost/mydb ^
echo   --env REDIS_URL=redis://localhost:6379 ^
echo   --max-memory 1G ^
echo   --port auto:3000-3100 ^
echo   node ^
echo   server.js --cluster --verbose
echo.

REM Process management commands
echo 9. Process Management Commands
echo REM List all processes:
echo pmdaemon list
echo.
echo REM Stop a process:
echo pmdaemon stop my-app
echo.
echo REM Restart with port override:
echo pmdaemon restart my-app --port 4000
echo.
echo REM Graceful reload with port override:
echo pmdaemon reload my-app --port auto:5000-5100
echo.
echo REM Delete a single process:
echo pmdaemon delete my-app
echo.
echo REM Delete by status (with confirmation):
echo pmdaemon delete stopped --status
echo.
echo REM Delete all processes (with confirmation):
echo pmdaemon delete all
echo.
echo REM Force delete without confirmation:
echo pmdaemon delete all --force
echo.

REM Monitoring commands
echo 10. Monitoring Commands
echo REM Real-time monitoring (default 1-second intervals):
echo pmdaemon monit
echo.
echo REM Custom monitoring interval:
echo pmdaemon monit --interval 5
echo.
echo REM View logs (default 20 lines):
echo pmdaemon logs my-app
echo.
echo REM View more log lines:
echo pmdaemon logs my-app --lines 100
echo.
echo REM Follow logs in real-time:
echo pmdaemon logs my-app --follow
echo.
echo REM Process information:
echo pmdaemon info my-app
echo.

REM Web monitoring server
echo 11. Web Monitoring Server
echo REM Start web server (default port 9615, host 127.0.0.1):
echo pmdaemon web
echo.
echo REM Custom port and host:
echo pmdaemon web --port 8080 --host 0.0.0.0
echo.

REM Global options
echo 12. Global Options
echo REM Verbose logging:
echo pmdaemon --verbose start --name debug-app node server.js
echo.
echo REM Note: --config option exists but is not yet implemented
echo.

REM Advanced examples with different applications
echo 13. Advanced Application Examples
echo.
echo REM Python application:
echo pmdaemon start --name python-api --instances 2 --port auto:8000-8100 --env PYTHONPATH=C:\opt\myapi python -m uvicorn main:app --host 0.0.0.0
echo.
echo REM .NET application:
echo pmdaemon start --name dotnet-api --max-memory 256M --port 9090 dotnet MyService.dll --urls http://0.0.0.0:9090
echo.
echo REM PowerShell script:
echo pmdaemon start --name ps-service --instances 2 --port 6000-6001 --cwd C:\opt\ps-app powershell -File service.ps1
echo.
echo REM Static file server (Python):
echo pmdaemon start --name static-server --port 8080 python -m http.server 8080
echo.

echo === Default Values Summary ===
echo instances: 1
echo autorestart: true
echo max_restarts: 16
echo min_uptime: 1000ms (1 second)
echo restart_delay: 0ms
echo kill_timeout: 1600ms (1.6 seconds)
echo exec_mode: fork (auto-cluster when instances ^> 1)
echo namespace: 'default'
echo watch: false (not implemented)
echo web_port: 9615
echo web_host: 127.0.0.1
echo monitor_interval: 1 second
echo log_lines: 20
echo max_memory_restart: none (unlimited)
echo port: none (no port assignment)
echo cwd: current directory
echo env: empty (inherits from parent)
echo.

echo === Windows-Specific Notes ===
echo - Use caret (^) for line continuation in batch files
echo - Use double quotes for paths with spaces: --cwd "C:\Program Files\MyApp"
echo - Environment variables can reference Windows variables: --env PATH=%PATH%
echo - Use start command for background execution if needed
echo.

echo === General Notes ===
echo - Process names must be unique within a namespace
echo - PMDaemon automatically adds PORT, PM2_INSTANCE_ID, and NODE_APP_INSTANCE environment variables
echo - Cluster mode is automatically enabled when instances ^> 1
echo - Port ranges must have enough ports for all instances
echo - Auto port assignment finds the first available consecutive ports
echo - Memory limits trigger automatic restarts when exceeded
echo - Log files are auto-generated in the logs/ directory if not specified
echo - PID files are auto-generated in the pids/ directory if not specified

pause
