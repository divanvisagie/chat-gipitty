# Command Line Options

Chat Gipitty provides a comprehensive set of command-line options to customize its behavior. This page documents all available options and their usage.

## Basic Syntax

```bash
cgip [OPTIONS] [QUERY] [COMMAND]
```

## Global Options

### Input Options

#### `-f, --file <FILE>`
Read query from a file. If a query is present, the file content is added to the prompt after the query. If this option is present, stdin is ignored.

```bash
# Read from file only
cgip -f input.txt

# Combine query with file content
cgip "analyze this code" -f script.py

# Multiple files can be specified
cgip -f file1.txt -f file2.txt "compare these files"
```

#### `-s, --system-prompt <SYSTEM_PROMPT>`
Specify a custom system prompt to influence the model's behavior.

```bash
# Custom system prompt
cgip -s "You are a helpful code reviewer" "check this function" -f code.py

# Behavioral modification
cgip -s "Respond only in bullet points" "summarize this article" -f article.txt
```

### Model Options

#### `-M, --model <MODEL>`
Specify which model to use. Defaults to `gpt-4`.

```bash
# Use GPT-3.5 Turbo
cgip -M gpt-3.5-turbo "simple question"

# Use GPT-4 Turbo
cgip -M gpt-4-turbo "complex analysis task"

# Use local model via Ollama
cgip -M llama2 "local processing"
```

#### `-l, --list-models`
List all available models for your configured API endpoint.

```bash
cgip -l
```

### Output Options

#### `-p, --show-progress`
Show progress indicator during processing. Note that this might interfere with stdout redirection.

```bash
cgip -p "long processing task"
```

#### `-c, --show-context`
Output the full context used in the query, including chat context with all assistant and user messages.

```bash
# See what context is being sent
ls | cgip -c "analyze these files"
```

#### `-m, --markdown`
Show context in a human-readable table format.

```bash
# Formatted context display
cgip -m "format this nicely" -f data.txt
```

### Session Options

#### `-n, --no-session`
Don't use messages from the session in this request. Useful for one-off queries or when you want to avoid session context.

```bash
# Isolated query without session history
cgip -n "independent question"

# Useful for sensitive queries
sensitive_command | cgip -n "analyze this privately"
```

### Special Options

#### `-j, --jarjar`
Speak like Jar Jar Binks. A fun easter egg option.

```bash
cgip -j "explain quantum computing"
```

### Help and Version

#### `-h, --help`
Print help information. Use with subcommands to get specific help.

```bash
# General help
cgip -h

# Subcommand help
cgip image -h
cgip tts -h
```

#### `-V, --version`
Print version information.

```bash
cgip -V
```

## Option Combinations

### Common Combinations

```bash
# Analyze code with custom model and system prompt
cgip -M gpt-4-turbo -s "You are a senior developer" "review this code" -f app.py

# Debug with context visibility
cargo build 2>&1 | cgip -c -m "explain this error"

# One-off analysis without session
cgip -n -f sensitive_data.txt "analyze this privately"

# Progress tracking for long tasks
cgip -p -M gpt-4-turbo "complex analysis task" -f large_dataset.csv
```

### File Input Combinations

```bash
# Multiple files with custom prompt
cgip -f config.yaml -f docker-compose.yml "check these configurations for compatibility"

# File input with model selection
cgip -M gpt-3.5-turbo -f simple_script.py "explain this code"

# File input with system prompt
cgip -s "You are a security expert" -f app.py "find security vulnerabilities"
```

## Order of Operations

Chat Gipitty processes input in this order:

1. **stdin** (if present and no `-f` flag)
2. **File content** (from `-f` flags, in order specified)
3. **Query argument** (the main text argument)
4. **System prompt** (applied as system message)

## Environment Integration

Options can be combined with environment variables:

```bash
# Use custom API endpoint with specific model
export OPENAI_BASE_URL="http://localhost:11434/v1"
cgip -M llama2 "local query"

# Session management with options
export CGIP_SESSION_NAME="project-review"
cgip -c "continue our discussion"
```

## Error Handling

### Invalid Options
```bash
# Invalid model
cgip -M invalid-model "test"
# Error: Model not available

# Invalid file
cgip -f nonexistent.txt "test"
# Error: File not found
```

### Option Conflicts
```bash
# Conflicting input sources
echo "stdin input" | cgip -f file.txt "query"
# stdin is ignored when -f is used
```

## Advanced Usage

### Debugging Options
```bash
# See exactly what's being sent to the API
cgip -c -m "debug query" -f input.txt

# Track progress on long operations
cgip -p -M gpt-4-turbo "complex task" -f large_file.txt
```

### Scripting with Options
```bash
#!/bin/bash
# Script example using multiple options

MODEL="gpt-4-turbo"
SYSTEM_PROMPT="You are a helpful assistant that responds concisely"

cgip -M "$MODEL" -s "$SYSTEM_PROMPT" -f "$1" "analyze this file"
```

### Pipeline Integration
```bash
# Complex pipeline with options
git log --oneline -10 | cgip -n -M gpt-3.5-turbo "summarize recent changes"

# Chained operations
cgip -f code.py "find issues" | cgip -n "prioritize these issues"
```
