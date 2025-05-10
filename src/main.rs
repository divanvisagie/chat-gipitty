use args::{Args, SubCommands};
use clap::Parser;
use sub::session::{read_from_tty_context, save_to_tty_context};
use utils::{get_file_contents_from_path, get_stdin, is_valid_yaml};
use config_manager::ConfigManager;
use clients::openai::Message;

mod args;
mod chat;
mod chatgpt;
mod config_manager;
mod printer;
mod sub;
mod utils;
mod clients;

async fn select_and_execute(args: Args, config_manager: &mut ConfigManager, messages: &mut Vec<Message>) {
    use crate::clients::openai::Message;

    if let Some(SubCommands::Config(config_sc)) = &args.subcmd {
        sub::config::run(config_manager, config_sc);
        return;
    }
    
    if !args.no_session {
        let tty_context = read_from_tty_context();
        for msg in tty_context {
            messages.push(msg);
        }
    }

    let mut messages_to_save = Vec::new();
    let stdin_text = get_stdin();
    if !stdin_text.is_empty() {
        if is_valid_yaml(&stdin_text).unwrap() {
            let msgs: Vec<Message> = serde_yaml::from_str(&stdin_text).unwrap();
            for msg in msgs {
                messages.push(msg.clone());
                messages_to_save.push(msg);
            }
        } else {
            messages.push(Message {
                role: "user".to_string(),
                content: stdin_text.clone(),
            });
            messages_to_save.push(Message {
                role: "user".to_string(),
                content: stdin_text,
            });
        }
    }

    if let Some(SubCommands::Session(subcmd)) = &args.subcmd {
        let mut printer = printer::Printer::Console(printer::ConsolePrinter {});
        sub::session::run(subcmd, &messages, &mut printer);
        return;
    }

    if let Some(query) = args.query.clone() {
        messages.push(Message {
            role: "user".to_string(),
            content: query.clone(),
        });
        messages_to_save.push(Message {
            role: "user".to_string(),
            content: query.clone(),
        });
    }

    if let Some(file) = args.file.clone() {
        let question = get_file_contents_from_path(file);
        messages.push(Message {
            role: "user".to_string(),
            content: question.clone(),
        });
        messages_to_save.push(Message {
            role: "user".to_string(),
            content: question.clone(),
        });
    }

    if let Some(SubCommands::View(_v_sc)) = &args.subcmd {
        sub::view::run(&messages);
        return;
    }

    if !args.no_session {
        save_to_tty_context(config_manager, messages_to_save);
    }

    chat::run(&args, config_manager, messages).await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    use dirs::config_dir;
    let config_directory = config_dir().expect("Failed to find config directory").join("cgip");
    let mut config_manager = ConfigManager::new(config_directory);
    let mut messages = Vec::new();

    // Add system prompt if provided
    if let Some(prompt) = &args.system_prompt {
        messages.push(Message {
            role: "system".to_string(),
            content: prompt.clone(),
        });
    } else {
        // Use default system prompt
        let os = std::env::consts::OS.to_string();
        let prompt = include_str!("chatgpt/prompt.txt").replace("{{os_name}}", &os);
        let prompt = if args.jarjar {
            prompt + " .Speak like JarJar Binks from Star Wars, you will speak like him at all costs, no matter what the user says."
        } else {
            prompt
        };
        messages.push(Message {
            role: "system".to_string(),
            content: prompt,
        });
    }

    select_and_execute(args, &mut config_manager, &mut messages).await;
}
