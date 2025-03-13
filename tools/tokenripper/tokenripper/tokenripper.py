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
    for tag in soup(
        ["script", "style", "meta", "link", "head", "noscript", "footer", "nav"]
    ):
        tag.decompose()

    # Special handling for man pages and documentation
    if url and ("man" in url or "doc" in url):
        # For man pages, main content is often in specific containers
        main_content = soup.find(
            "div", class_=["content", "main", "man-content", "manual"]
        )
        if main_content:
            # Extract just that content
            soup = main_content

    # Extract raw text and normalize whitespace
    text = soup.get_text()

    # Clean up the text
    text = re.sub(r"\n+", "\n", text)  # Replace multiple newlines with single
    text = re.sub(r"\s+\n", "\n", text)  # Remove whitespace before newlines
    text = re.sub(r"\n\s+", "\n", text)  # Remove whitespace after newlines
    text = re.sub(r"[ \t]+", " ", text)  # Normalize spaces and tabs

    return text.strip()


def fetch_and_clean(input_path):
    """Fetch HTML from a file or URL, clean it, and save as a text file."""
    is_url = input_path.startswith("http://") or input_path.startswith("https://")

    if is_url:
        print(f"🔄 Fetching {input_path}...")
        try:
            # More complete browser-like headers to avoid being blocked
            headers = {
                "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
                "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
                "Accept-Language": "en-US,en;q=0.5",
                "Accept-Encoding": "gzip, deflate, br",
                "Connection": "keep-alive",
                "Upgrade-Insecure-Requests": "1",
                "Sec-Fetch-Dest": "document",
                "Sec-Fetch-Mode": "navigate",
                "Sec-Fetch-Site": "none",
                "Sec-Fetch-User": "?1",
                "Cache-Control": "max-age=0",
                "Referer": "https://www.google.com/",
            }

            # Create a session to handle cookies and maintain a persistent connection
            session = requests.Session()
            response = session.get(input_path, headers=headers, timeout=15)

            # Print the response status for debugging
            print(f"Response status code: {response.status_code}")

            response.raise_for_status()  # Raise exception for 4XX/5XX responses
            html_content = response.text

            # Create a filename from the URL
            filename = input_path.split("/")[-1]
            if not filename or "." not in filename:
                filename = (
                    input_path.split("/")[-2]
                    if len(input_path.split("/")) > 2
                    else "webpage"
                )
            filename = re.sub(r"[^\w\-\.]", "_", filename)
            if not filename.endswith(".txt"):
                filename = filename.split(".")[0] + ".txt"

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

    print(
        f"✅ Cleaned text saved as {filename} ({file_size:.1f} KB, ~{word_count} words)"
    )


def display_help():
    """Display help information."""
    print(
        """
╭─────────────────────────────────────────────╮
│                TOKENRIPPER                  │
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
