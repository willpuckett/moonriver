use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 7125,
            api_key: None,
        }
    }
}

/// Load configuration from file or return default
pub fn load_config() -> anyhow::Result<Config> {
    // Try to load from moonriver.toml in current directory
    let config_path = PathBuf::from("moonriver.toml");
    
    if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    } else {
        // Return default config if file doesn't exist
        Ok(Config::default())
    }
}
