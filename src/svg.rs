use std::error::Error;

use image::RgbaImage;
use once_cell::sync::Lazy;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{self, fontdb, PostProcessingSteps, XmlOptions},
};

static FONT_DB: Lazy<usvg::fontdb::Database> = Lazy::new(|| {
    let mut font_db = fontdb::Database::new();
    if let Err(e) = font_db.load_font_file("./NotoSans-Regular.ttf") {
        panic!("Failed to load fonts: {e}");
    }
    font_db
});

pub fn render_svg(svg_string: &str, width: u32, height: u32) -> Result<RgbaImage, Box<dyn Error>> {
    let mut svg_tree = usvg::Tree::from_str(svg_string, &usvg::Options::default())?;

    svg_tree.postprocess(
        PostProcessingSteps {
            convert_text_into_paths: true,
        },
        &FONT_DB,
    );

    let mut pixmap = Pixmap::new(width, height).expect("Failed to create pixmap!");
    resvg::render(&svg_tree, usvg::Transform::identity(), &mut pixmap.as_mut());
    pixmap.save_png("apixmap.png").unwrap();

    let raster_image = RgbaImage::from_raw(width, height, pixmap.data().to_vec())
        .expect("Failed to convert pixmap to raster!");

    Ok(raster_image)
}
