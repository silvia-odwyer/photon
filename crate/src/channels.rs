//! Channel manipulation.

use image::Pixel as OtherPixel;

use image::{GenericImage, GenericImageView};

use crate::helpers;
use crate::iter::ImageIterator;
use crate::{PhotonImage, Rgb};
use palette::{FromColor, IntoColor};
use palette::{Hue, Lab, Lch, Saturate, Shade, Srgb, Srgba};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

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
/// use photon_rs::native::{open_image};
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// alter_channel(&mut img, 0_usize, 10_i16);
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn alter_channel(img: &mut PhotonImage, channel: usize, amt: i16) {
    if channel > 2 {
        panic!("Invalid channel index passed. Channel must be 0, 1, or 2 (Red=0, Green=1, Blue=2)");
    }
    if amt > 255 {
        panic!("Amount to increment/decrement should be between -255 and 255");
    }
    let end = img.raw_pixels.len();

    for i in (channel..end).step_by(4) {
        let inc_val: i16 = img.raw_pixels[i] as i16 + amt;
        img.raw_pixels[i] = inc_val.clamp(0, 255) as u8;
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
    let end = img.raw_pixels.len();

    for i in (0..end).step_by(4) {
        let inc_val1: i16 = img.raw_pixels[i + channel1] as i16 + amt1;
        let inc_val2: i16 = img.raw_pixels[i + channel2] as i16 + amt2;

        img.raw_pixels[i + channel1] = inc_val1.clamp(0, 255) as u8;
        img.raw_pixels[i + channel2] = inc_val2.clamp(0, 255) as u8;
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
    let end = img.raw_pixels.len();

    for i in (0..end).step_by(4) {
        let r_val: i16 = img.raw_pixels[i] as i16 + r_amt;
        let g_val: i16 = img.raw_pixels[i + 1] as i16 + g_amt;
        let b_val: i16 = img.raw_pixels[i + 2] as i16 + b_amt;

        img.raw_pixels[i] = r_val.clamp(0, 255) as u8;
        img.raw_pixels[i + 1] = g_val.clamp(0, 255) as u8;
        img.raw_pixels[i + 2] = b_val.clamp(0, 255) as u8;
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn remove_channel(img: &mut PhotonImage, channel: usize, min_filter: u8) {
    if channel > 2 {
        panic!("Invalid channel index passed. Channel must be equal to 0, 1, or 2.");
    }
    let end = img.raw_pixels.len();
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn swap_channels(img: &mut PhotonImage, mut channel1: usize, mut channel2: usize) {
    if channel1 > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }
    if channel2 > 2 {
        panic!("Invalid channel index passed. Channel2 must be equal to 0, 1, or 2.");
    }
    let end = img.raw_pixels.len();

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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn invert(photon_image: &mut PhotonImage) {
    let end = photon_image.get_raw_pixels().len();

    for i in (0..end).step_by(4) {
        let r_val = photon_image.raw_pixels[i];
        let g_val = photon_image.raw_pixels[i + 1];
        let b_val = photon_image.raw_pixels[i + 2];

        photon_image.raw_pixels[i] = 255 - r_val;
        photon_image.raw_pixels[i + 1] = 255 - g_val;
        photon_image.raw_pixels[i + 2] = 255 - b_val;
    }
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_hue_rotate(
    photon_image: &mut PhotonImage,
    ref_color: Rgb,
    degrees: f32,
) {
    let img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();

    let mut img = img.to_rgba8();
    for (x, y) in ImageIterator::new(width, height) {
        let px = img.get_pixel(x, y);

        // Reference colour to compare the current pixel's colour to
        let lab: Lab = Srgb::new(
            ref_color.r as f32 / 255.0,
            ref_color.g as f32 / 255.0,
            ref_color.b as f32 / 255.0,
        )
        .into_color();
        let channels = px.channels();
        // Convert the current pixel's colour to the l*a*b colour space
        let r_val: f32 = channels[0] as f32 / 255.0;
        let g_val: f32 = channels[1] as f32 / 255.0;
        let b_val: f32 = channels[2] as f32 / 255.0;

        let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into_color();

        let sim = color_sim(lab, px_lab);
        if sim > 0 && sim < 40 {
            let px_data = img.get_pixel(x, y).channels();
            let color = Srgba::new(
                px_data[0] as f32,
                px_data[1] as f32,
                px_data[2] as f32,
                255.0,
            );
            let hue_rotated_color = Lch::from_color(color).shift_hue(degrees);

            let final_color: Srgba =
                Srgba::from_linear(hue_rotated_color.into_color()).into_format();

            let components = final_color.into_components();

            img.put_pixel(
                x,
                y,
                image::Rgba([
                    (components.0 * 255.0) as u8,
                    (components.1 * 255.0) as u8,
                    (components.2 * 255.0) as u8,
                    255,
                ]),
            );
        }
    }

    photon_image.raw_pixels = img.to_vec();
}

/// Selectively change pixel colours which are similar to the reference colour provided.
///
/// Similarity between two colours is calculated via the CIE76 formula.
/// Only changes the color of a pixel if its similarity to the reference colour is within the range in the algorithm.
/// For example, with this function, a user can change the color of all blue pixels by mixing them with red by 10%.
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
/// * `new_color` - The `RGB` value of the new color (to be mixed with the matched pixels)
/// * `fraction` - The amount of mixing the new colour with the matched pixels
///
/// # Example
///
/// ```no_run
/// // For example, to only change the color of pixels that are similar to the RGB value RGB{200, 120, 30} by mixing RGB{30, 120, 200} with 25%:
/// use photon_rs::Rgb;
/// use photon_rs::channels::selective_color_convert;
/// use photon_rs::native::open_image;
///
/// let ref_color = Rgb::new(200, 120, 30);
/// let new_color = Rgb::new(30, 120, 200);
/// let mut img = open_image("img.jpg").expect("File should open");
/// selective_color_convert(&mut img, ref_color, new_color, 0.25);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_color_convert(
    photon_image: &mut PhotonImage,
    ref_color: Rgb,
    new_color: Rgb,
    fraction: f32,
) {
    let buffer = photon_image.raw_pixels.as_mut_slice();

    // Reference colour to compare the current pixel's colour to
    let ref_lab: Lab = Srgb::new(
        ref_color.r as f32 / 255.0,
        ref_color.g as f32 / 255.0,
        ref_color.b as f32 / 255.0,
    )
    .into_color();

    for px in buffer.chunks_mut(4) {
        let px_lab: Lab = Srgb::new(
            px[0] as f32 / 255.0,
            px[1] as f32 / 255.0,
            px[2] as f32 / 255.0,
        )
        .into_color();
        let sim = color_sim(ref_lab, px_lab);

        if sim > 0 && sim < 40 {
            px[0] = ((px[0] as f32) + fraction * ((new_color.r as f32) - (px[0] as f32)))
                .clamp(0.0, 255.0) as u8;
            px[1] = ((px[1] as f32) + fraction * ((new_color.g as f32) - (px[1] as f32)))
                .clamp(0.0, 255.0) as u8;
            px[2] = ((px[2] as f32) + fraction * ((new_color.b as f32) - (px[2] as f32)))
                .clamp(0.0, 255.0) as u8;
        }
    }
}

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
//     dynimage
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_saturate(img: &mut PhotonImage, ref_color: Rgb, amt: f32) {
    selective(img, "saturate", ref_color, amt);
}

fn selective(
    photon_image: &mut PhotonImage,
    mode: &'static str,
    ref_color: Rgb,
    amt: f32,
) {
    let img = helpers::dyn_image_from_raw(photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba8();

    for (x, y) in ImageIterator::new(width, height) {
        let px = img.get_pixel(x, y);

        // Reference colour to compare the current pixel's colour to
        let lab: Lab = Srgb::new(
            ref_color.r as f32 / 255.0,
            ref_color.g as f32 / 255.0,
            ref_color.b as f32 / 255.0,
        )
        .into_color();
        let channels = px.channels();
        // Convert the current pixel's colour to the l*a*b colour space
        let r_val: f32 = channels[0] as f32 / 255.0;
        let g_val: f32 = channels[1] as f32 / 255.0;
        let b_val: f32 = channels[2] as f32 / 255.0;

        let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into_color();

        let sim = color_sim(lab, px_lab);
        if sim > 0 && sim < 40 {
            let px_data = img.get_pixel(x, y).channels();
            let lch_colour: Lch = Srgb::new(px_data[0], px_data[1], px_data[2])
                .into_format()
                .into_linear()
                .into_color();

            let new_color = match mode {
                // Match a single value
                "desaturate" => lch_colour.desaturate(amt),
                "saturate" => lch_colour.saturate(amt),
                "lighten" => lch_colour.lighten(amt),
                "darken" => lch_colour.darken(amt),
                _ => lch_colour.saturate(amt),
            };

            // let final_color: Srgba = Srgba::from_linear(new_color.into_color());
            let final_color = Srgba::from_color(new_color);

            let components = final_color.into_components();

            img.put_pixel(
                x,
                y,
                image::Rgba([
                    (components.0 * 255.0) as u8,
                    (components.1 * 255.0) as u8,
                    (components.2 * 255.0) as u8,
                    255,
                ]),
            );
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
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn selective_greyscale(mut photon_image: PhotonImage, ref_color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    for (x, y) in ImageIterator::new(img.width(), img.height()) {
        let mut px = img.get_pixel(x, y);

        // Reference colour to compare the current pixel's colour to
        let lab: Lab = Srgb::new(
            ref_color.r as f32 / 255.0,
            ref_color.g as f32 / 255.0,
            ref_color.b as f32 / 255.0,
        )
        .into_color();
        let channels = px.channels();
        // Convert the current pixel's colour to the l*a*b colour space
        let r_val: f32 = channels[0] as f32 / 255.0;
        let g_val: f32 = channels[1] as f32 / 255.0;
        let b_val: f32 = channels[2] as f32 / 255.0;

        let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into_color();

        let sim = color_sim(lab, px_lab);
        if sim > 30 {
            let avg = channels[0] as f32 * 0.3
                + channels[1] as f32 * 0.59
                + channels[2] as f32 * 0.11;
            px = image::Rgba([avg as u8, avg as u8, avg as u8, 255]);
        }
        img.put_pixel(x, y, px);
    }

    let raw_pixels = img.into_bytes();
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
