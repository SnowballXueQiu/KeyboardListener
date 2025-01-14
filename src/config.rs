use std::{fs::OpenOptions, io::{Read, Write}, sync::LazyLock};
use serde::{Deserialize, Serialize};
use std::os::windows::fs::OpenOptionsExt;

const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

static CONFIG: LazyLock<Config> = LazyLock::new(|| read_config());

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub device_name: Option<String>,
    pub backend_url: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            device_name: Some("unnamed".to_string()),
            backend_url: Some("http://127.0.0.1:8080/receiver".to_string()),
        }
    }
}

pub fn read_config() -> Config {
    let mut file = String::new();
    let mut file_handle = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .custom_flags(FILE_ATTRIBUTE_HIDDEN)
        .open("config.toml")
        .expect("Failed to open config.toml");

    file_handle.read_to_string(&mut file)
        .expect("Failed to read config.toml");

    let config: Config = if file.is_empty() {
        let default_config = Config::default();
        let toml = toml::to_string(&default_config).expect("Failed to serialize default config");
        file_handle.write_all(toml.as_bytes()).expect("Failed to write default config");
        default_config
    } else {
        toml::from_str(&file).unwrap_or_else(|_| {
            let default_config = Config::default();
            let toml = toml::to_string(&default_config).expect("Failed to serialize default config");
            file_handle.set_len(0).expect("Failed to truncate config file");
            file_handle.write_all(toml.as_bytes()).expect("Failed to write default config");
            default_config
        })
    };

    config
}

pub fn get_device_name() -> String {
    CONFIG.device_name.clone().unwrap_or("unnamed".to_string())
}

pub fn get_backend_url() -> String {
    CONFIG.backend_url.clone().unwrap_or("http://127.0.0.1:8080/receiver".to_string())
}
