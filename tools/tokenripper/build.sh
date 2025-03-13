#!/bin/bash

# Tokenripper Installation Script with Auto-Versioning
# This script creates and installs the tokenripper command line tool

set -e  # Exit on error

echo "🚀 Installing Tokenripper..."

# Define version file location
VERSION_FILE="$HOME/.tokenripper_version"

# Check if version file exists and read current version
if [ -f "$VERSION_FILE" ]; then
    CURRENT_VERSION=$(cat "$VERSION_FILE")
    # Extract version parts
    MAJOR=$(echo $CURRENT_VERSION | cut -d. -f1)
    MINOR=$(echo $CURRENT_VERSION | cut -d. -f2)
    PATCH=$(echo $CURRENT_VERSION | cut -d. -f3)
    
    # Increment patch version
    PATCH=$((PATCH + 1))
    NEW_VERSION="$MAJOR.$MINOR.$PATCH"
    echo "📊 Incrementing version from $CURRENT_VERSION to $NEW_VERSION"
else
    # Default starting version
    NEW_VERSION="0.1.0"
    echo "📊 Starting with initial version $NEW_VERSION"
fi

# Save the new version
echo $NEW_VERSION > "$VERSION_FILE"

# Create a temporary directory
TEMP_DIR=$(mktemp -d)
echo "📁 Using temporary directory: $TEMP_DIR"

# Change to the temporary directory
cd "$TEMP_DIR"

# Create package structure
echo "📦 Creating package structure..."
mkdir -p tokenripper/tokenripper

# Create setup.py with dynamic version
echo "📝 Creating setup.py..."
cat > tokenripper/setup.py << EOF
from setuptools import setup, find_packages

