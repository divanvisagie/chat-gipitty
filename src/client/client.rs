use super::{anthropic::AnthropicClient, openai::OpenAiClient, test::TestClient, mistral::MistralClient, Message, Role};

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

    fn map_model_name(model: &str) -> String {
        match model {
            // Claude 3 models
            "claude-37" | "claude-3-7-sonnet" | "claude-3-7" => "claude-3-7-sonnet-20250219",
            // Mistral models
            "mistral-tiny" => "mistral-tiny-latest",
            "mistral-small" => "mistral-small-latest",
            "mistral-medium" => "mistral-medium-latest",
            _ => model,
        }
        .to_string()
    }
}

impl LanguageModelClient for SwitcherClient {
    fn complete(&mut self, request: &LanguageModelRequest) -> String {
        let model = Self::map_model_name(&request.model);
        let mapped_request = LanguageModelRequest {
            model: model.clone(),
            messages: request.messages.clone(),
        };

        match model.as_str() {
            model if model.starts_with("gpt-") => {
                let mut client = OpenAiClient::new();
                return client.complete(&mapped_request);
            }
            // Match all Claude model versions
            model if model.starts_with("claude-") => {
                let mut client = AnthropicClient::new();
                return client.complete(&mapped_request);
            }
            // Match all Mistral model versions
            model if model.starts_with("mistral-") => {
                let mut client = MistralClient::new();
                return client.complete(&mapped_request);
            }
            "test" => {
                let mut client = TestClient::new();
                return client.complete(&mapped_request);
            }
            _ => {
                let mut client = OpenAiClient::new();
                return client.complete(&mapped_request);
            }
        }
    }
}
