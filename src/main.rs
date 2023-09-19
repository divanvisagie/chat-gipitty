use args::Args;
use chatgpt::GptClient;
use clap::Parser;
use cli::run;
use utils::{get_stdin, get_file_contents_from_path};

mod cli;
mod chatgpt;
mod utils;
mod args;

fn main() {
    let args = Args::parse();
    let mut client = GptClient::new();
    
    if let Some(query) = args.query.clone() {
        client.add_message(chatgpt::Role::User, query);
    }

    let content = get_stdin();
    if !content.is_empty() {
        client.add_message(chatgpt::Role::User, content);
    }
    
    if let Some(file) = args.file.clone() {
        let question = get_file_contents_from_path(file);
        client.add_message(chatgpt::Role::User, question);
    }
  
    run(&args, &mut client);
}
