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
/// * `offset` - The offset is added to the pixels in the image.  
/// # Example
///
/// ```
/// // For example, to offset pixels by 30 pixels on the red channel:
/// use photon::effects;
/// photon::effects::offset(img, 0, 30);
/// ```
pub fn watermark(mut img: DynamicImage, watermark: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    image::imageops::overlay(&mut img, &watermark, 40, 10);
    
    return img;
}