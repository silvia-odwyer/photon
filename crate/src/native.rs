//! Native-only functions.
//! Includes functions that open images from the file-system, etc.,

extern crate image;
extern crate rand;
use image::DynamicImage::ImageRgba8;
use image::{GenericImageView, ImageBuffer, ImageError};
use std::io;
// use wasm_bindgen::prelude::*;
use crate::PhotonImage;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct OpenError(#[from] ImageError);

#[derive(Debug, Error)]
pub enum SaveError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("Buffer size is not big enough")]
    BufferSize,
}

/// Open an image at a given path from the filesystem.
/// A PhotonImage is returned.
/// # Arguments
/// * `img_path` - Path to the image you wish to edit.
///
/// # Example
/// ```no_run
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let img = open_image("img.jpg").expect("File should open");
///
/// // ... image editing functionality here ...
/// ```
pub fn open_image(img_path: &str) -> Result<PhotonImage, OpenError> {
    let img = image::open(img_path)?;

    let (width, height) = img.dimensions();

    // Convert the DynamicImage type to raw vec representing RGBA pixels (not RGB)
    let raw_pixels = img.to_rgba8().to_vec();

    Ok(PhotonImage {
        raw_pixels,
        width,
        height,
    })
}

/// Save the image to the filesystem at a given path.
/// # Arguments
/// * img: The PhotonImage you wish to save.
/// * `img_path` - Path for the outputted image.
///
/// # Example
/// ```no_run
/// use photon_rs::native::{open_image};
///
/// let img = open_image("img.jpg").expect("File should open");
/// // Save the image at the given path.
/// ```
pub fn save_image(img: PhotonImage, img_path: &str) {
    let raw_pixels = img.raw_pixels;
    let width = img.width;
    let height = img.height;

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = ImageRgba8(img_buffer);

    dynimage.save(img_path).unwrap();
}
