//! Draw text onto an image.
//! For extended graphic design/text-drawing functionality, see [GDL](https://github.com/silvia-odwyer/gdl),
//! which is a graphic design library, compatible with Photon.

use crate::iter::ImageIterator;
use crate::{helpers, PhotonImage};
use image::{DynamicImage, Rgba};
use imageproc::distance_transform::Norm;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use rusttype::{Font, Scale};

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
/// draw_text_with_border(&mut img, "Welcome to Photon!", 10_i32, 10_i32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text_with_border(photon_img: &mut PhotonImage, text: &str, x: i32, y: i32) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let mut image2: DynamicImage =
        DynamicImage::new_luma8(image.width(), image.height());

    let font = Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]);
    let font = Font::try_from_bytes(&font).unwrap();
    let height = 90f32;
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };
    draw_text_mut(
        &mut image2,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );

    let mut image2 = image2.to_luma8();
    dilate_mut(&mut image2, Norm::LInf, 4u8);

    // Add a border to the text.
    for (x, y) in ImageIterator::with_dimension(&image2.dimensions()) {
        let pixval = 255 - image2.get_pixel(x, y)[0];
        if pixval != 255 {
            let new_pix = Rgba([pixval, pixval, pixval, 255]);
            image.put_pixel(x, y, new_pix);
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
    let dynimage = image::DynamicImage::ImageRgba8(image);
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
/// draw_text(&mut img, "Welcome to Photon!", 10_i32, 10_i32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn draw_text(photon_img: &mut PhotonImage, text: &str, x: i32, y: i32) {
    let mut image = helpers::dyn_image_from_raw(photon_img).to_rgba8();

    let mut image2: DynamicImage =
        DynamicImage::new_luma8(image.width(), image.height());

    let font = Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]);
    let font = Font::try_from_bytes(&font).unwrap();
    let height = 90f32;
    let scale = Scale {
        x: height * 1.0,
        y: height,
    };
    draw_text_mut(
        &mut image2,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        10,
        10,
        scale,
        &font,
        text,
    );

    let mut image2 = image2.to_luma8();
    dilate_mut(&mut image2, Norm::LInf, 4u8);

    draw_text_mut(
        &mut image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        x,
        y,
        scale,
        &font,
        text,
    );
    let dynimage = image::DynamicImage::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.into_bytes();
}
