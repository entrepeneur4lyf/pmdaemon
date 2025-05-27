# Cross-Platform Support

PMDaemon is designed from the ground up to be truly cross-platform, providing native performance and full feature parity across Linux, Windows, and macOS.

## Platform Support Matrix

| Platform | Architecture | Status | Binary Name |
|----------|-------------|--------|-------------|
| **Linux** | x86_64 | ✅ Full Support | `pmdaemon-linux-x86_64` |
| **Windows** | x86_64 | ✅ Full Support | `pmdaemon-windows-x86_64.exe` |
| **macOS** | x86_64 (Intel) | ✅ Full Support | `pmdaemon-macos-x86_64` |
| **macOS** | aarch64 (Apple Silicon) | ✅ Full Support | `pmdaemon-macos-aarch64` |

## Key Advantages

### 🚀 Native Performance
- **No runtime dependencies** - Single binary, no Node.js required
- **Platform-optimized** - Uses native OS APIs for best performance
- **Small footprint** - ~15MB binary vs ~50MB+ for PM2 + Node.js
- **Fast startup** - ~100ms vs 2-3 seconds for PM2

### 🔧 Platform-Specific Optimizations

#### Linux
- **Signal Handling**: Full Unix signal support (SIGTERM, SIGINT, SIGUSR1, SIGUSR2, SIGKILL)
- **Process Management**: Native `nix` crate integration for optimal performance
- **File System**: POSIX-compliant operations
- **Container Ready**: Optimized for Docker and Kubernetes deployments

#### Windows
- **Signal Handling**: Native Ctrl+C handling and graceful shutdown
- **Process Management**: Windows APIs with `taskkill` integration
- **File System**: Native Windows path handling
- **Terminal Support**: Works with PowerShell, Command Prompt, and Windows Terminal

#### macOS
- **Universal Support**: Both Intel and Apple Silicon architectures
- **Signal Handling**: Full Unix signal support like Linux
- **Performance**: Native ARM64 performance on M1/M2/M3 Macs
- **Integration**: Works seamlessly with macOS development workflows

## Unified API

The same commands work identically across all platforms:

```bash
# These commands work exactly the same on Linux, Windows, and macOS
pmdaemon start app.js --name myapp
pmdaemon list
pmdaemon stop myapp
pmdaemon restart myapp
pmdaemon logs myapp
pmdaemon delete myapp
```

## Installation Options

### Pre-built Binaries

Download the appropriate binary for your platform:

```bash
# Linux
wget https://github.com/entrepeneur4lyf/pmdaemon/releases/latest/download/pmdaemon-linux-x86_64
chmod +x pmdaemon-linux-x86_64
sudo mv pmdaemon-linux-x86_64 /usr/local/bin/pmdaemon

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/entrepeneur4lyf/pmdaemon/releases/latest/download/pmdaemon-windows-x86_64.exe" -OutFile "pmdaemon.exe"

# macOS Intel
wget https://github.com/entrepeneur4lyf/pmdaemon/releases/latest/download/pmdaemon-macos-x86_64
chmod +x pmdaemon-macos-x86_64
sudo mv pmdaemon-macos-x86_64 /usr/local/bin/pmdaemon

# macOS Apple Silicon
wget https://github.com/entrepeneur4lyf/pmdaemon/releases/latest/download/pmdaemon-macos-aarch64
chmod +x pmdaemon-macos-aarch64
sudo mv pmdaemon-macos-aarch64 /usr/local/bin/pmdaemon
```

### From Source

Build for your current platform:

```bash
git clone https://github.com/entrepeneur4lyf/pmdaemon.git
cd pmdaemon
cargo build --release
```

### Cross-Compilation

Build for other platforms from Linux:

