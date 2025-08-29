# Homebrew Setup Guide

This guide explains how to set up Homebrew distribution for the somfy-cli project.

## Overview

To distribute via Homebrew, you need:
1. A GitHub repository with releases containing pre-built binaries
2. A Homebrew "tap" (custom formula repository)
3. A formula file that tells Homebrew how to install your software

## Step-by-Step Setup

### 1. Create a Release

First, create a tagged release to trigger the GitHub Actions workflow:

```bash
# Update version in Cargo.toml if needed
git add -A
git commit -m "Prepare for v0.1.0 release"
git tag v0.1.0
git push origin main --tags
```

This will trigger the release workflow which creates binaries for multiple platforms.

### 2. Create a Homebrew Tap Repository

Create a new GitHub repository named `homebrew-somfy-cli` (the `homebrew-` prefix is required):

```bash
# Create the repository on GitHub, then clone it
git clone https://github.com/markusz/homebrew-somfy-cli.git
cd homebrew-somfy-cli

# Create the formula directory structure
mkdir -p Formula
```

### 3. Update and Copy the Formula

After your first release is created:

```bash
# From your somfy-cli repository
cd /path/to/somfy-cli
./scripts/update-homebrew-formula.sh 0.1.0

# Copy the updated formula to your tap repository
cp homebrew/somfy-cli.rb /path/to/homebrew-somfy-cli/Formula/

# Commit to the tap repository
cd /path/to/homebrew-somfy-cli
git add Formula/somfy-cli.rb
git commit -m "Add somfy-cli formula v0.1.0"
git push origin main
```

### 4. Test the Installation

Once your tap is set up, test it:

```bash
# Add your tap
brew tap markusz/somfy-cli

# Install from your tap
brew install markusz/somfy-cli/somfy-cli

# Test that it works
somfy --help
```

## Updating for New Releases

For each new release:

1. Update version in `Cargo.toml`
2. Create a new git tag and push it
3. Wait for GitHub Actions to create the release
4. Run the update script:
   ```bash
   ./scripts/update-homebrew-formula.sh <new_version>
   ```
5. Copy the updated formula to your tap repository and commit

## Formula Details

The Homebrew formula (`homebrew/somfy-cli.rb`) includes:

- **Multi-platform support**: Different URLs and SHA256s for Intel/ARM macOS and Linux
- **Proper installation**: Installs the binary as `somfy` in the user's PATH
- **Tests**: Basic smoke tests to verify installation
- **Metadata**: Description, homepage, license information

## Troubleshooting

### SHA256 Mismatches
If you get SHA256 errors, the update script recalculates them from the actual releases. Make sure:
- The release exists on GitHub
- The asset names match exactly
- You have internet connectivity

### Binary Not Executable
The formula includes `chmod "+x"` to ensure the binary is executable after installation.

### Platform Detection Issues
The formula uses Homebrew's built-in hardware detection:
- `Hardware::CPU.intel?` for Intel processors
- `Hardware::CPU.arm?` for Apple Silicon
- `OS.mac?` vs `OS.linux?` for operating system detection