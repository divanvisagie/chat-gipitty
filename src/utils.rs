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

pub fn get_file_contents_from_path(path: String) -> String {
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
