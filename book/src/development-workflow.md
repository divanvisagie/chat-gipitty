# Development Workflow

This section covers development setup and workflow for Chat GipiTTY contributors and advanced users who want to build from source.

## Development Setup

### Prerequisites

To build Chat GipiTTY from source, you'll need:

- [Rust](https://rustup.rs/) (latest stable version)
- Git
- A POSIX-compliant system (Linux, macOS, or WSL on Windows)

### Clone and Build

```sh
git clone https://github.com/divanvisagie/chat-gipitty.git
cd chat-gipitty
cargo build
```

### Running from Source

You can run Chat GipiTTY directly from the source directory:

```sh
cargo run -- --help
```

### Running Tests

```sh
cargo test
```

## Ubuntu Development Setup

On Ubuntu, some additional packages are required to build the deb package:

```sh
sudo apt-get install build-essential dh-make debhelper devscripts
```

## Development Tools

### Building Documentation

The documentation is built using mdBook:

```sh
# Install mdbook if not already installed
cargo install mdbook

# Build the book
cd book
mdbook build

# Serve locally for development
mdbook serve
```

### Creating Releases

For maintainers, see the release process documentation for preparing new releases.

### Package Building

#### Debian Package

```sh
# Build deb package (Ubuntu/Debian)
./scripts/build_deb.sh
```

#### Homebrew Formula

```sh
# Update homebrew formula
./scripts/homebrew.sh
```

## Project Structure

```
chat-gipitty/
├── src/                    # Main Rust source code
│   ├── main.rs            # Application entry point
│   ├── args.rs            # Command line argument parsing
│   ├── chat.rs            # Core chat functionality
│   ├── chatgpt/           # OpenAI API client
│   ├── sub/               # Subcommands
│   └── ...
├── book/                   # Documentation source
├── docs/                   # Generated documentation
├── assets/                 # Logo and assets
├── scripts/               # Build and release scripts
└── test_data/             # Test files
```

## Contributing Guidelines

1. **Fork the repository** on GitHub
2. **Create a feature branch** from main
3. **Make your changes** with appropriate tests
4. **Run the test suite** to ensure nothing breaks
5. **Submit a pull request** with a clear description

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Run clippy for linting (`cargo clippy`)
- Add tests for new functionality
- Update documentation as needed

### Testing Your Changes

Before submitting a pull request:

```sh
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Test the binary
cargo run -- --help
```

## Debugging

### Environment Variables

Useful environment variables for development:

```sh
# Enable debug logging
export RUST_LOG=debug

# Show API requests (be careful with sensitive data)
export CGIP_DEBUG_API=1
```

### Common Development Tasks

#### Adding a New Subcommand

1. Create a new file in `src/sub/`
2. Implement the subcommand logic
3. Add the subcommand to `src/sub/mod.rs`
4. Update argument parsing in `src/args.rs`
5. Add documentation in `book/src/`

#### Testing API Changes

```sh
# Test with different models
cargo run -- -M gpt-3.5-turbo "test prompt"

# Test with custom endpoint
export OPENAI_BASE_URL=http://localhost:11434/v1
cargo run -- "test prompt"
```

## Release Process

Maintainers should follow these steps for releases:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create and push version tag
4. GitHub Actions will handle the rest

For detailed release instructions, see the project's internal documentation.
