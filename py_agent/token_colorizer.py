import tiktoken

from rich.console import Console
from rich.text import Text

console = Console()
# this is the tokenization for GPT-4 (no claude models)
tokenizer = tiktoken.get_encoding("cl100k_base")


def tokenize_and_style(text):
    """
    take in a string of text
    encode it in the configured format
    replace spaces with dot
    colorize the tokens
    return the colorized_tokens
    """
    tokens = tokenizer.encode(text)
    token_strings = [tokenizer.decode([t]) for t in tokens]
    text_obj = Text()
    colors = ["cyan", "magenta", "yellow", "green", "red" "blue"]
    for i, token in enumerate(token_strings):
        # replace space with a middle dot
        token = token.replace(" ", "·")
        # simply cylce through the colors
        color = colors[i % len(colors)]
        text_obj.append(token, style=color)
    # return the text object, replaced text and colorized
    # ultimately we will use the console.print(text_obj)
    return text_obj
