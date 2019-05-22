extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, GenericImageView};
use std::f64;
use std::cmp;

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}
 
/// Add a watermark to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `offset` - The watermark to be placed onto the `img` image.
/// * `x` - The x coordinate where the watermark's top corner should be positioned.
/// * `y` - The y coordinate where the watermark's top corner should be positioned.
/// # Example
///
/// ```
/// // For example, to offset pixels by 30 pixels on the red channel:
/// use photon::effects;
/// photon::multiple::watermark(img, watermark, 30, 40);
/// ```
pub fn watermark(mut img: DynamicImage, watermark: DynamicImage, x: u32, y: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    image::imageops::overlay(&mut img, &watermark, 40, 10);
    
    return img;
}