# Configuration

Chat Gipitty can be configured through multiple methods to customize its behavior for your needs. This page covers all the configuration options available and how to set them.

## Configuration Methods

Chat Gipitty supports configuration through:

1. **Configuration file** (`config.toml`)
2. **Environment variables**
3. **Command-line options**

Configuration is applied in order of precedence:
1. Command-line options (highest priority)
2. Environment variables
3. Configuration file (lowest priority)

## Configuration File

### Location

The configuration file is located in your system's configuration directory:

- **Linux**: `~/.config/cgip/config.toml`
- **macOS**: `~/Library/Application Support/cgip/config.toml`
- **Windows**: `%APPDATA%\cgip\config.toml`

### File Format

The configuration file uses TOML format:

```toml
# Chat Gipitty Configuration File

# Default model to use
model = "gpt-4"

# Custom API base URL (optional)
base_url = "https://api.openai.com"

# Default system prompt (optional)
system_prompt = "You are a helpful assistant."

# Show progress indicator by default
show_progress = false

# Default to markdown output
markdown = false

# Session configuration
session_name = ""

# File input settings
default_file_extensions = [".txt", ".md", ".rs", ".py", ".js"]
```

### Available Options

#### Core Settings

**`model`** (string)
- Default model to use for queries
- Default: `"gpt-4"`
- Example: `"gpt-3.5-turbo"`, `"gpt-4-turbo"`

**`base_url`** (string)
- Custom API base URL for alternative providers
- Default: `"https://api.openai.com"`
- Example: `"http://localhost:11434/v1"` for Ollama

**`system_prompt`** (string)
- Default system prompt for all queries
- Default: None
- Example: `"You are a coding assistant"`

#### Display Settings

**`show_progress`** (boolean)
- Show progress indicator by default
- Default: `false`
- Equivalent to `-p, --show-progress`

**`markdown`** (boolean)
- Format output in markdown by default
- Default: `false`
- Equivalent to `-m, --markdown`

**`show_context`** (boolean)
- Show context by default
- Default: `false`
- Equivalent to `-c, --show-context`

#### Session Settings

**`session_name`** (string)
- Default session name
- Default: Empty (no sessions)
- Can be overridden by `CGIP_SESSION_NAME` environment variable

**`no_session`** (boolean)
- Disable sessions by default
- Default: `false`
- Equivalent to `-n, --no-session`

## Using the Config Command

The `config` subcommand allows you to view and modify configuration values:

### View Configuration

```bash
# View all configuration
cgip config

# View specific setting
cgip config --get model
cgip config --get base_url
```

### Set Configuration

```bash
# Set model
cgip config --set model=gpt-4-turbo

# Set custom API endpoint
cgip config --set base_url=http://localhost:11434/v1

# Set system prompt
cgip config --set system_prompt="You are a helpful coding assistant"

# Enable progress indicator by default
cgip config --set show_progress=true
```

### Examples

```bash
# Configure for OpenAI GPT-4
cgip config --set model=gpt-4
cgip config --set base_url=https://api.openai.com

# Configure for local Ollama
cgip config --set model=llama2
cgip config --set base_url=http://localhost:11434/v1

# Configure for daily sessions
cgip config --set session_name=$(date -I)

# Set a default system prompt
cgip config --set system_prompt="You are an expert programmer. Provide concise, accurate answers."
```

## Environment Variables

Environment variables override configuration file settings:

### Required Variables

**`OPENAI_API_KEY`**
- Your API key for the configured provider
- Required for most functionality

### Optional Variables

**`OPENAI_BASE_URL`**
- Custom API endpoint
- Overrides `base_url` in config file

**`CGIP_SESSION_NAME`**
- Session name for session management
- Overrides `session_name` in config file

### Examples

```bash
# Basic OpenAI setup
export OPENAI_API_KEY="sk-your-key-here"

# Custom provider setup
export OPENAI_API_KEY="your-provider-key"
export OPENAI_BASE_URL="https://api.provider.com/v1"

# Session management
export CGIP_SESSION_NAME=$(date -I)
```

## Command-Line Options

Command-line options have the highest precedence and override both environment variables and configuration file settings.

### Model Selection

```bash
# Override configured model
cgip -M gpt-3.5-turbo "query"

# List available models
cgip -l
```

### Output Control

```bash
# Show progress (overrides config)
cgip -p "long running query"

# Show context (overrides config)
cgip -c "query with context"

# Markdown output (overrides config)
cgip -m "formatted query"
```

### Session Control

```bash
# Disable session for this query
cgip -n "isolated query"
```

## Configuration Examples

### Developer Setup

