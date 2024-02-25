use std::error::Error;

use image::{imageops::FilterType, ImageBuffer, Rgba, RgbaImage};

use crate::svg::render_svg;

pub struct ScreenshotConfig<'a> {
    pub resize_width: u32,
    pub resize_height: u32,
    pub crop_top: f64,
    pub crop_bottom: f64,
    pub crop_right: f64,
    pub crop_left: f64,
    pub x: f64,
    pub y: f64,
    pub file_path: &'a str,
}

pub fn read_screenshot_image(
    ScreenshotConfig {
        resize_width,
        resize_height,
        file_path,
        crop_left,
        crop_right,
        crop_top,
        crop_bottom,
        ..
    }: &ScreenshotConfig,
) -> Result<RgbaImage, Box<dyn Error>> {
    let image = image::open(file_path)?
        .resize_exact(*resize_width, *resize_height, FilterType::Gaussian)
        .crop_imm(
            (*resize_width as f64 * crop_left).round() as u32,
            (*resize_height as f64 * crop_top).round() as u32,
            (*resize_width as f64 - (*resize_width as f64 * (crop_right + crop_left))).round()
                as u32,
            (*resize_height as f64 - (*resize_height as f64 * (crop_top + crop_bottom))).round()
                as u32,
        )
        .to_rgba8();

    let mask = render_svg(
        r#"
    <rect width="100%" height="100%" rx="5%" />
    "#,
        image.width(),
        image.height(),
    )?;

    mask.save_with_format("mask.png", image::ImageFormat::Png);
    let masked_image = ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
        let mask_pixel = mask.get_pixel(x, y);
        if mask_pixel.0[3] != 0 {
            let z = image.get_pixel(x, y);
            return Rgba([z.0[0], z.0[1], z.0[2], mask_pixel.0[3]]);
        }
        Rgba([0, 0, 0, 0])
    });

    masked_image.save_with_format("masked.png", image::ImageFormat::Png);

    Ok(masked_image)
}
