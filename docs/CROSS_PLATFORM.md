# Cross-Platform Support

PMDaemon is designed to run natively on Linux, Windows, and macOS with full feature parity across all platforms.

## Supported Platforms

| Platform | Architecture | Status | Notes |
|----------|-------------|--------|-------|
| Linux | x86_64 | ✅ Full Support | Native Unix signal handling |
| Windows | x86_64 | ✅ Full Support | Native Windows APIs, taskkill integration |
| macOS | x86_64 (Intel) | ✅ Full Support | Optimized for Intel Macs |
| macOS | aarch64 (Apple Silicon) | ✅ Full Support | Native Apple Silicon support |

## Platform-Specific Features

### Linux
- **Signal Handling**: Full Unix signal support (SIGTERM, SIGINT, SIGUSR1, SIGUSR2, SIGKILL)
- **Process Management**: Native `nix` crate integration for optimal performance
- **File System**: POSIX-compliant file operations
- **Performance**: Optimized for server environments

### Windows
- **Signal Handling**: Ctrl+C handling and graceful shutdown
- **Process Management**: Native Windows APIs with `taskkill` integration
- **File System**: Windows-native path handling
- **Compatibility**: Works with PowerShell, Command Prompt, and Windows Terminal

### macOS
- **Signal Handling**: Full Unix signal support like Linux
- **Process Management**: Optimized for both Intel and Apple Silicon
- **File System**: POSIX-compliant with macOS-specific optimizations
- **Performance**: Native performance on both architectures

## Building for Different Platforms

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add cross-compilation targets
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### Cross-Compilation Commands

#### Linux (Native)
```bash
cargo build --release
```

#### Windows (from Linux)
```bash
# Install mingw-w64 for Windows cross-compilation
sudo apt install gcc-mingw-w64-x86-64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

#### macOS (requires macOS machine or CI)
```bash
# Intel Macs
cargo build --release --target x86_64-apple-darwin

# Apple Silicon Macs
cargo build --release --target aarch64-apple-darwin
```

## Platform Differences

### Signal Handling
- **Unix (Linux/macOS)**: Uses `nix` crate for comprehensive signal handling
- **Windows**: Uses Ctrl+C handling and `taskkill` for process termination

### Process Termination
- **Unix**: Graceful SIGTERM followed by SIGKILL if needed
- **Windows**: Uses `taskkill /F` for force termination

### File Paths
- **Unix**: Forward slashes (`/`) and POSIX paths
- **Windows**: Backslashes (`\`) and Windows paths (automatically handled)

## Testing Cross-Platform Compatibility

All features are tested across platforms in CI:

```bash
# Run tests on current platform
cargo test

# Test specific features
cargo test --test integration_tests
cargo test --test e2e_tests
```

## Deployment Recommendations

### Linux Servers
- Use the native Linux binary for optimal performance
- Ideal for production deployments and containers

### Windows Development
- Use the Windows binary for local development
- Works with all Windows development tools

### macOS Development
- Use the appropriate binary for your Mac architecture
- Excellent for local development and testing

## Known Limitations

1. **Cross-compilation**: Building macOS binaries requires a macOS machine or CI
2. **Windows MSVC**: Requires Visual Studio Build Tools for MSVC target
3. **Signal differences**: Some Unix-specific signals are not available on Windows

## Future Enhancements

- **ARM64 Windows**: Support for ARM64 Windows when Rust ecosystem matures
- **FreeBSD/OpenBSD**: Potential Unix-like platform support
- **Container optimization**: Platform-specific container optimizations
