use std::{fs::OpenOptions, io::{self, Write}};
use chrono::Utc;
#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

pub fn log_event(event_type: &str, content: &str) {
    let timestamp = Utc::now();
    let time_str = timestamp.format("%Y/%m/%d-%H:%M:%S%.3f").to_string();

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
