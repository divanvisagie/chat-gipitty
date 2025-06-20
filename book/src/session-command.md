# Session Command

The `session` subcommand provides tools for managing your current conversation session. It allows you to view, clear, and manage the context that Chat Gipitty maintains between interactions.

## Overview

Sessions in Chat Gipitty store conversation history to enable continuous, contextual conversations. The session command gives you control over this stored context.

## Usage

```bash
cgip session [OPTIONS]
```

## Options

### `--view, -v`
View the current session context, showing all stored messages.

```bash
cgip session --view
```

This displays:
- All user messages from the session
- All assistant responses
- The total number of messages
- Session metadata (name, creation time, etc.)

### `--clear, -c`
Clear all stored context from the current session.

```bash
cgip session --clear
```

This removes all conversation history while keeping the session name active for future interactions.

## Examples

### Viewing Session Content
```bash
# Check what's in your current session
cgip session --view
```

Example output:
```
Session: my-coding-session (5 messages)
Created: 2024-01-15 10:30:00

[User] I'm working on a Rust web API
[Assistant] That's great! Rust is excellent for web APIs due to its performance and safety...

[User] How should I handle authentication?
[Assistant] For authentication in a Rust web API, you have several good options...

[User] Can you show me an example with JWT?
[Assistant] Here's a practical example of JWT authentication in Rust...
```

### Clearing Session History
```bash
# Clear current session to start fresh
cgip session --clear
```

Example output:
```
Session 'my-coding-session' cleared successfully.
```

### Checking Session Status
```bash
# View current session (will show empty if no context)
cgip session --view
```

If no session is active:
```
No active session. Set CGIP_SESSION_NAME to enable sessions.
```

## Session Workflow

### Typical Session Management Workflow

1. **Start a session** by setting the session name:
   ```bash
   export CGIP_SESSION_NAME="debugging-session"
   ```

2. **Have conversations** with Chat Gipitty:
   ```bash
   cgip "I'm having issues with my React component"
   cgip "The error says 'Cannot read property of undefined'"
   ```

3. **Check session context** when needed:
   ```bash
   cgip session --view
   ```

4. **Clear session** when context becomes irrelevant:
   ```bash
   cgip session --clear
   ```

## When to Use Session Commands

### View Session (`--view`)
Use when you want to:
- See what context will be included in your next query
- Debug why the model's responses seem off-topic
- Review the conversation history
- Check if sensitive information is stored
- Understand how much context has accumulated

### Clear Session (`--clear`)
Use when you need to:
- Start a completely new topic
- Remove irrelevant or confusing context
- Clear sensitive information from the session
- Reset after the context has become too long
- Fix issues where the model seems confused

## Integration with Other Commands

### Session-Aware Queries
All regular Chat Gipitty commands use session context:

```bash
# First query establishes context
cgip "I'm learning Rust ownership concepts"

# Later queries build on this context
cgip "Can you give me an example of borrowing?"
cgip "What about mutable references?"

# Check what context is being used
cgip session --view
```

### Bypassing Sessions
Use `--no-session` to ignore session context:

```bash
# This won't add to or use session context
cgip --no-session "What's the weather like?" 

# Session context remains unchanged
cgip session --view  # Still shows previous Rust discussion
```

## Session Information Display

When viewing sessions, you'll see:

### Message Count
The total number of messages (user + assistant) stored.

### Session Name
The current `CGIP_SESSION_NAME` value.

### Creation Time
When the session was first created.

### Message History
All user questions and assistant responses in chronological order.

### Context Size
Indication of how much context is being stored (useful for token management).

## Best Practices

### Regular Session Maintenance
```bash
# Check session size periodically
cgip session --view | head -1

# Clear when context becomes too large or irrelevant
cgip session --clear
```

### Topic Transitions
```bash
# Before switching topics, check current context
cgip session --view

# Clear if the new topic is unrelated
cgip session --clear

# Then start the new topic
cgip "Now I want to learn about Docker containers"
```

### Debugging with Session Commands
```bash
# If responses seem off-topic, check session context
cgip session --view

# Clear and retry if context is confusing
cgip session --clear
cgip "Let me rephrase my question..."
```

## Error Handling

### No Session Active
```bash
cgip session --view
# Output: No active session. Set CGIP_SESSION_NAME to enable sessions.
```

Solution:
```bash
export CGIP_SESSION_NAME="my-session"
cgip session --view
```

### Empty Session
```bash
cgip session --view
# Output: Session 'my-session' is empty.
```

This is normal for new sessions or after clearing.

### Permission Issues
If you get permission errors, check session storage directory:
```bash
# Linux/macOS
ls -la ~/.config/cgip/sessions/
ls -la ~/Library/Application\ Support/cgip/sessions/
```

## Advanced Usage

### Scripting with Session Commands
```bash
#!/bin/bash
# Script to manage coding sessions

# Check if session has too many messages
MESSAGE_COUNT=$(cgip session --view | grep -o '[0-9]* messages' | cut -d' ' -f1)

if [ "$MESSAGE_COUNT" -gt 20 ]; then
    echo "Session has $MESSAGE_COUNT messages, clearing..."
    cgip session --clear
fi
```

### Session Inspection
```bash
# Get session info for debugging
cgip session --view > session_dump.txt

# Analyze session content
grep "User\]" session_dump.txt | wc -l  # Count user messages
grep "Assistant\]" session_dump.txt | wc -l  # Count assistant messages
```

### Conditional Session Management
```bash
# Clear session if working directory changes
CURRENT_PROJECT=$(basename $PWD)
if [[ "$LAST_PROJECT" != "$CURRENT_PROJECT" ]]; then
    cgip session --clear
    export LAST_PROJECT="$CURRENT_PROJECT"
fi
```
