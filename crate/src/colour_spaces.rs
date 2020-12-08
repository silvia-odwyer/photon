//! Image manipulation effects in HSL, LCh and HSV.

extern crate image;
extern crate rand;
use crate::{helpers, PhotonImage, Rgb};
use image::GenericImageView;
use palette::{Hsl, Hsv, Hue, Lch, Pixel, Saturate, Shade, Srgba, Srgb};
extern crate wasm_bindgen;
use crate::iter::ImageIterator;
use wasm_bindgen::prelude::*;
use image::Pixel as ImagePixel;
use image::GenericImage;
use palette::Lab;

/// Apply gamma correction.
// #[wasm_bindgen]
// pub fn gamma_correction(mut photon_image: &mut PhotonImage, red: f32, green: f32,  blue: f32) {
//     let img = helpers::dyn_image_from_raw(&photon_image);
//     let (width, height) = img.dimensions();
//     let mut img = img.to_rgba8();

//     // Initialize gamma arrays
//     let mut gammaR: Vec<u8> = vec![];
//     let mut gammaG: Vec<u8> = vec![];
//     let mut gammaB: Vec<u8> = vec![];

//     let MAX_VALUE_INT = 255;
//     let MAX_VALUE_FLT = 255.0;
//     let REVERSE = 1.0;

//     // Set values within gamma arrays
//     for i in 0..256 {
//         gammaR[i] = min(MAX_VALUE_INT, ((MAX_VALUE_FLT * ((i as f32 / MAX_VALUE_FLT) as u32).powf(REVERSE / red) + 0.5 ) as u8));
//         gammaG[i] = min(MAX_VALUE_INT, ((MAX_VALUE_FLT * ((i as f32 / MAX_VALUE_FLT) as u32).powf(REVERSE / green) + 0.5 ) as u8);
//         gammaB[i] = min(MAX_VALUE_INT, ((MAX_VALUE_FLT * ((i as f32 / MAX_VALUE_FLT) as u32).powf(REVERSE / blue) + 0.5 ) as u8);

//     }

//     for x in 0..width {
//         for y in 0..height {
//             let px_data = img.get_pixel(x, y).data;

//             let r_val = px_data[0];
//             let g_val = px_data[1];
//             let b_val = px_data[2];

//             px_data[0] = gammaR[r_val as usize];
//             px_data[1] = gammaG[g_val as usize];
//             px_data[2] = gammaB[b_val as usize];

//             img.put_pixel(x, y, px);
//             }
//         }
//     photon_image.raw_pixels = img.to_vec();
// }

/// Image manipulation effects in the LCh colour space
///
/// Effects include:
/// * **saturate** - Saturation increase.
/// * **desaturate** - Desaturate the image.
/// * **shift_hue** - Hue rotation by a specified number of degrees.
/// * **darken** - Decrease the brightness.
/// * **lighten** - Increase the brightness.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
/// * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
/// # Example
/// ```no_run
/// // For example to increase the saturation by 10%:
/// use photon_rs::colour_spaces::lch;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// lch(&mut img, "saturate", 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn lch(mut photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let px_data = img.get_pixel(x, y).channels();
        let lch_colour: Lch =
            Srgba::new(px_data[0], px_data[1], px_data[2], 255).into_format().into_linear().into();

        let new_color = match mode {
            // Match a single value
            "desaturate" => lch_colour.desaturate(amt),
            "saturate" => lch_colour.saturate(amt),
            "lighten" => lch_colour.lighten(amt),
            "darken" => lch_colour.darken(amt),
            "shift_hue" => lch_colour.shift_hue(amt * 360.0),
            _ => lch_colour.saturate(amt),
        };
        let final_color: Srgba = Srgba::from_linear(new_color.into())
        .into_format();

        let components = final_color
        .into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([(components.0 * 255.0) as u8, (components.1 * 255.0) as u8, (components.2 * 255.0) as u8 , 255])
        );
    }
    photon_image.raw_pixels = img.to_vec();
}

