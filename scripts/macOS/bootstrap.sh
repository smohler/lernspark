#!/bin/bash

# Get the directory of the bootstrap script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Check if Homebrew is installed
if ! command -v brew &> /dev/null
then
    echo "Homebrew not found. Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
fi

# Install packages from the Brewfile
echo "Installing packages from Brewfile..."
brew bundle --file="$SCRIPT_DIR/Brewfile"

# Make all the scripts in the macOS directory executable
chmod +x "$SCRIPT_DIR"/scripts/macOS/*.sh

# Define aliases for each script
alias lernspark-data="$SCRIPT_DIR/scripts/macOS/lernspark-data.sh"
alias lernspark-pr="$SCRIPT_DIR/scripts/macOS/lernspark-pr.sh"

# Add the aliases to the user's shell configuration file
CONFIG_FILE="$HOME/.bash_profile"
if [ -f "$HOME/.bashrc" ]; then
    CONFIG_FILE="$HOME/.bashrc"
elif [ -f "$HOME/.zshrc" ]; then
    CONFIG_FILE="$HOME/.zshrc"
fi

echo "" >> "$CONFIG_FILE"
echo "# Lernspark Commands" >> "$CONFIG_FILE"
echo "# added by bootstrap script; see more at (https://github.com/smohler/lernspark)" >> "$CONFIG_FILE"
echo "alias lernspark-pr=\"$SCRIPT_DIR/scripts/macOS/lernspark-pr.sh\"" >> "$CONFIG_FILE"
echo "alias lernspark-data=\"$SCRIPT_DIR/scripts/macOS/lernspark-data.sh\"" >> "$CONFIG_FILE"

echo "Bootstrap complete. Please restart your terminal or run 'source $CONFIG_FILE' to apply the changes."
