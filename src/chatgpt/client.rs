use std::env;
use dirs::config_dir;
use reqwest::header;
use serde_yaml;
use serde_json;

use crate::config_manager::ConfigManager;
use crate::chatgpt::message::{Message, MessageContent, ContentPart, ImageUrl};
use crate::chatgpt::role::Role;
use crate::chatgpt::request::ChatRequest;
use crate::chatgpt::response::{parse_response, parse_error_response};

pub struct GptClient {
    pub config_manager: ConfigManager,
    pub messages: Vec<Message>,
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
        let url = super::get_completions_url(&url);

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
        self.complete_with_max_tokens(None)
    }

    pub fn complete_with_tools(&mut self, tools: serde_json::Value) -> serde_json::Value {
        let last_content_text = match &self.messages.last().unwrap().content {
            MessageContent::Text(text) => text.clone(),
            MessageContent::Multi(parts) => parts
                .iter()
                .filter_map(|part| match part {
                    ContentPart::Text { text } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" "),
        };

        if last_content_text.to_lowercase().trim() == "ping" {
            self.add_message(Role::Assistant, "pong".to_string());
            return serde_json::json!({"choices": [{"message": {"content": "pong"}, "finish_reason": "stop"}]});
        }

        let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build client");

        let url = env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com".to_string());
        let url = super::get_completions_url(&url);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));
        let auth_header = header::HeaderValue::from_str(&format!("Bearer {}", api_key)).expect("auth header");
        headers.insert(header::AUTHORIZATION, auth_header);

        let model = self.config_manager.config.model.clone();

        let chat_request = serde_json::json!({
            "model": model,
            "messages": self.messages,
            "tools": tools,
            "tool_choice": "auto"
        });

        let response = client
            .post(url)
            .headers(headers)
            .json(&chat_request)
            .timeout(std::time::Duration::from_secs(60))
            .send();

        let response = match response {
            Ok(resp) => resp.text(),
            Err(e) => {
                if e.is_timeout() {
                    eprintln!("The request timed out.");
                }
                return serde_json::json!({});
            }
        };

        let response_text = response.expect("response text");
        let value: serde_json::Value = serde_json::from_str(&response_text).expect("parse json");

        if let Some(text) = value["choices"][0]["message"]["content"].as_str() {
            if !text.is_empty() {
                self.add_message(Role::Assistant, text.to_string());
            }
        }

        value
    }
}