/// Image manipulation effects in the HSL colour space.
///
/// Effects include:
/// * **saturate** - Saturation increase.
/// * **desaturate** - Desaturate the image.
/// * **shift_hue** - Hue rotation by a specified number of degrees.
/// * **darken** - Decrease the brightness.
/// * **lighten** - Increase the brightness.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
/// * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
/// # Example
/// ```no_run
/// // For example to increase the saturation by 10%:
/// use photon_rs::colour_spaces::hsl;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// hsl(&mut img, "saturate", 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn hsl(mut photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    // The function logic is kept separate from other colour spaces for now,
    // since other HSL-specific logic may be implemented here, which isn't available in other colour spaces
    let mut img = helpers::dyn_image_from_raw(&photon_image).to_rgba8();
    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let px_data = img.get_pixel(x, y).channels();

        let colour = Srgba::new(px_data[0], px_data[1], px_data[2], 255).into_format();

        let hsl_colour = Hsl::from(colour);

        let new_color = match mode {
            // Match a single value
            "desaturate" => hsl_colour.desaturate(amt),
            "saturate" => hsl_colour.saturate(amt),
            "lighten" => hsl_colour.lighten(amt),
            "darken" => hsl_colour.darken(amt),
            "shift_hue" => hsl_colour.shift_hue(amt * 360.0),
            _ => hsl_colour.saturate(amt),
        };
        let final_color: Srgba = Srgba::from_linear(new_color.into())
        .into_format();

        let components = final_color
        .into_components();

        
        img.put_pixel(
            x,
            y,
            image::Rgba([(components.0 * 255.0) as u8, (components.1 * 255.0) as u8, (components.2 * 255.0) as u8 , 255])

        );
    }

    photon_image.raw_pixels = img.to_vec();
}

/// Image manipulation in the HSV colour space.
///
/// Effects include:
/// * **saturate** - Saturation increase.
/// * **desaturate** - Desaturate the image.
/// * **shift_hue** - Hue rotation by a specified number of degrees.
/// * **darken** - Decrease the brightness.
/// * **lighten** - Increase the brightness.
///
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
/// * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
///
/// # Example
/// ```no_run
/// // For example to increase the saturation by 10%:
/// use photon_rs::colour_spaces::hsv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// hsv(&mut img, "saturate", 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn hsv(photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let px_data = img.get_pixel(x, y).channels();

        let color = Srgba::new(px_data[0], px_data[1], px_data[2], 255).into_format();

        let hsv_colour = Hsv::from(color);

        let new_color = match mode {
            // Match a single value
            "desaturate" => hsv_colour.desaturate(amt),
            "saturate" => hsv_colour.saturate(amt),
            "lighten" => hsv_colour.lighten(amt),
            "darken" => hsv_colour.darken(amt),
            "shift_hue" => hsv_colour.shift_hue(amt * 360.0),
            _ => hsv_colour.saturate(amt),
        };

        let final_color: Srgba = Srgba::from_linear(new_color.into())
        .into_format();

        let components = final_color
        .into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([(components.0 * 255.0) as u8, (components.1 * 255.0) as u8, (components.2 * 255.0) as u8 , 255])
        );
    }
    photon_image.raw_pixels = img.to_vec();
}

/// Shift hue by a specified number of degrees in the HSL colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
///
/// # Example
/// ```no_run
/// // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
/// use photon_rs::colour_spaces::hue_rotate_hsl;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// hue_rotate_hsl(&mut img, 120_f32);
/// ```
#[wasm_bindgen]
pub fn hue_rotate_hsl(img: &mut PhotonImage, degrees: f32) {
    hsl(img, "shift_hue", degrees);
}

