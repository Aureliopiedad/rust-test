use windows::{core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        EnumWindows(Some(find_window), LPARAM(0));
    }
}

extern "system" fn find_window(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, &mut text);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if !text.is_empty() && IsWindowVisible(window).into() {
            println!("{text}");
            return true.into();
        }

        true.into()
    }
}
