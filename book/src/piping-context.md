# Piping and Context

One of Chat Gipitty's most powerful features is its ability to seamlessly integrate with shell pipelines and compile context from multiple sources. This makes it incredibly useful for debugging, analysis, and processing command output.

## Context Priority Order

Chat Gipitty compiles context for queries by prioritizing input in this specific order:

1. **stdin** (highest priority)
2. **Command-line arguments** 
3. **Files** (specified with `-f` flag, lowest priority)

This ordering allows you to build complex prompts by combining different input sources.

## Piping Examples

### Basic Piping

Pipe command output directly to Chat Gipitty:

```bash
# Debug compilation errors
cargo build 2>&1 | cgip "explain this error"

# Analyze log files
tail -f /var/log/app.log | cgip "summarize the errors"

# Process directory listings
ls -la | cgip "what can you tell me about these files?"
```

### Converting stderr to stdout

Many commands output errors to stderr. Use `2>&1` to redirect stderr to stdout so it can be piped:

```bash
# Capture both stdout and stderr
cargo build 2>&1 | cgip "give me a short summary of the kind of error this is"

# Debug failed tests
cargo test 2>&1 | cgip "what tests are failing and why?"
```

### Complex Pipeline Examples

```bash
# Analyze system processes
ps aux | head -20 | cgip "which processes are using the most resources?"

# Git log analysis
git log --oneline -10 | cgip "summarize the recent changes"

# Network analysis
netstat -an | cgip "are there any suspicious network connections?"

# File system analysis
du -sh * | cgip "which directories are taking up the most space?"
```

## Combining Input Sources

### stdin + Arguments

```bash
# Pipe input and add additional context
ls -la | cgip "analyze these files and suggest cleanup actions"

# Process command output with specific instructions
docker ps | cgip "which containers might have issues?"
```

### stdin + Files

```bash
# Combine piped input with file content
cat error.log | cgip -f config.yaml "analyze this error in context of the config"
```

### Arguments + Files

```bash
# Combine direct text with file content
cgip "convert this to python" -f src/main.rs

# Add context to file analysis
cgip "explain this code and suggest improvements" -f script.sh
```

## Context Viewing

Use the `--show-context` or `-c` flag to see exactly what context is being sent:

```bash
ls | cgip -c "what files are here?"
```

This will show you the full context including:
- The piped input (ls output)
- Your query
- Any session history
- System prompts

## Advanced Context Options

### Markdown Output
Use `-m` or `--markdown` to format context in a human-readable table:

```bash
ps aux | cgip -m "analyze these processes"
```

### No Session Context
Use `-n` or `--no-session` to exclude session history from the context:

```bash
sensitive_command | cgip -n "analyze this output"
```

### Progress Indicators
Use `-p` or `--show-progress` to see progress (note: this might interfere with stdout):

```bash
large_command | cgip -p "process this data"
```

## Best Practices

### 1. Error Handling
Always use `2>&1` when you want to capture error output:

```bash
# Good: Captures both success and error output
command 2>&1 | cgip "analyze the result"

# Limited: Only captures success output
command | cgip "analyze the result"
```

### 2. Data Size Considerations
Be mindful of large outputs that might exceed token limits:

```bash
# Good: Limit output size
head -100 large_file.log | cgip "analyze these log entries"

# Potentially problematic: Entire large file
cat huge_file.log | cgip "analyze this"
```

### 3. Structured Output
Some commands have structured output that works well with Chat Gipitty:

```bash
# JSON output
kubectl get pods -o json | cgip "which pods are not running?"

# CSV data
cat data.csv | cgip "find anomalies in this data"

# YAML configuration
cat config.yaml | cgip "check this configuration for issues"
```

### 4. Real-time Processing
For real-time log analysis:

```bash
# Monitor logs in real-time
tail -f /var/log/app.log | cgip "alert me to any errors"

# Watch system resources
watch -n 5 'ps aux --sort=-%cpu | head -10' | cgip "monitor for resource issues"
```

## Integration with Development Workflow

### Code Analysis
```bash
# Check code quality
eslint src/ | cgip "what are the main code quality issues?"

# Analyze test failures
npm test 2>&1 | cgip "why are these tests failing?"
```

### System Administration
```bash
# Check system health
systemctl status | cgip "are there any service issues?"

# Analyze disk usage
df -h | cgip "do I have any disk space issues?"
```

### Data Processing
```bash
# Process CSV data
cat sales_data.csv | cgip "calculate the total revenue by region"

# Analyze API responses
curl -s https://api.example.com/data | cgip "extract key insights from this API response"
```
