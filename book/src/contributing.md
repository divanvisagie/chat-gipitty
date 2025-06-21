# Contributing

Thank you for your interest in contributing to Chat GipiTTY! This guide covers how to contribute to the project, from reporting bugs to submitting code changes.

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

## Development Setup

For detailed development setup instructions, see the [Development Workflow](./development-workflow.md) section, which covers:

- Prerequisites and dependencies
- Building from source
- Running tests
- Project structure
- Development tools and debugging

## Contribution Process

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
Before submitting, ensure your changes work correctly:

```bash
# Run the full test suite
cargo test

# Check formatting
cargo fmt --check

# Run clippy for linting
cargo clippy

# Test manually
cargo run -- "test query"
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
- Update the book documentation for user-facing changes

## Adding New Features

### New Command Line Options
1. Add to `args.rs` in the appropriate struct
2. Handle the option in `main.rs`
3. Update help text and documentation

### New Subcommands
1. Create a new file in `src/sub/`
2. Implement the subcommand logic
3. Add to `src/sub/mod.rs`
4. Update argument parsing in `args.rs`
5. Add documentation in `book/src/`

### New API Features
1. Add to the `chatgpt/` module
2. Update request/response structs
3. Add error handling
4. Update tests

## Testing Guidelines

### Types of Tests
- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test feature interactions
- **Manual testing**: Test real-world scenarios

### Test Requirements
- Add tests for new functionality
- Test error conditions and edge cases
- Ensure tests are deterministic and isolated
- Don't depend on external services in tests

### Manual Testing Checklist
```bash
# Test basic functionality
cargo run -- "test query"

# Test with different input sources
echo "test" | cargo run -- "analyze this"
cargo run -- -f test_file.txt "analyze this file"

# Test subcommands
cargo run -- config --help
cargo run -- session --help
```

## Review Process

### What to Expect
1. **Initial Review**: Maintainers will review your pull request
2. **Feedback**: You may receive suggestions for improvements
3. **Iterations**: Make requested changes and push updates
4. **Approval**: Once approved, your changes will be merged

### Review Criteria
- Code quality and style
- Test coverage
- Documentation updates
- Backward compatibility
- Performance impact

## Issue Reporting

### Bug Reports
When reporting bugs, please include:
- Steps to reproduce the issue
- Expected vs actual behavior
- System information (OS, Rust version)
- Relevant error messages or logs
- Minimal example if possible

### Feature Requests
For feature requests, please describe:
- The problem you're trying to solve
- Your proposed solution
- Alternative approaches considered
- Potential impact on existing functionality

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

Contributors are recognized through:
- Git commit history
- GitHub contributors list
- Release notes for significant contributions
- Project documentation

## Getting Help

If you need help contributing:
- Check the [Development Workflow](./development-workflow.md) for technical setup
- Review existing issues and pull requests
- Ask questions in GitHub discussions
- Look at recent commits for examples

## Release Process

For information about the release process, see the [Development Workflow](./development-workflow.md#release-process) section.

Thank you for contributing to Chat GipiTTY! Every contribution, no matter how small, helps make the project better for everyone.
