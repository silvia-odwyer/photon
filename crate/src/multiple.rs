//! Image manipulation with multiple images, including adding watermarks, changing backgrounds, etc.,

use crate::channels::color_sim;
use crate::iter::ImageIterator;
use crate::{helpers, GenericImage, PhotonImage, Rgb};
use image::DynamicImage::ImageRgba8;
use image::Pixel as ImagePixel;
use image::{DynamicImage, GenericImageView, RgbaImage};
use palette::{Blend, Gradient, Lab, Lch, LinSrgba, Srgb, Srgba};
use palette::{FromColor, IntoColor};
use std::cmp::{max, min};

#[cfg(feature = "enable_wasm")]
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
/// watermark(&mut img, &water_mark, 30_i64, 40_i64);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn watermark(img: &mut PhotonImage, watermark: &PhotonImage, x: i64, y: i64) {
    let dyn_watermark: DynamicImage = crate::helpers::dyn_image_from_raw(watermark);
    let mut dyn_img: DynamicImage = crate::helpers::dyn_image_from_raw(img);
    image::imageops::overlay(&mut dyn_img, &dyn_watermark, x, y);
    img.raw_pixels = dyn_img.into_bytes();
}

/// Blend two images together.
///
/// The `blend_mode` (3rd param) determines which blending mode to use; change this for varying effects.
/// The blend modes available include: `overlay`, `over`, `atop`, `xor`, `plus`, `multiply`, `burn`,
/// `difference`, `soft_light`, `screen`, `hard_light`, `dodge`, `exclusion`, `lighten`, `darken` (more to come)
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn blend(
    photon_image: &mut PhotonImage,
    photon_image2: &PhotonImage,
    blend_mode: &str,
) {
    let img = crate::helpers::dyn_image_from_raw(photon_image);
    let img2 = crate::helpers::dyn_image_from_raw(photon_image2);

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

        let color = LinSrgba::new(
            px_data[0] as f32 / 255.0,
            px_data[1] as f32 / 255.0,
            px_data[2] as f32 / 255.0,
            px_data[3] as f32 / 255.0,
        )
        .into_linear();

        let color2 = LinSrgba::new(
            px_data2[0] as f32 / 255.0,
            px_data2[1] as f32 / 255.0,
            px_data2[2] as f32 / 255.0,
            px_data2[3] as f32 / 255.0,
        )
        .into_linear();

        let blended = match blend_mode.to_lowercase().as_str() {
            // Match a single value
            "overlay" => color.overlay(color2),
            "over" => color2.over(color),
            "atop" => color2.atop(color),
            "xor" => color2.xor(color),
            "plus" => color2.plus(color),
            "multiply" => color2.multiply(color),
            "burn" => color2.burn(color),
            "difference" => color2.difference(color),
            "soft_light" | "soft light" | "softlight" => color2.soft_light(color),
            "screen" => color2.screen(color),
            "hard_light" | "hard light" | "hardlight" => color2.hard_light(color),
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
                (components.3 * 255.0) as u8,
            ]),
        );
    }
    let dynimage = ImageRgba8(img);
    photon_image.raw_pixels = dynimage.into_bytes();
}

