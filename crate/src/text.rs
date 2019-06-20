//! Draw text to an image.
//! For extended graphic design/text-drawing functionality, a new library is being developed currently.

extern crate image;
use image::{Rgba, DynamicImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use imageproc::distance_transform::Norm;
use rusttype::{FontCollection, Scale};
use crate::{PhotonImage, helpers};

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
/// ```
/// // For example to draw the string "Welcome to Photon!" at 10, 10:
/// use photon::text::draw_text_with_border;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/cats.PNG");
/// 
/// draw_text_with_border(&mut img, "Welcome to Photon!", 10, 10);
/// ```
#[wasm_bindgen]
pub fn draw_text_with_border(mut photon_img: &mut PhotonImage, text: &str, x: u32, y: u32) {

    let mut image = helpers::dyn_image_from_raw(&photon_img).to_rgba();

    let mut image2 : DynamicImage = DynamicImage::new_luma8(
        image.width(), image.height());

    let font = Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    let height = 90f32;
    let scale = Scale { x: height * 1.0, y: height };
    draw_text_mut(&mut image2, Rgba([255u8, 255u8, 255u8, 255u8]), x, y, scale, &font, text);

    let mut image2 = image2.to_luma();
    dilate_mut(&mut image2, Norm::LInf, 4u8);

    // Add a border to the text.
    for x in 0..image2.width() {
        for y in 0..image2.height() {
            let pixval = 255 - image2.get_pixel(x, y).data[0];
            if pixval != 255 {
                let new_pix = Rgba([pixval, pixval, pixval, 255]);
                image.put_pixel(x, y, new_pix);
            }
        }
    }

    draw_text_mut(&mut image, Rgba([255u8, 255u8, 255u8, 255u8]), x + 10, y - 10, scale, &font, text);
    let dynimage = image::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.raw_pixels();
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
/// ```
/// // For example to draw the string "Welcome to Photon!" at 10, 10:
/// use photon::text::draw_text;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/cats.PNG");
/// 
/// draw_text(&mut img, "Welcome to Photon!", 10, 10);
/// ```
#[wasm_bindgen]
pub fn draw_text(mut photon_img: &mut PhotonImage, text: &str, x: u32, y: u32) {

    let mut image = helpers::dyn_image_from_raw(&photon_img).to_rgba();

    let mut image2 : DynamicImage = DynamicImage::new_luma8(
        image.width(), image.height());

    let font = Vec::from(include_bytes!("../fonts/Roboto-Regular.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    let height = 90f32;
    let scale = Scale { x: height * 1.0, y: height };
    draw_text_mut(&mut image2, Rgba([255u8, 255u8, 255u8, 255u8]), 10, 10, scale, &font, text);

    let mut image2 = image2.to_luma();
    dilate_mut(&mut image2, Norm::LInf, 4u8);

    draw_text_mut(&mut image, Rgba([255u8, 255u8, 255u8, 255u8]), x, y, scale, &font, text);
    let dynimage = image::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.raw_pixels();
}