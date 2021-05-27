//! Special effects.

use crate::helpers;
use crate::iter::ImageIterator;
use crate::{PhotonImage, Rgb};
use image::Pixel;
use image::Rgba;
use image::{GenericImage, GenericImageView};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use perlin2d::PerlinNoise2D;
use std::collections::HashMap;
use std::f64;
use wasm_bindgen::prelude::*;

/// Adds an offset to the image by a certain number of pixels.
///
/// This creates an RGB shift effect.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `channel_index`: The index of the channel to increment. 0 for red, 1 for green and 2 for blue.
/// * `offset` - The offset is added to the pixels in the image.
/// # Example
///
/// ```no_run
/// // For example, to offset pixels by 30 pixels on the red channel:
/// use photon_rs::effects::offset;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// offset(&mut img, 0_usize, 30_u32);
/// ```
#[wasm_bindgen]
pub fn offset(photon_image: &mut PhotonImage, channel_index: usize, offset: u32) {
    if channel_index > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }

    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width - 10 {
        for y in 0..height - 10 {
            let px = img.get_pixel(x, y);

            if x + offset < width - 1 && y + offset < height - 1 {
                let offset_px = img.get_pixel(x + offset, y + offset);
                let offset_px_channels = offset_px.channels();

                let px_channels = px.channels();

                let px = match channel_index {
                    0 => image::Rgba([
                        offset_px_channels[0],
                        px_channels[1],
                        px_channels[2],
                        255,
                    ]),
                    1 => image::Rgba([
                        px_channels[0],
                        offset_px_channels[1],
                        px_channels[2],
                        255,
                    ]),
                    2 => image::Rgba([
                        px_channels[0],
                        px_channels[1],
                        offset_px_channels[2],
                        255,
                    ]),
                    _ => image::Rgba([
                        px_channels[0],
                        px_channels[1],
                        offset_px_channels[2],
                        255,
                    ]),
                };
                img.put_pixel(x, y, px);
            }
        }
    }
    let raw_pixels = img.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Adds an offset to the red channel by a certain number of pixels.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset you want to move the red channel by.
/// # Example
///
/// ```no_run
/// // For example, to add an offset to the red channel by 30 pixels.
/// use photon_rs::effects::offset_red;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// offset_red(&mut img, 30_u32);
/// ```
#[wasm_bindgen]
pub fn offset_red(img: &mut PhotonImage, offset_amt: u32) {
    offset(img, 0, offset_amt)
}

/// Adds an offset to the green channel by a certain number of pixels.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset you want to move the green channel by.
/// # Example
///
/// ```no_run
/// // For example, to add an offset to the green channel by 30 pixels.
/// use photon_rs::effects::offset_green;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// offset_green(&mut img, 30_u32);
/// ```
#[wasm_bindgen]
pub fn offset_green(img: &mut PhotonImage, offset_amt: u32) {
    offset(img, 1, offset_amt)
}

/// Adds an offset to the blue channel by a certain number of pixels.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset_amt` - The offset you want to move the blue channel by.
/// # Example
/// // For example, to add an offset to the green channel by 40 pixels.
///
/// ```no_run
/// use photon_rs::effects::offset_blue;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// offset_blue(&mut img, 40_u32);
/// ```
#[wasm_bindgen]
pub fn offset_blue(img: &mut PhotonImage, offset_amt: u32) {
    offset(img, 2, offset_amt)
}

