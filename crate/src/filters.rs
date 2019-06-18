//! Preset color filters.

extern crate image;
use image::{GenericImage, GenericImageView};
use wasm_bindgen::prelude::*;
use crate::{PhotonImage};
use crate::{helpers, monochrome, effects};
use crate::channels::{alter_two_channels, alter_blue_channel};

/// Solarization on the Blue channel.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```
/// photon::filters::neue(&mut img);
/// ```
#[wasm_bindgen]
pub fn neue(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if 255 as i32 - px.data[2] as i32 > 0 {
                px.data[2] = 255 - px.data[2];
            }
            img.put_pixel(x, y, px);
        }
    }
    photon_image.raw_pixels = img.raw_pixels();
}

/// Solarization on the Red and Green channels.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```
/// photon::filters::lix(&mut img);
/// ```
#[wasm_bindgen]
pub fn lix(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            
            px.data[0] = 255 - px.data[0];
            px.data[1] = 255 - px.data[1];
            
            img.put_pixel(x, y, px);
        }
    }
    photon_image.raw_pixels = img.raw_pixels();
}

/// Solarization on the Red and Blue channels.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```
/// photon::filters::ryo(&mut img);
/// ```
#[wasm_bindgen]
pub fn ryo(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if 255 as i32 - px.data[2] as i32 > 0 {
                px.data[0] = 255 - px.data[0];
                px.data[2] = 255 - px.data[2];
            }
            img.put_pixel(x, y, px);
        }
    }
    photon_image.raw_pixels = img.raw_pixels();
}


/// Apply a filter to an image. Over 20 filters are available.
/// The filters are as follows:
/// * **oceanic**: Add an aquamarine-tinted hue to an image.
/// * **islands**: Aquamarine tint.
/// * **marine**: Add a green/blue mixed hue to an image.
/// * **seagreen**: Dark green hue, with tones of blue.
/// * **flagblue**: Royal blue tint
/// * **liquid**: Blue-inspired tint.
/// * **diamante**: Custom filter with a blue/turquoise tint.
/// * **radio**: Fallout-style radio effect.
/// * **twenties**: Slight-blue tinted historical effect.
/// * **rosetint**: Rose-tinted filter.
/// * **mauve**: Purple-infused filter.
/// * **bluechrome**: Blue monochrome effect.
/// * **vintage**: Vintage filter with a red tint.
/// * **perfume**: Increase the blue channel, with moderate increases in the Red and Green channels.
/// * **serenity**: Custom filter with an increase in the Blue channel's values.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `blend_mode` - The blending mode to use. See above for complete list of blend modes available.
/// # Example
///
/// ```
/// // For example, to add a filter called "vintage" to an image:
/// use photon::filters;
/// photon::filters::filter(&mut img, "vintage");
/// ```
#[wasm_bindgen]
pub fn filter(img: &mut PhotonImage, filter_name: &str) {

    match filter_name {
        // Match filter name to its corresponding function.
        "oceanic" => alter_two_channels(img, 1, 9, 2, 173),
        "islands" => alter_two_channels(img, 1, 24, 2, 95),
        "marine" => alter_two_channels(img, 1, 14, 2, 119),
        "seagreen" => alter_two_channels(img, 1, 68, 2, 62),
        "flagblue" => alter_blue_channel(img, 131),
        "diamante" => alter_two_channels(img, 1, 82, 2, 87),
        "liquid" => alter_two_channels(img, 1, 10, 2, 75),
        "radio" => monochrome::monochrome(img, 5, 40, 20),
        "twenties" => monochrome::monochrome(img, 18, 12, 20),
        "rosetint" =>  monochrome::monochrome(img, 80, 20, 31),
        "mauve" => monochrome::monochrome(img, 90, 40, 80),
        "bluechrome" => monochrome::monochrome(img, 20, 30, 60),
        "vintage" => effects::tint(img, 120, 70, 13),
        "perfume" => effects::tint(img, 80, 40, 120),
        "serenity" => effects::tint(img, 10, 40, 90),
        _ => monochrome::monochrome(img, 90, 40, 80),
    };
}