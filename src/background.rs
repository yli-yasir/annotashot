use image::{Rgba, RgbaImage};
use palette::Srgb;
use std::error::Error;

pub struct BackgroundConfig {
    pub width: u32,
    pub height: u32,
    pub color: String,
}

impl TryFrom<BackgroundConfig> for RgbaImage {
    type Error = Box<dyn Error>;

    fn try_from(
        BackgroundConfig {
            width,
            height,
            color,
        }: BackgroundConfig,
    ) -> Result<Self, Self::Error> {
        let color: Srgb<u8> = color.parse()?;

        let image = RgbaImage::from_pixel(
            width,
            height,
            Rgba([color.red, color.green, color.blue, 255]),
        );

        Ok(image)
    }
}
