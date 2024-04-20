#!/bin/bash

# Setup script to handle errors and exit
set -e

# Trap specifically for Ctrl+C interruption
trap 'echo "Caught SIGINT, running cleanup..."; cleanup_on_exit; exit 130; cd $PWD' INT

SQL_FILE="$ROOT_DIR/data/data.sql"

# Cleanup function to handle unexpected exits
cleanup_on_exit() {
    echo "Cleaning up..."
    [ -f "$SQL_FILE.bak" ] && mv "$SQL_FILE.bak" "$SQL_FILE" || echo "No backup to restore."
}

# Backup existing SQL file
[ -f "$SQL_FILE" ] && cp "$SQL_FILE" "$SQL_FILE.bak"

# Initialize new SQL file
echo "-- SQL Database Schema" > "$SQL_FILE"

