#!/bin/bash
PWD = $(pwd)
echo "Present working directory is $PWD"

SCRIPT_PATH=$(readlink -f "$0")
echo "Build script location: $SCRIPT_PATH"

# Set the root lernspark folder
ROOT_DIR="$(dirname $(dirname $(dirname "$SCRIPT_PATH")))"
echo "The root folder for lernspark is $ROOT_DIR"

# Set the target directory for the binary
TARGET_DIR="$ROOT_DIR/bin"
echo "Building binaries and placing them in $TARGET_DIR"

# CD into the data folder
cd "$ROOT_DIR/data"

# Build for macOS
echo "Building for macOS..."
cargo build --release 
cp "$ROOT_DIR/data/target/release/data" "$TARGET_DIR/macOS/"


# Build for Windows
echo "Building for Windows..."
touch "$TARGET_DIR/windows/data.exe"
cargo build --release --target x86_64-pc-windows-gnu
cp "$ROOT_DIR/data/target/x86_64-pc-windows-gnu/release/data.exe" "$TARGET_DIR/windows/"

# Clean build files
if [[ $1 == "-c" || $1 == "--clean" ]]; then
  cargo clean
fi

cd $PWD
echo "Build completed successfully!"
