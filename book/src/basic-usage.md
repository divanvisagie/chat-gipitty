# Basic Usage

This page covers the fundamental usage patterns of Chat GipiTTY. Once you understand these core concepts, you'll be able to effectively integrate AI assistance into your terminal workflow.

## Core Concepts

Chat GipiTTY follows a simple input priority system:
1. **stdin** - Input piped from other commands
2. **command arguments** - Text provided as arguments
3. **files** - Content read from files with `-f` flag

This priority system allows for flexible composition with other Unix tools.

## Basic Command Structure

The general syntax for Chat GipiTTY is:

```sh
cgip [OPTIONS] [QUERY] [COMMAND]
```

### Simplest Usage

Ask a direct question:

```sh
cgip "What is the capital of France?"
```

### Piping Input

Pipe output from other commands for AI analysis:

```sh
ls -la | cgip "What can you tell me about these files?"
```

```sh
ps aux | cgip "Are there any processes that look suspicious?"
```

### Using Files as Context

Include file content in your query:

```sh
cgip "explain this code" -f src/main.rs
```

### Combining Inputs

You can combine multiple input sources:

```sh
# Pipe + argument + file
cat error.log | cgip "analyze this error" -f config.yaml
```

## Common Usage Patterns

### Debugging and Development

Debug compilation errors:
```sh
cargo build 2>&1 | cgip "what's wrong with this code?"
```

Analyze git logs:
```sh
git log --oneline -10 | cgip "summarize these recent changes"
```

Review code:
```sh
cgip "review this function for potential issues" -f utils.py
```

### System Administration

Analyze system resources:
```sh
df -h | cgip "are there any disk space concerns?"
```

Review logs:
```sh
tail -n 50 /var/log/syslog | cgip "any critical issues in these logs?"
```

Network diagnostics:
```sh
netstat -tuln | cgip "explain what services are running"
```

### Text Processing

Convert formats:
```sh
cgip "convert this JSON to YAML" -f data.json
```

Summarize content:
```sh
cgip "provide a brief summary" -f long-document.md
```

Generate documentation:
```sh
cgip "create documentation for this function" -f my-function.py
```

### Data Analysis

Analyze CSV data:
```sh
head -20 data.csv | cgip "what patterns do you see in this data?"
```

Process command output:
```sh
find . -name "*.py" | wc -l | cgip "what does this number tell us about the project?"
```

## Working with Different Input Types

### Standard Input (stdin)

When you pipe data to cgip, it becomes the primary context:

```sh
echo "Hello World" | cgip "translate this to French"
```

### Command Arguments

Direct text queries:

```sh
cgip "write a Python function to calculate fibonacci numbers"
```

### File Input

Reference file contents:

```sh
cgip "optimize this SQL query" -f slow-query.sql
```

### Combining All Three

```sh
# Error from compilation + question + config file context
cargo test 2>&1 | cgip "why is this test failing?" -f Cargo.toml
```

## Output Modes

### Standard Output

By default, cgip outputs the AI response to stdout:

```sh
cgip "hello" > response.txt
```

### Show Context

See what context is being sent to the AI:

```sh
cgip "test" --show-context
```

### Markdown Formatting

Display context in a human-readable table:

```sh
cgip "test" --markdown --show-context
```

## Model Selection

### Using Different Models

Specify a model for your query:

```sh
cgip --model gpt-4o "complex reasoning task"
```

### List Available Models

See what models are available:

```sh
cgip --list-models
```

### Set Default Model

Configure your preferred model:

```sh
cgip config --set model=gpt-4o
```

## System Prompts

### Custom System Prompt

Provide context about how the AI should respond:

```sh
cgip --system-prompt "You are a expert Python developer" "review this code" -f script.py
```

### Set Default System Prompt

Configure a default system prompt:

```sh
cgip config --set system_prompt="Always provide concise, technical answers"
```

## Progress and Debugging

### Show Progress

Display a progress indicator (useful for long operations):

```sh
cgip --show-progress "analyze this large dataset" -f big-data.csv
```

### Context Inspection

When things aren't working as expected, examine the context being sent:

```sh
echo "test input" | cgip "query" --show-context --markdown
```

## Error Handling

### Stderr Redirection

Since cgip reads from stdin, redirect stderr to stdout for error analysis:

```sh
cargo build 2>&1 | cgip "what's the error?"
```

### Network Issues

If you encounter API errors, check your configuration:

```sh
cgip config --get model
echo $OPENAI_API_KEY | cut -c1-10  # Check if key is set (shows first 10 chars)
```

## Best Practices

### Be Specific

Instead of:
```sh
cgip "fix this"
```

Try:
```sh
cgip "identify and suggest fixes for any Python syntax errors" -f script.py
```

### Use Context Effectively

Provide relevant context for better results:

```sh
# Good: includes error and relevant file
python script.py 2>&1 | cgip "debug this error" -f script.py

# Less effective: only the error
python script.py 2>&1 | cgip "fix this"
```

### Combine with Unix Tools

Leverage the Unix philosophy:

```sh
# Filter and analyze
grep "ERROR" app.log | tail -20 | cgip "what's causing these errors?"

# Process and summarize
find . -name "*.js" -exec wc -l {} + | cgip "analyze the code distribution"
```

## Next Steps

Now that you understand basic usage, explore:

- [Core Features](./core-features.md) - Sessions, web search, and advanced features
- [Subcommands](./subcommands.md) - Specialized commands for images, TTS, and more
- [Examples](./examples.md) - Real-world usage scenarios and workflows