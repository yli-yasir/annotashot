use std::error::Error;

use image::{Rgba, RgbaImage};
use palette::Srgb;

pub struct BackgroundConfig<'a> {
    pub width: u32,
    pub height: u32,
    pub color: &'a str,
}

pub fn create_bg_image(
    BackgroundConfig {
        width,
        height,
        color,
    }: &BackgroundConfig,
) -> Result<RgbaImage, Box<dyn Error>> {
    let color: Srgb<u8> = color.parse()?;

    let bg_image = RgbaImage::from_pixel(
        *width,
        *height,
        Rgba([color.red, color.green, color.blue, 255]),
    );

    Ok(bg_image)
}
