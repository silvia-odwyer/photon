//! Image transformations, ie: scale, crop, resize, etc.,

extern crate image;
use image::{GenericImageView, ImageBuffer};
extern crate wasm_bindgen;
use crate::helpers;
use crate::{PhotonImage};
use wasm_bindgen::prelude::*;
use image::RgbaImage;

/// Crop an image.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// 
/// ## Example
///
/// ```
/// // For example, to crop an image at (0, 0) to (500, 800)
/// use photon::transform;
/// let img = photon::open_image("img.jpg");
/// let cropped_img = photon::transform::crop(&mut img, 0, 0, 500, 800);
/// // Write the contents of this image in JPG format.
/// photon::helpers::save_image(cropped_img, "cropped_image.png");
/// ```
#[wasm_bindgen]
pub fn crop(photon_image: &mut PhotonImage, x1: u32, y1: u32, x2: u32, y2: u32) -> PhotonImage {
    let img = helpers::dyn_image_from_raw(&photon_image);

    let mut cropped_img: RgbaImage = ImageBuffer::new(x2 - x1, y2 - y1);

    for x in 0..cropped_img.width() {
        for y in 0..cropped_img.height() {
            let mut px = img.get_pixel(x, y);
            cropped_img.put_pixel(x, y, px);

        }
    }
    let dynimage = image::ImageRgba8(cropped_img);
    let raw_pixels = dynimage.raw_pixels();
    let cropped_photon_img = PhotonImage { raw_pixels: raw_pixels, width: dynimage.width(), height: dynimage.height()};
    return cropped_photon_img
}

/// Flip an image horizontally.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// 
/// ## Example
///
/// ```
/// // For example, to flip an image horizontally:
/// use photon::transform;
/// let img = photon::open_image("img.jpg");
/// let new_img = photon::transform::fliph(&mut img);
/// // Write the contents of this image in JPG format.
/// photon::helpers::save_image(new_img, "new_image.png");
/// ```
#[wasm_bindgen]
pub fn fliph(photon_image: &mut PhotonImage) {
    let img = helpers::dyn_image_from_raw(&photon_image);

    let width = img.width();
    let mut flipped_img: RgbaImage = ImageBuffer::new(width, img.height());

    for x in 0..width {
        for y in 0..img.height() {
            let px = img.get_pixel(x, y);
            flipped_img.put_pixel(width - x - 1, y, px);
        }
    }

    let dynimage = image::ImageRgba8(flipped_img);
    let raw_pixels = dynimage.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Flip an image vertically.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// 
/// ## Example
///
/// ```
/// // For example, to flip an image vertically:
/// use photon::transform;
/// let img = photon::open_image("img.jpg");
/// let new_img = photon::transform::flipv(&mut img);
/// // Write the contents of this image in JPG format.
/// photon::helpers::save_image(new_img, "new_image.png");
/// ```
#[wasm_bindgen]
pub fn flipv(photon_image: &mut PhotonImage) {
    let img = helpers::dyn_image_from_raw(&photon_image);

    let width = img.width();
    let height = img.height();

    let mut flipped_img: RgbaImage = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);
            flipped_img.put_pixel(x, height - y - 1, px);
        }
    }

    let dynimage = image::ImageRgba8(flipped_img);
    let raw_pixels = dynimage.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Resize an image on the web.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `width` - New width.
/// * `height` - New height.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn resize(photon_img: &PhotonImage, width: u32, height: u32) -> PhotonImage {
    let sampling_filter = image::FilterType::Nearest;

    let dyn_img = helpers::dyn_image_from_raw(&photon_img);
    let resized_img = image::ImageRgba8(image::imageops::resize(&dyn_img, width, height, sampling_filter));

    return PhotonImage{ raw_pixels: resized_img.raw_pixels(), width: resized_img.width(), height: resized_img.height()}
}
