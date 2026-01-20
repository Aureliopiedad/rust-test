mod image_capture;
mod video_capture;

use std::io::Write;

use windows_capture::capture::GraphicsCaptureApiHandler;
use windows_capture::graphics_capture_picker::GraphicsCapturePicker;
use windows_capture::settings::{
    ColorFormat, CursorCaptureSettings, DirtyRegionSettings, DrawBorderSettings,
    MinimumUpdateIntervalSettings, SecondaryWindowSettings, Settings,
};
use crate::image_capture::ImageCapture;
use crate::video_capture::VideoCapture;

fn main() {
    // Opens a dialog to pick a window or screen to capture; refer to the docs for other capture items.
    let item = GraphicsCapturePicker::pick_item().expect("Failed to pick item");

    // If the user canceled the selection, exit.
    let Some(item) = item else {
        println!("No item selected");
        return;
    };

    // Get the size of the item to pass to the settings.
    let size = item.size().expect("Failed to get item size");

    let settings = Settings::new(
        // Item to capture
        item,
        // Capture cursor settings
        CursorCaptureSettings::Default,
        // Draw border settings
        DrawBorderSettings::Default,
        // Secondary window settings, if you want to include secondary windows in the capture
        SecondaryWindowSettings::Default,
        // Minimum update interval, if you want to change the frame rate limit (default is 60 FPS or 16.67 ms)
        MinimumUpdateIntervalSettings::Default,
        // Dirty region settings
        DirtyRegionSettings::Default,
        // The desired color format for the captured frame.
        ColorFormat::Rgba8,
        // Additional flags for the capture settings that will be passed to the user-defined `new` function.
        size,
    );

    // Starts the capture and takes control of the current thread.
    // The errors from the handler trait will end up here.
    // VideoCapture::start(settings).expect("Screen capture failed");
    ImageCapture::start(settings).expect("Image capture failed");
}