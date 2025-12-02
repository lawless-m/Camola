use anyhow::Result;
use image::RgbImage;

/// Alpha matte: grayscale values where 0.0 = background, 1.0 = foreground
/// Dimensions match the input frame dimensions
pub type Matte = Vec<f32>;

/// Trait for segmentation models
/// Allows swapping between different backends (RVM, MODNet, MediaPipe, etc.)
pub trait SegmentationModel {
    /// Process a frame and return an alpha matte
    ///
    /// # Arguments
    /// * `frame` - Input RGB frame
    ///
    /// # Returns
    /// * Alpha matte with values 0.0-1.0, flattened in row-major order
    fn segment(&mut self, frame: &RgbImage) -> Result<Matte>;

    /// Reset internal state (for models with temporal/recurrent components)
    ///
    /// Call this when:
    /// - Switching cameras
    /// - Scene cuts detected
    /// - Starting a new video session
    fn reset_state(&mut self) {
        // Default implementation: no-op for stateless models
    }

    /// Get the model's preferred input dimensions
    ///
    /// Returns (width, height)
    fn input_size(&self) -> (u32, u32);

    /// Get the output dimensions (usually matches input)
    ///
    /// Returns (width, height)
    fn output_size(&self) -> (u32, u32) {
        self.input_size()
    }
}
