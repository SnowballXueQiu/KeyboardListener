use device_query::{DeviceQuery, DeviceState, Keycode};
use std::{collections::HashSet, thread, time};
use crate::logger;  // 导入 logger 模块

/// 管理当前按键的状态
pub fn monitor_keyboard() {
    let device_state = DeviceState::new();
    let mut keys_pressed: HashSet<Keycode> = HashSet::new(); // 存储当前按下的键

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let current_keys: HashSet<Keycode> = keys.into_iter().collect();

        // 查找按下的键（按下但未抬起）
        let newly_pressed: HashSet<_> = current_keys.difference(&keys_pressed).collect();
        for &key in newly_pressed.iter() {
            if let Some(key_name) = get_key_mapping(&key) {
                logger::log_event("按下键盘", &key_name); // 记录按下的键
            }
        }

        // 查找抬起的键（之前按下，现在松开）
        let newly_released: HashSet<_> = keys_pressed.difference(&current_keys).collect();
        for &key in newly_released.iter() {
            if let Some(key_name) = get_key_mapping(&key) {
                logger::log_event("松开键盘", &key_name); // 记录松开的键
            }
        }

        // 更新按键状态
        keys_pressed = current_keys;

        thread::sleep(time::Duration::from_millis(100)); // 每100毫秒检查一次
    }
}

/// 获取键盘输入的字符映射
pub fn get_key_mapping(key: &Keycode) -> Option<String> {
    match key {
        Keycode::A => Some("A".to_string()),
        Keycode::B => Some("B".to_string()),
        Keycode::C => Some("C".to_string()),
        Keycode::D => Some("D".to_string()),
        Keycode::E => Some("E".to_string()),
        Keycode::F => Some("F".to_string()),
        Keycode::G => Some("G".to_string()),
        Keycode::H => Some("H".to_string()),
        Keycode::I => Some("I".to_string()),
        Keycode::J => Some("J".to_string()),
        Keycode::K => Some("K".to_string()),
        Keycode::L => Some("L".to_string()),
        Keycode::M => Some("M".to_string()),
        Keycode::N => Some("N".to_string()),
        Keycode::O => Some("O".to_string()),
        Keycode::P => Some("P".to_string()),
        Keycode::Q => Some("Q".to_string()),
        Keycode::R => Some("R".to_string()),
        Keycode::S => Some("S".to_string()),
        Keycode::T => Some("T".to_string()),
        Keycode::U => Some("U".to_string()),
        Keycode::V => Some("V".to_string()),
        Keycode::W => Some("W".to_string()),
        Keycode::X => Some("X".to_string()),
        Keycode::Y => Some("Y".to_string()),
        Keycode::Z => Some("Z".to_string()),
        Keycode::Key1 => Some("1".to_string()),
        Keycode::Key2 => Some("2".to_string()),
        Keycode::Key3 => Some("3".to_string()),
        Keycode::Key4 => Some("4".to_string()),
        Keycode::Key5 => Some("5".to_string()),
        Keycode::Key6 => Some("6".to_string()),
        Keycode::Key7 => Some("7".to_string()),
        Keycode::Key8 => Some("8".to_string()),
        Keycode::Key9 => Some("9".to_string()),
        Keycode::Key0 => Some("0".to_string()),
        Keycode::Space => Some("Space".to_string()),
        Keycode::Enter => Some("Enter".to_string()),
        Keycode::LShift => Some("LShift".to_string()),
        Keycode::RShift => Some("RShift".to_string()),
        Keycode::LControl => Some("LCtrl".to_string()),
        Keycode::RControl => Some("RCtrl".to_string()),
        Keycode::LAlt => Some("LAlt".to_string()),
        Keycode::RAlt => Some("RAlt".to_string()),
        Keycode::Backspace => Some("Backspace".to_string()),
        Keycode::Escape => Some("Escape".to_string()),
        Keycode::F1 => Some("F1".to_string()),
        Keycode::F2 => Some("F2".to_string()),
        Keycode::F3 => Some("F3".to_string()),
        Keycode::F4 => Some("F4".to_string()),
        Keycode::F5 => Some("F5".to_string()),
        Keycode::F6 => Some("F6".to_string()),
        Keycode::F7 => Some("F7".to_string()),
        Keycode::F8 => Some("F8".to_string()),
        Keycode::F9 => Some("F9".to_string()),
        Keycode::F10 => Some("F10".to_string()),
        Keycode::F11 => Some("F11".to_string()),
        Keycode::F12 => Some("F12".to_string()),
        Keycode::CapsLock => Some("CapsLock".to_string()),
        _ => None,  // 不处理其他键
    }
}
