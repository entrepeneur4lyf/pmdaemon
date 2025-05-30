#!/bin/bash

# PMDaemon Restart Count Test Script
# This script specifically tests the restart counter functionality

set -e

echo "🔄 PMDaemon Restart Count Test"
echo "=============================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
TEST_APP_NAME="restart-count-test"

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

# Function to get restart count from output
get_restart_count() {
    local output="$1"
    echo "$output" | grep "$TEST_APP_NAME" | awk '{print $6}' | head -1
}

# Function to get status from output
get_status() {
    local output="$1"
    echo "$output" | grep "$TEST_APP_NAME" | awk '{print $3}' | head -1
}

# Cleanup function
cleanup() {
    echo
    print_test "Cleaning up test processes..."
    cargo run --bin pmdaemon -- delete "$TEST_APP_NAME" 2>/dev/null || true
    cargo run --bin pmdaemon -- stop all 2>/dev/null || true
    rm -f restart_test_config.json
}

# Set trap for cleanup
trap cleanup EXIT

echo "Building PMDaemon..."
cargo build --bin pmdaemon

echo
print_test "1. Creating test configuration"

# Create a test process that runs for a reasonable time
cat > restart_test_config.json << EOF
{
  "name": "$TEST_APP_NAME",
  "script": "/bin/bash",
  "args": ["-c", "echo 'Process starting...'; for i in {1..60}; do echo 'Running iteration \$i'; sleep 1; done; echo 'Process finished'"],
  "autorestart": false,
  "max_restarts": 5
}
EOF

print_success "✓ Test configuration created"

echo
print_test "2. Starting initial process"

# Start the test process
cargo run --bin pmdaemon -- start restart_test_config.json

# Wait a moment for the process to start
sleep 2

# Get initial status
INITIAL_STATUS=$(cargo run --bin pmdaemon -- list)
echo "Initial status:"
echo "$INITIAL_STATUS"
echo

INITIAL_RESTART_COUNT=$(get_restart_count "$INITIAL_STATUS")
INITIAL_PROCESS_STATUS=$(get_status "$INITIAL_STATUS")

print_test "Verifying initial state:"
echo "  Restart count: $INITIAL_RESTART_COUNT"
echo "  Process status: $INITIAL_PROCESS_STATUS"

if [[ "$INITIAL_RESTART_COUNT" == "0" ]]; then
    print_success "✓ Initial restart count is 0"
else
    print_error "✗ Initial restart count should be 0, got: $INITIAL_RESTART_COUNT"
fi

if [[ "$INITIAL_PROCESS_STATUS" == "online" ]]; then
    print_success "✓ Process is online"
else
    print_warning "⚠ Process status: $INITIAL_PROCESS_STATUS (expected: online)"
fi

echo
print_test "3. Testing first restart"

# Restart the process
print_test "Restarting process..."
cargo run --bin pmdaemon -- restart "$TEST_APP_NAME"

# Wait for restart to complete
sleep 3

# Get status after first restart
RESTART1_STATUS=$(cargo run --bin pmdaemon -- list)
echo "Status after first restart:"
echo "$RESTART1_STATUS"
echo

RESTART1_COUNT=$(get_restart_count "$RESTART1_STATUS")
RESTART1_PROCESS_STATUS=$(get_status "$RESTART1_STATUS")

print_test "Verifying after first restart:"
echo "  Restart count: $RESTART1_COUNT"
echo "  Process status: $RESTART1_PROCESS_STATUS"

if [[ "$RESTART1_COUNT" == "1" ]]; then
    print_success "✓ Restart count incremented to 1"
else
    print_error "✗ Restart count should be 1, got: $RESTART1_COUNT"
fi

echo
print_test "4. Testing second restart"

# Restart again
print_test "Restarting process again..."
cargo run --bin pmdaemon -- restart "$TEST_APP_NAME"

# Wait for restart to complete
sleep 3

# Get status after second restart
RESTART2_STATUS=$(cargo run --bin pmdaemon -- list)
echo "Status after second restart:"
echo "$RESTART2_STATUS"
echo

RESTART2_COUNT=$(get_restart_count "$RESTART2_STATUS")
RESTART2_PROCESS_STATUS=$(get_status "$RESTART2_STATUS")

print_test "Verifying after second restart:"
echo "  Restart count: $RESTART2_COUNT"
echo "  Process status: $RESTART2_PROCESS_STATUS"

