use crate::config::Config;
use crate::db;
use axum::{extract::Path, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Device {
    device_id: String,
    device_name: String,
}

#[derive(Serialize)]
pub struct DeviceLog {
    time: i64,
    event_type: String,
    content: String,
    timezone: String,
}

pub async fn get_device_list() -> Json<Vec<Device>> {
    match db::get_device_names() {
        Ok(devices) => Json(
            devices
                .into_iter()
                .map(|(device_id, device_name)| Device {
                    device_id,
                    device_name,
                })
                .collect(),
        ),
        Err(_) => Json(vec![]),
    }
}

pub async fn get_device_logs(Path(device_id): Path<String>) -> Json<Vec<DeviceLog>> {
    let config = Config::init_config_file("config.toml").unwrap_or_else(|err| {
        println!(
            "Failed to initialize or read config.toml: {}. Using default configuration.",
            err
        );
        Config::default()
    });

    let log_limit = config.log_limit;

    match db::get_device_logs(&device_id, log_limit) {
        Ok(logs) => Json(
            logs.into_iter()
                .map(|(time, event_type, content, timezone)| DeviceLog {
                    time,
                    event_type,
                    content,
                    timezone,
                })
                .collect(),
        ),
        Err(_) => Json(vec![]),
    }
}