/// Adds multiple offsets to the image by a certain number of pixels (on two channels).
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset is added to the pixels in the image.
/// # Example
///
/// ```no_run
/// // For example, to add a 30-pixel offset to both the red and blue channels:
/// use photon_rs::effects::multiple_offsets;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// multiple_offsets(&mut img, 30_u32, 0_usize, 2_usize);
/// ```
#[wasm_bindgen]
pub fn multiple_offsets(
    mut photon_image: &mut PhotonImage,
    offset: u32,
    channel_index: usize,
    channel_index2: usize,
) {
    if channel_index > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }
    if channel_index2 > 2 {
        panic!("Invalid channel index passed. Channel2 must be equal to 0, 1, or 2.");
    }
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for (x, y) in ImageIterator::new(width, height) {
        let mut px = img.get_pixel(x, y);

        if x + offset < width - 1 && y + offset < height - 1 {
            let offset_px = img.get_pixel(x + offset, y);

            px[channel_index] = offset_px[channel_index];
        }

        if x as i32 - offset as i32 > 0 && y as i32 - offset as i32 > 0 {
            let offset_px2 = img.get_pixel(x - offset, y);

            px[channel_index2] = offset_px2[channel_index2];
        }

        img.put_pixel(x, y, px);
    }
    let raw_pixels = img.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Halftoning effect.
pub fn halftone(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in (0..width - 4).step_by(2_usize) {
        for y in (0..height - 4).step_by(2_usize) {
            let mut px1 = img.get_pixel(x, y);
            let mut px2 = img.get_pixel(x, y + 1);
            let mut px3 = img.get_pixel(x + 1, y);
            let mut px4 = img.get_pixel(x + 1, y + 1);

            let gray1 = (px1[0] as f64 * 0.299)
                + (px1[1] as f64 * 0.587)
                + (px1[2] as f64 * 0.114);
            let gray2 = (px2[0] as f64 * 0.299)
                + (px2[1] as f64 * 0.587)
                + (px2[2] as f64 * 0.114);
            let gray3 = (px3[0] as f64 * 0.299)
                + (px3[1] as f64 * 0.587)
                + (px3[2] as f64 * 0.114);
            let gray4 = (px4[0] as f64 * 0.299)
                + (px4[1] as f64 * 0.587)
                + (px4[2] as f64 * 0.114);

            let sat = (gray1 + gray2 + gray3 + gray4) / 4.0;

            if sat > 200.0 {
                px1[0] = 255;
                px1[1] = 255;
                px1[2] = 255;

                px2[0] = 255;
                px2[1] = 255;
                px2[2] = 255;

                px3[0] = 255;
                px3[1] = 255;
                px3[2] = 255;

                px4[0] = 255;
                px4[1] = 255;
                px4[2] = 255;
            } else if sat > 159.0 {
                px1[0] = 255;
                px1[1] = 255;
                px1[2] = 255;

                px2[0] = 0;
                px2[1] = 0;
                px2[2] = 0;

                px3[0] = 255;
                px3[1] = 255;
                px3[2] = 255;

                px4[0] = 255;
                px4[1] = 255;
                px4[2] = 255;
            } else if sat > 95.0 {
                px1[0] = 255;
                px1[1] = 255;
                px1[2] = 255;

                px2[0] = 0;
                px2[1] = 0;
                px2[2] = 0;

                px3[0] = 0;
                px3[1] = 0;
                px3[2] = 0;

                px4[0] = 255;
                px4[1] = 255;
                px4[2] = 255;
            } else if sat > 32.0 {
                px1[0] = 0;
                px1[1] = 0;
                px1[2] = 0;

                px2[0] = 255;
                px2[1] = 255;
                px2[2] = 255;

                px3[0] = 0;
                px3[1] = 0;
                px3[2] = 0;

                px4[0] = 0;
                px4[1] = 0;
                px4[2] = 0;
            } else {
                px1[0] = 0;
                px1[1] = 0;
                px1[2] = 0;

                px2[0] = 0;
                px2[1] = 0;
                px2[2] = 0;

                px3[0] = 0;
                px3[1] = 0;
                px3[2] = 0;

                px4[0] = 0;
                px4[1] = 0;
                px4[2] = 0;
            }

            img.put_pixel(x, y, px1);
            // img.put_pixel(x, y + 1, px2);
        }
    }
    let raw_pixels = img.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Reduces an image to the primary colours.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to add a primary colour effect to an image of type `DynamicImage`:
/// use photon_rs::effects::primary;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// primary(&mut img);
/// ```
#[wasm_bindgen]
pub fn primary(img: &mut PhotonImage) {
    let end = img.raw_pixels.len() - 4;

    for i in (0..end).step_by(4) {
        let mut r_val = img.raw_pixels[0];
        let mut g_val = img.raw_pixels[1];
        let mut b_val = img.raw_pixels[2];

        if r_val > 128 {
            r_val = 255;
        } else {
            r_val = 0;
        }

        if g_val > 128 {
            g_val = 255;
        } else {
            g_val = 0;
        }

        if b_val > 128 {
            g_val = 255;
        } else {
            b_val = 0;
        }

        img.raw_pixels[i] = r_val;
        img.raw_pixels[i + 1] = g_val;
        img.raw_pixels[i + 2] = b_val;
    }
}

/// Colorizes the green channels of the image.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to colorize an image of type `PhotonImage`:
/// use photon_rs::effects::colorize;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// colorize(&mut img);
/// ```
#[wasm_bindgen]
pub fn colorize(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let threshold = 220;

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let channels = px.channels();
        let px_as_rgb = Rgb {
            r: channels[0],
            g: channels[1],
            b: channels[2],
        };

        let baseline_color = Rgb {
            r: 0,
            g: 255,
            b: 255,
        };

        let square_distance = crate::helpers::square_distance(baseline_color, px_as_rgb);

        let mut r = channels[0] as f32;
        let mut g = channels[1] as f32;
        let mut b = channels[2] as f32;

        if square_distance < i32::pow(threshold, 2) {
            r *= 0.5;
            g *= 1.25;
            b *= 0.5;
        }

        px = image::Rgba([r as u8, g as u8, b as u8, 255]);
        img.put_pixel(x, y, px);
    }
    let raw_pixels = img.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}

