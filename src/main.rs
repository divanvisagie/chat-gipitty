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
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut cli = GptClient::new();

    if let Some(query) = args.query {
        // Handle the case when the query is not provided
        cli.add_message(chatgpt::Role::User, query);
        let response_text = cli.complete().await;
        println!("{}", response_text);
        return;
    } else {
        let mut content = String::new();
        io::stdin()
            .read_to_string(&mut content)
            .expect("Failed to read from stdin");

        cli.add_message(chatgpt::Role::User, content);

        let response_text = cli.complete().await;

        println!("{}", response_text);
    }
}
