use crate::svg::render_svg;
use image::RgbaImage;
use std::error::Error;

pub struct AnnotationConfig<'a> {
    pub text: &'a str,
    pub font: &'a str,
    pub width: u32,
    pub height: u32,
    pub font_size: u32,
}

pub fn create_annotation(
    annotation_config: &AnnotationConfig,
) -> Result<RgbaImage, Box<dyn Error>> {
    let AnnotationConfig {
        width,
        height,
        font,
        font_size,
        text,
    } = *annotation_config;

    println!("{}", text.lines().count());

    let text_lines = text.lines().enumerate().fold(String::new(),|acc,(i,line)| {
        format!(
        r#"{acc} <text x="50%" y="{}%" font-family="{font}" font-size="{font_size}" text-anchor="middle">{line}</text>"#,
        line_y_percentage(height,0.5, i ,font_size)
        )
    });

    render_svg(
        &format!(
            r#"
    <svg
    xmlns="http://www.w3.org/2000/svg"
    width="{width}px"
    height="{height}px"
    viewbox="0 0 {width} {height}" 
    >
    {text_lines}
    </svg>
            "#
        ),
        width,
        height,
    )
}

fn line_y_percentage(
    total_height: u32,
    first_line_y_percentage: f32,
    line_index: usize,
    font_size: u32,
) -> f32 {
    let first_line_y = total_height as f32 * first_line_y_percentage;
    let line_y_offset = font_size as f32 * 1.2 * line_index as f32;
    ((first_line_y + line_y_offset) / total_height as f32) * 100.0
}
