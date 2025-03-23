use super::client::{LanguageModelClient, LanguageModelRequest};

pub struct TestClient {}

impl LanguageModelClient for TestClient {
    fn complete(&mut self, _request: &LanguageModelRequest) -> String {
        "This is a test language model response".to_string()
    }
}

impl TestClient {
    pub fn new() -> Self {
        Self {}
    }
}
