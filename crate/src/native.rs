//! Native-only functions.
//! Includes functions that open images from the file-system, etc.,

use image::DynamicImage::ImageRgba8;
use image::{GenericImageView, ImageBuffer};
use std::io;
use std::path::Path;
// use wasm_bindgen::prelude::*;
use crate::PhotonImage;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ImageError(#[from] image::ImageError),

    #[error(transparent)]
    IoError(#[from] io::Error),
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
pub fn open_image<P>(img_path: P) -> Result<PhotonImage, Error>
where
    P: AsRef<Path>,
{
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

/// Saves a image from a byte slice
/// A PhotonImage is returned.
/// # Arguments
/// * `buffer` - A byte slice containing the image you want to edit.
///
/// # Example
/// ```no_run
/// use photon_rs::native::open_image_from_bytes;
///
/// // Code to read a file to buffer. If you are reading from a file its better to use `open_image`
/// let buffer = std::fs::read("img.jpg").expect("File Should Open");
///
/// // Open the image. A PhotonImage is returned.
/// let img = open_image_from_bytes(buffer.as_slice()).expect("Buffer should be valid");
///
/// // ... image editing functionality here ...
/// ```
pub fn open_image_from_bytes(buffer: &[u8]) -> Result<PhotonImage, Error> {
    let img = image::load_from_memory(buffer)?;
    let (width, height) = img.dimensions();
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
/// use photon_rs::native::{open_image, save_image};
///
/// let img = open_image("img.jpg").expect("File should open");
/// // Save the image at the given path.
/// save_image(img,"manipulated_image.jpg").expect("Save failed");
/// ```
pub fn save_image<P>(img: PhotonImage, img_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let raw_pixels = img.raw_pixels;
    let width = img.width;
    let height = img.height;

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = ImageRgba8(img_buffer);

    dynimage.save(img_path)?;
    Ok(())
}

/// Save the image to a vector of bytes
/// # Arguments
/// * img: The PhotonImage you wish to save.
///
/// # Example
/// ```no_run
/// use photon_rs::native::{open_image,image_to_bytes};
///
/// let img = open_image("img.jpg").expect("File should open");
/// // Save the image at a vec<u8>
/// let byt = image_to_bytes(img);
/// ```
pub fn image_to_bytes(img: PhotonImage) -> Vec<u8> {
    let raw_pixels = img.raw_pixels;
    let width = img.width;
    let height = img.height;

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = ImageRgba8(img_buffer);
    dynimage.into_bytes()
}
