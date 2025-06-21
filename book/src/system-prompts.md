# System Prompts

System prompts allow you to customize the AI's behavior by providing persistent instructions that apply to all interactions. They're like setting the "personality" or "role" for the AI model, helping it respond in a specific way that matches your needs.

## What are System Prompts?

System prompts are special instructions that:
- Set the AI's role, personality, or expertise level
- Apply to all messages in a conversation
- Influence how the AI interprets and responds to queries
- Remain active throughout a session

## Setting System Prompts

### Command Line

Use the `--system` flag to set a system prompt for a single query:

```sh
cgip --system "You are a senior Rust developer" "explain this error" -f error.log
```

### Configuration

Set a default system prompt that applies to all queries:

```sh
# Set a default system prompt
cgip config --set system_prompt="You are a helpful assistant specializing in software development"

# View current system prompt
cgip config --get system_prompt

# Clear system prompt
cgip config --unset system_prompt
```

### Session-Specific Prompts

System prompts work great with sessions:

```sh
# Set session and system prompt for a specific project
export CGIP_SESSION_NAME="rust-project"
cgip --system "You are a Rust expert helping with systems programming" "initial question"

# Subsequent queries in the same session maintain the system prompt context
cgip "follow-up question"
```

## Common System Prompt Patterns

### Role-Based Prompts

#### Senior Developer
```sh
cgip --system "You are a senior software engineer with 15 years of experience. Provide detailed, production-ready advice." "review this code" -f app.py
```

#### Code Reviewer
```sh
cgip config --set system_prompt="You are a thorough code reviewer. Focus on security, performance, maintainability, and best practices."
```

#### DevOps Engineer
```sh
cgip --system "You are a DevOps engineer specializing in containerization and CI/CD pipelines." "analyze this Docker setup" -f docker-compose.yml
```

### Expertise Level Prompts

#### Beginner-Friendly
```sh
cgip --system "Explain everything in simple terms suitable for a beginner. Use analogies and avoid jargon." "what is a REST API?"
```

#### Expert Level
```sh
cgip --system "Assume deep technical expertise. Provide advanced insights and implementation details." "optimize this algorithm" -f algorithm.py
```

### Communication Style Prompts

#### Concise Responses
```sh
cgip config --set system_prompt="Be extremely concise. Provide only essential information."
```

#### Detailed Explanations
```sh
cgip --system "Provide comprehensive explanations with examples, reasoning, and context." "explain this concept"
```

#### Structured Output
```sh
cgip --system "Always structure responses with clear headings, bullet points, and numbered steps." "analyze this issue"
```

## Specialized System Prompts

### Security Analysis
```sh
cgip --system "You are a cybersecurity expert. Focus on identifying vulnerabilities, security best practices, and potential attack vectors." "review this authentication code" -f auth.py
```

### Performance Optimization
```sh
cgip --system "You are a performance optimization specialist. Focus on efficiency, scalability, and resource usage." "analyze this query" -f slow-query.sql
```

### Documentation Writing
```sh
cgip --system "You are a technical writer. Create clear, well-structured documentation with examples." "document this API" -f api.py
```

### Testing and QA
```sh
cgip --system "You are a QA engineer. Focus on test coverage, edge cases, and quality assurance." "create test cases for this function" -f function.js
```

## Context-Specific Prompts

### Project-Specific
```sh
# For a React project
export CGIP_SESSION_NAME="react-app"
cgip --system "You are a React expert working on a modern web application using hooks, TypeScript, and Next.js."

# For a data science project
export CGIP_SESSION_NAME="data-analysis"
cgip --system "You are a data scientist specializing in Python, pandas, and machine learning."
```

### Technology-Specific
```sh
# Rust development
cgip --system "You are a Rust systems programmer. Focus on memory safety, performance, and idiomatic Rust patterns."

# Cloud architecture
cgip --system "You are a cloud architect specializing in AWS. Consider scalability, cost optimization, and best practices."
```

