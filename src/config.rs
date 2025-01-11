use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub runtime: RuntimeConfig,
}

#[derive(Deserialize)]
pub struct RuntimeConfig {
    pub url: String,
    pub port: u16,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}
