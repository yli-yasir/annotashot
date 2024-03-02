use ::image::{imageops, ImageFormat};
use annotate::AnnotationConfig;
use background::BackgroundConfig;
use image::RgbaImage;
use screenshot::{Screenshot, ScreenshotConfig};
use std::error::Error;

pub mod annotate;
pub mod background;
pub mod screenshot;
pub mod svg;

pub fn run(
    out_file_name: String,
    background_config: BackgroundConfig,
    screenshot_config: ScreenshotConfig,
    annotation_config: AnnotationConfig,
) -> Result<(), Box<dyn Error>> {
    let mut canvas = RgbaImage::try_from(background_config)?;
    let annotation = RgbaImage::try_from(annotation_config)?;
    let screenshot = Screenshot::new(screenshot_config)?;

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    imageops::overlay(
        &mut canvas,
        screenshot.image(),
        screenshot.x_coord(canvas_width),
        screenshot.y_coord(canvas_height),
    );

    imageops::overlay(&mut canvas, &annotation, 0, 0);

    canvas.save_with_format(out_file_name, ImageFormat::Png)?;

    return Ok(());
}
