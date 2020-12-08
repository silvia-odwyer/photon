//! Image manipulation with multiple images, including adding watermarks, changing backgrounds, etc.,

extern crate image;
extern crate rand;
use crate::channels::color_sim;
use crate::iter::ImageIterator;
use crate::{helpers, GenericImage, PhotonImage, Rgb};
use image::DynamicImage::ImageRgba8;
use image::Pixel as ImagePixel;
use image::{DynamicImage, GenericImageView, RgbaImage};
use palette::{Blend, Gradient, Lab, Lch, LinSrgba, Srgb, Srgba};
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
/// ```no_run
/// // For example, to add a watermark to an image at x: 30, y: 40:
/// use photon_rs::multiple::watermark;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let water_mark = open_image("watermark.jpg").expect("File should open");
/// watermark(&mut img, &water_mark, 30_u32, 40_u32);
/// ```
#[wasm_bindgen]
pub fn watermark(mut img: &mut PhotonImage, watermark: &PhotonImage, x: u32, y: u32) {
    let dyn_watermark: DynamicImage = crate::helpers::dyn_image_from_raw(&watermark);
    let mut dyn_img: DynamicImage = crate::helpers::dyn_image_from_raw(&img);
    image::imageops::overlay(&mut dyn_img, &dyn_watermark, x, y);
    img.raw_pixels = dyn_img.to_bytes();
}

/// Blend two images together.
///
/// The `blend_mode` (3rd param) determines which blending mode to use; change this for varying effects.
/// The blend modes available include: `overlay`, `over`, `atop`, `xor`, `multiply`, `burn`, `soft_light`, `hard_light`,
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
/// ```no_run
/// // For example, to blend two images with the `multiply` blend mode:
/// use photon_rs::multiple::blend;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let img2 = open_image("img2.jpg").expect("File should open");
/// blend(&mut img, &img2, "multiply");
/// ```
#[wasm_bindgen]
pub fn blend(
    mut photon_image: &mut PhotonImage,
    photon_image2: &PhotonImage,
    blend_mode: &str,
) {
    let img = crate::helpers::dyn_image_from_raw(&photon_image);
    let img2 = crate::helpers::dyn_image_from_raw(&photon_image2);

    let (width, height) = img.dimensions();
    let (width2, height2) = img2.dimensions();

    if width > width2 || height > height2 {
        panic!("First image parameter must be smaller than second image parameter. To fix, swap img and img2 params.");
    }
    let mut img = img.to_rgba8();
    let img2 = img2.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let pixel = img.get_pixel(x, y);
        let pixel_img2 = img2.get_pixel(x, y);

        let px_data = pixel.channels();
        let px_data2 = pixel_img2.channels();
        // let rgb_color: Rgba = Rgba::new(px_data[0] as f32, px_data[1] as f32, px_data[2] as f32, 255.0);
        // let color: LinSrgba = LinSrgba::from_color(&rgb_color).into_format();

        let color = Srgb::new(
            px_data[0] as f32 / 255.0,
            px_data[1] as f32 / 255.0,
            px_data[2] as f32 / 255.0,
        )
        .into_linear();
        let color2 = Srgb::new(
            px_data2[0] as f32 / 255.0,
            px_data2[1] as f32 / 255.0,
            px_data2[2] as f32 / 255.0,
        )
        .into_linear();

        // let rgb_color2: Rgba = Rgba::new(px_data2[0] as f32, px_data2[1] as f32, px_data2[2] as f32, 255.0);
        // let color2: LinSrgba = LinSrgba::from_color(&rgb_color2).into_format();

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
            "exclusion" => color2.exclusion(color),
            "lighten" => color2.lighten(color),
            "darken" => color2.darken(color),
            _ => color2.overlay(color),
        };
        let components = blended.into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([
                (components.0 * 255.0) as u8,
                (components.1 * 255.0) as u8,
                (components.2 * 255.0) as u8,
                255,
            ]),
        );
    }
    let dynimage = ImageRgba8(img);
    photon_image.raw_pixels = dynimage.to_bytes();
}

// #[wasm_bindgen]
// pub fn blend_img_browser(
//     source_canvas: HtmlCanvasElement,
//     overlay_img: HtmlImageElement,
//     blend_mode: &str) {

//     let ctx = source_canvas
//     .get_context("2d").unwrap()
//     .unwrap()
//     .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

//     ctx.draw_image_with_html_image_element(&overlay_img, 0.0, 0.0);
//     ctx.set_global_composite_operation(blend_mode);
//     ctx.set_global_alpha(1.0);

// }

/// Change the background of an image (using a green screen/color screen).
///
/// # Arguments
/// * `img` - A PhotonImage which contains the desired background. Must be the same size as img2.
/// * `img2` - The image you would like to swap the background of. Must be the same size as img.
/// * `background_color` - The RGB value of the background, which should be replaced.
/// # Example
///
/// ```no_run
/// // For example, to replace the background of ImageA (which is RGB value 20, 40, 60) with the background of ImageB:
/// use photon_rs::Rgb;
/// use photon_rs::multiple::replace_background;
/// use photon_rs::native::open_image;
///
/// let rgb = Rgb::new(20_u8, 40_u8, 60_u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// let img2 = open_image("img2.jpg").expect("File should open");
/// replace_background(&mut img, &img2, rgb);
/// ```
pub fn replace_background(
    mut photon_image: &mut PhotonImage,
    img2: &PhotonImage,
    background_color: Rgb,
) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let img2 = helpers::dyn_image_from_raw(&img2);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let px = img.get_pixel(x, y);

        // Convert the current pixel's colour to the l*a*b colour space
        let lab: Lab = Srgb::new(
            background_color.r as f32 / 255.0,
            background_color.g as f32 / 255.0,
            background_color.b as f32 / 255.0,
        )
        .into();

        let channels = px.channels();

        let r_val: f32 = channels[0] as f32 / 255.0;
        let g_val: f32 = channels[1] as f32 / 255.0;
        let b_val: f32 = channels[2] as f32 / 255.0;

        let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();

        let sim = color_sim(lab, px_lab);

        // Match
        if sim < 20 {
            img.put_pixel(x, y, img2.get_pixel(x, y));
        } else {
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.to_bytes();
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

    for (i, c1) in grad1.take(width as usize).enumerate() {
        let c1: Srgba = Srgba::from_linear(c1).into_format();
        {
            let mut sub_image = image.sub_image(i as u32, 0, 1, height);
            for (x, y) in ImageIterator::with_dimension(&sub_image.dimensions()) {
                let components = c1.into_components();
                sub_image.put_pixel(
                    x,
                    y,
                    image::Rgba([
                        (components.0 * 255.0) as u8,
                        (components.1 * 255.0) as u8,
                        (components.2 * 255.0) as u8,
                        255,
                    ]),
                );
            }
        }
    }
    let rgba_img = ImageRgba8(image);
    let raw_pixels = rgba_img.to_bytes();
    PhotonImage {
        raw_pixels,
        width,
        height,
    }
}

/// Apply a gradient to an image.
#[wasm_bindgen]
pub fn apply_gradient(mut image: &mut PhotonImage) {
    let gradient = create_gradient(image.width, image.height);

    blend(&mut image, &gradient, "overlay");
}
