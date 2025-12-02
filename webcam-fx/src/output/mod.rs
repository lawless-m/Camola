mod loopback;

pub use loopback::V4L2Output;

use anyhow::Result;
use image::RgbImage;

/// Trait for output destinations
pub trait OutputSink {
    /// Write a frame to the output
    fn write_frame(&mut self, frame: &RgbImage) -> Result<()>;

    /// Get the expected output resolution
    fn resolution(&self) -> (u32, u32);
}
