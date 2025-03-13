# File structure:
# tokenripper/
# ├── setup.py
# ├── tokenripper/
#     ├── __init__.py
#     └── tokenripper.py

# First, create the directory structure:
# mkdir -p tokenripper/tokenripper

# tokenripper/tokenripper/__init__.py
# This can be empty or have version info:
__version__ = "0.1.0"

# tokenripper/tokenripper/tokenripper.py
# Your main script with minor modifications:
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

# tokenripper/setup.py
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

# Install the package with:
# pip install -e .
# or
# pip install .
