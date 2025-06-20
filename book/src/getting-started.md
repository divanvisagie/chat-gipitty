# Getting Started

Welcome to Chat GipiTTY! This guide will help you get up and running quickly with cgip, from installation to your first AI-powered terminal interactions.

## What You'll Learn

In this section, you'll learn how to:

- Install Chat GipiTTY on your system
- Set up your API credentials
- Configure custom API endpoints (optional)
- Run your first commands
- Understand the basic workflow

## Prerequisites

Before you begin, make sure you have:

- A POSIX-compliant system (Linux, macOS, or Windows with WSL)
- An OpenAI API key (or credentials for another supported provider)
- Rust and Cargo installed (recommended installation method)

## Quick Start

If you're eager to get started, here's the fastest path:

1. **Install via Cargo** (recommended):
   ```sh
   cargo install cgip
   ```

2. **Set your API key**:
   ```sh
   export OPENAI_API_KEY=your_key_here
   ```

3. **Test it out**:
   ```sh
   echo "Hello, AI!" | cgip "What can you tell me about this greeting?"
   ```

## Installation Methods

Chat GipiTTY offers several installation options to suit different preferences and environments. The recommended method is via Cargo for the latest version and best compatibility.

### Recommended: Cargo Installation
The most reliable way to install and keep Chat GipiTTY updated.

### Alternative Methods
Other installation options including manual builds and package managers.

## Next Steps

Once you have Chat GipiTTY installed:

1. **[Set up your environment](./setup.md)** - Configure API keys and optional settings
2. **[Learn basic usage](./basic-usage.md)** - Master the fundamental commands and patterns
3. **[Explore features](./core-features.md)** - Discover advanced capabilities like sessions and web search

## Getting Help

If you run into issues during installation or setup:

- Check the [Troubleshooting](./troubleshooting.md) section
- Run `cgip --help` for command-line assistance
- Visit the project repository for community support

Ready to begin? Let's start with [Installation](./installation.md).