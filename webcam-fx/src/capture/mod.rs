mod v4l_capture;

pub use v4l_capture::WebcamCapture;

use anyhow::Result;
use image::RgbImage;

/// Trait for camera capture sources
pub trait CaptureSource {
    /// Capture a single frame
    fn capture_frame(&mut self) -> Result<RgbImage>;

    /// Get the resolution of captured frames
    fn resolution(&self) -> (u32, u32);
}
