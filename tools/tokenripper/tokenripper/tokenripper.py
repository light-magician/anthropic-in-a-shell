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
