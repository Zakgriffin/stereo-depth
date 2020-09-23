use image::{GrayImage, Pixel};

const SOBEL_MASK_X: [[i32; 3]; 3] = [[1, 0, -1], [2, 0, -2], [1, 0, -1]];
const SOBEL_MASK_Y: [[i32; 3]; 3] = [[1, 2, 1], [0, 0, 0], [-1, -2, -1]];

pub fn laplace_edge(image_input: &Box<GrayImage>) -> Box<GrayImage> {
    let mut image_final = Box::new(GrayImage::new(image_input.width(), image_input.height()));

    let width = image_final.width() as i32;
    let height = image_final.height() as i32;

    for x in 1..width - 1 {
        for y in 1..height - 1 {
            // TODO incomplete range (border) ^
            let mut g_x = 0;
            let mut g_y = 0;
            for sub_x in 0..3 {
                for sub_y in 0..3 {
                    let pixel_at = image_input
                        .get_pixel((x + sub_x - 1) as u32, (y + sub_y - 1) as u32)
                        .channels()[0];
                    g_x += pixel_at as i32 * SOBEL_MASK_X[sub_x as usize][sub_y as usize];
                    g_y += pixel_at as i32 * SOBEL_MASK_Y[sub_x as usize][sub_y as usize];
                }
            }
            let hypot = ((g_x.pow(2) + g_y.pow(2)) as f64).sqrt() as u8;
            image_final.put_pixel(x as u32, y as u32, Pixel::from_channels(hypot, 0, 0, 0));
        }
    }
    image_final
}
