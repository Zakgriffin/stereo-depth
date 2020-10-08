extern crate image;
extern crate rayon;
extern crate show_image;

mod benchmark;
mod depth_sense;
// mod edge_detect;
// mod gaussian_smooth;
mod general;
// mod image_prepare;
// mod transparency;

use benchmark::benchmark;
// use core::time::Duration;
// use general::Block;
// use std::time::Instant;
// use transparency::transparency;

// use depth_sense::depth_sense;
// use edge_detect::laplace_edge;
// use gaussian_smooth::gaussian_smooth;
// use general::{rgb2gray, ShowImageWrapperGray};
// use image::{open, GrayImage, RgbImage};
// use show_image::{make_window, KeyCode};

// const ROOT_PATH: &str = r"C:\Users\zakgr\Desktop\stereo-depth\src\images\";

fn main() {
    benchmark();
    // depth();
}
// #[allow(dead_code)]
// fn depth() {
//     let time = Instant::now();
//     println!("Loading images...: {:?}", time.elapsed());
//     let image_l = unwrap_image_gray(ROOT_PATH.to_owned() + "left_ready.png");
//     let image_r = unwrap_image_gray(ROOT_PATH.to_owned() + "right_ready.png");
//     println!("Images Loaded: {:?}", time.elapsed());
//     // (2964, 2000)
//     let depth_image = depth_sense(
//         &image_l,
//         &image_r,
//         Block {
//             width: 10,  //26, 247
//             height: 10, // 25, 200
//         },
//         250, // 200
//     );
//     println!("Done: {:?}", time.elapsed());

//     let window = make_window("image").unwrap();

//     window
//         .set_image(
//             ShowImageWrapperGray {
//                 image: &depth_image,
//             },
//             "image",
//         )
//         .unwrap();

//     depth_image
//         .save(ROOT_PATH.to_owned() + "final_depth.png")
//         .unwrap();
// }

// #[allow(dead_code)]
// fn prep_image() {
//     let time = Instant::now();

//     println!("Loading images...: {:?}", time.elapsed());
//     let image = unwrap_image(ROOT_PATH.to_owned() + "im1.png");
//     // let image_r = unwrap_image(ROOT_PATH.to_owned() + "im1.png");
//     println!("Images Loaded: {:?}", time.elapsed());

//     let gray = rgb2gray(&image);
//     println!("Converted Gray: {:?}", time.elapsed());

//     let smooth_image = gaussian_smooth(&gray);
//     println!("Gaussian Smoothed: {:?}", time.elapsed());

//     let edge_image = laplace_edge(&gray);
//     println!("Converted Edges: {:?}", time.elapsed());

//     let blend_image = transparency(&smooth_image, &edge_image, 0.2);
//     println!("Blended: {:?}", time.elapsed());

//     let window = make_window("image").unwrap();

//     window
//         .set_image(
//             ShowImageWrapperGray {
//                 image: &blend_image,
//             },
//             "image",
//         )
//         .unwrap();

//     while let Ok(event) = window.wait_key(Duration::from_millis(100)) {
//         if let Some(event) = event {
//             if event.key == KeyCode::Escape {
//                 break;
//             }
//         }
//     }

//     blend_image
//         .save(ROOT_PATH.to_owned() + "left_ready.png")
//         .unwrap();

//     show_image::stop().unwrap();
// }

// fn unwrap_image(path: String) -> Box<RgbImage> {
//     Box::new(open(path).unwrap().as_mut_rgb8().unwrap().to_owned())
// }
// fn unwrap_image_gray(path: String) -> Box<GrayImage> {
//     Box::new(open(path).unwrap().as_mut_luma8().unwrap().to_owned())
// }
