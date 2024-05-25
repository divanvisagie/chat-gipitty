use std::str::FromStr;

use args::{Args, ConfigSubCommand, SubCommands};
use chat::run;
use chatgpt::{GptClient, Message, Role};
use clap::Parser;
use session::{delete_tty_context, read_from_tty_context, save_to_tty_context};
use utils::{get_file_contents_from_path, get_stdin, is_valid_yaml, markdown_from_messages};

mod args;
mod chat;
mod chatgpt;
mod config_manager;
mod session;
mod utils;

fn main() {
    let args = Args::parse();

    if let Some(SubCommands::Session(session_sc)) = &args.subcmd {
        if session_sc.clear {
            delete_tty_context();
            return;
        }
    }

    let mut client = GptClient::new();
    if let Some(SubCommands::Config(config_sc)) = &args.subcmd {
        handle_config_subcommand(&mut client, config_sc);
        return;
    }

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

    if let Some(SubCommands::View(_v_sc)) = &args.subcmd {
        // If view mode
        let visible_messages = client
            .messages
            .iter()
            .cloned()
            .filter(|msg| msg.role != "system")
            .collect();
        let md = markdown_from_messages(visible_messages);
        println!("{}", md);
        return;
    }

    let tty_context = read_from_tty_context();
    for msg in tty_context {
        let role = Role::from_str(msg.role.as_str()).expect("could not convert role");
        client.add_message(role, msg.content);
    }

    if let Some(SubCommands::Session(subcmd)) = &args.subcmd {
        if subcmd.view {
            let visible_messages: Vec<Message> = client
                .messages
                .iter()
                .cloned()
                .filter(|msg| msg.role != "system")
                .collect();

            for msg in visible_messages {
                let role = Role::from_str(msg.role.as_str()).expect("could not convert role");
                let role_str = role.to_string();
                let content = msg.content;
                if role_str == "user" {
                    println!("\x1b[1;34m{}\x1b[0m: {}", role_str, content);
                } else {
                    println!("\x1b[1;31m{}\x1b[0m: {}", role_str, content);
                }
            }
            //println!("{}", md);
            return;
        }
    }

    let mut messages_to_save = Vec::new();
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

    save_to_tty_context(&client.config_manager, messages_to_save);

    run(&args, &mut client);
}

fn handle_config_subcommand(client: &mut chatgpt::GptClient, config_subcommand: &ConfigSubCommand) {
    if let Some(ref set) = config_subcommand.set {
        let parts: Vec<&str> = set.split('=').collect();
        if parts.len() == 2 {
            client.config_manager.set_config_value(parts[0], parts[1]);
            println!(
                "Configuration set successfully for {} to {}",
                parts[0], parts[1]
            )
        } else {
            println!("Invalid format for setting configuration. Use cgip config --set key=value");
        }
    }
    if let Some(ref get) = config_subcommand.get {
        let value = client.config_manager.get_config_value(get);
        println!("Configuration for {} is {}", get, value);
    }
}
