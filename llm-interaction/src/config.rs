use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Config {
    pub api_key: Option<String>,
    pub config_path: PathBuf,
    pub tui_enabled: bool,
    pub current_model: Option<String>,
    pub token_tracking: bool,
}

impl Config {
    pub fn new() -> io::Result<Self> {
        let config_dir = dirs::config_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Could not find config directory")
        })?;
        let config_path = config_dir.join("claude-cli/config.json");

        if let Ok(config_str) = fs::read_to_string(&config_path) {
            let config: HashMap<String, serde_json::Value> =
                serde_json::from_str(&config_str).unwrap_or_default();

            let api_key = config
                .get("api_key")
                .and_then(|v| v.as_str())
                .map(String::from);

            let current_model = config
                .get("current_model")
                .and_then(|v| v.as_str())
                .map(String::from);

            let tui_enabled = config
                .get("tui_enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let token_tracking = config
                .get("token_tracking")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            Ok(Config {
                api_key,
                config_path,
                tui_enabled,
                current_model,
                token_tracking,
            })
        } else {
            Ok(Config {
                api_key: None,
                config_path,
                tui_enabled: false,
                current_model: None,
                token_tracking: true,
            })
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let mut config = HashMap::new();

        if let Some(key) = &self.api_key {
            config.insert(
                "api_key".to_string(),
                serde_json::Value::String(key.clone()),
            );
        }

        if let Some(model) = &self.current_model {
            config.insert(
                "current_model".to_string(),
                serde_json::Value::String(model.clone()),
            );
        }

        config.insert(
            "tui_enabled".to_string(),
            serde_json::Value::Bool(self.tui_enabled),
        );
        config.insert(
            "token_tracking".to_string(),
            serde_json::Value::Bool(self.token_tracking),
        );

        // Create directory if it doesn't exist
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let config_str = serde_json::to_string(&config)?;
        fs::write(&self.config_path, config_str)
    }

    pub fn set_key(&mut self, key: String) -> io::Result<()> {
        self.api_key = Some(key);
        self.save()
    }

    pub fn set_model(&mut self, model: String) -> io::Result<()> {
        self.current_model = Some(model);
        self.save()
    }

    pub fn set_tui_enabled(&mut self, enabled: bool) -> io::Result<()> {
        self.tui_enabled = enabled;
        self.save()
    }

    pub fn set_token_tracking(&mut self, enabled: bool) -> io::Result<()> {
        self.token_tracking = enabled;
        self.save()
    }

    pub fn get_conversations_dir(&self) -> PathBuf {
        self.config_path
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .join("conversations")
    }

    pub fn ensure_conversations_dir(&self) -> io::Result<()> {
        let dir = self.get_conversations_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(())
    }
}
