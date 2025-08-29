#!/bin/bash
set -e

# Script to test the Homebrew formula locally before publishing
# This builds the project locally and tests the formula installation

echo "Testing Homebrew formula locally..."

# Build the project
echo "Building project..."
cargo build --release

# Create a temporary directory structure that mimics a release
TEMP_DIR=$(mktemp -d)
echo "Using temporary directory: $TEMP_DIR"

# Copy the binary
cp target/release/somfy "$TEMP_DIR/somfy-cli-test"

# Create a simple HTTP server to serve the binary
echo "Starting local HTTP server..."
cd "$TEMP_DIR"
python3 -m http.server 8000 &
SERVER_PID=$!

# Function to cleanup on exit
cleanup() {
    echo "Cleaning up..."
    kill $SERVER_PID 2>/dev/null || true
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

# Wait a moment for the server to start
sleep 2

# Calculate SHA256 of the test binary
SHA256=$(shasum -a 256 somfy-cli-test | cut -d' ' -f1)
echo "Test binary SHA256: $SHA256"

# Create a test formula
cat > test-formula.rb << EOF
class SomfyCli < Formula
  desc "Command-line interface for controlling Somfy smart home devices"
  homepage "https://github.com/markusz/somfy-cli"
  version "test"
  url "http://localhost:8000/somfy-cli-test"
  sha256 "$SHA256"
  license "MIT"

  def install
    bin.install "somfy-cli-test" => "somfy"
    chmod "+x", bin/"somfy"
  end

  test do
    assert_match "somfy-cli", shell_output("#{bin}/somfy --version")
    assert_match "Somfy CLI", shell_output("#{bin}/somfy --help")
  end
end
EOF

echo "Created test formula with SHA256: $SHA256"
echo ""
echo "To test this formula manually:"
echo "1. brew install $TEMP_DIR/test-formula.rb"
echo "2. somfy --help"
echo "3. brew uninstall somfy-cli"
echo ""
echo "Press any key to continue with automatic test..."
read -n 1

# Test the formula
echo "Testing formula installation..."
if brew install "$TEMP_DIR/test-formula.rb"; then
    echo "✅ Formula installed successfully"
    
    # Test the installed binary
    if somfy --help >/dev/null 2>&1; then
        echo "✅ Binary works correctly"
    else
        echo "❌ Binary test failed"
    fi
    
    # Cleanup the test installation
    brew uninstall somfy-cli
    echo "✅ Test installation cleaned up"
else
    echo "❌ Formula installation failed"
    exit 1
fi

echo "🎉 Local Homebrew formula test completed successfully!"