/// Shift hue by a specified number of degrees in the HSV colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
///
/// # Example
/// ```no_run
/// // For example to hue rotate/shift the hue by 120 degrees in the HSV colour space:
/// use photon_rs::colour_spaces::hue_rotate_hsv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// hue_rotate_hsv(&mut img, 120_f32);
/// ```
#[wasm_bindgen]
pub fn hue_rotate_hsv(img: &mut PhotonImage, degrees: f32) {
    hsv(img, "shift_hue", degrees);
}

/// Shift hue by a specified number of degrees in the LCh colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
///
/// # Example
/// ```no_run
/// // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
/// use photon_rs::colour_spaces::hue_rotate_lch;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// hue_rotate_lch(&mut img, 120_f32);
/// ```
#[wasm_bindgen]
pub fn hue_rotate_lch(img: &mut PhotonImage, degrees: f32) {
    lch(img, "shift_hue", degrees)
}

/// Increase the image's saturation by converting each pixel's colour to the HSL colour space
/// and increasing the colour's saturation.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Increasing saturation by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to increase saturation by 10% in the HSL colour space:
/// use photon_rs::colour_spaces::saturate_hsl;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// saturate_hsl(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn saturate_hsl(img: &mut PhotonImage, level: f32) {
    hsl(img, "saturate", level)
}

/// Increase the image's saturation in the LCh colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Increasing saturation by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to increase saturation by 40% in the Lch colour space:
/// use photon_rs::colour_spaces::saturate_lch;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// saturate_lch(&mut img, 0.4_f32);
/// ```
#[wasm_bindgen]
pub fn saturate_lch(img: &mut PhotonImage, level: f32) {
    lch(img, "saturate", level)
}

/// Increase the image's saturation in the HSV colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level by which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Increasing saturation by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to increase saturation by 30% in the HSV colour space:
/// use photon_rs::colour_spaces::saturate_hsv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// saturate_hsv(&mut img, 0.3_f32);
/// ```
#[wasm_bindgen]
pub fn saturate_hsv(img: &mut PhotonImage, level: f32) {
    hsv(img, "saturate", level)
}

/// Lighten an image by a specified amount in the LCh colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Lightening by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to lighten an image by 10% in the LCh colour space:
/// use photon_rs::colour_spaces::lighten_lch;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// lighten_lch(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn lighten_lch(img: &mut PhotonImage, level: f32) {
    lch(img, "lighten", level)
}

/// Lighten an image by a specified amount in the HSL colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Lightening by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to lighten an image by 10% in the HSL colour space:
/// use photon_rs::colour_spaces::lighten_hsl;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// lighten_hsl(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn lighten_hsl(img: &mut PhotonImage, level: f32) {
    hsl(img, "lighten", level)
}

/// Lighten an image by a specified amount in the HSV colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Lightening by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to lighten an image by 10% in the HSV colour space:
/// use photon_rs::colour_spaces::lighten_hsv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// lighten_hsv(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn lighten_hsv(img: &mut PhotonImage, level: f32) {
    hsv(img, "lighten", level)
}

/// Darken the image by a specified amount in the LCh colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Darkening by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to darken an image by 10% in the LCh colour space:
/// use photon_rs::colour_spaces::darken_lch;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// darken_lch(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn darken_lch(img: &mut PhotonImage, level: f32) {
    lch(img, "darken", level)
}

/// Darken the image by a specified amount in the HSL colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Darkening by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to darken an image by 10% in the HSL colour space:
/// use photon_rs::colour_spaces::darken_hsl;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// darken_hsl(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn darken_hsl(img: &mut PhotonImage, level: f32) {
    hsl(img, "darken", level)
}

/// Darken the image's colours by a specified amount in the HSV colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Darkening by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to darken an image by 10% in the HSV colour space:
/// use photon_rs::colour_spaces::darken_hsv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// darken_hsv(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn darken_hsv(img: &mut PhotonImage, level: f32) {
    hsv(img, "darken", level)
}

/// Desaturate the image by a specified amount in the HSV colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Desaturating by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to desaturate an image by 10% in the HSV colour space:
/// use photon_rs::colour_spaces::desaturate_hsv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// desaturate_hsv(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn desaturate_hsv(img: &mut PhotonImage, level: f32) {
    hsv(img, "desaturate", level)
}

