use dirs::config_dir;
use reqwest::header;
use serde::{Deserialize, Serialize};

use crate::config_manager::ConfigManager;

use super::{
    client::{LanguageModelClient, LanguageModelRequest},
    Message,
};

#[derive(Debug, Serialize)]
struct MistralRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_prompt: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct MistralResponse {
    pub choices: Vec<MistralChoice>,
}

#[derive(Debug, Deserialize)]
struct MistralChoice {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
struct MistralError {
    pub error: MistralErrorDetail,
}

#[derive(Debug, Deserialize)]
struct MistralErrorDetail {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
}

pub struct MistralClient {
    pub mistral_api_key: String,
}

impl MistralClient {
    pub fn new() -> Self {
        let config_directory = config_dir()
            .expect("Failed to find config directory")
            .join("cgip");

        let config_manager = ConfigManager::new(config_directory);

        let api_key = config_manager.config.mistral_api_key
            .expect("MISTRAL_API_KEY is not set in config or environment");

        MistralClient {
            mistral_api_key: api_key,
        }
    }

    fn parse_error(response_text: &str) -> String {
        if let Ok(error_response) = serde_json::from_str::<MistralError>(response_text) {
            return format!(
                "Mistral API Error ({}): {}", 
                error_response.error.error_type,
                error_response.error.message
            );
        }
        format!("Failed to parse error response. Raw response: {}", response_text)
    }
}

impl LanguageModelClient for MistralClient {
    fn complete(&mut self, req: &LanguageModelRequest) -> String {
        let client = reqwest::blocking::Client::new();
        let url = "https://api.mistral.ai/v1/chat/completions";

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let auth_header = match header::HeaderValue::from_str(&format!("Bearer {}", self.mistral_api_key)) {
            Ok(header) => header,
            Err(e) => panic!("Failed to create auth header: {}", e),
        };
        headers.insert(header::AUTHORIZATION, auth_header);

        let chat_request = MistralRequest {
            model: req.model.clone(),
            messages: req.messages.clone(),
            temperature: Some(0.7),
            max_tokens: Some(1024),
            top_p: None,
            random_seed: None,
            safe_prompt: None,
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

        let response_object: MistralResponse = match serde_json::from_str(&response_text) {
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

        response_object.choices
            .first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| panic!("No choices in response"))
    }
}