//! Preset color filters.

extern crate image;
use image::{GenericImage, GenericImageView};
use wasm_bindgen::prelude::*;
use crate::{PhotonImage, Rgb};
use crate::{helpers, monochrome, effects};
use crate::channels::{alter_two_channels, alter_blue_channel};
use crate::colour_spaces::mix_with_colour;
use crate::colour_spaces;
use crate::effects::{inc_brightness, adjust_contrast};

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
/// * `filter_name` - The filter's name. Choose from the selection above, eg: "oceanic"
/// # Example
///
/// ```
/// // For example, to add a filter called "vintage" to an image:
/// use photon::filters;
/// photon::filters::filter(&mut img, "vintage");
/// ```
#[wasm_bindgen]
pub fn filter(img: &mut PhotonImage, filter_name: &str) {

    let oceanic_rgb = Rgb::new(0, 89, 173);
    let islands_rgb = Rgb::new(0, 24, 95);
    let marine_rgb = Rgb::new(0, 14, 119);
    let seagreen_rgb = Rgb::new(0, 68, 62);
    let flagblue_rgb = Rgb::new(0, 0, 131);
    let diamante_rgb = Rgb::new(30, 82, 87);
    let liquid_rgb = Rgb::new(0, 10, 75);
    let vintage_rgb = Rgb::new(120, 70, 13);
    let perfume_rgb = Rgb::new(80, 40, 120);
    let serenity_rgb = Rgb::new(10, 40, 90);

    match filter_name {
        // Match filter name to its corresponding function.
        "oceanic" => mix_with_colour(img, oceanic_rgb, 0.2),
        "islands" => mix_with_colour(img, islands_rgb, 0.2),
        "marine" => mix_with_colour(img, marine_rgb, 0.2),
        "seagreen" => mix_with_colour(img, seagreen_rgb, 0.2),
        "flagblue" => mix_with_colour(img, flagblue_rgb, 0.2),
        "diamante" => mix_with_colour(img, diamante_rgb, 0.1),
        "liquid" => mix_with_colour(img, liquid_rgb, 0.2),
        "radio" => monochrome::monochrome(img, 5, 40, 20),
        "twenties" => monochrome::monochrome(img, 18, 12, 20),
        "rosetint" =>  monochrome::monochrome(img, 80, 20, 31),
        "mauve" => monochrome::monochrome(img, 90, 40, 80),
        "bluechrome" => monochrome::monochrome(img, 20, 30, 60),
        "vintage" => mix_with_colour(img, vintage_rgb, 0.2),
        "perfume" => mix_with_colour(img, perfume_rgb, 0.2),
        "serenity" => mix_with_colour(img, serenity_rgb, 0.2),
        "golden" => golden(img),
        "pastel_pink" => pastel_pink(img),
        "cali" => cali(img),
        "lofi" => lofi(img),
        _ => monochrome::monochrome(img, 90, 40, 80),
    };
}

#[wasm_bindgen]
pub fn lofi(img: &mut PhotonImage) {
    adjust_contrast(img, 30.0);
    colour_spaces::saturate_hsl(img, 0.2);
}

#[wasm_bindgen]
pub fn pastel_pink(img: &mut PhotonImage) {
    let pastel_pink_rgb = Rgb::new(220, 112, 170);
    mix_with_colour(img, pastel_pink_rgb, 0.1);
    adjust_contrast(img, 30.0);
}

#[wasm_bindgen]
pub fn golden(img: &mut PhotonImage) {
    let vignette_rgb = Rgb::new(235, 145, 50);
    adjust_contrast(img, 30.0);
    mix_with_colour(img, vignette_rgb, 0.2);
}

#[wasm_bindgen]
pub fn cali(img: &mut PhotonImage) {
    let cali_rgb = Rgb::new(255, 45, 75);
    colour_spaces::mix_with_colour(img, cali_rgb, 0.1);
    adjust_contrast(img, 50.0);
}