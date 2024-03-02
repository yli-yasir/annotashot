use crate::svg::render_svg;
use image::{imageops::FilterType, ImageBuffer, Rgba, RgbaImage};
use std::error::Error;

pub struct ScreenshotConfig {
    pub resize_width: u32,
    pub resize_height: u32,
    pub crop_top: f64,
    pub crop_bottom: f64,
    pub crop_right: f64,
    pub crop_left: f64,
    pub x: f64,
    pub y: f64,
    pub file_path: String,
}

pub struct Screenshot {
    x: f64,
    y: f64,
    image: RgbaImage,
}

impl Screenshot {
    pub fn new(
        ScreenshotConfig {
            resize_width,
            resize_height,
            file_path,
            crop_left,
            crop_right,
            crop_top,
            crop_bottom,
            x,
            y,
        }: ScreenshotConfig,
    ) -> Result<Self, Box<dyn Error>> {
        let resize_width_f64 = resize_width as f64;
        let resize_height_f64 = resize_height as f64;

        let image = image::open(file_path)?
            .resize_exact(resize_width, resize_height, FilterType::Gaussian)
            .crop_imm(
                (resize_width_f64 * crop_left).round() as u32,
                (resize_height_f64 * crop_top).round() as u32,
                (resize_width_f64 - (resize_width_f64 * (crop_right + crop_left))).round() as u32,
                (resize_height_f64 - (resize_height_f64 * (crop_top + crop_bottom))).round() as u32,
            )
            .to_rgba8();

        let mask = render_svg(
            String::from(
                r#"
            <rect width="100%" height="100%" rx="7%" />
            "#,
            ),
            image.width(),
            image.height(),
        )?;

        let masked_image = ImageBuffer::from_fn(image.width(), image.height(), |x, y| {
            let mask_pixel = mask.get_pixel(x, y);
            if mask_pixel.0[3] != 0 {
                let z = image.get_pixel(x, y);
                return Rgba([z.0[0], z.0[1], z.0[2], mask_pixel.0[3]]);
            }
            Rgba([0, 0, 0, 0])
        });

        Ok(Self {
            x,
            y,
            image: masked_image,
        })
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn image(&self) -> &RgbaImage {
        &self.image
    }

    pub fn x_coord(&self, side_length: u32) -> i64 {
        Self::coord_origin_center(self.x, side_length)
    }

    pub fn y_coord(&self, side_length: u32) -> i64 {
        Self::coord_origin_center(self.y, side_length)
    }

    fn coord_origin_center(coord_ratio: f64, side_length: u32) -> i64 {
        let side_length = side_length as f64;
        (side_length * coord_ratio - side_length / 2.0).round() as i64
    }
}
