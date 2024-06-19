use std::str::FromStr;

use args::{Args, SubCommands};
use chatgpt::{GptClient, Message, Role};
use clap::Parser;
use sub::session::{read_from_tty_context, save_to_tty_context};
use utils::{get_file_contents_from_path, get_stdin, is_valid_yaml};

mod args;
mod chat;
mod chatgpt;
mod config_manager;
mod printer;
mod sub;
mod utils;

fn select_and_execute(args: Args, client: &mut GptClient) {

    if let Some(SubCommands::Config(config_sc)) = &args.subcmd {
        sub::config::run(client, config_sc);
        return;
    }
    
    if !args.no_session {
        let tty_context = read_from_tty_context();
        for msg in tty_context {
            let role = Role::from_str(msg.role.as_str()).expect("Could not convert role");
            client.add_message(role, msg.content);
        }
    }

    let mut messages_to_save = Vec::new();
    let stdin_text = get_stdin();
    if !stdin_text.is_empty() {
        if is_valid_yaml(&stdin_text).unwrap() {
            let messages: Vec<Message> = serde_yaml::from_str(&stdin_text).unwrap();
            for msg in messages {
                let role = Role::from_str(msg.role.as_str()).expect("could not convert role");
                client.add_message(role, msg.clone().content);
                messages_to_save.push(msg);
            }
        } else {
            client.add_message(chatgpt::Role::User, stdin_text.clone());
            messages_to_save.push(Message {
                role: Role::User.to_string().to_lowercase(),
                content: stdin_text,
            });
        }
    }

    if let Some(SubCommands::Session(subcmd)) = &args.subcmd {
        let mut printer = printer::Printer::Console(printer::ConsolePrinter {});
        sub::session::run(subcmd, &client.messages, &mut printer);
        return;
    }

    if let Some(query) = args.query.clone() {
        client.add_message(chatgpt::Role::User, query.clone());
        // save message to context
        let message = Message {
            role: Role::User.to_string().to_lowercase(),
            content: query.clone(),
        };
        messages_to_save.push(message);
    }

    if let Some(file) = args.file.clone() {
        let question = get_file_contents_from_path(file);
        client.add_message(chatgpt::Role::User, question.clone());
        // save message to context
        let message = Message {
            role: Role::User.to_string().to_lowercase(),
            content: question.clone(),
        };
        messages_to_save.push(message);
    }

    if let Some(SubCommands::View(_v_sc)) = &args.subcmd {
        sub::view::run(&client.messages);
        return;
    }

    if !args.no_session {
        save_to_tty_context(&client.config_manager, messages_to_save);
    }

    chat::run(&args, client);
}

fn main() {
    let args = Args::parse();
    let mut client = match &args.system_prompt {
        Some(prompt) => GptClient::new_with_system_prompt(prompt.clone()),
        None => GptClient::new(),
    };

    select_and_execute(args, &mut client)
}
