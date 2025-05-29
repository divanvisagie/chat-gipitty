use std::str::FromStr;
use std::{env, fmt};

use dirs::config_dir;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::config_manager::ConfigManager;

fn get_completions_url(base_url: &str) -> String {
    format!("{}/v1/chat/completions", base_url)
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_options: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.role, self.content)
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
                content: prompt,
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
                content: system_prompt.clone(),
            }],
        }
    }

    pub fn add_message(&mut self, role: Role, text: String) -> &mut Self {
        self.messages.push(Message {
            role: role.to_string(),
            content: text.trim().to_string(),
        });
        self
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
        if self.messages.last().unwrap().content.to_lowercase().trim() == "ping" {
            self.add_message(Role::Assistant, "pong".to_string());
            return "pong".to_string();
        }

        // Check if the last user message starts with "/search"
        let mut use_search = false;
        if let Some(last_message) = self.messages.last_mut() {
            let trimmed_content = last_message.content.trim();
            if last_message.role == "user" && trimmed_content.starts_with("/search") {
                // Remove "/search" prefix from the message and trim whitespace
                last_message.content = trimmed_content.strip_prefix("/search").unwrap().trim().to_string();
                use_search = true;
            }
        }

        // Retrieve the API key from the environment variable
        let api_key =
            env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build client");

        let url = env::var("OPENAI_API_URL")
            .unwrap_or_else(|_| "https://api.openai.com".to_string());
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

        let model = if use_search {
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
                    panic!("Error while parsing error response object: {}\n{}", e, response_text);
                }
            }
        };

        let result = response_object.choices[0].message.content.clone();
        self.add_message(Role::Assistant, result.clone());
        result
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
            let trimmed_content = last_message.content.trim();
            if last_message.role == "user" && trimmed_content.starts_with("/search") {
                last_message.content = trimmed_content.strip_prefix("/search").unwrap().trim().to_string();
                use_search = true;
            }
        }
        
        assert!(use_search);
        assert_eq!(client.messages.last().unwrap().content, "what is the weather today?");
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
                last_message.content = trimmed_content.strip_prefix("/search").unwrap().trim().to_string();
                use_search = true;
            }
        }
        
        assert!(!use_search);
        assert_eq!(client.messages.last().unwrap().content, "what is the weather today?");
    }

    #[test]
    fn test_search_prefix_with_whitespace() {
        let mut client = GptClient::new(false);
        client.add_message(Role::User, "   /search   what is the weather today?   ".to_string());
        
        // Simulate the search detection logic
        let mut use_search = false;
        if let Some(last_message) = client.messages.last_mut() {
            let trimmed_content = last_message.content.trim();
            if last_message.role == "user" && trimmed_content.starts_with("/search") {
                last_message.content = trimmed_content.strip_prefix("/search").unwrap().trim().to_string();
                use_search = true;
            }
        }
        
        assert!(use_search);
        assert_eq!(client.messages.last().unwrap().content, "what is the weather today?");
    }
}
