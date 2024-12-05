use windows::Win32::Foundation::LRESULT;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageA, GetMessageA, HC_ACTION, PostQuitMessage, TranslateMessage,
    WM_KEYDOWN,
};
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::WindowsAndMessaging::{KBDLLHOOKSTRUCT, SetWindowsHookExA, WH_KEYBOARD_LL},
};