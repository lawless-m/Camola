use super::OutputSink;
use anyhow::{Context, Result};
use image::{ImageBuffer, Rgb, RgbImage};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::video::Capture;
use v4l::{Device, FourCC};

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

        // Open the device file directly for writing
        // v4l2loopback accepts raw frame data written to the device file
        let file = File::options()
            .write(true)
            .open(path)
            .with_context(|| format!("Failed to open v4l2loopback device at {}", path.display()))?;

        tracing::info!("v4l2loopback device opened successfully");

        Ok(Self {
            file,
            width,
            height,
        })
    }

    /// Convert RGB frame to YUV422 (YUYV) format
    /// v4l2loopback typically expects YUYV format
    fn rgb_to_yuyv(rgb_image: &RgbImage) -> Vec<u8> {
        let (width, height) = rgb_image.dimensions();
        let mut yuyv = Vec::with_capacity((width * height * 2) as usize);

        for y in 0..height {
            for x in (0..width).step_by(2) {
                let pixel1 = rgb_image.get_pixel(x, y);
                let pixel2 = if x + 1 < width {
                    rgb_image.get_pixel(x + 1, y)
                } else {
                    pixel1
                };

                // Convert RGB to YUV
                let (y1, u1, v1) = rgb_to_yuv(pixel1[0], pixel1[1], pixel1[2]);
                let (y2, u2, v2) = rgb_to_yuv(pixel2[0], pixel2[1], pixel2[2]);

                // Average U and V for the pair of pixels
                let u = ((u1 as u16 + u2 as u16) / 2) as u8;
                let v = ((v1 as u16 + v2 as u16) / 2) as u8;

                // YUYV format: Y0 U Y1 V
                yuyv.push(y1);
                yuyv.push(u);
                yuyv.push(y2);
                yuyv.push(v);
            }
        }

        yuyv
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

        // Convert RGB to YUYV
        let yuyv_data = Self::rgb_to_yuyv(&frame);

        // Write directly to the device file
        self.file
            .write_all(&yuyv_data)
            .context("Failed to write frame to v4l2loopback device")?;

        Ok(())
    }

    fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
