# Image Subcommand Documentation

The `image` subcommand allows you to analyze images using OpenAI's vision-capable models (like GPT-4o). This feature enables you to ask questions about images, extract text from images, describe visual content, and more.

## Basic Usage

```bash
cgip image --file /path/to/image.jpg "What do you see in this image?"
```

## Options

- `--file <FILE>` (required): Path to the image file to analyze
- `[PROMPT]`: Additional prompt text to include with the image analysis (optional)
- `--max-tokens <MAX_TOKENS>`: Maximum number of tokens in the response (default: 300)

## Supported Image Formats

- JPEG (.jpg, .jpeg)
- PNG (.png)
- GIF (.gif)
- WebP (.webp)

## Examples

### Basic image description
```bash
cgip image --file photo.jpg
# Uses default prompt: "What is in this image?"
```

### Custom analysis prompt
```bash
cgip image --file screenshot.png "Extract all text from this image"
```

### Detailed analysis with more tokens
```bash
cgip image --file diagram.jpg "Explain this diagram in detail" --max-tokens 500
```

### Document analysis
```bash
cgip image --file receipt.jpg "What items are on this receipt and what's the total?"
```

### Code analysis
```bash
cgip image --file code_screenshot.png "What does this code do? Are there any potential issues?"
```

## Model Selection

The image subcommand automatically ensures you're using a vision-capable model:

- If your configured model doesn't support vision (like `gpt-3.5-turbo`), it will automatically use `gpt-4o`
- Supported vision models include: `gpt-4o`, `gpt-4-vision-preview`, `gpt-4-turbo`, and `gpt-4`

You can check your current model with:
```bash
cgip config --get model
```

And set it to a vision-capable model:
```bash
cgip config --set model=gpt-4o
```

## Image Processing

Images are automatically:
1. Read from the file system
2. Encoded as base64
3. Sent to the API with the appropriate MIME type based on file extension
4. Combined with your text prompt for analysis

## Error Handling

- If the image file doesn't exist, you'll get a clear error message
- If the image format is unsupported, it defaults to JPEG MIME type
- Network errors and API errors are handled gracefully with descriptive messages

## Environment Requirements

Make sure you have:
- `OPENAI_API_KEY` environment variable set
- Access to vision-capable models in your OpenAI account
- Sufficient API credits (vision requests consume more tokens than text-only)

## Tips

1. **Be specific**: More detailed prompts often yield better results
2. **Image quality**: Higher quality images generally produce better analysis
3. **File size**: Very large images may take longer to process
4. **Context**: The image subcommand works with session context, so previous conversation history is included