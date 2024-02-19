use std::process;

use annotate_screenshot::{background::BackgroundConfig, screenshot::ScreenshotConfig};
use clap::Parser;

fn main() {
    let Args {
        background_color,
        background_width,
        background_height,
        screenshot_width,
        screenshot_height,
        annotation,
        screenshot,
    } = Args::parse();

    if let Err(e) = annotate_screenshot::run(
        BackgroundConfig {
            width: background_width,
            height: background_height,
            color: &background_color,
        },
        ScreenshotConfig {
            width: screenshot_width,
            height: screenshot_height,
            file_path: &screenshot,
        },
    ) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    background_color: String,

    #[arg(long)]
    background_width: u32,

    #[arg(long)]
    background_height: u32,

    #[arg(long)]
    screenshot: String,

    #[arg(long)]
    screenshot_width: u32,

    #[arg(long)]
    screenshot_height: u32,

    #[arg(long)]
    annotation: String,
}
