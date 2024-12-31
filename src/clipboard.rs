use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use unicode_segmentation::UnicodeSegmentation;
use std::{thread, time};
use crate::logger; // 导入 logger 模块

/// 判断字符是否是可打印的
/*pub fn is_printable_char(c: char) -> bool {
    c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation() || c.is_ascii_graphic() || c.is_ascii_whitespace()
}*/

/// 监控剪贴板内容变化
pub fn monitor_clipboard() {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut last_clipboard = "".into();

    loop {
        // 捕获 get_contents 的错误，如果不是文本内容则跳过
        let current_clipboard = match clipboard.get_contents() {
            Ok(content) => content,
            Err(_) => {
                thread::sleep(time::Duration::from_millis(1));
                continue;
            }
        };
        
        if current_clipboard != last_clipboard {
            logger::log_event("复制", &current_clipboard); // 使用 logger 记录事件
            last_clipboard = current_clipboard;
        }
        thread::sleep(time::Duration::from_millis(1)); // 每毫秒检查一次剪贴板
    }
}
