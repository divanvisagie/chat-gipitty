# Image Command

The `image` command allows you to analyze images using OpenAI's vision-capable models (like GPT-4o). This feature enables you to ask questions about images, extract text from images, describe visual content, and more.

## Overview

The image subcommand automatically ensures you're using a vision-capable model and will switch to `gpt-4o` if your current model doesn't support vision. It supports multiple image formats and provides flexible prompting options.

## Basic Usage

```sh
cgip image --file photo.jpg "What do you see in this image?"
```

## Command Help

```
Analyze an image using vision models. Use --file to specify the image path

Usage: cgip image [OPTIONS] --file <FILE> [PROMPT]

Arguments:
  [PROMPT]  Additional prompt text to include with the image analysis

Options:
  -f, --file <FILE>              Path to the image file to analyze
  -m, --max-tokens <MAX_TOKENS>  Maximum number of tokens in the response [default: 300]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Supported Image Formats

The image command supports the following formats:

- **JPEG** (.jpg, .jpeg)
- **PNG** (.png) 
- **GIF** (.gif)
- **WebP** (.webp)

## Basic Examples

### Simple Image Description

```sh
cgip image --file photo.jpg
# Uses default prompt: "What is in this image?"
```

### Custom Analysis Prompt

```sh
cgip image --file screenshot.png "Extract all text from this image"
```

### Detailed Analysis

```sh
cgip image --file diagram.jpg "Explain this diagram in detail" --max-tokens 500
```

## Common Use Cases

### Text Extraction (OCR)

Extract text from screenshots, documents, or signs:

```sh
cgip image --file receipt.jpg "Extract all text from this receipt and format it as a list"
```

```sh
cgip image --file whiteboard.png "What does the text on this whiteboard say?"
```

### Document Analysis

Analyze documents, receipts, forms, and other text-heavy images:

```sh
cgip image --file receipt.jpg "What items are on this receipt and what's the total?"
```

```sh
cgip image --file form.png "Help me fill out this form - what information is required?"
```

### Code Analysis

Analyze screenshots of code or development environments:

```sh
cgip image --file code_screenshot.png "What does this code do? Are there any potential issues?"
```

```sh
cgip image --file error_screen.png "What error is shown here and how can I fix it?"
```

### Visual Content Description

Describe photos, artwork, charts, and other visual content:

```sh
cgip image --file chart.png "Describe the data shown in this chart"
```

```sh
cgip image --file artwork.jpg "Describe the style and composition of this artwork"
```

### Technical Diagrams

Analyze technical diagrams, flowcharts, and system architectures:

```sh
cgip image --file architecture.png "Explain this system architecture diagram"
```

```sh
cgip image --file flowchart.jpg "Walk me through this process flowchart"
```

## Advanced Usage

### Combining with Other Commands

Process image analysis results with other tools:

```sh
# Save analysis to file
cgip image --file diagram.png "Explain this" --max-tokens 1000 > analysis.txt

# Convert analysis to speech
cgip image --file chart.jpg "Describe this data" | cgip tts --voice nova
```

### Multiple Images Workflow

While the command handles one image at a time, you can process multiple images:

```sh
# Process multiple screenshots
for img in screenshots/*.png; do
    echo "=== Analyzing $img ==="
    cgip image --file "$img" "What error or issue is shown here?"
done
```

### Context-Aware Analysis

Combine image analysis with additional context:

```sh
# Analyze error with additional context
cgip image --file error.png "Given that this is a React application, what's causing this error?"
```

## Response Length Control

### Default Token Limit

The default max-tokens is 300, suitable for brief descriptions:

```sh
cgip image --file photo.jpg "Briefly describe this image"
```

### Extended Analysis

For detailed analysis, increase the token limit:

```sh
cgip image --file complex_diagram.png "Provide a comprehensive analysis" --max-tokens 1000
```

### Concise Responses

For very brief responses, use focused prompts:

```sh
cgip image --file text.png "Extract only the main heading" --max-tokens 50
```

## Model Selection

### Automatic Model Selection

The image command automatically uses vision-capable models:

- If your configured model supports vision, it uses that model
- If your configured model doesn't support vision, it switches to `gpt-4o`

### Supported Vision Models

Currently supported vision-capable models include:
- `gpt-4o`
- `gpt-4-vision-preview`
- `gpt-4-turbo`
- `gpt-4`

### Checking Your Model

Verify your current model configuration:

```sh
cgip config --get model
```

Set a vision-capable model as default:

```sh
cgip config --set model=gpt-4o
```

## Image Processing Details

### Automatic Processing

Images are automatically:
1. Read from the file system
2. Encoded as base64
3. Sent to the API with appropriate MIME type based on file extension
4. Combined with your text prompt for analysis

### File Size Considerations

- Large images may take longer to process
- Very large images might hit API limits
- Consider resizing extremely large images for better performance

### Quality Recommendations

- Higher resolution images generally produce better analysis
- Ensure text in images is clearly readable
- Good lighting and contrast improve results

## Error Handling

### Common Issues and Solutions

**File not found:**
```sh
cgip image --file nonexistent.jpg "analyze this"
# Error: No such file or directory
```
Check the file path and ensure the file exists.

**Unsupported format:**
```sh
cgip image --file document.pdf "analyze this"
# May not work - PDF is not a supported image format
```
Convert PDFs to images first or use supported formats.

**Network errors:**
Check your API key and network connection.

**Model not available:**
Ensure you have access to vision-capable models in your API account.

## Best Practices

### Prompt Engineering

**Be specific about what you want:**
```sh
# Good
cgip image --file chart.png "What are the main trends shown in this sales data chart?"

# Less effective
cgip image --file chart.png "What is this?"
```

**Ask for structured output when needed:**
```sh
cgip image --file receipt.png "Extract the items and prices as a formatted list"
```

### Image Quality

- Use clear, well-lit images
- Ensure text is readable
- Avoid blurry or low-contrast images

### Token Management

- Use appropriate max-tokens for your needs
- Brief descriptions: 100-300 tokens
- Detailed analysis: 500-1000 tokens
- Comprehensive reports: 1000+ tokens

### Cost Considerations

Vision requests consume more tokens than text-only requests:
- Monitor your usage
- Use appropriate token limits
- Consider image size and complexity

## Troubleshooting

### Image Not Loading

Check file permissions and path:
```sh
ls -la path/to/image.jpg
```

### Poor Analysis Quality

- Try a more specific prompt
- Ensure image quality is good
- Increase max-tokens for more detailed responses

### Model Errors

If you get model-related errors:
```sh
# Check current model
cgip config --get model

# Set to a known vision model
cgip config --set model=gpt-4o
```

## Integration Examples

### Development Workflow

```sh
# Analyze UI mockups
cgip image --file mockup.png "What UI elements are shown and how should they be implemented?"

# Debug visual issues
cgip image --file bug_screenshot.png "What's wrong with this UI layout?"
```

### Documentation

```sh
# Generate image descriptions for documentation
cgip image --file feature_screenshot.png "Write a caption for this screenshot" --max-tokens 100
```

### Data Analysis

```sh
# Analyze charts and graphs
cgip image --file sales_chart.png "What insights can you derive from this sales data?"
```

## Next Steps

- Learn about [TTS command](./tts-command.md) to convert image analysis to speech
- Explore [agent command](./agent-command.md) for automated workflows involving images
- Try [combining with other features](./examples.md) for powerful multi-modal workflows