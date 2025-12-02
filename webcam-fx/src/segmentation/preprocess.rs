use anyhow::Result;
use image::{imageops, RgbImage};
use ndarray::{Array4, s};

/// Preprocessor for converting RGB images to model input tensors
pub struct Preprocessor {
    target_width: u32,
    target_height: u32,
}

impl Preprocessor {
    pub fn new(target_width: u32, target_height: u32) -> Self {
        Self {
            target_width,
            target_height,
        }
    }

    /// Preprocess an RGB image into a normalized NCHW tensor
    ///
    /// Steps:
    /// 1. Resize to target dimensions
    /// 2. Convert to float and normalize to [0, 1]
    /// 3. Transpose from HWC to NCHW format
    ///
    /// Returns: Array4<f32> with shape [1, 3, height, width]
    pub fn preprocess(&self, image: &RgbImage) -> Result<Array4<f32>> {
        let _span = tracing::debug_span!("preprocess").entered();

        // Resize if needed
        let resized = if image.dimensions() != (self.target_width, self.target_height) {
            imageops::resize(
                image,
                self.target_width,
                self.target_height,
                imageops::FilterType::Lanczos3,
            )
        } else {
            image.clone()
        };

        // Convert to NCHW tensor and normalize
        let (width, height) = resized.dimensions();
        let mut tensor = Array4::<f32>::zeros((1, 3, height as usize, width as usize));

        for y in 0..height {
            for x in 0..width {
                let pixel = resized.get_pixel(x, y);

                // Normalize to [0, 1]
                let r = pixel[0] as f32 / 255.0;
                let g = pixel[1] as f32 / 255.0;
                let b = pixel[2] as f32 / 255.0;

                // Store in NCHW format
                tensor[[0, 0, y as usize, x as usize]] = r;
                tensor[[0, 1, y as usize, x as usize]] = g;
                tensor[[0, 2, y as usize, x as usize]] = b;
            }
        }

        Ok(tensor)
    }

    /// Postprocess model output matte back to original frame dimensions
    ///
    /// # Arguments
    /// * `matte` - Flattened matte at model resolution
    /// * `matte_width` - Width of the matte
    /// * `matte_height` - Height of the matte
    /// * `target_width` - Desired output width
    /// * `target_height` - Desired output height
    ///
    /// Returns: Resized matte flattened in row-major order
    pub fn postprocess_matte(
        matte: &[f32],
        matte_width: u32,
        matte_height: u32,
        target_width: u32,
        target_height: u32,
    ) -> Result<Vec<f32>> {
        let _span = tracing::debug_span!("postprocess").entered();

        // If dimensions match, no resize needed
        if matte_width == target_width && matte_height == target_height {
            return Ok(matte.to_vec());
        }

        // Convert to grayscale image for resizing
        let gray_image = image::GrayImage::from_fn(matte_width, matte_height, |x, y| {
            let idx = (y * matte_width + x) as usize;
            let value = (matte[idx] * 255.0).clamp(0.0, 255.0) as u8;
            image::Luma([value])
        });

        // Resize
        let resized = imageops::resize(
            &gray_image,
            target_width,
            target_height,
            imageops::FilterType::Lanczos3,
        );

        // Convert back to f32 values
        let output: Vec<f32> = resized
            .pixels()
            .map(|p| p[0] as f32 / 255.0)
            .collect();

        Ok(output)
    }

    /// Convert matte to grayscale RGB image for visualization
    pub fn matte_to_rgb(matte: &[f32], width: u32, height: u32) -> RgbImage {
        RgbImage::from_fn(width, height, |x, y| {
            let idx = (y * width + x) as usize;
            let value = (matte[idx] * 255.0).clamp(0.0, 255.0) as u8;
            image::Rgb([value, value, value])
        })
    }
}
