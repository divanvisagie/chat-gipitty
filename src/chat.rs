use std::str::FromStr;

use dirs::config_dir;
use spinners::{Spinner, Spinners};

use crate::client::client::{LanguageModelRequest, SwitcherClient};
use crate::client::get_system_prompt;
use crate::config_manager::ConfigManager;
use crate::sub::session::read_from_tty_context;
use crate::utils::{get_file_contents_from_path, get_stdin, is_valid_yaml};
use crate::{
    args::Args,
    client::{Message, Role},
    sub::session::save_to_tty_context,
    utils::markdown_from_messages,
};
use crate::{client, LanguageModelClient};

fn should_read_from_session(args: &Args) -> bool {
    !args.no_session
}

fn build_request_message_context(args: &Args) -> Vec<Message> {
    let mut messages = Vec::new();
    if should_read_from_session(args) {
        let tty_context = read_from_tty_context();
        for msg in tty_context {
            let role = Role::from_str(msg.role.as_str()).expect("Could not convert role");
            messages.push(Message {
                role: role.to_string().to_lowercase(),
                content: msg.content,
            });
        }
    }

    if let Some(file) = args.file.clone() {
        let question = get_file_contents_from_path(file);
        let message = Message {
            role: Role::User.to_string().to_lowercase(),
            content: question.clone(),
        };
        messages.push(message);
    }

    let stdin_text = get_stdin();
    if !stdin_text.is_empty() {
        if is_valid_yaml(&stdin_text).unwrap() {
            let messages_from_yaml: Vec<Message> = serde_yaml::from_str(&stdin_text).unwrap();
            for msg in messages_from_yaml {
                messages.push(msg);
            }
        } else {
            messages.push(Message {
                role: Role::User.to_string().to_lowercase(),
                content: stdin_text,
            });
        }
    }

    if let Some(query) = args.query.clone() {
        messages.push(Message {
            role: Role::User.to_string().to_lowercase(),
            content: query,
        });
    }

    messages
}

pub fn run(args: &Args) {
    if args.list_models {
        let models = vec!["gpt-3.5-turbo", "gpt-4", "gpt-4-turbo", "gpt-4o"];
        for model in models {
            println!("{}", model);
        }
        return;
    }

    let response_text: String;

    let config_directory = config_dir()
        .expect("Failed to find config directory")
        .join("cgip");

    let config_manager = ConfigManager::new(config_directory);
    let show_progress = args.show_progress || config_manager.config.show_progress;
    let model = args
        .model
        .clone()
        .unwrap_or(config_manager.config.model.clone());
    let mut client = SwitcherClient::new();
    let mut request = LanguageModelRequest::new(model);

    if let Some(system_prompt) = args.system_prompt.clone() {
        request.add_message(Role::System, system_prompt);
    } else {
        let sp = get_system_prompt();
        request.add_message(Role::System, sp);
    }
    
    let messages = build_request_message_context(args);
    for msg in messages {
        request.add_message(Role::User, msg.content);
    }

    if let Some(query) = args.query.clone() {
        request.add_message(client::Role::User, query.clone());
    }

    if show_progress {
        let mut spinner = Spinner::new(Spinners::Dots9, "Thinking...".into());
        response_text = client.complete(&request);
        spinner.stop();
        print!("\x1B[2K"); // Clear the current line
        print!("\r"); // Move the cursor to the beginning of the current line
    } else {
        response_text = client.complete(&request);
    }

    let show_context = args.show_context || config_manager.config.show_context;
    let markdown = args.markdown || config_manager.config.markdown;

    if show_context {
        if markdown {
            let visible_messages = request
                .messages
                .iter()
                .cloned()
                .filter(|msg| msg.role != "system")
                .collect();
            let context = markdown_from_messages(visible_messages);

            println!("{}", context);
            return;
        }
        let context = request.to_yaml(true);
        println!("{}", context);
        return;
    }

    println!("{}", response_text);
    let response_message = Message {
        role: Role::Assistant.to_string().to_lowercase(),
        content: response_text,
    };

    if !args.no_session {
        let mut messages_to_save = Vec::new();
        for msg in request.messages {
            if msg.role == Role::System.to_string().to_lowercase() {
                continue;
            }
            messages_to_save.push(Message {
                role: Role::User.to_string().to_lowercase(),
                content: msg.content,
            });
        }
        messages_to_save.push(response_message);

        save_to_tty_context(&config_manager, messages_to_save);
    }
}
