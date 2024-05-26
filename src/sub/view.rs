use crate::{chatgpt::Message, utils::markdown_from_messages};

pub fn run(messages: &Vec<Message>) {
    let visible_messages = messages
        .iter()
        .cloned()
        .filter(|msg| msg.role != "system")
        .collect();
    let md = markdown_from_messages(visible_messages);
    println!("{}", md);
}
