use crate::modules::{
    parse_sse_line, ClaudeMessage, ClaudeStreamApiRequest, ClaudeUsage, StreamEvent,
};
use crate::tui::TerminalUi;
use crossterm::Result as TermResult;
use std::io::{self, Write};

pub struct ApiClient {
    api_key: String,
}

impl ApiClient {
    pub fn new(api_key: String) -> Self {
        ApiClient { api_key }
    }

    pub async fn send_message(
        &self,
        content: &str,
        model: &str,
    ) -> Result<(String, Option<(u32, u32)>), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let request = ClaudeStreamApiRequest {
            model: model.to_string(),
            max_tokens: 1024,
            messages: vec![ClaudeMessage {
                role: "user".to_string(),
                content: content.to_string(),
            }],
            stream: true,
        };

        let mut response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        let mut full_response = String::new();
        let mut input_tokens = None;
        let mut output_tokens = None;

        print!("🤖 "); // Claude emoji prompt
        io::stdout().flush()?;

        let mut stdout = io::stdout();

        while let Some(chunk) = response.chunk().await? {
            let chunk_str = String::from_utf8_lossy(&chunk);
            for line in chunk_str.lines() {
                if let Some(event) = parse_sse_line(line) {
                    match event {
                        StreamEvent::ContentBlockDelta { delta, .. } => {
                            stdout.write_all(delta.text.as_bytes())?;
                            stdout.flush()?;
                            full_response.push_str(&delta.text);
                        }
                        StreamEvent::MessageStop => {
                            println!(); // New line after message is complete
                        }
                        StreamEvent::MessageDelta { usage, .. } => {
                            input_tokens = usage.input_tokens;
                            output_tokens = usage.output_tokens;
                        }
                        _ => {} // Ignore other events
                    }
                }
            }
        }

        let token_info = if let (Some(input), Some(output)) = (input_tokens, output_tokens) {
            Some((input, output))
        } else {
            None
        };

        Ok((full_response, token_info))
    }

    pub async fn send_message_tui(
        &self,
        content: &str,
        model: &str,
        ui: &TerminalUi,
    ) -> Result<(String, Option<(u32, u32)>), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let request = ClaudeStreamApiRequest {
            model: model.to_string(),
            max_tokens: 1024,
            messages: vec![ClaudeMessage {
                role: "user".to_string(),
                content: content.to_string(),
            }],
            stream: true,
        };

        // Show thinking indicator
        ui.draw_thinking_spinner()?;

        let mut response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        // Clear thinking indicator
        ui.clear_thinking_spinner()?;

        let mut full_response = String::new();
        let mut input_tokens = None;
        let mut output_tokens = None;

        let model_display = match model {
            "claude-3-5-haiku-latest" => "Claude 3.5 Haiku",
            "claude-3-7-sonnet-latest" => "Claude 3.7 Sonnet",
            other => other,
        };

        // Start response
        let mut stdout = io::stdout();
        crossterm::queue!(
            stdout,
            crossterm::style::SetForegroundColor(crossterm::style::Color::Cyan),
            crossterm::style::Print(format!("🤖 {}:\n", model_display)),
            crossterm::style::ResetColor
        )?;
        stdout.flush()?;

        // Draw initial message box
        ui.draw_model_message("", None, None)?;

        while let Some(chunk) = response.chunk().await? {
            let chunk_str = String::from_utf8_lossy(&chunk);
            for line in chunk_str.lines() {
                if let Some(event) = parse_sse_line(line) {
                    match event {
                        StreamEvent::ContentBlockDelta { delta, .. } => {
                            full_response.push_str(&delta.text);
                            // Redraw the message box with updated content
                            ui.draw_model_message(&full_response, None, None)?;
                        }
                        StreamEvent::MessageStop => {
                            // Final update
                        }
                        StreamEvent::MessageDelta { usage, .. } => {
                            input_tokens = usage.input_tokens;
                            output_tokens = usage.output_tokens;
                        }
                        _ => {} // Ignore other events
                    }
                }
            }
        }

        let token_info = if let (Some(input), Some(output)) = (input_tokens, output_tokens) {
            Some((input, output))
        } else {
            None
        };

        // Redraw one final time with token stats if available
        if let Some((input, output)) = token_info {
            let cost = match model {
                "claude-3-5-haiku-latest" => (input as f64 * 0.000001) + (output as f64 * 0.000005),
                "claude-3-7-sonnet-latest" => {
                    (input as f64 * 0.000005) + (output as f64 * 0.000020)
                }
                _ => 0.0,
            };

            ui.draw_model_message(&full_response, Some((input, output)), Some(cost))?;
        }

        Ok((full_response, token_info))
    }
}