if [[ "$RESTART2_COUNT" == "2" ]]; then
    print_success "✓ Restart count incremented to 2"
else
    print_error "✗ Restart count should be 2, got: $RESTART2_COUNT"
fi

echo
print_test "5. Testing third restart"

# Restart one more time
print_test "Restarting process third time..."
cargo run --bin pmdaemon -- restart "$TEST_APP_NAME"

# Wait for restart to complete
sleep 3

# Get status after third restart
RESTART3_STATUS=$(cargo run --bin pmdaemon -- list)
echo "Status after third restart:"
echo "$RESTART3_STATUS"
echo

RESTART3_COUNT=$(get_restart_count "$RESTART3_STATUS")
RESTART3_PROCESS_STATUS=$(get_status "$RESTART3_STATUS")

print_test "Verifying after third restart:"
echo "  Restart count: $RESTART3_COUNT"
echo "  Process status: $RESTART3_PROCESS_STATUS"

if [[ "$RESTART3_COUNT" == "3" ]]; then
    print_success "✓ Restart count incremented to 3"
else
    print_error "✗ Restart count should be 3, got: $RESTART3_COUNT"
fi

echo
print_test "6. Testing restart persistence"

# Stop and start the process (not restart) to see if count persists
print_test "Stopping and starting process (not restart)..."
cargo run --bin pmdaemon -- stop "$TEST_APP_NAME"
sleep 2
cargo run --bin pmdaemon -- start restart_test_config.json
sleep 3

# Get status after stop/start
STOPSTART_STATUS=$(cargo run --bin pmdaemon -- list)
echo "Status after stop/start:"
echo "$STOPSTART_STATUS"
echo

STOPSTART_COUNT=$(get_restart_count "$STOPSTART_STATUS")

print_test "Verifying restart count persistence:"
echo "  Restart count after stop/start: $STOPSTART_COUNT"

if [[ "$STOPSTART_COUNT" == "3" ]]; then
    print_success "✓ Restart count persisted through stop/start"
else
    print_warning "⚠ Restart count after stop/start: $STOPSTART_COUNT (may reset to 0, which is also valid behavior)"
fi

echo
print_test "7. Testing process deletion and recreation"

# Delete and recreate the process
print_test "Deleting and recreating process..."
cargo run --bin pmdaemon -- delete "$TEST_APP_NAME"
sleep 1
cargo run --bin pmdaemon -- start restart_test_config.json
sleep 3

# Get status after delete/recreate
RECREATE_STATUS=$(cargo run --bin pmdaemon -- list)
echo "Status after delete/recreate:"
echo "$RECREATE_STATUS"
echo

RECREATE_COUNT=$(get_restart_count "$RECREATE_STATUS")

print_test "Verifying restart count reset:"
echo "  Restart count after recreate: $RECREATE_COUNT"

if [[ "$RECREATE_COUNT" == "0" ]]; then
    print_success "✓ Restart count reset to 0 after recreation"
else
    print_error "✗ Restart count should be 0 after recreation, got: $RECREATE_COUNT"
fi

echo
print_test "8. Summary of Restart Count Test"
echo "================================="

echo "Test Results:"
echo "  • Initial restart count: $INITIAL_RESTART_COUNT (expected: 0)"
echo "  • After 1st restart: $RESTART1_COUNT (expected: 1)"
echo "  • After 2nd restart: $RESTART2_COUNT (expected: 2)"
echo "  • After 3rd restart: $RESTART3_COUNT (expected: 3)"
echo "  • After stop/start: $STOPSTART_COUNT (expected: 3 or 0)"
echo "  • After delete/recreate: $RECREATE_COUNT (expected: 0)"

echo
echo "Expected behavior:"
echo "  ✓ Restart count starts at 0"
echo "  ✓ Restart count increments with each restart command"
echo "  ✓ Restart count resets to 0 when process is deleted and recreated"
echo "  ? Restart count may or may not persist through stop/start (implementation dependent)"

echo
print_success "Restart Count Test Complete!"
echo "Check the output above for any failures or unexpected behavior."

echo
echo "Manual verification commands:"
echo "  cargo run --bin pmdaemon -- list                    # Check current status"
echo "  cargo run --bin pmdaemon -- restart $TEST_APP_NAME  # Restart and increment counter"
echo "  cargo run --bin pmdaemon -- info $TEST_APP_NAME     # Detailed process info"
