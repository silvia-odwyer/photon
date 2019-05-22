extern crate image;
use image::{GenericImage, DynamicImage, GenericImageView};
extern crate palette;
use palette::{Srgb, Xyz, Lab};
use palette::{Color, Shade, Saturate};
use palette::{Hsv, LinSrgb, Pixel};
use image::{RgbImage};
use crate::effects::{Rgb};

/// Alter a select channel by incrementing its value by a constant.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `channel` - The channel you wish to inc, it should be either 0, 1 or 2, 
/// representing R, G, or B respectively
/// * `offset` - The amount you want to increment the channel's value by for that pixel.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon::channels;
/// photon::channels::inc_channel(img, 0, 10);
/// ```
/// Adds a constant to a select R, G, or B channel's value.
pub fn inc_channel(mut img: DynamicImage, channel: usize, offset: i16) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            
            if px.data[channel] <= 255 - offset as u8 {
                let px_data = px.data[channel];
                let final_px_data = px_data + offset as u8;
                px.data[channel] = final_px_data as u8;
            }
            else {
                px.data[channel] = 255;
            }

            // else if offset < 0 {
            //     if 255 as u16 + offset as u16 > 0 as u16 {
            //         let px_data = px.data[channel];
            //         let final_px_data = px_data + offset as u8;
            //         px.data[channel] = final_px_data as u8;
            //     }
            //     else {
            //         px.data[channel] = 255;
            //     }
            // }

            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Increment every pixel's Red channel by a constant.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `offset` - The amount you want to increment the channel's value by for that pixel.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon::channels;
/// photon::channels::inc_red_channel(img, 10);
/// ```
pub fn inc_red_channel(img: DynamicImage, offset: i16) -> DynamicImage {
    let res_img = inc_channel(img, 0, offset);
    return res_img;
}

/// Increment every pixel's Green channel by a constant.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `offset` - The amount you want to increment the channel's value by for that pixel.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Green channel for all pixels by 20:
/// use photon::channels;
/// photon::channels::inc_green_channel(img, 10);
/// ```
pub fn inc_green_channel(img: DynamicImage, offset: i16) -> DynamicImage {
    let res_img = inc_channel(img, 1, offset);
    return res_img;
}

/// Increment every pixel's Blue channel by a constant.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `offset` - The amount you want to increment the channel's value by for that pixel.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Blue channel for all pixels by 10:
/// use photon::channels;
/// photon::channels::inc_blue_channel(img, 10);
/// ```
pub fn inc_blue_channel(img: DynamicImage, offset: i16) -> DynamicImage {
    let res_img = inc_channel(img, 2, offset);
    return res_img;
}

