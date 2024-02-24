use annotate_screenshot::{
    annotate::AnnotationConfig, background::BackgroundConfig, screenshot::ScreenshotConfig,
};
use clap::Parser;
use std::process;

fn main() {
    let Args {
        background_color,
        background_width,
        background_height,
        screenshot_width,
        screenshot_height,
        screenshot,
        annotation,
        annotation_font,
        annotation_font_size,
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
        AnnotationConfig {
            width: background_width,
            height: background_height,
            text: &annotation.replace("\\n", "\n"),
            font_size: annotation_font_size,
            font: &annotation_font,
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

    #[arg(long)]
    annotation_font: String,

    #[arg(long)]
    annotation_font_size: u32,
}
