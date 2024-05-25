use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{
    fs,
    io::{self, BufRead},
};

use atty::Stream;
use serde_yaml::Error;

use crate::chatgpt::Message;
use crate::config_manager::AppConfig;

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
            contents.contains("gpt-4"),
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
