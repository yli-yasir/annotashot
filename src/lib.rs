use annotate::{create_annotation, AnnotationConfig};
use background::{create_bg_image, BackgroundConfig};
use image::{imageops, ImageFormat};
use screenshot::{read_screenshot_image, ScreenshotConfig};
use std::error::Error;

pub mod annotate;
pub mod background;
pub mod screenshot;
pub mod svg;

pub fn run(
    background_config: BackgroundConfig,
    screenshot_config: ScreenshotConfig,
    annotation_config: AnnotationConfig,
) -> Result<(), Box<dyn Error>> {
    let mut background = create_bg_image(&background_config)?;
    let screenshot = read_screenshot_image(&screenshot_config)?;
    let annotation = create_annotation(&annotation_config)?;

    imageops::overlay(
        &mut background,
        &screenshot,
        shift_origin_to_center(
            background_config.width as f64 * screenshot_config.x,
            screenshot_config.resize_width,
        ),
        shift_origin_to_center(
            background_config.height as f64 * screenshot_config.y,
            screenshot_config.resize_height,
        ),
    );

    imageops::overlay(&mut background, &annotation, 0, 0);

    background.save_with_format("background.png", ImageFormat::Png);
    return Ok(());
}

fn shift_origin_to_center(coord: f64, side_length: u32) -> i64 {
    (coord - side_length as f64 / 2.0).round() as i64
}
