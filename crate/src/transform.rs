//! Image transformations, ie: scale, crop, resize, etc.,

extern crate image;
use image::{GenericImageView, ImageBuffer};
extern crate wasm_bindgen;
use crate::helpers;
use crate::{PhotonImage};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use image::RgbaImage;
// use std::f64::consts::PI;
// use std::f64;
use web_sys::{ImageData, HtmlCanvasElement};
use wasm_bindgen::Clamped;

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
            let px = img.get_pixel(x, y);
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

#[wasm_bindgen]
pub enum SamplingFilter {
    Nearest = 1,
    Triangle = 2,
    CatmullRom = 3,
    Gaussian = 4,
    Lanczos3 = 5,
}

fn filter_type_from_sampling_filter(sampling_filter: SamplingFilter) -> image::FilterType {
    match sampling_filter {
        SamplingFilter::Nearest => image::FilterType::Nearest,
        SamplingFilter::Triangle => image::FilterType::Triangle,
        SamplingFilter::CatmullRom => image::FilterType::CatmullRom,
        SamplingFilter::Gaussian => image::FilterType::Gaussian,
        SamplingFilter::Lanczos3 => image::FilterType::Lanczos3
    }
}

/// Resize an image on the web.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `width` - New width.
/// * `height` - New height.
/// * `sampling_filter` - Nearest = 1, Triangle = 2, CatmullRom = 3, Gaussian = 4, Lanczos3 = 5
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn resize(photon_img: &PhotonImage, width: u32, height: u32, sampling_filter: SamplingFilter) -> HtmlCanvasElement {
    let sampling_filter = filter_type_from_sampling_filter(sampling_filter);
    let dyn_img = helpers::dyn_image_from_raw(&photon_img);
    let resized_img = image::ImageRgba8(image::imageops::resize(&dyn_img, width, height, sampling_filter));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    canvas.set_width(resized_img.width());
    canvas.set_height(resized_img.height());

    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut resized_img.raw_pixels()), canvas.width(), canvas.height());

    let ctx = canvas
    .get_context("2d").unwrap()
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    // Place the new imagedata onto the canvas
    ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0);

    return canvas;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn resize(photon_img: &PhotonImage, width: u32, height: u32, sampling_filter: SamplingFilter) -> PhotonImage {
    let sampling_filter = filter_type_from_sampling_filter(sampling_filter);

    let dyn_img = helpers::dyn_image_from_raw(&photon_img);
    let resized_img = image::ImageRgba8(image::imageops::resize(&dyn_img, width, height, sampling_filter));

    return PhotonImage{ raw_pixels: resized_img.raw_pixels(), width: resized_img.width(), height: resized_img.height()}
}
