use std::sync::OnceLock;
use std::sync::mpsc::Sender;
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VIRTUAL_KEY, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, GetMessageW, HHOOK, KBDLLHOOKSTRUCT, MSG, SetWindowsHookExW,
    UnhookWindowsHookEx, WH_KEYBOARD_LL, WM_KEYDOWN, WM_SYSKEYDOWN,
};

pub struct ParsedBinding {
    pub command: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub win: bool,
    pub vk: VIRTUAL_KEY,
}

#[derive(Clone, Copy)]
struct HookHandle(HHOOK);
unsafe impl Send for HookHandle {}
unsafe impl Sync for HookHandle {}

static HHOOK_HANDLE: OnceLock<HookHandle> = OnceLock::new();
static BINDINGS: OnceLock<Vec<ParsedBinding>> = OnceLock::new();
static COMMAND_SENDER: OnceLock<Sender<String>> = OnceLock::new();

unsafe extern "system" fn keyboard_hook_callback(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // 2024 edition requires unsafe blocks inside unsafe fn
    unsafe {
        if code >= 0 && (wparam.0 as u32 == WM_KEYDOWN || wparam.0 as u32 == WM_SYSKEYDOWN) {
            let kbd_struct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            let key = VIRTUAL_KEY(kbd_struct.vkCode as u16);

            let ctrl_pressed = (GetAsyncKeyState(VK_CONTROL.0 as i32) as u16 & 0x8000) != 0;
            let alt_pressed = (GetAsyncKeyState(VK_MENU.0 as i32) as u16 & 0x8000) != 0;
            let shift_pressed = (GetAsyncKeyState(VK_SHIFT.0 as i32) as u16 & 0x8000) != 0;
            let win_pressed = (GetAsyncKeyState(VK_LWIN.0 as i32) as u16 & 0x8000) != 0
                || (GetAsyncKeyState(VK_RWIN.0 as i32) as u16 & 0x8000) != 0;

            if let Some(bindings) = BINDINGS.get() {
                for binding in bindings {
                    if binding.vk == key
                        && binding.ctrl == ctrl_pressed
                        && binding.alt == alt_pressed
                        && binding.shift == shift_pressed
                        && binding.win == win_pressed
                    {
                        if let Some(sender) = COMMAND_SENDER.get() {
                            let _ = sender.send(binding.command.clone());
                        }
                        return LRESULT(1); // Swallow the keypress
                    }
                }
            }
        }

        let hook = HHOOK_HANDLE
            .get()
            .copied()
            .map(|h| h.0)
            .unwrap_or(HHOOK::default());
        CallNextHookEx(hook, code, wparam, lparam)
    }
}

pub fn start_hook(bindings: Vec<ParsedBinding>, sender: Sender<String>) -> Result<(), String> {
    if BINDINGS.set(bindings).is_err() {
        return Err("Bindings already set".into());
    }
    if COMMAND_SENDER.set(sender).is_err() {
        return Err("Sender already set".into());
    }

    unsafe {
        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_callback),
            windows::Win32::Foundation::HINSTANCE::default(),
            0,
        )
        .map_err(|e| format!("Failed to install keyboard hook: {}", e))?;

        let _ = HHOOK_HANDLE.set(HookHandle(hook));

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = windows::Win32::UI::WindowsAndMessaging::TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        UnhookWindowsHookEx(hook).map_err(|e| format!("Failed to unhook: {}", e))?;
    }

    Ok(())
}