```bash
# Add targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu

# Build for macOS (requires macOS SDK)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

## Platform Differences

### Signal Handling

**Unix (Linux/macOS):**
```bash
# Full signal support
pmdaemon stop myapp    # Sends SIGTERM
pmdaemon kill myapp    # Sends SIGKILL
# Also supports SIGUSR1, SIGUSR2, SIGINT
```

**Windows:**
```cmd
# Uses Windows-native termination
pmdaemon stop myapp    # Uses taskkill
pmdaemon kill myapp    # Uses taskkill /F
# Ctrl+C handling for graceful shutdown
```

### File Paths

PMDaemon automatically handles platform-specific paths:

**Linux/macOS:**
- Config: `~/.config/pmdaemon/`
- Logs: `~/.local/share/pmdaemon/logs/`
- PIDs: `~/.local/share/pmdaemon/pids/`

**Windows:**
- Config: `%APPDATA%\pmdaemon\`
- Logs: `%LOCALAPPDATA%\pmdaemon\logs\`
- PIDs: `%LOCALAPPDATA%\pmdaemon\pids\`

### Process Management

All platforms support the same features:
- ✅ Process lifecycle management
- ✅ Auto-restart on crash
- ✅ Memory monitoring
- ✅ Health checks
- ✅ Port management
- ✅ Log management
- ✅ Configuration files

## Development Workflows

### Cross-Platform Development Teams

PMDaemon enables consistent tooling across diverse development environments:

```bash
# Developer on macOS
pmdaemon start --config ecosystem.json

# Developer on Windows
pmdaemon start --config ecosystem.json

# Developer on Linux
pmdaemon start --config ecosystem.json

# Same config file, same behavior, same output
```

### CI/CD Pipelines

Use the same PMDaemon commands in all CI environments:

```yaml
# GitHub Actions example
- name: Start services
  run: pmdaemon start --config ci-ecosystem.json --wait-ready
  # Works on ubuntu-latest, windows-latest, macos-latest
```

## Migration Benefits

### From PM2

**Before (PM2):**
- Requires Node.js on all platforms
- Windows support is problematic
- Different behavior across platforms
- Large resource footprint

**After (PMDaemon):**
- Native binaries for each platform
- Consistent behavior everywhere
- Better Windows support
- Smaller resource usage

### From Platform-Specific Tools

Replace multiple tools with one:
- **Linux**: systemd, supervisor → PMDaemon
- **Windows**: Windows Services, NSSM → PMDaemon  
- **macOS**: launchd, brew services → PMDaemon

## Best Practices

### 1. Use Configuration Files
```json
{
  "apps": [{
    "name": "myapp",
    "script": "node",
    "args": ["server.js"],
    "instances": 2,
    "port": "auto:3000-3100"
  }]
}
```

### 2. Platform-Agnostic Scripts
```bash
#!/bin/bash
# Works on all platforms with appropriate shell
pmdaemon start --config ecosystem.json
pmdaemon list
```

### 3. Health Checks
```bash
# Same health check works everywhere
pmdaemon start app.js \
  --health-check-url http://localhost:3000/health \
  --wait-ready
```

## Troubleshooting

### Platform-Specific Issues

**Linux:**
- Ensure binary has execute permissions: `chmod +x pmdaemon`
- Check PATH includes installation directory

**Windows:**
- Run as Administrator if needed for system-wide installation
- Add to PATH environment variable
- Use PowerShell or Command Prompt

**macOS:**
- Allow binary in Security & Privacy settings
- Use `sudo` for system-wide installation
- Ensure Xcode Command Line Tools are installed

### Common Solutions

1. **Permission Denied**: Use `chmod +x` on Unix or run as Administrator on Windows
2. **Command Not Found**: Add binary location to PATH
3. **Port Conflicts**: Use PMDaemon's auto port assignment
4. **Process Not Starting**: Check logs with `pmdaemon logs <name>`

---

PMDaemon's cross-platform design ensures you can use the same powerful process management tool regardless of your operating system, providing consistency and reliability across your entire infrastructure.
