use std::error::Error;

use image::RgbaImage;
use resvg::{
    tiny_skia::Pixmap,
    usvg::{self, fontdb, PostProcessingSteps},
};

pub fn render_svg(svg_string: &str, width: u32, height: u32) -> Result<RgbaImage, Box<dyn Error>> {
    let mut svg_tree = usvg::Tree::from_str(svg_string, &usvg::Options::default())?;
    svg_tree.postprocess(PostProcessingSteps::default(), &fontdb::Database::new());

    let mut pixmap = Pixmap::new(width, height).expect("Failed to create pixmap!");
    resvg::render(&svg_tree, usvg::Transform::identity(), &mut pixmap.as_mut());

    let raster_image = RgbaImage::from_raw(width, height, pixmap.data().to_vec())
        .expect("Failed to convert pixmap to raster!");

    Ok(raster_image)
}
