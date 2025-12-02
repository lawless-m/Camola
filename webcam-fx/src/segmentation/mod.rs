mod preprocess;
mod rvm;
pub mod types;

pub use preprocess::Preprocessor;
pub use rvm::RobustVideoMatting;
pub use types::{Matte, SegmentationModel};

use anyhow::Result;
use image::RgbImage;

/// Create a default segmentation model (RVM)
pub fn create_default_model(model_path: &str) -> Result<Box<dyn SegmentationModel>> {
    let model = RobustVideoMatting::new(model_path)?;
    Ok(Box::new(model))
}
