use super::OutputSink;
use anyhow::{Context, Result};
use image::RgbImage;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use v4l::{Device, FourCC, Format};

pub struct V4L2Output {
    file: File,
    width: u32,
    height: u32,
}

impl V4L2Output {
    pub fn new<P: AsRef<Path>>(device_path: P, width: u32, height: u32) -> Result<Self> {
        let path = device_path.as_ref();
        tracing::info!(
            "Opening v4l2loopback device at {} ({}x{})",
            path.display(),
            width,
            height
        );

        // Open the v4l2 device for format configuration
        let device = Device::with_path(path)
            .with_context(|| format!("Failed to open v4l2loopback device at {}", path.display()))?;

        // Set the output format to BGR0 (32-bit BGRA) at the requested resolution
        let format = Format::new(width, height, FourCC::new(b"BGR4"));

        // Configure the format using ioctl
        let format = v4l::video::Output::set_format(&device, &format)
            .context("Failed to set v4l2 device format")?;

        tracing::info!(
            "v4l2loopback device configured: {}x{}, fourcc: {}",
            format.width,
            format.height,
            format.fourcc
        );

        // Drop the Device to close it
        drop(device);

        // Now open the device as a regular file for writing
        let file = File::options()
            .write(true)
            .open(path)
            .with_context(|| format!("Failed to open v4l2loopback device for writing at {}", path.display()))?;

        Ok(Self {
            file,
            width,
            height,
        })
    }

    /// Convert RGB image to BGRA bytes (32-bit with alpha)
    fn rgb_to_bgra(rgb_image: &RgbImage) -> Vec<u8> {
        let (width, height) = rgb_image.dimensions();
        let mut bgra = Vec::with_capacity((width * height * 4) as usize);

        for pixel in rgb_image.pixels() {
            // BGRA format: Blue, Green, Red, Alpha
            bgra.push(pixel[2]); // B
            bgra.push(pixel[1]); // G
            bgra.push(pixel[0]); // R
            bgra.push(255);      // A (fully opaque)
        }

        bgra
    }
}

/// Convert RGB to YUV color space
fn rgb_to_yuv(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32;
    let g = g as f32;
    let b = b as f32;

    let y = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 255.0) as u8;
    let u = ((-0.147 * r - 0.289 * g + 0.436 * b) + 128.0).clamp(0.0, 255.0) as u8;
    let v = ((0.615 * r - 0.515 * g - 0.100 * b) + 128.0).clamp(0.0, 255.0) as u8;

    (y, u, v)
}

impl OutputSink for V4L2Output {
    fn write_frame(&mut self, frame: &RgbImage) -> Result<()> {
        // Resize frame if needed
        let frame = if frame.dimensions() != (self.width, self.height) {
            image::imageops::resize(
                frame,
                self.width,
                self.height,
                image::imageops::FilterType::Lanczos3,
            )
        } else {
            frame.clone()
        };

        // Convert to BGRA (32-bit with alpha)
        let bgra_data = Self::rgb_to_bgra(&frame);

        // Write directly to the device file
        self.file
            .write_all(&bgra_data)
            .context("Failed to write frame to v4l2loopback device")?;

        Ok(())
    }

    fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
