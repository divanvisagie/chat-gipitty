# Agent Subcommand Documentation

The `agent` subcommand provides an agentic workflow that lets the model run shell commands on your behalf. It limits all operations to the directory you specify and makes a special `execute` tool available to the model for running commands.

## Basic Usage

```bash
cgip agent /path/to/project "build the project"
```

## Options

- `DIRECTORY`: Directory the agent is allowed to operate in.
- `INSTRUCTION`: Natural language instruction describing the goal.
- `--input <FILES>`: Comma separated list of files whose contents should be added to the context.
- `--max-actions <N>`: Maximum number of commands the agent will execute before stopping (default: 10).

## How It Works

When invoked, the agent sends your instruction and any provided file contents to the model along with a tool definition:

```json
{"type": "function", "function": {"name": "execute", "description": "Run a shell command", "parameters": {"type": "object", "properties": {"command": {"type": "string"}}, "required": ["command"]}}}
```

The model can call this tool to run commands. Command output is printed to your terminal and also fed back to the model until it responds with a final answer. When the agent finishes, it prints a short summary of the commands that were executed.

## Example

```bash
cgip agent . "list the current directory"
```

This will let the model issue an `execute` call with `ls` and return the result.
When the model needs additional information to answer your instruction, it will
automatically run commands using `execute` and use the output to craft the final
response.
