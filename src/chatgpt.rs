use std::str::FromStr;
use std::{env, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use dirs::config_dir;
use reqwest::header;

use crate::config_manager::ConfigManager;

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
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
    pub id: String,
    pub object: String,
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

impl GptClient {
    pub fn new() -> Self {
        let config_directory = config_dir()
            .expect("Failed to find config directory")
            .join("cgip");

        let config_manager = ConfigManager::new(config_directory);

        let os = env::consts::OS;

        let system_prompt = format!(
            r#"
            You are a helpul command line assistant running in a terminal on {}, users can
            pass you the standard output from their command line and you will try and 
            help them debug their issues or answer questions. Since you are a command line tool,
            you write to standard out. So it is possible for your output to be directly executed
            in the shell if your output is piped to it.
        "#,
            os
        );

        GptClient {
            config_manager,
            messages: vec![Message {
                role: Role::System.to_string().to_lowercase(),
                content: system_prompt.trim().to_string(),
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
        // Retrieve the API key from the environment variable
        let api_key =
            env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

        let client = reqwest::blocking::Client::new();
        let url = "https://api.openai.com/v1/chat/completions";

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

        let chat_request = ChatRequest {
            model: self.config_manager.config.model.clone(),
            messages: self.messages.clone(),
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
            Err(e) => panic!("Error in response: {}", e),
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
                    panic!("Error while parsing response object: {}", e);
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
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    use crate::config_manager::AppConfig;

    #[test]
    fn test_custom_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir_path = temp_dir.path().join("cgip");
        let mut config_manager = ConfigManager::new(config_dir_path.clone());

        // Manually create a custom config
        let custom_config = AppConfig {
            model: "gpt-3.5-turbo".to_string(),
            show_progress: true,
            show_context: false,
            markdown: false,
        };

        // Serialize and save this custom config
        let config_path = config_dir_path.join("config.toml");
        let contents = toml::to_string(&custom_config).expect("Failed to serialize custom config");
        let mut file = File::create(&config_path).expect("Failed to open config file");
        file.write_all(contents.as_bytes()).expect("Failed to write custom config to file");

        // Reload config from file
        config_manager.config = ConfigManager::load_config(&config_dir_path);

        assert_eq!(config_manager.config.model, "gpt-3.5-turbo", "Model should be 'gpt-3.5-turbo'");
        assert_eq!(config_manager.config.show_progress, true, "show_progress should be true");
    }

    #[test]
    fn test_custom_config_with_missing() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir_path = temp_dir.path().join("cgip");
        let mut config_manager = ConfigManager::new(config_dir_path.clone());

        // Create a partial config file manually
        let config_path = config_dir_path.join("config.toml");
        let contents = "show_progress = true";
        let mut file = File::create(&config_path).expect("Failed to open config file");
        file.write_all(contents.as_bytes()).expect("Failed to write partial config to file");

        // Reload config from file
        config_manager.config = ConfigManager::load_config(&config_dir_path);

        assert_eq!(config_manager.config.model, "gpt-4", "Model should default to 'gpt-4'");
        assert_eq!(config_manager.config.show_progress, true, "show_progress should be true");
    }

    #[test]
    fn test_default_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir_path = temp_dir.path().join("cgip");
        let config_manager = ConfigManager::new(config_dir_path);

        assert_eq!(config_manager.config.model, "gpt-4", "Model should default to 'gpt-4'");
        assert_eq!(config_manager.config.show_progress, false, "show_progress should default to false");
    }


}
