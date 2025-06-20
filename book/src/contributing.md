# Contributing

Thank you for your interest in contributing to Chat Gipitty! This guide covers how to contribute to the project, from reporting bugs to submitting code changes.

## Ways to Contribute

### Reporting Issues
- **Bug Reports**: Found a bug? Please report it with detailed steps to reproduce
- **Feature Requests**: Have an idea for a new feature? Let us know!
- **Documentation**: Help improve the documentation with corrections or additions
- **Performance Issues**: Report performance problems or suggest optimizations

### Code Contributions
- **Bug Fixes**: Fix reported issues
- **New Features**: Implement requested features
- **Performance Improvements**: Optimize existing code
- **Tests**: Add or improve test coverage
- **Documentation**: Update code comments and documentation

## Getting Started

### Prerequisites
- **Rust**: Install the latest stable version from [rustup.rs](https://rustup.rs/)
- **Git**: For version control and submitting changes
- **Code Editor**: Any editor with Rust support (VSCode, IntelliJ, etc.)

### Development Setup
```bash
# Clone the repository
git clone https://github.com/divanvisagie/chat-gipitty.git
cd chat-gipitty

# Build the project
cargo build

# Run tests
cargo test

# Run the development version
cargo run -- --help
```

### Ubuntu Development Setup
On Ubuntu, additional packages are required for building distribution packages:

```bash
sudo apt-get install build-essential dh-make debhelper devscripts
```

## Development Workflow

### 1. Fork and Clone
1. Fork the repository on GitHub
2. Clone your fork locally
3. Add the upstream repository as a remote

```bash
git clone https://github.com/your-username/chat-gipitty.git
cd chat-gipitty
git remote add upstream https://github.com/divanvisagie/chat-gipitty.git
```

### 2. Create a Branch
Create a new branch for your work:

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number
```

### 3. Make Changes
- Write clear, well-documented code
- Follow existing code style and conventions
- Add tests for new functionality
- Update documentation as needed

### 4. Test Your Changes
```bash
# Run the full test suite
cargo test

# Test your changes manually
cargo run -- "test query"

# Check formatting
cargo fmt --check

# Run clippy for linting
cargo clippy
```

### 5. Commit and Push
```bash
# Stage your changes
git add .

# Commit with a clear message
git commit -m "Add feature: description of your changes"

# Push to your fork
git push origin feature/your-feature-name
```

### 6. Submit a Pull Request
1. Go to the GitHub repository
2. Click "New Pull Request"
3. Select your branch
4. Fill out the pull request template
5. Submit for review

## Code Style and Standards

### Rust Code Style
- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` to catch common issues
- Write clear, descriptive variable and function names
- Add documentation comments for public APIs

### Commit Messages
Use clear, descriptive commit messages:
```
Add feature: web search integration

- Implement /search command prefix
- Add automatic model switching for GPT models
- Update documentation with search examples
```

### Documentation
- Update relevant documentation for new features
- Add examples for new functionality
- Keep the README up to date
- Add or update man page content if needed

## Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with verbose output
cargo test -- --nocapture
```

### Adding Tests
- Add unit tests for new functions
- Add integration tests for new features
- Test error conditions and edge cases
- Ensure tests are deterministic and isolated

### Manual Testing
Test your changes with real scenarios:
```bash
# Test basic functionality
cargo run -- "test query"

# Test with files
cargo run -- -f test_file.txt "analyze this"

# Test subcommands
cargo run -- image --help
cargo run -- tts "test speech"
```

## Release Process

The project uses Git tags and GitHub Actions for releases. Here's how releases are prepared:

### 1. Update Version Numbers
- Edit `Cargo.toml` and set the new `version`
- Update the version in `assets/cgip.1` and `scripts/build_deb.sh`
- Run `cargo build --release` to update `Cargo.lock`

### 2. Write Release Notes
- Add a markdown file under `notes/` named after the version (e.g., `notes/0.5.0.md`)
- Summarize new features, fixes, and maintenance items
- Include any breaking changes or migration notes

### 3. Commit Changes
```bash
git add .
git commit -m "Bump version to 0.5.0"
```

### 4. Tag the Release
```bash
# Verify the tag first
make tag-release-dry

# Create and push the tag (triggers GitHub Actions)
make tag-release
```

### 5. Publish Packages (Optional)
```bash
# Build and upload artifacts to GitHub release
make tarball-publish
# or
make deb-publish
```

This process ensures version consistency and triggers automated release builds.

## Project Structure

```
chat-gipitty/
├── src/                 # Source code
│   ├── main.rs         # Entry point
│   ├── args.rs         # Command line argument parsing
│   ├── chat.rs         # Main chat functionality
│   ├── config_manager.rs # Configuration management
│   ├── printer.rs      # Output formatting
│   ├── utils.rs        # Utility functions
│   ├── chatgpt/        # ChatGPT API client
│   └── sub/            # Subcommand implementations
├── docs/               # Documentation
├── book/               # mdBook documentation
├── scripts/            # Build and release scripts
├── notes/              # Release notes
├── test_data/          # Test files
├── Cargo.toml          # Rust project configuration
├── Cargo.lock          # Dependency lock file
└── Makefile           # Build automation
```

## Code Organization

### Main Components

- **`main.rs`**: Entry point and argument parsing
- **`chat.rs`**: Core chat functionality and context management
- **`config_manager.rs`**: Configuration file handling
- **`chatgpt/`**: OpenAI API client implementation
- **`sub/`**: Individual subcommand implementations

### Adding New Features

#### New Command Line Options
1. Add to `args.rs` in the appropriate struct
2. Handle the option in `main.rs`
3. Update help text and documentation

#### New Subcommands
1. Create a new file in `src/sub/`
2. Implement the subcommand logic
3. Add to `src/sub/mod.rs`
4. Update argument parsing in `args.rs`
5. Add documentation in `book/src/`

#### New API Endpoints
1. Add to the `chatgpt/` module
2. Update request/response structs
3. Add error handling
4. Update tests

## Common Issues and Solutions

### Build Issues
```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for compilation errors
cargo check
```

### Test Failures
- Ensure tests don't depend on external services
- Check for race conditions in concurrent tests
- Verify test data files exist and are correct

### Documentation Issues
- Run `mdbook build` to test documentation locally
- Check internal links are working
- Ensure code examples are correct

## Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Maintain professionalism in all interactions

### Communication
- Use GitHub issues for bug reports and feature requests
- Join discussions in pull requests
- Ask questions if anything is unclear
- Provide context and examples when reporting issues

## Recognition

Contributors are recognized in:
- Git commit history
- GitHub contributors list
- Release notes for significant contributions
- Project documentation

## Getting Help

If you need help contributing:
- Check existing issues and pull requests
- Ask questions in GitHub discussions
- Review the documentation
- Look at recent commits for examples

Thank you for contributing to Chat Gipitty! Every contribution, no matter how small, helps make the project better for everyone.