/// Increment two channels' values simultaneously by adding an offset to each channel per pixel.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `channel1` - A usize that represents an index into the RGB vec. 
/// * `offset1` - The amount you want to increment the channel's value by for that pixel.
/// * `channel2` - A usize that represents an index into the RGB vec. 0 would return the Red channel. 
/// * `offset2` - The amount you want to increment the channel's value by for that pixel.
/// 
/// # Example
///
/// ```
/// // For example, to increase the values of the Blue and Red channels per pixel:
/// photon::channels::inc_two_channels(img, 0, 10, 2, 20);
/// ```
pub fn inc_two_channels(mut img: DynamicImage, channel1: usize, offset1: i16, channel2: usize, offset2: i16) -> DynamicImage {
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if px.data[channel1] <= 255 - offset1 as u8 {
                let px_data = px.data[channel1];
                let final_px_data = px_data + offset1 as u8;
                px.data[channel1] = final_px_data;
            }
            else {
                px.data[channel1] = 255;
            }
                
            if px.data[channel2] <= 255 - offset2 as u8 {
                let px_data = px.data[channel2];
                let final_px_data = px_data + offset2 as u8;
                px.data[channel2] = final_px_data;
            }
            else {
                px.data[channel2] = 255
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Set a certain channel to zero, thus removing the channel's influence in the pixels' final rendered colour.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `channel` - The channel to be removed; must be a usize from 0 to 2, with 0 representing Red, 1 representing Green, and 2 representing Blue.
/// * `min_filter` - Value between 0 and 255. Only remove the channel if the current pixel's channel value is less than this minimum filter. To completely 
/// remove the channel, set this value to 255, to leave the channel as is, set to 0, and to set a channel to zero for a pixel whose red value is greater than 50, 
/// then channel would be 0 and min_filter would be 50.
/// 
/// # Example
///
/// ```
/// // For example, to remove the Red channel with a min_filter of 100:
/// photon::channels::remove_channel(img, 0, 100);
/// ```
pub fn remove_channel(mut img: DynamicImage, channel: usize, min_filter: u8) -> DynamicImage {
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if px.data[channel] < min_filter{
                px.data[channel] = 0;
                px.data[1] += 2;
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Remove the Red channel's influence in an image, by setting its value to zero.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter. 
/// 
/// # Example
///
/// ```
/// // For example, to remove the red channel for red channel pixel values less than 50:
/// photon::channels::remove_red_channel(img, 50);
/// ```
pub fn remove_red_channel(img: DynamicImage, min_filter: u8) -> DynamicImage {
    let filtered_img = remove_channel(img, 0, min_filter);
    return filtered_img;
}

/// Remove the Green channel's influence in an image, by setting its value to zero.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter. 
/// 
/// # Example
///
/// ```
/// // For example, to remove the green channel for green channel pixel values less than 50:
/// photon::channels::remove_green_channel(img, 50);
/// ```
pub fn remove_green_channel(img: DynamicImage, min_filter: u8) -> DynamicImage {
    let filtered_img = remove_channel(img, 1, min_filter);
    return filtered_img;
}

/// Remove the Blue channel's influence in an image, by setting its value to zero.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter. 
/// 
/// # Example
///
/// ```
/// // For example, to remove the blue channel for blue channel pixel values less than 50:
/// photon::channels::remove_blue_channel(img, 50);
/// ```
pub fn remove_blue_channel(img: DynamicImage, min_filter: u8) -> DynamicImage {
    let filtered_img = remove_channel(img, 2, min_filter);
    return filtered_img;
}

/// Swap two channels.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `channel1` - An index from 0 to 2, representing either the Red, Green or Blue channels respectively.
/// * `channel2` - An index from 0 to 2, representing either the Red, Green or Blue channels respectively.
/// 
/// # Example
///
/// ```
/// // For example, to swap the values of the Red channel with the values of the Blue channel:
/// photon::channels::swap_channels(img, 0, 2);
/// ```
pub fn swap_channels(mut img: DynamicImage, channel1: usize, channel2: usize) -> DynamicImage {
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
    return img;
}

/// Selective colour change.
/// Only changes the colour of a pixel if its RGB values are within a specified range.
/// This function only changes a pixel's colour to another colour if it is visually similar to the colour specified.
/// For example, if a user wishes all pixels that are yellow to be changed to red, they can selectively specify  only the yellow pixels to be changed.
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `r_val_min` - The minimum value the R channel must be.
/// * `r_val_max` - The maximum value the R channel must be.
/// 
/// # Example
///
/// ```
/// // For example, to only change the colour of the ranges specified:
/// photon::channels::selective_color_change(img, 100, 120);
/// ```
pub fn selective_color_change(mut img: DynamicImage, ref_color: Rgb, new_color: Rgb) -> DynamicImage {
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {

            let mut px = img.get_pixel(x, y);

            let color: Color = Srgb::new(ref_color.r as f32 / 255.0, ref_color.g as f32 / 255.0, ref_color.b as f32 / 255.0).into_linear().into();
            let lighter = color.lighten(0.1);
            let desaturated = color.desaturate(0.5);

            // Reference colour to compare the current pixel's colour to
            let lab: Lab = Srgb::new(0.9, 0.7, 0.07).into();

            // Convert the current pixel's colour to the l*a*b colour space
            let r_val: f32 = px.data[0] as f32 / 255.0;
            let g_val: f32 = px.data[1] as f32 / 255.0;
            let b_val: f32 = px.data[2] as f32 / 255.0;

            let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();

            let sim = color_sim(lab, px_lab);
            if sim > 0 && sim < 40 {
                px.data[0] = new_color.r;
                px.data[1] = new_color.g;
                px.data[2] = new_color.b;
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

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