/// Desaturate the image by a specified amount in the HSL colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Desaturating by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to desaturate an image by 10% in the LCh colour space:
/// use photon_rs::colour_spaces::desaturate_hsl;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// desaturate_hsl(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn desaturate_hsl(img: &mut PhotonImage, level: f32) {
    hsl(img, "desaturate", level)
}

/// Desaturate the image by a specified amount in the LCh colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Desaturating by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to desaturate an image by 10% in the LCh colour space:
/// use photon_rs::colour_spaces::desaturate_lch;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// desaturate_lch(&mut img, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn desaturate_lch(img: &mut PhotonImage, level: f32) {
    lch(img, "desaturate", level)
}

/// Mix image with a single color, supporting passing `opacity`.
/// The algorithm comes from Jimp. See `function mix` and `function colorFn` at following link:
/// https://github.com/oliver-moran/jimp/blob/29679faa597228ff2f20d34c5758e4d2257065a3/packages/plugin-color/src/index.js
/// Specifically, result_value = (mix_color_value - origin_value) * opacity + origin_value =
/// mix_color_value * opacity + (1 - opacity) * origin_value for each
/// of RGB channel.
///
/// # Arguments
/// * `photon_image` - A PhotonImage that contains a view into the image.
/// * `mix_color` - the color to be mixed in, as an RGB value.
/// * `opacity` - the opacity of color when mixed to image. Float value from 0 to 1.
/// # Example
///
/// ```no_run
/// // For example, to mix an image with rgb (50, 255, 254) and opacity 0.4:
/// use photon_rs::Rgb;
/// use photon_rs::colour_spaces::mix_with_colour;
/// use photon_rs::native::open_image;
///
/// let mix_colour = Rgb::new(50_u8, 255_u8, 254_u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// mix_with_colour(&mut img, mix_colour, 0.4_f32);
/// ```
#[wasm_bindgen]
pub fn mix_with_colour(photon_image: &mut PhotonImage, mix_colour: Rgb, opacity: f32) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    // cache (mix_color_value * opacity) and (1 - opacity) so we dont need to calculate them each time during loop.
    let mix_red_offset = mix_colour.r as f32 * opacity;
    let mix_green_offset = mix_colour.g as f32 * opacity;
    let mix_blue_offset = mix_colour.b as f32 * opacity;
    let factor = 1.0 - opacity;

    for (x, y) in ImageIterator::new(width, height) {
        let px = img.get_pixel(x, y);
        let channels = px.channels();

        let r_value = mix_red_offset + (channels[0] as f32) * factor;
        let g_value = mix_green_offset + (channels[1] as f32) * factor;
        let b_value = mix_blue_offset + (channels[2] as f32) * factor;
        let alpha = channels[3];
        img.put_pixel(
            x,
            y,
            image::Rgba([r_value as u8, g_value as u8, b_value as u8, alpha]),
        );
    }
    photon_image.raw_pixels = img.to_vec();
}

// #[wasm_bindgen]
// pub fn selective_color_convert(mut photon_image: &mut PhotonImage, ref_color:Rgb, new_color:Rgb, fraction: f32) {
//     let img = helpers::dyn_image_from_raw(&photon_image);
//     let (_width, _height) = img.dimensions();
//     let mut img = img.to_rgba8();
//     for x in 0.._width {
//         for y in 0.._height {
//             let mut px = img.get_pixel(x, y);

//             // Reference colour to compare the current pixel's colour to
//             let lab: Lab = Srgb::new(ref_color.r as f32 / 255.0, ref_color.g as f32 / 255.0, ref_color.b as f32 / 255.0).into();

//             // Convert the current pixel's colour to the l*a*b colour space
//             let r_val: f32 = px.data[0] as f32 / 255.0;
//             let g_val: f32 = px.data[1] as f32 / 255.0;
//             let b_val: f32 = px.data[2] as f32 / 255.0;

