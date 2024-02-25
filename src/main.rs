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
        screenshot_resize_width,
        screenshot_resize_height,
        screenshot,
        screenshot_x,
        screenshot_y,
        screenshot_crop_top,
        screenshot_crop_left,
        screenshot_crop_right,
        screenshot_crop_bottom,
        annotation,
        annotation_font,
        annotation_font_size,
        annotation_x,
        annotation_y,
        annotation_font_color,
    } = Args::parse();

    if let Err(e) = annotate_screenshot::run(
        BackgroundConfig {
            width: background_width,
            height: background_height,
            color: &background_color,
        },
        ScreenshotConfig {
            resize_width: screenshot_resize_width,
            resize_height: screenshot_resize_height,
            crop_top: screenshot_crop_top,
            crop_bottom: screenshot_crop_bottom,
            crop_right: screenshot_crop_right,
            crop_left: screenshot_crop_left,
            x: screenshot_x,
            y: screenshot_y,
            file_path: &screenshot,
        },
        AnnotationConfig {
            width: background_width,
            height: background_height,
            x: annotation_x,
            y: annotation_y,
            text: &annotation.replace("\\n", "\n"),
            font_size: annotation_font_size,
            font: &annotation_font,
            font_color: &annotation_font_color,
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
    screenshot_resize_width: u32,

    #[arg(long)]
    screenshot_resize_height: u32,

    #[arg(long)]
    screenshot_x: f64,

    #[arg(long)]
    screenshot_crop_top: f64,

    #[arg(long)]
    screenshot_crop_right: f64,

    #[arg(long)]
    screenshot_crop_left: f64,

    #[arg(long)]
    screenshot_crop_bottom: f64,

    #[arg(long)]
    screenshot_y: f64,

    #[arg(long)]
    annotation: String,

    #[arg(long)]
    annotation_font: String,

    #[arg(long)]
    annotation_font_size: u32,

    #[arg(long)]
    annotation_font_color: String,

    #[arg(long)]
    annotation_x: f64,

    #[arg(long)]
    annotation_y: f64,
}