// #[wasm_bindgen]
// pub fn inc_luminosity(mut photon_image: PhotonImage) -> PhotonImage {
//     let mut img = helpers::dyn_image_from_raw(&photon_image);
//     let (width, height) = img.dimensions();
//     let mut min_intensity = 255;
//     let mut max_intensity = 0;

//     // find the max and min intensities in the image
//     for x in 0..width {
//         for y in 0..height {
//             let px = img.get_pixel(x, y);
//             let intensity = (px.data[0] as u32 + px.data[1] as u32 + px.data[2] as u32) / 3;
//             if intensity > 0{
//                 min_intensity = cmp::min(min_intensity, intensity);
//                 max_intensity = cmp::max(max_intensity, intensity);
//             }

//         }
//     }

//     for x in 0..width {
//         for y in 0..height {
//             let mut px = img.get_pixel(x, y);
//             // let px_as_rgb = Rgb{r: px.data[0], g: px.data[1], b: px.data[2]};

//             let mut r = px.data[0] as f32;
//             let mut g = px.data[1] as f32;
//             let mut b = px.data[2] as f32;

//             let lum = (r + g + b) / 3.0;

//             let new_lum = 255.0 * (lum - min_intensity as f32) / (max_intensity / min_intensity) as f32;

//             r = r * new_lum / lum;
//             g = g * new_lum / lum;
//             b = b * new_lum / lum;

//             px.data[0] = r as u8;
//             px.data[1] = g as u8;
//             px.data[2] = b as u8;

//             img.put_pixel(x, y, px);
//         }
//     }
//     let mut raw_pixels = img.raw_pixels();
//     photon_image.raw_pixels = raw_pixels;
//     photon_image
// }

/// Applies a solarizing effect to an image.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to colorize an image of type `PhotonImage`:
/// use photon_rs::effects::solarize;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// solarize(&mut img);
/// ```
#[wasm_bindgen]
pub fn solarize(photon_image: &mut PhotonImage) {
    let end = photon_image.get_raw_pixels().len() - 4;

    for i in (0..end).step_by(4) {
        let r_val = photon_image.raw_pixels[i];

        if 200 - r_val as i32 > 0 {
            photon_image.raw_pixels[i] = 200 - r_val;
        }
    }
}

