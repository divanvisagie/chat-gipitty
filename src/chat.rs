use spinners::{Spinner, Spinners};

use crate::{
    args::Args,
    chatgpt::{GptClient, Message, Role},
    sub::session::save_to_tty_context,
    utils::markdown_from_messages,
};

pub fn run(args: &Args, client: &mut GptClient) {
    let response_text: String;

    // TODO: Think about whether it is better to be passing this config around
    // or go with someting more functional
    //
    // Override model from config if it was provided in args
    if let Some(ref model) = args.model {
        client.config_manager.config.model = model.clone();
    }

    // List available models
    if args.list_models {
        let models = vec!["gpt-4o", "gpt-4.1", "o4-mini", "o3-mini", "o1"];
        for model in models {
            println!("{}", model);
        }
        return;
    }

    // Override show_progress from config if it was provided in args
    let show_progress = args.show_progress || client.config_manager.config.show_progress;

    if show_progress {
        let mut spinner = Spinner::new(Spinners::Dots9, "Thinking...".into());
        response_text = client.complete();
        spinner.stop();
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

    if !args.no_session {
        save_to_tty_context(&client.config_manager, messages_to_save);
    }
}
