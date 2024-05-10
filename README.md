# Chat Gipity
[![crates.io](https://img.shields.io/crates/v/cgip.svg)](https://crates.io/crates/cgip)

Chat gipity is a command line client for ChatGPT. It allows you to chat with your chosen model of ChatGPT in a terminal and even pipe output into it. The default model is GPT-4.

For example, say you wanted to debug your rust program that doesnt compile and 
want ChatGPT to explain it, you can pipe the output through chat-gipityto help you
debug like this:

![Gif of Piping](docs/piping.gif)

Another usage is reading from a file. In this example we read from a file and ask 
ChatGPT to convert that file to another programming language:

```sh
cgip "convert this to python" -f src/main.rs
```

# Installation

## Install from crates.io with cargo

```bash
cargo install cgip
```

## Install via homebrew
If you dont have the tap, add it:
```bash
brew tap divanvisagie/homebrew-tap
```
Install cgip with brew:
```
brew install cgip
```

## Manual installation
Download this repository and then install the `cgip` command
```bash
sudo make install
```

## Set up API Key
Next, set up your OpenAI key by exporting it as `OPENAI_API_KEY`
```
export OPENAI_API_KEY=your_key_here
```

You can now pipe data to `cgip`, remember to use `2>&1` to convert `stderr` to 
`stdout` if you are debugging, as the app can only read `stdin`


