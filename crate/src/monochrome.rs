//! Monochrome-related effects and greyscaling/duotoning.

use crate::helpers;
use crate::iter::ImageIterator;
use crate::PhotonImage;
use image::Pixel;
use image::{GenericImage, GenericImageView};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// Apply a monochrome effect of a certain colour.
///
/// It does so by averaging the R, G, and B values of a pixel, and then adding a
/// separate value to that averaged value for each channel to produce a tint.
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `r_offset` - The value to add to the Red channel per pixel.
/// * `g_offset` - The value to add to the Green channel per pixel.
/// * `b_offset` - The value to add to the Blue channel per pixel.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a monochrome effect to an image:
/// use photon_rs::monochrome::monochrome;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// monochrome(&mut img, 40_u32, 50_u32, 100_u32);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn monochrome(img: &mut PhotonImage, r_offset: u32, g_offset: u32, b_offset: u32) {
    let end = img.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = img.raw_pixels[i] as u32;
        let g_val = img.raw_pixels[i + 1] as u32;
        let b_val = img.raw_pixels[i + 2] as u32;
        let mut avg: u32 = (r_val + g_val + b_val) / 3;
        if avg >= 255 {
            avg = 255
        }
        let new_r = if avg + r_offset < 255 {
            avg as u8 + r_offset as u8
        } else {
            255
        };
        let new_g = if avg + g_offset < 255 {
            avg as u8 + g_offset as u8
        } else {
            255
        };
        let new_b = if avg + b_offset < 255 {
            avg as u8 + b_offset as u8
        } else {
            255
        };

        img.raw_pixels[i] = new_r;
        img.raw_pixels[i + 1] = new_g;
        img.raw_pixels[i + 1] = new_b;
    }
}

/// Convert an image to sepia.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// # Example
///
/// ```no_run
/// // For example, to sepia an image of type `PhotonImage`:
/// use photon_rs::monochrome::sepia;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// sepia(&mut img);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn sepia(img: &mut PhotonImage) {
    let end = img.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = img.raw_pixels[i] as f32;
        let g_val = img.raw_pixels[i + 1] as f32;
        let b_val = img.raw_pixels[i + 2] as f32;
        let mut avg = 0.3 * r_val + 0.59 * g_val + 0.11 * b_val;
        if avg >= 255.0 {
            avg = 255.0
        }
        let new_r = if avg as u32 + 100 < 255 {
            avg as u8 + 100
        } else {
            255
        };
        let new_g = if avg as u32 + 50 < 255 {
            avg as u8 + 50
        } else {
            255
        };

        img.raw_pixels[i] = new_r;
        img.raw_pixels[i + 1] = new_g;
    }
}

/// Convert an image to grayscale using the conventional averaging algorithm.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```no_run
/// // For example, to convert an image of type `PhotonImage` to grayscale:
/// use photon_rs::monochrome::grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// grayscale(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn grayscale(img: &mut PhotonImage) {
    let end = img.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = img.raw_pixels[i] as u32;
        let g_val = img.raw_pixels[i + 1] as u32;
        let b_val = img.raw_pixels[i + 2] as u32;
        let mut avg: u32 = (r_val + g_val + b_val) / 3;
        if avg >= 255 {
            avg = 255
        }

        img.raw_pixels[i] = avg as u8;
        img.raw_pixels[i + 1] = avg as u8;
        img.raw_pixels[i + 2] = avg as u8;
    }
}

/// Convert an image to grayscale with a human corrected factor, to account for human vision.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```no_run
/// // For example, to convert an image of type `PhotonImage` to grayscale with a human corrected factor:
/// use photon_rs::monochrome::grayscale_human_corrected;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// grayscale_human_corrected(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn grayscale_human_corrected(img: &mut PhotonImage) {
    let end = img.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = img.raw_pixels[i] as f32;
        let g_val = img.raw_pixels[i + 1] as f32;
        let b_val = img.raw_pixels[i + 2] as f32;

        let avg: u8 = (r_val * 0.3 + g_val * 0.59 + b_val * 0.11) as u8;

        img.raw_pixels[i] = avg;
        img.raw_pixels[i + 1] = avg;
        img.raw_pixels[i + 2] = avg;
    }
}

/// Desaturate an image by getting the min/max of each pixel's RGB values.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```no_run
/// // For example, to desaturate an image:
/// use photon_rs::monochrome::desaturate;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// desaturate(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn desaturate(img: &mut PhotonImage) {
    let end = img.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = img.raw_pixels[i] as u32;
        let g_val = img.raw_pixels[i + 1] as u32;
        let b_val = img.raw_pixels[i + 2] as u32;

        // get the max and min vals of a pixel's 3 rgb values by sorting a vec of these
        let mut rgb_vals = [r_val, g_val, b_val];
        rgb_vals.sort_unstable();

        let gray: u8 = ((rgb_vals[0] + rgb_vals[2]) / 2) as u8;

        img.raw_pixels[i] = gray;
        img.raw_pixels[i + 1] = gray;
        img.raw_pixels[i + 2] = gray;
    }
}

