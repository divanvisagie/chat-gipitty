use std::env;
use tracing::info;

#[allow(dead_code)]
pub struct ModelInfo {
    /// The maximum number of input tokens for the model
    pub input_tokens: usize,
    pub output_tokens: usize,
    /// Name of the model
    pub name: String,
    pub key: String,
    /// Base URL for the model API
    pub base_url: String,
}

const OPENAI_BASE_URL: &str = "OPENAI_BASE_URL";
const OLLAMA_BASE_URL: &str = "OLLAMA_BASE_URL";
const MISTRAL_BASE_URL: &str = "MISTRAL_BASE_URL";
const GEMINI_BASE_URL: &str = "GEMINI_BASE_URL";

fn ensure_chat_completions_path(mut base_url: String) -> String {
    let path = "/v1/chat/completions";
    if !base_url.ends_with(path) {
        if base_url.ends_with('/') {
            base_url.pop();
        }
        base_url.push_str(path);
    }
    base_url
}

fn openai_base_url() -> String {
    let url = env::var(OPENAI_BASE_URL)
        .unwrap_or_else(|_| "https://api.openai.com".to_string());
    ensure_chat_completions_path(url)
}

fn ollama_base_url() -> String {
    let url = env::var(OLLAMA_BASE_URL).unwrap_or_else(|_| "http://localhost:11434".to_string());
    ensure_chat_completions_path(url)
}

fn mistral_base_url() -> String {
    let url = env::var(MISTRAL_BASE_URL)
        .unwrap_or_else(|_| "https://api.mistral.ai".to_string());
    ensure_chat_completions_path(url)
}

fn gemini_base_url() -> String {
    let url = env::var(GEMINI_BASE_URL).unwrap_or_else(|_| "https://generativelanguage.googleapis.com/v1beta/openai".to_string());
    // For Gemini, append /chat/completions (no v1)
    let path = "/chat/completions";
    let mut base_url = url;
    if !base_url.ends_with(path) {
        if base_url.ends_with('/') {
            base_url.pop();
        }
        base_url.push_str(path);
    }
    base_url
}

impl ModelInfo {
    pub fn new(
        name: String,
    ) -> Self {
        info!("ModelInfo::new called with model name: {}", name);
        match name.as_str() {
            "gpt-4.1" => Self::new_gpt_4_1(),
            "gpt-4o" => Self::new_gpt_4o(),
            "gpt-4o-mini" => Self::new_gpt_4o_mini(),
            "llama3.2" => Self::new_llama3_2(),
            "mistral-large-2402" => Self::new_mistral_large_2402(),
            "gemini-2.0-flash" => Self::new_gemini_2_0_flash(),
            _ => Self::ollama_fallback(name),
        }
    }

    pub fn new_gpt_4_1() -> Self {
        ModelInfo {
            input_tokens: 128_000,
            output_tokens: 4_096,
            name: "gpt-4.1".to_string(),
            key: env::var("OPENAI_API_KEY").unwrap_or_default(),
            base_url: openai_base_url(),
        }
    }

    pub fn new_gpt_4o() -> Self {
        ModelInfo {
            input_tokens: 128_000,
            output_tokens: 4_096,
            name: "gpt-4o".to_string(),
            key: env::var("OPENAI_API_KEY").unwrap_or_default(),
            base_url: openai_base_url(),
        }
    }

    fn new_gpt_4o_mini() -> ModelInfo {
        ModelInfo {
            input_tokens: 48_000,
            output_tokens: 4_096,
            name: "gpt-4o-mini".to_string(),
            key: env::var("OPENAI_API_KEY").unwrap_or_default(),
            base_url: openai_base_url(),
        }
    }

    fn new_llama3_2() -> ModelInfo {
        ModelInfo {
            input_tokens: 128_000,
            output_tokens: 2048,
            name: "llama3.2".to_string(),
            key: env::var("OLLAMA_API_KEY").unwrap_or_default(),
            base_url: ollama_base_url(),
        }
    }

    fn new_mistral_large_2402() -> ModelInfo {
        ModelInfo {
            input_tokens: 128_000,
            output_tokens: 2048,
            name: "mistral-large-2402".to_string(),
            key: env::var("MISTRAL_API_KEY").unwrap_or_default(),
            base_url: mistral_base_url(),
        }
    }

    fn new_gemini_2_0_flash() -> ModelInfo {
        ModelInfo {
            input_tokens: 128_000,
            output_tokens: 2048,
            name: "gemini-2.0-flash".to_string(),
            key: env::var("GEMINI_API_KEY").unwrap_or_default(),
            base_url: gemini_base_url(),
        }
    }

    fn ollama_fallback(name: String) -> ModelInfo {
        let ollama_url = ollama_base_url();
        info!("Using Ollama fallback for model: {} (endpoint: {})", name, ollama_url);
        ModelInfo {
            input_tokens: 128_000,
            output_tokens: 2048,
            name,
            key: env::var("OLLAMA_API_KEY").unwrap_or_default(),
            base_url: ollama_url,
        }
    }
} 