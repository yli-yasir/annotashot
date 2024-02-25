use crate::svg::render_svg;
use image::RgbaImage;
use std::error::Error;

pub struct AnnotationConfig<'a> {
    pub text: &'a str,
    pub font: &'a str,
    pub width: u32,
    pub height: u32,
    pub x: f64,
    pub y: f64,
    pub font_size: u32,
    pub font_color: &'a str,
}

pub fn create_annotation(
    annotation_config: &AnnotationConfig,
) -> Result<RgbaImage, Box<dyn Error>> {
    let AnnotationConfig {
        width,
        height,
        x,
        y,
        font,
        font_size,
        text,
        font_color,
    } = *annotation_config;

    println!("{}", text.lines().count());

    let text_lines = text.lines().enumerate().fold(String::new(),|acc,(i,line)| {
        format!(
        r##"{acc} <text x="{}%" y="{}%" font-family="{font}" font-size="{font_size}" fill="#{font_color}" text-anchor="middle">{line}</text>"##,
        x * 100.0,
        line_y_percentage(height, y, i, font_size)
        )
    });

    let annotation = render_svg(&text_lines, width, height)?;

    annotation.save_with_format("anno.png", image::ImageFormat::Png);
    Ok(annotation)
}

fn line_y_percentage(
    total_height: u32,
    first_line_y_percentage: f64,
    line_index: usize,
    font_size: u32,
) -> f64 {
    let first_line_y = total_height as f64 * first_line_y_percentage;
    let line_y_offset = font_size as f64 * 1.2 * line_index as f64;
    ((first_line_y + line_y_offset) / total_height as f64) * 100.0
}
