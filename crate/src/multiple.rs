//! Image manipulation with multiple images, including adding watermarks, changing backgrounds, etc.,

extern crate image;
extern crate rand;
use image::{DynamicImage, GenericImageView, RgbaImage};
use palette::{Srgba, LinSrgba, Lab, Blend, Lch, Pixel, Gradient, Srgb};
use crate::channels::color_sim;
use crate::{PhotonImage, Rgb, helpers, GenericImage};
use wasm_bindgen::prelude::*;

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
#[wasm_bindgen]
pub fn watermark(mut img: &mut PhotonImage, watermark: PhotonImage, x: u32, y: u32) {
    let dyn_watermark: DynamicImage = crate::helpers::dyn_image_from_raw(&watermark);
    let mut dyn_img: DynamicImage = crate::helpers::dyn_image_from_raw(&img);
    image::imageops::overlay(&mut dyn_img, &dyn_watermark, x, y);
    img.raw_pixels = dyn_img.raw_pixels();
}

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
#[wasm_bindgen]
pub fn blend(mut photon_image: &mut PhotonImage, photon_image2: &PhotonImage, blend_mode: &str) {
    let img = crate::helpers::dyn_image_from_raw(&photon_image);
    let img2 = crate::helpers::dyn_image_from_raw(&photon_image2);

    let (width, height) = img.dimensions();
    let (width2, height2) = img2.dimensions();

    if width > width2 || height > height2 {

        panic!("img must be smaller than img2! First image parameter must be smaller than second image parameter.To fix, swap img and img2 params.");
    }
    let mut img = img.to_rgba();

    for x in 0..width {
        for y in 0..height {
            
            let px_data = img.get_pixel(x, y).data;

            let color: LinSrgba = LinSrgba::from_raw(&px_data).into_format();

            let px_data2 = img2.get_pixel(x, y).data;

            let color2: LinSrgba = LinSrgba::from_raw(&px_data2).into_format();

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
            
            img.put_pixel(x, y, image::Rgba {
                    data: Srgba::from_linear(blended.into()).into_format().into_raw()
            });

        }
    }
    let dynimage = image::ImageRgba8(img);
    photon_image.raw_pixels = dynimage.raw_pixels();
}

/// Change the background of an image (using a green screen/color screen).
/// 
/// # Arguments
/// * `img` - A PhotonImage which contains the desired background. Must be the same size as img2.
/// * `img2` - The image you would like to swap the background of. Must be the same size as img.
/// * `background_color` - The RGB value of the background, which should be replaced.
/// # Example
///
/// ```
/// // For example, to replace the background of ImageA (which is RGB value 20, 40, 60) with the background of ImageB:
/// use photon::multiple;
/// let rgb = Rgb{20, 40, 60};
/// photon::multiple::replace_background(img_b, img_a, rgb);
/// ```
pub fn replace_background(mut photon_image: &mut PhotonImage, img2: &PhotonImage, background_color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let img2 = helpers::dyn_image_from_raw(&img2);
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);

            // Convert the current pixel's colour to the l*a*b colour space
            let lab: Lab = Srgb::new(background_color.r as f32 / 255.0, background_color.g as f32 / 255.0, background_color.b as f32 / 255.0).into();

            let r_val: f32 = px.data[0] as f32 / 255.0;
            let g_val: f32 = px.data[1] as f32 / 255.0;
            let b_val: f32 = px.data[2] as f32 / 255.0;

            let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();

            let sim = color_sim(lab, px_lab);

            // Match
            if sim < 20 {
                img.put_pixel(x, y, img2.get_pixel(x, y));
            }
            else {
                img.put_pixel(x, y, px);
            }
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

#[wasm_bindgen]
pub fn create_gradient(width: u32, height: u32) -> PhotonImage {
    let mut image = RgbaImage::new(width, height);

    // Create a gradient.
    let grad1 = Gradient::new(vec![
        LinSrgba::new(1.0, 0.1, 0.1, 1.0),
        LinSrgba::new(0.1, 0.1, 1.0, 1.0),
        LinSrgba::new(0.1, 1.0, 0.1, 1.0),
    ]);

    let _grad3 = Gradient::new(vec![
        Lch::from(LinSrgba::new(1.0, 0.1, 0.1, 1.0)),
        Lch::from(LinSrgba::new(0.1, 0.1, 1.0, 1.0)),
        Lch::from(LinSrgba::new(0.1, 1.0, 0.1, 1.0)),
    ]);

    for (i, c1) in grad1
        .take(width as usize)
        .enumerate()
    {
        let c1 = Srgba::from_linear(c1).into_format().into_raw();
        {
            let mut sub_image = image.sub_image(i as u32, 0, 1, height);
            let (width, height) = sub_image.dimensions();
            for x in 0..width {
                for y in 0..height {
                    sub_image.put_pixel(x, y, image::Rgba {
                        data: c1
                    });
                }
            }
        }
    }
    let rgba_img = image::ImageRgba8(image);
    let raw_pixels = rgba_img.raw_pixels();
    return PhotonImage { raw_pixels: raw_pixels, width: width, height: height};
}

/// Apply a gradient to an image.
#[wasm_bindgen]
pub fn apply_gradient(mut image: &mut PhotonImage) {
    
    let gradient = create_gradient(image.width, image.height);

    blend(&mut image, &gradient, "overlay");

}