use std::fs;
use std::io::{self, BufRead};

use atty::Stream;
use serde_yaml::Error;

use crate::chatgpt::Message;

pub fn new_ensure_config_directory(config_directory: &std::path::PathBuf) {
    if !config_directory.exists() {
        std::fs::create_dir_all(config_directory).expect("Failed to create config directory");
    }
}

pub fn markdown_from_messages(messages: Vec<Message>) -> String {
    let initial = String::from("");
    let md = messages.iter().fold(initial, |acc, msg| {
        format!("{}**{}**: {}\n\n", acc, msg.role, msg.content.to_string())
    });
    md
}

pub fn get_stdin() -> String {
    let mut lines: Vec<String> = Vec::new();

    // Check if stdin is attached to a terminal or is being piped from another process
    if !atty::is(Stream::Stdin) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => lines.push(line),
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    break;
                }
            }
        }
    }

    lines.join("\n")
}

pub fn get_file_contents_from_path(path: String) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

pub fn is_valid_yaml(yaml_str: &str) -> Result<bool, Error> {
    let messages: Result<Vec<Message>, Error> = serde_yaml::from_str(yaml_str);

    match messages {
        Ok(msgs) => {
            for msg in msgs {
                if msg.role.is_empty() || msg.content.is_empty() {
                    return Ok(false);
                }
                // Add more validation logic if needed
            }
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

pub fn ensure_config_file(
    config_directory: &std::path::PathBuf,
) -> Result<std::path::PathBuf, std::io::Error> {
    new_ensure_config_directory(config_directory);

    let config_file_path = config_directory.join("config.toml");

    // Check if the config file exists
    if !config_file_path.exists() {
        // Create the config file with default content
        let default_config = r#"
# Configuration file for cgip

# Default model to use for completions
model = "gpt-4o"

# Whether to show progress indicators
show_progress = false

# Whether to show context information
show_context = false

# Whether to output in markdown format
markdown = false

# Whether to speak like Jar Jar Binks
jarjar = false

# Number of context messages to store
stored_context_length = 20
"#;

        std::fs::write(&config_file_path, default_config)?;
    }

    Ok(config_file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_ensure_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir_path = temp_dir.path().join("cgip");

        let config_file_path = ensure_config_file(&config_dir_path).unwrap();

        assert!(config_file_path.exists());
        let contents = fs::read_to_string(&config_file_path).unwrap();
        assert!(
            contents.contains("gpt-4o"),
            "Default settings should include the model name"
        );
    }

    #[test]
    fn test_get_file_contents_from_path() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let test_content = "Hello, world!";

        std::fs::write(&file_path, test_content).unwrap();

        let result = get_file_contents_from_path(file_path.to_string_lossy().to_string());
        assert_eq!(result, test_content);
    }

    #[test]
    fn test_markdown_from_messages() {
        let messages = vec![
            Message {
                role: "user".to_string(),
                name: None,
                tool_call_id: None,
                tool_calls: None,
                content: crate::chatgpt::MessageContent::Text("Hello".to_string()),
            },
            Message {
                role: "assistant".to_string(),
                name: None,
                tool_call_id: None,
                tool_calls: None,
                content: crate::chatgpt::MessageContent::Text("Hi there!".to_string()),
            },
        ];

        let markdown = markdown_from_messages(messages);
        assert!(markdown.contains("**user**: Hello"));
        assert!(markdown.contains("**assistant**: Hi there!"));
    }
}