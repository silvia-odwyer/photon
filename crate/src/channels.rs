//! Channel manipulation.

extern crate image;

use image::{GenericImage, GenericImageView};

extern crate wasm_bindgen;
use crate::helpers;
use crate::{PhotonImage, Rgb};
extern crate palette;
use crate::channels::palette::Hue;
use palette::{Lab, Lch, Pixel, Saturate, Shade, Srgb, Srgba};
use wasm_bindgen::prelude::*;
use crate::channels::image::Pixel as OtherPixel;

/// Alter a select channel by incrementing or decrementing its value by a constant.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `channel` - The channel you wish to alter, it should be either 0, 1 or 2,
/// representing R, G, or B respectively. (O=Red, 1=Green, 2=Blue)
/// * `amount` - The amount to increment/decrement the channel's value by for that pixel.
/// A positive value will increment/decrement the channel's value, a negative value will decrement the channel's value.
///
/// ## Example
///
/// ```no_run
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon_rs::channels::alter_channel;
/// use photon_rs::native::{open_image, save_image};
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// alter_channel(&mut img, 0_usize, 10_i16);
/// // Write the contents of this image in JPG format.
/// save_image(img, "new_image.jpg").expect("File should be saved");
/// ```
///
/// Adds a constant to a select R, G, or B channel's value.
///
/// ### Decrease a channel's value
/// // For example, to decrease the Green channel for all pixels by 20:
/// ```no_run
/// use photon_rs::channels::alter_channel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// alter_channel(&mut img, 1_usize, -20_i16);
/// ```
/// **Note**: Note the use of a minus symbol when decreasing the channel.
#[wasm_bindgen]
pub fn alter_channel(img: &mut PhotonImage, channel: usize, amt: i16) {
    if channel > 2 {
        panic!("Invalid channel index passed. Channel must be 0, 1, or 2 (Red=0, Green=1, Blue=2)");
    }
    if amt > 255 {
        panic!("Amount to increment/decrement should be between -255 and 255");
    }
    let end = img.raw_pixels.len() - 4;

    for i in (channel..end).step_by(4) {
        let inc_val: i16 = img.raw_pixels[i] as i16 + amt as i16;
        img.raw_pixels[i] = num::clamp(inc_val, 0, 255) as u8;
    }
}

/// Increment or decrement every pixel's Red channel by a constant.
///
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `amt` - The amount to increment or decrement the channel's value by for that pixel.
///
/// # Example
///
/// ```no_run
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon_rs::channels::alter_red_channel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// alter_red_channel(&mut img, 10_i16);
/// ```
#[wasm_bindgen]
pub fn alter_red_channel(photon_image: &mut PhotonImage, amt: i16) {
    alter_channel(photon_image, 0, amt)
}

/// Increment or decrement every pixel's Green channel by a constant.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `amt` - The amount to increment/decrement the channel's value by for that pixel.
///
/// # Example
///
/// ```no_run
/// // For example, to increase the Green channel for all pixels by 20:
/// use photon_rs::channels::alter_green_channel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// alter_green_channel(&mut img, 20_i16);
/// ```
#[wasm_bindgen]
pub fn alter_green_channel(img: &mut PhotonImage, amt: i16) {
    alter_channel(img, 1, amt)
}

/// Increment or decrement every pixel's Blue channel by a constant.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `amt` - The amount to increment or decrement the channel's value by for that pixel.
///
/// # Example
///
/// ```no_run
/// // For example, to increase the Blue channel for all pixels by 10:
/// use photon_rs::channels::alter_blue_channel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// alter_blue_channel(&mut img, 10_i16);
/// ```
#[wasm_bindgen]
pub fn alter_blue_channel(img: &mut PhotonImage, amt: i16) {
    alter_channel(img, 2, amt)
}

