use image::{ImageBuffer, Rgb, Rgba};
use windows::Win32::Foundation::{FALSE, HWND, RECT};
use windows::Win32::Graphics::Gdi::{BITMAP, CreateCompatibleDC, DeleteDC, GetDC, HALFTONE, HBITMAP, HDC, ReleaseDC, SRCCOPY, SetStretchBltMode, StretchBlt, CreateCompatibleBitmap, DeleteObject, SelectObject, BitBlt, GetObjectW, BITMAPFILEHEADER, BITMAPINFOHEADER, BI_RGB, GetWindowDC, GetDIBits, BITMAPINFO, DIB_RGB_COLORS, RGBQUAD};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetClientRect, GetSystemMetrics, GetWindowRect, SetForegroundWindow, ShowWindow, SM_CXSCREEN, SM_CXVIRTUALSCREEN, SM_CYSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SW_RESTORE};
use windows::core::w;
use windows::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2};

fn main() {
    let x = unsafe { GetSystemMetrics(SM_XVIRTUALSCREEN) };
    let y = unsafe { GetSystemMetrics(SM_YVIRTUALSCREEN) };

    unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2); }

    let h_wnd = find_window();
    if h_wnd.is_none() {
        println!("can not find hwnd");
        return;
    }

    unsafe {
        ShowWindow(h_wnd.unwrap(), SW_RESTORE);
        SetForegroundWindow(h_wnd.unwrap());
    }

    let mut rect = RECT::default();
    unsafe { GetClientRect(h_wnd.unwrap(), &mut rect) };
    // let width = rect.right - rect.left;
    // let height = rect.bottom - rect.top;

    let width = unsafe { GetSystemMetrics(SM_CXVIRTUALSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYVIRTUALSCREEN) };

    let hdc_window = unsafe { GetDC(None) };
    let hdc_mem = unsafe { CreateCompatibleDC(Some(hdc_window)) };
    let hbitmap = unsafe { CreateCompatibleBitmap(hdc_window, width, height) };
    let hdc_old = unsafe { SelectObject(hdc_mem, hbitmap.into()) };

    unsafe {
        BitBlt(
            hdc_mem,
            0, 0,
            width, height,
            Some(hdc_window),
            x, y,
            SRCCOPY,
        );
    }

    let bi: BITMAPINFOHEADER = BITMAPINFOHEADER {
        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: width,
        biHeight: -height, // 负高度表示从上到下的像素顺序
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB.0,
        ..Default::default()
    };

    let mut pixels = vec![0u8; (width * height * 4) as usize];
    let mut bits_ptr: *mut u8 = pixels.as_mut_ptr();

    unsafe {
        GetDIBits(
            GetDC(None),
            hbitmap,
            0,
            height as u32,
            Some(&mut bits_ptr as *mut _ as *mut _),
            &mut BITMAPINFO { bmiHeader: bi, bmiColors: [RGBQUAD { rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 }; 1] },
            DIB_RGB_COLORS,
        );
    }

    let img = ImageBuffer::<Rgb<u8>, _>::from_raw(width as u32, height as u32, pixels);
    img.unwrap().save("capture.png");
}

fn find_window() -> Option<HWND> {
    unsafe {
        match FindWindowW(None, w!("*asdasdasd - Notepad")) {
            Ok(h_wnd) => Some(h_wnd),
            Err(_) => None,
        }
    }
}

fn find_rect(h_wnd: HWND) -> Option<RECT> {
    let mut rect: RECT = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    unsafe {
        // match GetWindowRect(h_wnd, &mut rect) {
        match GetClientRect(h_wnd, &mut rect) {
            Ok(_) => Some(rect),
            Err(_) => None,
        }
    }
}
