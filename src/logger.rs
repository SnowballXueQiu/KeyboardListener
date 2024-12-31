use std::{fs::OpenOptions, io::{self, Write}};
use chrono::Utc;

pub fn log_event(event_type: &str, content: &str) {
    let timestamp = Utc::now();
    let time_str = timestamp.format("%Y/%m/%d-%H:%M:%S%.3f").to_string();

    let file = match OpenOptions::new().append(true).create(true).open("latest.log") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening log file: {}", e);
            return;
        }
    };

    let mut file = io::BufWriter::new(file);

    if let Err(e) = writeln!(file, "{} {} {}", time_str, event_type, content) {
        eprintln!("Error writing to log file: {}", e);
    }
}