//             let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();

//             let sim = color_sim(lab, px_lab);
//             if sim > 0 && sim < 40 {
//                 let newr = (((new_color.r - ref_color.r) as f32) * fraction + new_color.r as f32) as u8;
//                 let newg = (((new_color.g - ref_color.g) as f32) * fraction + new_color.g as f32) as u8;
//                 let newb = (((new_color.b - ref_color.b) as f32) * fraction + new_color.b as f32) as u8;

//                 img.put_pixel(x, y, image::Rgba([newr, newg, newb, 255]));
//             }
//         }
//     }
//     photon_image.raw_pixels = img.to_vec();
// }

// pub fn correct(img: &DynamicImage, mode: &'static str, colour_space: &'static str, amt: f32) -> DynamicImage {
//     let mut img  = img.to_rgb();

//     let (width, height) = img.dimensions();

//         for x in 0..width {
//             for y in 0..height {
//                 let px_data = img.get_pixel(x, y).data;

//                 let colour_to_cspace;
//                 if colour_space == "hsv" {
//                     colour_to_cspace: Hsv = Srgb::from_raw(&px_data).into_format();
//                 }
//                 else if colour_space == "hsl" {
//                     colour_to_cspace = Hsl::from(color);
//                 }
//                 else {
//                     colour_to_cspace = Lch::from(color);
//                 }

//                 let new_color  = match mode {
//                     // Match a single value
//                     "desaturate" => colour_to_cspace.desaturate(amt),
//                     "saturate" => colour_to_cspace.saturate(amt),
//                     "lighten" => colour_to_cspace.lighten(amt),
//                     "darken" => colour_to_cspace.darken(amt),
//                     _ => colour_to_cspace.saturate(amt),
//                 };

//                 img.put_pixel(x, y, image::Rgb {
//                     data: Srgb::from_linear(new_color.into()).into_format().into_raw()
//                 });
//             }
//         }

//     let dynimage = image::ImageRgb8(img);
//     return dynimage;
// }


/// Selectively lighten an image.
///
/// Only lighten the hue of a pixel if its colour matches or is similar to the RGB colour specified.
/// For example, if a user wishes all pixels that are blue to be lightened, they can selectively specify  only the blue pixels to be changed.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
/// * `amt` - The level from 0 to 1 to lighten the hue by. Increasing by 10% would have an `amt` of 0.1
///
/// # Example
///
/// ```no_run
/// // For example, to only lighten the pixels that are of or similar to RGB value RGB{20, 40, 60}:
/// use photon_rs::Rgb;
/// use photon_rs::channels::selective_lighten;
/// use photon_rs::native::open_image;
///
/// let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// selective_lighten(&mut img, ref_color, 0.2_f32);
/// ```
#[wasm_bindgen]
pub fn selective_lighten(img: &mut PhotonImage, ref_color: Rgb, amt: f32) {
    selective(img, "lighten", ref_color, amt)
}

/// Selectively desaturate pixel colours which are similar to the reference colour provided.
///
/// Similarity between two colours is calculated via the CIE76 formula.
/// Only desaturates the hue of a pixel if its similarity to the reference colour is within the range in the algorithm.
/// For example, if a user wishes all pixels that are blue to be desaturated by 0.1, they can selectively specify  only the blue pixels to be changed.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
/// * `amt` - The amount of desaturate the colour by.
///
/// # Example
///
/// ```no_run
/// // For example, to only desaturate the pixels that are similar to the RGB value RGB{20, 40, 60}:
/// use photon_rs::Rgb;
/// use photon_rs::channels::selective_desaturate;
/// use photon_rs::native::open_image;
///
/// let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// selective_desaturate(&mut img, ref_color, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn selective_desaturate(img: &mut PhotonImage, ref_color: Rgb, amt: f32) {
    selective(img, "desaturate", ref_color, amt)
}

