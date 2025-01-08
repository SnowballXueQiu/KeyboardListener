use rusqlite::{params, Connection, Result};
use crate::receiver::EventData;
use std::fs;

pub fn store_event(device_id: &str, event: &EventData) -> Result<()> {
    let db_path = format!("./data/{}.db", device_id);
    fs::create_dir_all("./data").map_err(|e| rusqlite::Error::InvalidPath(std::path::PathBuf::from(e.to_string())))?;
    let conn = Connection::open(db_path)?;

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
        params![event.timestamp, event.timezone, event.event_type, event.content],
    )?;

    Ok(())
}