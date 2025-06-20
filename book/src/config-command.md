# Config Command

The `config` command allows you to set and retrieve default configuration values for Chat GipiTTY. This eliminates the need to specify common options repeatedly and provides a centralized way to manage your preferences.

## Overview

The configuration system uses a TOML file located in your system's config directory under `cgip/config.toml`. The config command provides a convenient interface to manage these settings without manually editing the file.

## Basic Usage

```sh
# Set a configuration value
cgip config --set key=value

# Get a configuration value
cgip config --get key
```

## Command Help

```
Set or get default configuration values with your config.toml

Usage: cgip config [OPTIONS]

Options:
  -s, --set <SET>  Set a configuration value. Use the format key=value. `cgip config --set model=gpt-4-turbo`
  -g, --get <GET>  Get your current configuration value. `cgip config --get model`
  -h, --help       Print help
  -V, --version    Print version
```

## Configuration Options

### Model Settings

Set your preferred default model:

```sh
cgip config --set model=gpt-4o
```

Get your current model:

```sh
cgip config --get model
```

### System Prompt

Configure a default system prompt:

```sh
cgip config --set system_prompt="You are a helpful coding assistant. Provide concise, accurate answers."
```

### API Settings

Configure API-related settings:

```sh
# Set custom base URL (alternatively use OPENAI_BASE_URL env var)
cgip config --set base_url=http://localhost:11434/v1

# Set default max tokens
cgip config --set max_tokens=2000
```

### Response Settings

Control response behavior:

```sh
# Set creativity level (0.0 to 2.0)
cgip config --set temperature=0.7

# Set response diversity
cgip config --set top_p=0.9
```

## Common Configuration Examples

### For Development Work

```sh
cgip config --set model=gpt-4o
cgip config --set system_prompt="You are an expert software engineer. Provide technical, detailed responses with code examples when relevant."
cgip config --set temperature=0.3
```

### For Creative Tasks

```sh
cgip config --set model=gpt-4
cgip config --set system_prompt="You are a creative writing assistant. Be imaginative and expressive."
cgip config --set temperature=1.2
```

### For Local AI Models

```sh
cgip config --set base_url=http://localhost:11434/v1
cgip config --set model=llama3
cgip config --set system_prompt="You are a helpful assistant running locally."
```

## Viewing All Configuration

To see all your current configuration settings, you can manually inspect the config file:

```sh
# On Linux/macOS
cat ~/.config/cgip/config.toml

# On macOS (alternative location)
cat ~/Library/Application\ Support/cgip/config.toml
```

## Configuration File Format

The configuration file uses TOML format:

```toml
model = "gpt-4o"
system_prompt = "You are a helpful assistant."
temperature = 0.7
max_tokens = 1500
base_url = "https://api.openai.com"
```

## Environment Variables vs Configuration

Configuration precedence (highest to lowest):

1. **Command-line options** (e.g., `--model gpt-4`)
2. **Environment variables** (e.g., `OPENAI_BASE_URL`)
3. **Configuration file** (set via `cgip config`)
4. **Built-in defaults**

This means you can override configuration file settings with environment variables or command-line options when needed.

## Common Configuration Patterns

### Project-Specific Configuration

For different projects, you might want different settings:

```sh
# Web development project
cgip config --set system_prompt="You are a web development expert. Focus on modern JavaScript, React, and Node.js."

# Data science project  
cgip config --set system_prompt="You are a data science expert. Focus on Python, pandas, and machine learning."
```

### Provider-Specific Settings

When switching between different AI providers:

```sh
# OpenAI setup
cgip config --set base_url=https://api.openai.com
cgip config --set model=gpt-4o

# Local Ollama setup
cgip config --set base_url=http://localhost:11434/v1
cgip config --set model=llama3
```

## Resetting Configuration

To reset a configuration value to its default:

```sh
# This will remove the setting from your config file
cgip config --set model=
```

Or manually edit the config file to remove unwanted settings.

## Troubleshooting

### Configuration Not Applied

If your configuration changes aren't taking effect:

1. Check the config file location and permissions
2. Verify the TOML syntax is correct
3. Remember that command-line options override config file settings
4. Check for environment variables that might override config

### Invalid Configuration Values

If you set an invalid value:

```sh
# Invalid model name
cgip config --set model=invalid-model
```

You'll get an error when trying to use Chat GipiTTY. Use `cgip config --get model` to verify your settings.

### Config File Location

If you're unsure where your config file is located:

```sh
# The config command will create the file if it doesn't exist
cgip config --set temp_key=temp_value
cgip config --set temp_key=  # Remove the temporary key
```

Check your system's standard config directory:
- Linux: `~/.config/cgip/config.toml`
- macOS: `~/Library/Application Support/cgip/config.toml` or `~/.config/cgip/config.toml`

## Best Practices

### Start with Essentials

Set up the most important settings first:

```sh
cgip config --set model=gpt-4o
cgip config --set system_prompt="Be concise and technical"
```

### Use Descriptive System Prompts

Create system prompts that clearly define the AI's role:

```sh
cgip config --set system_prompt="You are a senior software engineer specializing in Python and web development. Provide practical, production-ready solutions."
```

### Test Your Configuration

After setting up configuration, test it:

```sh
cgip "Hello, test my configuration"
```

### Back Up Your Configuration

Since configuration improves your workflow, consider backing up your config file:

```sh
cp ~/.config/cgip/config.toml ~/backups/cgip-config-backup.toml
```

## Next Steps

- Learn about [environment variables](./environment-variables.md) for temporary overrides
- Explore [advanced usage](./advanced-usage.md) patterns with your configured defaults
- Try [different models](./supported-models.md) to find what works best for your use cases