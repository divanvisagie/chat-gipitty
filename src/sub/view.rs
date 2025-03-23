use crate::{sub::session::read_from_tty_context, utils::markdown_from_messages};

pub fn run() {
    let messages = read_from_tty_context();
    let visible_messages = messages
        .iter()
        .cloned()
        .filter(|msg| msg.role != "system")
        .collect();
    let md = markdown_from_messages(visible_messages);
    println!("{}", md);
}