Configuration for software development:

```toml
# config.toml
model = "gpt-4"
system_prompt = "You are an expert software developer. Provide concise, practical solutions."
show_progress = true
show_context = false
```

```bash
# Environment
export OPENAI_API_KEY="your-key"
export CGIP_SESSION_NAME="dev-$(date +%Y%m%d)"
```

### Data Analysis Setup

Configuration for data analysis tasks:

```toml
# config.toml
model = "gpt-4-turbo"
system_prompt = "You are a data analyst. Focus on insights and actionable recommendations."
markdown = true
show_progress = true
```

### Local AI Setup

Configuration for local AI models via Ollama:

```toml
# config.toml
model = "llama2"
base_url = "http://localhost:11434/v1"
show_progress = true
```

```bash
# Environment (Ollama doesn't need real API key)
export OPENAI_API_KEY="ollama"
```

### Multi-Provider Setup

Using shell functions for different providers:

```bash
# In ~/.bashrc or ~/.zshrc

# OpenAI function
openai() {
    OPENAI_API_KEY="$OPENAI_KEY" \
    OPENAI_BASE_URL="https://api.openai.com" \
    cgip -M gpt-4 "$@"
}

# Local function
local() {
    OPENAI_API_KEY="ollama" \
    OPENAI_BASE_URL="http://localhost:11434/v1" \
    cgip -M llama2 "$@"
}

# Claude function (via proxy)
claude() {
    OPENAI_API_KEY="$CLAUDE_KEY" \
    OPENAI_BASE_URL="https://claude-proxy.com/v1" \
    cgip -M claude-3 "$@"
}
```

## Configuration Validation

### Check Current Configuration

```bash
# View all current settings
cgip config

# Test configuration
cgip "test query"

# Verify model availability
cgip -l
```

### Common Configuration Issues

**Invalid Model**:
```bash
cgip config --set model=gpt-4
cgip -l  # Verify model is available
```

**Wrong API Endpoint**:
```bash
cgip config --set base_url=https://api.openai.com
# Test with a simple query
cgip "hello"
```

**Missing API Key**:
```bash
export OPENAI_API_KEY="your-key"
# Or check if it's set
echo $OPENAI_API_KEY
```

## Advanced Configuration

### Project-Specific Configuration

Create project-specific configuration with shell functions:

```bash
# In project directory
project_cgip() {
    local config_file="$(pwd)/.cgip.toml"
    if [[ -f "$config_file" ]]; then
        CGIP_CONFIG_FILE="$config_file" cgip "$@"
    else
        cgip "$@"
    fi
}
```

### Conditional Configuration

Configure based on environment:

```bash
# In ~/.bashrc or ~/.zshrc
if [[ "$ENVIRONMENT" == "development" ]]; then
    export OPENAI_BASE_URL="http://localhost:11434/v1"
    export OPENAI_API_KEY="ollama"
else
    export OPENAI_API_KEY="$PRODUCTION_OPENAI_KEY"
fi
```

### Configuration Templates

Save common configurations as templates:

```bash
# Save current config as template
cp ~/.config/cgip/config.toml ~/.config/cgip/templates/dev.toml

# Load template
cp ~/.config/cgip/templates/dev.toml ~/.config/cgip/config.toml
```

## Troubleshooting Configuration

### Debug Configuration

```bash
# Check where config file should be
cgip config --get model 2>&1 | grep -i "config"

# Verify environment variables
env | grep -E "(OPENAI|CGIP)"

# Test with explicit options
cgip -M gpt-3.5-turbo "test"
```

### Reset Configuration

```bash
# Remove config file to reset to defaults
rm ~/.config/cgip/config.toml

# Or reset specific settings
cgip config --set model=gpt-4
cgip config --set base_url=https://api.openai.com
```

### Configuration Precedence Testing

```bash
# Test precedence (command line > env > config)
# 1. Set in config file
cgip config --set model=gpt-3.5-turbo

# 2. Override with environment
CGIP_MODEL=gpt-4 cgip "test"

# 3. Override with command line
CGIP_MODEL=gpt-4 cgip -M gpt-4-turbo "test"
```

## Security Considerations

### API Key Security

- Never commit API keys to version control
- Use environment variables for sensitive data
- Consider using secret management systems
- Rotate keys regularly

### Configuration File Permissions

```bash
# Secure config file
chmod 600 ~/.config/cgip/config.toml

# Verify permissions
ls -la ~/.config/cgip/config.toml
```

### Environment Variable Security

```bash
# Use env files that are gitignored
echo "OPENAI_API_KEY=your-key" > .env
echo ".env" >> .gitignore
source .env
```
