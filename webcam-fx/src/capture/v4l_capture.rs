use super::CaptureSource;
use anyhow::{Context, Result};
use image::RgbImage;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;

pub struct WebcamCapture {
    camera: Camera,
    width: u32,
    height: u32,
}

impl WebcamCapture {
    pub fn new(device_index: u32, width: u32, height: u32) -> Result<Self> {
        tracing::info!(
            "Initializing webcam {} at {}x{}",
            device_index,
            width,
            height
        );

        let index = CameraIndex::Index(device_index);
        let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestResolution);

        let mut camera = Camera::new(index, requested)
            .context("Failed to open camera")?;

        camera.open_stream()
            .context("Failed to open camera stream")?;

        tracing::info!("Webcam initialized successfully");

        Ok(Self {
            camera,
            width,
            height,
        })
    }
}

impl CaptureSource for WebcamCapture {
    fn capture_frame(&mut self) -> Result<RgbImage> {
        let frame = self
            .camera
            .frame()
            .context("Failed to capture frame")?;

        let decoded = frame.decode_image::<RgbFormat>()
            .context("Failed to decode frame")?;

        Ok(decoded)
    }

    fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
