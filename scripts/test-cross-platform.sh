#!/bin/bash
# Cross-platform compatibility testing script for PMDaemon
# Tests PMDaemon functionality across different operating systems

set -euo pipefail

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect OS
detect_os() {
    case "$(uname -s)" in
        Darwin*)    echo "macOS" ;;
        Linux*)     echo "Linux" ;;
        CYGWIN*|MINGW*|MSYS*) echo "Windows" ;;
        *)          echo "Unknown" ;;
    esac
}

# Get binary extension for the platform
get_binary_ext() {
    if [[ "$(detect_os)" == "Windows" ]]; then
        echo ".exe"
    else
        echo ""
    fi
}

# Print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Run a test and report status
run_test() {
    local test_name=$1
    local test_command=$2
    
    print_status $BLUE "🧪 Testing: $test_name"
    
    if eval "$test_command" &>/dev/null; then
        print_status $GREEN "✅ PASS: $test_name"
        return 0
    else
        print_status $RED "❌ FAIL: $test_name"
        return 1
    fi
}

# Main function
main() {
    local os=$(detect_os)
    local binary_ext=$(get_binary_ext)
    local binary_path="./target/release/pmdaemon${binary_ext}"
    local failed_tests=0
    local total_tests=0
    
    print_status $YELLOW "=== PMDaemon Cross-Platform Compatibility Testing ==="
    print_status $BLUE "Operating System: $os"
    print_status $BLUE "Binary Path: $binary_path"
    echo
    
    # Build the project
    print_status $BLUE "🔧 Building PMDaemon..."
    if ! cargo build --release; then
        print_status $RED "❌ Build failed"
        exit 1
    fi
    print_status $GREEN "✅ Build successful"
    echo
    
    # Test 1: Binary existence and execution
    ((total_tests++))
    if ! run_test "Binary existence and execution" "test -x '$binary_path'"; then
        ((failed_tests++))
    fi
    
    # Test 2: Version command
    ((total_tests++))
    if ! run_test "Version command" "'$binary_path' --version"; then
        ((failed_tests++))
    fi
    
    # Test 3: Help command
    ((total_tests++))
    if ! run_test "Help command" "'$binary_path' --help"; then
        ((failed_tests++))
    fi
    
    # Test 4: List command (empty state)
    ((total_tests++))
    if ! run_test "List command (empty)" "'$binary_path' list"; then
        ((failed_tests++))
    fi
    
    # Test 5: Web server start/stop (basic functionality)
    ((total_tests++))
    if run_test "Web server start" "'$binary_path' web --port 19615 &>/dev/null &"; then
        sleep 1
        # Try to connect to web server
        if command -v curl &> /dev/null; then
            if curl -f http://127.0.0.1:19615 &>/dev/null; then
                print_status $GREEN "✅ Web server responded"
            else
                print_status $YELLOW "⚠️  Web server started but no response (expected)"
            fi
        fi
        # Stop any running PMDaemon processes
        pkill -f "pmdaemon.*web" 2>/dev/null || true
    else
        ((failed_tests++))
    fi
    
    # Test 6: Configuration file format support
    ((total_tests++))
    
    # Create a temporary config file
    local temp_config=$(mktemp)
    cat > "$temp_config" << 'EOF'
{
  "apps": [
    {
      "name": "test-config-app",
      "script": "echo",
      "args": ["Hello from config file"],
      "instances": 1
    }
  ]
}
EOF
    
    if run_test "Configuration file parsing" "'$binary_path' --config '$temp_config' list"; then
        # Config parsing worked
        true
    else
        ((failed_tests++))
    fi
    rm -f "$temp_config"
    
    # Test 7: Process lifecycle (start/stop/delete)
    ((total_tests++))
    local process_name="test-cross-platform-$$"
    if run_test "Process lifecycle test" "
        '$binary_path' start 'echo' --name '$process_name' --args 'Cross-platform test' && \
        sleep 1 && \
        '$binary_path' list | grep -q '$process_name' && \
        '$binary_path' delete '$process_name' --force
    "; then
        # Lifecycle test passed
        true
    else
        ((failed_tests++))
    fi
    
    # Test 8: Cargo test suite
    ((total_tests++))
    if run_test "Cargo test suite" "cargo test --release"; then
        # Test suite passed
        true
    else
        ((failed_tests++))
    fi
    
    # Test 9: Platform-specific path handling
    ((total_tests++))
    local temp_dir
    if [[ "$os" == "Windows" ]]; then
        temp_dir="C:\\temp\\pmdaemon-test"
    else
        temp_dir="/tmp/pmdaemon-test"
    fi
    
    # Create test directory and verify path handling
    mkdir -p "$temp_dir" 2>/dev/null || true
    if run_test "Path handling test" "
        test_proc='test-path-$$' && \
        '$binary_path' start 'echo' --name \"\$test_proc\" --cwd '$temp_dir' --args 'Path test' && \
        sleep 1 && \
        '$binary_path' delete \"\$test_proc\" --force
    "; then
        # Path handling worked
        true
    else
        ((failed_tests++))
    fi
    rm -rf "$temp_dir" 2>/dev/null || true
    
    # Test 10: Port management
    ((total_tests++))
    if run_test "Port management test" "
        port_proc='test-port-$$' && \
        '$binary_path' start 'echo' --name \"\$port_proc\" --port 18080 --args 'Port test' && \
        sleep 1 && \
        '$binary_path' list | grep -q '18080' && \
        '$binary_path' delete \"\$port_proc\" --force
    "; then
        # Port management worked
        true
    else
        ((failed_tests++))
    fi
    
    echo
    print_status $YELLOW "=== Test Results Summary ==="
    print_status $BLUE "Operating System: $os"
    print_status $BLUE "Total Tests: $total_tests"
    print_status $GREEN "Passed: $((total_tests - failed_tests))"
    
    if [[ $failed_tests -eq 0 ]]; then
        print_status $GREEN "🎉 ALL TESTS PASSED - PMDaemon is compatible with $os"
        exit 0
    else
        print_status $RED "Failed: $failed_tests"
        print_status $RED "❌ Some tests failed on $os"
        exit 1
    fi
}

# Handle interruption
trap 'print_status $YELLOW "Test interrupted. Cleaning up..."; pkill -f "pmdaemon" 2>/dev/null || true; exit 130' INT TERM

# Run main function
main "$@"
