import os
import tiktoken
from anthropic import Anthropic


class LLMInterface:

    def __init__(self):
        # default to the GPT-4 tokenization
        self.tokenizer = tiktoken.get_encoding("cl100k_base")

    def send_message(self, messages, stream=True, max_tokens=1024):
        raise NotImplementedError("subclass must implement send_message")

    def count_tokens(self, text):
        if isinstance(text, list):
            text = " ".join(msg.get("content", "") for msg in text)
        return len(self.tokenizer.encode(text))

    def get_available_models(self):
        """Get a list of available models for this LLM."""
        raise NotImplementedError("Subclasses must implement get_available_models")

    def set_model(self, model_name):
        """Set the active model."""
        raise NotImplementedError("Subclasses must implement set_model")

    def get_current_model(self):
        """Get the current model name."""
        raise NotImplementedError("Subclasses must implement get_current_model")

    def get_token_cost(self, token_count, is_input=True):
        """Get the cost for the specified number of tokens."""
        raise NotImplementedError("Subclasses must implement get_token_cost")


class ClaudeLLM(LLMInterface):
    """Claude API implementation."""

    def __init__(self, model=None):
        """Initialize the Claude API client."""
        super().__init__()

        api_key = os.environ.get("ANTHROPIC_API_KEY")
        if not api_key:
            raise ValueError("ANTHROPIC_API_KEY environment variable is required")

        self.client = Anthropic(api_key=api_key)

        # Define available models with their costs per 1M tokens
        self.models = {
            "claude-3-opus-latest": {
                "input_cost": 15.00,  # $15.00 per 1M tokens
                "output_cost": 75.00,  # $75.00 per 1M tokens
                "description": "Most powerful model for complex tasks",
            },
            "claude-3-5-sonnet-latest": {
                "input_cost": 3.00,  # $3.00 per 1M tokens
                "output_cost": 15.00,  # $15.00 per 1M tokens
                "description": "Balanced performance and cost",
            },
            "claude-3-haiku-latest": {
                "input_cost": 0.25,  # $0.25 per 1M tokens
                "output_cost": 1.25,  # $1.25 per 1M tokens
                "description": "Fastest and most cost-effective",
            },
        }

        # Set default model to haiku for cost efficiency
        self.model = model or "claude-3-haiku-latest"
        if self.model not in self.models:
            raise ValueError(f"Unknown model: {self.model}")

    def send_message(self, messages, stream=True, max_tokens=1024):
        """Send a message to Claude and get a response."""
        try:
            response = self.client.messages.create(
                model=self.model,
                messages=messages,
                max_tokens=max_tokens,
                stream=stream,
            )

            if stream:
                # Return a generator that yields chunks
                return self._handle_stream(response)
            else:
                # Return the complete response text
                return response.content[0].text
        except Exception as e:
            error_msg = f"Error: {str(e)}"
            if stream:

                def error_generator():
                    yield error_msg

                return error_generator()
            else:
                return error_msg

    def _handle_stream(self, stream):
        """Process a streaming response from Claude."""
        for event in stream:
            if event.type == "content_block_delta":
                yield event.delta.text

    def get_available_models(self):
        """Get a list of available models for Claude."""
        return self.models

    def set_model(self, model_name):
        """Set the active Claude model."""
        if model_name not in self.models:
            raise ValueError(f"Unknown model: {model_name}")

        self.model = model_name
        return True

    def get_current_model(self):
        """Get the current Claude model name."""
        return self.model

    def get_model_info(self, model_name=None):
        """Get information about a specific model."""
        model = model_name or self.model
        if model not in self.models:
            raise ValueError(f"Unknown model: {model}")

        return self.models[model]

    def get_token_cost(self, token_count, is_input=True):
        """
        Calculate the cost for the specified number of tokens.

        Args:
            token_count (int): Number of tokens
            is_input (bool): Whether these are input or output tokens

        Returns:
            float: Cost in USD
        """
        if self.model not in self.models:
            raise ValueError(f"Unknown model: {self.model}")

        cost_per_million = (
            self.models[self.model]["input_cost"]
            if is_input
            else self.models[self.model]["output_cost"]
        )
        return (token_count / 1_000_000) * cost_per_million


# Factory function to get the appropriate LLM interface
def get_llm_interface(name="claude", model=None):
    """Get an LLM interface by name."""
    if name.lower() == "claude":
        return ClaudeLLM(model=model)
    else:
        raise ValueError(f"Unknown LLM interface: {name}")
