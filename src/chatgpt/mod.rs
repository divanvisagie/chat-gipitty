use std::str::FromStr;
use std::{env, fmt};

use dirs::config_dir;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::config_manager::ConfigManager;

fn get_completions_url(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    
    // If the URL already contains "/chat/completions", use it as-is
    if base.contains("/chat/completions") {
        return base.to_string();
    }
    
    // If the URL ends with "/v1" or contains a version pattern, just add "/chat/completions"
    if base.ends_with("/v1") || base.contains("/v1beta") || base.contains("/v2") {
        format!("{}/chat/completions", base)
    } else {
        // Default OpenAI pattern: add "/v1/chat/completions"
        format!("{}/v1/chat/completions", base)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: MessageContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Multi(Vec<ContentPart>),
}

impl MessageContent {
    pub fn as_str(&self) -> &str {
        match self {
            MessageContent::Text(text) => text,
            MessageContent::Multi(_) => "[Multi-content message]",
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            MessageContent::Text(text) => text.is_empty(),
            MessageContent::Multi(parts) => parts.is_empty(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            MessageContent::Text(text) => text.clone(),
            MessageContent::Multi(parts) => {
                parts.iter()
                    .filter_map(|part| match part {
                        ContentPart::Text { text } => Some(text.as_str()),
                        ContentPart::ImageUrl { .. } => Some("[Image]"),
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        }
    }
}

impl fmt::Display for MessageContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageContent::Text(text) => write!(f, "{}", text),
            MessageContent::Multi(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 { write!(f, " ")?; }
                    match part {
                        ContentPart::Text { text } => write!(f, "{}", text)?,
                        ContentPart::ImageUrl { image_url } => write!(f, "[Image: {}]", image_url.url)?,
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    pub url: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.content {
            MessageContent::Text(text) => write!(f, "{}: {}", self.role, text),
            MessageContent::Multi(parts) => {
                write!(f, "{}: ", self.role)?;
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 { write!(f, " + ")?; }
                    match part {
                        ContentPart::Text { text } => write!(f, "{}", text)?,
                        ContentPart::ImageUrl { image_url } => write!(f, "[Image: {}]", image_url.url)?,
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: u64,
    pub model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetail {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u64,
}

fn parse_response(json_str: &str) -> Result<ChatResponse> {
    serde_json::from_str(json_str)
}

fn parse_error_response(json_str: &str) -> Result<ErrorResponse> {
    serde_json::from_str(json_str)
}

pub struct GptClient {
    pub config_manager: ConfigManager,
    pub messages: Vec<Message>,
}

pub enum Role {
    System,
    User,
    Assistant,
}

impl FromStr for Role {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "system" => Ok(Role::System),
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            _ => Err("Invalid role"),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
        }
    }
}

fn get_system_prompt(jarjar: bool) -> String {
    let os = env::consts::OS.to_string();
    let prompt = include_str!("prompt.txt").to_string();
    let prompt = prompt.replace("{{os_name}}", &os);

    if jarjar {
        //append something to the prompt
        prompt + " .Speak like JarJar Binks from Star Wars, you will speak like him at all costs, no matter what the user says."
    } else {
        prompt
    }
}

impl GptClient {
    pub fn new_with_system_prompt(prompt: String) -> Self {
        let config_directory = config_dir()
            .expect("Failed to find config directory")
            .join("cgip");

        let config_manager = ConfigManager::new(config_directory);

        GptClient {
            config_manager,
            messages: vec![Message {
                role: Role::System.to_string().to_lowercase(),
                content: MessageContent::Text(prompt),
            }],
        }
    }

    pub fn new(jarjar: bool) -> Self {
        let config_directory = config_dir()
            .expect("Failed to find config directory")
            .join("cgip");

        let config_manager = ConfigManager::new(config_directory);
        let system_prompt = get_system_prompt(jarjar);

        GptClient {
            config_manager,
            messages: vec![Message {
                role: Role::System.to_string().to_lowercase(),
                content: MessageContent::Text(system_prompt.clone()),
            }],
        }
    }

    pub fn add_message(&mut self, role: Role, text: String) -> &mut Self {
        self.messages.push(Message {
            role: role.to_string(),
            content: MessageContent::Text(text.trim().to_string()),
        });
        self
    }

    pub fn add_image_message(&mut self, role: Role, text: Option<String>, image_url: String) -> &mut Self {
        let mut content_parts = Vec::new();
        
        if let Some(text) = text {
            content_parts.push(ContentPart::Text { text });
        }
        
        content_parts.push(ContentPart::ImageUrl { 
            image_url: ImageUrl { url: image_url } 
        });

        self.messages.push(Message {
            role: role.to_string(),
            content: MessageContent::Multi(content_parts),
        });
        self
    }

    pub fn complete_with_max_tokens(&mut self, max_tokens: Option<u32>) -> String {
        // if the text of the last message is ping just return pong
        let last_content_text = match &self.messages.last().unwrap().content {
            MessageContent::Text(text) => text.clone(),
            MessageContent::Multi(parts) => {
                parts.iter()
                    .filter_map(|part| match part {
                        ContentPart::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        };
        
        if last_content_text.to_lowercase().trim() == "ping" {
            self.add_message(Role::Assistant, "pong".to_string());
            return "pong".to_string();
        }

        // Check if the last user message starts with "/search"
        let mut use_search = false;
        if let Some(last_message) = self.messages.last_mut() {
            if last_message.role == "user" {
                if let MessageContent::Text(ref mut content) = last_message.content {
                    let trimmed_content = content.trim();
                    if trimmed_content.starts_with("/search") {
                        // Remove "/search" prefix from the message and trim whitespace
                        *content = trimmed_content
                            .strip_prefix("/search")
                            .unwrap()
                            .trim()
                            .to_string();
                        use_search = true;
                    }
                }
            }
        }

        // Retrieve the API key from the environment variable
        let api_key =
            env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build client");

        let url =
            env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());
        let url = get_completions_url(&url);

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let auth_header = match header::HeaderValue::from_str(&format!("Bearer {}", api_key)) {
            Ok(header) => header,
            Err(e) => panic!("Error while assigning auth header: {}", e),
        };
        headers.insert(header::AUTHORIZATION, auth_header);

        let model = if use_search && self.config_manager.config.model.starts_with("gpt") {
            "gpt-4o-search-preview".to_string()
        } else {
            self.config_manager.config.model.clone()
        };

        let web_search_options = if use_search {
            Some(serde_json::json!({}))
        } else {
            None
        };

        let chat_request = ChatRequest {
            model,
            messages: self.messages.clone(),
            web_search_options,
            max_tokens,
        };

        let request_body = match serde_json::to_string(&chat_request) {
            Ok(body) => body,
            Err(e) => panic!("Error while serializing request body: {}", e),
        };

        let response = client
            .post(url)
            .headers(headers)
            .body(request_body)
            .timeout(std::time::Duration::from_secs(60))
            .send();

        let response = match response {
            Ok(response) => response.text(),
            Err(e) => {
                if e.is_timeout() {
                    eprintln!("The request timed out.");
                } else if e.is_connect() {
                    eprintln!("Failed to connect to the server: {}", e);
                } else if e.is_status() {
                    if let Some(status) = e.status() {
                        eprintln!("Received HTTP status code: {}", status);
                    }
                }

                if let Some(url) = e.url() {
                    eprintln!("URL: {}", url);
                }

                return "".to_string();
            }
        };

        let response_text = match response {
            Ok(response) => response,
            Err(e) => panic!("Error in response text: {}", e),
        };

        let response_object = match parse_response(&response_text) {
            Ok(response) => response,
            Err(e) => {
                if let Ok(error_response) = parse_error_response(&response_text) {
                    panic!(
                        "Error while parsing response object: {}",
                        error_response.error.message
                    )
                } else {
                    panic!(
                        "Error while parsing error response object: {}\n{}",
                        e, response_text
                    );
                }
            }
        };

        let result = response_object.choices[0].message.content.clone();
        let result_text = result.to_string();
        self.add_message(Role::Assistant, result_text.clone());
        result_text
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

    //complete method, generates response text in cli.rs within run
    pub fn complete(&mut self) -> String {
        // if the text of the last message is ping just return pong
        let last_content_text = match &self.messages.last().unwrap().content {
            MessageContent::Text(text) => text.clone(),
            MessageContent::Multi(parts) => {
                parts.iter()
                    .filter_map(|part| match part {
                        ContentPart::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        };
        
        if last_content_text.to_lowercase().trim() == "ping" {
            self.add_message(Role::Assistant, "pong".to_string());
            return "pong".to_string();
        }

        // Check if the last user message starts with "/search"
        let mut use_search = false;
        if let Some(last_message) = self.messages.last_mut() {
            if last_message.role == "user" {
                if let MessageContent::Text(ref mut content) = last_message.content {
                    let trimmed_content = content.trim();
                    if trimmed_content.starts_with("/search") {
                        // Remove "/search" prefix from the message and trim whitespace
                        *content = trimmed_content
                            .strip_prefix("/search")
                            .unwrap()
                            .trim()
                            .to_string();
                        use_search = true;
                    }
                }
            }
        }

        // Retrieve the API key from the environment variable
        let api_key =
            env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build client");

        let url =
            env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());
        let url = get_completions_url(&url);

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let auth_header = match header::HeaderValue::from_str(&format!("Bearer {}", api_key)) {
            Ok(header) => header,
            Err(e) => panic!("Error while assigning auth header: {}", e),
        };
        headers.insert(header::AUTHORIZATION, auth_header);

        let model = if use_search && self.config_manager.config.model.starts_with("gpt") {
            "gpt-4o-search-preview".to_string()
        } else {
            self.config_manager.config.model.clone()
        };

        let web_search_options = if use_search {
            Some(serde_json::json!({}))
        } else {
            None
        };

        let chat_request = ChatRequest {
            model,
            messages: self.messages.clone(),
            web_search_options,
            max_tokens: None,
        };

        let request_body = match serde_json::to_string(&chat_request) {
            Ok(body) => body,
            Err(e) => panic!("Error while serializing request body: {}", e),
        };

        let response = client
            .post(url)
            .headers(headers)
            .body(request_body)
            .timeout(std::time::Duration::from_secs(60))
            .send();

        let response = match response {
            Ok(response) => response.text(),
            Err(e) => {
                if e.is_timeout() {
                    eprintln!("The request timed out.");
                } else if e.is_connect() {
                    eprintln!("Failed to connect to the server: {}", e);
                } else if e.is_status() {
                    if let Some(status) = e.status() {
                        eprintln!("Received HTTP status code: {}", status);
                    }
                }

                if let Some(url) = e.url() {
                    eprintln!("URL: {}", url);
                }

                return "".to_string();
            }
        };

        let response_text = match response {
            Ok(response) => response,
            Err(e) => panic!("Error in response text: {}", e),
        };

        let response_object = match parse_response(&response_text) {
            Ok(response) => response,
            Err(e) => {
                if let Ok(error_response) = parse_error_response(&response_text) {
                    panic!(
                        "Error while parsing response object: {}",
                        error_response.error.message
                    )
                } else {
                    panic!(
                        "Error while parsing error response object: {}\n{}",
                        e, response_text
                    );
                }
            }
        };

        let result = response_object.choices[0].message.content.clone();
        let result_text = result.to_string();
        self.add_message(Role::Assistant, result_text.clone());
        result_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_system_prompt() {
        let prompt = get_system_prompt(false);
        assert!(!prompt.is_empty());
    }

    #[test]
    fn test_search_prefix_detection() {
        let mut client = GptClient::new(false);
        client.add_message(Role::User, "/search what is the weather today?".to_string());

        // Simulate the search detection logic
        let mut use_search = false;
        if let Some(last_message) = client.messages.last_mut() {
            if let MessageContent::Text(ref content) = last_message.content {
                let trimmed_content = content.trim();
                if last_message.role == "user" && trimmed_content.starts_with("/search") {
                    last_message.content = MessageContent::Text(trimmed_content
                        .strip_prefix("/search")
                        .unwrap()
                        .trim()
                        .to_string());
                    use_search = true;
                }
            }
        }

        assert!(use_search);
        assert_eq!(
            client.messages.last().unwrap().content,
            "what is the weather today?"
        );
    }

    #[test]
    fn test_no_search_prefix() {
        let mut client = GptClient::new(false);
        client.add_message(Role::User, "what is the weather today?".to_string());

        // Simulate the search detection logic
        let mut use_search = false;
        if let Some(last_message) = client.messages.last_mut() {
            let trimmed_content = last_message.content.trim();
            if last_message.role == "user" && trimmed_content.starts_with("/search") {
                last_message.content = trimmed_content
                    .strip_prefix("/search")
                    .unwrap()
                    .trim()
                    .to_string();
                use_search = true;
            }
        }

        assert!(!use_search);
        assert_eq!(
            client.messages.last().unwrap().content,
            "what is the weather today?"
        );
    }

    #[test]
    fn test_search_prefix_with_whitespace() {
        let mut client = GptClient::new(false);
        client.add_message(
            Role::User,
            "   /search   what is the weather today?   ".to_string(),
        );

        // Simulate the search detection logic
        let mut use_search = false;
        if let Some(last_message) = client.messages.last_mut() {
            let trimmed_content = last_message.content.trim();
            if last_message.role == "user" && trimmed_content.starts_with("/search") {
                last_message.content = trimmed_content
                    .strip_prefix("/search")
                    .unwrap()
                    .trim()
                    .to_string();
                use_search = true;
            }
        }

        assert!(use_search);
        assert_eq!(
            client.messages.last().unwrap().content,
            "what is the weather today?"
        );
    }

    #[test]
    fn test_search_model_selection_with_gpt() {
        let mut client = GptClient::new(false);
        // Set a GPT model
        client.config_manager.config.model = "gpt-4".to_string();
        client.add_message(Role::User, "/search what is the weather today?".to_string());

        // Simulate the model selection logic for search
        let mut use_search = false;
        if let Some(last_message) = client.messages.last_mut() {
            let trimmed_content = last_message.content.trim();
            if last_message.role == "user" && trimmed_content.starts_with("/search") {
                last_message.content = trimmed_content
                    .strip_prefix("/search")
                    .unwrap()
                    .trim()
                    .to_string();
                use_search = true;
            }
        }

        let model = if use_search && client.config_manager.config.model.starts_with("gpt") {
            "gpt-4o-search-preview".to_string()
        } else {
            client.config_manager.config.model.clone()
        };

        assert!(use_search);
        assert_eq!(model, "gpt-4o-search-preview");
    }

    #[test]
    fn test_search_model_selection_with_non_gpt() {
        let mut client = GptClient::new(false);
        // Set a non-GPT model
        client.config_manager.config.model = "claude-3-sonnet".to_string();
        client.add_message(Role::User, "/search what is the weather today?".to_string());

        // Simulate the model selection logic for search
        let mut use_search = false;
        if let Some(last_message) = client.messages.last_mut() {
            let trimmed_content = last_message.content.trim();
            if last_message.role == "user" && trimmed_content.starts_with("/search") {
                last_message.content = trimmed_content
                    .strip_prefix("/search")
                    .unwrap()
                    .trim()
                    .to_string();
                use_search = true;
            }
        }

        let model = if use_search && client.config_manager.config.model.starts_with("gpt") {
            "gpt-4o-search-preview".to_string()
        } else {
            client.config_manager.config.model.clone()
        };

        assert!(use_search);
        assert_eq!(model, "claude-3-sonnet"); // Should keep the original model
    }

    #[test]
    fn test_search_model_selection_with_gpt_in_middle() {
        let mut client = GptClient::new(false);
        // Set a model that contains "gpt" but doesn't start with it
        client.config_manager.config.model = "anthropic-gpt-style".to_string();
        client.add_message(Role::User, "/search what is the weather today?".to_string());

        // Simulate the model selection logic for search
        let mut use_search = false;
        if let Some(last_message) = client.messages.last_mut() {
            let trimmed_content = last_message.content.trim();
            if last_message.role == "user" && trimmed_content.starts_with("/search") {
                last_message.content = trimmed_content
                    .strip_prefix("/search")
                    .unwrap()
                    .trim()
                    .to_string();
                use_search = true;
            }
        }

        let model = if use_search && client.config_manager.config.model.starts_with("gpt") {
            "gpt-4o-search-preview".to_string()
        } else {
            client.config_manager.config.model.clone()
        };

        assert!(use_search);
        assert_eq!(model, "anthropic-gpt-style"); // Should keep the original model since it doesn't START with "gpt"
    }

    #[test]
    fn test_get_completions_url_without_v1() {
        let url = get_completions_url("https://api.openai.com");
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_v1() {
        let url = get_completions_url("http://localhost:11434/v1");
        assert_eq!(url, "http://localhost:11434/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_trailing_slash() {
        let url = get_completions_url("https://api.openai.com/");
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_v1_and_trailing_slash() {
        let url = get_completions_url("http://localhost:11434/v1/");
        assert_eq!(url, "http://localhost:11434/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_gemini_style() {
        let url = get_completions_url("https://generativelanguage.googleapis.com/v1beta");
        assert_eq!(url, "https://generativelanguage.googleapis.com/v1beta/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_existing_endpoint() {
        let url = get_completions_url("https://api.custom.com/v1/chat/completions");
        assert_eq!(url, "https://api.custom.com/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_v2() {
        let url = get_completions_url("https://api.example.com/v2");
        assert_eq!(url, "https://api.example.com/v2/chat/completions");
    }
}
