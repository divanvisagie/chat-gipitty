# Chat Gipity
[![crates.io](https://img.shields.io/crates/v/cgip.svg)](https://crates.io/crates/cgip)

Chat gipity is a command line client for ChatGPT. It allows you to chat with the 
GPT-3 model in a terminal and even pipe output into it.


For example, say you wanted to debug your rust program that doesnt compile and 
want ChatGPT to explain it, you can type:

```sh
cargo run 2>&1 | cgip
```

The error output will then be forwarded to ChatGPT which will return the following

```sh
It seems like there are three errors in the codebase:

1. In `src/main.rs:106:2`, it appears that there was a missing `;` at the end of line 106. Adding the semicolon should resolve this error.

2. In `src/main.rs:107:5`, there is an issue with the let statement. It's likely that this is caused by not having a fully declared variable type. You could try specifying the type of `response_text`, like this `let response_text: String = get_response(content).await;`
3. Lastly, the `main` function cannot be `async`. In the rust programming language, the `async` keyword is used when working with asynchronous code within other functions that are not `main`. You could try removing the `async` keyword from the `main` function because it is not allowed.
```

Another usage is reading from a file. In this example we read from a file and ask 
ChatGPT to convert that file to another programming language:

```sh
cgip -q "convert this to python" -f src/main.rs
```

# Installation

## Install via crates.io

```bash
cargo install cgip
```

## Install via homebrew
```bash
brew tap divanvisagie/homebrew-tap
brew install cgip
```

## Manual installation
Download this repository and then install the `cgip` command
```bash
cargo install --path .
```

## Set up API Key
Next, set up your OpenAI key by exporting it as `OPENAI_API_KEY`
```
export OPENAI_API_KEY=your_key_here
```

You can now pipe data to `cgip`, remember to use 2>&1 to convert `stderr` to 
`stdout` if you are debugging, as the app can only read `stdin`


