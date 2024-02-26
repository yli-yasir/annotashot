use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use image::{Rgba, RgbaImage};
use once_cell::sync::Lazy;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{self, fontdb, PostProcessingSteps},
};

static FONT_DB: Lazy<usvg::fontdb::Database> = Lazy::new(|| {
    let mut font_db = fontdb::Database::new();
    font_db.load_fonts_dir("./fonts");
    font_db
});

pub fn render_svg(
    inner_svg_elements: &str,
    width: u32,
    height: u32,
) -> Result<RgbaImage, Box<dyn Error>> {
    println!("{inner_svg_elements}");
    let mut svg_tree = usvg::Tree::from_str(
        &format!(
            r#"
<svg
xmlns="http://www.w3.org/2000/svg"
width="{width}px"
height="{height}px"
viewbox="0 0 {width} {height}" 
>
{inner_svg_elements}
</svg>
        "#
        ),
        &usvg::Options::default(),
    )?;

    svg_tree.postprocess(
        PostProcessingSteps {
            convert_text_into_paths: true,
        },
        &FONT_DB,
    );

    let mut pixmap = Pixmap::new(width, height).expect("Failed to create pixmap!");
    resvg::render(&svg_tree, usvg::Transform::identity(), &mut pixmap.as_mut());

    println!("{:?}", pixmap.pixel(395, 241).unwrap().demultiply());
    let raster_image = RgbaImage::from_fn(width, height, |x, y| {
        let p = pixmap.pixel(x, y).unwrap().demultiply();
        Rgba([p.red(), p.green(), p.blue(), p.alpha()])
    });

    raster_image.save_with_format("afterpixmap.png", image::ImageFormat::Png);

    Ok(raster_image)
}
