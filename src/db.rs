use crate::receiver::EventData;
use crate::websocket::DeviceSubscribers;
use rusqlite::{params, Connection, OptionalExtension, Result};
use std::fs;

const DATA_DIR: &str = "./data";

fn get_connection(path: &str) -> Result<Connection> {
    Connection::open(path).map_err(|e| {
        println!("Failed to open database {}: {}", path, e);
        e
    })
}

fn ensure_data_dir() -> Result<()> {
    fs::create_dir_all(DATA_DIR).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
}

pub async fn store_event(
    device_id: &str,
    event: &EventData,
    subscribers: Option<&DeviceSubscribers>,
) -> Result<()> {
    ensure_data_dir()?;
    let db_path = format!("{}/{}.db", DATA_DIR, device_id);
    let conn = get_connection(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            time INTEGER,
            timezone TEXT,
            event_type TEXT,
            content TEXT
        )",
        [],
    )?;

    conn.execute(
        "INSERT INTO events (time, timezone, event_type, content) VALUES (?1, ?2, ?3, ?4)",
        params![
            event.timestamp,
            event.timezone,
            event.event_type,
            event.content
        ],
    )?;

    if let Some(subs) = subscribers {
        crate::websocket::broadcast_log(
            device_id,
            event.timestamp,
            &event.event_type,
            &event.content,
            &event.timezone,
            subs,
        )
        .await;
    }

    Ok(())
}

pub fn store_device(device_id: &str, device_name: &str) -> Result<()> {
    ensure_data_dir()?;
    let db_path = format!("{}/device.db", DATA_DIR);
    let conn = get_connection(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS devices (
            device_id TEXT PRIMARY KEY,
            device_name TEXT
        )",
        [],
    )?;

    let mut stmt = conn.prepare("SELECT device_name FROM devices WHERE device_id = ?1")?;
    let current_name: Option<String> = stmt
        .query_row(params![device_id], |row| row.get(0))
        .optional()?;

    match current_name {
        Some(existing_name) => {
            if existing_name != device_name {
                conn.execute(
                    "UPDATE devices SET device_name = ?1 WHERE device_id = ?2",
                    params![device_name, device_id],
                )?;
            }
        }
        None => {
            conn.execute(
                "INSERT INTO devices (device_id, device_name) VALUES (?1, ?2)",
                params![device_id, device_name],
            )?;
        }
    }

    Ok(())
}

pub fn get_device_names() -> Result<Vec<(String, String)>> {
    ensure_data_dir()?;
    let db_path = format!("{}/device.db", DATA_DIR);
    let conn = get_connection(&db_path)?;

    let query = "SELECT device_id, device_name FROM devices";

    let mut stmt = conn.prepare(&query)?;
    let device_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let mut devices = Vec::new();
    for device in device_iter {
        devices.push(device?);
    }

    Ok(devices)
}

pub fn get_device_logs(
    device_id: &str,
    log_limit: usize,
) -> Result<Vec<(i64, String, String, String)>> {
    let db_path = format!("{}/{}.db", DATA_DIR, device_id);
    let conn = get_connection(&db_path)?;

    let query = format!(
        "SELECT time, event_type, content, timezone FROM events ORDER BY time DESC LIMIT {}",
        log_limit
    );

    let mut stmt = conn.prepare(&query)?;
    let log_iter = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    })?;

    let mut logs = Vec::new();
    for log in log_iter {
        logs.push(log?);
    }

    Ok(logs)
}
