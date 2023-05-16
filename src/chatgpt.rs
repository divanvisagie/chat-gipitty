use reqwest::header;
use serde_json::Result;
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
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

pub async fn get_response(text: String) -> String {
    let system_prompt = r#"
        You are a helpul command line assistant running in a terminal, users can
        pass you the standard output from their command line and you will try and 
        help them debug their issues or answer questions.
    "#;

    // Retrieve the API key from the environment variable
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    let chat_request = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "system".to_string(),
            content: system_prompt.trim().to_string(),
        },Message {
            role: "user".to_string(),
            content: text.trim().to_string(),
        }],
    };

    let request_body = serde_json::to_string(&chat_request).unwrap();

    let response = client
        .post(url)
        .headers(headers)
        .body(request_body)
        .send()
        .await
        .unwrap();

    let response_text = response.text().await.unwrap();

    let response_object = parse_response(&response_text).unwrap();

    response_object.choices[0].message.content.clone()
}
