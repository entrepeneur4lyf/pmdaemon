#!/bin/bash

# PMDaemon CLI Status Display Test Script
# This script tests all status items in the CLI UI to ensure they're working correctly

set -e

echo "🧪 PMDaemon CLI Status Display Test"
echo "=================================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
TEST_APP_NAME="test-status-app"
TEST_PORT=8899

# Function to print test status
print_test() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

# Function to wait for process state
wait_for_state() {
    local expected_state="$1"
    local timeout=10
    local count=0
    
    while [ $count -lt $timeout ]; do
        if cargo run --bin pmdaemon -- list | grep -q "$expected_state"; then
            return 0
        fi
        sleep 1
        count=$((count + 1))
    done
    return 1
}

# Cleanup function
cleanup() {
    echo
    print_test "Cleaning up test processes..."
    cargo run --bin pmdaemon -- delete "$TEST_APP_NAME" 2>/dev/null || true
    cargo run --bin pmdaemon -- stop all 2>/dev/null || true
}

# Set trap for cleanup
trap cleanup EXIT

echo "Building PMDaemon..."
cargo build --bin pmdaemon

echo
print_test "1. Testing basic process creation and status display"

# Create a simple test process
cat > test_config.json << EOF
{
  "name": "$TEST_APP_NAME",
  "script": "python3",
  "args": ["-c", "import time; [print(f'Test app running... {i}') or time.sleep(2) for i in range(1000)]"],
  "cwd": ".",
  "env": {
    "TEST_ENV": "test_value"
  },
  "port": $TEST_PORT,
  "autorestart": true,
  "max_restarts": 3
}
EOF

# Start the test process
print_test "Starting test process..."
cargo run --bin pmdaemon -- start test_config.json

# Wait a moment for the process to start
sleep 2

echo
print_test "2. Checking all status fields in CLI output"

# Get the status output
STATUS_OUTPUT=$(cargo run --bin pmdaemon -- list)
echo "Current status output:"
echo "$STATUS_OUTPUT"
echo

# Test each field
print_test "Verifying status fields:"

# Check if ID field is present (should be 8 characters)
if echo "$STATUS_OUTPUT" | grep -E "[a-f0-9]{8}" > /dev/null; then
    print_success "✓ ID field: Present (8-char UUID prefix)"
else
    print_error "✗ ID field: Missing or invalid format"
fi

# Check if Name field is present
if echo "$STATUS_OUTPUT" | grep "$TEST_APP_NAME" > /dev/null; then
    print_success "✓ Name field: Present ($TEST_APP_NAME)"
else
    print_error "✗ Name field: Missing"
fi

# Check if Status field is present (should be 'online' for running process)
if echo "$STATUS_OUTPUT" | grep -E "(online|starting)" > /dev/null; then
    print_success "✓ Status field: Present (online/starting)"
else
    print_error "✗ Status field: Missing or unexpected value"
fi

# Check if PID field is present (should be a number)
if echo "$STATUS_OUTPUT" | grep -E "[0-9]{3,}" > /dev/null; then
    print_success "✓ PID field: Present (numeric)"
else
    print_error "✗ PID field: Missing or invalid"
fi

# Check if Uptime field is present (should show time format like 1s, 1m, etc.)
if echo "$STATUS_OUTPUT" | grep -E "[0-9]+[smhd]" > /dev/null; then
    print_success "✓ Uptime field: Present (time format)"
else
    print_warning "⚠ Uptime field: May be missing or showing '-'"
    echo "  Current uptime values in output:"
    echo "$STATUS_OUTPUT" | awk '{print "  " $5}' | tail -n +2
fi

# Check if Restarts field is present (should be 0 for new process)
if echo "$STATUS_OUTPUT" | grep -E "\s+0\s+" > /dev/null; then
    print_success "✓ Restarts field: Present (showing 0 for new process)"
else
    print_warning "⚠ Restarts field: May be missing or unexpected value"
fi

# Check if CPU field is present (should show percentage)
if echo "$STATUS_OUTPUT" | grep -E "[0-9]+\.[0-9]" > /dev/null; then
    print_success "✓ CPU field: Present (decimal format)"
else
    print_warning "⚠ CPU field: May be missing or showing 0.0"
fi

# Check if Memory field is present (should show with unit like MB, KB)
if echo "$STATUS_OUTPUT" | grep -E "[0-9]+(\.[0-9]+)?(MB|KB|GB|B)" > /dev/null; then
    print_success "✓ Memory field: Present (with units)"
