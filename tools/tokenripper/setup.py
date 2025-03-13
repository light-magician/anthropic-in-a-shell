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
