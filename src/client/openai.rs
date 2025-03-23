use dirs::config_dir;
use reqwest::header;

use crate::config_manager::ConfigManager;

use super::{
    client::{LanguageModelClient, LanguageModelRequest},
    parse_error_response, parse_response, ChatRequest,
};

pub struct OpenAiClient {
    pub open_ai_api_key: String,
}

impl LanguageModelClient for OpenAiClient {

    fn complete(&mut self, req: &LanguageModelRequest) -> String {
        let client = reqwest::blocking::Client::new();
        let url = "https://api.openai.com/v1/chat/completions";

        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let auth_header =
            match header::HeaderValue::from_str(&format!("Bearer {}", self.open_ai_api_key)) {
                Ok(header) => header,
                Err(e) => panic!("Error while assigning auth header: {}", e),
            };
        headers.insert(header::AUTHORIZATION, auth_header);

        let chat_request = ChatRequest {
            model: req.model.clone(),
            messages: req.messages.clone(),
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
        result
    }
}

impl OpenAiClient {
    pub fn new() -> Self {
        let config_directory = config_dir()
            .expect("Failed to find config directory")
            .join("cgip");

        let config_manager = ConfigManager::new(config_directory);

        OpenAiClient {
            open_ai_api_key: config_manager.config.openai_api_key.clone(),
        }
    }
}
