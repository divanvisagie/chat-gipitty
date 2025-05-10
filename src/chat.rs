use spinners::{Spinner, Spinners};

use crate::{
    args::Args,
    config_manager::ConfigManager,
    utils::markdown_from_messages,
};
use crate::clients::openai::{ModelInfo, Message, ChatRequest, get_completion_message};

pub async fn run(args: &Args, config_manager: &mut ConfigManager, messages: &mut Vec<Message>) {
    let response_text: String;

    // Override model from config if it was provided in args
    let model_name = if let Some(ref model) = args.model {
        config_manager.config.model = model.clone();
        model.clone()
    } else {
        config_manager.config.model.clone()
    };

    // List available models
    if args.list_models {
        let models = vec!["gpt-4o", "gpt-4.1", "o4-mini", "o3-mini", "o1"];
        for model in models {
            println!("{}", model);
        }
        return;
    }

    // Override show_progress from config if it was provided in args
    let show_progress = args.show_progress || config_manager.config.show_progress;

    let model_info = ModelInfo::new(model_name);
    let chat_request = ChatRequest {
        model: model_info.name.clone(),
        messages: messages.clone(),
    };

    if show_progress {
        let mut spinner = Spinner::new(Spinners::Dots9, "Thinking...".into());
        response_text = match get_completion_message(&model_info, &chat_request).await {
            Ok(resp) => resp.choices[0].message.content.clone(),
            Err(e) => format!("Error: {}", e),
        };
        spinner.stop();
        print!("\x1B[2K"); // Clear the current line
        print!("\r"); // Move the cursor to the beginning of the current line
    } else {
        response_text = match get_completion_message(&model_info, &chat_request).await {
            Ok(resp) => resp.choices[0].message.content.clone(),
            Err(e) => format!("Error: {}", e),
        };
    }

    let show_context = args.show_context || config_manager.config.show_context;
    let markdown = args.markdown || config_manager.config.markdown;

    if show_context {
        if markdown {
            let visible_messages = messages
                .iter()
                .cloned()
                .filter(|msg| msg.role != "system")
                .collect();
            let context = markdown_from_messages(visible_messages);

            println!("{}", context);
            return;
        }
        let context = serde_yaml::to_string(&messages.iter().filter(|msg| msg.role != "system").cloned().collect::<Vec<_>>()).unwrap();
        println!("{}", context);
        return;
    }
    println!("{}", response_text);
    let message = Message {
        role: "assistant".to_string(),
        content: response_text,
    };
    let messages_to_save = vec![message.clone()];
    messages.push(message);

    if !args.no_session {
        crate::sub::session::save_to_tty_context(config_manager, messages_to_save);
    }
}
