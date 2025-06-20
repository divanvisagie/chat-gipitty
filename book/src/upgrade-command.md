# Upgrade Command

The `upgrade` subcommand provides a convenient way to upgrade Chat Gipitty to the latest release directly from the command line. This ensures you always have access to the newest features and bug fixes.

## Overview

The upgrade command automatically:
- Checks for the latest version available
- Downloads and installs the new version
- Preserves your existing configuration
- Provides feedback on the upgrade process

## Usage

```bash
cgip upgrade
```

The upgrade command takes no additional arguments or options.

## How It Works

When you run `cgip upgrade`, the following process occurs:

1. **Version Check**: Compares your current version with the latest available release
2. **Download**: Downloads the appropriate binary for your system
3. **Installation**: Replaces the current binary with the new version
4. **Verification**: Confirms the upgrade was successful

## Examples

### Basic Upgrade
```bash
cgip upgrade
```

Example output:
```
Current version: 0.4.5
Latest version: 0.5.0
Downloading cgip v0.5.0...
✓ Download complete
✓ Installation successful
Chat Gipitty has been upgraded to v0.5.0
```

### Already Up to Date
```bash
cgip upgrade
```

When already on the latest version:
```
Current version: 0.5.0
You are already running the latest version of cgip.
```

## Installation Methods

The upgrade command works differently depending on how Chat Gipitty was originally installed:

### Cargo Installation (Recommended)
If installed via `cargo install cgip`, the upgrade command will:
- Use `cargo install --force cgip` to reinstall with the latest version
- Maintain all cargo-specific configurations
- Work seamlessly with Rust toolchain

### Manual Installation
For manual installations, the upgrade command will:
- Download the appropriate binary for your platform
- Replace the existing binary in place
- Preserve file permissions

## Platform Support

The upgrade command supports automatic upgrades on:

- **Linux** (x86_64, ARM64)
- **macOS** (Intel, Apple Silicon)
- **Windows** (x86_64)

Platform detection is automatic based on your system.

## Prerequisites

### Permissions
The upgrade command needs appropriate permissions to:
- Write to the installation directory
- Replace the existing binary
- Create temporary files for download

### Network Access
- Internet connection for downloading releases
- Access to GitHub releases API
- Access to release asset downloads

### System Requirements
- Same system requirements as the new version
- Sufficient disk space for download and installation

## Configuration Preservation

The upgrade process preserves:
- **Configuration files**: Your `config.toml` settings remain unchanged
- **Session data**: Existing sessions are maintained
- **Environment variables**: All environment settings continue to work
- **Shell integration**: Existing shell aliases and functions remain functional

## Safety Features

### Backup
The upgrade process includes safety measures:
- Creates backup of current binary before replacement
- Can rollback if upgrade fails
- Validates new binary before finalizing upgrade

### Verification
After upgrade:
- Verifies new binary is functional
- Confirms version number matches expected version
- Tests basic functionality

## Troubleshooting

### Permission Denied
If you encounter permission errors:

```bash
# Linux/macOS: Use sudo if needed
sudo cgip upgrade

# Or change ownership of installation directory
sudo chown -R $(whoami) /usr/local/bin/cgip
```

### Network Issues
For network-related problems:

```bash
# Check internet connectivity
ping github.com

# Check if GitHub is accessible
curl -I https://api.github.com/repos/divanvisagie/chat-gipitty/releases/latest
```

### Download Failures
If downloads fail:
- Check available disk space
- Verify write permissions to temp directory
- Try again later (temporary GitHub issues)

### Corrupted Download
If the downloaded binary appears corrupted:
- The upgrade will automatically retry
- Check your internet connection stability
- Clear temporary files and try again

## Manual Upgrade Alternative

If the automatic upgrade fails, you can upgrade manually:

### Via Cargo (Recommended)
```bash
cargo install --force cgip
```

### Via GitHub Releases
1. Visit the [releases page](https://github.com/divanvisagie/chat-gipitty/releases)
2. Download the appropriate binary for your system
3. Replace the existing binary
4. Make the new binary executable (Linux/macOS)

### Via Package Managers
Some package managers may have their own update mechanisms:

```bash
# Homebrew (if available)
brew upgrade cgip

# APT (if available)
sudo apt update && sudo apt upgrade cgip
```

## Upgrade Frequency

### Checking for Updates
You can check if updates are available without upgrading:

```bash
# Check current version
cgip --version

# Visit releases page or use upgrade command (it will show if updates are available)
cgip upgrade
```

### Recommended Schedule
- **Patch releases**: Upgrade promptly for bug fixes
- **Minor releases**: Upgrade when convenient for new features  
- **Major releases**: Review changelog and upgrade when ready

## Release Channels

Chat Gipitty follows semantic versioning:

- **Patch releases** (0.5.1): Bug fixes, security updates
- **Minor releases** (0.6.0): New features, improvements
- **Major releases** (1.0.0): Breaking changes, major rewrites

The upgrade command always targets the latest stable release.

## Rollback

If you need to rollback after an upgrade:

### Automatic Rollback
The upgrade process includes automatic rollback on failure.

### Manual Rollback
If you need to manually rollback:

```bash
# Reinstall specific version via cargo
cargo install --force cgip --version 0.4.5

# Or download specific version from releases
# and replace the binary manually
```

## Integration with CI/CD

For automated environments:

```bash
# Check if upgrade is needed in scripts
CURRENT_VERSION=$(cgip --version | cut -d' ' -f2)
LATEST_VERSION=$(curl -s https://api.github.com/repos/divanvisagie/chat-gipitty/releases/latest | jq -r .tag_name)

if [ "$CURRENT_VERSION" != "$LATEST_VERSION" ]; then
    echo "Upgrade available: $CURRENT_VERSION -> $LATEST_VERSION"
    cgip upgrade
fi
```

## Best Practices

### Before Upgrading
1. **Read the changelog** to understand what's changing
2. **Backup important sessions** if doing major upgrades
3. **Test in non-production environments** first

### After Upgrading
1. **Verify functionality** with a simple test query
2. **Check that your configuration** still works
3. **Review new features** in the documentation

### Staying Updated
- Subscribe to releases on GitHub for notifications
- Check periodically with `cgip upgrade`
- Join community channels for upgrade announcements

## Security Considerations

The upgrade process:
- Downloads only from official GitHub releases
- Verifies checksums when available
- Uses secure HTTPS connections
- Preserves existing file permissions
