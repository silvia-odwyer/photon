//! Channel manipulation.

extern crate image;
use image::{GenericImage, GenericImageView};
extern crate wasm_bindgen;
use crate::helpers;
use crate::{PhotonImage, Rgb};
extern crate palette;
use palette::{Pixel, Lch, Shade, Saturate, Srgba, Srgb, Lab};
use wasm_bindgen::prelude::*;
use crate::channels::palette::Hue;

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
/// ```
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon::channels;
/// let img = photon::open_image("img.jpg");
/// photon::channels::alter_channel(&mut img, 0, 10);
/// // Write the contents of this image in JPG format.
/// photon::helpers::save_image(img, "new_image.png");
/// ```
/// 
/// Adds a constant to a select R, G, or B channel's value.
/// 
/// ### Decrease a channel's value
/// // For example, to decrease the Green channel for all pixels by 20:
/// ```
/// use photon::channels;
/// photon::channels::alter_channel(&mut img, 1, -20);
/// ```
/// **Note**: Note the use of a minus symbol when decreasing the channel. 
#[wasm_bindgen]
pub fn alter_channel(mut photon_image: &mut PhotonImage, channel: usize, amt: i16) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            
            let final_px_data_1: i32 = px.data[channel] as i32 + amt as i32;
            px.data[channel] = num::clamp(final_px_data_1, 0, 255) as u8;
            
            img.put_pixel(x, y, px);

        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Increment or decrement every pixel's Red channel by a constant.
/// 
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `amt` - The amount to increment or decrement the channel's value by for that pixel.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon::channels;
/// photon::channels::alter_red_channel(&mut img, 10);
/// ```
#[wasm_bindgen]
pub fn alter_red_channel(img: &mut PhotonImage, amt: i16) {
    return alter_channel(img, 0, amt);
}

/// Increment or decrement every pixel's Green channel by a constant.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `amt` - The amount to increment/decrement the channel's value by for that pixel.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Green channel for all pixels by 20:
/// use photon::channels;
/// photon::channels::alter_green_channel(&mut img, 10);
/// ```
#[wasm_bindgen]
pub fn alter_green_channel(img: &mut PhotonImage, amt: i16) {
    return alter_channel(img, 1, amt);
}

