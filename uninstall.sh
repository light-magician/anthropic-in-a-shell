
#!/bin/bash

set -e  # Exit on error

# Define the binary name and path
BINARY_NAME="agent"
BIN_PATH="/usr/local/bin/$BINARY_NAME"

# Check if the binary exists before attempting removal
if [ -f "$BIN_PATH" ]; then
    echo "🗑️ Removing $BINARY_NAME from $BIN_PATH..."
    sudo rm "$BIN_PATH"
    echo "✅ Uninstallation complete. '$BINARY_NAME' has been removed."
else
    echo "⚠️ $BINARY_NAME is not installed or already removed."
fi