/// Selectively saturate pixel colours which are similar to the reference colour provided.
///
/// Similarity between two colours is calculated via the CIE76 formula.
/// Only saturates the hue of a pixel if its similarity to the reference colour is within the range in the algorithm.
/// For example, if a user wishes all pixels that are blue to have an increase in saturation by 10%, they can selectively specify only the blue pixels to be changed.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
/// * `amt` - The amount of saturate the colour by.
///
/// # Example
///
/// ```no_run
/// // For example, to only increase the saturation of pixels that are similar to the RGB value RGB{20, 40, 60}:
/// use photon_rs::Rgb;
/// use photon_rs::channels::selective_saturate;
/// use photon_rs::native::open_image;
///
/// let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// selective_saturate(&mut img, ref_color, 0.1_f32);
/// ```
#[wasm_bindgen]
pub fn selective_saturate(img: &mut PhotonImage, ref_color: Rgb, amt: f32) {
    selective(img, "saturate", ref_color, amt);
}

fn selective(
    mut photon_image: &mut PhotonImage,
    mode: &'static str,
    ref_color: Rgb,
    amt: f32,
) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();
    for x in (0..width) {
        for y in (0..height) {
            let px = img.get_pixel(x, y);

            // Reference colour to compare the current pixel's colour to
            let lab: Lab = Srgb::new(
                ref_color.r as f32 / 255.0,
                ref_color.g as f32 / 255.0,
                ref_color.b as f32 / 255.0,
            )
            .into();
            let channels = px.channels();
            // Convert the current pixel's colour to the l*a*b colour space
            let r_val: f32 = channels[0] as f32 / 255.0;
            let g_val: f32 = channels[1] as f32 / 255.0;
            let b_val: f32 = channels[2] as f32 / 255.0;
    
            let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();
    
            let sim = color_sim(lab, px_lab);
            if sim > 0 && sim < 40 {
                let px_data = img.get_pixel(x, y).channels();
                let lch_colour: Lch =
                    Srgb::new(px_data[0], px_data[1], px_data[2]).into_format().into_linear().into();
    
                let new_color = match mode {
                    // Match a single value
                    "desaturate" => lch_colour.desaturate(amt),
                    "saturate" => lch_colour.saturate(amt),
                    "lighten" => lch_colour.lighten(amt),
                    "darken" => lch_colour.darken(amt),
                    _ => lch_colour.saturate(amt),
                };

                let final_color: Srgba = Srgba::from_linear(new_color.into())
                .into_format();
        
                let components = final_color
                .into_components();
        
                img.put_pixel(
                    x,
                    y,
                    image::Rgba([(components.0 * 255.0) as u8, (components.1 * 255.0) as u8, (components.2 * 255.0) as u8 , 255])
                );
        }
    } 
    }
    photon_image.raw_pixels = img.to_vec();
}

