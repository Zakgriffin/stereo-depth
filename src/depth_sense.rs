use crate::general::Block;
use image::GenericImageView;
use image::{GrayImage, Pixel, SubImage};
use std::cmp::min;

// calculating sub-pixel accuracy based on Lagrange interpolation
// fn subpixel_accuracy(self, row: u32, col: u32, col_l: u32, block_size: Block, window_right: SubImage<&RgbImage>, disparity_value: u32, SAD: u32) -> u32 {
//     if col == 0 || col == self.image_l.height() - block_size.height - 1 {
//         disparity_value
//     } else {
//         let C2 = SAD;
//         let window = self.generate_window(row, col_l - 1, &self.image_l, block_size);
//         let C1 = abs(window_right - window).sum();
//         window = self.generate_window(row, col_l + 1, &self.image_l, block_size);
//         let C3 = abs(window_right - window).sum();
//         if C3 + C1 - 2 * C2 == 0 {
//             return disparity_value
//         }
//         let d_est = disparity_value - (C3 - C1) / (C3 + C1 - 2 * C2);

//         d_est
//     }
// }

pub fn depth_sense(
    image_l: &Box<GrayImage>,
    image_r: &Box<GrayImage>,
    block_size: Block,
    search_range: u32,
) -> Box<GrayImage> {
    let (width, height) = image_l.dimensions();
    let mut disparity_map = Box::new(GrayImage::new(width, height));

    for x in (0..=width - block_size.width).step_by(block_size.width as usize) {
        println!("Working... {}%", x as f64 / width as f64 * 100.0);
        for y in (0..=height - block_size.height).step_by(block_size.height as usize) {
            let window_left = image_l.view(x, y, block_size.width, block_size.height);
            let mut best_dif = i32::MAX;
            let mut disparity_val = 0;
            for x_scan in (x..min(x + search_range, width - block_size.width)).step_by(3) {
                let window_right = image_r.view(x_scan, y, block_size.width, block_size.height);
                let dif = image_dif(&window_left, &window_right, &block_size);
                if dif.abs() < best_dif.abs() {
                    best_dif = dif;
                    disparity_val = x_scan as i32 - x as i32;
                    if dif == 0 {
                        break;
                    }
                    // disparity_val = subpixel_accuracy(row , col , col_l , block_size , window_right, disparity_val , SAD);
                }
            }

            for x_draw in x..x + block_size.width {
                for y_draw in y..y + block_size.height {
                    disparity_map.put_pixel(
                        x_draw,
                        y_draw,
                        Pixel::from_channels(disparity_val as u8, 0, 0, 0),
                    )
                }
            }
        }
    }
    disparity_map
}

fn image_dif(a: &SubImage<&GrayImage>, b: &SubImage<&GrayImage>, block_size: &Block) -> i32 {
    let mut sum = 0;
    for x in 0..block_size.width {
        for y in 0..block_size.height {
            let pixel_a = a.get_pixel(x, y).channels()[0] as i32;
            let pixel_b = b.get_pixel(x, y).channels()[0] as i32;
            sum += (pixel_a - pixel_b).abs();
        }
    }
    sum
}