/// Applies a solarizing effect to an image and returns the resulting PhotonImage.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to solarize "retimg" an image of type `PhotonImage`:
/// use photon_rs::effects::solarize_retimg;
/// use photon_rs::native::open_image;
/// use photon_rs::PhotonImage;
///
/// let img = open_image("img.jpg").expect("File should open");
/// let result: PhotonImage = solarize_retimg(&img);
/// ```
#[wasm_bindgen]
pub fn solarize_retimg(photon_image: &PhotonImage) -> PhotonImage {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let channels = px.channels();
        if 200_i32 - channels[0] as i32 > 0 {
            let new_r_val = 200 - channels[0];
            px = image::Rgba([new_r_val, channels[1], channels[2], channels[3]]);
        }
        img.put_pixel(x, y, px);
    }

    PhotonImage {
        raw_pixels: img.to_bytes(),
        width: img.width(),
        height: img.height(),
    }
}

/// Increase the brightness of an image by a factor.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `brightness` - A u8 to add to the brightness.
/// # Example
///
/// ```no_run
/// use photon_rs::effects::inc_brightness;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// inc_brightness(&mut img, 10_u8);
/// ```
#[wasm_bindgen]
pub fn inc_brightness(photon_image: &mut PhotonImage, brightness: u8) {
    let end = photon_image.get_raw_pixels().len() - 4;
    for i in (0..end).step_by(4) {
        let r_val = photon_image.raw_pixels[i];
        let g_val = photon_image.raw_pixels[i + 1];
        let b_val = photon_image.raw_pixels[i + 2];

        if r_val <= 255 - brightness {
            photon_image.raw_pixels[i] += brightness;
        } else {
            photon_image.raw_pixels[i] = 255;
        }
        if g_val <= 255 - brightness {
            photon_image.raw_pixels[i + 1] += brightness;
        } else {
            photon_image.raw_pixels[1] = 255
        }

        if b_val <= 255 - brightness {
            photon_image.raw_pixels[i + 2] += brightness;
        } else {
            photon_image.raw_pixels[i + 2] = 255
        }
    }
}

/// Adjust the contrast of an image by a factor.
///
/// # Arguments
/// * `photon_image` - A PhotonImage that contains a view into the image.
/// * `contrast` - An f32 factor used to adjust contrast. Between [-255.0, 255.0]. The algorithm will
/// clamp results if passed factor is out of range.
/// # Example
///
/// ```no_run
/// use photon_rs::effects::adjust_contrast;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// adjust_contrast(&mut img, 30_f32);
/// ```
#[wasm_bindgen]
pub fn adjust_contrast(mut photon_image: &mut PhotonImage, contrast: f32) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    let clamped_contrast = num::clamp(contrast, -255.0, 255.0);

    // Some references:
    // https://math.stackexchange.com/questions/906240/algorithms-to-increase-or-decrease-the-contrast-of-an-image
    // https://www.dfstudios.co.uk/articles/programming/image-programming-algorithms/image-processing-algorithms-part-5-contrast-adjustment/
    let factor =
        (259.0 * (clamped_contrast + 255.0)) / (255.0 * (259.0 - clamped_contrast));
    let mut lookup_table: Vec<u8> = vec![0; 256];
    let offset = -128.0 * factor + 128.0;

    for (i, table) in lookup_table.iter_mut().enumerate().take(256_usize) {
        let new_val = i as f32 * factor + offset;
        *table = num::clamp(new_val, 0.0, 255.0) as u8;
    }

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let channels = px.channels();

        px = image::Rgba([
            lookup_table[channels[0] as usize],
            lookup_table[channels[1] as usize],
            lookup_table[channels[2] as usize],
            255,
        ]);
        img.put_pixel(x, y, px);
    }

    photon_image.raw_pixels = img.to_bytes();
}

