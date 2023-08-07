use chatgpt::GptClient;
use clap::{command, Parser};
use std::io::{self, Read, Write};
use std::fs;
use spinners::{Spinner, Spinners};

mod chatgpt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What you want to ask Chat GPT. Query is optional but is added to the prompt
    /// before file if file is present. If this is present stdin is ignored.
    #[arg(short, long)]
    query: Option<String>,

    /// Read query from a file. If query is present this is added to the prompt after query.
    /// If this is present stdin is ignored.
    #[arg(short, long)]
    file: Option<String>,

    /// Enter interactive chat mode after asking the initial question.
    /// This will be ignored if reading from stdin.
    #[arg(short, long)]
    interactive: bool,
    
    /// Show progress indicator (might mess up stdout)
    #[arg(short, long)]
    progress: bool
}

fn get_stdin() -> String {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .expect("Failed to read from stdin");

    content
}

fn get_logged_in_user_name() -> String {
    let output = std::process::Command::new("whoami")
        .output()
        .expect("Failed to get logged in user name");
    let user_name = String::from_utf8(output.stdout).unwrap();
    user_name.trim().to_string()
}

fn get_file_contents_from_path(path: String) -> String {
    // read file contents from filesyste
    fs::read_to_string(path)
        .expect("Something went wrong reading the file")
}

fn main() {
    let args = Args::parse();
    let mut cli = GptClient::new();

    let mut response_text: String;
   
    if let Some(q) = args.query.clone() {
        cli.add_message(chatgpt::Role::User, q);
    }

    if let Some(file) = args.file.clone() {
        let q = get_file_contents_from_path(file);
        cli.add_message(chatgpt::Role::User, q);
    }
    
    if !args.query.is_none() || !args.file.is_none() { // Only run the loop if not reading stdin
        if args.progress {
            let mut sp = Spinner::new(Spinners::Dots9, "Thinking...".into());
            response_text = cli.complete();
            sp.stop(); 
            print!("\x1B[2K"); // Clear the current line
            print!("\r"); // Move the cursor to the beginning of the current line
        } else {
            response_text = cli.complete();
        }
        println!("{}", response_text);

        if args.interactive {
            loop {
                println!("chat-gipity> {}", response_text);
                let username = get_logged_in_user_name();
                print!("{}> ", username);    
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin() 
                    .read_line(&mut input)
                    .expect("Failed to read from stdin");

                cli.add_message(chatgpt::Role::Assistant, response_text.clone());
                cli.add_message(chatgpt::Role::User, input.trim().to_string());

                response_text = cli.complete();
            }
        }
    } else {
        let content = get_stdin();
        cli.add_message(chatgpt::Role::User, content);

        let response_text = cli.complete();
        println!("{}", response_text);
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
