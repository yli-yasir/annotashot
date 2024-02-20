use std::error::Error;

use image::{imageops::FilterType, RgbaImage};

pub struct ScreenshotConfig<'a> {
    pub width: u32,
    pub height: u32,
    pub file_path: &'a str,
}

pub fn read_screenshot_image(
    ScreenshotConfig {
        width,
        height,
        file_path,
    }: &ScreenshotConfig,
) -> Result<RgbaImage, Box<dyn Error>> {
    Ok(image::open(file_path)?
        .resize_exact(*width, *height, FilterType::Gaussian)
        .to_rgba8())
}