/// Tint an image by adding an offset to averaged RGB channel values.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `r_offset` - The amount the R channel should be incremented by.
/// * `g_offset` - The amount the G channel should be incremented by.
/// * `b_offset` - The amount the B channel should be incremented by.
/// # Example
///
/// ```no_run
/// // For example, to tint an image of type `PhotonImage`:
/// use photon_rs::effects::tint;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// tint(&mut img, 10_u32, 20_u32, 15_u32);
/// ```
///
#[wasm_bindgen]
pub fn tint(
    mut photon_image: &mut PhotonImage,
    r_offset: u32,
    g_offset: u32,
    b_offset: u32,
) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let channels = px.channels();
        let (r_val, g_val, b_val) =
            (channels[0] as u32, channels[1] as u32, channels[2] as u32);

        let new_r_val = if r_val as u32 + r_offset < 255 {
            r_val as u8 + r_offset as u8
        } else {
            255
        };
        let new_g_val = if g_val as u32 + g_offset < 255 {
            g_val as u8 + g_offset as u8
        } else {
            255
        };
        let new_b_val = if b_val as u32 + b_offset < 255 {
            b_val as u8 + b_offset as u8
        } else {
            255
        };

        px = image::Rgba([new_r_val, new_g_val, new_b_val, 255]);

        img.put_pixel(x, y, px);
    }

    let raw_pixels = img.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}

fn draw_horizontal_strips(
    mut photon_image: &mut PhotonImage,
    num_strips: u8,
    color: Rgb,
) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    let total_strips = (num_strips * 2) - 1;
    let height_strip = height / total_strips as u32;
    let mut y_pos: u32 = 0;
    for i in 1..num_strips {
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, (y_pos + height_strip) as i32).of_size(width, height_strip),
            Rgba([color.r, color.g, color.b, 255u8]),
        );
        y_pos = i as u32 * (height_strip * 2);
    }

    let raw_pixels = img.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Horizontal strips. Divide an image into a series of equal-height strips, for an artistic effect.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `num_strips` - The number of strips
/// # Example
///
/// ```no_run
/// // For example, to draw horizontal strips on a `PhotonImage`:
/// use photon_rs::effects::horizontal_strips;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// horizontal_strips(&mut img, 8u8);
/// ```
///
#[wasm_bindgen]
pub fn horizontal_strips(mut photon_image: &mut PhotonImage, num_strips: u8) {
    let color = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    draw_horizontal_strips(&mut photon_image, num_strips, color)
}

/// Horizontal strips. Divide an image into a series of equal-width strips, for an artistic effect. Sepcify a color as well.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `num_strips` - The numbder of strips
/// * `color` - Color of strips.
/// # Example
///
/// ```no_run
/// // For example, to draw blue horizontal strips on a `PhotonImage`:
/// use photon_rs::effects::color_horizontal_strips;
/// use photon_rs::native::open_image;
/// use photon_rs::Rgb;
///
/// let color = Rgb::new(255u8, 0u8, 0u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// color_horizontal_strips(&mut img, 8u8, color);
/// ```
///
#[wasm_bindgen]
pub fn color_horizontal_strips(
    mut photon_image: &mut PhotonImage,
    num_strips: u8,
    color: Rgb,
) {
    draw_horizontal_strips(&mut photon_image, num_strips, color)
}

