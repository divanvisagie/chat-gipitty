pub struct Message {
    pub role: String,
    pub message: String,
}

pub struct ChatListItem {
    pub name: String,
    pub messages: Vec<Message>,
}
