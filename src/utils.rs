use std::{
    fs,
    io::{self, BufRead},
};
use atty::Stream;

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

pub fn get_logged_in_user_name() -> String {
    let output = std::process::Command::new("whoami")
        .output()
        .expect("Failed to get logged in user name");
    let user_name = String::from_utf8(output.stdout).unwrap();
    user_name.trim().to_string()
}

pub fn get_file_contents_from_path(path: String) -> String {
    // read file contents from filesyste
    fs::read_to_string(path).expect("Something went wrong reading the file")
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
