use image::{ImageBuffer, Rgba};
use std::ffi::c_void;
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, RGBQUAD, SRCCOPY};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowRect};

fn main() {
    let windows_option = find_windows();
    if windows_option.is_none() {
        return;
    }

    let windows = windows_option.unwrap();
    let windows_rect = get_window_rect(windows_option.unwrap());
    if windows_rect.is_none() {
        return;
    }

    let _ = capture_windows_in_memory(windows, windows_rect.unwrap());
}

fn find_windows() -> Option<HWND> {
    let find_window_result = unsafe { FindWindowW(PCWSTR::null(), w!("Terminal")) };
    match find_window_result {
        Ok(h_wnd) => {
            Some(h_wnd)
        },
        Err(_) => {
            println!("No windows named {} found", "Terminal");
            None
        }
    }
}

fn get_window_rect(h_wnd: HWND) -> Option<RECT> {
    let mut rect: RECT = RECT { left: (0), top: (0), right: (0), bottom: (0) };
    unsafe {
        match GetWindowRect(h_wnd, &mut rect) {
            Ok(_) => {
                println!("{:?}", rect);
                Some(rect)
            },
            Err(_) => {
                println!("No window rect found");
                None
            }
        }
    }
}

fn capture_windows_in_memory(h_wnd: HWND, rect: RECT) -> Result<String, String> {
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;

    // A device context is a structure that defines a set of graphic objects and their associated attributes, as well as the graphic modes that affect output.
    // 简单来说，设备上下文(DC)就是图形对象；支持画笔、复制屏幕的位图等等
    // 这里获取的是已存在的窗口的dc
    let device_context = unsafe { GetDC(Some(h_wnd)) };
    if device_context.is_invalid() {
        return Err(String::from("can not get dc"));
    }

    // To enable applications to place output in memory rather than sending it to an actual device, use a special device context for bitmap operations called a memory device context.
    // 简单来说就是存在内存中
    // An application can create a memory DC by calling the CreateCompatibleDC function.
    // 创建和当前窗口dc兼容的内存中的dc
    let mem_dc = unsafe { CreateCompatibleDC(Some(device_context)) };
    if mem_dc.is_invalid() {
        unsafe { ReleaseDC(Some(h_wnd), device_context) };
        return Err(String::from("can not create compatible dc"));
    }

    // A bitmap is a graphical object used to create, manipulate (scale, scroll, rotate, and paint), and store images as files on a disk.
    // To create a bitmap of the appropriate dimensions, use the CreateBitmap, CreateBitmapIndirect, or CreateCompatibleBitmap function.
    // The CreateCompatibleBitmap function creates a bitmap compatible with the device that is associated with the specified device context.
    // cx: The bitmap width, in pixels. cy: The bitmap height, in pixels.
    let bitmap: HBITMAP = unsafe { CreateCompatibleBitmap(device_context, width, height) };
    if bitmap.is_invalid() {
        unsafe {
            let _ = DeleteDC(mem_dc);
            ReleaseDC(Some(h_wnd), mem_dc);
            return Err(String::from("can not create compatible bitmap"));
        }
    }

    // The SelectObject function selects an object into the specified device context (DC). The new object replaces the previous object of the same type.
    // Before an application can begin drawing, it must select a bitmap with the appropriate width and height into the DC by calling the SelectObject function.
    // 创建位图后，将该位图放到内存dc中
    let old_bitmap = unsafe { SelectObject(mem_dc, bitmap.into()) };
    unsafe { if old_bitmap.is_invalid() {
        let _ = DeleteObject(bitmap.into());
        let _ = DeleteDC(mem_dc);
        ReleaseDC(Some(h_wnd), mem_dc);
        return Err(String::from("can not create compatible bitmap"));
    } };

    // The BitBlt function performs a bit-block transfer of the color data corresponding to a rectangle of pixels from the specified source device context into a destination device context.
    // 用于将窗口dc复制到内存dc上
    unsafe { match BitBlt(mem_dc, 0, 0, width, height, Some(device_context), rect.left, rect.top, SRCCOPY) {
        Ok(_) => {},
        Err(_) => {
            let _ = DeleteObject(bitmap.into());
            let _ = DeleteDC(mem_dc);
            ReleaseDC(Some(h_wnd), mem_dc);
            return Err(String::from("can not create compatible bitmap"));
        }
    } };

    // 下一步是将位图存储到文件中
    // 首先需要确定位图的文件格式，一般来说有几种 BITMAPINFOHEADER、BITMAPV4HEADER或 BITMAPV5HEADER
    // 位图信息标头结构RGBQUAD
    // 索引数据，也就是实际位图数据
    let mut bitmap_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0
        },
        bmiColors: [RGBQUAD {rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0}; 1]
    };

    let mut pixel_data: Vec<u8> = vec![0; (width * height * 4) as usize];

    // The GetDIBits function retrieves the bits of the specified compatible bitmap and copies them into a buffer as a DIB using the specified format.
    // 读取位图到缓冲区
    let get_di_bits_result = unsafe { GetDIBits(mem_dc, bitmap, 0, height as u32, Some(pixel_data.as_mut_ptr() as *mut c_void), &mut bitmap_info, DIB_RGB_COLORS) };
    println!("{:?}", get_di_bits_result);

    // let _ = unsafe { DeleteObject(bitmap.into()) };
    // let _ = unsafe { DeleteDC(mem_dc) };
    // unsafe { ReleaseDC(Some(h_wnd), device_context) };

    for i in (0..pixel_data.len()).step_by(4) {
        pixel_data.swap(i, i + 2); // 交换 B 和 R
    }

    let img: Result<ImageBuffer<Rgba<u8>, Vec<u8>>, &str> =
        ImageBuffer::from_raw(width as u32, height as u32, pixel_data).ok_or("failed to create imageBuffer");
    image::imageops::flip_vertical(&img.unwrap()).save("capture.png");

    Ok(String::from(""))
}