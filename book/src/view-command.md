# View Command

The `view` command allows you to inspect the context that would be sent to the AI model without actually making an API call. This is invaluable for debugging, understanding how your inputs are processed, and optimizing your queries.

## Overview

The `view` command renders the complete context that Chat GipiTTY would send to the AI, including:
- Input from stdin
- Command line arguments
- File contents (when using `-f`)
- Session history (if enabled)
- System prompts

## Basic Usage

```sh
cgip view
```

## Command Help

```
Render the context without running a query against the model

Usage: cgip view

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

### Basic Context Inspection

View what gets sent when combining different inputs:

```sh
echo "Here is some data" | cgip view "What can you tell me about this?"
```

### File Content Preview

See how file content is included in context:

```sh
cgip view "Analyze this code" -f src/main.rs
```

### Complex Context

Inspect complex multi-source contexts:

```sh
grep "ERROR" app.log | cgip view "Analyze these errors" -f config.yaml
```

### Session Context

View current session context (when sessions are enabled):

```sh
export CGIP_SESSION_NAME=$(date -I)
cgip "Hello, I'm working on a project"
cgip view "What were we discussing?"
```

## Use Cases

### Debugging Input Processing

When your AI responses aren't what you expect, use `view` to understand what context is actually being sent:

```sh
# Check if your pipe is working correctly
ps aux | head -10 | cgip view "analyze these processes"
```

### Optimizing Context Size

Large contexts can be expensive and slow. Use `view` to see how much context you're sending:

```sh
# Check context size before sending
cat large-file.txt | cgip view "summarize this"
```

### Understanding File Processing

See exactly how files are being read and included:

```sh
cgip view "explain this configuration" -f docker-compose.yml
```

### Session Context Review

Before important queries, review what conversation history will be included:

```sh
cgip view "based on our previous discussion, what should I do next?"
```

## Output Formatting

The `view` command outputs the context in a structured format showing:

1. **System Messages**: Any system prompts or instructions
2. **User Messages**: Your input, including stdin, arguments, and files
3. **Assistant Messages**: Previous AI responses (if session is active)
4. **Context Metadata**: Information about the context structure

### Example Output

```
=== CONTEXT VIEW ===

System Message:
You are a helpful AI assistant.

User Message:
[stdin]: Here is some error output
[argument]: What caused this error?
[file: config.yaml]: 
database:
  host: localhost
  port: 5432

Assistant Message (from session):
Based on the previous error, it seems like a connection issue...

=== END CONTEXT ===
```

## Working with Other Options

The `view` command respects the same options as regular queries:

### Custom System Prompt

```sh
cgip view --system-prompt "You are a senior developer" "review this code" -f app.py
```

### Different Models

```sh
cgip view --model gpt-4o "complex analysis task"
```

### Show Context Options

Combine with context display options:

```sh
cgip view --show-context --markdown "test query"
```

## Best Practices

### Before Expensive Queries

Always use `view` before sending large or complex contexts:

```sh
# Preview first
find . -name "*.py" -exec cat {} \; | cgip view "analyze all Python files"

# If context looks good, run the actual query
find . -name "*.py" -exec cat {} \; | cgip "analyze all Python files"
```

### Debugging Unexpected Results

When AI responses don't match expectations:

```sh
# See what context was actually sent
cat data.csv | cgip view "analyze this data"
```

### Context Size Management

Monitor context size for cost and performance:

```sh
# Check context before sending
tail -n 1000 huge-log.txt | cgip view "find critical errors"
```

### Session State Verification

Verify session state before important queries:

```sh
cgip view "continue our previous analysis"
```

## Troubleshooting

### Empty Context

If `view` shows no context:
- Check that your input methods (stdin, files) are working
- Verify file paths are correct
- Make sure you're not in a new session when expecting history

### Unexpected Context

If context includes unexpected content:
- Check for active sessions that might include previous conversation
- Verify file contents are what you expect
- Look for hidden characters in piped input

### Missing Context

If expected context is missing:
- Ensure stdin is properly piped (use `2>&1` for stderr)
- Verify file permissions and paths
- Check that session is properly configured

## Next Steps

- Learn about [session management](./session-command.md) to control conversation history
- Explore [configuration options](./config-command.md) to customize default behavior
- Try [basic usage patterns](./basic-usage.md) with the confidence of knowing your context