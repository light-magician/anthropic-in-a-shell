mod modules;
use modules::{parse_sse_line, ClaudeApiError, ClaudeApiRequest, ClaudeApiResponse, ClaudeMessage, ClaudeStreamApiRequest, StreamEvent};

use clap::{Parser, Subcommand};
use dirs;
use reqwest;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use tokio;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set your Claude API key
    SetKey {
        /// Your Claude API key
        key: String,
    },
    /// Show available commands and usage information
    Status, // Changed from Help to Status since help is built-in
}

struct Config {
    api_key: Option<String>,
    config_path: std::path::PathBuf,
}

impl Config {
    fn new() -> io::Result<Self> {
        let config_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find config directory"))?;
        let config_path = config_dir.join("claude-cli/config.json");

        if let Ok(config_str) = fs::read_to_string(&config_path) {
            let config: HashMap<String, String> =
                serde_json::from_str(&config_str).unwrap_or_default();
            Ok(Config {
                api_key: config.get("api_key").cloned(),
                config_path,
            })
        } else {
            Ok(Config {
                api_key: None,
                config_path,
            })
        }
    }

    fn save(&self) -> io::Result<()> {
        let mut config = HashMap::new();
        if let Some(key) = &self.api_key {
            config.insert("api_key".to_string(), key.clone());
        }

        // Create directory if it doesn't exist
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_str = serde_json::to_string(&config)?;
        fs::write(&self.config_path, config_str)
    }

    fn set_key(&mut self, key: String) -> io::Result<()> {
        self.api_key = Some(key);
        self.save()
    }
}

async fn send_message(api_key: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    // sonnet is claude-3-5-sonnet-20241022
    let request = ClaudeStreamApiRequest {
        model: "claude-3-5-haiku-20241022".to_string(),
        max_tokens: 1024,
        messages: vec![ClaudeMessage {
            role: "user".to_string(),
            content: content.to_string(),
        }],
        stream: true,
    };

    let mut response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await?;

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
                    },
                    StreamEvent::MessageStop => {
                        println!(); // New line after message is complete
                        return Ok(());
                    },
                    _ => {} // Ignore other events
                }
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = Config::new()?;

    match &cli.command {
        Some(Commands::SetKey { key }) => {
            config.set_key(key.clone())?;
            println!("API key has been set successfully.");
            return Ok(());
        }
        Some(Commands::Status) => {
            println!("Available commands:");
            println!("  setkey <key>    Set your Claude API key");
            println!("  status          Show this status message");
            println!("\nIn chat mode:");
            println!("  /quit           Exit the program");
            println!("  /help           Show help message");
            return Ok(());
        }
        None => {}
    }

    if config.api_key.is_none() {
        println!(
            "No API key found. Please set your API key using: claude-cli setkey <your-api-key>"
        );
        return Ok(());
    }

    println!("Claude CLI started. Type /quit to exit, /help for commands.");

    loop {
        print!("👤 "); // Human emoji prompt
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        match input {
            "/quit" => break,
            "/help" => {
                println!("Available commands:");
                println!("  /quit           Exit the program");
                println!("  /help           Show this help message");
                continue;
            }
            "" => continue,
            _ => {
                match send_message(config.api_key.as_ref().unwrap(), input).await {
                    Ok(()) => {},
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
