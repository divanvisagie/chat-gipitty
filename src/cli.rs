use spinners::{Spinner, Spinners};

use crate::{
    args::Args,
    chatgpt::GptClient, utils::markdown_from_messages,
};

pub fn run(args: &Args, client: &mut GptClient) {
    let response_text: String;

    let models = vec!["gpt-3.5-turbo", "gpt-4", "gpt-4-turbo"];

    if args.list_models {
        for model in models {
            println!("{}", model);
        }
        return;
    }

    let model_to_use = args.model.as_deref().unwrap_or("gpt-4");

    if args.show_progress {
        let mut sp = Spinner::new(Spinners::Dots9, "Thinking...".into());
        response_text = client.complete(model_to_use);
        sp.stop();
        print!("\x1B[2K"); // Clear the current line
        print!("\r"); // Move the cursor to the beginning of the current line
    } else {
        response_text = client.complete(model_to_use);
    }

    if args.show_context {
        if args.markdown {
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
}
