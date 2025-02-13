use crate::key_mapping::get_key_mapping;
use crate::logger;
use device_query::{DeviceQuery, Keycode};
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::ffi::c_void;
use std::mem::ManuallyDrop;
use std::sync::Mutex;
use windows::Win32::Foundation::{BOOL, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA, TranslateMessage, HHOOK,
    KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
};

lazy_static! {
    pub static ref keyStates: Mutex<HashSet<Keycode>> = Mutex::new(HashSet::new());
}

pub unsafe extern "system" fn kbd_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // SAFETY: lparam must be a valid KBDLLHOOKSTRUCT pointer
    let ctx = unsafe { *(lparam.0 as *const KBDLLHOOKSTRUCT) };
    match wparam.0 as u32 {
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            if let Some(key) = win_key_to_keycode(ctx.vkCode as u16) {
                let mut t = keyStates.lock().unwrap();
                t.insert(key);
            }
            if let Some(key) = get_key_mapping(VIRTUAL_KEY(ctx.vkCode as u16)) {
                logger::log_event(logger::EventType::KeyboardPress, key.as_str());
            }
        }
        _ => panic!("unexpected key state change"),
    }
    // SAFETY: All parameters are valid
    unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) }
}

pub fn monitor_keyboard() {
    // SAFETY: kbd_hook is a valid function pointer
    unsafe {
        let _ = ManuallyDrop::new(
            SetWindowsHookExA(
                WH_KEYBOARD_LL,
                Some(kbd_hook),
                HINSTANCE::from(HMODULE(0 as *mut c_void)),
                0u32,
            )
            .expect("failed to register low-level keyboard hook"),
        );
    }
    {
        let mut t = keyStates.lock().unwrap();
        device_query::DeviceState::new()
            .get_keys()
            .iter()
            .for_each(|i| {
                t.insert(i.clone());
            });
    }
    let mut msg: MSG = MSG::default();
    loop {
        let ret = unsafe { GetMessageA(&mut msg as *mut MSG, HWND(0 as *mut c_void), 0, 0) };
        if ret == BOOL(0) {
            break;
        }
        if ret == BOOL(-1) {
            panic!("exception in message handling");
        } else {
            unsafe {
                let _ = TranslateMessage(&msg as *const MSG);
                let _ = DispatchMessageA(&msg as *const MSG);
            }
        }
    }
}
pub fn win_key_to_keycode(win_key: u16) -> Option<Keycode> {
    let mut keycode = match VIRTUAL_KEY(win_key) {
        VK_F1 => Some(Keycode::F1),
        VK_F2 => Some(Keycode::F2),
        VK_F3 => Some(Keycode::F3),
        VK_F4 => Some(Keycode::F4),
        VK_F5 => Some(Keycode::F5),
        VK_F6 => Some(Keycode::F6),
        VK_F7 => Some(Keycode::F7),
        VK_F8 => Some(Keycode::F8),
        VK_F9 => Some(Keycode::F9),
        VK_F10 => Some(Keycode::F10),
        VK_F11 => Some(Keycode::F11),
        VK_F12 => Some(Keycode::F12),
        VK_F13 => Some(Keycode::F13),
        VK_F14 => Some(Keycode::F14),
        VK_F15 => Some(Keycode::F15),
        VK_F16 => Some(Keycode::F16),
        VK_F17 => Some(Keycode::F17),
        VK_F18 => Some(Keycode::F18),
        VK_F19 => Some(Keycode::F19),
        VK_F20 => Some(Keycode::F20),
        VK_NUMPAD0 => Some(Keycode::Numpad0),
        VK_NUMPAD1 => Some(Keycode::Numpad1),
        VK_NUMPAD2 => Some(Keycode::Numpad2),
        VK_NUMPAD3 => Some(Keycode::Numpad3),
        VK_NUMPAD4 => Some(Keycode::Numpad4),
        VK_NUMPAD5 => Some(Keycode::Numpad5),
        VK_NUMPAD6 => Some(Keycode::Numpad6),
        VK_NUMPAD7 => Some(Keycode::Numpad7),
        VK_NUMPAD8 => Some(Keycode::Numpad8),
        VK_NUMPAD9 => Some(Keycode::Numpad9),
        VK_ADD => Some(Keycode::NumpadAdd),
        VK_SUBTRACT => Some(Keycode::NumpadSubtract),
        VK_DIVIDE => Some(Keycode::NumpadDivide),
        VK_MULTIPLY => Some(Keycode::NumpadMultiply),
        VK_OEM_NEC_EQUAL => Some(Keycode::NumpadEquals),
        VK_DECIMAL => Some(Keycode::NumpadDecimal),
        VK_SPACE => Some(Keycode::Space),
        VK_LCONTROL => Some(Keycode::LControl),
        VK_RCONTROL => Some(Keycode::RControl),
        VK_LSHIFT => Some(Keycode::LShift),
        VK_RSHIFT => Some(Keycode::RShift),
        VK_LMENU => Some(Keycode::LAlt),
        VK_RMENU => Some(Keycode::RAlt),
        VK_LWIN => Some(Keycode::LMeta),
        VK_RWIN => Some(Keycode::RMeta),
        VK_RETURN => Some(Keycode::Enter),
        VK_ESCAPE => Some(Keycode::Escape),
        VK_UP => Some(Keycode::Up),
        VK_DOWN => Some(Keycode::Down),
        VK_LEFT => Some(Keycode::Left),
        VK_RIGHT => Some(Keycode::Right),
        VK_BACK => Some(Keycode::Backspace),
        VK_CAPITAL => Some(Keycode::CapsLock),
        VK_TAB => Some(Keycode::Tab),
        VK_HOME => Some(Keycode::Home),
        VK_END => Some(Keycode::End),
        VK_PRIOR => Some(Keycode::PageUp),
        VK_NEXT => Some(Keycode::PageDown),
        VK_INSERT => Some(Keycode::Insert),
        VK_DELETE => Some(Keycode::Delete),
        VK_OEM_3 => Some(Keycode::Grave),
        VK_OEM_MINUS => Some(Keycode::Minus),
        VK_OEM_PLUS => Some(Keycode::Equal),
        VK_OEM_4 => Some(Keycode::LeftBracket),
        VK_OEM_6 => Some(Keycode::RightBracket),
        VK_OEM_5 => Some(Keycode::BackSlash),
        VK_OEM_1 => Some(Keycode::Semicolon),
        VK_OEM_7 => Some(Keycode::Apostrophe),
        VK_OEM_COMMA => Some(Keycode::Comma),
        VK_OEM_PERIOD => Some(Keycode::Dot),
        VK_OEM_2 => Some(Keycode::Slash),

        _ => None,
    };

    if keycode.is_none() {
        let win_key = win_key as u8;
        keycode = match win_key as char {
            '0' => Some(Keycode::Key0),
            '1' => Some(Keycode::Key1),
            '2' => Some(Keycode::Key2),
            '3' => Some(Keycode::Key3),
            '4' => Some(Keycode::Key4),
            '5' => Some(Keycode::Key5),
            '6' => Some(Keycode::Key6),
            '7' => Some(Keycode::Key7),
            '8' => Some(Keycode::Key8),
            '9' => Some(Keycode::Key9),
            'A' => Some(Keycode::A),
            'B' => Some(Keycode::B),
            'C' => Some(Keycode::C),
            'D' => Some(Keycode::D),
            'E' => Some(Keycode::E),
            'F' => Some(Keycode::F),
            'G' => Some(Keycode::G),
            'H' => Some(Keycode::H),
            'I' => Some(Keycode::I),
            'J' => Some(Keycode::J),
            'K' => Some(Keycode::K),
            'L' => Some(Keycode::L),
            'M' => Some(Keycode::M),
            'N' => Some(Keycode::N),
            'O' => Some(Keycode::O),
            'P' => Some(Keycode::P),
            'Q' => Some(Keycode::Q),
            'R' => Some(Keycode::R),
            'S' => Some(Keycode::S),
            'T' => Some(Keycode::T),
            'U' => Some(Keycode::U),
            'V' => Some(Keycode::V),
            'W' => Some(Keycode::W),
            'X' => Some(Keycode::X),
            'Y' => Some(Keycode::Y),
            'Z' => Some(Keycode::Z),
            _ => None,
        }
    }
    keycode
}
