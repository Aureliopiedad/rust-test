use std::time::Instant;
use windows_capture::capture::{Context, GraphicsCaptureApiHandler};
use windows_capture::encoder::{ImageEncoder, ImageEncoderPixelFormat, ImageFormat};
use windows_capture::frame::Frame;
use windows_capture::graphics_capture_api::InternalCaptureControl;

pub struct ImageCapture {
    encoder: Option<ImageEncoder>,
    start: Instant
}

impl GraphicsCaptureApiHandler for ImageCapture {
    /// The type of flags used to get the values from the settings, here they are the width and height.
    type Flags = (i32, i32);

    /// The type of error that can be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    /// Function that will be called to create a new instance. The flags can be passed from settings.
    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        let encoder = ImageEncoder::new(ImageFormat::Png, ImageEncoderPixelFormat::Bgra8)?;
        Ok(Self{encoder: Some(encoder), start: Instant::now()})
    }

    /// Called every time a new frame is available.
    fn on_frame_arrived(&mut self, frame: &mut Frame, capture_control: InternalCaptureControl) -> Result<(), Self::Error> {
        let mut buffer = frame.buffer()?;
        buffer.save_as_image("screenshot.png", ImageFormat::Png)?;

        capture_control.stop();
        Ok(())
    }
}