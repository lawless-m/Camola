use super::preprocess::Preprocessor;
use super::types::{Matte, SegmentationModel};
use anyhow::{Context, Result};
use image::RgbImage;
use ndarray::{Array4, IxDyn};
use ort::{GraphOptimizationLevel, Session};
use std::path::Path;

/// RobustVideoMatting segmentation model
///
/// This model uses recurrent connections to maintain temporal consistency.
/// Hidden states (r1-r4) are carried between frames for smooth results.
pub struct RobustVideoMatting {
    session: Session,
    preprocessor: Preprocessor,
    width: u32,
    height: u32,

    // Recurrent hidden states
    // These are updated after each inference and fed back in the next frame
    r1: Option<Array4<f32>>,
    r2: Option<Array4<f32>>,
    r3: Option<Array4<f32>>,
    r4: Option<Array4<f32>>,

    // Downsample ratio for hidden states
    downsample_ratio: f32,
}

impl RobustVideoMatting {
    /// Create a new RVM model from an ONNX file
    ///
    /// # Arguments
    /// * `model_path` - Path to the ONNX model file
    ///
    /// # Default Configuration
    /// - Input size: 512x512 (can be adjusted for performance/quality tradeoff)
    /// - Downsample ratio: 0.25 (hidden states are 1/4 of input resolution)
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self> {
        let path = model_path.as_ref();

        tracing::info!("Loading RVM model from {}", path.display());

        // Configure ONNX Runtime with CUDA execution provider
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file(path)
            .with_context(|| format!("Failed to load model from {}", path.display()))?;

        tracing::info!("RVM model loaded successfully");
        tracing::debug!("Available execution providers: {:?}", session.metadata()?.producer_name()?);

        // Default to 512x512 input (good balance of quality and performance)
        let width = 512;
        let height = 512;

        let preprocessor = Preprocessor::new(width, height);

        Ok(Self {
            session,
            preprocessor,
            width,
            height,
            r1: None,
            r2: None,
            r3: None,
            r4: None,
            downsample_ratio: 0.25,
        })
    }

    /// Initialize hidden states to zeros
    fn init_hidden_states(&mut self) {
        let h = (self.height as f32 * self.downsample_ratio) as usize;
        let w = (self.width as f32 * self.downsample_ratio) as usize;

        tracing::debug!("Initializing hidden states to {}x{}", w, h);

        self.r1 = Some(Array4::zeros((1, 16, h, w)));
        self.r2 = Some(Array4::zeros((1, 20, h / 2, w / 2)));
        self.r3 = Some(Array4::zeros((1, 24, h / 4, w / 4)));
        self.r4 = Some(Array4::zeros((1, 28, h / 8, w / 8)));
    }
}

impl SegmentationModel for RobustVideoMatting {
    fn segment(&mut self, frame: &RgbImage) -> Result<Matte> {
        let _span = tracing::debug_span!("rvm_segment").entered();

        // Initialize hidden states on first frame
        if self.r1.is_none() {
            self.init_hidden_states();
        }

        // Preprocess frame to NCHW tensor
        let input_tensor = self.preprocessor.preprocess(frame)?;

        // Prepare inputs for ONNX Runtime
        // RVM expects: src (frame), r1, r2, r3, r4
        let r1 = self.r1.as_ref().unwrap();
        let r2 = self.r2.as_ref().unwrap();
        let r3 = self.r3.as_ref().unwrap();
        let r4 = self.r4.as_ref().unwrap();

        // Run inference
        let _infer_span = tracing::debug_span!("inference").entered();
        let outputs = self
            .session
            .run(ort::inputs![
                input_tensor.view(),
                r1.view(),
                r2.view(),
                r3.view(),
                r4.view()
            ]?)
            .context("Failed to run inference")?;
        drop(_infer_span);

        // Extract outputs: fgr (foreground), pha (alpha), r1, r2, r3, r4
        // We only need pha (the matte) and the updated hidden states

        // Alpha matte is typically the second output (index 1)
        let pha = outputs[1]
            .try_extract_tensor::<f32>()?
            .view()
            .to_owned()
            .into_dimensionality::<IxDyn>()?;

        // Update hidden states for next frame
        self.r1 = Some(
            outputs[2]
                .try_extract_tensor::<f32>()?
                .view()
                .to_owned()
                .into_dimensionality()?,
        );
        self.r2 = Some(
            outputs[3]
                .try_extract_tensor::<f32>()?
                .view()
                .to_owned()
                .into_dimensionality()?,
        );
        self.r3 = Some(
            outputs[4]
                .try_extract_tensor::<f32>()?
                .view()
                .to_owned()
                .into_dimensionality()?,
        );
        self.r4 = Some(
            outputs[5]
                .try_extract_tensor::<f32>()?
                .view()
                .to_owned()
                .into_dimensionality()?,
        );

        // Extract matte values (shape: [1, 1, H, W])
        let matte_shape = pha.shape();
        let matte_height = matte_shape[2];
        let matte_width = matte_shape[3];

        // Flatten to Vec<f32>
        let matte_flat: Vec<f32> = pha.iter().copied().collect();

        // Postprocess: resize back to original frame dimensions
        let (frame_width, frame_height) = frame.dimensions();
        let final_matte = Preprocessor::postprocess_matte(
            &matte_flat,
            matte_width as u32,
            matte_height as u32,
            frame_width,
            frame_height,
        )?;

        Ok(final_matte)
    }

    fn reset_state(&mut self) {
        tracing::info!("Resetting RVM hidden states");
        self.r1 = None;
        self.r2 = None;
        self.r3 = None;
        self.r4 = None;
    }

    fn input_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
