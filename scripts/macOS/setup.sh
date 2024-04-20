#!/bin/bash

# Setup script to handle errors and exit
set -e

# Trap specifically for Ctrl+C interruption
trap 'echo "Caught SIGINT, running cleanup..."; cleanup_on_exit; exit 130' INT

# Determine OS type and adjust the sed in-place edit command accordingly
if [[ "$(uname)" == "Darwin" ]]; then
    SED_CMD="sed -i ''"
else
    SED_CMD="sed -i''"
fi

# Define the SQL file path relative to the script location
SCRIPT_DIR=$(dirname "$0")
BASE_DIR=$(realpath "$SCRIPT_DIR/../..")
SQL_FILE="$BASE_DIR/data/data.sql"

# Cleanup function to handle unexpected exits
cleanup_on_exit() {
    echo "Cleaning up..."
    [ -f "$SQL_FILE.bak" ] && mv "$SQL_FILE.bak" "$SQL_FILE" || echo "No backup to restore."
}

# Backup existing SQL file
[ -f "$SQL_FILE" ] && cp "$SQL_FILE" "$SQL_FILE.bak"

# Initialize new SQL file
echo "-- SQL Database Schema" > "$SQL_FILE"

