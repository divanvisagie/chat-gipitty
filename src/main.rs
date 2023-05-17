use chatgpt::GptClient;
use clap::{command, Parser};
use std::io::{self, Read};

mod chatgpt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// What you want to ask Chat GPT
    #[arg(short, long)]
    query: Option<String>,

    /// Still read from stdin when other options are provided (can break if no stdin is provided)
    #[arg(short, long)]
    stdin: bool
}

fn get_stdin() -> String {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .expect("Failed to read from stdin");

    content
}

#[tokio::main]
async fn main() {
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

    let response_text = cli.complete().await;
    println!("{}", response_text);
}
