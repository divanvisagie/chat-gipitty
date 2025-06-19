# Chat GipiTTY
[![crates.io](https://img.shields.io/crates/v/cgip.svg)](https://crates.io/crates/cgip)

Chat Gipitty (Chat Get Information, Print Information TTY) is a command line client primarily intended for the **official OpenAI Chat Completions API**. It allows you to chat with
language models in a terminal and even pipe output into it. While optimized for OpenAI's ChatGPT (with GPT-4 as the default model), it can also work with other
providers that expose OpenAI-compatible endpoints, including:

- **OpenAI** (ChatGPT, GPT-4, GPT-3.5, etc.)
- **Local models** via [Ollama](https://ollama.com)
- **Google Gemini** (via OpenAI-compatible endpoints)
- **Mistral AI** (via OpenAI-compatible endpoints)
- **Anthropic Claude** (via OpenAI-compatible endpoints)
- **Any other provider** implementing the OpenAI Chat Completions API standard

Custom `OPENAI_BASE_URL` values can point to these or other OpenAI-compatible endpoints, but such providers might not implement the complete API and compatibility cannot be guaranteed.

![logo](./docs/logo-256.png)

For example, say you wanted to debug your Rust program that doesn't compile and
want ChatGPT to explain the error in depth, you can pipe the output through Chat
Gipitty to help you debug like this, which would directly use the build error
output as the prompt:

```sh
cargo build 2>&1 | cgip
```

This would result in something like the following
```sh
❯ cargo build 2>&1 | cgip 'give me a short summary of the kind of error this is'
The error you're encountering is a **lifetime error** in Rust, specifically an issue with **borrowed values not living long enough**.
**borrowed values not living long enough**.
```

Another usage is reading from a file. In this example, we read from a file and
ask ChatGPT to convert that file to another programming language:
```sh cgip
cgip "convert this to python" -f src/main.rs
```

# Web Search Feature

Chat Gipitty supports web search functionality through the `/search` command prefix. When you start your message with `/search`, the application will enable web search capabilities to provide you with up-to-date information from the internet.

## How Web Search Works

- **For GPT models** (models starting with "gpt"): The application automatically switches to the `gpt-4o-search-preview` model and enables web search options for optimal search results.
- **For non-GPT models** (like Claude, Llama, or other custom models): The application keeps your configured model and adds web search options to the request.

## Usage Examples

```sh
# Search for current events
cgip "/search What are the latest developments in AI?"

# Search for technical information
echo "/search What is the current stable version of Rust?" | cgip

# Search combined with file input
cgip "/search How can I optimize this code for performance?" -f my_script.py
```

The `/search` prefix will be automatically removed from your message before it's sent to the model, so you don't need to worry about it affecting the actual prompt.

# Image Analysis Feature

Chat Gipitty supports image analysis using OpenAI's vision-capable models (like GPT-4o). You can analyze images, extract text, describe visual content, and more using the `image` subcommand.

## Basic Image Usage

```sh
# Analyze an image with a custom prompt
cgip image --file photo.jpg "What do you see in this image?"

# Extract text from an image
cgip image --file screenshot.png "Extract all text from this image"

# Analyze a document or receipt
cgip image --file receipt.jpg "What items are on this receipt and what's the total?"
```

## Supported Image Formats

- JPEG (.jpg, .jpeg)
- PNG (.png) 
- GIF (.gif)
- WebP (.webp)

The image subcommand automatically ensures you're using a vision-capable model and will switch to `gpt-4o` if your current model doesn't support vision.

For detailed documentation including examples, configuration options, and troubleshooting, see [docs/IMAGE_SUBCOMMAND.md](docs/IMAGE_SUBCOMMAND.md).

# Text-to-Speech Feature

Chat Gipitty supports text-to-speech conversion using OpenAI's TTS models. You can convert text to high-quality audio with various voice options and customization settings using the `tts` subcommand.

## Basic TTS Usage

```sh
# Convert text to speech with default settings
cgip tts "Hello, this is a test of text-to-speech functionality"

# Use a different voice and output file
cgip tts --voice nova --output welcome.mp3 "Welcome to our application!"

# Read text from stdin
echo "This text comes from stdin" | cgip tts --voice echo --output stdin_speech.mp3

# Combine stdin and argument text
hostname | cgip tts "and that's all she wrote" --output combined.mp3

# Use high-definition model with custom speed
cgip tts --model tts-1-hd --speed 0.8 --voice shimmer "This is spoken slowly and clearly"
```

## Available Options

- **Models**: `tts-1` (standard), `tts-1-hd` (high definition)
- **Voices**: `alloy`, `echo`, `fable`, `onyx`, `nova`, `shimmer`
- **Formats**: MP3, OPUS, AAC, FLAC
- **Speed**: 0.25 to 4.0 (1.0 is normal speed)
- **Instructions**: Custom tone and emotion directions

## Supported Audio Formats

- MP3 (.mp3) - Most compatible, good compression
- OPUS (.opus) - Excellent compression, modern codec  
- AAC (.aac) - Good quality and compression, widely supported
- FLAC (.flac) - Lossless compression, larger file size

The TTS subcommand reads text from command arguments, stdin, or both. When both are provided, it combines the stdin text with the argument text (stdin first, then argument), making it easy to pipe content from other commands and add additional text.

For detailed documentation including voice characteristics, examples, and advanced usage, see [docs/TTS_SUBCOMMAND.md](docs/TTS_SUBCOMMAND.md).

# Embedding Generation

You can generate embedding vectors for text using the `embedding` subcommand.
Text may be provided as an argument or via stdin. The resulting vector is printed
to stdout or saved to a file with `--output`.

```sh
# Basic usage
cgip embedding "Hello world"

# Read text from stdin and save to file
echo "some text" | cgip embedding --output vec.txt
```

See [docs/EMBEDDING_SUBCOMMAND.md](docs/EMBEDDING_SUBCOMMAND.md) for more details.

# Agentic Command Execution

The `agent` subcommand lets the model control a tiny shell agent. You provide a
directory and an instruction, and the model issues `execute` tool calls to run
commands in that directory. Command output is fed back into the model until it
returns a final answer. Once the session ends, the CLI prints a short summary of
all commands that were executed. Use `--max-actions` to limit how many commands
the agent may run (default is 10).

```sh
cgip agent . "list the directory contents"
```

See [docs/AGENT_SUBCOMMAND.md](docs/AGENT_SUBCOMMAND.md) for the full
documentation.

# Installation

chat-gipitty is designed to be run on POSIX compliant systems. **The recommended way to install Chat Gipitty is via Cargo.** This ensures you always get the latest version directly from crates.io, with minimal dependencies and maximum compatibility.

If you do not already have Cargo (the Rust package manager) installed, please visit [rustup.rs](https://rustup.rs/) for instructions on installing Rust and Cargo.

### Recommended: Install from crates.io with cargo
```sh
cargo install cgip
```

### Updating
To update to the latest release, run:
```sh
cgip update
```

Other installation methods are available for convenience, but may not always provide the latest version:

#### Manual Installation
Download this repository and then install the `cgip` command using:
```sh
sudo make install
```

## Set up API Key
Next, set up your OpenAI key by exporting it as `OPENAI_API_KEY`:
```sh
export OPENAI_API_KEY=your_key_here
```

## (Optional) Use a Custom API Endpoint

Chat Gipitty is designed to work with any OpenAI Chat Completions API-compatible provider. You can specify a custom API endpoint by setting the `OPENAI_BASE_URL` environment variable. If not set, the default is `https://api.openai.com`.

### Examples for Different Providers

```sh
# For local Ollama instance
export OPENAI_BASE_URL=http://localhost:11434/v1

# For Google Gemini (via OpenAI-compatible proxy)
export OPENAI_BASE_URL=https://generativelanguage.googleapis.com/v1beta

# For Mistral AI (via OpenAI-compatible endpoint)
export OPENAI_BASE_URL=https://api.mistral.ai/v1

# For other OpenAI-compatible services
export OPENAI_BASE_URL=https://your-provider.com/v1

# If your provider uses a different endpoint pattern, you can specify the full URL
export OPENAI_BASE_URL=https://custom-api.com/v2/chat/completions
```

**URL Construction**: Chat Gipitty intelligently constructs the API endpoint:
- If your base URL already contains `/chat/completions`, it uses it as-is
- If your base URL ends with `/v1` (or similar version pattern), it appends `/chat/completions`
- Otherwise, it appends `/v1/chat/completions` (standard OpenAI pattern)

As long as your provider implements the OpenAI Chat Completions API standard, Chat Gipitty will work with it seamlessly.

You can now pipe data to `cgip`. Remember to use `2>&1` to convert `stderr` to
`stdout` if you are debugging, as it can only read `stdin`.

# Session management
The `session` feature allows for continuous chat session management and shell
integration. To enable session caching in your terminal, you need to provide the
environment variable `CGIP_SESSION_NAME`. The uniqueness of your session is
dependant on the value of this variable.

This can be done by generating a session id for cgip to use in your `.bashrc`
or `.zshrc`:
```sh
export CGIP_SESSION_NAME=$(uuid)
```

The user has control over how unique their session is, for example to have a new
session for every new terminal opened you can use `uuid`:

```sh
export CGIP_SESSION_NAME=$(uuid)
```
However if you want the session to work across terminals you may do something
like us the date as a session id instead:
```sh
export CGIP_SESSION_NAME=$(date -I)
```

This implementation allows users to decide how they want to manage their
sessions and gives full control for them to have complex session management if
required without cluttering cgip with session management features.

### Session Subcommand Usage
The `session` subcommand provides options for managing the current session
context, you can use it to either view or clear the session, this can sometimes
be useful if the context has become too long, or if the LLM has become confused
by the context.


# Development Setup

## Ubuntu

On ubuntu, some additional packages are required to build the deb package.

```sh
sudo apt-get install build-essential dh-make debhelper devscripts
```

## Release Process
See [docs/RELEASE_PROCESS.md](docs/RELEASE_PROCESS.md) for details on preparing a new release.
