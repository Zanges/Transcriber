use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hotkey: String,
    pub language: String,
    pub openai_api_key: String,
    pub keypress_delay: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hotkey: "F7".to_string(),
            language: "en".to_string(),
            openai_api_key: String::new(),
            keypress_delay: 10,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = "config.yaml";
        if Path::new(config_path).exists() {
            let config_str = fs::read_to_string(config_path)?;
            let config: Config = serde_yaml::from_str(&config_str)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_yaml = serde_yaml::to_string(self)?;
        fs::write("config.yaml", config_yaml)?;
        Ok(())
    }
}
