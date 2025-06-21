# Introduction

Chat Gipitty (Chat Get Information, Print Information TTY) is a command line client primarily intended for the **official OpenAI Chat Completions API**. It allows you to chat with language models in a terminal and even pipe output into it. While optimized for OpenAI's ChatGPT (with GPT-4 as the default model), it can also work with other providers that expose OpenAI-compatible endpoints.

![Chat GipiTTY Logo](logo-256.png)

## What is Chat GipiTTY?

Chat GipiTTY is designed to make AI assistance seamlessly available in your terminal workflow. Whether you're debugging code, processing text, analyzing images, or generating speech, cgip brings powerful AI capabilities directly to your command line.

## Key Features

- **Universal AI Access**: Works with OpenAI, local models via Ollama, Google Gemini, Mistral AI, Anthropic Claude, and any OpenAI-compatible provider
- **Intelligent Piping**: Pipe command output directly to AI models for instant analysis
- **Multi-Modal Support**: Text, image analysis, and text-to-speech capabilities
- **Session Management**: Maintain conversation context across terminal sessions
- **Web Search**: Get up-to-date information from the internet
- **Agentic Workflows**: Let the AI execute shell commands to accomplish tasks
- **Flexible Configuration**: Extensive customization options and provider support

## Quick Examples

### Debugging Output
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

## Supported Providers

Chat GipiTTY works with any service that implements the OpenAI Chat Completions API standard:

- **OpenAI** (ChatGPT, GPT-4, GPT-3.5, etc.)
- **Local models** via [Ollama](https://ollama.com)
- **Google Gemini** (via OpenAI-compatible endpoints)
- **Mistral AI** (via OpenAI-compatible endpoints) 
- **Anthropic Claude** (via OpenAI-compatible endpoints)
- **Any other provider** implementing the OpenAI Chat Completions API standard

Custom `OPENAI_BASE_URL` values can point to these or other OpenAI-compatible endpoints, though compatibility cannot be guaranteed for all providers.

## Philosophy

Chat GipiTTY is built around the Unix philosophy of doing one thing well and composing with other tools. It seamlessly integrates AI capabilities into your existing terminal workflow without forcing you to change how you work.