fn draw_vertical_strips(mut photon_image: &mut PhotonImage, num_strips: u8, color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    let total_strips = (num_strips * 2) - 1;
    let width_strip = width / total_strips as u32;
    let mut x_pos: u32 = 0;
    for i in 1..num_strips {
        draw_filled_rect_mut(
            &mut img,
            Rect::at((x_pos + width_strip) as i32, 0).of_size(width_strip, height),
            Rgba([color.r, color.g, color.b, 255u8]),
        );
        x_pos = i as u32 * (width_strip * 2);
    }

    let raw_pixels = img.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `num_strips` - The numbder of strips
/// # Example
///
/// ```no_run
/// // For example, to draw vertical strips on a `PhotonImage`:
/// use photon_rs::effects::vertical_strips;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// vertical_strips(&mut img, 8u8);
/// ```
///
#[wasm_bindgen]
pub fn vertical_strips(mut photon_image: &mut PhotonImage, num_strips: u8) {
    let color = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    draw_vertical_strips(&mut photon_image, num_strips, color)
}

/// Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect. Sepcify a color as well.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `num_strips` - The numbder of strips
/// * `color` - Color of strips.
/// # Example
///
/// ```no_run
/// // For example, to draw red vertical strips on a `PhotonImage`:
/// use photon_rs::effects::color_vertical_strips;
/// use photon_rs::native::open_image;
/// use photon_rs::Rgb;
///
/// let color = Rgb::new(255u8, 0u8, 0u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// color_vertical_strips(&mut img, 8u8, color);
/// ```
///
#[wasm_bindgen]
pub fn color_vertical_strips(
    mut photon_image: &mut PhotonImage,
    num_strips: u8,
    color: Rgb,
) {
    draw_vertical_strips(&mut photon_image, num_strips, color)
}

struct Intensity {
    val: i32,
    r: i32,
    g: i32,
    b: i32,
}
/// Turn an image into an oil painting
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `radius` - Radius of each paint particle
/// * `intesnity` - How artsy an Image should be
/// # Example
///
/// ```no_run
/// // For example, to oil an image of type `PhotonImage`:
/// use photon_rs::effects::oil;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// oil(&mut img, 4i32, 55.0);
/// ```
///
#[wasm_bindgen]
pub fn oil(mut photon_image: &mut PhotonImage, radius: i32, intensity: f64) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    let mut target = image::DynamicImage::new_rgba8(width, height);
    let mut pixel_intensity_count: HashMap<usize, Intensity>;
    let mut intensity_lut = vec![vec![0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let single_pix = img.get_pixel(x, y);
            let current_val = single_pix.channels();
            let avg = (current_val[0] as i32
                + current_val[1] as i32
                + current_val[2] as i32) as f64
                / 3.0;
            let val = (avg * intensity) / 255.0;
            intensity_lut[y as usize][x as usize] = val.round() as usize;
        }
    }

    for y in 0..height {
        for x in 0..width {
            pixel_intensity_count = HashMap::new();

            for yy in -radius..=radius {
                let yyy = (y as i32) + yy;
                for xx in -radius..=radius {
                    let xxx = (x as i32) + xx;
                    if yyy > 0
                        && yyy < (height as i32)
                        && xxx > 0
                        && xxx < (width as i32)
                    {
                        let idx_x = xxx as usize;
                        let idx_y = yyy as usize;
                        let intensity_val = intensity_lut[idx_y][idx_x];
                        let single_pix = img.get_pixel(idx_x as u32, idx_y as u32);
                        let pix = single_pix.channels();
                        match pixel_intensity_count.get_mut(&(intensity_val as usize)) {
                            Some(val) => {
                                val.val += 1;
                                val.r += pix[0] as i32;
                                val.g += pix[1] as i32;
                                val.b += pix[2] as i32;
                            }
                            None => {
                                pixel_intensity_count.insert(
                                    intensity_val as usize,
                                    Intensity {
                                        val: 1,
                                        r: pix[0] as i32,
                                        g: pix[1] as i32,
                                        b: pix[2] as i32,
                                    },
                                );
                            }
                        }
                    }
                }
            }

            let mut map_vec: Vec<_> = pixel_intensity_count.iter().collect();
            map_vec.sort_by(|a, b| (b.1.val - a.1.val).cmp(&0));

            let cur_max = map_vec[0].1;
            target.put_pixel(
                x,
                y,
                Rgba::<u8>([
                    (cur_max.r / cur_max.val) as u8,
                    (cur_max.g / cur_max.val) as u8,
                    (cur_max.b / cur_max.val) as u8,
                    255,
                ]),
            )
        }
    }
    let raw_pixels = target.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}
