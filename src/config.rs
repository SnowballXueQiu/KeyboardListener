use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub runtime: RuntimeConfig,
    pub log_limit: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuntimeConfig {
    pub url: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            runtime: RuntimeConfig {
                url: "127.0.0.1".to_string(),
                port: 8080,
            },
            log_limit: 1000,
        }
    }
}

impl Config {
    pub fn init_config_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file_content = String::new();
        let mut file_handle = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;

        file_handle.read_to_string(&mut file_content)?;

        if file_content.is_empty() {
            let default_config = Config::default();
            let toml = toml::to_string(&default_config)?;
            file_handle.write_all(toml.as_bytes())?;
            return Ok(default_config);
        }

        match toml::from_str(&file_content) {
            Ok(config) => Ok(config),
            Err(_) => {
                let default_config = Config::default();
                let toml = toml::to_string(&default_config)?;
                file_handle.set_len(0)?;
                file_handle.write_all(toml.as_bytes())?;
                Ok(default_config)
            }
        }
    }
}
