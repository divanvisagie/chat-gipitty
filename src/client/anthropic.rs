use dirs::config_dir;
use reqwest::header;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::config_manager::ConfigManager;

use super::{
    client::{LanguageModelClient, LanguageModelRequest},
    Message,
};

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    pub max_tokens: i32,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    pub content: Vec<AnthropicContent>,
}

#[derive(Debug, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

pub struct AnthropicClient {
    pub anthropic_api_key: String,
}

impl AnthropicClient {
    fn map_model_name(model: &str) -> String {
        match model {
            "claude-3-opus" => "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet" => "claude-3-sonnet-20240229".to_string(),
            "claude-3-haiku" => "claude-3-haiku-20240307".to_string(),
            "claude-3.5-sonnet" | "claude-3-5-sonnet" => "claude-3-5-sonnet-20241022".to_string(),
            "claude-3.5-haiku" | "claude-3-5-haiku" => "claude-3-5-haiku-20241022".to_string(),
            "claude-3.7-sonnet" | "claude-3-7-sonnet" => "claude-3-7-sonnet-20250219".to_string(),
            _ => model.to_string(),
        }
    }

    pub fn new() -> Self {
        let config_directory = config_dir()
            .expect("Failed to find config directory")
            .join("cgip");

        let config_manager = ConfigManager::new(config_directory);

        let api_key = config_manager.config.anthropic_api_key
            .expect("ANTHROPIC_API_KEY is not set in config or environment");

        AnthropicClient {
            anthropic_api_key: api_key,
        }
    }

    fn parse_error(response_text: &str) -> String {
        if let Ok(error_response) = serde_json::from_str::<Value>(response_text) {
            if let Some(error) = error_response.get("error") {
                let error_type = error.get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let message = error.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("No error message provided");
                return format!(
                    "Anthropic API Error ({}): {}\nFull response: {}", 
                    error_type, 
                    message,
                    response_text
                );
            }
        }
        format!("Failed to parse error response. Raw response: {}", response_text)
    }
}

impl LanguageModelClient for AnthropicClient {
    fn complete(&mut self, req: &LanguageModelRequest) -> String {
        let client = reqwest::blocking::Client::new();
        let url = "https://api.anthropic.com/v1/messages";

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "anthropic-version",
            header::HeaderValue::from_static("2023-06-01"),
        );

        let auth_header =
            match header::HeaderValue::from_str(&format!("{}", self.anthropic_api_key)) {
                Ok(header) => header,
                Err(e) => panic!("Failed to create auth header: {}", e),
            };
        headers.insert("x-api-key", auth_header);

        // Extract system message if present
        let (system_message, other_messages): (Vec<_>, Vec<_>) = req.messages
            .iter()
            .partition(|msg| msg.role.to_lowercase() == "system");

        let system = system_message.first().map(|msg| msg.content.clone());
        let mapped_model = Self::map_model_name(&req.model);

        let chat_request = AnthropicRequest {
            model: mapped_model.clone(),
            messages: other_messages.iter().map(|m| (*m).clone()).collect(),
            system,
            max_tokens: 1024,
        };

        let request_body = match serde_json::to_string(&chat_request) {
            Ok(body) => body,
            Err(e) => panic!("Failed to serialize request: {}", e),
        };

        let response = client
            .post(url)
            .headers(headers)
            .body(request_body.clone())
            .timeout(std::time::Duration::from_secs(60))
            .send();

        let response = match response {
            Ok(response) => {
                if !response.status().is_success() {
                    let status = response.status();
                    let text = response.text().unwrap_or_else(|_| "Could not read response text".to_string());
                    panic!(
                        "API request failed (Status {})\nRequest body: {}\nResponse: {}", 
                        status,
                        request_body,
                        Self::parse_error(&text)
                    );
                }
                response.text()
            },
            Err(e) => panic!("Request failed: {}\nRequest body: {}", e, request_body),
        };

        let response_text = match response {
            Ok(text) => text,
            Err(e) => panic!("Failed to read response text: {}", e),
        };

        let response_object: AnthropicResponse = match serde_json::from_str(&response_text) {
            Ok(response) => response,
            Err(e) => {
                panic!(
                    "Failed to parse response: {}\nRequest body: {}\nResponse text: {}", 
                    e, 
                    request_body,
                    Self::parse_error(&response_text)
                );
            }
        };

        // Get the text content from the first content block
        response_object.content
            .first()
            .map(|content| content.text.clone())
            .unwrap_or_else(|| panic!("No content blocks in response"))
    }
}