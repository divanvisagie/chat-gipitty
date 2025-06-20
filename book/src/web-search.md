# Web Search

Chat Gipitty supports web search functionality through the `/search` command prefix. When you start your message with `/search`, the application will enable web search capabilities to provide you with up-to-date information from the internet.

## How Web Search Works

The web search feature adapts based on your configured model:

- **For GPT models** (models starting with "gpt"): The application automatically switches to the `gpt-4o-search-preview` model and enables web search options for optimal search results.
- **For non-GPT models** (like Claude, Llama, or other custom models): The application keeps your configured model and adds web search options to the request.

## Basic Usage

To use web search, simply prefix your query with `/search`:

```bash
cgip "/search What are the latest developments in AI?"
```

The `/search` prefix will be automatically removed from your message before it's sent to the model, so you don't need to worry about it affecting the actual prompt.

## Usage Examples

### Current Events
```bash
# Search for recent news
cgip "/search What are the latest developments in renewable energy?"

# Get current market information
cgip "/search What is the current price of Bitcoin?"

# Find recent technology updates
cgip "/search What are the new features in the latest Python release?"
```

### Technical Information
```bash
# Search for current best practices
cgip "/search What are the current best practices for React performance optimization?"

# Find up-to-date documentation
cgip "/search How to configure Docker containers for production in 2024?"

# Get current software versions
cgip "/search What is the current stable version of Rust?"
```

### Research and Analysis
```bash
# Market research
cgip "/search What are the current trends in mobile app development?"

# Academic research
cgip "/search What are the latest findings on climate change mitigation?"

# Competitive analysis
cgip "/search What are the main competitors to OpenAI in the AI space?"
```

## Combining Web Search with Other Features

### Web Search with File Input
You can combine web search with file analysis:

```bash
# Search for context about your code
cgip "/search How can I optimize this code for performance?" -f my_script.py

# Get current information about technologies in your project
cgip "/search What are the latest security best practices for this framework?" -f package.json
```

### Web Search with Piped Input
Web search works with piped input as well:

```bash
# Search for solutions to error messages
command_that_fails 2>&1 | cgip "/search How to fix this error?"

# Get current information about command output
ps aux | cgip "/search What do these system processes indicate about performance?"
```

### Web Search in Sessions
Web search results become part of your session context:

```bash
# First query with search
cgip "/search What are the current JavaScript frameworks for 2024?"

# Follow-up question using search results
cgip "Which of these would be best for a small team project?"
```

## Model Behavior

### GPT Models
When using GPT models with web search:
- Automatically switches to `gpt-4o-search-preview`
- Provides real-time web search results
- Cites sources when possible
- Combines web information with the model's knowledge

### Non-GPT Models
When using other models (Claude, Llama, etc.):
- Keeps your configured model
- Adds web search capabilities to the request
- May have varying levels of web search integration depending on the provider

## Best Practices

### 1. Be Specific
More specific search queries yield better results:

```bash
# Good: Specific query
cgip "/search React 18 performance optimization techniques 2024"

# Less effective: Vague query
cgip "/search React performance"
```

### 2. Use Current Context
Include temporal context when relevant:

```bash
# Good: Includes timeframe
cgip "/search Current cybersecurity threats in 2024"

# Good: Includes version
cgip "/search Python 3.12 new features and changes"
```

### 3. Combine with Local Context
Use web search to enhance local analysis:

```bash
# Analyze local file with current best practices
cgip "/search Current Node.js security best practices" -f package.json
```

### 4. Follow-up Questions
Use session context to build on search results:

```bash
# Initial search
cgip "/search Latest trends in machine learning deployment"

# Follow-up without search (uses previous context)
cgip "Which of these trends would be most relevant for a startup?"
```

## Limitations and Considerations

### Rate Limits
- Web search may be subject to additional rate limits
- Consider the cost implications of web search requests

### Accuracy
- Always verify important information from multiple sources
- Web search results reflect current information but may not always be accurate

### Privacy
- Web search queries may be logged by the API provider
- Be mindful of sensitive information in search queries

### Model Compatibility
- Web search effectiveness varies by model and provider
- Some custom endpoints may not support web search features

## Troubleshooting

### Search Not Working
If web search doesn't seem to be working:

1. Check your model configuration:
   ```bash
   cgip config --get model
   ```

2. Verify your API endpoint supports web search:
   ```bash
   cgip config --get base_url
   ```

3. Try with a GPT model explicitly:
   ```bash
   cgip -M gpt-4o "/search test query"
   ```

### Limited Results
If you're getting limited search results:
- Try rephrasing your query
- Be more specific about what you're looking for
- Check if your API provider has web search enabled
