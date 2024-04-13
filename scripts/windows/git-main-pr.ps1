# This script automates the pull request from develop to main
# it recommends installing gh (GitHub CLI) on your development PC
# Check if GitHub CLI is installed
if (Get-Command gh -ErrorAction SilentlyContinue) {
    # Check the current Git branch
    $currentBranch = git rev-parse --abbrev-ref HEAD
    if ($currentBranch -eq "develop") {
        # Get PR details from the user
        $title = Read-Host "Enter the title for your pull request"
        $description = Read-Host "Enter the description of your feature"

        # Create the pull request using GitHub CLI
        gh pr create --base main --head develop --title $title --body $description
        Write-Output "Pull request created successfully."
        Exit 0
    } else {
        Write-Output "You are not on the 'develop' branch. Please switch to 'develop' to create a pull request."
        Exit 1
    }
} else {
    Write-Output "GitHub CLI is not installed. Please install GitHub CLI using Chocolatey."
    Write-Output "If you do not have Chocolatey installed, visit: https://chocolatey.org/install"
    Write-Output "To install GitHub CLI using Chocolatey, run: choco install gh"
    Exit 2
}
