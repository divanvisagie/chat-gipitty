use spinners::{Spinner, Spinners};

use crate::{
    args::Args,
    chatgpt::{self, GptClient}, utils::get_stdin,
};

pub fn run_cli_mode(args: &Args, client: &mut GptClient) {
    let response_text: String;

    if !args.query.is_none() || !args.file.is_none() {
        // Only run the loop if not reading stdin
        if args.progress {
            let mut sp = Spinner::new(Spinners::Dots9, "Thinking...".into());
            response_text = client.complete();
            sp.stop();
            print!("\x1B[2K"); // Clear the current line
            print!("\r"); // Move the cursor to the beginning of the current line
        } else {
            response_text = client.complete();
        }
        println!("{}", response_text);

    } else {
        let content = get_stdin();
        client.add_message(chatgpt::Role::User, content);

        let response_text = client.complete();
        println!("{}", response_text);
    }
}
