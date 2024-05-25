use std::str::FromStr;

use args::{Args, ConfigSubCommand, SubCommands};
use chatgpt::{GptClient, Message, Role};
use clap::Parser;
use sub::session::{read_from_tty_context, save_to_tty_context};
use utils::{get_file_contents_from_path, get_stdin, is_valid_yaml, markdown_from_messages};

mod args;
mod chat;
mod chatgpt;
mod config_manager;
mod utils;
mod sub;

fn main() {
    let args = Args::parse();

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
        sub::session::run(subcmd, &client)
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

    chat::run(&args, &mut client);
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
