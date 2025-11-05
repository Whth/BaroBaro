use base64::{Engine as _, engine::general_purpose};
use image::{DynamicImage, ImageFormat};
use libblur::{
    ConvolutionMode, EdgeMode, EdgeMode2D, GaussianBlurParams, ThreadingPolicy, gaussian_blur_image,
};
// src/lib.rs
use rayon::prelude::*;
use std::path::PathBuf;

/// Configuration: background image path, blur radius, opacity
#[derive(Debug, Clone)]
pub struct BackgroundConfig {
    pub image_path: Option<String>,
    pub blur_radius: u64,
    pub opacity: f64,
}

/// Process result: returns a base64 encoded data URL (image/png;base64,...), or None
pub fn process_background(config: BackgroundConfig) -> Result<Option<String>, BlurError> {
    let path_str = match &config.image_path {
        Some(p) if !p.trim().is_empty() => p,
        _ => return Ok(None),
    };

    let image_path = PathBuf::from(path_str);

    if !image_path.exists() {
        return Ok(None);
    }

    // 1. Decode image
    let img_data = std::fs::read(&image_path).map_err(BlurError::Io)?;
    let mut img = image::load_from_memory(&img_data).map_err(BlurError::ImageDecode)?;

    // 2. Adjust opacity
    apply_opacity(&mut img, config.opacity);

    // 3. Apply Gaussian blur
    let blurred_img = apply_gaussian_blur(&img, config.blur_radius as f64)?;

    // 4. Encode to PNG + Base64 Data URL
    let data_url = encode_to_data_url(blurred_img)?;
    Ok(Some(data_url))
}

// ------------------------------
// 🎨 Adjust Opacity
// ------------------------------

fn apply_opacity(img: &mut DynamicImage, opacity: f64) {
    let alpha = (opacity.clamp(0.0, 1.0) * 255.0).round() as u8;
    let mut rgba_image = img.to_rgba8();
    rgba_image.pixels_mut().par_bridge().for_each(|pixel| {
        pixel[3] = alpha;
    });
    *img = DynamicImage::ImageRgba8(rgba_image);
}

// ------------------------------
// 🔮 Gaussian Blur
// ------------------------------

fn apply_gaussian_blur(img: &DynamicImage, radius: f64) -> Result<DynamicImage, BlurError> {
    gaussian_blur_image(
        img.clone(),
        GaussianBlurParams::new_from_kernel(radius),
        EdgeMode2D::new(EdgeMode::Clamp),
        ConvolutionMode::FixedPoint,
        ThreadingPolicy::Adaptive,
    )
    .ok_or_else(|| BlurError::Processing("Failed to apply Gaussian blur"))
}

// ------------------------------
// 💾 Encode to Data URL
// ------------------------------

fn encode_to_data_url(img: DynamicImage) -> Result<String, BlurError> {
    let mut buf = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buf);

    img.write_to(&mut cursor, ImageFormat::Png)
        .map_err(BlurError::ImageDecode)?;

    let base64_data = general_purpose::STANDARD.encode(&buf);
    Ok(format!("data:image/png;base64,{}", base64_data))
}

// ------------------------------
// 🧨 Error Types
// ------------------------------

#[derive(Debug, thiserror::Error)]
pub enum BlurError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image decode error: {0}")]
    ImageDecode(#[from] image::ImageError),

    #[error("Blur processing error: {0}")]
    Blur(#[from] libblur::BlurError),

    #[error("Processing error: {0}")]
    Processing(&'static str),
}
