use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ClaudeMessage {
    pub role: String,
    pub content: String,
}
#[derive(Serialize)]
pub struct ClaudeApiRequest {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<ClaudeMessage>,
}

#[derive(Deserialize, Debug)]
pub struct ClaudeContentItem {
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: String,
}

#[derive(Deserialize, Debug)]
pub struct ClaudeUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct ClaudeApiResponse {
    pub content: Vec<ClaudeContentItem>,
    pub id: String,
    pub model: String,
    pub role: String,
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
    #[serde(rename = "type")]
    pub response: String,
    pub usage: ClaudeUsage,
}

#[derive(Deserialize, Debug)]
pub struct ClaudeErrorDetails {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ClaudeApiError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub error: ClaudeErrorDetails,
}
