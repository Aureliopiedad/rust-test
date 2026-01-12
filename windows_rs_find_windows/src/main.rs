use std::sync::OnceLock;
use std::os::windows::ffi::OsStrExt;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

static WINDOW_1: OnceLock<String> = OnceLock::new();

const HARD_MATCH_WINDOW_NAME: &str = "Terminal";
const TRY_MATCH_WINDOW_NAME: &str = "dafeihou";

fn main() {
    let hard_match_hwnd = hard_match();
    // try_match(String::from("Terminal"));

    println!("{}", WINDOW_1.get().unwrap());
}

fn str_2_u16(str: &str) -> Vec<u16> {
    std::ffi::OsStr::new(str)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn set_window_1_name(window_name: String) {
    WINDOW_1.set(window_name).expect("failed to set window 1");
}

fn hard_match() -> Option<HWND> {
    let hwnd_result = unsafe { FindWindowW(PCWSTR::null(), PCWSTR::from_raw(str_2_u16(HARD_MATCH_WINDOW_NAME).as_ptr())) };

    match hwnd_result {
        Ok(_hwnd) => {
            set_window_1_name(String::from(HARD_MATCH_WINDOW_NAME));
            Some(_hwnd)
        },
        Err(_) => {
            None
        }
    }
}

fn try_match() {
    unsafe {
        let hwnd_result = EnumWindows(Some(enum_windows_handler), None);
        match hwnd_result {
            Ok(_hwnd) => {},
            Err(_) => {},
        };
    }
}

extern "system" fn enum_windows_handler(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    unsafe {
        if IsWindowVisible(hwnd) == FALSE {
            return TRUE;
        }

        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(hwnd, &mut text);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if text.is_empty() {
            return TRUE;
        }

        if text.contains(TRY_MATCH_WINDOW_NAME) {
            set_window_1_name(text);
            return FALSE;
        }
    }

    TRUE
}