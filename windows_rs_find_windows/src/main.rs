use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
    Win32::Graphics::Gdi::*,
};

fn main() {
    // let hard_match_hwnd = hard_match(w!("Terminal"));
    try_match(String::from("Terminal"));
}

fn hard_match(title: PCWSTR) -> Option<HWND> {
    let hard_match_title = title;
    let hwnd_result = unsafe { FindWindowW(None, hard_match_title) };

    match hwnd_result {
        Ok(_hwnd) => Some(_hwnd),
        Err(_) => None,
    }
}

#[derive(Debug)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TryMatchWindowInfo {
    window_name: String,
    window: Option<HWND>
}

fn try_match(title: String) -> Option<HWND> {
    let mut match_result: TryMatchWindowInfo = TryMatchWindowInfo {
        window_name: title,
        window: None
    };
    unsafe {
        let result_ptr : *mut TryMatchWindowInfo = &mut match_result;
        let hwnd_result = EnumWindows(Some(enum_windows_handler), LPARAM(result_ptr as isize));
        match hwnd_result {
            Ok(_hwnd) => {},
            Err(_) => return None,
        };
    }

    return  match_result.window;
}

extern "system" fn enum_windows_handler(hwnd: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let match_result = *(lparam.0 as *mut TryMatchWindowInfo);
    }

    TRUE
}