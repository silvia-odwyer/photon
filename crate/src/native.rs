/// Native-only functions. 
/// Includes functions that open images from the file-system, etc.,

extern crate image;
extern crate rand;
use image::{GenericImage, GenericImageView, ImageBuffer};
use rand::Rng;
use image::{Pixel};
use wasm_bindgen::prelude::*;
use crate::{PhotonImage};
use crate::{helpers};

/// Open an image at a given path from the filesystem.
pub fn open_image(img_path: &'static str) -> PhotonImage {
    let img = image::open(img_path).unwrap();

    let (width, height) = img.dimensions();

    // Convert the DynamicImage type to raw vec representing RGBA pixels (not RGB)
    let raw_pixels = img.to_rgba().to_vec();

    let photon_image: PhotonImage = PhotonImage {raw_pixels: raw_pixels, width: width, height: height};
    return photon_image;
}

/// Save the image to the file-system at a given pathname.
pub fn save_image(img: PhotonImage, filtered_img_path: &str) {
    let raw_pixels = img.raw_pixels;
    let width = img.width;
    let height = img.height;

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = image::ImageRgba8(img_buffer);
    
    dynimage.save(filtered_img_path).unwrap();
}