# Custom API Endpoints

Chat GipiTTY is designed to work with any OpenAI Chat Completions API-compatible provider. You can specify a custom API endpoint by setting the `OPENAI_BASE_URL` environment variable. If not set, the default is `https://api.openai.com`.

## URL Construction

Chat GipiTTY intelligently constructs the API endpoint:
- If your base URL already contains `/chat/completions`, it uses it as-is
- If your base URL ends with `/v1` (or similar version pattern), it appends `/chat/completions`
- Otherwise, it appends `/v1/chat/completions` (standard OpenAI pattern)

## Provider Examples

### Local Ollama Instance
```sh
export OPENAI_BASE_URL=http://localhost:11434/v1
```

### Google Gemini (via OpenAI-compatible proxy)
```sh
export OPENAI_BASE_URL=https://generativelanguage.googleapis.com/v1beta
```

### Mistral AI (via OpenAI-compatible endpoint)
```sh
export OPENAI_BASE_URL=https://api.mistral.ai/v1
```

### Anthropic Claude (via OpenAI-compatible endpoint)
```sh
export OPENAI_BASE_URL=https://api.anthropic.com/v1
```

### Other OpenAI-compatible Services
```sh
export OPENAI_BASE_URL=https://your-provider.com/v1
```

### Custom Endpoint Patterns
If your provider uses a different endpoint pattern, you can specify the full URL:
```sh
export OPENAI_BASE_URL=https://custom-api.com/v2/chat/completions
```

## Supported Providers

Chat GipiTTY works with any service that implements the OpenAI Chat Completions API standard:

- **OpenAI** (ChatGPT, GPT-4, GPT-3.5, etc.)
- **Local models** via [Ollama](https://ollama.com)
- **Google Gemini** (via OpenAI-compatible endpoints)
- **Mistral AI** (via OpenAI-compatible endpoints)
- **Anthropic Claude** (via OpenAI-compatible endpoints)
- **Any other provider** implementing the OpenAI Chat Completions API standard

## Compatibility Notes

Custom `OPENAI_BASE_URL` values can point to these or other OpenAI-compatible endpoints, but such providers might not implement the complete API and compatibility cannot be guaranteed.

As long as your provider implements the OpenAI Chat Completions API standard, Chat GipiTTY will work with it seamlessly.

## Authentication

Most providers will still require you to set an API key:

```sh
export OPENAI_API_KEY=your_provider_api_key_here
```

Some providers may use different authentication methods or environment variable names. Consult your provider's documentation for specific authentication requirements.
