# Environment Variables

Chat Gipitty uses several environment variables for configuration. These variables allow you to customize the behavior without modifying configuration files or passing command-line arguments repeatedly.

## Required Variables

### `OPENAI_API_KEY`
**Required for most functionality**

Your OpenAI API key or compatible API key for your chosen provider.

```bash
export OPENAI_API_KEY="sk-your-api-key-here"
```

For other providers:
```bash
# Google Gemini
export OPENAI_API_KEY="your-gemini-api-key"

# Anthropic Claude
export OPENAI_API_KEY="your-claude-api-key"

# Mistral AI
export OPENAI_API_KEY="your-mistral-api-key"
```

## Optional Variables

### `OPENAI_BASE_URL`
**Default:** `https://api.openai.com`

Specify a custom API endpoint for OpenAI-compatible providers.

```bash
# Local Ollama instance
export OPENAI_BASE_URL="http://localhost:11434/v1"

# Google Gemini (via OpenAI-compatible proxy)
export OPENAI_BASE_URL="https://generativelanguage.googleapis.com/v1beta"

# Mistral AI
export OPENAI_BASE_URL="https://api.mistral.ai/v1"

# Anthropic Claude (via proxy)
export OPENAI_BASE_URL="https://api.anthropic.com/v1"

# Custom provider
export OPENAI_BASE_URL="https://your-provider.com/v1"
```

#### URL Construction Logic

Chat Gipitty intelligently constructs the API endpoint:

- If your base URL already contains `/chat/completions`, it uses it as-is
- If your base URL ends with `/v1` (or similar version pattern), it appends `/chat/completions`
- Otherwise, it appends `/v1/chat/completions` (standard OpenAI pattern)

Examples:
```bash
# These all work correctly:
export OPENAI_BASE_URL="https://api.example.com/v1"
# Results in: https://api.example.com/v1/chat/completions

export OPENAI_BASE_URL="https://api.example.com/v2/chat/completions"
# Results in: https://api.example.com/v2/chat/completions (used as-is)

export OPENAI_BASE_URL="https://api.example.com"
# Results in: https://api.example.com/v1/chat/completions
```

### `CGIP_SESSION_NAME`
**Default:** No session management

Controls session management and uniqueness. The uniqueness of your session depends on the value of this variable.

```bash
# New session for each terminal (using uuid)
export CGIP_SESSION_NAME=$(uuid)

# Daily sessions (same session for entire day)
export CGIP_SESSION_NAME=$(date -I)

# Weekly sessions
export CGIP_SESSION_NAME=$(date +%Y-W%U)

# Project-specific sessions
export CGIP_SESSION_NAME="project-$(basename $PWD)"

# Manual session naming
export CGIP_SESSION_NAME="my-coding-session"
```

#### Session Examples

**Terminal-specific sessions:**
```bash
# Add to ~/.bashrc or ~/.zshrc
export CGIP_SESSION_NAME=$(uuid)
```

**Date-based sessions:**
```bash
# Daily sessions
export CGIP_SESSION_NAME=$(date -I)  # 2024-01-15

# Weekly sessions  
export CGIP_SESSION_NAME=$(date +%Y-W%U)  # 2024-W03

# Monthly sessions
export CGIP_SESSION_NAME=$(date +%Y-%m)  # 2024-01
```

**Project-based sessions:**
```bash
# Use current directory name
export CGIP_SESSION_NAME="project-$(basename $PWD)"

# Use git repository name
export CGIP_SESSION_NAME="git-$(git rev-parse --show-toplevel | xargs basename)"
```

## Configuration in Shell Profiles

### Bash (~/.bashrc)
```bash
# Basic configuration
export OPENAI_API_KEY="your-api-key-here"
export CGIP_SESSION_NAME=$(uuid)

# Custom provider configuration
export OPENAI_BASE_URL="http://localhost:11434/v1"
export OPENAI_API_KEY="ollama"  # Ollama doesn't require a real key
```

