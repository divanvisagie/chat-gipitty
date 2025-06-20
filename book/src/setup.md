# Setup

After installing Chat GipiTTY, you'll need to configure it with your API credentials and any custom settings. This page covers the essential setup steps to get you started.

## API Key Configuration

Chat GipiTTY requires an API key to communicate with AI services. The setup process depends on which provider you're using.

### OpenAI API Key

For OpenAI services (recommended), set up your API key as an environment variable:

```sh
export OPENAI_API_KEY=your_key_here
```

To make this permanent, add the export statement to your shell profile:

```sh
# For bash users
echo 'export OPENAI_API_KEY=your_key_here' >> ~/.bashrc
source ~/.bashrc

# For zsh users  
echo 'export OPENAI_API_KEY=your_key_here' >> ~/.zshrc
source ~/.zshrc
```

### Getting an OpenAI API Key

1. Visit [OpenAI's website](https://openai.com)
2. Sign up for an account or log in
3. Navigate to the API section
4. Generate a new API key
5. Copy the key and use it in the export command above

**Security Note**: Keep your API key secure and never commit it to version control. The environment variable approach keeps your key out of your code and configuration files.

## Custom API Endpoints

Chat GipiTTY supports any OpenAI-compatible API provider. You can specify a custom API endpoint by setting the `OPENAI_BASE_URL` environment variable.

### Default Behavior

If not set, Chat GipiTTY uses the default OpenAI endpoint:
```
https://api.openai.com
```

### Provider Examples

#### Local Ollama Instance
```sh
export OPENAI_BASE_URL=http://localhost:11434/v1
```

#### Google Gemini (via OpenAI-compatible proxy)
```sh
export OPENAI_BASE_URL=https://generativelanguage.googleapis.com/v1beta
```

#### Mistral AI
```sh
export OPENAI_BASE_URL=https://api.mistral.ai/v1
```

#### Other OpenAI-compatible Services
```sh
export OPENAI_BASE_URL=https://your-provider.com/v1
```

#### Custom Endpoint Patterns
```sh
# If your provider uses a different endpoint pattern
export OPENAI_BASE_URL=https://custom-api.com/v2/chat/completions
```

### URL Construction Logic

Chat GipiTTY intelligently constructs the API endpoint:

- If your base URL already contains `/chat/completions`, it uses it as-is
- If your base URL ends with `/v1` (or similar version pattern), it appends `/chat/completions`
- Otherwise, it appends `/v1/chat/completions` (standard OpenAI pattern)

## Basic Configuration

### Default Model

Set your preferred default model:

```sh
cgip config --set model=gpt-4o
```

### View Current Configuration

Check your current settings:

```sh
cgip config --get model
```

### Available Configuration Options

Common configuration options include:

- `model`: Default AI model to use
- `system_prompt`: Default system prompt for conversations
- `max_tokens`: Maximum response length
- `temperature`: Response creativity (0.0 to 2.0)

## Session Management Setup

To enable session management across terminal sessions, set up a session name:

### Unique Session Per Terminal
```sh
export CGIP_SESSION_NAME=$(uuid)
```

### Daily Sessions (shared across terminals)
```sh
export CGIP_SESSION_NAME=$(date -I)
```

### Custom Session Management
```sh
# Custom session based on your needs
export CGIP_SESSION_NAME="my-project-session"
```

Add your chosen session configuration to your shell profile for persistence.

## Verification

Test your setup with a simple command:

```sh
echo "Hello, world!" | cgip "What can you tell me about this text?"
```

If everything is configured correctly, you should receive a response from the AI model.

### Common Issues

**"API key not found" error**: Make sure you've exported the `OPENAI_API_KEY` environment variable in your current shell session.

**"Model not found" error**: Check that your configured model is available with your API provider.

**Network errors**: Verify your `OPENAI_BASE_URL` is correct and accessible.

## Next Steps

Now that Chat GipiTTY is set up, you're ready to:

1. Learn [basic usage patterns](./basic-usage.md)
2. Explore [core features](./core-features.md) like sessions and web search
3. Try different [subcommands](./subcommands.md) for specialized tasks

## Advanced Configuration

For more detailed configuration options, see:

- [Environment Variables](./environment-variables.md) - Complete list of configuration options
- [Custom API Endpoints](./custom-api-endpoints.md) - Detailed provider setup guides
- [Configuration](./configuration.md) - Advanced configuration management