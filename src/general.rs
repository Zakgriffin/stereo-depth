use image::Luma;
use image::{GrayImage, Pixel, RgbImage};
use show_image::PixelFormat::{Mono8, Rgb8};
use show_image::{ImageData, ImageInfo};

pub fn rgb2gray(image: &Box<RgbImage>) -> Box<GrayImage> {
    let mut gray_image = Box::new(GrayImage::new(image.width(), image.height()));
    for i in 0..image.width() {
        for j in 0..image.height() {
            let pixel = image.get_pixel(i, j);
            let (r, g, b, _) = pixel.channels4();
            let pixel_gray =
                (r as f64 * 0.299) as u8 + (g as f64 * 0.587) as u8 + (b as f64 * 0.114) as u8;
            gray_image.put_pixel(i, j, Luma::from_channels(pixel_gray, 0, 0, 0));
        }
    }
    // converting each pixel r-g-b to grayscale using weighted average.
    gray_image
}

#[derive(Debug)]
pub struct Block {
    pub width: u32,
    pub height: u32,
}

// trait WrappedPixel<Pixel> {
//     fn get_wrapped_pixel(&self, x: u32, y: u32) -> Pixel;
// }

// impl<I> WrappedPixel<I::Pixel> for SubImage<I> {
//     fn get_wrapped_pixel(&self, x: u32, y: u32) -> Self::Pixel {
//         self.get_pixel(x % self.width(), y % self.height())
//     }
// }

pub struct ShowImageWrapperGray<'a> {
    pub image: &'a Box<GrayImage>,
}

impl ImageData for ShowImageWrapperGray<'_> {
    fn data(self) -> Box<[u8]> {
        self.image.to_vec().into_boxed_slice()
    }

    fn info(&self) -> Result<ImageInfo, String> {
        Ok(ImageInfo {
            width: self.image.width() as usize,
            height: self.image.height() as usize,
            row_stride: self.image.width() as usize * 1,
            pixel_format: Mono8,
        })
    }
}

pub struct ShowImageWrapperRgb {
    pub image: Box<RgbImage>,
}

impl ImageData for ShowImageWrapperRgb {
    fn data(self) -> Box<[u8]> {
        self.image.to_vec().into_boxed_slice()
    }

    fn info(&self) -> Result<ImageInfo, String> {
        Ok(ImageInfo {
            width: self.image.width() as usize,
            height: self.image.height() as usize,
            row_stride: self.image.width() as usize * 3,
            pixel_format: Rgb8,
        })
    }
}