/// Increment/decrement two channels' values simultaneously by adding an amt to each channel per pixel.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `channel1` - A usize from 0 to 2 that represents either the R, G or B channels.
/// * `amt1` - The amount to increment/decrement the channel's value by for that pixel.
/// * `channel2` -A usize from 0 to 2 that represents either the R, G or B channels.
/// * `amt2` - The amount to increment/decrement the channel's value by for that pixel.
///
/// # Example
///
/// ```no_run
/// // For example, to increase the values of the Red and Blue channels per pixel:
/// use photon_rs::channels::alter_two_channels;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// alter_two_channels(&mut img, 0_usize, 10_i16, 2_usize, 20_i16);
/// ```
#[wasm_bindgen]
pub fn alter_two_channels(
    img: &mut PhotonImage,
    channel1: usize,
    amt1: i16,
    channel2: usize,
    amt2: i16,
) {
    if channel1 > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }
    if channel2 > 2 {
        panic!("Invalid channel index passed. Channel2 must be equal to 0, 1, or 2");
    }
    if amt1 > 255 {
        panic!("Amount to inc/dec channel by should be between -255 and 255");
    }
    if amt2 > 255 {
        panic!("Amount to inc/dec channel by should be between -255 and 255");
    }
    let end = img.raw_pixels.len() - 4;

    for i in (0..end).step_by(4) {
        let inc_val1: i16 = img.raw_pixels[i + channel1] as i16 + amt1 as i16;
        let inc_val2: i16 = img.raw_pixels[i + channel2] as i16 + amt2 as i16;

        img.raw_pixels[i + channel1] = num::clamp(inc_val1, 0, 255) as u8;
        img.raw_pixels[i + channel2] = num::clamp(inc_val2, 0, 255) as u8;
    }
}

/// Increment all 3 channels' values by adding an amt to each channel per pixel.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `r_amt` - The amount to increment/decrement the Red channel by.
/// * `g_amt` - The amount to increment/decrement the Green channel by.
/// * `b_amt` - The amount to increment/decrement the Blue channel by.
///
/// # Example
///
/// ```no_run
/// // For example, to increase the values of the Red channel by 10, the Green channel by 20,
/// // and the Blue channel by 50:
/// use photon_rs::channels::alter_channels;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// alter_channels(&mut img, 10_i16, 20_i16, 50_i16);
/// ```
#[wasm_bindgen]
pub fn alter_channels(img: &mut PhotonImage, r_amt: i16, g_amt: i16, b_amt: i16) {
    if r_amt > 255 {
        panic!("Invalid r_amt passed. Amount to inc/dec channel by should be between -255 and 255");
    }
    if g_amt > 255 {
        panic!("Invalid g_amt passed. Amount to inc/dec channel by should be between -255 and 255");
    }
    if b_amt > 255 {
        panic!("Invalid b_amt passed. Amount to inc/dec channel by should be between -255 and 255");
    }
    let end = img.raw_pixels.len() - 4;

    for i in (0..end).step_by(4) {
        let r_val: i16 = img.raw_pixels[i] as i16 + r_amt as i16;
        let g_val: i16 = img.raw_pixels[i + 1] as i16 + g_amt as i16;
        let b_val: i16 = img.raw_pixels[i + 2] as i16 + b_amt as i16;

        img.raw_pixels[i] = num::clamp(r_val, 0, 255) as u8;
        img.raw_pixels[i + 1] = num::clamp(g_val, 0, 255) as u8;
        img.raw_pixels[i + 2] = num::clamp(b_val, 0, 255) as u8;
    }
}

