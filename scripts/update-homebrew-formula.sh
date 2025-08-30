#!/bin/bash
set -e

# Script to update the Homebrew formula with the latest release information
# Usage: ./scripts/update-homebrew-formula.sh <version>

VERSION=${1:-$(git describe --tags --abbrev=0 | sed 's/^v//')}
REPO_OWNER="markusz"
REPO_NAME="somfy-cli"

if [ -z "$VERSION" ]; then
    echo "Error: No version provided and no git tags found"
    echo "Usage: $0 <version>"
    exit 1
fi

echo "Updating Homebrew formula for version $VERSION"

# Function to get SHA256 of a file from GitHub release
get_sha256() {
    local asset_name=$1
    local url="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v${VERSION}/${asset_name}"
    echo "Downloading $asset_name to calculate SHA256..." >&2
    curl -sL "$url" | shasum -a 256 | cut -d' ' -f1
}

# Get SHA256 hashes for different architectures
echo "Calculating SHA256 hashes..."
MACOS_X86_64_SHA=$(get_sha256 "somfy-cli-v${VERSION}.tar.gz")
MACOS_ARM64_SHA=$(get_sha256 "somfy-cli-macos-aarch64")
LINUX_X86_64_SHA=$(get_sha256 "somfy-cli-linux-x86_64")

echo "macOS x86_64 SHA256: $MACOS_X86_64_SHA"
echo "macOS ARM64 SHA256: $MACOS_ARM64_SHA"
echo "Linux x86_64 SHA256: $LINUX_X86_64_SHA"

# Update the Homebrew formula
cat > homebrew/somfy-cli.rb << EOF
class SomfyCli < Formula
  desc "Command-line interface for controlling Somfy smart home devices"
  homepage "https://github.com/${REPO_OWNER}/${REPO_NAME}"
  version "${VERSION}"
  
  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v#{version}/somfy-cli-v#{version}.tar.gz"
      sha256 "${MACOS_X86_64_SHA}"
    elsif Hardware::CPU.arm?
      url "https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v#{version}/somfy-cli-macos-aarch64"
      sha256 "${MACOS_ARM64_SHA}"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v#{version}/somfy-cli-linux-x86_64"
      sha256 "${LINUX_X86_64_SHA}"
    end
  end

  license "MIT"

  def install
    if OS.mac?
      if Hardware::CPU.intel?
        # Extract from tarball
        system "tar", "-xzf", cached_download
        bin.install "somfy-cli-macos-x86_64" => "somfy"
      elsif Hardware::CPU.arm?
        bin.install cached_download => "somfy"
      end
    elsif OS.linux? && Hardware::CPU.intel?
      bin.install cached_download => "somfy"
    end
    
    chmod "+x", bin/"somfy"
  end

  test do
    assert_match "somfy-cli", shell_output("#{bin}/somfy --version")
    assert_match "Somfy CLI", shell_output("#{bin}/somfy --help")
  end
end
EOF

echo "Homebrew formula updated successfully!"
echo "Next steps:"
echo "1. Commit the updated formula: git add homebrew/somfy-cli.rb && git commit -m 'Update Homebrew formula for v${VERSION}'"
echo "2. Create a tap repository if you haven't already"
echo "3. Copy the formula to your tap repository"