/// Increment or decrement every pixel's Blue channel by a constant.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `amt` - The amount to increment or decrement the channel's value by for that pixel.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Blue channel for all pixels by 10:
/// use photon::channels;
/// photon::channels::alter_blue_channel(&mut img, 10);
/// ```
#[wasm_bindgen]
pub fn alter_blue_channel(img: &mut PhotonImage, amt: i16) {
    return alter_channel(img, 2, amt);
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
/// ```
/// // For example, to increase the values of the Red and Blue channels per pixel:
/// photon::channels::inc_two_channels(&mut img, 0, 10, 2, 20);
/// ```
#[wasm_bindgen]
pub fn alter_two_channels(mut photon_image: &mut PhotonImage, channel1: usize, amt1: i16, channel2: usize, amt2: i16) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let px_data_1 = px.data[channel1];
            let mut final_px_data_1: u32 = px_data_1 as u32 + amt1 as u32;
                
            final_px_data_1 = num::clamp(final_px_data_1, 0, 255);
            px.data[channel1] = final_px_data_1 as u8;

            let px_data_2 = px.data[channel2];
            let mut final_px_data_2: u32 = px_data_2 as u32 + amt2 as u32;
                
            final_px_data_2 = num::clamp(final_px_data_2, 0, 255);
            px.data[channel2] = final_px_data_2 as u8;

            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
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
/// ```
/// // For example, to increase the values of the Red channel by 10, the Green channel by 20, 
/// // and the Blue channel by 50:
/// // photon::channels::alter_channels(&mut img, 10, 20, 50);
/// ```
#[wasm_bindgen]
pub fn alter_channels(mut photon_image: &mut PhotonImage, r_amt: i16, g_amt: i16, b_amt: i16) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let final_px_data_1: i32 = px.data[0] as i32 + r_amt as i32;
                
            px.data[0] = num::clamp(final_px_data_1, 0, 255) as u8;

            let final_px_data_2: i32 = px.data[1] as i32 + g_amt as i32;
                
            px.data[1] = num::clamp(final_px_data_2, 0, 255) as u8;

            let final_px_data_3: i32 = px.data[2] as i32 + b_amt as i32;
            px.data[2] = num::clamp(final_px_data_3, 0, 255) as u8;

            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Set a certain channel to zero, thus removing the channel's influence in the pixels' final rendered colour.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `channel` - The channel to be removed; must be a usize from 0 to 2, with 0 representing Red, 1 representing Green, and 2 representing Blue.
/// * `min_filter` - Value between 0 and 255. Only remove the channel if the current pixel's channel value is less than this minimum filter. To completely 
/// remove the channel, set this value to 255, to leave the channel as is, set to 0, and to set a channel to zero for a pixel whose red value is greater than 50, 
/// then channel would be 0 and min_filter would be 50.
/// 
/// # Example
///
/// ```
/// // For example, to remove the Red channel with a min_filter of 100:
/// photon::channels::remove_channel(&mut img, 0, 100);
/// ```
#[wasm_bindgen]
pub fn remove_channel(mut photon_image: &mut PhotonImage, channel: usize, min_filter: u8) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if px.data[channel] < min_filter {
                px.data[channel] = 0;
            }
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Remove the Red channel's influence in an image.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter. 
/// 
/// # Example
///
/// ```
/// // For example, to remove the red channel for red channel pixel values less than 50:
/// photon::channels::remove_red_channel(&mut img, 50);
/// ```
#[wasm_bindgen]
pub fn remove_red_channel(img: &mut PhotonImage, min_filter: u8) {
    return remove_channel(img, 0, min_filter);
}

/// Remove the Green channel's influence in an image.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter. 
/// 
/// # Example
///
/// ```
/// // For example, to remove the green channel for green channel pixel values less than 50:
/// photon::channels::remove_green_channel(img, 50);
/// ```
#[wasm_bindgen]
pub fn remove_green_channel(img: &mut PhotonImage, min_filter: u8) {
    return remove_channel(img, 1, min_filter);
}

/// Remove the Blue channel's influence in an image.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter. 
/// 
/// # Example
///
/// ```
/// // For example, to remove the blue channel for blue channel pixel values less than 50:
/// photon::channels::remove_blue_channel(&mut img, 50);
/// ```
#[wasm_bindgen]
pub fn remove_blue_channel(img: &mut PhotonImage, min_filter: u8) {
    return remove_channel(img, 2, min_filter);
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
/// ```
/// // For example, to swap the values of the Red channel with the values of the Blue channel:
/// photon::channels::swap_channels(&mut img, 0, 2);
/// ```
#[wasm_bindgen]
pub fn swap_channels(mut photon_image: &mut PhotonImage, channel1: usize, channel2: usize) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let temp_channel1 = px.data[channel1];
            px.data[channel1] = px.data[channel2];
            px.data[channel2] = temp_channel1;
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
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
/// ```
/// // For example, to only rotate the pixels that are of RGB value RGB{20, 40, 60}:
/// let ref_color = Rgb{20, 40, 60};
/// photon::channels::selective_hue_rotate(&mut img, ref_color, 180);
/// ```
#[wasm_bindgen]
pub fn selective_hue_rotate(mut photon_image: &mut PhotonImage, ref_color: Rgb, degrees: f32) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    
    let (_width, _height) = img.dimensions();
    let mut img = img.to_rgba();
    for x in 0.._width {
        for y in 0.._height {
            let px = img.get_pixel(x, y);

            // Reference colour to compare the current pixel's colour to
            let lab: Lab = Srgb::new(ref_color.r as f32 / 255.0, ref_color.g as f32 / 255.0, ref_color.b as f32 / 255.0).into();
      
            // Convert the current pixel's colour to the l*a*b colour space
            let r_val: f32 = px.data[0] as f32 / 255.0;
            let g_val: f32 = px.data[1] as f32 / 255.0;
            let b_val: f32 = px.data[2] as f32 / 255.0;

            let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();

            let sim = color_sim(lab, px_lab);
            if sim > 0 && sim < 40 {
                let px_data = img.get_pixel(x, y).data;
                let color = Srgba::from_raw(&px_data).into_format();

                let hue_rotated_color = Lch::from(color).shift_hue(degrees);
                img.put_pixel(x, y, image::Rgba {
                    data: Srgba::from_linear(hue_rotated_color.into()).into_format().into_raw()
                });
            }
        }
    }
    photon_image.raw_pixels = img.to_vec();
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
    let sq_rt = (total as f64).sqrt() as i64 + 1;

    return sq_rt;
}


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
/// ```
/// // For example, to only lighten the pixels that are of or similar to RGB value RGB{20, 40, 60}:
/// let ref_color = Rgb{20, 40, 60};
/// photon::channels::selective_lighten(&mut img, ref_color, 0.2);
/// ```
#[wasm_bindgen]
pub fn selective_lighten(img: &mut PhotonImage, ref_color: Rgb, amt: f32) {
    return selective(img, "lighten", ref_color, amt);
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
/// ```
/// // For example, to only desaturate the pixels that are similar to the RGB value RGB{20, 40, 60}:
/// let ref_color = Rgb{20, 40, 60};
/// photon::channels::selective_desaturate(&mut img, ref_color, 0.1);
/// ```
#[wasm_bindgen]
pub fn selective_desaturate(img: &mut PhotonImage, ref_color: Rgb, amt: f32) {
    return selective(img, "desaturate", ref_color, amt);
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
/// ```
/// // For example, to only increase the saturation of pixels that are similar to the RGB value RGB{20, 40, 60}:
/// let ref_color = Rgb{20, 40, 60};
/// photon::channels::selective_saturate(&mut img, ref_color, 0.1);
/// ```
#[wasm_bindgen]
pub fn selective_saturate(img: &mut PhotonImage, ref_color: Rgb, amt: f32) {
    selective(img, "saturate", ref_color, amt);
}


fn selective(mut photon_image: &mut PhotonImage, mode: &'static str, ref_color:Rgb, amt: f32) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let (_width, _height) = img.dimensions();
    let mut img = img.to_rgba();
    for x in 0.._width {
        for y in 0.._height {
            let px = img.get_pixel(x, y);

            // Reference colour to compare the current pixel's colour to
            let lab: Lab = Srgb::new(ref_color.r as f32 / 255.0, ref_color.g as f32 / 255.0, ref_color.b as f32 / 255.0).into();
      
            // Convert the current pixel's colour to the l*a*b colour space
            let r_val: f32 = px.data[0] as f32 / 255.0;
            let g_val: f32 = px.data[1] as f32 / 255.0;
            let b_val: f32 = px.data[2] as f32 / 255.0;

            let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();

            let sim = color_sim(lab, px_lab);
            if sim > 0 && sim < 40 {
                let px_data = img.get_pixel(x, y).data;
                let lch_colour: Lch = Srgb::from_raw(&px_data)
                    .into_format()
                    .into_linear()
                    .into();

                let new_color = match mode {
                // Match a single value
                "desaturate" => lch_colour.desaturate(amt),
                "saturate" => lch_colour.saturate(amt),
                "lighten" => lch_colour.lighten(amt), 
                "darken" => lch_colour.darken(amt),
                _ => lch_colour.saturate(amt),
                };
            
                img.put_pixel(x, y, image::Rgba {
                        data: Srgba::from_linear(new_color.into()).into_format().into_raw()
                });

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
/// ```
/// // For example, to greyscale all pixels that are *not* visually similar to the RGB colour RGB{20, 40, 60}:
/// let ref_color = Rgb{20, 40, 60};
/// photon::channels::selective_greyscale(&mut img, ref_color);
/// ```
#[wasm_bindgen]
pub fn selective_greyscale(mut photon_image: PhotonImage, ref_color: Rgb) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    for x in 0..photon_image.width {
        for y in 0..photon_image.height {
            let mut px = img.get_pixel(x, y);

            // Reference colour to compare the current pixel's colour to
            let lab: Lab = Srgb::new(ref_color.r as f32 / 255.0, ref_color.g as f32 / 255.0, ref_color.b as f32 / 255.0).into();

            // Convert the current pixel's colour to the l*a*b colour space
            let r_val: f32 = px.data[0] as f32 / 255.0;
            let g_val: f32 = px.data[1] as f32 / 255.0;
            let b_val: f32 = px.data[2] as f32 / 255.0;

            let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();

            let sim = color_sim(lab, px_lab);
            if sim > 30 {
                let avg = px.data[0] as f32 * 0.3 + px.data[1] as f32 * 0.59 + px.data[2] as f32 * 0.11;
                px.data[0] = avg as u8;
                px.data[1] = avg as u8;
                px.data[2] = avg as u8;
            }
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}
