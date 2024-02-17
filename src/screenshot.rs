use std::error::Error;

use image::{imageops::FilterType, DynamicImage, RgbImage};

pub struct ScreenshotConfig<'a> {
    width: u32,
    height: u32,
    file_path: &'a str,
}

pub fn read_screenshot_image(
    ScreenshotConfig {
        width,
        height,
        file_path,
    }: ScreenshotConfig,
) -> Result<RgbImage, Box<dyn Error>> {
    Ok(image::open(file_path)?
        .resize(width, height, FilterType::Gaussian)
        .to_rgb8())
}
