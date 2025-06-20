# Embedding Command

The `embedding` subcommand generates embedding vectors for text using OpenAI's embedding models. Embeddings are dense numerical representations of text that capture semantic meaning and can be used for similarity search, clustering, and other machine learning tasks.

## Overview

Embeddings convert text into high-dimensional vectors that represent the semantic meaning of the text. Similar texts will have similar embeddings, making them useful for:

- Semantic search and similarity matching
- Text clustering and classification
- Recommendation systems
- Information retrieval
- Machine learning feature vectors

## Usage

```bash
cgip embedding [OPTIONS] [TEXT]
```

### Arguments

- `[TEXT]` - Text to generate embeddings for. If not provided, reads from stdin

### Options

- `-m, --model <MODEL>` - Embedding model to use (default: text-embedding-3-small)
- `-o, --output <OUTPUT>` - Output file path. If not set, prints to stdout

## Basic Examples

### Generate Embeddings for Text
```bash
# Basic usage with text argument
cgip embedding "Hello, world!"
```

Output:
```
-0.0123456789, 0.0234567890, -0.0345678901, 0.0456789012, ...
```

### Read from stdin
```bash
# Pipe text to embedding command
echo "This is example text" | cgip embedding
```

### Save to File
```bash
# Save embedding vector to file
cgip embedding "Important text" --output embedding.txt

# Read from stdin and save to file
echo "Text from stdin" | cgip embedding --output vector.txt
```

## Model Options

### Available Models

#### text-embedding-3-small (default)
- **Dimensions**: 1536
- **Cost**: Lower cost option
- **Performance**: Good for most use cases
- **Speed**: Faster processing

```bash
cgip embedding "sample text" --model text-embedding-3-small
```

#### text-embedding-3-large
- **Dimensions**: 3072
- **Cost**: Higher cost
- **Performance**: Best accuracy and quality
- **Speed**: Slower processing

```bash
cgip embedding "sample text" --model text-embedding-3-large
```

#### text-embedding-ada-002 (legacy)
- **Dimensions**: 1536
- **Status**: Legacy model, still supported
- **Note**: Consider upgrading to newer models

```bash
cgip embedding "sample text" --model text-embedding-ada-002
```

## Advanced Examples

### Batch Processing
```bash
# Process multiple texts from a file
cat texts.txt | while read line; do
    echo "$line" | cgip embedding --output "embeddings/$(echo "$line" | tr ' ' '_').txt"
done
```

### Compare Text Similarity
```bash
# Generate embeddings for comparison
cgip embedding "The cat sat on the mat" --output cat_text.txt
cgip embedding "A feline rested on the rug" --output feline_text.txt

# Use external tools to calculate cosine similarity
```

### Document Processing
```bash
# Process documentation files
for file in docs/*.md; do
    filename=$(basename "$file" .md)
    cgip embedding --output "embeddings/${filename}.vec" -f "$file"
done
```

### Search Index Creation
```bash
# Create embeddings for search index
find . -name "*.txt" -exec sh -c '
    filename=$(basename "$1" .txt)
    cgip embedding --output "search_index/${filename}.vec" -f "$1"
' _ {} \;
```

## Input Handling

The embedding command handles input in the following priority:

1. **Text argument only**: Uses the provided text argument
2. **Stdin only**: Uses text from stdin (when no text argument is provided)
3. **Both stdin and text argument**: Combines stdin text with argument text
4. **Neither**: Shows error message

### Examples of Input Combinations

```bash
# Text argument only
cgip embedding "Hello world"

# Stdin only
echo "Hello world" | cgip embedding

# Both (combines stdin + argument)
echo "Hello" | cgip embedding "world"
# Results in embedding for: "Hello world"
```

## Output Formats

### Standard Output (default)
Comma-separated floating point numbers:
```
-0.012345, 0.023456, -0.034567, 0.045678, ...
```

