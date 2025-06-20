# File Input

Chat GipiTTY provides powerful file input capabilities that allow you to include file contents as context for AI analysis. This section covers advanced file handling techniques and best practices.

## Basic File Input

The `-f` or `--file` flag includes file contents in your query:

```sh
cgip "explain this code" -f src/main.py
```

## Multiple File Input

You can include multiple files by using the `-f` flag multiple times:

```sh
cgip "compare these implementations" -f version1.py -f version2.py
```

## File Input with Other Context

File input works seamlessly with other input sources:

```sh
# Combine piped input, arguments, and files
git diff | cgip "review these changes" -f CONTRIBUTING.md
```

## Input Priority Order

Chat GipiTTY processes input in this priority order:
1. **stdin** (highest priority)
2. **Command arguments**
3. **Files** (lowest priority)

This means files provide background context while stdin and arguments take precedence.

## Advanced File Patterns

### Wildcard Expansion
Use shell wildcards to include multiple files:

```sh
# Include all Python files (shell expands the wildcard)
cgip "analyze this codebase" -f src/*.py

# Include specific file types
cgip "review configuration" -f config/*.yaml -f config/*.json
```

### Directory Analysis
Analyze entire directories by combining with other tools:

```sh
# Analyze all files in a directory
find src/ -name "*.py" | head -5 | xargs -I {} cgip "brief analysis" -f {}

# Get file listing with analysis
ls -la src/ | cgip "what can you tell me about this project structure?" -f README.md
```

## File Type Handling

### Text Files
Most text-based files work seamlessly:

```sh
# Code files
cgip "review this code" -f app.py -f test.py

# Configuration files  
cgip "check this configuration" -f config.yaml -f .env

# Documentation
cgip "summarize these docs" -f README.md -f CHANGELOG.md
```

### Binary Files
Binary files are not supported directly, but you can analyze them indirectly:

```sh
# Analyze binary metadata
file suspicious_binary | cgip "what type of file is this?"

# Check binary dependencies
ldd my_program | cgip "analyze these dependencies"
```

### Large Files
For large files, consider limiting content:

```sh
# First 100 lines
head -100 large_log.txt | cgip "analyze these log entries"

# Last 50 lines
tail -50 large_log.txt | cgip "what are the recent issues?"

# Specific sections
sed -n '100,200p' large_file.txt | cgip "analyze this section"
```

## Practical Examples

### Code Review Workflow
```sh
# Review changes with context
git diff HEAD~1 | cgip "review these changes" -f README.md -f package.json

# Analyze function with its tests
cgip "review this function and its tests" -f src/utils.py -f tests/test_utils.py
```

### Configuration Analysis
```sh
# Check configuration consistency
cgip "are these configs consistent?" -f prod.yaml -f staging.yaml -f dev.yaml

# Analyze configuration with documentation
cgip "validate this configuration" -f config.yaml -f config-schema.json
```

### Documentation Tasks
```sh
# Generate documentation from code
cgip "create API docs for these modules" -f api/*.py

# Update documentation based on changes
git diff --name-only | grep '\.py$' | head -3 | xargs -I {} cgip "update docs for changes" -f {} -f docs/api.md
```

### Data Analysis
```sh
# Analyze data with metadata
cgip "analyze this dataset" -f data.csv -f metadata.json

# Compare data files
cgip "what changed between these datasets?" -f old_data.csv -f new_data.csv
```

## Best Practices

### File Size Considerations
- **Small files** (< 1KB): Include multiple files freely
- **Medium files** (1-10KB): Include selectively 
- **Large files** (> 10KB): Use `head`/`tail`/`grep` to extract relevant sections

### Context Management
```sh
# Include relevant context files
cgip "debug this error" -f error.log -f config.yaml -f README.md

# Avoid overwhelming context
cgip "analyze main function only" -f <(grep -A 50 "def main" script.py)
```

### Security Considerations
- Never include files with secrets or API keys
- Be cautious with sensitive configuration files
- Use `.gitignore` patterns as a guide for what to exclude

### Performance Tips
```sh
# Use specific file sections
cgip "analyze the error handling" -f <(grep -A 10 -B 5 "try:" app.py)

# Combine preprocessing with file input
cgip "analyze recent errors" -f <(grep ERROR app.log | tail -20)
```

## Error Handling

### File Not Found
```sh
# Check if file exists before using
[[ -f "config.yaml" ]] && cgip "analyze config" -f config.yaml || echo "Config file not found"
```

### Permission Issues
```sh
# Use sudo if needed for system files
sudo cat /var/log/syslog | tail -50 | cgip "analyze system logs"
```

### Large Context Warnings
If you get context too large errors:
- Reduce file count
- Use file excerpts instead of full files
- Process files individually

## Integration with Workflows

### CI/CD Integration
```sh
# Analyze test failures with source
npm test 2>&1 | cgip "analyze test failures" -f package.json -f src/main.js
```

### Development Workflow
```sh
# Pre-commit analysis
git diff --cached --name-only | grep -E '\.(py|js|ts)$' | head -3 | xargs -I {} cgip "quick review" -f {}
```

### System Administration
```sh
# Analyze system issues with config context
dmesg | tail -20 | cgip "analyze system messages" -f /etc/fstab -f /etc/systemd/system/my-service.service
```

File input is one of Chat GipiTTY's most powerful features, enabling you to provide rich context for AI analysis while maintaining the flexibility of command-line workflows.