/// Uses a min. decomposition algorithm to convert an image to greyscale.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```no_run
/// // For example, to decompose an image with min decomposition:
/// use photon_rs::monochrome::decompose_min;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// decompose_min(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn decompose_min(img: &mut PhotonImage) {
    let end = img.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = img.raw_pixels[i] as u32;
        let g_val = img.raw_pixels[i + 1] as u32;
        let b_val = img.raw_pixels[i + 2] as u32;

        // get the max and min vals of a pixel's 3 rgb values by sorting a vec of these
        let mut rgb_vals = [r_val, g_val, b_val];
        rgb_vals.sort_unstable();

        let gray: u8 = rgb_vals[0] as u8;

        img.raw_pixels[i] = gray;
        img.raw_pixels[i + 1] = gray;
        img.raw_pixels[i + 2] = gray;
    }
}

/// Uses a max. decomposition algorithm to convert an image to greyscale.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```no_run
/// // For example, to decompose an image with max decomposition:
/// use photon_rs::monochrome::decompose_max;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// decompose_max(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn decompose_max(img: &mut PhotonImage) {
    let end = img.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = img.raw_pixels[i] as u32;
        let g_val = img.raw_pixels[i + 1] as u32;
        let b_val = img.raw_pixels[i + 2] as u32;

        // get the max and min vals of a pixel's 3 rgb values by sorting a vec of these
        let mut rgb_vals = [r_val, g_val, b_val];
        rgb_vals.sort_unstable();

        let gray: u8 = rgb_vals[2] as u8;

        img.raw_pixels[i] = gray;
        img.raw_pixels[i + 1] = gray;
        img.raw_pixels[i + 2] = gray;
    }
}

/// Employ only a limited number of gray shades in an image.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `num_shades` - The number of grayscale shades to be displayed in the image.

/// # Example
///
/// ```no_run
/// // For example, to limit an image to four shades of gray only:
/// use photon_rs::monochrome::grayscale_shades;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// grayscale_shades(&mut img, 4_u8);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn grayscale_shades(photon_image: &mut PhotonImage, num_shades: u8) {
    let mut img = helpers::dyn_image_from_raw(photon_image);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let px = img.get_pixel(x, y);

        let conversion: f32 = 255.0 / (num_shades as f32 - 1.0);
        let channels = px.channels();
        let (r_val, g_val, b_val) =
            (channels[0] as u32, channels[1] as u32, channels[2] as u32);

        let avg: f32 = (r_val + g_val + b_val) as f32 / 3.0;

        let dividend = avg / conversion;

        let gray = ((dividend + 0.5) * conversion) as u8;

        img.put_pixel(x, y, image::Rgba([gray, gray, gray, 255]));
    }
    let raw_pixels = img.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to the Red channel's value.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```no_run
/// use photon_rs::monochrome::r_grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// r_grayscale(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn r_grayscale(photon_image: &mut PhotonImage) {
    single_channel_grayscale(photon_image, 0)
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to the Green channel's value.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```no_run
/// use photon_rs::monochrome::g_grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// g_grayscale(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn g_grayscale(photon_image: &mut PhotonImage) {
    single_channel_grayscale(photon_image, 1)
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to the Blue channel's value.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```no_run
/// use photon_rs::monochrome::b_grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// b_grayscale(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn b_grayscale(photon_image: &mut PhotonImage) {
    single_channel_grayscale(photon_image, 2)
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to a chosen channel's value.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `channel` - A usize representing the channel from 0 to 2. O represents the Red channel, 1 the Green channel, and 2 the Blue channel.

/// # Example
/// To grayscale using only values from the Red channel:
/// ```no_run
/// use photon_rs::monochrome::single_channel_grayscale;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// single_channel_grayscale(&mut img, 0_usize);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn single_channel_grayscale(photon_image: &mut PhotonImage, channel: usize) {
    let mut img = helpers::dyn_image_from_raw(photon_image);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let px = img.get_pixel(x, y);

        let channels = px.channels();
        let channel_data = channels[channel];

        img.put_pixel(
            x,
            y,
            image::Rgba([channel_data, channel_data, channel_data, 255]),
        );
    }
    let raw_pixels = img.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Threshold an image using a standard thresholding algorithm.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `threshold` - The amount the image should be thresholded by from 0 to 255.
/// # Example
///
/// ```no_run
/// // For example, to threshold an image of type `PhotonImage`:
/// use photon_rs::monochrome::threshold;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// threshold(&mut img, 30_u32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn threshold(img: &mut PhotonImage, threshold: u32) {
    let end = img.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r = img.raw_pixels[i] as f32;
        let g = img.raw_pixels[i + 1] as f32;
        let b = img.raw_pixels[i + 2] as f32;

        let mut v = 0.2126 * r + 0.7152 * g + 0.072 * b;

        if v >= threshold as f32 {
            v = 255.0;
        } else {
            v = 0.0;
        }

        img.raw_pixels[i] = v as u8;
        img.raw_pixels[i + 1] = v as u8;
        img.raw_pixels[i + 2] = v as u8;
    }
}
