use crate::logger;
use crate::key_mapping::get_key_mapping;
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
