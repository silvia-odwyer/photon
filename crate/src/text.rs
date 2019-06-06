extern crate image;
use image::{GenericImage, GenericImageView, Rgba, DynamicImage};
extern crate imageproc;
extern crate rusttype;
use wasm_bindgen::prelude::*;
use imageproc::drawing::draw_text_mut;
use imageproc::morphology::dilate_mut;
use imageproc::distance_transform::Norm;
use rusttype::{FontCollection, Scale};
use crate::{PhotonImage, helpers};

/// Add text to an image.
#[wasm_bindgen]
pub fn draw_text_with_border(mut photon_img: PhotonImage, text: &str, x: u32, y: u32) -> PhotonImage {

    let mut image = helpers::dyn_image_from_raw(&photon_img).to_rgba();

    let mut image2 : DynamicImage = DynamicImage::new_luma8(
        image.width(), image.height());

    let font = Vec::from(include_bytes!("../Roboto-Regular.ttf") as &[u8]);
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

    draw_text_mut(&mut image, Rgba([255u8, 255u8, 255u8, 255u8]), 10, 10, scale, &font, text);
    let dynimage = image::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.raw_pixels();
    return photon_img;
}

/// Add bordered-text to an image.
#[wasm_bindgen]
pub fn draw_text(mut photon_img: PhotonImage, text: &str, x: u32, y: u32) -> PhotonImage {

    let mut image = helpers::dyn_image_from_raw(&photon_img).to_rgba();

    let mut image2 : DynamicImage = DynamicImage::new_luma8(
        image.width(), image.height());

    let font = Vec::from(include_bytes!("../Roboto-Regular.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    let height = 90f32;
    let scale = Scale { x: height * 1.0, y: height };
    draw_text_mut(&mut image2, Rgba([255u8, 255u8, 255u8, 255u8]), 10, 10, scale, &font, text);

    let mut image2 = image2.to_luma();
    dilate_mut(&mut image2, Norm::LInf, 4u8);

    draw_text_mut(&mut image, Rgba([255u8, 255u8, 255u8, 255u8]), 10, 10, scale, &font, text);
    let dynimage = image::ImageRgba8(image);
    photon_img.raw_pixels = dynimage.raw_pixels();
    return photon_img;
}