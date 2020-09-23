use image::{GrayImage, Pixel};

// adds % color from original image and highlights edges for better disparity measurement
pub fn transparency(
    input_image_base: &Box<GrayImage>,
    input_image_to_blend: &Box<GrayImage>,
    interp: f64,
) -> Box<GrayImage> {
    if interp > 1.0 {
        panic!("no bad");
    }

    let mut final_image = Box::new(GrayImage::new(
        input_image_base.width(),
        input_image_base.height(),
    ));
    for x in 0..final_image.width() {
        for y in 0..final_image.height() {
            let pixel = input_image_base.get_pixel(x, y).channels()[0] as f64 * interp
                + input_image_to_blend.get_pixel(x, y).channels()[0] as f64 * (1.0 - interp);
            final_image.put_pixel(x, y, Pixel::from_channels(pixel as u8, 0, 0, 0))
        }
    }

    final_image
}
