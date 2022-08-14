//! Preset color filters.

use crate::colour_spaces;
use crate::colour_spaces::mix_with_colour;
use crate::effects::{adjust_contrast, inc_brightness};
use crate::monochrome;
use crate::{PhotonImage, Rgb};
use wasm_bindgen::prelude::*;

/// Solarization on the Blue channel.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::neue;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// neue(&mut img);
/// ```
#[wasm_bindgen]
pub fn neue(photon_image: &mut PhotonImage) {
    let end = photon_image.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let b_val = photon_image.raw_pixels[i + 2];
        if 255_i32 - b_val as i32 > 0 {
            photon_image.raw_pixels[i + 2] = 255 - b_val;
        }
    }
}

/// Solarization on the Red and Green channels.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::lix;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// lix(&mut img);
/// ```
#[wasm_bindgen]
pub fn lix(photon_image: &mut PhotonImage) {
    let end = photon_image.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = photon_image.raw_pixels[i];
        let g_val = photon_image.raw_pixels[i + 1];

        photon_image.raw_pixels[i] = 255 - r_val;
        photon_image.raw_pixels[i + 1] = 255 - g_val;
    }
}

/// Solarization on the Red and Blue channels.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::ryo;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// ryo(&mut img);
/// ```
#[wasm_bindgen]
pub fn ryo(photon_image: &mut PhotonImage) {
    let end = photon_image.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = photon_image.raw_pixels[i];
        let b_val = photon_image.raw_pixels[i + 2];

        photon_image.raw_pixels[i] = 255 - r_val;
        photon_image.raw_pixels[i + 2] = 255 - b_val;
    }
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
/// ```no_run
/// // For example, to add a filter called "vintage" to an image:
/// use photon_rs::filters::filter;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// filter(&mut img, "vintage");
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
        "rosetint" => monochrome::monochrome(img, 80, 20, 31),
        "mauve" => monochrome::monochrome(img, 90, 40, 80),
        "bluechrome" => monochrome::monochrome(img, 20, 30, 60),
        "vintage" => mix_with_colour(img, vintage_rgb, 0.2),
        "perfume" => mix_with_colour(img, perfume_rgb, 0.2),
        "serenity" => mix_with_colour(img, serenity_rgb, 0.2),
        "golden" => golden(img),
        "pastel_pink" => pastel_pink(img),
        "cali" => cali(img),
        "dramatic" => dramatic(img),
        "firenze" => firenze(img),
        "obsidian" => obsidian(img),
        "lofi" => lofi(img),
        _ => monochrome::monochrome(img, 90, 40, 80),
    };
}

/// Apply a lofi effect to an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::lofi;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// lofi(&mut img);
/// ```
#[wasm_bindgen]
pub fn lofi(img: &mut PhotonImage) {
    adjust_contrast(img, 30.0);
    colour_spaces::saturate_hsl(img, 0.2);
}

/// Apply a rose tint to an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::pastel_pink;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// pastel_pink(&mut img);
/// ```
#[wasm_bindgen]
pub fn pastel_pink(img: &mut PhotonImage) {
    let pastel_pink_rgb = Rgb::new(220, 112, 170);
    mix_with_colour(img, pastel_pink_rgb, 0.1);
    adjust_contrast(img, 30.0);
}

/// Apply a vintage, golden hue to an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::golden;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// golden(&mut img);
/// ```
#[wasm_bindgen]
pub fn golden(img: &mut PhotonImage) {
    let vignette_rgb = Rgb::new(235, 145, 50);
    mix_with_colour(img, vignette_rgb, 0.2);
    adjust_contrast(img, 30.0);
}

/// Increased contrast filter effect.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::cali;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// cali(&mut img);
/// ```
#[wasm_bindgen]
pub fn cali(img: &mut PhotonImage) {
    let cali_rgb = Rgb::new(255, 45, 75);
    colour_spaces::mix_with_colour(img, cali_rgb, 0.1);
    adjust_contrast(img, 50.0);
}

/// Greyscale effect with increased contrast.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::dramatic;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// dramatic(&mut img);
/// ```
#[wasm_bindgen]
pub fn dramatic(img: &mut PhotonImage) {
    monochrome::grayscale(img);
    adjust_contrast(img, 60.0);
}

/// Apply a red hue, with increased contrast and brightness.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::firenze;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// firenze(&mut img);
/// ```
#[wasm_bindgen]
pub fn firenze(img: &mut PhotonImage) {
    let cali_rgb = Rgb::new(255, 47, 78);
    colour_spaces::mix_with_colour(img, cali_rgb, 0.1);

    inc_brightness(img, 30);
    adjust_contrast(img, 50.0);
}

/// Apply a greyscale effect with increased contrast.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// # Example
///
/// ```no_run
/// use photon_rs::filters::obsidian;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// obsidian(&mut img);
/// ```
#[wasm_bindgen]
pub fn obsidian(img: &mut PhotonImage) {
    monochrome::grayscale(img);
    adjust_contrast(img, 25.0);
}
