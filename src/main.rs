use std::process;

use annotate_screenshot;

use clap::Parser;
use palette::Srgb;
/// Simple program to greet a person

fn main() {
    let Args {
        background_color,
        background_width,
        background_height,
        screenshot_width,
        screenshot_height,
        annotation,
        screenshot,
    } = Args::parse();

    // if let Err(e) = annotate_screenshot::run(
    //     background_color,
    //     background_width,
    //     background_height,
    //     screenshot,
    //     screenshot_width,
    //     screenshot_height,
    // ) {
    //     process::exit(1);
    // }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    background_color: String,

    #[arg(long)]
    background_width: i32,

    #[arg(long)]
    background_height: i32,

    #[arg(long)]
    screenshot: String,

    #[arg(long)]
    screenshot_width: i32,

    #[arg(long)]
    screenshot_height: i32,

    #[arg(long)]
    annotation: String,
}

// use std::io::Error;

// use image::{
//     imageops::{self, FilterType},
//     DynamicImage, ImageBuffer, Rgb, RgbImage, Rgba,
// };
// use palette::{chromatic_adaptation::AdaptInto, Srgb};
// use resvg::{
//     tiny_skia,
//     usvg::{self, fontdb::Database, PostProcessingSteps},
// };
// fn main() {
//     let width = 500;
//     let height = 500;
//     let mut bg_image_buffer = create_bg_image(width, height, "#a0433c").unwrap();
//     let screenshot_image_buffer = read_screenshot_image("icon.png", 800, 800).into_rgb8();
//     // imageops::overlay(&mut bg_image_buffer, &screenshot_image_buffer, 0, 0);
//     let _ = bg_image_buffer.save_with_format("rustshot.png", image::ImageFormat::Png);
//     let mask = create_screen_mask();
//     let masked_image = ImageBuffer::from_fn(800, 800, |x, y| {
//         let mask_pixel = mask.get_pixel(x, y);
//         if mask_pixel.0[3] != 0 {
//             let z = screenshot_image_buffer.get_pixel(x, y);
//             return Rgba([z.0[0], z.0[1], z.0[2], 255]);
//         }
//         Rgba([0, 0, 0, 0])
//     });

//     masked_image
//         .save_with_format("out.png", image::ImageFormat::Png)
//         .unwrap();
// }

//

// fn create_screen_mask() -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
//     let mut result = usvg::Tree::from_str(
//         r#"
// <svg  version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
// 	 width="800px" height="800px" viewBox="0 0 36.09 36.09" xml:space="preserve"
// 	>
// <g>
// 	<path d="M36.042,13.909c-0.123-0.377-0.456-0.646-0.85-0.688l-11.549-1.172L18.96,1.43c-0.16-0.36-0.519-0.596-0.915-0.596
// 		s-0.755,0.234-0.915,0.598L12.446,12.05L0.899,13.221c-0.394,0.04-0.728,0.312-0.85,0.688c-0.123,0.377-0.011,0.791,0.285,1.055
// 		l8.652,7.738L6.533,34.045c-0.083,0.387,0.069,0.787,0.39,1.02c0.175,0.127,0.381,0.191,0.588,0.191
// 		c0.173,0,0.347-0.045,0.503-0.137l10.032-5.84l10.03,5.84c0.342,0.197,0.77,0.178,1.091-0.059c0.32-0.229,0.474-0.633,0.391-1.02
// 		l-2.453-11.344l8.653-7.737C36.052,14.699,36.165,14.285,36.042,13.909z M25.336,21.598c-0.268,0.24-0.387,0.605-0.311,0.957
// 		l2.097,9.695l-8.574-4.99c-0.311-0.182-0.695-0.182-1.006,0l-8.576,4.99l2.097-9.695c0.076-0.352-0.043-0.717-0.311-0.957
// 		l-7.396-6.613l9.87-1.002c0.358-0.035,0.668-0.264,0.814-0.592l4.004-9.077l4.003,9.077c0.146,0.328,0.456,0.557,0.814,0.592
// 		l9.87,1.002L25.336,21.598z"/>
// </g>
// </svg>
//         "#,
//         &usvg::Options::default(),
//     ).unwrap();

//     result.postprocess(PostProcessingSteps::default(), &Database::new());

//     println!("{:?}", result.size);
//     let mut pixmap = tiny_skia::Pixmap::new(800, 800).unwrap();
//     let mut mutpixmap = pixmap.as_mut();
//     resvg::render(&result, usvg::Transform::identity(), &mut mutpixmap);
//     pixmap.save_png("fd.png").unwrap();

//     let final_result: ImageBuffer<Rgba<u8>, Vec<u8>> =
//         ImageBuffer::from_raw(800, 800, pixmap.data().to_vec()).unwrap();

//     return final_result;
// }
