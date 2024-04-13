# This script automates the pull request from develop to main
# it recommends installing gh (GitHub CLI) on your development PC
#!/bin/bash

# Function to check if a command exists
command_exists () {
    type "$1" &> /dev/null ;
}

# Check if GitHub CLI is installed
if command_exists gh ; then
    # Check the current Git branch
    currentBranch=$(git rev-parse --abbrev-ref HEAD)
    if [ "$currentBranch" = "develop" ]; then
        # Get PR details from the user
        echo -n "Enter the title for your pull request: "
        read title
        echo -n "Enter the description of your feature: "
        read description

        # Create the pull request using GitHub CLI
        gh pr create --base main --head develop --title "$title" --body "$description"
        echo "Pull request created successfully."
    else
        echo "You are not on the 'develop' branch. Please switch to 'develop' to create a pull request."
    fi
else
    echo "GitHub CLI is not installed. Please install GitHub CLI."
    echo "For macOS with Homebrew:"
    echo "  brew install gh"
    echo "For Linux (Debian/Ubuntu):"
    echo "  sudo apt install gh"
    echo "For other distributions, please find the package in your package manager or visit:"
    echo "  https://github.com/cli/cli#installation"
fi
