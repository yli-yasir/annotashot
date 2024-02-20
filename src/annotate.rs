use std::error::Error;

use image::{RgbImage, RgbaImage};

use crate::svg::render_svg;

pub fn create_annotation(text: &str, width: u32, height: u32) -> Result<RgbaImage, Box<dyn Error>> {
    render_svg(
        &format!(
            r#"
    <svg
    xmlns="http://www.w3.org/2000/svg"
    width="{width}px"
    height="{height}px"
    viewbox="0 0 {width} {height}" 
    >
    <text x="50%" y="50%" font-family="Noto Sans" font-size="32" text-anchor="middle">{text}</text>
    </svg>
            "#
        ),
        width,
        height,
    )
}
