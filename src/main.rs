use chatgpt::GptClient;
use clap::{command, Parser};
use std::{io::{self, Read, Write}, future::Future};

mod chatgpt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What you want to ask Chat GPT
    #[arg(short, long)]
    query: Option<String>,

    /// Still read from stdin when other options are provided (can break if no stdin is provided)
    #[arg(short, long)]
    stdin: bool,

    /// Enter interactive chat mode after asking the initial question
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

fn get_user_input(cli: &mut GptClient ,response_text: String) {
    println!("chat-gipity> {}", response_text);
    let username = get_logged_in_user_name();
    print!("\n{}> ", username);
    let mut input = String::new();
    io::stdin() 
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    cli.add_message(chatgpt::Role::Assistant, response_text.clone());
    cli.add_message(chatgpt::Role::User, input.trim().to_string());

    let response_text = cli.complete();
    get_user_input(cli, response_text);
}

fn main() {
    let args = Args::parse();
    let mut cli = GptClient::new();

    if let Some(query) = args.query {
        cli.add_message(chatgpt::Role::User, query);
        if args.stdin {
            let content = get_stdin();
            cli.add_message(chatgpt::Role::User, content);
        } 
    } else {
        let content = get_stdin();
        cli.add_message(chatgpt::Role::User, content);
    }

    let mut response_text = cli.complete();

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
    } else {
        println!("{}", response_text);
    }
}
