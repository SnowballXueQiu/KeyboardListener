use std::{fs::OpenOptions, io::Read, sync::LazyLock};

use serde::{Deserialize, Serialize};

static CONFIG: LazyLock<Config> = LazyLock::new(|| read_config());

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub device_name: Option<String>,
    pub backend_url: Option<String>,
}

pub fn read_config() -> Config {
    let mut file = String::new();
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open("config.toml")
        .expect("config.toml not found")
        .read_to_string(&mut file)
        .expect(":<String>");
    toml::from_str(&file).expect("config.json is invalid")
}

pub fn get_device_name() -> String {
    CONFIG.device_name.clone().unwrap_or("Unnamed".to_string())
}

pub fn get_backend_url() -> String {
    CONFIG.backend_url.clone().unwrap_or("http://localhost:8080".to_string())
}