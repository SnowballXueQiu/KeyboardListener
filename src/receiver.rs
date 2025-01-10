use crate::db;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EventData {
    pub device_id: String,
    pub device_name: String,
    pub event_type: String,
    pub content: String,
    pub timestamp: i64,
    pub timezone: String,
}

pub async fn event_handler(Json(event): Json<EventData>) -> String {
    println!("Received Event:");
    println!("----------------------------");
    println!("Device ID   : {}", event.device_id);
    println!("Device Name : {}", event.device_name);
    println!("Event Type  : {}", event.event_type);
    println!("Content     : {}", event.content);
    println!("Timestamp   : {}", event.timestamp);
    println!("Timezone    : {}", event.timezone);
    println!("----------------------------");

    db::store_event(&event.device_id, &event).expect("Failed to store event");

    db::store_device(&event.device_id, &event.device_name)
        .expect("Failed to store or update device");

    "Event received successfully".to_string()
}