## Advanced System Prompt Techniques

### Multi-Role Prompts
```sh
cgip --system "You are both a senior developer and a mentor. Provide code solutions while teaching underlying concepts." "help me understand this pattern" -f design-pattern.py
```

### Output Format Specifications
```sh
cgip --system "Always respond in this format: 1) Problem Summary 2) Solution 3) Implementation Steps 4) Testing Approach" "debug this issue"
```

### Constraint-Based Prompts
```sh
cgip --system "You can only suggest solutions using standard library functions. No external dependencies allowed." "implement this feature" -f requirements.txt
```

### Industry-Specific
```sh
# Financial technology
cgip --system "You are a fintech developer. Consider regulatory compliance, security, and financial accuracy."

# Healthcare
cgip --system "You are a healthcare software developer. Prioritize HIPAA compliance, data privacy, and reliability."
```

## Best Practices

### Effective System Prompts

1. **Be Specific**: Clear, specific instructions work better than vague ones
2. **Set Expertise Level**: Specify the level of technical detail you want
3. **Define Output Format**: Tell the AI how to structure responses
4. **Include Constraints**: Mention any limitations or requirements
5. **Set Context**: Provide relevant background information

### Examples of Good vs. Poor System Prompts

#### Good System Prompts
```sh
# Good: Specific role and output format
cgip --system "You are a senior Python developer. Provide code solutions with explanations, focusing on readability and performance. Include error handling and type hints."

# Good: Clear constraints and context
cgip --system "You are a DevOps engineer working with Kubernetes. Suggest solutions that work in production environments and follow security best practices."
```

#### Poor System Prompts
```sh
# Poor: Too vague
cgip --system "Be helpful"

# Poor: Contradictory instructions
cgip --system "Be extremely detailed but also very concise"
```

### Managing System Prompts

#### Environment-Based Configuration
```sh
# Development environment
cgip config --set system_prompt="You are a developer focused on rapid prototyping and debugging."

# Production environment
cgip config --set system_prompt="You are a senior engineer focused on reliability, security, and maintainability."
```

#### Project-Specific Setup
Create project-specific configurations:

```sh
# In project directory
echo 'export CGIP_SESSION_NAME="$(basename $(pwd))"' >> .envrc
echo 'cgip config --set system_prompt="You are a [PROJECT_TYPE] expert working on [PROJECT_NAME]."' >> .envrc
```

## System Prompt Examples by Use Case

### Code Development
```sh
# General development
cgip config --set system_prompt="You are a senior full-stack developer. Provide clean, maintainable code with proper error handling and documentation."

# Specific language
cgip --system "You are a Go expert. Focus on idiomatic Go patterns, concurrency, and performance." "optimize this function" -f handler.go
```

### System Administration
```sh
# Linux administration
cgip config --set system_prompt="You are a Linux system administrator. Focus on security, performance, and best practices for production systems."

# Network administration
cgip --system "You are a network engineer. Consider security, performance, and monitoring when suggesting solutions."
```

### Data Analysis
```sh
# Data science
cgip --system "You are a data scientist. Provide Python solutions using pandas, numpy, and matplotlib. Include data validation and visualization suggestions."

# Database optimization
cgip --system "You are a database administrator specializing in PostgreSQL. Focus on query optimization, indexing, and performance tuning."
```

## Troubleshooting System Prompts

### System Prompt Not Working
```sh
# Check if system prompt is set
cgip config --get system_prompt

# Verify with context display
cgip --show-context "test query"
```

### Conflicting Instructions
If system prompts conflict with query instructions:
- System prompts take precedence
- Be specific in your query to override system behavior
- Consider updating the system prompt for better alignment

### Session Confusion
If the AI seems confused about its role:
- Clear the session: `cgip session --clear`
- Restart with a clearer system prompt
- Make system prompts more specific

System prompts are a powerful way to customize Chat GipiTTY's behavior for your specific workflows and requirements. Experiment with different prompts to find what works best for your use cases.
