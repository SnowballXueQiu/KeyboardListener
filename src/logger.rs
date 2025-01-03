use chrono::Local;
#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;
use std::{
    fs::OpenOptions,
    io::{self, Write},
};

const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

pub fn log_event(event_type: &str, content: &str) {
    let timestamp = Local::now();
    let offset = timestamp.offset();
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

    if let Err(e) = writeln!(file, "{} {} {}", time_str, event_type, content) {
        eprintln!("Error writing to log file: {}", e);
    }
}
