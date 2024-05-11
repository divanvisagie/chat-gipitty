use std::str::FromStr;

use args::{Args, SubCommands, ConfigSubCommand};
use chatgpt::{GptClient, Message, Role};
use clap::Parser;
use cli::run;
use utils::{get_file_contents_from_path, get_stdin, markdown_from_messages, is_valid_yaml};

mod args;
mod chatgpt;
mod cli;
mod utils;

fn main() {

    let mut client = GptClient::new();
    let args = Args::parse();

    if let Some(SubCommands::Config(config_sc)) = &args.subcmd{
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
fn handle_config_subcommand(client: &mut chatgpt::GptClient, config_sc: &ConfigSubCommand) {
    if let Some(ref set) = config_sc.set {
        let parts: Vec<&str> = set.split('=').collect();
        if parts.len() == 2 {
            client.set_config_value(parts[0], parts[1]);
            println!("Configuration set successfully for {} to {}", parts[0], parts[1])
        } else {
            println!("Invalid format for setting configuration. Use key=value");
        }
        // Additional logic to set the configuration
    }
    if let Some(ref get) = config_sc.get {
        let value = client.get_config_value(get);
        println!("Configuration for {} is {}", get, value);
    }
}
