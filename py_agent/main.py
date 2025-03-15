import os
import sys
import antrhopoc
import dotenv


def main():
    # Load environment variables from .env file
    dotenv.load_dotenv()

    # Initialize the Anthropic client
    client = Anthropic()

    # Store conversation history
    conversation = []

    print("Claude CLI (Press Ctrl+C to exit)")
    print("-----------------------------------")

    try:
        while True:
            # Get user input
            user_input = input("👤: ")

            # Add user message to conversation history
            conversation.append({"role": "user", "content": user_input})

            # Print bot indicator before streaming response
            sys.stdout.write("🤖: ")
            sys.stdout.flush()

            # Create streaming response
            stream = client.messages.create(
                model="claude-3-5-sonnet-latest",
                messages=conversation,
                max_tokens=1024,
                stream=True,
            )

            # Collect assistant's response for conversation history
            assistant_response = ""

            # Stream the response token by token
            for event in stream:
                if event.type == "content_block_delta":
                    chunk = event.delta.text
                    assistant_response += chunk
                    sys.stdout.write(chunk)
                    sys.stdout.flush()

            # Add a newline after the response
            print()

            # Add assistant message to conversation history
            conversation.append({"role": "assistant", "content": assistant_response})

    except KeyboardInterrupt:
        print("\nExiting Claude CLI. Goodbye!")
        sys.exit(0)


if __name__ == "__main__":
    main()