/// Turn an image into an frosted glass see through
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// // For example, to turn an image of type `PhotonImage` into frosted glass see through:
/// use photon_rs::effects::frosted_glass;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// frosted_glass(&mut img);
/// ```
///
#[wasm_bindgen]
pub fn frosted_glass(photon_image: &mut PhotonImage) {
    let img_orig_buf = photon_image.get_raw_pixels();
    let width = photon_image.get_width();
    let height = photon_image.get_height();
    let end = img_orig_buf.len();

    let mut img_buf = Vec::<u8>::new();
    Vec::resize(&mut img_buf, end, 0_u8);

    let perlin = PerlinNoise2D::new(2, 10.0, 10.0, 10.0, 1.0, (100.0, 100.0), 0.5, 101);

    for pixel in (0..end).step_by(4) {
        let x = (pixel / 4) / width as usize;
        let y = (pixel / 4) % width as usize;

        let res = [
            perlin.get_noise(x as f64, y as f64) - 0.5,
            (perlin.get_noise(100.0 + x as f64, y as f64) - 0.5) * 4.0,
        ];

        let x_new = f64::clamp(f64::floor(x as f64 + res[0]), 0.0, height as f64 - 1.0);
        let x_new = x_new as usize;
        let y_new = f64::clamp(f64::floor(y as f64 + res[1]), 0.0, width as f64 - 1.0);
        let y_new = y_new as usize;

        let pixel_new = (x_new * width as usize + y_new) * 4;
        if pixel_new > end as usize {
            continue;
        }
        img_buf[pixel as usize] = img_orig_buf[pixel_new as usize];
        img_buf[pixel as usize + 1] = img_orig_buf[pixel_new as usize + 1];
        img_buf[pixel as usize + 2] = img_orig_buf[pixel_new as usize + 2];
        img_buf[pixel as usize + 3] = img_orig_buf[pixel_new as usize + 3];
    }

    photon_image.raw_pixels = img_buf;
}
// pub fn create_gradient_map(color_a : Rgb, color_b: Rgb) -> Vec<Rgb> {
//     println!("hi");
//     println!("{}", color_a.get_red());
//     let mut gradient_map = vec![];

//     let max_val = 255;
//     let mut r_val = 0;

//     let end: i32 = 256 * 4;

//     for i in (0..end).step_by(4){
//         let i: u8 = i as u8;
//         let intensity_b = max_val - i;

//         let res1 = (i * color_a.get_red() + intensity_b * color_b.get_red());
//         let res2 = res1 / max_val;
//         println!("res 1 {}", res1);
//         println!("res 2 {}", res2);

//         r_val = (i * color_a.get_red() + intensity_b * color_b.get_red()) / max_val;
//         println!("r_val {}", r_val);
//         gradient_map.push(Rgb {
//             r: (256 - (i / 4) * color_a.get_red() + (i / 4) * color_b.r) / 256 ,
//             g: (i * color_a.get_green() + intensity_b * color_b.get_green()) / max_val ,
//             b: (i * color_a.get_blue() + intensity_b * color_b.get_blue()) / max_val
//         });

//     }
//     println!("{:?}", gradient_map);

//     gradient_map
// }

// pub fn duotone(mut img: DynamicImage, color_a : Rgb, color_b : Rgb) -> DynamicImage {
//     let (width, height) = img.dimensions();
//     let gradient_map = create_gradient_map(color_a, color_b);
//     println!("entering for loop");

//     for x in 0..width {
//         for y in 0..height {

//             let mut px = img.get_pixel(x, y);

//             let r = px.data[0];
//             let g = px.data[1];
//             let b = px.data[2];

//             px.data[0] = gradient_map[r as usize].r as u8;
//             px.data[1] = gradient_map[g as usize].g as u8;
//             px.data[2] = gradient_map[b as usize].b as u8;

//             img.put_pixel(x, y, px);
//         }
//     }
//     img
// }
