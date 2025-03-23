use super::{openai::OpenAiClient, test::TestClient, Message, Role};

pub trait LanguageModelClient {
    fn complete(&mut self, request: &LanguageModelRequest) -> String;
}

pub struct LanguageModelRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

impl LanguageModelRequest {
    pub fn new(model: String) -> Self {
        Self {
            model,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, role: Role, content: String) -> &mut Self {
        self.messages.push(Message {
            role: role.to_string(),
            content,
        });
        return self;
    }

    pub fn to_yaml(&self, exclude_system: bool) -> String {
        let filtered_messages: Vec<Message> = if exclude_system {
            self.messages
                .iter()
                .filter(|msg| msg.role.to_lowercase() != "system")
                .cloned()
                .collect()
        } else {
            self.messages.clone()
        };

        serde_yaml::to_string(&filtered_messages).unwrap()
    }
}

pub struct SwitcherClient {}

impl SwitcherClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl LanguageModelClient for SwitcherClient {
    fn complete(&mut self, request: &LanguageModelRequest) -> String {
        match request.model.as_str() {
            "gpt-3.5-turbo" | "gpt-4" | "gpt-4-turbo" | "gpt-4o" => {
                let mut client = OpenAiClient::new();
                return client.complete(request);
            }
            "test" => {
                let mut client = TestClient::new();
                return client.complete(request);
            }
            _ => {
                let mut client = OpenAiClient::new();
                return client.complete(request);
            }
        }
    }
}
