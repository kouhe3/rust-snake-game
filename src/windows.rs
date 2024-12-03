use crate::{DIRECTION, Direction};
use windows::Win32::Foundation::LRESULT;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageA, GetMessageA, HC_ACTION, TranslateMessage, WM_KEYDOWN,
};
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::WindowsAndMessaging::{KBDLLHOOKSTRUCT, SetWindowsHookExA, WH_KEYBOARD_LL},
};

pub fn create_hook() {
    //start hook for listen keyboard input
    let hh = unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(LowLevelKeyboardProc), None, 0) };
    if hh.is_err() {
        panic!("hook error");
    }
    //msg loop
    unsafe {
        let mut msg = std::mem::zeroed();
        while GetMessageA(&mut msg, None, 0, 0).0 != 0 {
            TranslateMessage(&mut msg);
            DispatchMessageA(&mut msg);
        }
    }
}

#[allow(non_snake_case)]
unsafe extern "system" fn LowLevelKeyboardProc(
    nCode: i32,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    if nCode as u32 == HC_ACTION {
        let kb_data = unsafe { *(lParam.0 as *mut KBDLLHOOKSTRUCT) };
        if wParam.0 as u32 == WM_KEYDOWN {
            match kb_data.vkCode {
                0x25 => *DIRECTION.lock().unwrap() = Direction::Left,
                0x26 => *DIRECTION.lock().unwrap() = Direction::Up,
                0x27 => *DIRECTION.lock().unwrap() = Direction::Right,
                0x28 => *DIRECTION.lock().unwrap() = Direction::Down,
                _ => {}
            }
        }
    }

    unsafe { CallNextHookEx(None, nCode, wParam, lParam) }
}
