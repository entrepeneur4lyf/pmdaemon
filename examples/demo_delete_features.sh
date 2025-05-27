#!/bin/bash

# Demo script for the new delete features in PMDaemon
# This script demonstrates:
# 1. Delete all processes
# 2. Delete processes by status
echo "Building PMDaemon..."
 cargo build --release || {
     echo "❌ Build failed. Please fix any compilation errors and try again."
     exit 1
 }
echo

# Set up demo environment
export PM2R_HOME="/tmp/pmdaemon-demo"
mkdir -p "$PM2R_HOME"
PMDAEMON="./target/release/pmdaemon"

echo "=== Demo 1: Delete All Processes ==="
echo

# Start some demo processes
echo "Starting demo processes..."
$PMDAEMON start "sleep 30" --name "demo-app-1"
$PMDAEMON start "sleep 30" --name "demo-app-2"
$PMDAEMON start "sleep 30" --name "demo-app-3"
echo

# List processes
echo "Current processes:"
$PMDAEMON list
echo

# Delete all processes with force flag
echo "Deleting all processes with --force flag..."
$PMDAEMON delete all --force
echo

# Verify all deleted
echo "Processes after delete all:"
$PMDAEMON list
echo

echo "=== Demo 2: Delete by Status ==="
echo

# Start some processes that will have different states
echo "Starting processes with different behaviors..."
$PMDAEMON start "sleep 60" --name "long-running"
$PMDAEMON start "echo 'quick exit'; exit 0" --name "quick-exit"
$PMDAEMON start "echo 'another quick'; exit 0" --name "another-quick"
echo

# Wait a moment for quick processes to exit
echo "Waiting for quick processes to exit..."
sleep 2
echo

# List processes to see their states
echo "Current processes and their states:"
$PMDAEMON list
echo

# Delete stopped processes
echo "Deleting processes with 'stopped' status..."
$PMDAEMON delete stopped --status --force
echo

# List remaining processes
echo "Remaining processes after deleting stopped ones:"
$PMDAEMON list
echo

# Clean up remaining processes
echo "Cleaning up remaining processes..."
$PMDAEMON delete all --force
echo

echo "=== Demo Complete ==="
echo "New delete features:"
echo "1. 'pmdaemon delete all [--force]' - Stop and delete all processes"
echo "2. 'pmdaemon delete <status> --status [--force]' - Stop and delete by status"
echo "   Valid statuses: starting, online, stopping, stopped, errored, restarting"
echo "3. '--force' flag skips confirmation prompts"
echo "4. Running processes are automatically stopped before deletion"
echo
echo "Examples:"
echo "  pmdaemon delete all --force"
echo "  pmdaemon delete stopped --status --force"
echo "  pmdaemon delete errored --status"
echo "  pmdaemon delete online --status  # Stops all running processes"
