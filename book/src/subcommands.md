# Subcommands

Chat GipiTTY provides several specialized subcommands for different types of AI-powered tasks. Each subcommand is optimized for specific use cases, from image analysis to text-to-speech generation.

## Overview of Available Subcommands

| Subcommand | Purpose | Key Features |
|------------|---------|--------------|
| [view](./view-command.md) | Context inspection | View context without API calls |
| [config](./config-command.md) | Configuration management | Get/set configuration values |
| [session](./session-command.md) | Session management | View and clear conversation history |
| [image](./image-command.md) | Image analysis | Analyze images with vision models |
| [tts](./tts-command.md) | Text-to-speech | Convert text to high-quality audio |
| [embedding](./embedding-command.md) | Text embeddings | Generate vector representations |
| [agent](./agent-command.md) | Autonomous execution | Let AI execute shell commands |
| [upgrade](./upgrade-command.md) | Software updates | Upgrade to latest version |

## Subcommand Categories

### Core Utilities
- **view**: Debug and inspect context before sending to AI
- **config**: Manage your Chat GipiTTY configuration
- **session**: Control conversation context and history

### AI Capabilities
- **image**: Multi-modal image understanding and analysis
- **tts**: High-quality voice synthesis
- **embedding**: Vector generation for semantic search
- **agent**: Autonomous task execution with tool calling

### Maintenance
- **upgrade**: Keep Chat GipiTTY up to date

## Common Usage Patterns

### Inspection and Debugging
Use `view` to understand what context will be sent to the AI:
```sh
echo "test data" | cgip view
```

### Configuration Management
Set up your preferred defaults:
```sh
cgip config --set model=gpt-4o
cgip config --set system_prompt="Be concise and technical"
```

### Multi-Modal Tasks
Analyze images alongside text:
```sh
cgip image --file screenshot.png "What errors do you see in this code?"
```

### Voice Output
Convert AI responses to speech:
```sh
cgip "explain quantum computing" | cgip tts --voice nova --output explanation.mp3
```

### Autonomous Workflows
Let the AI execute commands to complete tasks:
```sh
cgip agent ~/project "run tests and fix any failing ones"
```

## Subcommand Help

Each subcommand has its own help system:

```sh
cgip <subcommand> --help
```

For example:
```sh
cgip image --help
cgip tts --help
cgip agent --help
```

## Combining Subcommands

While subcommands are typically used independently, you can combine them in powerful ways:

### Image Analysis to Speech
```sh
cgip image --file chart.png "describe this data" | cgip tts --voice alloy
```

### Code Analysis to Documentation
```sh
cgip "document this function" -f code.py > docs.md
cgip tts --voice fable < docs.md
```

### Session Management with Analysis
```sh
# Start a session, analyze, then view history
cgip "analyze this error" -f error.log
cgip session --view
```

## Next Steps

Explore each subcommand in detail:

- Start with [view](./view-command.md) to understand context inspection
- Learn [config](./config-command.md) for personalized setup
- Try [image](./image-command.md) for visual AI capabilities
- Experiment with [tts](./tts-command.md) for voice synthesis
- Discover [agent](./agent-command.md) for autonomous workflows

Each subcommand page includes detailed examples, options, and best practices for that specific functionality.