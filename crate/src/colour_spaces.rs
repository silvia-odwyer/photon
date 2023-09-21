//! Image manipulation effects in HSL, HSLuv, LCh and HSV.

use crate::iter::ImageIterator;
use crate::{helpers, PhotonImage, Rgb};
use image::GenericImageView;
use image::Pixel as ImagePixel;
use palette::{FromColor, IntoColor};
use palette::{Hsla, Hsluva, Hsva, Hue, Lcha, Saturate, Shade, Srgba};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// Applies gamma correction to an image.
/// # Arguments
/// * `photon_image` - A PhotonImage that contains a view into the image.
/// * `red` - Gamma value for red channel.
/// * `green` - Gamma value for green channel.
/// * `blue` - Gamma value for blue channel.
/// # Example
///
/// ```no_run
/// // For example, to turn an image of type `PhotonImage` into a gamma corrected image:
/// use photon_rs::colour_spaces::gamma_correction;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// gamma_correction(&mut img, 2.2, 2.2, 2.2);
/// ```
///
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn gamma_correction(
    photon_image: &mut PhotonImage,
    red: f32,
    green: f32,
    blue: f32,
) {
    let buf = photon_image.raw_pixels.as_mut_slice();
    let buf_size = buf.len();

    // Initialize gamma arrays
    let mut gamma_r: Vec<u8> = vec![0; 256];
    let mut gamma_g: Vec<u8> = vec![0; 256];
    let mut gamma_b: Vec<u8> = vec![0; 256];

    let inv_red = 1.0 / red;
    let inv_green = 1.0 / green;
    let inv_blue = 1.0 / blue;

    // Set values within gamma arrays
    for i in 0..256 {
        let input = (i as f32) / 255.0;
        gamma_r[i] = (255.0 * input.powf(inv_red) + 0.5).clamp(0.0, 255.0) as u8;
        gamma_g[i] = (255.0 * input.powf(inv_green) + 0.5).clamp(0.0, 255.0) as u8;
        gamma_b[i] = (255.0 * input.powf(inv_blue) + 0.5).clamp(0.0, 255.0) as u8;
    }

    // Apply gamma correction
    for i in (0..buf_size).step_by(4) {
        let r = buf[i];
        let g = buf[i + 1];
        let b = buf[i + 2];

        buf[i] = gamma_r[r as usize];
        buf[i + 1] = gamma_g[g as usize];
        buf[i + 2] = gamma_b[b as usize];
    }
}

