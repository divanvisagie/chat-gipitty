use chatgpt::GptClient;
use clap::{command, Parser};
use std::io::{self, Read, Write};
use std::fs;

mod chatgpt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What you want to ask Chat GPT
    #[arg(short, long)]
    query: Option<String>,

    /// Read query from a file
    #[arg(short, long)]
    file: Option<String>,

    /// Format is JSON
    #[arg(short, long)]
    json: bool,

    /// Enter interactive chat mode after asking the initial question (not recommended if passing
    /// in data via stdin) !!!Experimental!!!
    #[arg(short, long)]
    interactive: bool
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

    // Check if any arguments have been passed in at all
    if args.query.is_none() && args.file.is_none() {
        let mut response_text: String;
        let mut query = String::new();
       
        if let Some(q) = args.query {
            query = q;
        }

        if let Some(file) = args.file {
            query = get_file_contents_from_path(file);
        }

        cli.add_message(chatgpt::Role::User, query);
        response_text = cli.complete();
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
    fn test_get_stdin() {
        let stdin = get_stdin();
        assert_eq!(stdin, "test\n");
    }

    #[test]
    fn test_get_file_contents_from_path() {
        let file_contents = get_file_contents_from_path("./test_data/test.txt".to_string());
        assert_eq!(file_contents, "test\n");
    }
}