else
    print_warning "⚠ Memory field: May be missing or showing '-'"
fi

# Check if Port field is present
if echo "$STATUS_OUTPUT" | grep "$TEST_PORT" > /dev/null; then
    print_success "✓ Port field: Present ($TEST_PORT)"
else
    print_warning "⚠ Port field: May be missing or showing '-'"
fi

echo
print_test "3. Testing uptime progression over time"

echo "Waiting 5 seconds to test uptime progression..."
sleep 5

STATUS_OUTPUT_2=$(cargo run --bin pmdaemon -- list)
echo "Status after 5 seconds:"
echo "$STATUS_OUTPUT_2"

# Extract uptime values
UPTIME_1=$(echo "$STATUS_OUTPUT" | grep "$TEST_APP_NAME" | awk '{print $5}' || echo "N/A")
UPTIME_2=$(echo "$STATUS_OUTPUT_2" | grep "$TEST_APP_NAME" | awk '{print $5}' || echo "N/A")

echo "Uptime comparison:"
echo "  Initial: $UPTIME_1"
echo "  After 5s: $UPTIME_2"

if [[ "$UPTIME_1" != "$UPTIME_2" ]] && [[ "$UPTIME_2" != "-" ]] && [[ "$UPTIME_2" != "N/A" ]]; then
    print_success "✓ Uptime progression: Working (values changed)"
else
    print_error "✗ Uptime progression: Not working or stuck"
fi

echo
print_test "4. Testing process restart and restart counter"

print_test "Restarting process to test restart counter..."
cargo run --bin pmdaemon -- restart "$TEST_APP_NAME"

# Wait for restart to complete
sleep 3

STATUS_OUTPUT_3=$(cargo run --bin pmdaemon -- list)
echo "Status after restart:"
echo "$STATUS_OUTPUT_3"

# Check if restart counter increased
if echo "$STATUS_OUTPUT_3" | grep "$TEST_APP_NAME" | grep -E "\s+[1-9]\s+" > /dev/null; then
    print_success "✓ Restart counter: Working (incremented)"
else
    print_warning "⚠ Restart counter: May not have incremented"
fi

echo
print_test "5. Testing monitoring mode (real-time updates)"

print_test "Testing monitoring mode for 10 seconds..."
echo "You should see real-time updates with changing uptime values:"

# Run monitoring mode in background for 10 seconds
timeout 10s cargo run --bin pmdaemon -- monitor 2>/dev/null || true

print_success "✓ Monitoring mode: Completed"

echo
print_test "6. Testing process stop and status changes"

print_test "Stopping process to test status changes..."
cargo run --bin pmdaemon -- stop "$TEST_APP_NAME"

sleep 2

STATUS_OUTPUT_4=$(cargo run --bin pmdaemon -- list)
echo "Status after stop:"
echo "$STATUS_OUTPUT_4"

# Check if status changed to stopped
if echo "$STATUS_OUTPUT_4" | grep "$TEST_APP_NAME" | grep -E "(stopped|offline)" > /dev/null; then
    print_success "✓ Status change: Working (stopped)"
else
    print_error "✗ Status change: Process may still show as running"
fi

# Check if PID is cleared
if echo "$STATUS_OUTPUT_4" | grep "$TEST_APP_NAME" | grep -E "\s+-\s+" > /dev/null; then
    print_success "✓ PID clearing: Working (shows '-' when stopped)"
else
    print_warning "⚠ PID clearing: May still show PID when stopped"
fi

echo
print_test "7. Summary of CLI Status Display Test"
echo "======================================"

echo "All major status fields have been tested:"
echo "  • ID: Process UUID (8-char prefix)"
echo "  • Name: Process name"
echo "  • Status: Process state (online/stopped/etc.)"
echo "  • PID: System process ID"
echo "  • Uptime: Time since process started"
echo "  • Restarts: Number of restarts"
echo "  • CPU %: CPU usage percentage"
echo "  • Memory: Memory usage with units"
echo "  • Port: Assigned port number"

echo
print_success "CLI Status Display Test Complete!"
echo "Check the output above for any warnings or failures."

# Cleanup
rm -f test_config.json

echo
echo "To manually verify uptime is working:"
echo "1. Start a process: cargo run --bin pmdaemon -- start <config>"
echo "2. Run: cargo run --bin pmdaemon -- list"
echo "3. Wait a few seconds and run list again"
echo "4. Verify the uptime value increases"