setup(
    name="tokenripper",
    version="$NEW_VERSION",
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

# Create __init__.py with dynamic version
echo "📝 Creating __init__.py..."
cat > tokenripper/tokenripper/__init__.py << EOF
__version__ = '$NEW_VERSION'
EOF

# Create tokenripper.py with the improved version
echo "📝 Creating tokenripper.py..."
cat > tokenripper/tokenripper/tokenripper.py << 'EOF'
#!/usr/bin/env python3
import requests
from bs4 import BeautifulSoup
import re
import sys
import os


def clean_html(html_content, url=None):
    """Extract and clean meaningful text from HTML."""
    soup = BeautifulSoup(html_content, "html.parser")
    
    # Remove scripts, styles, and metadata
    for tag in soup(["script", "style", "meta", "link", "head", "noscript", "footer", "nav"]):
        tag.decompose()
    
    # Special handling for man pages and documentation
    if url and ('man' in url or 'doc' in url):
        # For man pages, main content is often in specific containers
        main_content = soup.find('div', class_=['content', 'main', 'man-content', 'manual'])
        if main_content:
            # Extract just that content
            soup = main_content
    
    # Extract raw text and normalize whitespace
    text = soup.get_text()
    
    # Clean up the text
    text = re.sub(r'\n+', '\n', text)  # Replace multiple newlines with single
    text = re.sub(r'\s+\n', '\n', text)  # Remove whitespace before newlines
    text = re.sub(r'\n\s+', '\n', text)  # Remove whitespace after newlines
    text = re.sub(r'[ \t]+', ' ', text)  # Normalize spaces and tabs
    
    return text.strip()


def fetch_and_clean(input_path):
    """Fetch HTML from a file or URL, clean it, and save as a text file."""
    is_url = input_path.startswith("http://") or input_path.startswith("https://")
    
    if is_url:
        print(f"🔄 Fetching {input_path}...")
        try:
            # More complete browser-like headers to avoid being blocked
            headers = {
                'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36',
                'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8',
                'Accept-Language': 'en-US,en;q=0.5',
                'Accept-Encoding': 'gzip, deflate, br',
                'Connection': 'keep-alive',
                'Upgrade-Insecure-Requests': '1',
                'Sec-Fetch-Dest': 'document',
                'Sec-Fetch-Mode': 'navigate',
                'Sec-Fetch-Site': 'none',
                'Sec-Fetch-User': '?1',
                'Cache-Control': 'max-age=0',
                'Referer': 'https://www.google.com/'
            }
            
            # Create a session to handle cookies and maintain a persistent connection
            session = requests.Session()
            response = session.get(input_path, headers=headers, timeout=15)
            
            # Print the response status for debugging
            print(f"Response status code: {response.status_code}")
            
            response.raise_for_status()  # Raise exception for 4XX/5XX responses
            html_content = response.text
            
            # Create a filename from the URL
            filename = input_path.split('/')[-1]
            if not filename or '.' not in filename:
                filename = input_path.split('/')[-2] if len(input_path.split('/')) > 2 else "webpage"
            filename = re.sub(r'[^\w\-\.]', '_', filename)
            if not filename.endswith('.txt'):
                filename = filename.split('.')[0] + '.txt'
                
        except requests.RequestException as e:
            print(f"❌ Failed to fetch page: {e}")
            return
    else:
        print(f"📂 Reading local file {input_path}...")
        try:
            with open(input_path, "r", encoding="utf-8") as f:
                html_content = f.read()
        except FileNotFoundError:
            print(f"❌ File not found: {input_path}")
            return
        except UnicodeDecodeError:
            try:
                # Try with a different encoding if utf-8 fails
                with open(input_path, "r", encoding="latin-1") as f:
                    html_content = f.read()
                print("ℹ️ File opened with latin-1 encoding")
            except Exception as e:
                print(f"❌ Error reading file: {e}")
                return
        
        filename = os.path.splitext(os.path.basename(input_path))[0] + ".txt"

    # Clean the HTML content
    cleaned_text = clean_html(html_content, input_path if is_url else None)

    # Write to file
    with open(filename, "w", encoding="utf-8") as f:
        f.write(cleaned_text)

    # Get file size
    file_size = os.path.getsize(filename) / 1024  # Size in KB
    word_count = len(cleaned_text.split())
    
    print(f"✅ Cleaned text saved as {filename} ({file_size:.1f} KB, ~{word_count} words)")


def display_help():
    """Display help information."""
    print(
        """
╭─────────────────────────────────────────────╮
│                TOKENRIPPER                   │
╰─────────────────────────────────────────────╯

A command-line tool to extract clean text from HTML documents and webpages.

USAGE:
  tokenripper <URL|HTML_FILE>    Extract text from a URL or local HTML file
  tokenripper help               Display this help message

FUNCTIONALITY:
  - Extracts readable text from HTML documents and webpages
  - Special handling for man pages and documentation sites
  - Removes scripts, styles, navigation, and metadata
  - Normalizes whitespace and formatting
  - Saves clean text to a file in the current directory

EXAMPLES:
  tokenripper https://linux.die.net/man/1/imagemagick  # Process a man page
  tokenripper https://example.com                      # Process a website
  tokenripper ./local_document.html                    # Process a local file

VERSION: 0.2.0
    """
    )


def main():
    if len(sys.argv) != 2:
        print(
            "Usage: tokenripper <URL|HTML_FILE> or 'tokenripper help' for more information"
        )
        sys.exit(1)

    if sys.argv[1].lower() in ["help", "--help", "-h"]:
        display_help()
        return

    fetch_and_clean(sys.argv[1])


if __name__ == "__main__":
    main()
EOF

# Update the version in the help display to match our auto-versioning
# Use different sed syntax depending on OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS requires an extension with -i
    sed -i '' "s/VERSION: [0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*/VERSION: $NEW_VERSION/" tokenripper/tokenripper/tokenripper.py
else
    # Linux doesn't require an extension
    sed -i "s/VERSION: [0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*/VERSION: $NEW_VERSION/" tokenripper/tokenripper/tokenripper.py
fi

# Uninstall existing package to ensure clean installation
echo "🗑️ Removing previous installation..."
pip uninstall -y tokenripper 2>/dev/null || true

# Install the package
echo "📦 Installing package..."
cd tokenripper
pip install .

# Check if tokenripper is available in PATH
if command -v tokenripper > /dev/null; then
    echo "✅ Tokenripper v$NEW_VERSION has been installed successfully and is available in your PATH"
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

echo "🎉 Installation complete! Tokenripper v$NEW_VERSION is now ready."
echo ""
echo "To use it right now in this shell session, run:"
echo "  export PATH=\"\$PATH:$PIP_BIN_DIR\""
echo "Or restart your terminal/shell."
echo ""
echo "Usage: tokenripper <URL|HTML_FILE> or 'tokenripper help' for more information"
