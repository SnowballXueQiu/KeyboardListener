use crate::logger;
use std::ffi::c_void;
use std::mem::ManuallyDrop;
use windows::Win32::Foundation::{BOOL, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageA, GetMessageA, SetWindowsHookExA,
    TranslateMessage, HHOOK, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    WM_SYSKEYDOWN, WM_SYSKEYUP,
};

pub unsafe extern "system" fn kbd_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // SAFETY: lparam must be a valid KBDLLHOOKSTRUCT pointer
    let ctx = unsafe { *(lparam.0 as *const KBDLLHOOKSTRUCT) };
    match wparam.0 as u32 {
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            if let Some(key) = get_key_mapping(VIRTUAL_KEY(ctx.vkCode as u16)) {
                logger::log_event("按下按键", key.as_str());
            }
        }
        WM_KEYUP | WM_SYSKEYUP => {
            if let Some(key) = get_key_mapping(VIRTUAL_KEY(ctx.vkCode as u16)) {
                logger::log_event("松开按键", key.as_str());
            }
        }
        _ => panic!("unexpected key state change"),
    }
    // SAFETY: All parameters are valid
    unsafe { CallNextHookEx(HHOOK::default(), code, wparam, lparam) }
}
/// 管理当前按键的状态
pub fn monitor_keyboard() {
    /* SAFETY: kbd_hook is a valid function pointer */
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

/// 获取键盘输入的字符映射
pub fn get_key_mapping(key: VIRTUAL_KEY) -> Option<String> {
    if let VK_A = key {
        Some("A".into())
    } else if let VK_B = key {
        Some("B".into())
    } else if let VK_C = key {
        Some("C".into())
    } else if let VK_D = key {
        Some("D".into())
    } else if let VK_E = key {
        Some("E".into())
    } else if let VK_F = key {
        Some("F".into())
    } else if let VK_G = key {
        Some("G".into())
    } else if let VK_H = key {
        Some("H".into())
    } else if let VK_I = key {
        Some("I".into())
    } else if let VK_J = key {
        Some("J".into())
    } else if let VK_K = key {
        Some("K".into())
    } else if let VK_L = key {
        Some("L".into())
    } else if let VK_M = key {
        Some("M".into())
    } else if let VK_N = key {
        Some("N".into())
    } else if let VK_O = key {
        Some("O".into())
    } else if let VK_P = key {
        Some("P".into())
    } else if let VK_Q = key {
        Some("Q".into())
    } else if let VK_R = key {
        Some("R".into())
    } else if let VK_S = key {
        Some("S".into())
    } else if let VK_T = key {
        Some("T".into())
    } else if let VK_U = key {
        Some("U".into())
    } else if let VK_V = key {
        Some("V".into())
    } else if let VK_W = key {
        Some("W".into())
    } else if let VK_X = key {
        Some("X".into())
    } else if let VK_Y = key {
        Some("Y".into())
    } else if let VK_Z = key {
        Some("Z".into())
    } else if let VK_1 = key {
        Some("1".into())
    } else if let VK_NUMPAD1 = key {
        Some("NUMPAD1".into())
    } else if let VK_2 = key {
        Some("2".into())
    } else if let VK_NUMPAD2 = key {
        Some("NUMPAD2".into())
    } else if let VK_3 = key {
        Some("3".into())
    } else if let VK_NUMPAD3 = key {
        Some("NUMPAD3".into())
    } else if let VK_4 = key {
        Some("4".into())
    } else if let VK_NUMPAD4 = key {
        Some("NUMPAD4".into())
    } else if let VK_5 = key {
        Some("5".into())
    } else if let VK_NUMPAD5 = key {
        Some("NUMPAD5".into())
    } else if let VK_6 = key {
        Some("6".into())
    } else if let VK_NUMPAD6 = key {
        Some("NUMPAD6".into())
    } else if let VK_7 = key {
        Some("7".into())
    } else if let VK_NUMPAD7 = key {
        Some("NUMPAD7".into())
    } else if let VK_8 = key {
        Some("8".into())
    } else if let VK_NUMPAD8 = key {
        Some("NUMPAD8".into())
    } else if let VK_9 = key {
        Some("9".into())
    } else if let VK_NUMPAD9 = key {
        Some("NUMPAD9".into())
    } else if let VK_0 = key {
        Some("0".into())
    } else if let VK_NUMPAD0 = key {
        Some("NUMPAD0".into())
    } else if let VK_NUMLOCK = key {
        Some("NUMLOCK".into())
    } else if let VK_SPACE = key {
        Some("Space".into())
    } else if let VK_SHIFT = key {
        Some("Shift".into())
    } else if let VK_CONTROL = key {
        Some("Ctrl".into())
    } else if let VK_MENU = key {
        Some("Alt".into())
    } else if let VK_BACK = key {
        Some("Backspace".into())
    } else if let VK_ESCAPE = key {
        Some("Escape".into())
    } else if let VK_RETURN = key {
        Some("Enter".into())
    } else if let VK_UP = key {
        Some("Up".into())
    } else if let VK_LEFT = key {
        Some("Left".into())
    } else if let VK_DOWN = key {
        Some("Down".into())
    } else if let VK_RIGHT = key {
        Some("Right".into())
    } else if let VK_F1 = key {
        Some("F1".into())
    } else if let VK_F2 = key {
        Some("F2".into())
    } else if let VK_F3 = key {
        Some("F3".into())
    } else if let VK_F4 = key {
        Some("F4".into())
    } else if let VK_F5 = key {
        Some("F5".into())
    } else if let VK_F6 = key {
        Some("F6".into())
    } else if let VK_F7 = key {
        Some("F7".into())
    } else if let VK_F8 = key {
        Some("F8".into())
    } else if let VK_F9 = key {
        Some("F9".into())
    } else if let VK_F10 = key {
        Some("F10".into())
    } else if let VK_F11 = key {
        Some("F11".into())
    } else if let VK_F12 = key {
        Some("F12".into())
    } else if let VK_CAPITAL = key {
        Some("CapsLock".into())
    } else {
        None
    }
}
