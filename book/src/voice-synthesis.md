# Voice Synthesis

Chat GipiTTY supports text-to-speech (TTS) conversion using OpenAI's TTS models. You can convert text to high-quality audio with various voice options and customization settings.

For detailed information about the TTS feature, see the [TTS Command](./tts-command.md) documentation.

## Quick Overview

The `tts` subcommand allows you to:

- Convert text to speech with natural-sounding voices
- Choose from multiple voice options (alloy, echo, fable, onyx, nova, shimmer)
- Control speech speed and audio quality
- Support multiple audio formats (MP3, OPUS, AAC, FLAC)
- Process text from command arguments, files, or stdin

## Basic Usage

```sh
# Simple text-to-speech
cgip tts "Hello, this is a test of text-to-speech functionality"

# Use a different voice
cgip tts --voice nova "Welcome to our application!"

# Read from stdin
echo "This text comes from stdin" | cgip tts --output speech.mp3
```

## Key Features

- **High-Quality Voices**: Choose from 6 distinct AI voices
- **Flexible Input**: Accept text from arguments, stdin, or both
- **Audio Formats**: Support for MP3, OPUS, AAC, and FLAC
- **Speed Control**: Adjust speech rate from 0.25x to 4.0x
- **Quality Options**: Standard (`tts-1`) and HD (`tts-1-hd`) models

## Common Use Cases

### Accessibility
```sh
# Read documentation aloud
cgip tts --voice echo -f README.md --output readme_audio.mp3

# Convert articles to audio
curl -s https://example.com/article.txt | cgip tts --voice nova
```

### Content Creation
```sh
# Create audio announcements
cgip tts --voice shimmer "This is an important announcement" --output announcement.mp3

# Generate audio for presentations
cgip tts --voice onyx -f presentation_script.txt --output presentation.mp3
```

### Development Workflow
```sh
# Audio alerts for long-running commands
make build && cgip tts --voice alloy "Build completed successfully" || cgip tts --voice echo "Build failed"
```

For complete documentation including all options, voice samples, and advanced usage patterns, see the [full TTS command documentation](./tts-command.md).