/// Set a certain channel to zero, thus removing the channel's influence in the pixels' final rendered colour.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `channel` - The channel to be removed; must be a usize from 0 to 2, with 0 representing Red, 1 representing Green, and 2 representing Blue.
/// * `min_filter` - Minimum filter. Value between 0 and 255. Only remove the channel if the current pixel's channel value is less than this minimum filter. To completely
/// remove the channel, set this value to 255, to leave the channel as is, set to 0, and to set a channel to zero for a pixel whose red value is greater than 50,
/// then channel would be 0 and min_filter would be 50.
///
/// # Example
///
/// ```no_run
/// // For example, to remove the Red channel with a min_filter of 100:
/// use photon_rs::channels::remove_channel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// remove_channel(&mut img, 0_usize, 100_u8);
/// ```
#[wasm_bindgen]
pub fn remove_channel(img: &mut PhotonImage, channel: usize, min_filter: u8) {
    if channel > 2 {
        panic!("Invalid channel index passed. Channel must be equal to 0, 1, or 2.");
    }
    let end = img.raw_pixels.len() - 4;
    for i in (channel..end).step_by(4) {
        if img.raw_pixels[i] < min_filter {
            img.raw_pixels[i] = 0;
        };
    }
}

/// Remove the Red channel's influence in an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter.
///
/// # Example
///
/// ```no_run
/// // For example, to remove the red channel for red channel pixel values less than 50:
/// use photon_rs::channels::remove_red_channel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// remove_red_channel(&mut img, 50_u8);
/// ```
#[wasm_bindgen]
pub fn remove_red_channel(img: &mut PhotonImage, min_filter: u8) {
    remove_channel(img, 0, min_filter)
}

/// Remove the Green channel's influence in an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter.
///
/// # Example
///
/// ```no_run
/// // For example, to remove the green channel for green channel pixel values less than 50:
/// use photon_rs::channels::remove_green_channel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// remove_green_channel(&mut img, 50_u8);
/// ```
#[wasm_bindgen]
pub fn remove_green_channel(img: &mut PhotonImage, min_filter: u8) {
    remove_channel(img, 1, min_filter)
}

/// Remove the Blue channel's influence in an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter.
///
/// # Example
///
/// ```no_run
/// // For example, to remove the blue channel for blue channel pixel values less than 50:
/// use photon_rs::channels::remove_blue_channel;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// remove_blue_channel(&mut img, 50_u8);
/// ```
#[wasm_bindgen]
pub fn remove_blue_channel(img: &mut PhotonImage, min_filter: u8) {
    remove_channel(img, 2, min_filter)
}

/// Swap two channels.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `channel1` - An index from 0 to 2, representing the Red, Green or Blue channels respectively. Red would be represented by 0, Green by 1, and Blue by 2.
/// * `channel2` - An index from 0 to 2, representing the Red, Green or Blue channels respectively. Same as above.
///
/// # Example
///
/// ```no_run
/// // For example, to swap the values of the Red channel with the values of the Blue channel:
/// use photon_rs::channels::swap_channels;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// swap_channels(&mut img, 0_usize, 2_usize);
/// ```
#[wasm_bindgen]
pub fn swap_channels(img: &mut PhotonImage, mut channel1: usize, mut channel2: usize) {
    if channel1 > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }
    if channel2 > 2 {
        panic!("Invalid channel index passed. Channel2 must be equal to 0, 1, or 2.");
    }
    let end = img.raw_pixels.len() - 4;

    if channel1 > channel2 {
        std::mem::swap(&mut channel1, &mut channel2);
    }

    for i in (channel1..end).step_by(4) {
        let difference = channel2 - channel1;

        img.raw_pixels.swap(i, i + difference);
    }
}

/// Invert RGB value of an image.
///
/// # Arguments
/// * `photon_image` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```no_run
/// use photon_rs::channels::invert;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// invert(&mut img);
/// ```
#[wasm_bindgen]
pub fn invert(photon_image: &mut PhotonImage) {
    let end = photon_image.get_raw_pixels().len() - 4;

    for i in (0..end).step_by(4) {
        let r_val = photon_image.raw_pixels[i];
        let g_val = photon_image.raw_pixels[i + 1];
        let b_val = photon_image.raw_pixels[i + 2];

        photon_image.raw_pixels[i] = 255 - r_val;
        photon_image.raw_pixels[i + 1] = 255 - g_val;
        photon_image.raw_pixels[i + 2] = 255 - b_val;
    }
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
