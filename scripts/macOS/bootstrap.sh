#!/bin/bash

# Get the directory of the bootstrap script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Get the project root directory
project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." &>/dev/null && pwd)"

# Check if Homebrew is installed
if ! command -v brew &> /dev/null
then
    echo "Homebrew not found. Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
fi

# Install packages from the Brewfile
echo "Installing packages from Brewfile..."
brew bundle --file="$SCRIPT_DIR/Brewfile"

# Define aliases for each script
alias lernspark-data="$SCRIPT_DIR/scripts/macOS/lernspark-data.sh"
alias lernspark-pr="$SCRIPT_DIR/scripts/macOS/lernspark-pr.sh"

# Define symlinks in folders
# Get the directory of the bootstrap.sh script
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

# Function to create symlinks
create_symlink() {
  local target=$1
  local symlink=$2

  if [ -e "$symlink" ]; then
    echo "Symlink $symlink already exists. Skipping..."
  else
    ln -s "$target" "$symlink"
    echo "Created symlink: $symlink -> $target"
  fi
}

# Create symlink from data/data.sql to pipeline/data.sql
create_symlink "$project_root/data/data.sql" "$project_root/pipeline/data.sql"

# Create symlink from pipeline/pipeline.sql to data/pipeline.sql
create_symlink "$project_root/pipeline/pipeline.sql" "$project_root/data/pipeline.sql"
echo "Symlink creation completed."

# Add the aliases to the user's shell configuration file
CONFIG_FILE="$HOME/.bash_profile"
if [ -f "$HOME/.bashrc" ]; then
    CONFIG_FILE="$HOME/.bashrc"
elif [ -f "$HOME/.zshrc" ]; then
    CONFIG_FILE="$HOME/.zshrc"
fi

# Make all shell scripts in the scripts directory executable
find "$project_root/scripts" -type f -name "*.sh" -exec chmod +x {} \;
echo "Shell scripts made executable."

echo "" >> "$CONFIG_FILE"
echo "# Lernspark Commands" >> "$CONFIG_FILE"
echo "# added by bootstrap script; see more at (https://github.com/smohler/lernspark)" >> "$CONFIG_FILE"
echo "alias lernspark-pr=\"$SCRIPT_DIR/scripts/macOS/lernspark-pr.sh\"" >> "$CONFIG_FILE"
echo "alias lernspark-data=\"$SCRIPT_DIR/scripts/macOS/lernspark-data.sh\"" >> "$CONFIG_FILE"

echo "Bootstrap complete. Please restart your terminal or run 'source $CONFIG_FILE' to apply the changes."
