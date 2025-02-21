#!/bin/bash

set -e  # Exit on error

# Detect the project directory (assumes this script is in the root)
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/llm-interaction" && pwd)"

# Define the binary name
BINARY_NAME="agent"

echo "📦 Building the Rust project..."
cargo build --release --manifest-path "$PROJECT_DIR/Cargo.toml"

# Move binary to /usr/local/bin
BIN_PATH="/usr/local/bin/$BINARY_NAME"
echo "🚀 Installing $BINARY_NAME to $BIN_PATH..."
sudo mv "$PROJECT_DIR/target/release/$BINARY_NAME" "$BIN_PATH"
sudo chmod +x "$BIN_PATH"

# Check installation
if command -v $BINARY_NAME &> /dev/null; then
    echo "✅ $BINARY_NAME installed successfully! You can now run it from anywhere using '$BINARY_NAME'."
else
    echo "❌ Installation failed. Please check your PATH or try running the script again."
fi
