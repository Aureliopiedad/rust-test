use std::io;
use std::io::Write;
use std::time::Instant;
use windows_capture::capture::{Context, GraphicsCaptureApiHandler};
use windows_capture::encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder};
use windows_capture::frame::Frame;
use windows_capture::graphics_capture_api::InternalCaptureControl;

pub struct VideoCapture {
    encoder: Option<VideoEncoder>,
    start: Instant
}

impl GraphicsCaptureApiHandler for VideoCapture {
    // The type of flags used to get the values from the settings, here they are the width and height.
    type Flags = (i32, i32);

    // The type of error that can be returned from `CaptureControl` and `start`
    // functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create a new instance. The flags can be
    // passed from settings.
    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        // If we didn't want to get the size from the settings, we could use frame.width() and frame.height()
        // in the on_frame_arrived function, but we would need to create the encoder there.
        let encoder = VideoEncoder::new(
            VideoSettingsBuilder::new(ctx.flags.0 as u32, ctx.flags.1 as u32),
            AudioSettingsBuilder::default().disabled(true),
            ContainerSettingsBuilder::default(),
            "video.mp4",
        )?;

        Ok(Self { encoder: Some(encoder), start: Instant::now() })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        print!("\rRecording for: {} seconds", self.start.elapsed().as_secs());
        io::stdout().flush()?;

        // Send the frame to the video encoder
        self.encoder.as_mut().unwrap().send_frame(frame)?;

        // The frame has other uses too, for example, you can save a single frame
        // to a file, like this: frame.save_as_image("frame.png", ImageFormat::Png)?;
        // Or get the raw data like this so you have full control: let data = frame.buffer()?;

        // Stop the capture after 6 seconds
        if self.start.elapsed().as_secs() >= 6 {
            // Finish the encoder and save the video.
            self.encoder.take().unwrap().finish()?;

            capture_control.stop();

            // Because the previous prints did not include a newline.
            println!();
        }

        Ok(())
    }

    // Optional handler called when the capture item (usually a window) is closed.
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture session ended");

        Ok(())
    }
}