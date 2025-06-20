# Core Features

Chat Gipitty is a command-line tool that leverages OpenAI's models to address and respond to user queries. It provides several core features that make it powerful and flexible for various use cases.

## Context Compilation

Chat Gipitty compiles context for queries by prioritizing input in a specific order:

1. **stdin** - Input piped from other commands
2. **Command-line arguments** - Direct text provided as arguments
3. **Files** - Content from files specified with the `-f` flag

This ordering allows for flexible composition of prompts from multiple sources.

## Model Support

While optimized for OpenAI's ChatGPT (with GPT-4 as the default model), Chat Gipitty works with multiple providers through OpenAI-compatible endpoints:

- **OpenAI** (ChatGPT, GPT-4, GPT-3.5, etc.)
- **Local models** via [Ollama](https://ollama.com)
- **Google Gemini** (via OpenAI-compatible endpoints)
- **Mistral AI** (via OpenAI-compatible endpoints)
- **Anthropic Claude** (via OpenAI-compatible endpoints)
- **Any other provider** implementing the OpenAI Chat Completions API standard

## Session Management

Chat Gipitty supports continuous chat sessions that persist across multiple interactions. Sessions are managed through the `CGIP_SESSION_NAME` environment variable, giving users control over session uniqueness and persistence.

## Web Search Integration

The `/search` command prefix enables web search functionality:
- For GPT models: Automatically switches to `gpt-4o-search-preview` for optimal search results
- For non-GPT models: Adds web search capabilities while maintaining your configured model

## Multimodal Capabilities

### Image Analysis
- Analyze images using vision-capable models (GPT-4o)
- Extract text from images
- Describe visual content
- Supports JPEG, PNG, GIF, and WebP formats

### Text-to-Speech
- Convert text to high-quality audio using OpenAI's TTS models
- Multiple voice options (alloy, echo, fable, onyx, nova, shimmer)
- Various audio formats (MP3, OPUS, AAC, FLAC)
- Customizable speed and instructions

### Embedding Generation
- Generate embedding vectors for text using OpenAI's embedding models
- Support for different embedding models
- Output to file or stdout

## Agentic Capabilities

The agent subcommand provides an agentic workflow that lets the model:
- Execute shell commands in a specified directory
- Receive command output as feedback
- Iterate through multiple commands to complete complex tasks
- Maintain safety by restricting operations to a chosen directory

## Flexible Input/Output

- **Piping support**: Seamlessly pipe output from other commands
- **File input**: Read content from files with the `-f` flag
- **Combined input**: Mix stdin, arguments, and file content
- **Progress indicators**: Optional progress display
- **Context viewing**: Inspect the full context being sent to the model
- **Markdown formatting**: Human-readable output formatting
