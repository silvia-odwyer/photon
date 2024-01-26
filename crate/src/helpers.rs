//! Helper functions for converting between various formats

use crate::{PhotonImage, Rgb};
use image::DynamicImage::ImageRgba8;
use image::{DynamicImage, ImageBuffer};

#[cfg(feature = "enable_wasm")]
extern crate wasm_bindgen;

/// Gets the square distance between two colours
pub fn square_distance(color1: Rgb, color2: Rgb) -> i32 {
    let (r1, g1, b1) = (color1.r as i32, color1.g as i32, color1.b as i32);
    let (r2, g2, b2) = (color2.r as i32, color2.g as i32, color2.b as i32);
    i32::pow(r1 - r2, 2) + i32::pow(g1 - g2, 2) + i32::pow(b1 - b2, 2)
}

// Read a DynamicImage from a given path.
pub fn open_dyn_image(img_path: &'static str) -> DynamicImage {
    image::open(img_path).unwrap()
}

/// Save a DynamicImage to a path.
pub fn save_dyn_image(img: DynamicImage, filtered_img_path: &str) {
    // let raw_pixels = img.raw_pixels;
    // let width = img.width;
    // let height = img.height;

    // let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    // let dynimage = image::ImageRgba8(img_buffer);

    img.save(filtered_img_path).unwrap();
}

/// Get raw pixels (as a vec of u8s) from a DynamicImage
pub fn get_pixels(img: DynamicImage) -> Vec<u8> {
    // get an image's raw pixels, and return as a vec of u8s
    img.into_bytes()
}

/// Convert a PhotonImage to a DynamicImage type (struct used by the `image` crate)
pub fn dyn_image_from_raw(photon_image: &PhotonImage) -> DynamicImage {
    // convert a vec of raw pixels (as u8s) to a DynamicImage type
    let _len_vec = photon_image.raw_pixels.len() as u128;
    let raw_pixels = &photon_image.raw_pixels;
    let img_buffer = ImageBuffer::from_vec(
        photon_image.width,
        photon_image.height,
        raw_pixels.to_vec(),
    )
    .unwrap();
    ImageRgba8(img_buffer)
}