// #[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
/// replace_background(&mut img, &img2, &rgb);
/// ```
pub fn replace_background(
    photon_image: &mut PhotonImage,
    img2: &PhotonImage,
    background_color: &Rgb,
) {
    let mut img = helpers::dyn_image_from_raw(photon_image);
    let img2 = helpers::dyn_image_from_raw(img2);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let px = img.get_pixel(x, y);

        // Convert the current pixel's colour to the l*a*b colour space
        let lab: Lab = Srgb::new(
            background_color.r as f32 / 255.0,
            background_color.g as f32 / 255.0,
            background_color.b as f32 / 255.0,
        )
        .into_color();

        let channels = px.channels();

        let r_val: f32 = channels[0] as f32 / 255.0;
        let g_val: f32 = channels[1] as f32 / 255.0;
        let b_val: f32 = channels[2] as f32 / 255.0;

        let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into_color();

        let sim = color_sim(lab, px_lab);

        // Match
        if sim < 20 {
            img.put_pixel(x, y, img2.get_pixel(x, y));
        } else {
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn create_gradient(width: u32, height: u32) -> PhotonImage {
    let mut image = RgbaImage::new(width, height);

    // Create a gradient.
    let grad1 = Gradient::new(vec![
        LinSrgba::new(1.0, 0.1, 0.1, 1.0),
        LinSrgba::new(0.1, 0.1, 1.0, 1.0),
        LinSrgba::new(0.1, 1.0, 0.1, 1.0),
    ]);

    let _grad3 = Gradient::new(vec![
        Lch::from_color(LinSrgba::new(1.0, 0.1, 0.1, 1.0)),
        Lch::from_color(LinSrgba::new(0.1, 0.1, 1.0, 1.0)),
        Lch::from_color(LinSrgba::new(0.1, 1.0, 0.1, 1.0)),
    ]);

    for (i, c1) in grad1.take(width as usize).enumerate() {
        let c1: Srgba<f32> = Srgba::from_linear(c1).into_format();
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
    let raw_pixels = rgba_img.into_bytes();
    PhotonImage {
        raw_pixels,
        width,
        height,
    }
}

/// Apply a gradient to an image.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_gradient(image: &mut PhotonImage) {
    let gradient = create_gradient(image.width, image.height);

    blend(image, &gradient, "overlay");
}

/// Build a simple horizontal gradient.
fn build_horizontal_gradient(
    width: usize,
    height: usize,
    start_x: i32,
    end_x: i32,
) -> Vec<f32> {
    let min_x = min(start_x, end_x);
    let max_x = max(start_x, end_x);
    let total_grad_len = max_x - min_x;
    let total_size = width * height;
    let mut gradient = std::iter::repeat(0.0).take(total_size).collect::<Vec<_>>();
    if total_grad_len <= 0 {
        // Nothing to do. Return a vector filled with zeros.
        return gradient;
    }

    // Fill every column from 0 to the leftmost x with zeros.
    for row in 0..height {
        for col in 0..min_x {
            let pos = row * width + col as usize;
            gradient[pos] = 0.0;
        }
    }

    // Fill every column from the rightmost x to image width with ones.
    // If the rightmost x is less than 0, start with 0.
    let first_col = max(max_x, 0) as usize;
    for row in 0..height {
        for col in first_col..width {
            let pos = row * width + col;
            gradient[pos] = 1.0;
        }
    }

    // Build gradient between the leftmost and the rightmost x.
    // Clamp values in such a way that they belong to the visible area.
    let first_col = max(min_x, 0);
    let last_col = min(max_x, width as i32);
    for row in 0..height {
        for col in first_col..last_col {
            let pos = row * width + col as usize;
            let total_len_f32 = total_grad_len as f32;
            let column_f32 = (col - min_x) as f32;
            gradient[pos] = (column_f32 / total_len_f32).clamp(0.0, 1.0);
        }
    }

    // Inverse values when start_x is on the right.
    if start_x > end_x {
        gradient.iter_mut().for_each(|grad| *grad = 1.0 - *grad);
    }

    gradient
}

/// Build a simple vertical gradient.
fn build_vertical_gradient(
    width: usize,
    height: usize,
    start_y: i32,
    end_y: i32,
) -> Vec<f32> {
    let min_y = min(start_y, end_y);
    let max_y = max(start_y, end_y);
    let total_grad_len = max_y - min_y;
    let total_size = width * height;
    let mut gradient = std::iter::repeat(0.0).take(total_size).collect::<Vec<_>>();
    if total_grad_len <= 0 {
        // Nothing to do. Return a vector filled with zeros.
        return gradient;
    }

    // Fill every row from 0 to the top y with zeros.
    for row in 0..min_y {
        for col in 0..width {
            let pos = (row as usize) * width + col;
            gradient[pos] = 0.0;
        }
    }

    // Fill every row from the bottom y to image height with ones.
    // If the bottom y is less than 0, start with 0.
    let first_row = max(max_y, 0) as usize;
    for row in first_row..height {
        for col in 0..width {
            let pos = row * width + col;
            gradient[pos] = 1.0;
        }
    }

    // Build gradient between the top and the bottom y.
    // Clamp values in such a way that they belong to the visible area.
    let first_row = max(min_y, 0);
    let last_row = min(max_y, height as i32);
    for row in first_row..last_row {
        for col in 0..width {
            let pos = (row as usize) * width + col;
            let total_len_f32 = total_grad_len as f32;
            let row_f32 = (row - min_y) as f32;
            gradient[pos] = (row_f32 / total_len_f32).clamp(0.0, 1.0);
        }
    }

    // Inverse values when start_y is at the bottom.
    if start_y > end_y {
        gradient.iter_mut().for_each(|grad| *grad = 1.0 - *grad);
    }

    gradient
}

/// Build an axial gradient.
fn build_axial_gradient(
    width: usize,
    height: usize,
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
) -> Vec<f32> {
    let len_x = (end_x - start_x) as f32;
    let len_y = (end_y - start_y) as f32;
    let total_grad_len = (len_x * len_x + len_y * len_y).sqrt();

    let total_size = width * height;
    let mut gradient = std::iter::repeat(0.0).take(total_size).collect::<Vec<_>>();
    if total_grad_len <= 0.0 {
        // Nothing to do. Return a vector filled with zeros.
        return gradient;
    }

    let min_x = min(start_x, end_x) as f32;
    let max_x = max(start_x, end_x) as f32;
    let min_y = min(start_y, end_y) as f32;
    let max_y = max(start_y, end_y) as f32;
    let len_x_sq = len_x * len_x;
    let len_y_sq = len_y * len_y;
    let start_x_f32 = start_x as f32;
    let start_y_f32 = start_y as f32;

    // Build gradient between start_x, end_x, start_y and end_y.
    // The idea is to find the foot of perpendicular from each point to the gradient line.
    // If the foot belongs to the gradient line, find the distance from (start_x, start_y)
    // to the foot point and divide it by total gradient length.
    // If the foot exceeds gradient line bounds, fill it with zeros or ones in accordance with
    // the direction of gradient vector.
    for row in 0..height {
        for col in 0..width {
            let pos = row * width + col;
            let col_f32 = col as f32;
            let row_f32 = row as f32;
            let foot_x = (start_x_f32 * len_y_sq
                + col_f32 * len_x_sq
                + len_x * len_y * (row_f32 - start_y_f32))
                / (len_y_sq + len_x_sq);
            let foot_y = (len_x * (col_f32 - foot_x)) / len_y + row_f32;

            // Check that found coordinates do not exceed gradient bounds.
            if min_x <= foot_x && foot_x <= max_x && min_y <= foot_y && foot_y <= max_y {
                let norm_x = foot_x - start_x_f32;
                let norm_y = foot_y - start_y_f32;
                let grad_dist = (norm_x * norm_x + norm_y * norm_y).sqrt();
                let total_len_f32 = total_grad_len;
                gradient[pos] = (grad_dist / total_len_f32).clamp(0.0, 1.0);
            } else {
                let fill_bottom_right =
                    start_x < end_x && start_y < end_y && foot_x > max_x;
                let fill_bottom_left =
                    start_x > end_x && start_y < end_y && foot_x < min_x;
                let fill_top_right =
                    start_x < end_x && start_y > end_y && foot_y < min_y;
                let fill_top_left = start_x > end_x && start_y > end_y && foot_y < min_y;
                if fill_bottom_right
                    || fill_bottom_left
                    || fill_top_right
                    || fill_top_left
                {
                    gradient[pos] = 1.0;
                }
            }
        }
    }

    gradient
}

/// Fades one image into another.
///
/// For horizontal fading, set both `start_y` and `end_y` to the same value.
/// For vertical fading, set both `start_x` and `end_x` to the same value.
/// Otherwise, axial fading is applied.
///
/// # Arguments
/// * `img1` - Image to fade from. Must be the same size as img2.
/// * `img2` - Image to fade to. Must be the same size as img1.
/// * `start_x` - Column where the fading begins.
/// * `end_x` - Column where the fading ends.
/// * `start_y` - Row where the fading begins.
/// * `end_y` - Row where the fading ends.
/// # Example
///
/// ```no_run
/// use photon_rs::multiple::fade;
/// use photon_rs::native::open_image;
///
/// let img1 = open_image("img1.jpg").expect("File should open");
/// let img2 = open_image("img2.jpg").expect("File should open");
/// let _faded_img = fade(&img1, &img2, 0, 100, 0, 100);
/// ```
pub fn fade(
    img1: &PhotonImage,
    img2: &PhotonImage,
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
) -> PhotonImage {
    if img1.width != img2.width || img1.height != img2.height {
        panic!("Images must have the same size.");
    }

    let width = img1.width as usize;
    let height = img1.height as usize;

    let buf_img1 = &img1.raw_pixels;
    let buf_img2 = &img2.raw_pixels;
    let mut buf_res = Vec::with_capacity(width * height * 4);

    // Determine, which gradient must be built.
    let gradient = if end_y == start_y {
        build_horizontal_gradient(width, height, start_x, end_x)
    } else if start_x == end_x {
        build_vertical_gradient(width, height, start_y, end_y)
    } else {
        build_axial_gradient(width, height, start_x, end_x, start_y, end_y)
    };

    for row in 0..height {
        for col in 0..width {
            let grad_idx = row * width + col;
            let opacity_img1 = gradient[grad_idx];
            let opacity_img2 = 1.0 - opacity_img1;

            let buf_idx = row * width * 4 + col * 4;

            let img1_r = buf_img1[buf_idx] as f32;
            let img1_g = buf_img1[buf_idx + 1] as f32;
            let img1_b = buf_img1[buf_idx + 2] as f32;

            let img2_r = buf_img2[buf_idx] as f32;
            let img2_g = buf_img2[buf_idx + 1] as f32;
            let img2_b = buf_img2[buf_idx + 2] as f32;

            let res_r = ((img1_r * opacity_img1) + (img2_r * opacity_img2))
                .clamp(0.0, 255.0) as u8;
            let res_g = ((img1_g * opacity_img1) + (img2_g * opacity_img2))
                .clamp(0.0, 255.0) as u8;
            let res_b = ((img1_b * opacity_img1) + (img2_b * opacity_img2))
                .clamp(0.0, 255.0) as u8;

            // Set alpha channel to 100%.
            let res_a = 255;

            buf_res.push(res_r);
            buf_res.push(res_g);
            buf_res.push(res_b);
            buf_res.push(res_a);
        }
    }

    PhotonImage::new(buf_res, img1.width, img1.height)
}
