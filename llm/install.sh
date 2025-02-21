#!/bin/bash

# install.sh

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Version
VERSION="0.1.0"

# Logging functions
log_info() {
    echo -e "${GREEN}INFO: $1${NC}"
}

log_warn() {
    echo -e "${YELLOW}WARN: $1${NC}"
}

log_error() {
    echo -e "${RED}ERROR: $1${NC}"
}

# Check if running on macOS
check_platform() {
    if [[ "$(uname)" != "Darwin" ]]; then
        log_error "This script is currently only supported on macOS"
        exit 1
    }
}

# Check if Rust is installed
check_rust() {
    if ! command -v cargo &> /dev/null; then
        log_error "Rust is not installed. Please install it first:"
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    }
}

# Build the release version
build_release() {
    log_info "Building release version..."
    cargo build --release
    if [ $? -ne 0 ]; then
        log_error "Build failed"
        exit 1
    }
}

# Install to /usr/local/bin
install_binary() {
    local binary_path="/usr/local/bin/claude_cli"
    
    # Create /usr/local/bin if it doesn't exist
    if [ ! -d "/usr/local/bin" ]; then
        log_info "Creating /usr/local/bin directory..."
        sudo mkdir -p /usr/local/bin
    fi

    log_info "Installing claude_cli to $binary_path..."
    
    # Remove existing binary if it exists
    if [ -f "$binary_path" ]; then
        log_warn "Removing existing installation..."
        sudo rm "$binary_path"
    fi

    # Copy and set permissions
    sudo cp target/release/claude_cli "$binary_path"
    sudo chmod +x "$binary_path"
    
    if [ $? -eq 0 ]; then
        log_info "Installation successful!"
    else
        log_error "Installation failed"
        exit 1
    fi
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    if command -v claude_cli &> /dev/null; then
        local version_output=$(claude_cli --version 2>&1)
        log_info "claude_cli is installed: $version_output"
        log_info "Installation location: $(which claude_cli)"
    else
        log_error "Verification failed - claude_cli not found in PATH"
        exit 1
    fi
}

# Create completion scripts
create_completions() {
    local completion_dir="/usr/local/share/zsh/site-functions"
    local bash_completion_dir="/usr/local/etc/bash_completion.d"
    
    log_info "Creating shell completions..."

    # Create directories if they don't exist
    sudo mkdir -p "$completion_dir"
    sudo mkdir -p "$bash_completion_dir"

    # Generate completions if your CLI supports it
    # Uncomment these lines once you've implemented shell completion generation
    # claude_cli completions zsh | sudo tee "$completion_dir/_claude_cli" > /dev/null
    # claude_cli completions bash | sudo tee "$bash_completion_dir/claude_cli" > /dev/null
}

# Main installation process
main() {
    log_info "Starting claude_cli installation (v$VERSION)..."
    
    check_platform
    check_rust
    build_release
    install_binary
    create_completions
    verify_installation
    
    log_info "Installation complete! You can now run 'claude_cli' from anywhere."
    log_info "To get started, run: claude_cli --help"
}

# Check if script is being run with sudo
if [ "$EUID" -eq 0 ]; then
    log_error "Please do not run this script with sudo directly."
    log_error "The script will ask for sudo permissions when needed."
    exit 1
fi

# Run main function
main