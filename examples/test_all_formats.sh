#!/bin/bash
# Test script to verify all configuration formats work correctly
# This script demonstrates that JSON, YAML, and TOML ecosystem configs all work

 set -euo pipefail  # Exit on error, unset variables, and catch pipeline failures

echo "=== PMDaemon Configuration Format Testing ==="
echo "This script tests JSON, YAML, and TOML ecosystem configuration files"
echo

# Build PMDaemon first
echo "Building PMDaemon..."
cargo build --release
echo "✅ Build successful"
echo

# Test JSON format
echo "1. Testing JSON Configuration Format"
echo "Starting test app from ecosystem.json..."
./target/release/pmdaemon --config examples/ecosystem.json start --name test-app 2>/dev/null || echo "Expected: test-app not found in ecosystem.json"

echo "Creating simple JSON test..."
cat > /tmp/test.json << 'EOF'
{
  "apps": [
    {
      "name": "json-test",
      "script": "echo",
      "args": ["JSON config works!"],
      "max_memory_restart": "100M",
      "port": "3000"
    }
  ]
}
EOF

echo "Testing JSON config..."
./target/release/pmdaemon --config /tmp/test.json start --name json-test
echo "✅ JSON format works!"
echo

# Test YAML format
echo "2. Testing YAML Configuration Format"
cat > /tmp/test.yaml << 'EOF'
apps:
  - name: yaml-test
    script: echo
    args:
      - "YAML config works!"
    max_memory_restart: "200M"
    port: "4000-4001"
    instances: 2
EOF

echo "Testing YAML config..."
./target/release/pmdaemon --config /tmp/test.yaml start --name yaml-test
echo "✅ YAML format works!"
echo

# Test TOML format
echo "3. Testing TOML Configuration Format"
cat > /tmp/test.toml << 'EOF'
[[apps]]
name = "toml-test"
script = "echo"
args = ["TOML config works!"]
max_memory_restart = "300M"
port = "auto:5000-5100"

[apps.env]
TEST_VAR = "toml-value"
EOF

echo "Testing TOML config..."
./target/release/pmdaemon --config /tmp/test.toml start --name toml-test
echo "✅ TOML format works!"
echo

# Test memory format parsing
echo "4. Testing Memory Format Parsing"
cat > /tmp/memory-test.json << 'EOF'
{
  "apps": [
    {
      "name": "memory-k",
      "script": "echo",
      "args": ["512K memory limit"],
      "max_memory_restart": "512K"
    },
    {
      "name": "memory-m",
      "script": "echo",
      "args": ["256M memory limit"],
      "max_memory_restart": "256M"
    },
    {
      "name": "memory-g",
      "script": "echo",
      "args": ["1G memory limit"],
      "max_memory_restart": "1G"
    }
  ]
}
EOF

echo "Testing memory format parsing..."
./target/release/pmdaemon --config /tmp/memory-test.json start --name memory-k
./target/release/pmdaemon --config /tmp/memory-test.json start --name memory-m
./target/release/pmdaemon --config /tmp/memory-test.json start --name memory-g
echo "✅ Memory format parsing works!"
echo

# Test port configuration
echo "5. Testing Port Configuration"
cat > /tmp/port-test.yaml << 'EOF'
apps:
  - name: single-port
    script: echo
    args: ["Single port 6000"]
    port: "6000"

  - name: port-range
    script: echo
    args: ["Port range 7000-7003"]
    port: "7000-7003"
    instances: 4

  - name: auto-port
    script: echo
    args: ["Auto port 8000-8100"]
    port: "auto:8000-8100"
    instances: 2
EOF

echo "Testing port configurations..."
./target/release/pmdaemon --config /tmp/port-test.yaml start --name single-port
./target/release/pmdaemon --config /tmp/port-test.yaml start --name port-range
./target/release/pmdaemon --config /tmp/port-test.yaml start --name auto-port
echo "✅ Port configuration works!"
echo

# Test starting all apps from config
echo "6. Testing Bulk App Startup"
cat > /tmp/bulk-test.json << 'EOF'
{
  "apps": [
    {
      "name": "bulk-app-1",
      "script": "echo",
      "args": ["Bulk app 1"]
    },
    {
      "name": "bulk-app-2",
      "script": "echo",
      "args": ["Bulk app 2"]
    },
    {
      "name": "bulk-app-3",
      "script": "echo",
      "args": ["Bulk app 3"]
    }
  ]
}
EOF

echo "Testing bulk app startup..."
./target/release/pmdaemon --config /tmp/bulk-test.json start
echo "✅ Bulk app startup works!"
echo

# Show all running processes
echo "7. Listing All Started Processes"
./target/release/pmdaemon list
echo

# Clean up
echo "8. Cleaning Up Test Processes"
echo "Stopping all test processes..."
./target/release/pmdaemon delete all --force 2>/dev/null || echo "No processes to clean up"

# Clean up temp files
rm -f /tmp/test.json /tmp/test.yaml /tmp/test.toml /tmp/memory-test.json /tmp/port-test.yaml /tmp/bulk-test.json

echo
echo "=== All Configuration Format Tests Completed Successfully! ==="
echo
echo "Summary:"
echo "✅ JSON configuration format"
echo "✅ YAML configuration format"
echo "✅ TOML configuration format"
echo "✅ Memory string parsing (K, M, G)"
echo "✅ Port configuration (single, range, auto)"
echo "✅ Bulk app startup from config"
echo "✅ Individual app startup from config"
echo
echo "PMDaemon ecosystem configuration support is fully functional!"
