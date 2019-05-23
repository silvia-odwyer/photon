extern crate image;
extern crate rand;
use image::{DynamicImage, GenericImageView, GenericImage};
use palette::{Blend, Srgb, Pixel, LinSrgb};
use image::FilterType::Nearest;
/// Add a watermark to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `watermark` - The watermark to be placed onto the `img` image.
/// * `x` - The x coordinate where the watermark's top corner should be positioned.
/// * `y` - The y coordinate where the watermark's top corner should be positioned.
/// # Example
///
/// ```
/// // For example, to add a watermark to an image at x: 30, y: 40:
/// use photon::multiple;
/// photon::multiple::watermark(img, watermark, 30, 40);
/// ```
pub fn watermark(mut img: DynamicImage, watermark: DynamicImage, x: u32, y: u32) -> DynamicImage {
    image::imageops::overlay(&mut img, &watermark, x, y);
    
    return img;
}

/// Add a rotated watermark to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `watermark` - The watermark to be placed onto the `img` image.
/// * `x` - The x coordinate where the watermark's top corner should be positioned.
/// * `y` - The y coordinate where the watermark's top corner should be positioned.
/// * `degrees` - The number of degrees the watermark should be rotated by.
/// # Example
///
/// ```
/// // For example, to rotate the watermark by 30 degrees and place it at x: 30, y: 40:
/// use photon::multiple;
/// photon::multiple::watermark(img, watermark, 30, 40, 120);
/// ```
// pub fn watermark_rotate(mut img: DynamicImage, watermark: DynamicImage, x: u32, y: u32, degrees: u8) -> DynamicImage {
//     image::imageops::overlay(&mut img, &watermark, x, y);
    
//     return img;
// }

/// Blend two images together.
/// The `blend_mode` (3rd param) determines which blending mode to use; change this for varying effects.
/// The blend modes available include: `overlay`, `over`, `atop`, `xor`, `multiply`, `burn`, `soft_light`, `hard_light`
/// `difference`, `lighten`, `darken`, `dodge`, `plus`, `exclusion` (more to come)
/// NOTE: The first image must be smaller than the second image passed as params. 
/// If the first image were larger than the second, then there would be overflowing pixels which would have no corresponding pixels 
/// in the second image. 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `img2` - The 2nd DynamicImage to be blended with the first.
/// * `blend_mode` - The blending mode to use. See above for complete list of blend modes available.
/// # Example
///
/// ```
/// // For example, to add a watermark to an image at x: 30, y: 40:
/// use photon::multiple;
/// photon::multiple::watermark(img, watermark, 30, 40);
/// ```
pub fn blend(img: DynamicImage, mut img2: DynamicImage, blend_mode: &'static str) -> DynamicImage {
    let (width, height) = img.dimensions();
    let (width2, height2) = img2.dimensions();

    if (width > width2 || height > height2) {

        panic!("img must be smaller than img2! First image parameter must be smaller than second image parameter.To fix, swap img and img2 params.");
    }
    let mut img = img.to_rgb();

    for x in 0..width {
        for y in 0..height {
            let mut px2 = img2.get_pixel(x, y);
            
            let px_data = img.get_pixel(x, y).data;

            let color: LinSrgb = LinSrgb::from_raw(&px_data).into_format();

            let px_data2 = img2.get_pixel(x, y).data;

            let color2: LinSrgb = LinSrgb::from_raw(&px_data2).into_format();

            let number = 1;
            let blended = match blend_mode {
                // Match a single value
                "overlay" => color2.overlay(color),
                "over" => color2.over(color),
                "atop" => color2.atop(color),
                "xor" => color2.xor(color),
                "plus" => color2.plus(color),
                "multiply" => color2.multiply(color),
                "burn" => color2.burn(color),
                "difference" => color2.difference(color),
                "soft_light" => color2.soft_light(color),
                "hard_light" => color2.hard_light(color),
                "dodge" => color2.dodge(color),
                "exclusion" => color2.dodge(color),
                "lighten" => color2.lighten(color),
                "darken" => color2.darken(color),
                _ => color2.overlay(color),
                };
            
            img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(blended.into()).into_format().into_raw()
            });

        }
    }
    let dynimage = image::ImageRgb8(img);
    return dynimage;

}
