use annotate::create_annotation;
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
) -> Result<(), Box<dyn Error>> {
    let mut background = create_bg_image(&background_config)?;
    let screenshot = read_screenshot_image(screenshot_config)?;
    let annotation = create_annotation(
        "hello world",
        background_config.width,
        (background_config.height / 4).into(),
    )?;

    imageops::overlay(
        &mut background,
        &screenshot,
        (background_config.width / 2).into(),
        (background_config.height / 2).into(),
    );

    imageops::overlay(
        &mut background,
        &annotation,
        (background_config.width / 2).into(),
        0,
    );

    background.save_with_format("background.png", ImageFormat::Png);
    return Ok(());
}
