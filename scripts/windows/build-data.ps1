$PWD = Get-Location
Write-Output "Present working directory is $PWD"

$SCRIPT_PATH = $MyInvocation.MyCommand.Path
Write-Output "Build script location: $SCRIPT_PATH"

# Set the root lernspark folder
$ROOT_DIR = Split-Path (Split-Path (Split-Path $SCRIPT_PATH -Parent) -Parent) -Parent
Write-Output "The root folder for lernspark is $ROOT_DIR"

# Set the target directory for the binary
$TARGET_DIR = Join-Path $ROOT_DIR "bin"
Write-Output "Building binaries and placing them in $TARGET_DIR"

# CD into the data folder
Set-Location (Join-Path $ROOT_DIR "data")

# Build for macOS
Write-Output "Building for macOS..."
cargo build --release --target aarch64-apple-darwin
Copy-Item (Join-Path $ROOT_DIR "data/target/aarch64-apple-darwin/release/data") (Join-Path $TARGET_DIR "macOS/")

# Build for Windows
Write-Output "Building for Windows..."
New-Item -ItemType File -Path (Join-Path $TARGET_DIR "windows/data.exe") -Force
cargo build --release --target 
Copy-Item (Join-Path $ROOT_DIR "data/target/release/data.exe") (Join-Path $TARGET_DIR "windows/")

# Clean build files
if ($args[0] -eq "-c" -or $args[0] -eq "--clean") {
    cargo clean
}

Set-Location $PWD
Write-Output "Build completed successfully!"
