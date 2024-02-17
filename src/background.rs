use std::error::Error;

use image::{Rgb, RgbImage};
use palette::Srgb;

pub struct BackgroundConfig<'a> {
    width: u32,
    height: u32,
    color: &'a str,
}

pub fn create_bg_image(
    BackgroundConfig {
        width,
        height,
        color,
    }: BackgroundConfig,
) -> Result<RgbImage, Box<dyn Error>> {
    let color: Srgb<u8> = color.parse()?;

    let bg_image = RgbImage::from_pixel(width, height, Rgb([color.red, color.green, color.blue]));

    Ok(bg_image)
}
