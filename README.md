# Chat GipiTTY
[![crates.io](https://img.shields.io/crates/v/cgip.svg)](https://crates.io/crates/cgip)

Chat Gipitty (Chat Get Information, Print Information TTY) is a command line client primarily intended for the **official OpenAI Chat Completions API**. It allows you to chat with language models in a terminal and even pipe output into it. While optimized for OpenAI's ChatGPT (with GPT-4 as the default model), it can also work with other providers that expose OpenAI-compatible endpoints.

![logo](./assets/logo-256.png)

## Quick Examples

### Using debugging output
Debug a Rust compilation error by piping the build output directly to ChatGPT:

```sh
cargo build 2>&1 | cgip "give me a short summary of the kind of error this is"
```

This results in something like:
```sh
❯ cargo build 2>&1 | cgip 'give me a short summary of the kind of error this is'
The error you're encountering is a **lifetime error** in Rust, specifically an issue with **borrowed values not living long enough**.
```

### Prototyping New Command Line Programs

You can create useful command line utilities by combining cgip with shell aliases:

#### Language Translation Utility
```sh
# Set up the alias
alias translate='cgip --system-prompt "You are a translator, you translate text to Spanish"'

# Use it
echo "Hello, world!" | translate
echo "Good morning" | translate
```

#### Code Review Assistant
```sh
# Set up the alias
alias review='cgip --system-prompt "You are a senior developer" "review this code for bugs and improvements"'

# Use it
git diff | review
cat src/main.py | review
```

## Key Features

- **Universal AI Access**: Works with OpenAI, local models via Ollama, Google Gemini, Mistral AI, Anthropic Claude, and any OpenAI-compatible provider
- **Intelligent Piping**: Pipe command output directly to AI models for instant analysis
- **Multi-Modal Support**: Text, image analysis, and text-to-speech capabilities
- **Session Management**: Maintain conversation context across terminal sessions
- **Web Search**: Get up-to-date information from the internet
- **Agentic Workflows**: Let the AI execute shell commands to accomplish tasks
- **Flexible Configuration**: Extensive customization options and provider support

## Quick Start

### Installation

**Recommended**: Install from crates.io with cargo:
```sh
cargo install cgip
```

### Setup

Set up your OpenAI API key:
```sh
export OPENAI_API_KEY=your_key_here
```

For custom providers, set the base URL:
```sh
# For local Ollama
export OPENAI_BASE_URL=http://localhost:11434/v1

# For other providers
export OPENAI_BASE_URL=https://your-provider.com/v1
```

### Basic Usage

```sh
# Ask a question
cgip "What is the capital of France?"

# Pipe command output for analysis
ls -la | cgip "What can you tell me about these files?"

# Include file content in your query
cgip "explain this code" -f src/main.rs
```

## Supported Providers

Chat GipiTTY works with any service that implements the OpenAI Chat Completions API standard:

- **OpenAI** (ChatGPT, GPT-4, GPT-3.5, etc.)
- **Local models** via [Ollama](https://ollama.com)
- **Google Gemini** (via OpenAI-compatible endpoints)
- **Mistral AI** (via OpenAI-compatible endpoints)
- **Anthropic Claude** (via OpenAI-compatible endpoints)
- **Any other provider** implementing the OpenAI Chat Completions API standard

## Documentation

📖 **Complete documentation is available [here](https://divanv.com/chat-gipitty/)**

The documentation includes:
- [Installation Guide](https://divanv.com/chat-gipitty/installation.html)
- [Getting Started](https://divanv.com/chat-gipitty/getting-started.html)
- [Core Features](https://divanv.com/chat-gipitty/core-features.html)
- [All Subcommands](https://divanv.com/chat-gipitty/subcommands.html) (image, tts, embedding, agent, etc.)
- [Configuration](https://divanv.com/chat-gipitty/configuration.html)
- [Advanced Usage](https://divanv.com/chat-gipitty/advanced-usage.html)
- [Examples](https://divanv.com/chat-gipitty/examples.html)

## Contributing

Contributions are welcome! Please see the [Contributing Guide](https://divanv.com/chat-gipitty/contributing.html) and [Development Workflow](https://divanv.com/chat-gipitty/development-workflow.html) in the documentation.

## License

This project is licensed under the terms specified in [LICENSE.md](LICENSE.md).
