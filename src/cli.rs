use spinners::{Spinner, Spinners};

use crate::{args::Args, cache::save_to_tty_context, chatgpt::{GptClient, Message, Role}, utils::markdown_from_messages};

pub fn run(args: &Args, client: &mut GptClient) {
    let response_text: String;
    // Override model from config if it was provided in args
    if let Some(ref model) = args.model {
        client.config_manager.config.model = model.clone();
    }

    // List available models
    let models = vec!["gpt-3.5-turbo", "gpt-4", "gpt-4-turbo", "gpt-4o"];
    if args.list_models {
        for model in models {
            println!("{}", model);
        }
        return;
    }

    // Override show_progress from config if it was provided in args
    let show_progress = args.show_progress || client.config_manager.config.show_progress;

    if show_progress {
        let mut sp = Spinner::new(Spinners::Dots9, "Thinking...".into());
        response_text = client.complete();
        sp.stop();
        print!("\x1B[2K"); // Clear the current line
        print!("\r"); // Move the cursor to the beginning of the current line
    } else {
        response_text = client.complete();
    }

    let show_context = args.show_context || client.config_manager.config.show_context;
    let markdown = args.markdown || client.config_manager.config.markdown;

    if show_context {
        if markdown {
            let visible_messages = client
                .messages
                .iter()
                .cloned()
                .filter(|msg| msg.role != "system")
                .collect();
            let context = markdown_from_messages(visible_messages);

            println!("{}", context);
            return;
        }
        let context = client.to_yaml(true);
        println!("{}", context);
        return;
    }
    println!("{}", response_text);
    let message = Message {
        role: Role::Assistant.to_string().to_lowercase(),
        content: response_text,
    };
    let messages_to_save = vec![message];
    save_to_tty_context(messages_to_save);
}
