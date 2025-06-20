# Installation

Chat GipiTTY is designed to be run on POSIX compliant systems. This page covers all available installation methods, with the recommended approach being installation via Cargo.

## Recommended: Install from crates.io with Cargo

**The recommended way to install Chat GipiTTY is via Cargo.** This ensures you always get the latest version directly from crates.io, with minimal dependencies and maximum compatibility.

If you do not already have Cargo (the Rust package manager) installed, please visit [rustup.rs](https://rustup.rs/) for instructions on installing Rust and Cargo.

```sh
cargo install cgip
```

### Upgrading

To upgrade to the latest release, you can use the built-in upgrade command:

```sh
cgip upgrade
```

Alternatively, you can reinstall via Cargo:

```sh
cargo install cgip --force
```

## Alternative Installation Methods

Other installation methods are available for convenience, but may not always provide the latest version:

### Manual Installation

If you prefer to build from source:

1. Clone the repository:
   ```sh
   git clone https://github.com/divanvisagie/chat-gipitty.git
   cd chat-gipitty
   ```

2. Install using make:
   ```sh
   sudo make install
   ```

### Development Setup

For development purposes, you can run Chat GipiTTY directly from the source:

```sh
git clone https://github.com/divanvisagie/chat-gipitty.git
cd chat-gipitty
cargo run -- --help
```

## Platform-Specific Notes

### Ubuntu/Debian

On Ubuntu and Debian systems, some additional packages may be required if you plan to build the deb package:

```sh
sudo apt-get install build-essential dh-make debhelper devscripts
```

### macOS

No additional setup required beyond having Cargo installed.

### Windows

Chat GipiTTY is designed for POSIX systems. On Windows, use WSL (Windows Subsystem for Linux) for the best experience.

## Verification

After installation, verify that Chat GipiTTY is working correctly:

```sh
cgip --version
```

You should see version information displayed. If you get a "command not found" error, make sure Cargo's bin directory is in your PATH:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

Add this line to your shell profile (`.bashrc`, `.zshrc`, etc.) to make it permanent.

## Next Steps

Once installed, you'll need to [set up your environment](./setup.md) with API credentials and any custom configuration.