#!/bin/bash

# Tokenripper Installation Script
# This script creates and installs the tokenripper command line tool

set -e  # Exit on error

echo "🚀 Installing Tokenripper..."

# Create a temporary directory
TEMP_DIR=$(mktemp -d)
echo "📁 Using temporary directory: $TEMP_DIR"

# Change to the temporary directory
cd "$TEMP_DIR"

# Create package structure
echo "📦 Creating package structure..."
mkdir -p tokenripper/tokenripper

# Create setup.py
echo "📝 Creating setup.py..."
cat > tokenripper/setup.py << 'EOF'
from setuptools import setup, find_packages

setup(
    name="tokenripper",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[
        "requests",
        "beautifulsoup4",
    ],
    entry_points={
        "console_scripts": [
            "tokenripper=tokenripper.tokenripper:main",
        ],
    },
    author="Your Name",
    author_email="your.email@example.com",
    description="A tool to extract clean text from HTML files or URLs",
    keywords="html, text, cleaning, scraping",
    python_requires=">=3.6",
)
EOF

# Create __init__.py
echo "📝 Creating __init__.py..."
cat > tokenripper/tokenripper/__init__.py << 'EOF'
__version__ = '0.1.0'
EOF

# Create tokenripper.py
echo "📝 Creating tokenripper.py..."
cat > tokenripper/tokenripper/tokenripper.py << 'EOF'
#!/usr/bin/env python3
import requests
from bs4 import BeautifulSoup
import re
import sys
import os

def clean_html(html_content):
    """Extract and clean meaningful text from HTML."""
    soup = BeautifulSoup(html_content, "html.parser")
    # Remove scripts, styles, and metadata
    for tag in soup(["script", "style", "meta", "link", "head", "noscript"]):
        tag.decompose()
    # Extract raw text and normalize whitespace
    text = re.sub(r"\s+", " ", soup.get_text()).strip()
    return text

def fetch_and_clean(input_path):
    """Fetch HTML from a file or URL, clean it, and save as a text file."""
    if input_path.startswith("http://") or input_path.startswith("https://"):
        print(f"🔄 Fetching {input_path}...")
        response = requests.get(input_path)
        if response.status_code != 200:
            print(f"❌ Failed to fetch page. Status code: {response.status_code}")
            return
        html_content = response.text
        filename = os.path.basename(input_path).split(".")[0] + ".txt"
    else:
        print(f"📂 Reading local file {input_path}...")
        try:
            with open(input_path, "r", encoding="utf-8") as f:
                html_content = f.read()
        except FileNotFoundError:
            print(f"❌ File not found: {input_path}")
            return
        filename = os.path.splitext(os.path.basename(input_path))[0] + ".txt"
    
    cleaned_text = clean_html(html_content)
    
    with open(filename, "w", encoding="utf-8") as f:
        f.write(cleaned_text)
    
    print(f"✅ Cleaned text saved as {filename}")

def main():
    if len(sys.argv) != 2:
        print("Usage: tokenripper <URL|HTML_FILE>")
        sys.exit(1)
    
    fetch_and_clean(sys.argv[1])

if __name__ == "__main__":
    main()
EOF

# Install the package
echo "📦 Installing package..."
cd tokenripper
pip install .

# Check if tokenripper is available in PATH
if command -v tokenripper > /dev/null; then
    echo "✅ Tokenripper has been installed successfully and is available in your PATH"
else
    # If not available, we need to find where pip installed it and update PATH
    echo "⚠️ Tokenripper installed but not found in PATH. Updating your PATH..."
    
    # Find where pip installs binaries
    PIP_BIN_DIR=$(python -c "import site; print(site.USER_BASE + '/bin')" 2>/dev/null || echo "$HOME/.local/bin")
    
    # Make sure the directory exists
    mkdir -p "$PIP_BIN_DIR"
    
    # Check if the binary is there
    if [ -f "$PIP_BIN_DIR/tokenripper" ]; then
        echo "📍 Found tokenripper at $PIP_BIN_DIR/tokenripper"
    else
        # Try to find it elsewhere
        TOKENRIPPER_PATH=$(find ~/.local/bin /usr/local/bin /usr/bin -name tokenripper 2>/dev/null | head -n 1)
        
        if [ -n "$TOKENRIPPER_PATH" ]; then
            echo "📍 Found tokenripper at $TOKENRIPPER_PATH"
            PIP_BIN_DIR=$(dirname "$TOKENRIPPER_PATH")
        else
            echo "⚠️ Could not locate tokenripper binary. Using default path: $PIP_BIN_DIR"
        fi
    fi
    
    # Update .zshrc
    if grep -q "PATH=.*$PIP_BIN_DIR" ~/.zshrc; then
        echo "✅ PATH already contains $PIP_BIN_DIR in ~/.zshrc"
    else
        echo "🔄 Updating ~/.zshrc..."
        echo "" >> ~/.zshrc
        echo "# Added by tokenripper installer" >> ~/.zshrc
        echo "export PATH=\"\$PATH:$PIP_BIN_DIR\"" >> ~/.zshrc
        echo "✅ Updated ~/.zshrc"
        
        # Source the updated .zshrc
        echo "🔄 Updating current shell PATH..."
        export PATH="$PATH:$PIP_BIN_DIR"
    fi
fi

# Clean up
echo "🧹 Cleaning up temporary files..."
rm -rf "$TEMP_DIR"

echo "🎉 Installation complete! You can now use 'tokenripper' from anywhere."
echo ""
echo "To use it right now in this shell session, run:"
echo "  export PATH=\"\$PATH:$PIP_BIN_DIR\""
echo "Or restart your terminal/shell."
echo ""
echo "Usage: tokenripper <URL|HTML_FILE>"
