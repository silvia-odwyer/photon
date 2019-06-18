//! Native-only functions. 
//! Includes functions that open images from the file-system, etc.,

extern crate image;
extern crate rand;
use image::{GenericImageView, ImageBuffer};
// use wasm_bindgen::prelude::*;
use crate::{PhotonImage};

/// Open an image at a given path from the filesystem.
/// A PhotonImage is returned.
/// # Arguments
/// * `img_path` - Path to the image you wish to edit.
/// 
/// # Example
/// ```
/// // For example:
/// use photon::native::open_image;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// // ... image editing functionality here ...
/// ```
pub fn open_image(img_path: &'static str) -> PhotonImage {
    let img = image::open(img_path).unwrap();

    let (width, height) = img.dimensions();

    // Convert the DynamicImage type to raw vec representing RGBA pixels (not RGB)
    let raw_pixels = img.to_rgba().to_vec();

    let photon_image: PhotonImage = PhotonImage {raw_pixels: raw_pixels, width: width, height: height};
    return photon_image;
}

/// Save the image to the filesystem at a given path.
/// # Arguments
/// * img: The PhotonImage you wish to save.
/// * `img_path` - Path for the outputted image.
/// 
/// # Example
/// ```
/// // For example:
/// use photon::native::save_image;
/// 
/// // Save the image at the given path.
/// save_image(img, "images/flowers.PNG");
/// 
/// ```
pub fn save_image(img: PhotonImage, img_path: &str) {
    let raw_pixels = img.raw_pixels;
    let width = img.width;
    let height = img.height;

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = image::ImageRgba8(img_buffer);
    
    dynimage.save(img_path).unwrap();
}