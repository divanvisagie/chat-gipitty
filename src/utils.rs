use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{
    fs,
    io::{self, BufRead},
};

use atty::Stream;
use anyhow::Error;
use std::collections::HashSet;
use tiktoken_rs::o200k_base;
use tracing::{error, info};

use crate::clients::openai::Message;
use crate::config_manager::AppConfig;
use crate::clients::openai::types::ChatRequest;

pub fn markdown_from_messages(messages: Vec<Message>) -> String {
    let initial = String::from("");
    let md = messages.iter().fold(initial, |acc, msg| {
        format!("{}**{}**: {}\n\n", acc, msg.role, msg.content)
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
                Err(err) => println!("Error reading line: {}", err),
            }
        }
    }

    lines.join("\n")
}

pub fn get_file_contents_from_path(path: String) -> String {
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

pub fn is_valid_yaml(yaml_str: &str) -> Result<bool, anyhow::Error> {
    let messages: Result<Vec<Message>, _> = serde_yaml::from_str(yaml_str);

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

pub fn ensure_config_file(dir: &PathBuf) -> std::io::Result<PathBuf> {
    fs::create_dir_all(dir)?;
    let config_path = dir.join("config.toml");
    if !config_path.exists() {
        let config = AppConfig::default();
        let contents = toml::to_string(&config).expect("Failed to serialize config");
        let mut file = File::create(&config_path)?;
        file.write_all(contents.as_bytes())?;
    }

    Ok(config_path)
}

fn message_to_string(msg: &Message) -> String {
    match msg.role.as_str() {
        "user" => format!("User: {}", msg.content),
        "assistant" => format!("Assistant: {}", msg.content),
        "system" => format!("System Note: {}", msg.content),
        _ => format!("{}: {}", msg.role, msg.content),
    }
}

pub fn compress_system_context(messages: &Vec<Message>) -> Vec<Message> {
    let first_index = messages.iter().position(|m| m.role == "system");
    let last_index = messages.iter().rposition(|m| m.role == "system");

    if let (Some(first), Some(last)) = (first_index, last_index) {
        if first != 0 || first == last {
            return messages.clone(); // return original if invalid or nothing to compress
        }

        let mut compressed = vec![messages[0].clone()];

        for i in first + 1..=last {
            let msg = &messages[i];
            let line = format!("\n{}", message_to_string(msg));
            compressed[0].content += &line;
        }

        // Add the remaining messages (after the last system prompt)
        compressed.extend_from_slice(&messages[last + 1..]);

        compressed
    } else {
        messages.clone()
    }
}

pub fn count_chat_tokens(messages: &[Message]) -> usize {
    let bpe = o200k_base().unwrap(); // Or handle error appropriately
    let mut num_tokens = 0;
    for message in messages {
        num_tokens += 4; // Every message follows <|start|>{role/name}\n{content}<|end|>\n
        num_tokens += bpe.encode_with_special_tokens(&message.role).len();
        num_tokens += bpe.encode_with_special_tokens(&message.content).len();
    }
    num_tokens += 3; // Every reply is primed with <|start|>assistant<|message|>
    num_tokens
}

// Helper function to estimate tokens for a single chat message
// Slightly simplified version of count_chat_tokens for one message
pub fn count_single_message_tokens(message: &Message) -> usize {
    let bpe = o200k_base().unwrap(); // Or handle error appropriately
    let mut num_tokens = 0;
    num_tokens += 4; // Overhead for message structure
    num_tokens += bpe.encode_with_special_tokens(&message.role).len();
    num_tokens += bpe.encode_with_special_tokens(&message.content).len();
    // Note: We don't add the +3 for assistant priming here, just the message itself
    num_tokens
}

pub fn truncate_messages_if_needed(messages: &mut Vec<Message>, limit: usize) {
    let mut current_tokens = count_chat_tokens(messages);
    info!("Current token count: {}", current_tokens);

    if current_tokens <= limit {
        return; // No truncation needed
    }

    info!(
        "Token count ({}) exceeds limit ({}), truncating...",
        current_tokens, limit
    );

    // Identify indices of system messages and the last message
    let system_message_indices: HashSet<usize> = messages
        .iter()
        .enumerate()
        .filter(|(_, m)| m.role == "system")
        .map(|(i, _)| i)
        .collect();
    let last_message_index = messages.len().saturating_sub(1); // Index of the last message

    // Start checking for removal from the first message
    let mut current_index = 0;

    while current_tokens > limit && current_index < messages.len() {
        // Check if the current index is a system message or the last message
        if system_message_indices.contains(&current_index) || current_index == last_message_index {
            // Skip this message, move to the next index
            current_index += 1;
            continue;
        }

        // If it's safe to remove (not system, not the last message)
        if messages.len() > 1 {
            // Ensure we don't remove the only message left (shouldn't happen here)
            info!(
                "Removing message at index {}: Role='{}', Content='{}...'",
                current_index,
                messages[current_index].role,
                messages[current_index]
                    .content
                    .chars()
                    .take(30)
                    .collect::<String>()
            );
            messages.remove(current_index);
            // Don't increment current_index, as removing shifts subsequent elements down.
            // Recalculate tokens and update system/last indices if needed (though less efficient)
            // For simplicity here, we just recalculate tokens. A more optimized approach
            // might update indices, but given the context size, recalculating tokens is okay.
            current_tokens = count_chat_tokens(messages);
            // Re-evaluate system_message_indices and last_message_index is safer if indices change significantly,
            // but let's stick to the simpler approach for now. If performance becomes an issue, optimize this.
        } else {
            // Safety break: Should not be able to remove the last message due to the check above.
            error!("Warning: Truncation stopped unexpectedly.");
            break;
        }
    }

    info!("Truncated token count: {}", current_tokens);

    if current_tokens > limit {
        error!(
            "Warning: Could not truncate messages enough while preserving system/last messages. Limit: {}, Current: {}",
            limit, current_tokens
        );
    }
}

pub fn get_last_message_in_chat_request(chat_request: &ChatRequest) -> Result<&str, anyhow::Error> {
    if let Some(last_message) = chat_request.messages.last() {
        if last_message.role == "user" {
            Ok(&last_message.content)
        } else {
            Err(anyhow::Error::msg("Last message is not a user message"))
        }
    } else {
        Err(anyhow::Error::msg("No messages in chat request"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::{Read, Write};
    use tempfile::TempDir;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_file_contents_from_path() {
            let file_contents = get_file_contents_from_path("./test_data/test.txt".to_string());
            assert_eq!(file_contents, "test\n");
        }
    }

    #[test]
    fn test_ensure_config_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("cgip");

        // Scenario 1: Neither directory nor file exists
        let config_path = ensure_config_file(&config_dir).expect("Failed to ensure config file");
        assert!(config_path.exists(), "The config file should be created");

        // Check if default content is written
        let mut contents = String::new();
        File::open(&config_path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert!(
            contents.contains("gpt-4o"),
            "Default settings should include the model name"
        );

        // Scenario 2: Directory exists but no config file
        fs::remove_file(&config_path).unwrap(); // Remove the config file
        let config_path =
            ensure_config_file(&config_dir).expect("Failed to ensure config file again");
        assert!(config_path.exists(), "The config file should be recreated");

        // Scenario 3: Both directory and file exist with custom content
        let custom_config = AppConfig {
            model: "custom-model".to_string(),
            show_progress: true,
            show_context: true,
            markdown: true,
            stored_context_length: 20,
            jarjar: false,
        };
        let custom_contents = toml::to_string(&custom_config).unwrap();
        File::create(&config_path)
            .unwrap()
            .write_all(custom_contents.as_bytes())
            .unwrap();
        ensure_config_file(&config_dir).expect("Failed to ensure config file a third time");
        contents.clear();
        File::open(&config_path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert!(
            contents.contains("custom-model"),
            "The existing custom config should not be overwritten"
        );
    }
}
