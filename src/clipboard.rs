use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use unicode_segmentation::UnicodeSegmentation;
use std::{thread, time};
use crate::logger; // 导入 logger 模块

/// 判断字符是否是可打印的
fn is_printable_char(c: char) -> bool {
    c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation() || c.is_ascii_graphic() || c.is_ascii_whitespace()
}

/// 监控剪贴板内容变化
pub fn monitor_clipboard() {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut last_clipboard = clipboard.get_contents().unwrap();

    loop {
        // 捕获 get_contents 的错误，如果不是文本内容则跳过
        let current_clipboard = match clipboard.get_contents() {
            Ok(content) => content,
            Err(_) => {
                thread::sleep(time::Duration::from_secs(1)); // 如果发生错误，继续下一次循环
                continue;
            }
        };

        // 确保只记录有效的文本类型剪贴板内容
        if current_clipboard != last_clipboard && current_clipboard.graphemes(true).all(|c| is_printable_char(c.chars().next().unwrap())) {
            logger::log_event("复制", &current_clipboard); // 使用 logger 记录事件
            last_clipboard = current_clipboard;
        }

        thread::sleep(time::Duration::from_secs(1)); // 每秒检查一次剪贴板
    }
}
