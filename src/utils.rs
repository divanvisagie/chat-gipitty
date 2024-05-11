use atty::Stream;
use std::{
    fs,
    io::{self, BufRead},
};

use crate::chatgpt::Message;
use serde_yaml::Error;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_contents_from_path() {
        let file_contents = get_file_contents_from_path("./test_data/test.txt".to_string());
        assert_eq!(file_contents, "test\n");
    }
}
