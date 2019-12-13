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
/// * `x1` - x-coordinate of origin/starting point 
/// * `y1` - y-coordinate of origin/starting point
/// * `x2` - x-coordinate of end point
///  * `y2` - y-coordinate of end point
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
pub fn crop(mut photon_image: &mut PhotonImage, x1: u32, y1: u32, x2: u32, y2: u32) -> PhotonImage {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

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
/// * `x1` - x-coordinate of origin/starting point 
/// * `y1` - y-coordinate of origin/starting point
/// * `x2` - x-coordinate of end point
///  * `y2` - y-coordinate of end point
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
pub fn fliph(mut photon_image: &mut PhotonImage) -> PhotonImage {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    let width = img.width();
    let mut flipped_img: RgbaImage = ImageBuffer::new(width, img.height());

    for x in 0..width {
        for y in 0..img.height() {
            let mut px = img.get_pixel(x, y);
            flipped_img.put_pixel(width - x - 1, y, px);
        }
    }

    let dynimage = image::ImageRgba8(flipped_img);
    let raw_pixels = dynimage.raw_pixels();
    let flipped_photon_img = PhotonImage { raw_pixels: raw_pixels, width: dynimage.width(), height: dynimage.height()};
    return flipped_photon_img
}

/// Flip an image vertically.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `x1` - x-coordinate of origin/starting point 
/// * `y1` - y-coordinate of origin/starting point
/// * `x2` - x-coordinate of end point
///  * `y2` - y-coordinate of end point
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
pub fn flipv(mut photon_image: &mut PhotonImage) -> PhotonImage {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    let width = img.width();
    let height = img.height();

    let mut flipped_img: RgbaImage = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            flipped_img.put_pixel(x, height - y - 1, px);
        }
    }

    let dynimage = image::ImageRgba8(flipped_img);
    let raw_pixels = dynimage.raw_pixels();
    let flipped_photon_img = PhotonImage { raw_pixels: raw_pixels, width: dynimage.width(), height: dynimage.height()};
    return flipped_photon_img
}

// Rotate an image

// from PIL import Image, ImageDraw
// from math import sin, cos, pi


// # Load image:
// input_image = Image.open("input.png")
// input_pixels = input_image.load()

// # Create output image
// output_image = Image.new("RGB", input_image.size)
// draw = ImageDraw.Draw(output_image)

// angle = pi / 3  # angle in radian
// center_x = input_image.width / 2
// center_y = input_image.height / 2

// # Copy pixels
// for x in range(input_image.width):
//     for y in range(input_image.height):
//         # Compute coordinate in input image
//         xp = int((x - center_x) * cos(angle) - (y - center_y) * sin(angle) + center_x)
//         yp = int((x - center_x) * sin(angle) + (y - center_y) * cos(angle) + center_y)
//         if 0 <= xp < input_image.width and 0 <= yp < input_image.height:
//             draw.point((x, y), input_pixels[xp, yp])

// output_image.save("output.png")