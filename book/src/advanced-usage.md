# Advanced Usage

This section covers advanced usage patterns and features for power users who want to get the most out of Chat GipiTTY. These features allow for more sophisticated workflows and customization options.

## Overview

Advanced usage includes:

- **[File Input](./file-input.md)** - Advanced file handling and multiple file processing
- **[Model Selection](./model-selection.md)** - Choosing and switching between different AI models
- **[System Prompts](./system-prompts.md)** - Customizing AI behavior with system-level instructions

## Key Concepts

### Context Management
Advanced users can precisely control how Chat GipiTTY builds context from multiple sources:

```sh
# Combine multiple inputs with priority control
cat data.csv | cgip "analyze this data" -f config.yaml --no-session
```

### Custom Workflows
Create sophisticated pipelines that leverage Chat GipiTTY's flexibility:

```sh
# Multi-step analysis pipeline
find . -name "*.py" | xargs wc -l | sort -nr | head -10 | cgip "which files need refactoring?"
```

### Provider-Specific Features
Take advantage of unique capabilities offered by different AI providers:

```sh
# Use specific models for specialized tasks
cgip -M gpt-4o-search-preview "/search latest Python security vulnerabilities"
cgip -M claude-3-sonnet "provide detailed code analysis" -f complex_algorithm.py
```

## Advanced Configuration

### Environment-Based Configuration
Set up different configurations for different environments:

```sh
# Development environment
export CGIP_SESSION_NAME="dev-$(date +%Y-%m-%d)"
export OPENAI_BASE_URL="http://localhost:11434/v1"

# Production analysis environment  
export CGIP_SESSION_NAME="prod-analysis"
export OPENAI_BASE_URL="https://api.openai.com"
```

### Custom Model Aliases
Create shortcuts for frequently used model configurations:

```sh
# Set up aliases in your shell
alias cgip-fast="cgip -M gpt-3.5-turbo"
alias cgip-smart="cgip -M gpt-4o"
alias cgip-local="cgip -M llama2:7b"
```

## Performance Optimization

### Token Management
For large inputs, be strategic about token usage:

```sh
# Limit context size for faster responses
head -100 large_file.log | cgip "summarize the key issues"

# Use specific prompts to get concise answers
cgip "list the top 3 issues only" -f error_log.txt
```

### Batch Processing
Process multiple files efficiently:

```sh
# Process files in batches
for file in *.py; do
    echo "=== $file ===" 
    cgip "review this code" -f "$file" --no-session
    echo
done
```

## Integration Patterns

### Shell Functions
Create reusable shell functions for common tasks:

```bash
# Add to your .bashrc or .zshrc
analyze_logs() {
    tail -n ${2:-100} "$1" | cgip "analyze these logs for issues"
}

review_code() {
    cgip "review this code for bugs and improvements" -f "$1"
}

explain_command() {
    man "$1" | cgip "explain this command in simple terms"
}
```

### Git Hooks
Integrate Chat GipiTTY into your git workflow:

```bash
#!/bin/bash
# pre-commit hook
git diff --cached | cgip "review these changes for potential issues" --no-session
```

## Troubleshooting Advanced Usage

### Context Too Large
If you hit token limits:
- Use `head`/`tail` to limit input size
- Use `--no-session` to exclude session history
- Be more specific in your prompts

### Model Compatibility
Different models have different capabilities:
- Vision models for image analysis
- Search-enabled models for web queries
- Local models may have limited features

### Performance Issues
To improve response times:
- Choose appropriate models for the task
- Limit context size
- Use local models for simple tasks

For specific details on each advanced feature, see the individual sections in this chapter.
