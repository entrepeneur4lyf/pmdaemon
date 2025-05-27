# Installation

PMDaemon can be installed through multiple methods. Choose the one that works best for your environment.

## Prerequisites

- **Rust 1.70+** (for building from source)
- **Operating System**: Linux, macOS, or Windows
- **Architecture**: x86_64 or ARM64

## Method 1: Install from Crates.io (Recommended)

The easiest way to install PMDaemon is using Cargo:

```bash
cargo install pmdaemon
```

This will:
- Download and compile the latest stable version
- Install the `pmdaemon` binary to `~/.cargo/bin/`
- Make it available in your PATH (if Cargo is configured correctly)

### Verify Installation

```bash
pmdaemon --version
# Output: pmdaemon 0.1.2
```

## Method 2: Build from Source

For the latest development version or custom builds:

```bash
# Clone the repository
git clone https://github.com/entrepeneur4lyf/pmdaemon.git
cd pmdaemon

# Build in release mode
cargo build --release

# Install to system (optional)
sudo cp target/release/pmdaemon /usr/local/bin/
```

### Development Build

For development or testing:

```bash
# Build in debug mode (faster compilation)
cargo build

# Run directly
./target/debug/pmdaemon --help
```

## Method 3: Pre-built Binaries

Download pre-built binaries from the [GitHub Releases](https://github.com/entrepeneur4lyf/pmdaemon/releases) page:

### Linux (x86_64)
```bash
wget https://github.com/entrepeneur4lyf/pmdaemon/releases/latest/download/pmdaemon-linux-x86_64.tar.gz
tar -xzf pmdaemon-linux-x86_64.tar.gz
sudo mv pmdaemon /usr/local/bin/
```

### macOS (x86_64)
```bash
wget https://github.com/entrepeneur4lyf/pmdaemon/releases/latest/download/pmdaemon-macos-x86_64.tar.gz
tar -xzf pmdaemon-macos-x86_64.tar.gz
sudo mv pmdaemon /usr/local/bin/
```

### Windows
Download `pmdaemon-windows-x86_64.zip` and extract to a directory in your PATH.

## Method 4: Package Managers

### Homebrew (macOS/Linux)
```bash
# Coming soon
brew install pmdaemon
```

### Chocolatey (Windows)
```bash
# Coming soon
choco install pmdaemon
```

### APT (Ubuntu/Debian)
```bash
# Coming soon
sudo apt install pmdaemon
```

## Configuration Setup

After installation, PMDaemon will create its configuration directories on first run:

```bash
# Linux/macOS
~/.config/pmdaemon/
~/.local/share/pmdaemon/logs/
~/.local/share/pmdaemon/pids/

# Windows
%APPDATA%\pmdaemon\
%LOCALAPPDATA%\pmdaemon\logs\
%LOCALAPPDATA%\pmdaemon\pids\
```

## Verify Installation

Test that PMDaemon is working correctly:

```bash
# Check version
pmdaemon --version

# View help
pmdaemon --help

# Start a simple process
pmdaemon start "echo 'Hello PMDaemon'" --name test

# List processes
pmdaemon list

# Clean up
pmdaemon delete test
```

## Troubleshooting

### Command Not Found

If you get `command not found: pmdaemon`:

1. **Check Cargo bin directory is in PATH**:
   ```bash
   echo $PATH | grep -q "$HOME/.cargo/bin" && echo "✓ Cargo bin in PATH" || echo "✗ Add ~/.cargo/bin to PATH"
   ```

2. **Add to PATH** (add to your shell profile):
   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

3. **Reload shell**:
   ```bash
   source ~/.bashrc  # or ~/.zshrc
   ```

### Permission Denied

If you get permission errors:

```bash
# Make binary executable
chmod +x /path/to/pmdaemon

# Or install to user directory
mkdir -p ~/.local/bin
cp pmdaemon ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
```

### Build Errors

Common build issues:

1. **Rust version too old**:
   ```bash
   rustup update
   ```

2. **Missing system dependencies** (Linux):
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install build-essential pkg-config libssl-dev

   # CentOS/RHEL
   sudo yum groupinstall "Development Tools"
   sudo yum install openssl-devel
   ```

3. **Windows build tools**:
   - Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
   - Or use [rustup-init.exe](https://rustup.rs/) which handles this automatically

## Updating PMDaemon

### From Crates.io
```bash
cargo install pmdaemon --force
```

### From Source
```bash
cd pmdaemon
git pull origin main
cargo build --release
sudo cp target/release/pmdaemon /usr/local/bin/
```

## Uninstalling

### Cargo Installation
```bash
cargo uninstall pmdaemon
```

### Manual Installation
```bash
# Remove binary
sudo rm /usr/local/bin/pmdaemon

# Remove configuration (optional)
rm -rf ~/.config/pmdaemon
rm -rf ~/.local/share/pmdaemon
```

## Next Steps

Now that PMDaemon is installed:

1. **[Quick Start](./quick-start.md)** - Get up and running in 5 minutes
2. **[CLI Commands](../cli/commands.md)** - Learn the available commands
3. **[Configuration](../configuration/ecosystem-files.md)** - Set up configuration files

---

Need help? Check the [troubleshooting guide](../advanced/troubleshooting.md) or [open an issue](https://github.com/entrepeneur4lyf/pmdaemon/issues).
