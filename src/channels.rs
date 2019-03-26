extern crate image;
use image::{GenericImage, DynamicImage, GenericImageView};

/// Alter a select channel by incrementing its value by a constant.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `channel` - The channel you wish to alter, it should be either 0, 1 or 2, 
/// representing R, G, or B respectively
/// * `offset` - The amount you want to increment the channel's value by for that pixel.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon::channels;
/// photon::channels::alter_channel(img, 0, 10);
/// ```
/// Adds a constant to a select R, G, or B channel's value.
pub fn alter_channel(mut img: DynamicImage, channel: usize, offset: u8) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if px.data[channel] <= 255 - offset {
                px.data[channel] += offset;
            }
            else {
                px.data[channel] = 255;
            }
            img.put_pixel(x, y, px)
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
/// photon::channels::alter_red_channel(img, 10);
/// ```
pub fn alter_red_channel(img: DynamicImage, offset: u8) -> DynamicImage {
    let res_img = alter_channel(img, 0, offset);
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
/// photon::channels::alter_green_channel(img, 10);
/// ```
pub fn alter_green_channel(img: DynamicImage, offset: u8) -> DynamicImage {
    let res_img = alter_channel(img, 1, offset);
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
/// photon::channels::alter_blue_channel(img, 10);
/// ```
pub fn alter_blue_channel(img: DynamicImage, offset: u8) -> DynamicImage {
    let res_img = alter_channel(img, 2, offset);
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
/// photon::channels::alter_two_channels(img, 0, 10, 2, 20);
/// ```
pub fn alter_two_channels(mut img: DynamicImage, channel1: usize, offset1: u8, channel2: usize, offset2: u8) -> DynamicImage {
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if px.data[channel1] <= 255 - offset1 {
                px.data[channel1] += offset1;
            }
            else {
                px.data[channel1] = 255;
            }
                
            if px.data[channel2] <= 255 - offset2 {
                px.data[channel2] += offset2;
            }
            else {
                px.data[channel2] = 255
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Set a certain channel to zero, thus removing the channel's influence in the pixels' final colours.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter. 
/// * `channel` - The channel to be removed; must be a usize from 0 to 2, with 0 representing Red, 1 representing Green, and 2 representing Blue.
/// 
/// # Example
///
/// ```
/// // For example, to remove the Red channel with a min_filter of 100:
/// photon::channels::remove_channel(img, 100, 0);
/// ```
pub fn remove_channel(mut img: DynamicImage, min_filter: u8, channel: usize) -> DynamicImage {
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
    let filtered_img = remove_channel(img, min_filter, 0);
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
    let filtered_img = remove_channel(img, min_filter, 1);
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
    let filtered_img = remove_channel(img, min_filter, 2);
    return filtered_img;
}