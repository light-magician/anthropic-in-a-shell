class ContextManager:
    """Manages conversation history for LLM interactions."""

    def __init__(self):
        """Initialize an empty conversation context."""
        self.conversation = []  # List of message dicts with role and content
        self.active_indices = (
            None  # When None, use all context. Otherwise, use specified indices.
        )

    def add_message(self, role, content):
        """Add a message to the conversation history."""
        self.conversation.append({"role": role, "content": content})
        # After adding a message, reset active indices to use all context
        self.active_indices = None

    def get_active_context(self):
        """Get the currently active conversation context based on selected indices."""
        if self.active_indices is None:
            return self.conversation

        # Return only the selected messages
        return [
            self.conversation[i]
            for i in self.active_indices
            if i < len(self.conversation)
        ]

    def select_context(self, indices):
        """Select specific context items to include in future prompts."""
        # Validate indices are within range
        valid_indices = [i for i in indices if 0 <= i < len(self.conversation)]

        if not valid_indices:
            raise ValueError("No valid indices provided")

        self.active_indices = valid_indices

    def reset_selection(self):
        """Reset context selection to include all messages."""
        self.active_indices = None

    def clear_context(self):
        """Clear all conversation history."""
        self.conversation = []
        self.active_indices = None

    def get_context_summary(self):
        """Get a summary of the current conversation context."""
        if not self.conversation:
            return "No conversation history."

        summary = []
        for i, msg in enumerate(self.conversation):
            # Truncate long messages for the summary
            content = msg["content"]
            if len(content) > 50:
                content = content[:47] + "..."

            # Mark active messages with an asterisk
            active = (
                "*"
                if (self.active_indices is None or i in self.active_indices)
                else " "
            )

            summary.append(f"{active} [{i}] {msg['role']}: {content}")

        return "\n".join(summary)

    def get_message_at_index(self, index):
        """Get the message at a specific index."""
        if 0 <= index < len(self.conversation):
            return self.conversation[index]
        return None