### File Output
Same format but written to specified file:
```bash
cgip embedding "text" --output vector.txt
cat vector.txt
# -0.012345, 0.023456, -0.034567, 0.045678, ...
```

## Integration Examples

### With Python
```python
# Process embedding output in Python
import subprocess
import numpy as np

def get_embedding(text):
    result = subprocess.run(
        ['cgip', 'embedding', text], 
        capture_output=True, 
        text=True
    )
    return np.array([float(x.strip()) for x in result.stdout.split(',')])

embedding = get_embedding("Hello world")
print(f"Embedding shape: {embedding.shape}")
```

### With Shell Scripts
```bash
#!/bin/bash
# Generate embeddings for all text files

for file in *.txt; do
    echo "Processing $file..."
    basename=$(basename "$file" .txt)
    cgip embedding --output "${basename}.vec" -f "$file"
    echo "Saved embedding to ${basename}.vec"
done
```

### With JSON Processing
```bash
# Create JSON structure with embeddings
jq -n --arg text "Hello world" --argjson embedding "$(cgip embedding "Hello world" | jq -R 'split(",") | map(tonumber)')" '{
    text: $text,
    embedding: $embedding,
    timestamp: now
}'
```

## Use Cases

### Semantic Search
```bash
# Index documents
for doc in documents/*.txt; do
    cgip embedding -f "$doc" --output "index/$(basename "$doc").vec"
done

# Search for similar documents (requires similarity calculation)
cgip embedding "search query" --output query.vec
```

### Content Recommendation
```bash
# Generate embeddings for user preferences
cgip embedding "user likes science fiction and technology" --output user_profile.vec

# Generate embeddings for content items
cgip embedding "article about artificial intelligence" --output content_item.vec
```

### Text Classification
```bash
# Generate embeddings for training data
while IFS=',' read -r label text; do
    echo "$text" | cgip embedding --output "training/${label}_$(date +%s).vec"
done < training_data.csv
```

## Error Handling

### No Input Provided
```bash
cgip embedding
# Error: No text provided. Please provide text as an argument or via stdin.
```

### Invalid Model
```bash
cgip embedding "text" --model invalid-model
# Error: Model 'invalid-model' not found
```

### Output File Issues
```bash
cgip embedding "text" --output /root/readonly.txt
# Error: Cannot write to output file: Permission denied
```

## Best Practices

### 1. Choose the Right Model
- Use `text-embedding-3-small` for most applications
- Use `text-embedding-3-large` when accuracy is critical
- Consider cost vs. performance trade-offs

### 2. Preprocessing Text
```bash
# Clean and normalize text before embedding
echo "Text with    extra   spaces" | tr -s ' ' | cgip embedding
```

### 3. Batch Processing
```bash
# Process multiple texts efficiently
cat input_texts.txt | cgip embedding --output batch_embeddings.txt
```

### 4. Error Handling in Scripts
```bash
#!/bin/bash
if ! cgip embedding "test" > /dev/null 2>&1; then
    echo "Error: Embedding service not available"
    exit 1
fi
```

## Performance Considerations

### Token Limits
- Each model has input token limits
- Very long texts may be truncated
- Consider splitting long documents

### API Rate Limits
- Be mindful of API rate limits for batch processing
- Add delays between requests if needed
- Consider caching results

### Storage
- Embedding vectors can be large (1536-3072 dimensions)
- Consider compression for large-scale storage
- Use appropriate data types (float32 vs float64)

## Troubleshooting

### Common Issues

**API Key Issues**:
```bash
# Verify API key is set
echo $OPENAI_API_KEY

# Test with simple embedding
cgip embedding "test"
```

**Model Not Available**:
```bash
# List available models
cgip --list-models

# Use default model
cgip embedding "text" # Uses text-embedding-3-small
```

**Output File Problems**:
```bash
# Check directory permissions
ls -la output_directory/

# Use full path
cgip embedding "text" --output /full/path/to/output.txt
```
