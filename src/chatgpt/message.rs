use std::fmt;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Value>,
    pub content: MessageContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Multi(Vec<ContentPart>),
}

impl MessageContent {
    pub fn as_str(&self) -> &str {
        match self {
            MessageContent::Text(text) => text,
            MessageContent::Multi(_) => "[Multi-content message]",
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            MessageContent::Text(text) => text.is_empty(),
            MessageContent::Multi(parts) => parts.is_empty(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            MessageContent::Text(text) => text.clone(),
            MessageContent::Multi(parts) => {
                parts.iter()
                    .filter_map(|part| match part {
                        ContentPart::Text { text } => Some(text.as_str()),
                        ContentPart::ImageUrl { .. } => Some("[Image]"),
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        }
    }
}

impl fmt::Display for MessageContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageContent::Text(text) => write!(f, "{}", text),
            MessageContent::Multi(parts) => {
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 { write!(f, " ")?; }
                    match part {
                        ContentPart::Text { text } => write!(f, "{}", text)?,
                        ContentPart::ImageUrl { image_url } => write!(f, "[Image: {}]", image_url.url)?,
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    pub url: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.content {
            MessageContent::Text(text) => write!(f, "{}: {}", self.role, text),
            MessageContent::Multi(parts) => {
                write!(f, "{}: ", self.role)?;
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 { write!(f, " + ")?; }
                    match part {
                        ContentPart::Text { text } => write!(f, "{}", text)?,
                        ContentPart::ImageUrl { image_url } => write!(f, "[Image: {}]", image_url.url)?,
                    }
                }
                Ok(())
            }
        }
    }
}