extern crate image;
use image::{GenericImage, DynamicImage, GenericImageView};
extern crate palette;
use palette::{Srgb, Lab};
use palette::{Pixel, Lch, Shade, Saturate};
use crate::effects::{Rgb};
use crate::channels::palette::Hue;

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
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
/// * `new_color` - The `RGB` value of the new color to replace the reference color.
/// 
/// # Example
///
/// ```
/// // For example, to only change those pixels that are of RGB value RGB{20, 40, 60} and replace 
/// // them with RGB value{90, 30, 10}:
/// let ref_color = Rgb{20, 40, 60};
/// let new_color = Rgb{90, 30, 10};
/// photon::channels::selective_color_change(img, ref_color, new_color);
/// ```
pub fn selective_color_change(mut img: DynamicImage, ref_color: Rgb, new_color: Rgb) -> DynamicImage {
    let (_width, _height) = img.dimensions();
    for x in 0.._width {
        for y in 0.._height {
            let mut px = img.get_pixel(x, y);

            // Reference colour to compare the current pixel's colour to
            let lab: Lab = Srgb::new(ref_color.r as f32 / 255.0, ref_color.g as f32 / 255.0, ref_color.b as f32 / 255.0).into();

            // Convert the current pixel's colour to the l*a*b colour space
            let r_val: f32 = px.data[0] as f32 / 255.0;
            let g_val: f32 = px.data[1] as f32 / 255.0;
            let b_val: f32 = px.data[2] as f32 / 255.0;

            let px_lab: Lab = Srgb::new(r_val, g_val, b_val).into();

            let sim = color_sim(lab, px_lab);
            if sim > 0 && sim < 30 {
                px.data[0] = new_color.r;
                px.data[1] = new_color.g;
                px.data[2] = new_color.b;
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Selective hue rotation.
/// Only rotate the hue of a pixel if its RGB values are within a specified range.
/// This function only rotates a pixel's hue to another  if it is visually similar to the colour specified.
/// For example, if a user wishes all pixels that are yellow to be changed to red, they can selectively specify  only the yellow pixels to be changed.
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
/// * `degrees` - The amount of degrees to hue rotate by.
/// 
/// # Example
///
/// ```
/// // For example, to only rotate the pixels that are of RGB value RGB{20, 40, 60}:
/// let ref_color = Rgb{20, 40, 60};
/// photon::channels::selective_hue_rotate(img, ref_color, 180);
/// ```
pub fn selective_hue_rotate(img: DynamicImage, ref_color: Rgb, degrees: f32) -> DynamicImage {
    let (_width, _height) = img.dimensions();
    let mut img = img.to_rgb();
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
                let color = Srgb::from_raw(&px_data).into_format();

                let hue_rotated_color = Lch::from(color).shift_hue(degrees);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(hue_rotated_color.into()).into_format().into_raw()
                });
            }
        }
    }
    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

/// Selectively lighten an image.
pub fn selective_lighten(img: DynamicImage, ref_color: Rgb, amt: u8) -> DynamicImage {
    let (_width, _height) = img.dimensions();
    let mut img = img.to_rgb();
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
                let color = Srgb::from_raw(&px_data).into_format();

                let lightened = Lch::from(color).lighten(0.1);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(lightened.into()).into_format().into_raw()
                });
            }
        }
    }
    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

/// Selectively desaturate pixel colours which are similar to the reference colour provided.
/// Similarity between two colours is calculated via the CIE76 formula.
/// Only desaturates the hue of a pixel if its similarity to the reference colour is within the range in the algorithm.
/// For example, if a user wishes all pixels that are yellow to be desaturated by 0.1, they can selectively specify  only the yellow pixels to be changed.
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
/// * `amt` - The amount of desaturate the colour by. 
/// 
/// # Example
///
/// ```
/// // For example, to only desaturate the pixels that are similar to the RGB value RGB{20, 40, 60}:
/// let ref_color = Rgb{20, 40, 60};
/// photon::channels::selective_desaturate(img, ref_color, 0.1);
/// ```
pub fn selective_desaturate(mut img: DynamicImage, ref_color: Rgb, amt: f32) -> DynamicImage {
    let (_width, _height) = img.dimensions();
    let mut img = img.to_rgb();
    for x in 0.._width {
        for y in 0.._height {
            let mut px = img.get_pixel(x, y);

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

                let desaturated = lch_colour.desaturate(amt);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(desaturated.into()).into_format().into_raw()
                });        
            }
        }
    }
    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

/// Selectively saturate pixel colours which are similar to the reference colour provided.
/// Similarity between two colours is calculated via the CIE76 formula.
/// Only saturates the hue of a pixel if its similarity to the reference colour is within the range in the algorithm.
/// For example, if a user wishes all pixels that are yellow to have an increase in saturation by 10%, they can selectively specify only the yellow pixels to be changed.
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `ref_color` - The `RGB` value of the reference color (to be compared to)
/// * `amt` - The amount of saturate the colour by. 
/// 
/// # Example
///
/// ```
/// // For example, to only increase the saturation of pixels that are similar to the RGB value RGB{20, 40, 60}:
/// let ref_color = Rgb{20, 40, 60};
/// photon::channels::selective_saturate(img, ref_color, 0.1);
/// ```
pub fn selective_saturate(mut img: DynamicImage, ref_color: Rgb, amt: f32) -> DynamicImage {
    let img = selective(img, "saturate", ref_color, amt);
    img
}

fn selective(mut img: DynamicImage, mode: &'static str, ref_color:Rgb, amt: f32) -> DynamicImage {
    let (_width, _height) = img.dimensions();
    let mut img = img.to_rgb();
    for x in 0.._width {
        for y in 0.._height {
            let mut px = img.get_pixel(x, y);

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
            
                img.put_pixel(x, y, image::Rgb {
                        data: Srgb::from_linear(new_color.into()).into_format().into_raw()
                });

            }
        }
    }
    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

/// Selective colour change.
/// Only changes the colour of a pixel if its RGB values are within a specified range.
/// This function only changes a pixel's colour to another colour if it is visually similar to the colour specified.
pub fn selective_monochrome(mut img: DynamicImage, ref_color: Rgb) -> DynamicImage {
    let (_width, _height) = img.dimensions();
    for x in 0.._width {
        for y in 0.._height {
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
    return img;
}


// Get the similarity of two colours in the l*a*b colour space using the CIE76 formula.
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