//! Monochrome-related effects and greyscaling/duotoning.

extern crate image;
use image::{GenericImage, GenericImageView};
use crate::{PhotonImage};
use crate::helpers;
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
/// ```
/// // For example, to apply a monochrome effect to an image:
/// use photon::monochrome;
/// monochrome::monochroma(&mut img, 40, 50, 100);
/// ```
/// 
#[wasm_bindgen]
pub fn monochrome(mut photon_image: &mut PhotonImage, r_offset: u32, g_offset: u32, b_offset: u32) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            let mut avg = (r_val + g_val + b_val) / 3;
            if avg >= 255 {
                avg = 255
            }
            
            px.data[0] = if avg as u32 + r_offset < 255 { avg as u8 + r_offset as u8} else { 255 };
            px.data[1] = if avg as u32 + g_offset < 255 { avg as u8 + g_offset as u8} else { 255 };
            px.data[2] = if avg as u32 + b_offset < 255 { avg as u8 + b_offset as u8} else { 255 };

            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Convert an image to sepia.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// # Example
///
/// ```
/// // For example, to tint an image of type `PhotonImage`:
/// use photon::monochrome;
/// monochrome::sepia(&mut img);
/// ```
/// 
#[wasm_bindgen]
pub fn sepia(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as f32, px.data[1] as f32, px.data[2] as f32);
            let avg = 0.3 * r_val + 0.59 * g_val + 0.11 * b_val;

            px.data[0] = if avg as u32 + 100 < 255 { avg as u8 + 100} else { 255 };
            px.data[1] = if avg as u32 + 50 < 255 { avg as u8 + 50 } else { 255 };
      
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Convert an image to grayscale using the conventional averaging algorithm.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```
/// // For example, to convert an image of type `PhotonImage` to greyscale:
/// use photon::monochrome;
/// monochrome::grayscale(&mut img);
/// ```
#[wasm_bindgen]
pub fn grayscale(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);    
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            let mut avg = (r_val + g_val + b_val) / 3;
            if avg >= 255 {
                avg = 255
            }
            px.data[0] = avg as u8;
            px.data[1] = avg as u8;
            px.data[2] = avg as u8;
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Convert an image to grayscale with a human corrected factor, to account for human vision.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```
/// // For example, to convert an image of type `PhotonImage` to greyscale with a human corrected factor:
/// use photon::monochrome;
/// monochrome::grayscale_human_corrected(&mut img);
/// ```
#[wasm_bindgen]
pub fn grayscale_human_corrected(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as f32, px.data[1] as f32, px.data[2] as f32);

            let mut avg = r_val * 0.3 + g_val * 0.59 + b_val * 0.11;
            
            if avg >= 255.0 {
                avg = 255.0
            }
            
            px.data[0] = avg as u8;
            px.data[1] = avg as u8;
            px.data[2] = avg as u8;
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Desaturate an image by getting the min/max of each pixel's RGB values.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```
/// // For example, to desaturate an image:
/// use photon::monochrome;
/// monochrome::desaturate(&mut img);
/// ```
#[wasm_bindgen]
pub fn desaturate(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            // get the max and min vals of a pixel's 3 rgb values by sorting a vec of these
            let mut rgb_vals = vec![r_val, g_val, b_val];
            rgb_vals.sort();

            let mut gray = (rgb_vals[0] + rgb_vals[2]) / 2;

            if gray >= 255 {
                gray = 255
            }
            
            px.data[0] = gray as u8;
            px.data[1] = gray as u8;
            px.data[2] = gray as u8;
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Uses a min. decomposition algorithm to convert an image to greyscale.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```
/// // For example, to decompose an image with min decomposition:
/// monochrome::decompose_min(&mut img);
/// ```
#[wasm_bindgen]
pub fn decompose_min(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            // get the max and min vals of a pixel's 3 rgb values by sorting a vec of these
            let mut rgb_vals = vec![r_val, g_val, b_val];
            rgb_vals.sort();

            let mut gray = rgb_vals[0];

            if gray >= 255 {
                gray = 255
            }
            
            px.data[0] = gray as u8;
            px.data[1] = gray as u8;
            px.data[2] = gray as u8;
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Uses a max. decomposition algorithm to convert an image to greyscale.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```
/// // For example, to decompose an image with max decomposition:
/// monochrome::decompose_max(&mut img);
/// ```
#[wasm_bindgen]
pub fn decompose_max(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            // get the max and min vals of a pixel's 3 rgb values by sorting a vec of these
            let mut rgb_vals = vec![r_val, g_val, b_val];
            rgb_vals.sort();

            let gray = rgb_vals[2];
            
            px.data[0] = gray as u8;
            px.data[1] = gray as u8;
            px.data[2] = gray as u8;
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Employ only a limited number of gray shades in an image.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `num_shades` - The number of grayscale shades to be displayed in the image.

/// # Example
///
/// ```
/// // For example, to limit an image to four shades of gray only:
/// monochrome::grayscale_shades(&mut img, 4);
/// ```
#[wasm_bindgen]
pub fn grayscale_shades(mut photon_image: &mut PhotonImage, num_shades: u8) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);

            let conversion: f32 = 255.0 / (num_shades as f32 - 1.0);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            let avg: f32 = (r_val + g_val + b_val) as f32 / 3.0;
            
            let dividend = avg / conversion as f32;

            let gray = (dividend + 0.5) * conversion;
            let mut px = img.get_pixel(x, y);
            
            px.data[0] = gray as u8;
            px.data[1] = gray as u8;
            px.data[2] = gray as u8;
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to the Red channel's value.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```
/// monochrome::r_grayscale(&mut img);
/// ```
#[wasm_bindgen]
pub fn r_grayscale(photon_image: &mut PhotonImage) {
    return single_channel_grayscale(photon_image, 0)
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to the Green channel's value.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```
/// monochrome::g_grayscale(&mut img);
/// ```
#[wasm_bindgen]
pub fn g_grayscale(photon_image: &mut PhotonImage) {
    return single_channel_grayscale(photon_image, 1)
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to the Blue channel's value.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.

/// # Example
///
/// ```
/// monochrome::b_grayscale(&mut img);
/// ```
#[wasm_bindgen]
pub fn b_grayscale(photon_image: &mut PhotonImage) {
    return single_channel_grayscale(photon_image, 2)
}

/// Convert an image to grayscale by setting a pixel's 3 RGB values to a chosen channel's value.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `channel` - A usize representing the channel from 0 to 2. O represents the Red channel, 1 the Green channel, and 2 the Blue channel. 

/// # Example
/// To grayscale using only values from the Red channel:
/// ```
/// monochrome::single_channel_grayscale(&mut img, 0);
/// ```
#[wasm_bindgen]
pub fn single_channel_grayscale(mut photon_image: &mut PhotonImage, channel: usize) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let channel_data = px.data[channel];
            
            px.data[0] = channel_data as u8;
            px.data[1] = channel_data as u8;
            px.data[2] = channel_data as u8;
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Threshold an image using a standard thresholding algorithm.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `threshold` - The amount the image should be thresholded by from 0 to 255.
/// # Example
///
/// ```
/// // For example, to threshold an image of type `PhotonImage`:
/// use photon::monochrome;
/// monochrome::threshold(&mut img, 30);
/// ```
#[wasm_bindgen]
pub fn threshold(mut photon_image: &mut PhotonImage, threshold: u32) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let r: f32 = px.data[0].into();
            let g: f32 = px.data[1].into();
            let b: f32 = px.data[2].into();

            let mut v = 0.2126 * r + 0.7152 * g + 0.072 * b;

            if v >= threshold as f32 {
                v = 255.0;
            }
            else {
                v = 0.0;
            }
            px.data[0] = v as u8;
            px.data[1] = v as u8;
            px.data[2] = v as u8;

            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}