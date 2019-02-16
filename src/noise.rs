extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use image::Pixel;
use rand::Rng;
// pub fn add_noise(mut img: DynamicImage, offset: u8) -> DynamicImage {
//     // Add Gaussian Noise Sample with offset specified by the user.
//     let (width, height) = img.dimensions();

//     for x in 0..width {
//         for y in 0..height {
//             let px = img.get_pixel(x, y).map(|ch| if ch <= 255 - offset { ch + offset } else { 255});
//             img.put_pixel(x, y, px);
//     }
//     }
//     return img;
// }

pub fn add_noise_rand(mut img: DynamicImage) -> DynamicImage {
    // Add Gaussian Noise Sample by including a random offset to each channel in each pixel.
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();

    for x in 0..width {
        for y in 0..height {
            let offset = rng.gen_range(0, 150);
            let px = img.get_pixel(x, y).map(|ch| if ch <= 255 - offset { ch + offset } else { 255});
            img.put_pixel(x, y, px);
        }
    }
    return img;
}