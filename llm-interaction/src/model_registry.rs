use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmModel {
    pub id: String,
    pub display_name: String,
    pub input_cost: f64,  // Cost per 1M input tokens in USD
    pub output_cost: f64, // Cost per 1M output tokens in USD
    pub default_max_tokens: u32,
    pub description: String,
}

impl LlmModel {
    pub fn calculate_cost(&self, input_tokens: u32, output_tokens: u32) -> f64 {
        let input_cost = (input_tokens as f64 / 1_000_000.0) * self.input_cost;
        let output_cost = (output_tokens as f64 / 1_000_000.0) * self.output_cost;
        input_cost + output_cost
    }
}

pub struct ModelRegistry {
    models: HashMap<String, LlmModel>,
    current_model_id: String,
}

impl ModelRegistry {
    pub fn new() -> Self {
        let mut models = HashMap::new();

        // Define available models
        let haiku = LlmModel {
            id: "claude-3-5-haiku-latest".to_string(),
            display_name: "Claude 3.5 Haiku".to_string(),
            input_cost: 1_000.0,  // $1.00 per 1M input tokens
            output_cost: 5_000.0, // $5.00 per 1M output tokens
            default_max_tokens: 2048,
            description: "Fast and efficient model for everyday tasks.".to_string(),
        };

        let sonnet = LlmModel {
            id: "claude-3-7-sonnet-latest".to_string(),
            display_name: "Claude 3.7 Sonnet".to_string(),
            input_cost: 5_000.0,   // $5.00 per 1M input tokens
            output_cost: 20_000.0, // $20.00 per 1M output tokens
            default_max_tokens: 4096,
            description: "Powerful model with advanced reasoning capabilities.".to_string(),
        };

        models.insert(haiku.id.clone(), haiku);
        models.insert(sonnet.id.clone(), sonnet);

        ModelRegistry {
            models,
            current_model_id: "claude-3-5-haiku-latest".to_string(), // Default to Haiku
        }
    }

    pub fn get_current_model(&self) -> &LlmModel {
        self.models.get(&self.current_model_id).unwrap()
    }

    pub fn list_models(&self) -> Vec<&LlmModel> {
        self.models.values().collect()
    }

    pub fn select_model(&mut self, model_id: &str) -> Result<&LlmModel, String> {
        if let Some(model) = self.models.get(model_id) {
            self.current_model_id = model_id.to_string();
            Ok(model)
        } else {
            Err(format!("Model '{}' not found", model_id))
        }
    }

    pub fn save_to_config(&self, config_path: &Path) -> io::Result<()> {
        let mut config: HashMap<String, serde_json::Value> =
            if let Ok(config_str) = fs::read_to_string(config_path) {
                serde_json::from_str(&config_str).unwrap_or_default()
            } else {
                HashMap::new()
            };

        config.insert(
            "current_model".to_string(),
            serde_json::Value::String(self.current_model_id.clone()),
        );

        let config_str = serde_json::to_string(&config)?;
        fs::write(config_path, config_str)
    }

    pub fn load_from_config(&mut self, config_path: &Path) -> io::Result<()> {
        if let Ok(config_str) = fs::read_to_string(config_path) {
            let config: HashMap<String, serde_json::Value> =
                serde_json::from_str(&config_str).unwrap_or_default();

            if let Some(model_id) = config.get("current_model").and_then(|v| v.as_str()) {
                if self.models.contains_key(model_id) {
                    self.current_model_id = model_id.to_string();
                }
            }
        }

        Ok(())
    }
}
