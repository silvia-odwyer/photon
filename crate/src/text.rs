//! Draw text onto an image.
//! For extended graphic design/text-drawing functionality, see [GDL](https://github.com/silvia-odwyer/gdl),
//! which is a graphic design library, compatible with Photon.

use crate::iter::ImageIterator;
use crate::{helpers, PhotonImage};
use image::{DynamicImage, GrayImage, Luma, Rgba};
use imageproc::distance_transform::Norm;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use ab_glyph::{FontArc, PxScale};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// Add bordered-text to an image.
/// The only font available as of now is Roboto.
/// Note: A graphic design/text-drawing library is currently being developed, so stay tuned.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
///
/// # Example
///
/// ```no_run
/// // For example to draw the string "Welcome to Photon!" at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text_with_border;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// draw_text_with_border(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_border(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let mut mask: GrayImage = GrayImage::new(image.width(), image.height());
    let font_bytes: &[u8] = include_bytes!("../fonts/Roboto-Regular.ttf");
    let font = FontArc::try_from_slice(font_bytes).expect("invalid font bytes");

    let scale: PxScale = PxScale::from(font_size);
    draw_text_mut(&mut mask, Luma([255u8]), x, y, scale, &font, text);
    dilate_mut(&mut mask, Norm::LInf, 4u8);

    for (px, py) in ImageIterator::with_dimension(&mask.dimensions()) {
        let v = mask.get_pixel(px, py)[0];
        let pixval = 255u8 - v;
        if pixval != 255 {
            image.put_pixel(px, py, Rgba([pixval, pixval, pixval, 255]));
        }
    }

    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x + 10,
        y - 10,
        scale,
        &font,
        text,
    );

    let dynimage = DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}

/// Add text to an image.
/// The only font available as of now is Roboto.
/// Note: A graphic design/text-drawing library is currently being developed, so stay tuned.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `text` - Text string to be drawn to the image.
/// * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
/// * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
/// * `font_size` - Font size in pixels of the text to be drawn.
///
/// # Example
///
/// ```no_run
/// // For example to draw the string "Welcome to Photon!" at 10, 10:
/// use photon_rs::native::open_image;
/// use photon_rs::text::draw_text;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// draw_text(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text(
    photon_img: &mut PhotonImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let font_bytes: &[u8] = include_bytes!("../fonts/Roboto-Regular.ttf");
    let font = FontArc::try_from_slice(font_bytes).expect("invalid font bytes");

    let scale: PxScale = PxScale::from(font_size);

    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );

    let dynimage = DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}
