use spinners::{Spinner, Spinners};

use crate::{
    args::Args,
    chatgpt::GptClient,
};

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
    println!("{}", response_text);
}