### Zsh (~/.zshrc)
```bash
# Basic configuration
export OPENAI_API_KEY="your-api-key-here"
export CGIP_SESSION_NAME=$(uuid)

# Project-aware sessions
export CGIP_SESSION_NAME="$(basename $PWD)-$(date -I)"
```

### Fish (~/.config/fish/config.fish)
```fish
# Basic configuration
set -gx OPENAI_API_KEY "your-api-key-here"
set -gx CGIP_SESSION_NAME (uuid)

# Daily sessions
set -gx CGIP_SESSION_NAME (date -I)
```

## Provider-Specific Configurations

### OpenAI (Default)
```bash
export OPENAI_API_KEY="sk-your-openai-key"
# OPENAI_BASE_URL uses default: https://api.openai.com
```

### Ollama (Local)
```bash
export OPENAI_BASE_URL="http://localhost:11434/v1"
export OPENAI_API_KEY="ollama"  # Placeholder, not used by Ollama
```

### Google Gemini
```bash
export OPENAI_BASE_URL="https://generativelanguage.googleapis.com/v1beta"
export OPENAI_API_KEY="your-gemini-api-key"
```

### Mistral AI
```bash
export OPENAI_BASE_URL="https://api.mistral.ai/v1"
export OPENAI_API_KEY="your-mistral-api-key"
```

### Anthropic Claude (via proxy)
```bash
export OPENAI_BASE_URL="https://api.anthropic.com/v1"
export OPENAI_API_KEY="your-claude-api-key"
```

## Advanced Configuration

### Multiple API Keys
For different projects or providers:

```bash
# Project-specific configuration
if [[ "$PWD" == *"work-project"* ]]; then
    export OPENAI_API_KEY="$WORK_OPENAI_KEY"
    export OPENAI_BASE_URL="https://work-api.company.com/v1"
else
    export OPENAI_API_KEY="$PERSONAL_OPENAI_KEY"
    # Use default OpenAI endpoint
fi
```

### Dynamic Session Names
```bash
# Function to generate smart session names
cgip_session() {
    if git rev-parse --git-dir > /dev/null 2>&1; then
        # In a git repository
        echo "git-$(git rev-parse --show-toplevel | xargs basename)-$(date +%Y%m%d)"
    else
        # Not in git, use directory and date
        echo "$(basename $PWD)-$(date +%Y%m%d)"
    fi
}

export CGIP_SESSION_NAME=$(cgip_session)
```

### Conditional Configuration
```bash
# Use local Ollama during development, OpenAI in production
if [[ "$ENVIRONMENT" == "development" ]]; then
    export OPENAI_BASE_URL="http://localhost:11434/v1"
    export OPENAI_API_KEY="ollama"
else
    export OPENAI_API_KEY="$PRODUCTION_OPENAI_KEY"
fi
```

## Troubleshooting

### Common Issues

**API Key not found:**
```bash
# Check if variable is set
echo $OPENAI_API_KEY

# Set temporarily for testing
OPENAI_API_KEY="your-key" cgip "test query"
```

**Wrong endpoint:**
```bash
# Check current base URL
echo $OPENAI_BASE_URL

# Test with explicit URL
OPENAI_BASE_URL="http://localhost:11434/v1" cgip "test query"
```

**Session not working:**
```bash
# Check session name
echo $CGIP_SESSION_NAME

# Test session functionality
cgip session --view
```

### Debugging Environment
```bash
# Show all cgip-related environment variables
env | grep -E "(OPENAI|CGIP)"

# Test configuration
cgip config --get model
cgip config --get base_url
```

## Security Considerations

### API Key Security
- Never commit API keys to version control
- Use environment files that are gitignored
- Consider using secret management tools for production

### Environment Files
Create a `.env` file (add to `.gitignore`):
```bash
# .env file
OPENAI_API_KEY=your-secret-key
OPENAI_BASE_URL=https://api.openai.com
CGIP_SESSION_NAME=my-session
```

Load with:
```bash
# Load environment file
source .env
```

### Session Privacy
- Be aware that session names might be visible in process lists
- Use generic session names for sensitive work
- Clear sessions when working with confidential information:
  ```bash
  cgip session --clear
  ```
