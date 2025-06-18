# Embedding Subcommand Documentation

The `embedding` subcommand generates embedding vectors for a piece of text using OpenAI-compatible embedding models.

## Basic Usage

```bash
cgip embedding "Hello, world"
```

Text can also be provided via stdin:

```bash
echo "some text" | cgip embedding
```

## Options

- `[TEXT]`: Text to embed (optional, reads from stdin if not provided, or combines with stdin if both are present)
- `--model <MODEL>`: Embedding model to use (default: text-embedding-3-small)
- `--output <FILE>`: Write the embedding vector to a file instead of stdout

The output is a comma-separated list of floating point numbers representing the embedding vector.
