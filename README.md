# Chat GipiTTY
[![crates.io](https://img.shields.io/crates/v/cgip.svg)](https://crates.io/crates/cgip)

Chat Gipitty (Chat Get Information, Print Information TTY) is a command line client for ChatGPT. It allows you to chat with
your chosen model of ChatGPT in a terminal and even pipe output into it. The
default model is GPT-4.

![logo](./docs/logo-256.png)

For example, say you wanted to debug your Rust program that doesn't compile and
want ChatGPT to explain the error in depth, you can pipe the output through Chat
Gipitty to help you debug like this, which would directly use the build error
output as the prompt:

```sh
cargo build 2>&1 | cgip
```

This would result in something like the following
```sh
❯ cargo build 2>&1 | cgip 'give me a short summary of the kind of error this is'
The error you're encountering is a **lifetime error** in Rust, specifically an issue with **borrowed values not living long enough**.
**borrowed values not living long enough**.
```

Another usage is reading from a file. In this example, we read from a file and
ask ChatGPT to convert that file to another programming language:
```sh cgip
"convert this to python" -f src/main.rs
```


# Installation

chat-gipitty is designed to be run on POSIX compliant systems, you have mutliple options for installing released versions depending on your system. All systems should be able to install from source or from cargo, but a homebrew tap is also available as well as a debian package attacked to the github releases.

### Install from crates.io with cargo
```sh
cargo install cgip
```

### Install via homebrew
If you don't have the tap, add it with:
```sh
brew tap divanvisagie/homebrew-tap
```

Install cgip with brew:
```sh
brew install cgip
```

### Manual Installation
Download this repository and then install the `cgip` command using:
```sh
sudo make install
```

## Set up API Key
Next, set up your OpenAI key by exporting it as `OPENAI_API_KEY`:
```sh
export OPENAI_API_KEY=your_key_here
```

## (Optional) Use a Custom API Endpoint

You can specify a custom API endpoint (for example, to use a local LLM server or an alternative provider) by setting the `OPENAI_BASE_URL` environment variable.
If not set, the default is `https://api.openai.com`.

```sh
export OPENAI_BASE_URL=http://localhost:11434/v1
```

You can now pipe data to `cgip`. Remember to use `2>&1` to convert `stderr` to
`stdout` if you are debugging, as it can only read `stdin`.

# Session management
The `session` feature allows for continuous chat session management and shell
integration. To enable session caching in your terminal, you need to provide the
environment variable `CGIP_SESSION_NAME`. The uniqueness of your session is
dependant on the value of this variable.

This can be done by generating a session id for cgip to use in your `.bashrc`
or `.zshrc`:
```sh
export CGIP_SESSION_NAME=$(uuid)
```

The user has control over how unique their session is, for example to have a new
session for every new terminal opened you can use `uuid`:

```sh
export CGIP_SESSION_NAME=$(uuid)
```
However if you want the session to work across terminals you may do something
like us the date as a session id instead:
```sh
export CGIP_SESSION_NAME=$(date -I)
```

This implementation allows users to decide how they want to manage their
sessions and gives full control for them to have complex session management if
required without cluttering cgip with session management features.

### Session Subcommand Usage
The `session` subcommand provides options for managing the current session
context, you can use it to either view or clear the session, this can sometimes
be useful if the context has become too long, or if the LLM has become confused
by the context.


# Development Setup

## Ubuntu

On ubuntu, some additional packages are required to build the deb package.

```sh
sudo apt-get install build-essential dh-make debhelper devscripts
```