/// Selectively changes a pixel to greyscale if it is *not* visually similar or close to the colour specified.
/// Only changes the colour of a pixel if its RGB values are within a specified range.
///
/// (Similarity between two colours is calculated via the CIE76 formula.)
/// For example, if a user wishes all pixels that are *NOT* blue to be displayed in greyscale, they can selectively specify only the blue pixels to be
/// kept in the photo.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
///
/// # Example
///
/// ```no_run
/// // For example, to greyscale all pixels that are *not* visually similar to the RGB colour RGB{20, 40, 60}:
/// use photon_rs::Rgb;
/// use photon_rs::channels::selective_greyscale;
/// use photon_rs::native::open_image;
///
/// let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// selective_greyscale(img, ref_color);
/// ```
#[wasm_bindgen]
pub fn selective_greyscale(mut photon_image: PhotonImage, ref_color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    for x in (0..img.width()) {
        for y in (0..img.height()) {
            let mut px = img.get_pixel(x, y);

            // Reference colour to compare the current pixel's colour to
            let lab: Lab = Srgb::new(
                ref_color.r as f32 / 255.0,
                ref_color.g as f32 / 255.0,
                ref_color.b as f32 / 255.0,
            )
            .into();
            let channels = px.channels();
            // Convert the current pixel's colour to the l*a*b colour space
            let r_val: f32 = channels[0] as f32 / 255.0;
            let g_val: f32 = channels[1] as f32 / 255.0;
            let b_val: f32 = channels[2] as f32 / 255.0;
    
            let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();
    
            let sim = color_sim(lab, px_lab);
            if sim > 30 {
                let avg = channels[0] as f32 * 0.3
                    + channels[1] as f32 * 0.59
                    + channels[2] as f32 * 0.11;
                px = image::Rgba([avg as u8, avg as u8, avg as u8, 255]);                
            }
            img.put_pixel(x, y, px);
        
        }
    } 
    
    let raw_pixels = img.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Get the similarity of two colours in the l*a*b colour space using the CIE76 formula.
pub fn color_sim(lab1: Lab, lab2: Lab) -> i64 {
    let l_comp = lab2.l - lab1.l;
    let a_comp = lab2.a - lab1.a;
    let b_comp = lab2.b - lab1.b;

    let l_comp_sq = l_comp.powf(2.0);
    let a_comp_sq = a_comp.powf(2.0);
    let b_comp_sq = b_comp.powf(2.0);

    let total = l_comp_sq + a_comp_sq + b_comp_sq;
    (total as f64).sqrt() as i64 + 1
}

/// Selective hue rotation.
///
/// Only rotate the hue of a pixel if its RGB values are within a specified range.
/// This function only rotates a pixel's hue to another  if it is visually similar to the colour specified.
/// For example, if a user wishes all pixels that are blue to be changed to red, they can selectively specify  only the blue pixels to be changed.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
/// * `degrees` - The amount of degrees to hue rotate by.
///
/// # Example
///
/// ```no_run
/// // For example, to only rotate the pixels that are of RGB value RGB{20, 40, 60}:
/// use photon_rs::Rgb;
/// use photon_rs::channels::selective_hue_rotate;
/// use photon_rs::native::open_image;
///
/// let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
/// let mut img = open_image("img.jpg").expect("File should open");
/// selective_hue_rotate(&mut img, ref_color, 180_f32);
/// ```
#[wasm_bindgen]
pub fn selective_hue_rotate(
    mut photon_image: &mut PhotonImage,
    ref_color: Rgb,
    degrees: f32,
) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    let mut img = img.to_rgba8();
    for x in (0..width) {
        for y in (0..height) {
            let px = img.get_pixel(x, y);

            // Reference colour to compare the current pixel's colour to
            let lab: Lab = Srgb::new(
                ref_color.r as f32 / 255.0,
                ref_color.g as f32 / 255.0,
                ref_color.b as f32 / 255.0,
            )
            .into();
            let channels = px.channels();
            // Convert the current pixel's colour to the l*a*b colour space
            let r_val: f32 = channels[0] as f32 / 255.0;
            let g_val: f32 = channels[1] as f32 / 255.0;
            let b_val: f32 = channels[2] as f32 / 255.0;
    
            let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();
    
            let sim = color_sim(lab, px_lab);
            if sim > 0 && sim < 40 {
                let px_data = img.get_pixel(x, y).channels();
                let color = Srgba::new(px_data[0], px_data[1], px_data[2], 255).into_format();
    
                let hue_rotated_color = Lch::from(color).shift_hue(degrees);

                let final_color: Srgba = Srgba::from_linear(hue_rotated_color.into())
                .into_format();
        
                let components = final_color
                .into_components();
        
                img.put_pixel(
                    x,
                    y,
                    image::Rgba([(components.0 * 255.0) as u8, (components.1 * 255.0) as u8, (components.2 * 255.0) as u8 , 255])
                );
            }
        }
    }
        
    photon_image.raw_pixels = img.to_vec();
}