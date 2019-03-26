//! An image processing crate that provides a set of functions for image filtering, convolution, colour manipulation, and more.
//! ## Example 
//! ```rust
//! extern crate photon;
//! extern crate image;

//! fn main() {
//!     let img = image::open("background3.JPG").unwrap();
    
//!     let filtered_img = photon::effects::threshold(img);
    
//!     // Write the contents of this image in PNG format.
//!     filtered_img.save("test.png").unwrap();
//! }
//! ```
//! 
//! This crate contains built-in preset functions, which provide default image processing functionality, as well as functions
//! that allow for direct, low-level access to channel manipulation.

use image::{GenericImage, DynamicImage, GenericImageView};

pub mod filters;
pub mod channels;
pub mod noise;
pub mod effects;
pub mod conv;
pub mod monochrome;
pub mod helpers;

/// Threshold an image using a standard thresholding algorithm.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `threshold` - The amount the image should be thresholded by.
/// # Example
///
/// ```
/// // For example, to threshold an image of type `DynamicImage`:
/// use photon::channels;
/// photon::channels::threshold(img);
/// ```
pub fn threshold(mut img: DynamicImage, threshold: u32) -> DynamicImage {
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
    return img;
}

/// Tint an image by adding an offset to averaged RGB channel values.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `r_offset` - The amount the  R channel should be incremented by.
/// * `g_offset` - The amount the G channel should be incremented by.
/// * `b_offset` - The amount the B channel should be incremented by.
/// # Example
///
/// ```
/// // For example, to tint an image of type `DynamicImage`:
/// photon::tint(img, 10, 20, 15);
/// ```
/// 
pub fn tint(mut img: DynamicImage, r_offset: u32, g_offset: u32, b_offset: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            px.data[0] = if r_val as u32 + r_offset < 255 { r_val as u8 + r_offset as u8} else { 255 };
            px.data[1] = if g_val as u32 + g_offset < 255 { g_val as u8 + g_offset as u8} else { 255 };
            px.data[2] = if b_val as u32 + b_offset < 255 { b_val as u8 + b_offset as u8} else { 255 };

            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Increase the brightness of an image by a factor.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `brightness` - A u8 to add to the brightness.
/// # Example
///
/// ```
/// photon::channels::g_grayscale(img);
/// ```
pub fn inc_brightness(mut img: DynamicImage, brightness: u8) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if px.data[0] <= 255 - brightness {
                px.data[0] += brightness;
            }
            else {
                px.data[0] = 255;
            }            
            if px.data[1] <= 255 - brightness {
                px.data[1] += brightness;
            }

            else {
                px.data[1] = 255
            }

            if px.data[2] <= 255 - brightness {
                px.data[2] += brightness;
            }

            else {
                px.data[2] = 255
            }

            img.put_pixel(x, y, px);
        }
    }
    return img;
}