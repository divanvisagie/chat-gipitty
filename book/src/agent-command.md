# Agent Command

The `agent` subcommand provides an agentic workflow that lets the model run shell commands on your behalf. It offers a powerful way to delegate complex tasks that require multiple command executions and iterative problem-solving.

## Overview

The agent subcommand gives the AI model access to a special `execute` tool that allows it to run shell commands in a specified directory. The model can use command output as feedback to make decisions about what to do next, creating an autonomous workflow for task completion.

## Usage

```bash
cgip agent [OPTIONS] <DIRECTORY> <INSTRUCTION>
```

### Arguments

- `<DIRECTORY>` - Directory the agent is allowed to operate in (required)
- `<INSTRUCTION>` - Natural language instruction describing the goal (required)

### Options

- `--input <FILES>` - Comma separated list of files whose contents should be added to the context
- `--max-actions <N>` - Maximum number of commands the agent will execute before stopping (default: 10)

## How It Works

When invoked, the agent:

1. **Receives your instruction** and any provided file contents
2. **Gets access to an `execute` tool** that can run shell commands
3. **Runs commands iteratively**, using output to inform next steps
4. **Continues until** the task is complete or max actions reached
5. **Provides a summary** of all commands executed

The model has access to this tool definition:
```json
{
  "type": "function",
  "function": {
    "name": "execute",
    "description": "Run a shell command",
    "parameters": {
      "type": "object",
      "properties": {
        "command": {"type": "string"}
      },
      "required": ["command"]
    }
  }
}
```

## Basic Examples

### Simple Directory Listing
```bash
cgip agent . "list the current directory"
```

The agent will execute `ls` and return the results.

### Project Analysis
```bash
cgip agent /path/to/project "analyze the project structure and identify the main components"
```

The agent might:
1. Run `find . -name "*.py" | head -10` to find Python files
2. Run `cat requirements.txt` to check dependencies  
3. Run `ls -la` to see the overall structure
4. Provide a comprehensive analysis based on findings

### Build and Test
```bash
cgip agent . "build the project and run tests, report any issues"
```

The agent could:
1. Detect the project type (e.g., `cargo build` for Rust)
2. Run the build command
3. Execute tests if build succeeds
4. Analyze any errors and suggest fixes

## Advanced Examples

### Code Quality Analysis
```bash
cgip agent src/ "analyze code quality and suggest improvements" --input "package.json,README.md"
```

With file input providing context, the agent might:
1. Run linting tools appropriate for the language
2. Check for security vulnerabilities
3. Analyze code complexity
4. Review documentation coverage

### Environment Setup
```bash
cgip agent . "set up development environment for this project" --max-actions 15
```

The agent could:
1. Detect project requirements
2. Install dependencies
3. Set up configuration files
4. Run initial setup commands
5. Verify everything works

### Debugging Assistance
```bash
cgip agent . "investigate why tests are failing and fix the issues"
```

The agent might:
1. Run the test suite to see failures
2. Examine failing test output
3. Look at relevant source files
4. Make necessary fixes
5. Re-run tests to verify fixes

## Options in Detail

### `--input <FILES>`
Provide additional context by including file contents:

```bash
# Single file
cgip agent . "optimize this code" --input "src/main.py"

# Multiple files
cgip agent . "review these components" --input "src/user.py,src/auth.py,tests/test_auth.py"

# Configuration files for context
cgip agent . "deploy this application" --input "docker-compose.yml,package.json"
```

### `--max-actions <N>`
Control how many commands the agent can execute:

```bash
# Quick tasks
cgip agent . "check project status" --max-actions 3

# Complex tasks
cgip agent . "refactor the codebase" --max-actions 20

# Default is 10 actions
cgip agent . "analyze and improve performance"
```

## Safety Features

### Directory Restriction
The agent can only operate within the specified directory:

```bash
# Agent limited to current directory
cgip agent . "clean up temporary files"

# Agent limited to specific subdirectory
cgip agent src/ "refactor the source code"
```

This prevents the agent from:
- Accessing files outside the specified directory
- Making system-wide changes
- Affecting other projects

### Action Limits
The `--max-actions` limit prevents runaway execution:
- Stops infinite loops
- Limits resource usage
- Ensures predictable completion time

### Command Visibility
All executed commands are:
- Shown in real-time as they run
- Included in the final summary
- Available for review and audit

## Best Practices

### 1. Start with Clear Instructions
```bash
# Good: Specific and actionable
cgip agent . "find all Python files with TODO comments and create a task list"

# Less effective: Vague goal
cgip agent . "improve the project"
```

### 2. Use Appropriate Directory Scope
```bash
# Focused scope for specific tasks
cgip agent src/ "refactor utility functions"

# Broader scope for project-wide tasks
cgip agent . "set up CI/CD pipeline"
```

### 3. Provide Relevant Context
```bash
# Include relevant files for context
cgip agent . "update dependencies" --input "package.json,requirements.txt"
```

### 4. Set Appropriate Action Limits
```bash
# Simple tasks: low limit
cgip agent . "check syntax errors" --max-actions 5

# Complex tasks: higher limit
cgip agent . "migrate database schema" --max-actions 15
```

## Common Use Cases

### Development Tasks
```bash
# Code generation
cgip agent . "create unit tests for all functions in src/utils.py"

# Refactoring
cgip agent src/ "extract common code into shared utilities"

# Documentation
cgip agent . "generate API documentation from code comments"
```

### Project Management
```bash
# Dependency management
cgip agent . "audit and update all dependencies to latest versions"

# Project setup
cgip agent . "initialize project with standard configuration files"

# Cleanup
cgip agent . "remove unused files and clean up directory structure"
```

### Analysis and Reporting
```bash
# Code analysis
cgip agent . "analyze code complexity and generate report"

# Security audit
cgip agent . "scan for potential security vulnerabilities"

# Performance analysis
cgip agent . "profile the application and identify bottlenecks"
```

## Output and Summary

After completion, the agent provides:

### Real-time Output
Commands and their output are shown as they execute:
```
Executing: ls -la
total 24
drwxr-xr-x  5 user user 4096 Jan 15 10:30 .
drwxr-xr-x  3 user user 4096 Jan 15 10:25 ..
-rw-r--r--  1 user user  123 Jan 15 10:30 main.py
...

Executing: python -m pytest
===== test session starts =====
...
```

### Final Summary
A summary of all executed commands:
```
Agent completed successfully. Commands executed:
1. ls -la
2. python -m pytest
3. cat test_results.txt

Task completed: All tests are passing and the project structure looks good.
```

## Troubleshooting

### Permission Issues
If the agent can't execute commands:
```bash
# Check directory permissions
ls -la /path/to/directory

# Ensure you have execute permissions
chmod +x /path/to/directory
```

### Command Not Found
If commands fail:
```bash
# Check if required tools are installed
which python
which npm
which cargo

# Install missing dependencies
```

### Action Limit Reached
If the agent stops due to action limits:
```bash
# Increase the limit for complex tasks
cgip agent . "complex task" --max-actions 20

# Or break down into smaller tasks
cgip agent . "first part of complex task" --max-actions 10
```

## Integration Examples

### With CI/CD
```bash
# In a CI script
cgip agent . "run all tests and generate coverage report" --max-actions 5
```

### With Development Workflow
```bash
# Pre-commit hook
cgip agent . "check code quality and fix simple issues" --max-actions 8
```

### With Documentation
```bash
# Documentation generation
cgip agent . "update README with current project status" --input "package.json,src/main.py"
```
