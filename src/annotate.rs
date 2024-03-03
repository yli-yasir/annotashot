use crate::svg::render_svg;
use image::RgbaImage;
use std::error::Error;

pub struct AnnotationConfig {
    pub text: String,
    pub font: String,
    pub width: u32,
    pub height: u32,
    pub x: f64,
    pub y: f64,
    pub font_size: u32,
    pub font_color: String,
}

impl TryFrom<AnnotationConfig> for RgbaImage {
    type Error = Box<dyn Error>;

    fn try_from(
        AnnotationConfig {
            width,
            height,
            x,
            y,
            font,
            font_size,
            text,
            font_color,
        }: AnnotationConfig,
    ) -> Result<RgbaImage, Self::Error> {
        let text_lines = text
            .lines()
            .enumerate()
            .fold(String::new(), |acc, (i, line)| {
                format!(
                    r##"
{acc}
<text x="{x_percent:.2}%"
y="{y_percent:.2}%"
font-family="{font}"
font-size="{font_size}"
fill="#{font_color}"
text-anchor="middle">
{line}
</text>
 "##,
                    x_percent = x * 100.0,
                    y_percent = line_y_offset_percent(height, y, i, font_size)
                )
            });

        let annotation_image = render_svg(text_lines, width, height)?;

        Ok(annotation_image)
    }
}

fn line_y_offset_percent(
    total_height: u32,
    first_line_y_percentage: f64,
    line_index: usize,
    font_size: u32,
) -> f64 {
    let first_line_y = total_height as f64 * first_line_y_percentage;
    let line_y_offset = font_size as f64 * 1.2 * line_index as f64;
    ((first_line_y + line_y_offset) / total_height as f64) * 100.0
}
