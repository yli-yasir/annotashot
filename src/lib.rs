use background::{create_bg_image, BackgroundConfig};
use screenshot::{read_screenshot_image, ScreenshotConfig};
use std::error::Error;

pub mod background;
pub mod screenshot;
pub mod svg;

pub fn run(
    background_config: BackgroundConfig,
    screenshot_config: ScreenshotConfig,
) -> Result<(), Box<dyn Error>> {
    let background = create_bg_image(background_config)?;
    let screenshot = read_screenshot_image(screenshot_config)?;
    // let annotation = create_annotation(annotation_config);

    return Ok(());
}
