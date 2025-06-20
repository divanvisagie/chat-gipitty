# TTS Command

The `tts` command allows you to convert text to speech using OpenAI's Text-to-Speech models. This feature enables you to generate high-quality audio from text input with various voice options and customization settings.

## Overview

The TTS subcommand reads text from command arguments, stdin, or both. When both are provided, it combines the stdin text with the argument text (stdin first, then argument), making it easy to pipe content from other commands and add additional text.

## Basic Usage

```sh
cgip tts "Hello, this is a test of text-to-speech functionality"
```

## Command Help

```
Convert text to speech using OpenAI's TTS models

Usage: cgip tts [OPTIONS] [TEXT]

Arguments:
  [TEXT]  Text to convert to speech. If not provided, reads from stdin

Options:
  -m, --model <MODEL>                TTS model to use [default: tts-1]
  -v, --voice <VOICE>                Voice to use for speech synthesis [default: alloy]
  -o, --output <OUTPUT>              Output file path for the audio [default: speech.mp3]
  -i, --instructions <INSTRUCTIONS>  Instructions for the voice (how to speak)
  -f, --format <FORMAT>              Audio format (mp3, opus, aac, flac) [default: mp3]
  -s, --speed <SPEED>                Speed of speech (0.25 to 4.0) [default: 1.0]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Available Options

### Models
- **tts-1**: Standard quality, faster generation
- **tts-1-hd**: High definition quality, slower generation

### Voices
- **alloy**: Neutral, balanced voice
- **echo**: Male voice with clear pronunciation
- **fable**: British accent, expressive
- **onyx**: Deep, authoritative voice
- **nova**: Young, energetic voice
- **shimmer**: Soft, gentle voice

### Audio Formats
- **MP3** (.mp3): Most compatible, good compression
- **OPUS** (.opus): Excellent compression, modern codec
- **AAC** (.aac): Good quality and compression, widely supported
- **FLAC** (.flac): Lossless compression, larger file size

### Speed Settings
- `0.25`: Very slow, good for learning or accessibility
- `0.5`: Slow, clear pronunciation
- `1.0`: Normal speed (default)
- `1.5`: Slightly faster, efficient listening
- `2.0`: Fast, good for familiar content
- `4.0`: Very fast, maximum speed

## Basic Examples

### Simple Text to Speech
```sh
cgip tts "Welcome to our application!"
```

### Custom Voice and Output
```sh
cgip tts --voice nova --output welcome.mp3 "Welcome to our application!"
```

### High-Definition Model
```sh
cgip tts --model tts-1-hd --speed 0.8 --voice shimmer "This is spoken slowly and clearly"
```

## Input Handling

The TTS command handles text input in the following priority:

1. **Text argument only**: Uses the provided text argument
2. **Stdin only**: Uses text from stdin (when no text argument is provided)
3. **Both stdin and text argument**: Combines stdin text with argument text (stdin first, then argument)
4. **Neither**: Shows error message

### Reading from Stdin
```sh
echo "This text comes from stdin" | cgip tts --voice echo --output stdin_speech.mp3
```

### Combining Stdin and Argument
```sh
# Combines both inputs
hostname | cgip tts "and that's all she wrote" --output combined.mp3
# Result: "Hello from hostname" + "and that's all she wrote"
```

### Reading from Files
```sh
cat announcement.txt | cgip tts --voice fable --output announcement.mp3
```

## Voice Characteristics

### alloy
- Neutral and versatile
- Works well for most content types
- Balanced tone and pace

### echo  
- Clear male voice
- Good for professional content
- Authoritative but approachable

### fable
- British accent
- Expressive and engaging
- Good for storytelling

### onyx
- Deep male voice
- Authoritative and confident
- Professional presentations

### nova
- Young and energetic
- Good for casual content
- Upbeat and friendly

### shimmer
- Soft and gentle
- Good for calming content
- Soothing and peaceful

## Audio Format Details

### MP3
- Most widely supported format
- Good balance of quality and file size
- Compatible with virtually all devices

### OPUS
- Modern, efficient codec
- Excellent compression
- Smaller file sizes than MP3
- Good for web applications

### AAC
- High quality audio
- Good compression
- Widely supported by modern devices
- Apple ecosystem preferred format

### FLAC
- Lossless compression
- Highest audio quality
- Larger file sizes
- Good for archival purposes

## Advanced Usage

### Custom Instructions
```sh
cgip tts --instructions "Speak in a cheerful and enthusiastic tone" "Today is a wonderful day!"
```

### Speed Adjustments
```sh
# Slow for learning
cgip tts --speed 0.5 --voice onyx "Technical explanation spoken slowly"

