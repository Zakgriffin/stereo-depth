use crate::image::Pixel;
use image::GrayImage;
use std::cmp::min;
use std::f64::consts::{E, PI};

pub fn gaussian_smooth(image_input: &Box<GrayImage>) -> Box<GrayImage> {
    let mut image_final = Box::new(GrayImage::new(image_input.width(), image_input.height()));

    let width = image_final.width() as i32;
    let height = image_final.height() as i32;

    let kernel_size_out: i32 = 3;
    let sigma = 3.0;
    let gauss_mask = gauss_kernel(kernel_size_out, sigma);
    for x in 0..width {
        for y in 0..height {
            let sub_x_start = -min(kernel_size_out, x);
            let sub_x_end = min(kernel_size_out, width - x - 1);
            let sub_y_start = -min(kernel_size_out, y);
            let sub_y_end = min(kernel_size_out, height - y - 1);

            let mut conv_sum = 0.0;
            let mut weight_sum = 0.0;
            for sub_x in sub_x_start..=sub_x_end {
                for sub_y in sub_y_start..=sub_y_end {
                    let pixel_at = image_input
                        .get_pixel((x + sub_x) as u32, (y + sub_y) as u32)
                        .channels()[0];
                    let kernel_weight = gauss_mask[sub_x.abs() as usize][sub_y.abs() as usize];
                    conv_sum += pixel_at as f64 * kernel_weight;
                    weight_sum += kernel_weight;
                }
            }
            // conv_sum /= ((sub_x_end - sub_x_start) * (sub_y_end - sub_y_start)) as f64;
            image_final.put_pixel(
                x as u32,
                y as u32,
                Pixel::from_channels((conv_sum / weight_sum) as u8, 0, 0, 0), // TODO this doesn't seem right
            );
        }
    }
    image_final
}

fn gauss_kernel(kernel_size_out: i32, sigma: f64) -> Vec<Vec<f64>> {
    fn gauss_exp(x: i32, y: i32, sigma: f64) -> f64 {
        (1.0 / (2.0 * PI * (sigma * sigma)))
            * E.powf(-(x * x + y * y) as f64 / (2.0 * sigma * sigma))
    }
    // fn calc_exp(x: i32, y: i32, sigma: f64) -> f64 {
    //     E.powf(-(x * x + y * y) as f64 / sigma * sigma)
    // }

    let kernel_vec_size = (kernel_size_out + 1) as usize;
    let mut kernel = vec![vec![0f64; kernel_vec_size]; kernel_vec_size];
    for x in 0..=kernel_size_out {
        for y in 0..=kernel_size_out {
            kernel[x as usize][y as usize] = gauss_exp(x, y, sigma);
        }
    }

    kernel
}
