extern crate image;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use image::Pixel;
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

pub fn alter_red_channel(mut img: DynamicImage, offset: u8) -> DynamicImage {
    let res_img = alter_channel(img, 0, offset);
    return res_img;
}

pub fn alter_green_channel(mut img: DynamicImage, offset: u8) -> DynamicImage {
    let res_img = alter_channel(img, 1, offset);
    return res_img;
}

pub fn alter_blue_channel(mut img: DynamicImage, offset: u8) -> DynamicImage {
    let res_img = alter_channel(img, 2, offset);
    return res_img;
}

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