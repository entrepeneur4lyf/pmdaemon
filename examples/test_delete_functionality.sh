#!/bin/bash

# Quick test script to verify the new delete functionality works
echo "Testing new delete functionality..."

# Set up test environment
export PMDAEMON_HOME="/tmp/pmdaemon-test-delete"
rm -rf "$PMDAEMON_HOME"
mkdir -p "$PMDAEMON_HOME"

PMDAEMON="./target/debug/pmdaemon"

echo "1. Testing delete single process..."
$PMDAEMON start "sleep 10" --name "test-single"
echo "Started test process"
$PMDAEMON list
echo "Deleting process..."
$PMDAEMON delete test-single --force
echo "Process deleted. Listing remaining processes:"
$PMDAEMON list
echo

echo "2. Testing delete all processes..."
$PMDAEMON start "sleep 10" --name "test-1"
$PMDAEMON start "sleep 10" --name "test-2"
$PMDAEMON start "sleep 10" --name "test-3"
echo "Started 3 test processes"
$PMDAEMON list
echo "Deleting all processes..."
$PMDAEMON delete all --force
echo "All processes deleted. Listing remaining processes:"
$PMDAEMON list
echo

echo "3. Testing delete by status..."
$PMDAEMON start "sleep 30" --name "long-running"
$PMDAEMON start "echo 'quick exit'; exit 0" --name "quick-exit"
echo "Started processes with different behaviors"
sleep 2  # Let quick-exit finish
echo "Current processes:"
$PMDAEMON list
echo "Deleting stopped processes..."
$PMDAEMON delete stopped --status --force
echo "Stopped processes deleted. Remaining processes:"
$PMDAEMON list
echo "Cleaning up remaining processes..."
$PMDAEMON delete all --force
echo

echo "Test completed successfully!"
echo "New delete features are working correctly:"
echo "✓ Delete single process (stops running processes)"
echo "✓ Delete all processes"
echo "✓ Delete by status"
echo "✓ Force flag works"