/// Image manipulation effects in the HSLuv colour space
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
/// use photon_rs::colour_spaces::hsluv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// hsluv(&mut img, "saturate", 0.1_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hsluv(photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let px_data = img.get_pixel(x, y).channels();
        let hsluv_color: Hsluva = Srgba::new(
            px_data[0] as f32 / 255.0,
            px_data[1] as f32 / 255.0,
            px_data[2] as f32 / 255.0,
            px_data[3] as f32 / 255.0,
        )
        .into_linear()
        .into_color();

        let new_color = match mode {
            // Match a single value
            "desaturate" => hsluv_color.desaturate(amt),
            "saturate" => hsluv_color.saturate(amt),
            "lighten" => hsluv_color.lighten(amt),
            "darken" => hsluv_color.darken(amt),
            "shift_hue" => hsluv_color.shift_hue(amt * 360.0),
            _ => hsluv_color.saturate(amt),
        };
        let final_color: Srgba =
            Srgba::from_linear(new_color.into_color()).into_format();

        let components = final_color.into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([
                (components.0 * 255.0) as u8,
                (components.1 * 255.0) as u8,
                (components.2 * 255.0) as u8,
                (components.3 * 255.0) as u8,
            ]),
        );
    }
    photon_image.raw_pixels = img.to_vec();
}

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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn lch(photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let px_data = img.get_pixel(x, y).channels();
        let lch_colour: Lcha = Srgba::new(
            px_data[0] as f32 / 255.0,
            px_data[1] as f32 / 255.0,
            px_data[2] as f32 / 255.0,
            px_data[3] as f32 / 255.0,
        )
        .into_linear()
        .into_color();

        let new_color = match mode {
            // Match a single value
            "desaturate" => lch_colour.desaturate(amt),
            "saturate" => lch_colour.saturate(amt),
            "lighten" => lch_colour.lighten(amt),
            "darken" => lch_colour.darken(amt),
            "shift_hue" => lch_colour.shift_hue(amt * 360.0),
            _ => lch_colour.saturate(amt),
        };
        let final_color: Srgba =
            Srgba::from_linear(new_color.into_color()).into_format();

        let components = final_color.into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([
                (components.0 * 255.0) as u8,
                (components.1 * 255.0) as u8,
                (components.2 * 255.0) as u8,
                (components.3 * 255.0) as u8,
            ]),
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hsl(photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    // The function logic is kept separate from other colour spaces for now,
    // since other HSL-specific logic may be implemented here, which isn't available in other colour spaces
    let mut img = helpers::dyn_image_from_raw(photon_image).to_rgba8();
    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let px_data = img.get_pixel(x, y).channels();

        let colour = Srgba::new(
            px_data[0] as f32 / 255.0,
            px_data[1] as f32 / 255.0,
            px_data[2] as f32 / 255.0,
            px_data[3] as f32 / 255.0,
        );

        let hsl_colour = Hsla::from_color(colour);

        let new_color = match mode {
            // Match a single value
            "desaturate" => hsl_colour.desaturate(amt),
            "saturate" => hsl_colour.saturate(amt),
            "lighten" => hsl_colour.lighten(amt),
            "darken" => hsl_colour.darken(amt),
            "shift_hue" => hsl_colour.shift_hue(amt * 360.0),
            _ => hsl_colour.saturate(amt),
        };
        let final_color = Srgba::from_color(new_color);

        let components = final_color.into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([
                (components.0 * 255.0) as u8,
                (components.1 * 255.0) as u8,
                (components.2 * 255.0) as u8,
                (components.3 * 255.0) as u8,
            ]),
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hsv(photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let px_data = img.get_pixel(x, y).channels();

        let color = Srgba::new(
            px_data[0] as f32 / 255.0,
            px_data[1] as f32 / 255.0,
            px_data[2] as f32 / 255.0,
            px_data[3] as f32 / 255.0,
        );

        let hsv_colour = Hsva::from_color(color);

        let new_color = match mode {
            // Match a single value
            "desaturate" => hsv_colour.desaturate(amt),
            "saturate" => hsv_colour.saturate(amt),
            "lighten" => hsv_colour.lighten(amt),
            "darken" => hsv_colour.darken(amt),
            "shift_hue" => hsv_colour.shift_hue(amt * 360.0),
            _ => hsv_colour.saturate(amt),
        };

        let srgba_new_color = Srgba::from_color(new_color);
        // let final_color: Srgba = Srgba::from_linear(srgba_new_color).into_format();

        let components = srgba_new_color.into_components();

        img.put_pixel(
            x,
            y,
            image::Rgba([
                (components.0 * 255.0) as u8,
                (components.1 * 255.0) as u8,
                (components.2 * 255.0) as u8,
                (components.3 * 255.0) as u8,
            ]),
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hue_rotate_lch(img: &mut PhotonImage, degrees: f32) {
    lch(img, "shift_hue", degrees)
}

/// Shift hue by a specified number of degrees in the HSLuv colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
///
/// # Example
/// ```no_run
/// // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
/// use photon_rs::colour_spaces::hue_rotate_hsluv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// hue_rotate_hsluv(&mut img, 120_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn hue_rotate_hsluv(img: &mut PhotonImage, degrees: f32) {
    hsluv(img, "shift_hue", degrees)
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn saturate_lch(img: &mut PhotonImage, level: f32) {
    lch(img, "saturate", level)
}

/// Increase the image's saturation in the HSLuv colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Increasing saturation by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to increase saturation by 40% in the HSLuv colour space:
/// use photon_rs::colour_spaces::saturate_hsluv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// saturate_hsluv(&mut img, 0.4_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn saturate_hsluv(img: &mut PhotonImage, level: f32) {
    hsluv(img, "saturate", level)
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn lighten_lch(img: &mut PhotonImage, level: f32) {
    lch(img, "lighten", level)
}

/// Lighten an image by a specified amount in the HSLuv colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Lightening by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to lighten an image by 10% in the HSLuv colour space:
/// use photon_rs::colour_spaces::lighten_hsluv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// lighten_hsluv(&mut img, 0.1_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn lighten_hsluv(img: &mut PhotonImage, level: f32) {
    hsluv(img, "lighten", level)
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn darken_lch(img: &mut PhotonImage, level: f32) {
    lch(img, "darken", level)
}

/// Darken the image by a specified amount in the HSLuv colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Darkening by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to darken an image by 10% in the HSLuv colour space:
/// use photon_rs::colour_spaces::darken_hsluv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// darken_hsluv(&mut img, 0.1_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn darken_hsluv(img: &mut PhotonImage, level: f32) {
    hsluv(img, "darken", level)
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn desaturate_lch(img: &mut PhotonImage, level: f32) {
    lch(img, "desaturate", level)
}

/// Desaturate the image by a specified amount in the HSLuv colour space.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format.
/// Desaturating by 80% would be represented by a `level` of 0.8
///
/// # Example
/// ```no_run
/// // For example to desaturate an image by 10% in the HSLuv colour space:
/// use photon_rs::colour_spaces::desaturate_hsluv;
/// use photon_rs::native::open_image;
///
/// // Open the image. A PhotonImage is returned.
/// let mut img = open_image("img.jpg").expect("File should open");
/// desaturate_hsluv(&mut img, 0.1_f32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn desaturate_hsluv(img: &mut PhotonImage, level: f32) {
    hsluv(img, "desaturate", level)
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn mix_with_colour(photon_image: &mut PhotonImage, mix_colour: Rgb, opacity: f32) {
    let img = helpers::dyn_image_from_raw(photon_image);
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
