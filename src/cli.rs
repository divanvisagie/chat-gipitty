use spinners::{Spinner, Spinners};

use crate::{
    args::Args,
    chatgpt::{GptClient, Message},
};

fn format_messages_as_table(messages: &Vec<Message>) -> String {
    let mut output = String::from("Role       Content\n");

    for msg in messages {
        let role = format!("{: <10}", msg.role);
        let mut lines = msg.content.split('\n').peekable();
        if msg.role == "system" { // Skip system messages
            continue;
        }
        if let Some(first_line) = lines.next() {
            output.push_str(&format!("{} {}\n", role, first_line));
        }
        
        for line in lines {
            output.push_str(&format!("{: <10} {}\n", "", line));
        }
    }

    output
}


pub fn run(args: &Args, client: &mut GptClient) {
    let response_text: String;

    if args.progress {
        let mut sp = Spinner::new(Spinners::Dots9, "Thinking...".into());
        response_text = client.complete();
        sp.stop();
        print!("\x1B[2K"); // Clear the current line
        print!("\r"); // Move the cursor to the beginning of the current line
    } else {
        response_text = client.complete();
    }

    if args.show_context { 
        let context = format_messages_as_table(&client.messages);
        println!("{}", context);
        return;
    }
    println!("{}", response_text);
}
