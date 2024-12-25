use std::{fs::OpenOptions, io::{self, Write}};
use chrono::Utc;

/// 写入日志到 latest.log 文件
pub fn log_event(event_type: &str, content: &str) {
    let timestamp = Utc::now();
    let time_str = timestamp.format("%Y/%m/%d-%H:%M:%S%.3f").to_string();

    // 打开文件以追加内容
    let file = match OpenOptions::new().append(true).create(true).open("latest.log") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening log file: {}", e);
            return;
        }
    };

    // 使用缓冲写入文件，以提高性能
    let mut file = io::BufWriter::new(file);

    // 写入日志
    if let Err(e) = writeln!(file, "{} {} {} {}", timestamp.timestamp_millis(), time_str, event_type, content) {
        eprintln!("Error writing to log file: {}", e);
    }
}
