pub mod client;
pub mod message;
pub mod request;
pub mod response;
pub mod role;

pub use client::GptClient;
pub use message::{Message, MessageContent};
pub use role::Role;

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

fn get_models_url(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');

    if base.contains("/models") {
        return base.to_string();
    }

    if base.ends_with("/v1") || base.contains("/v1beta") || base.contains("/v2") {
        format!("{}/models", base)
    } else {
        format!("{}/v1/models", base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chatgpt::message::MessageContent;

    #[test]
    fn test_get_system_prompt() {
        let client = GptClient::new(false);
        assert!(!client.messages.is_empty());
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
                    use_search = true;
                }
            }
        }

        assert!(use_search, "Search prefix should be detected");
    }

    #[test]
    fn test_no_search_prefix() {
        let mut client = GptClient::new(false);
        client.add_message(Role::User, "what is the weather today?".to_string());

        // Simulate the search detection logic
        let mut use_search = false;
        if let Some(last_message) = client.messages.last_mut() {
            if let MessageContent::Text(ref content) = last_message.content {
                let trimmed_content = content.trim();
                if last_message.role == "user" && trimmed_content.starts_with("/search") {
                    use_search = true;
                }
            }
        }

        assert!(!use_search, "Search prefix should not be detected");
    }

    #[test]
    fn test_search_prefix_with_whitespace() {
        let mut client = GptClient::new(false);
        client.add_message(
            Role::User,
            "  /search what is the weather today?  ".to_string(),
        );

        // Simulate the search detection logic
        let mut use_search = false;
        if let Some(last_message) = client.messages.last_mut() {
            if let MessageContent::Text(ref content) = last_message.content {
                let trimmed_content = content.trim();
                if last_message.role == "user" && trimmed_content.starts_with("/search") {
                    use_search = true;
                }
            }
        }

        assert!(
            use_search,
            "Search prefix should be detected even with whitespace"
        );
    }

    #[test]
    fn test_search_model_selection_with_gpt() {
        let client = GptClient::new(false);
        let model = &client.config_manager.config.model;

        let use_search = true;
        let selected_model = if use_search && model.starts_with("gpt") {
            "gpt-4o-search-preview".to_string()
        } else {
            model.clone()
        };

        if model.starts_with("gpt") {
            assert_eq!(selected_model, "gpt-4o-search-preview");
        }
    }

    #[test]
    fn test_search_model_selection_with_non_gpt() {
        // This test assumes the model is not gpt-based
        let mut client = GptClient::new(false);
        client.config_manager.config.model = "claude-3".to_string();

        let use_search = true;
        let selected_model = if use_search && client.config_manager.config.model.starts_with("gpt")
        {
            "gpt-4o-search-preview".to_string()
        } else {
            client.config_manager.config.model.clone()
        };

        assert_eq!(selected_model, "claude-3");
    }

    #[test]
    fn test_search_model_selection_with_gpt_in_middle() {
        let mut client = GptClient::new(false);
        client.config_manager.config.model = "anthropic-gpt-4".to_string();

        let use_search = true;
        let selected_model = if use_search && client.config_manager.config.model.starts_with("gpt")
        {
            "gpt-4o-search-preview".to_string()
        } else {
            client.config_manager.config.model.clone()
        };

        assert_eq!(selected_model, "anthropic-gpt-4");
    }

    #[test]
    fn test_get_completions_url_without_v1() {
        let url = get_completions_url("https://api.openai.com");
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_v1() {
        let url = get_completions_url("https://api.openai.com/v1");
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_trailing_slash() {
        let url = get_completions_url("https://api.openai.com/");
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_v1_and_trailing_slash() {
        let url = get_completions_url("https://api.openai.com/v1/");
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_gemini_style() {
        let url = get_completions_url(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent",
        );
        assert_eq!(url, "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_existing_endpoint() {
        let url = get_completions_url("https://api.example.com/v1/chat/completions");
        assert_eq!(url, "https://api.example.com/v1/chat/completions");
    }

    #[test]
    fn test_get_completions_url_with_v2() {
        let url = get_completions_url("https://api.example.com/v2");
        assert_eq!(url, "https://api.example.com/v2/chat/completions");
    }

    #[test]
    fn test_get_models_url_without_v1() {
        let url = get_models_url("https://api.openai.com");
        assert_eq!(url, "https://api.openai.com/v1/models");
    }

    #[test]
    fn test_get_models_url_with_v1() {
        let url = get_models_url("https://api.openai.com/v1");
        assert_eq!(url, "https://api.openai.com/v1/models");
    }

    #[test]
    fn test_get_models_url_with_trailing_slash() {
        let url = get_models_url("https://api.openai.com/");
        assert_eq!(url, "https://api.openai.com/v1/models");
    }

    #[test]
    fn test_get_models_url_with_v2() {
        let url = get_models_url("https://api.example.com/v2");
        assert_eq!(url, "https://api.example.com/v2/models");
    }
}
