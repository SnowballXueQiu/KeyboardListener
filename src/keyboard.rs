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

pub fn get_key_mapping(key: VIRTUAL_KEY) -> Option<String> {
    match key {
        VK_A => Some("A".into()),
        VK_B => Some("B".into()),
        VK_C => Some("C".into()),
        VK_D => Some("D".into()),
        VK_E => Some("E".into()),
        VK_F => Some("F".into()),
        VK_G => Some("G".into()),
        VK_H => Some("H".into()),
        VK_I => Some("I".into()),
        VK_J => Some("J".into()),
        VK_K => Some("K".into()),
        VK_L => Some("L".into()),
        VK_M => Some("M".into()),
        VK_N => Some("N".into()),
        VK_O => Some("O".into()),
        VK_P => Some("P".into()),
        VK_Q => Some("Q".into()),
        VK_R => Some("R".into()),
        VK_S => Some("S".into()),
        VK_T => Some("T".into()),
        VK_U => Some("U".into()),
        VK_V => Some("V".into()),
        VK_W => Some("W".into()),
        VK_X => Some("X".into()),
        VK_Y => Some("Y".into()),
        VK_Z => Some("Z".into()),
        VK_0 => Some("0".into()),
        VK_1 => Some("1".into()),
        VK_2 => Some("2".into()),
        VK_3 => Some("3".into()),
        VK_4 => Some("4".into()),
        VK_5 => Some("5".into()),
        VK_6 => Some("6".into()),
        VK_7 => Some("7".into()),
        VK_8 => Some("8".into()),
        VK_9 => Some("9".into()),
        VK_F1 => Some("F1".into()),
        VK_F2 => Some("F2".into()),
        VK_F3 => Some("F3".into()),
        VK_F4 => Some("F4".into()),
        VK_F5 => Some("F5".into()),
        VK_F6 => Some("F6".into()),
        VK_F7 => Some("F7".into()),
        VK_F8 => Some("F8".into()),
        VK_F9 => Some("F9".into()),
        VK_F10 => Some("F10".into()),
        VK_F11 => Some("F11".into()),
        VK_F12 => Some("F12".into()),
        VK_LSHIFT => Some("LShift".into()),
        VK_RSHIFT => Some("RShift".into()),
        VK_LCONTROL => Some("LCtrl".into()),
        VK_RCONTROL => Some("RCtrl".into()),
        VK_LMENU => Some("LAlt".into()),
        VK_RMENU => Some("RAlt".into()),
        VK_LWIN => Some("LWin".into()),
        VK_RWIN => Some("RWin".into()),
        VK_OEM_1 => Some(";".into()),
        VK_OEM_2 => Some("/".into()),
        VK_OEM_3 => Some("`".into()),
        VK_OEM_4 => Some("[".into()),
        VK_OEM_5 => Some("\\".into()),
        VK_OEM_6 => Some("]".into()),
        VK_OEM_7 => Some("'".into()),
        VK_OEM_COMMA => Some(",".into()),
        VK_OEM_MINUS => Some("-".into()),
        VK_OEM_PERIOD => Some(".".into()),
        VK_OEM_PLUS => Some("=".into()),
        VK_ESCAPE => Some("Escape".into()),
        VK_SPACE => Some("Space".into()),
        VK_RETURN => Some("Enter".into()),
        VK_TAB => Some("Tab".into()),
        VK_BACK => Some("Backspace".into()),
        VK_CAPITAL => Some("CapsLock".into()),
        VK_UP => Some("Up".into()),
        VK_DOWN => Some("Down".into()),
        VK_LEFT => Some("Left".into()),
        VK_RIGHT => Some("Right".into()),
        VK_HOME => Some("Home".into()),
        VK_END => Some("End".into()),
        VK_PRIOR => Some("PageUp".into()),
        VK_NEXT => Some("PageDown".into()),
        VK_INSERT => Some("Insert".into()),
        VK_DELETE => Some("Delete".into()),
        VK_SNAPSHOT => Some("PrintScreen".into()),
        VK_SCROLL => Some("ScrollLock".into()),
        VK_PAUSE => Some("Pause".into()),
        VK_NUMLOCK => Some("NUMLOCK".into()),
        VK_NUMPAD0 => Some("NUMPAD0".into()),
        VK_NUMPAD1 => Some("NUMPAD1".into()),
        VK_NUMPAD2 => Some("NUMPAD2".into()),
        VK_NUMPAD3 => Some("NUMPAD3".into()),
        VK_NUMPAD4 => Some("NUMPAD4".into()),
        VK_NUMPAD5 => Some("NUMPAD5".into()),
        VK_NUMPAD6 => Some("NUMPAD6".into()),
        VK_NUMPAD7 => Some("NUMPAD7".into()),
        VK_NUMPAD8 => Some("NUMPAD8".into()),
        VK_NUMPAD9 => Some("NUMPAD9".into()),
        VK_MULTIPLY => Some("Numpad*".into()),
        VK_ADD => Some("Numpad+".into()),
        VK_SUBTRACT => Some("Numpad-".into()),
        VK_DECIMAL => Some("Numpad.".into()),
        VK_DIVIDE => Some("Numpad/".into()),
        _ => None,
    }
}
