mod clipboard;
mod keyboard;
mod logger;

use std::thread;

fn main() {
    let clipboard_thread = thread::spawn(|| {
        clipboard::monitor_clipboard();
    });

    let keyboard_thread = thread::spawn(|| {
        keyboard::monitor_keyboard();
    });

    clipboard_thread.join().unwrap();
    keyboard_thread.join().unwrap();
}
