# TTS Subcommand Documentation

The `tts` subcommand allows you to convert text to speech using OpenAI's Text-to-Speech models. This feature enables you to generate high-quality audio from text input with various voice options and customization settings.

## Basic Usage

```bash
cgip tts "Hello, this is a test of text-to-speech functionality"
```

## Options

- `[TEXT]`: Text to convert to speech (optional, reads from stdin if not provided, or combines with stdin if both are present)
- `--model <MODEL>`: TTS model to use (default: tts-1)
- `--voice <VOICE>`: Voice to use for speech synthesis (default: alloy)
- `--output <OUTPUT>`: Output file path for the audio (default: speech.mp3)
- `--instructions <INSTRUCTIONS>`: Instructions for the voice (how to speak)
- `--format <FORMAT>`: Audio format - mp3, opus, aac, flac (default: mp3)
- `--speed <SPEED>`: Speed of speech from 0.25 to 4.0 (default: 1.0)

## Available Models

- `tts-1`: Standard quality, faster generation
- `tts-1-hd`: High definition quality, slower generation

## Available Voices

- `alloy`: Neutral, balanced voice
- `echo`: Male voice with clear pronunciation
- `fable`: British accent, expressive
- `onyx`: Deep, authoritative voice
- `nova`: Young, energetic voice
- `shimmer`: Soft, gentle voice

## Examples

### Basic text-to-speech
```bash
cgip tts "Welcome to our application!"
```

### Using different voice and output file
```bash
cgip tts --voice nova --output welcome.mp3 "Welcome to our application!"
```

### High-definition model with custom speed
```bash
cgip tts --model tts-1-hd --speed 0.8 --voice shimmer "This is spoken slowly and clearly"
```

### Reading from stdin
```bash
echo "This text comes from stdin" | cgip tts --voice echo --output stdin_speech.mp3
```

### Combining stdin and argument text
```bash
# Combines stdin and argument: "Hello from hostname" + "and that's all she wrote"
hostname | cgip tts "and that's all she wrote" --output combined.mp3
```

### Custom instructions for tone
```bash
cgip tts --instructions "Speak in a cheerful and enthusiastic tone" "Today is a wonderful day!"
```

### Different audio formats
```bash
# Generate FLAC format for high quality
cgip tts --format flac --output speech.flac "High quality audio"

# Generate OPUS format for smaller file size
cgip tts --format opus --output speech.opus "Compressed audio"
```

### Reading from file content
```bash
cat announcement.txt | cgip tts --voice fable --output announcement.mp3
```

### Fast speech for quick playback
```bash
cgip tts --speed 1.5 --voice onyx "This will be spoken quickly"
```

## Audio Formats

- **MP3**: Most compatible, good compression
- **OPUS**: Excellent compression, modern codec
- **AAC**: Good quality and compression, widely supported
- **FLAC**: Lossless compression, larger file size

## Voice Characteristics

- **alloy**: Neutral and versatile, works well for most content
- **echo**: Clear male voice, good for professional content
- **fable**: British accent, expressive and engaging
- **onyx**: Deep male voice, authoritative and confident
- **nova**: Young and energetic, good for casual content
- **shimmer**: Soft and gentle, good for calming content

## Speed Settings

- `0.25`: Very slow, good for learning or accessibility
- `0.5`: Slow, clear pronunciation
- `1.0`: Normal speed (default)
- `1.5`: Slightly faster, efficient listening
- `2.0`: Fast, good for familiar content
- `4.0`: Very fast, maximum speed

## Environment Requirements

Make sure you have:
- `OPENAI_API_KEY` environment variable set
- Access to OpenAI's TTS models in your account
- Sufficient API credits (TTS requests are charged per character)

## Custom API Endpoint

You can use a custom API endpoint by setting the `OPENAI_BASE_URL` environment variable:

```bash
export OPENAI_BASE_URL=https://your-custom-api.com
cgip tts "Hello world"
```

## Input Handling

The TTS subcommand handles text input in the following priority:

1. **Text argument only**: Uses the provided text argument
2. **Stdin only**: Uses text from stdin (when no text argument is provided)
3. **Both stdin and text argument**: Combines stdin text with argument text (stdin first, then argument)
4. **Neither**: Shows error message

Examples:
```bash
# Uses only argument text
cgip tts "Hello world"

# Uses only stdin text  
echo "Hello world" | cgip tts

# Combines both (stdin + argument)
echo "Hello from" | cgip tts "the terminal"
# Result: "Hello from the terminal"
```

## Error Handling

- If no text is provided via argument or stdin, you'll get a clear error message
- Invalid speed values (outside 0.25-4.0 range) will be rejected
- Invalid audio formats will show available options
- Network errors and API errors are handled gracefully with descriptive messages

## Tips

1. **Choose the right voice**: Different voices work better for different content types
2. **Use instructions**: Add instructions to get the desired tone and emotion
3. **Consider format**: Use FLAC for archival quality, MP3 for general use, OPUS for web
4. **Adjust speed**: Slower speeds improve comprehension, faster speeds save time
5. **Batch processing**: Pipe multiple texts through for batch conversion

## Pricing Considerations

- TTS is charged per character of input text
- HD models cost more than standard models
- Monitor your usage to manage costs effectively
- Consider caching frequently used audio files