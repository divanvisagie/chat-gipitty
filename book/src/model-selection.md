# Model Selection

Chat GipiTTY supports multiple AI models and providers, giving you flexibility to choose the right model for your specific task. This section covers how to select models, understand their capabilities, and optimize your usage.

## Default Model

By default, Chat GipiTTY uses `gpt-4o` when connected to OpenAI. This provides a good balance of capability and performance for most tasks.

## Selecting Models

### Command Line Selection

Use the `-M` or `--model` flag to specify a model for a single query:

```sh
cgip -M gpt-3.5-turbo "Simple question that doesn't need GPT-4"
cgip -M gpt-4o "Complex reasoning task"
cgip -M claude-3-sonnet "Detailed code analysis"
```

### Configuration

Set a default model in your configuration:

```sh
cgip config --set model=gpt-4o
```

### List Available Models

See what models are available with your current provider:

```sh
cgip --list-models
```

## Model Categories

### OpenAI Models

For up-to-date information about OpenAI models, capabilities, and pricing, see the [official OpenAI documentation](https://platform.openai.com/docs/models).

Common models include:
- **GPT-4 family** - Most capable models for complex reasoning
- **GPT-3.5 family** - Fast and cost-effective for simple tasks
- **Specialized models** - Vision, search, and other specialized capabilities

### Local Models (via Ollama)

When using Ollama (`OPENAI_BASE_URL=http://localhost:11434/v1`):

```sh
# Popular local models
cgip -M llama2:7b "Question for local Llama model"
cgip -M codellama:13b "Code-related question"
cgip -M mistral:7b "General purpose query"
cgip -M phi:2.7b "Lightweight model for simple tasks"
```

### Other Providers

#### Claude (Anthropic)
For current Claude model information, see [Anthropic's documentation](https://docs.anthropic.com/claude/docs/models-overview).

```sh
cgip -M claude-3-opus "Most capable Claude model"
cgip -M claude-3-sonnet "Balanced Claude model"
cgip -M claude-3-haiku "Fast Claude model"
```

#### Gemini (Google)
For current Gemini model information, see [Google's AI documentation](https://ai.google.dev/models/gemini).

```sh
cgip -M gemini-pro "Google's flagship model"
cgip -M gemini-pro-vision "Gemini with vision capabilities"
```

## Model Selection Strategy

### By Task Type

#### Code Analysis and Programming
```sh
# Best models for code
cgip -M gpt-4o "review this code" -f complex_algorithm.py
cgip -M claude-3-sonnet "explain this codebase" -f src/*.py
cgip -M codellama:13b "simple code question" -f script.sh  # Local option
```

#### Creative Writing and Content
```sh
# Creative tasks
cgip -M gpt-4o "write a creative story about..."
cgip -M claude-3-opus "generate marketing copy for..."
```

#### Data Analysis
```sh
# Analytical tasks
cgip -M gpt-4-turbo "analyze this dataset" -f data.csv
cgip -M claude-3-sonnet "find patterns in this data" -f logs.txt
```

#### Quick Questions and Simple Tasks
```sh
# Fast, simple queries
cgip -M gpt-3.5-turbo "what's the capital of France?"
cgip -M mistral:7b "simple calculation or fact check"  # Local option
```

#### Image Analysis
```sh
# Vision-capable models
cgip -M gpt-4o image --file photo.jpg "describe this image"
cgip -M gpt-4-vision-preview image --file diagram.png "explain this diagram"
```

#### Web Search
```sh
# Search automatically uses optimal model
cgip --search "latest developments in AI"  # Auto-selects gpt-4o-search-preview
```

### By Performance Requirements

#### Speed-Optimized
```sh
# Fastest responses
cgip -M gpt-3.5-turbo "quick question"
cgip -M phi:2.7b "simple local query"  # Local, very fast
```

#### Quality-Optimized
```sh
# Best quality responses
cgip -M gpt-4o "complex reasoning task"
cgip -M claude-3-opus "detailed analysis requiring nuance"
```

#### Cost-Optimized
```sh
# Lower cost options
cgip -M gpt-3.5-turbo "cost-sensitive query"
cgip -M llama2:7b "free local processing"  # No API costs
```

### By Context Length

#### Large Context Requirements
```sh
# For large files or long conversations
cgip -M gpt-4-turbo "analyze entire codebase" -f src/*.py  # 128k context
cgip -M claude-3-sonnet "process long document" -f book.txt  # 200k context
```

#### Standard Context
```sh
# Normal usage
cgip -M gpt-4o "regular queries"  # 128k context
cgip -M gpt-3.5-turbo "simple tasks"  # 16k context
```

## Advanced Model Usage

### Model-Specific Features

#### Automatic Model Selection
Some features automatically select appropriate models:

```sh
# Web search auto-selects search-optimized model
cgip --search "current events"

# Image analysis ensures vision-capable model
cgip image --file photo.jpg "describe this"

# TTS uses speech models automatically
cgip tts "text to convert"
```

#### Provider-Specific Capabilities
```sh
# OpenAI specific
cgip -M gpt-4o "/search with web browsing"

# Claude specific (via compatible endpoint)
cgip -M claude-3-opus "long-form analysis with nuanced reasoning"

# Local model benefits
cgip -M llama2:7b "private, offline processing"
```

### Model Switching in Sessions

```sh
# Start with fast model
cgip -M gpt-3.5-turbo "initial question"

# Switch to more capable model for follow-up
cgip -M gpt-4o "complex follow up based on previous answer"
```

### Environment-Based Selection

Set up different defaults for different environments:

```sh
# Development environment - use local models
export OPENAI_BASE_URL=http://localhost:11434/v1
cgip config --set model=codellama:13b

# Production analysis - use best models
export OPENAI_BASE_URL=https://api.openai.com
cgip config --set model=gpt-4o
```

## Best Practices

### Model Selection Guidelines

1. **Start simple**: Use `gpt-3.5-turbo` for straightforward queries
2. **Upgrade when needed**: Switch to `gpt-4o` for complex reasoning
3. **Use specialized models**: Choose vision models for images, code models for programming
4. **Consider context**: Use high-context models for large files
5. **Balance cost and quality**: Expensive models aren't always necessary

### Performance Tips

```sh
# Cache expensive model responses in sessions
export CGIP_SESSION_NAME="analysis-session"
cgip -M gpt-4o "expensive analysis" -f large_dataset.csv

# Use cheaper models for iteration
cgip -M gpt-3.5-turbo "quick clarification about previous response"
```

### Cost Management

```sh
# Preview context before sending to expensive models
cgip --show-context -M gpt-4o "preview what will be sent"

# Use local models for development/testing
cgip -M llama2:7b "test query structure before using paid API"
```

## Troubleshooting Model Selection

### Model Not Available
```sh
# Check available models
cgip --list-models

# Verify provider configuration
cgip config --get base_url
```

### Feature Not Supported
```sh
# Some features require specific model families
cgip -M gpt-4o image --file photo.jpg  # Vision requires vision-capable model
cgip -M gpt-4o-search-preview --search "query"  # Search requires search-enabled model
```

### Performance Issues
```sh
# Switch to faster model if response time is important
cgip -M gpt-3.5-turbo "time-sensitive query"

# Use local models to avoid network latency
cgip -M llama2:7b "local processing"
```

## Model Resources

For detailed model comparisons, capabilities, and current pricing:

- **OpenAI Models**: [OpenAI Platform Documentation](https://platform.openai.com/docs/models)
- **Anthropic Claude**: [Claude Model Overview](https://docs.anthropic.com/claude/docs/models-overview)
- **Google Gemini**: [Gemini Model Documentation](https://ai.google.dev/models/gemini)
- **Ollama Models**: [Ollama Model Library](https://ollama.com/library)

These official sources provide the most up-to-date information about model capabilities, context lengths, pricing, and availability.
