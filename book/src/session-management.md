# Session Management

Chat Gipitty provides powerful session management capabilities that allow for continuous conversations across multiple interactions. Sessions persist context between commands, enabling more natural and contextual conversations.

## How Sessions Work

Sessions store the conversation history (both user messages and assistant responses) and reuse this context in subsequent interactions. This allows the model to:

- Remember previous questions and answers
- Build upon earlier context
- Maintain conversation flow across commands
- Provide more coherent and contextually aware responses

## Enabling Sessions

Sessions are controlled by the `CGIP_SESSION_NAME` environment variable. Without this variable, Chat Gipitty operates in stateless mode.

```bash
# Enable sessions with a unique session ID
export CGIP_SESSION_NAME=$(uuidgen)

# Or use any custom session name
export CGIP_SESSION_NAME="my-coding-session"
```

## Session Naming Strategies

The uniqueness and persistence of your session depends on how you set the `CGIP_SESSION_NAME` variable:

### Terminal-Specific Sessions
Each new terminal gets its own session:
```bash
# Add to ~/.bashrc or ~/.zshrc
export CGIP_SESSION_NAME=$(uuidgen)
```

### Daily Sessions
Same session for the entire day:
```bash
export CGIP_SESSION_NAME=$(date -I)  # 2024-01-15
```

### Weekly Sessions
Sessions that persist for a week:
```bash
export CGIP_SESSION_NAME=$(date +%Y-W%U)  # 2024-W03
```

### Project-Based Sessions
Different sessions for different projects:
```bash
# Use current directory name
export CGIP_SESSION_NAME="project-$(basename $PWD)"

# Use git repository name if in a git repo
export CGIP_SESSION_NAME="git-$(git rev-parse --show-toplevel | xargs basename 2>/dev/null || echo 'no-git')"
```

### Custom Session Names
Manually managed sessions:
```bash
export CGIP_SESSION_NAME="debugging-api-issue"
export CGIP_SESSION_NAME="code-review-session"
export CGIP_SESSION_NAME="learning-rust"
```

## Session Workflow Examples

### Continuous Development Session
```bash
# First interaction
cgip "I'm working on a Rust web API project"

# Later, without needing to re-explain context
cgip "How should I structure the authentication middleware?"

# Even later
cgip "Can you help me write tests for the authentication we discussed?"
```

### Debugging Session
```bash
# Start debugging session
cargo build 2>&1 | cgip "This build is failing, what's wrong?"

# Continue debugging with context
cgip "How can I fix the lifetime issues you mentioned?"

# Test the fix
cargo build 2>&1 | cgip "Is this error related to our previous discussion?"
```

### Learning Session
```bash
# Initial question
cgip "I'm learning about async programming in Rust"

# Follow-up questions maintain context
cgip "Can you give me a practical example?"

# Build on previous examples
cgip "How would I modify that example to handle errors?"
```

## Session Management Commands

The `session` subcommand provides tools for managing your current session:

### View Session Context
```bash
cgip session --view
```
Shows all messages in the current session, allowing you to see what context will be included in future queries.

### Clear Session
```bash
cgip session --clear
```
Removes all stored context from the current session, starting fresh while keeping the same session name.

## Session Best Practices

### 1. Choose Appropriate Session Scope
- **Terminal sessions** (`uuidgen`): Best for isolated work sessions
- **Daily sessions** (`date -I`): Good for ongoing projects
- **Project sessions**: Best for long-term project work

### 2. Clear Sessions When Needed
Clear sessions when:
- Context becomes too long and irrelevant
- Switching to a completely different topic
- The model becomes confused by accumulated context
- Working with sensitive information

```bash
cgip session --clear
```

### 3. Use Session-Aware Queries
Take advantage of session context:
```bash
# Instead of repeating context
cgip "In the React project I mentioned earlier, how should I..."

# You can simply say
cgip "How should I implement user authentication in this project?"
```

### 4. Bypass Sessions When Needed
Use the `--no-session` flag for one-off queries:
```bash
# Quick lookup that doesn't need session context
cgip -n "What's the syntax for Python list comprehensions?"

# Sensitive query that shouldn't be stored
sensitive_command | cgip -n "analyze this output"
```

## Session Storage

Sessions are stored locally on your system in the Chat Gipitty configuration directory. The exact location depends on your operating system:

- **Linux**: `~/.config/cgip/sessions/`
- **macOS**: `~/Library/Application Support/cgip/sessions/`
- **Windows**: `%APPDATA%\cgip\sessions\`

Each session is stored as a separate file named after your `CGIP_SESSION_NAME`.

## Privacy and Security

### Session Privacy
- Sessions are stored locally and never sent to external servers
- Only the session content (messages) is sent to the API, not metadata
- Session files are readable only by your user account

### Managing Sensitive Sessions
For sensitive work:
```bash
# Use temporary session names
export CGIP_SESSION_NAME="temp-$(date +%s)"

# Clear after sensitive work
cgip session --clear

# Or use no-session mode
cgip -n "sensitive query"
```

## Advanced Session Patterns

### Conditional Session Management
```bash
# Different session strategies based on directory
if [[ "$PWD" == *"work"* ]]; then
    export CGIP_SESSION_NAME="work-$(date -I)"
else
    export CGIP_SESSION_NAME="personal-$(date -I)"
fi
```

### Session Inheritance
```bash
# Base session for project
export CGIP_BASE_SESSION="project-myapp"

# Feature-specific sessions that inherit context
export CGIP_SESSION_NAME="${CGIP_BASE_SESSION}-auth"
export CGIP_SESSION_NAME="${CGIP_BASE_SESSION}-frontend"
```

### Multi-User Environments
```bash
# Include username in session names
export CGIP_SESSION_NAME="$(whoami)-$(date -I)"

# Or use more specific naming
export CGIP_SESSION_NAME="$(whoami)-$(basename $PWD)-$(date +%Y%m%d)"
```

## Troubleshooting Sessions

### Session Not Working
If sessions aren't working:

1. Check if `CGIP_SESSION_NAME` is set:
   ```bash
   echo $CGIP_SESSION_NAME
   ```

2. Verify session storage permissions:
   ```bash
   ls -la ~/.config/cgip/sessions/  # Linux
   ls -la ~/Library/Application\ Support/cgip/sessions/  # macOS
   ```

3. Clear and restart session:
   ```bash
   cgip session --clear
   ```

### Session Too Long
If sessions become too long and unwieldy:
- Clear the session: `cgip session --clear`
- Use more specific session names for different topics
- Use `--no-session` for unrelated queries

### Context Confusion
If the model seems confused by session context:
- View the session: `cgip session --view`
- Clear irrelevant context: `cgip session --clear`
- Start a new session with a different name
