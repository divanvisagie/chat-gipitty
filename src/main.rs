use args::Args;
use chatgpt::GptClient;
use clap::Parser;
use cli::run_cli_mode;
use utils::{get_stdin, get_file_contents_from_path};

use crate::tui::run_tui_mode;

mod cli;
mod tui;
mod chatgpt;
mod utils;
mod args;

fn main() {
    let args = Args::parse();
    let mut client = GptClient::new();

    let content = get_stdin();
    if !content.is_empty() {
        client.add_message(chatgpt::Role::User, content);
    }
    
    if let Some(file) = args.file.clone() {
        let question = get_file_contents_from_path(file);
        client.add_message(chatgpt::Role::User, question);
    }
    
    if let Some(question) = args.query.clone() {
        client.add_message(chatgpt::Role::User, question);
    }

    if args.interactive {
        run_tui_mode(&mut client).expect("Failed to run tui mode");
    } else {
        run_cli_mode(&args, &mut client);
    }
}
