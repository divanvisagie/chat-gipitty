use std::str::FromStr;

use args::Args;
use chatgpt::{GptClient, Message, Role};
use clap::Parser;
use cli::run;
use serde_yaml::Error;
use utils::{get_file_contents_from_path, get_stdin, markdown_from_messages};


mod args;
mod chatgpt;
mod cli;
mod utils;

fn is_valid_yaml(yaml_str: &str) -> Result<bool, Error> {
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
fn main() {
    let args = Args::parse();
    let mut client = GptClient::new();

    let stdin_text = get_stdin();
    if !stdin_text.is_empty() {
        if is_valid_yaml(&stdin_text).unwrap() {
            let messages: Vec<Message> = serde_yaml::from_str(&stdin_text).unwrap();
            for msg in messages {
                let role = Role::from_str(msg.role.as_str()).expect("could not convert role");
                client.add_message(role, msg.content);
            }
        } else {
            client.add_message(chatgpt::Role::User, stdin_text);
        }
    }

    if let Some(_) = args.subcmd { // If view mode
        let visible_messages = client.messages.iter().cloned().filter(|msg| msg.role != "system").collect();      
        let md = markdown_from_messages(visible_messages);
        println!("{}", md);
        return;
    }

    if let Some(query) = args.query.clone() {
        client.add_message(chatgpt::Role::User, query);
    }

    if let Some(file) = args.file.clone() {
        let question = get_file_contents_from_path(file);
        client.add_message(chatgpt::Role::User, question);
    }

    run(&args, &mut client);
}
