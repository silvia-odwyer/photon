extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use rand::Rng;
    
pub fn offset(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();

    for x in 0..width {
        for y in 0..height {
            let offset = rng.gen_range(0, 150);
            let mut px = img.get_pixel(x, y);

            if x + 10 < width - 1 && y + 10 < height - 1  {
                let offset_px = img.get_pixel(x + 10, y + 10);
                px = offset_px;
            }
        }
    }
    return img;
}