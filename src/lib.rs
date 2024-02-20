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
    let screenshot = read_screenshot_image(&screenshot_config)?;
    let annotation = create_annotation(
        "hello world",
        background_config.width,
        (background_config.height / 4).into(),
    )?;

    imageops::overlay(
        &mut background,
        &screenshot,
        center_coord(background_config.width, screenshot.width()).into(),
        (background_config.height / 2).into(),
    );

    imageops::overlay(&mut background, &annotation, 0, 0);

    annotation.save_with_format("anno.png", ImageFormat::Png);
    background.save_with_format("background.png", ImageFormat::Png);
    return Ok(());
}

fn center_coord(parent_length: u32, child_length: u32) -> u32 {
    (parent_length - child_length) / 2
}
