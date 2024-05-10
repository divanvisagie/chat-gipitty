use config::{Config, File as ConfigFile, FileFormat};
use dirs::config_dir;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, fmt};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub model: String,
    pub show_progress: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            show_progress: false,
        }
    }
}

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
    pub config: AppConfig,
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
    pub fn setup_config(config_dir: &PathBuf) {
        let cgip_config_dir = config_dir.join("cgip");
        if !cgip_config_dir.exists() {
            fs::create_dir_all(&cgip_config_dir).expect("Failed to create cgip config directory");
        }

        let config_path = cgip_config_dir.join("config.toml");
        if !config_path.exists() {
            let config = AppConfig::default();
            let contents = toml::to_string(&config).expect("Failed to serialize config");
            let mut file = File::create(&config_path).expect("Failed to create config file");
            file.write_all(contents.as_bytes())
                .expect("Failed to write to config file");
        }
    }

    pub fn load_config(config_dir: &PathBuf) -> AppConfig {
        let config_path = config_dir.join("cgip").join("config.toml");
        let defaults = Config::try_from(&AppConfig::default()).unwrap();
        let config = Config::builder()
            .add_source(defaults)
            .add_source(ConfigFile::new(
                config_path.to_str().unwrap(),
                FileFormat::Toml,
            ))
            .build()
            .unwrap();
        let loaded_config = config
            .try_deserialize::<AppConfig>()
            .expect("Failed to deserialize config");

        loaded_config
    }

    pub fn new() -> Self {
        let config_directory = config_dir().expect("Failed to find config directory");
        Self::setup_config(&config_directory);

        let config = Self::load_config(&config_directory);

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
            config,
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
            model: self.config.model.clone(),
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
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_dir_path = temp_dir.path().join("cgip");
        let config_dir_path_buf = PathBuf::from(&config_dir_path);

        assert!(
            !config_dir_path_buf.join("config.toml").exists(),
            "Config file should not exist for this test"
        );

        GptClient::setup_config(&config_dir_path_buf);
        let config = GptClient::load_config(&config_dir_path_buf);

        assert_eq!(config.model, "gpt-4", "Model should default to 'gpt-4'");
        assert_eq!(
            config.show_progress, false,
            "show_progress should default to false"
        );
    }
}