# Fast for quick review
cgip tts --speed 1.5 --voice nova "Quick summary of key points"

# Very fast for familiar content
cgip tts --speed 2.0 "Content you want to review quickly"
```

### Different Formats for Different Uses
```sh
# High quality for archival
cgip tts --format flac --output archive.flac "Important announcement"

# Compressed for web
cgip tts --format opus --output web.opus "Website audio content"

# Compatible for sharing
cgip tts --format mp3 --output share.mp3 "Content to share widely"
```

## Common Workflows

### Documentation to Audio
```sh
# Convert README to audio
cat README.md | cgip tts --voice fable --output readme_audio.mp3
```

### Code Comments to Audio
```sh
# Extract and vocalize code comments
grep -n "#" script.py | cgip tts --voice echo --output code_comments.mp3
```

### Email or Message Reading
```sh
# Read important messages aloud
cat important_email.txt | cgip tts --voice alloy --speed 0.8
```

### Multilingual Content
```sh
# TTS works with multiple languages
cgip tts --voice nova "Bonjour, comment allez-vous?"
```

### Batch Processing
```sh
# Process multiple text files
for file in *.txt; do
    cgip tts --voice alloy --output "${file%.txt}.mp3" < "$file"
done
```

## Integration with Other Commands

### Combining with Chat GipiTTY
```sh
# Generate text and convert to speech
cgip "Write a short story about a robot" | cgip tts --voice fable --output story.mp3

# Analyze code and speak the explanation
cgip "explain this function" -f script.py | cgip tts --voice echo
```

### Pipeline Processing
```sh
# Process data and create audio summary
cat data.csv | cgip "summarize this data" | cgip tts --voice nova --output summary.mp3
```

### System Integration
```sh
# Create audio notifications
echo "Build completed successfully" | cgip tts --voice alloy --output notification.mp3
play notification.mp3
```

## Error Handling

### Common Issues

**No text provided:**
```
Error: No text provided via argument or stdin
```
Solution: Provide text via argument or pipe it through stdin.

**Invalid speed value:**
```
Error: Speed must be between 0.25 and 4.0
```
Solution: Use a speed value within the valid range.

**Invalid audio format:**
```
Error: Unsupported format. Use: mp3, opus, aac, flac
```
Solution: Use one of the supported audio formats.

**API errors:**
Check your API key and network connection.

## Best Practices

### Choose Appropriate Voice
- **alloy**: General purpose, neutral content
- **echo**: Professional, technical content
- **fable**: Engaging, storytelling content
- **onyx**: Authoritative, formal content
- **nova**: Casual, energetic content
- **shimmer**: Calming, gentle content

### Optimize Speed
- Use slower speeds (0.5-0.8) for complex or technical content
- Use normal speed (1.0) for general content
- Use faster speeds (1.2-2.0) for familiar or review content

### Select Appropriate Format
- **MP3**: General use, sharing, compatibility
- **OPUS**: Web applications, smaller files
- **AAC**: Apple devices, streaming
- **FLAC**: Archival, highest quality

### Use Instructions Effectively
```sh
# For technical content
cgip tts --instructions "Speak clearly and pause at technical terms"

# For emotional content
cgip tts --instructions "Speak with appropriate emotion and emphasis"

# For educational content
cgip tts --instructions "Speak slowly and emphasize key points"
```

## Cost Considerations

- TTS is charged per character of input text
- HD models (tts-1-hd) cost more than standard models (tts-1)
- Monitor usage to manage costs
- Consider caching frequently used audio files

## Troubleshooting

### Audio Quality Issues
- Try the HD model: `--model tts-1-hd`
- Adjust speed for clarity: `--speed 0.8`
- Use appropriate voice for content type

### File Output Issues
- Check write permissions in output directory
- Ensure output path is valid
- Verify disk space availability

### Network Issues
- Check internet connection
- Verify API key is set correctly
- Check API rate limits

## Environment Requirements

Make sure you have:
- `OPENAI_API_KEY` environment variable set
- Access to OpenAI's TTS models in your account
- Sufficient API credits (TTS requests are charged per character)

## Custom API Endpoints

You can use custom API endpoints:
```sh
export OPENAI_BASE_URL=https://your-custom-api.com
cgip tts "Hello world"
```

## Next Steps

- Learn about [embedding command](./embedding-command.md) for text analysis
- Explore [agent command](./agent-command.md) for automated workflows
- Try [combining with image analysis](./image-command.md) for multi-modal content