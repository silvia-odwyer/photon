/// Preset filters you can apply to images.
extern crate image;

use image::{GenericImage, DynamicImage, GenericImageView};

/// Add an aquamarine-tinted hue to an image.
pub fn oceanic(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::alter_two_channels(img, 1, 9, 2, 173);
    return filtered_img;
}

pub fn islands(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::alter_two_channels(img, 1, 24, 2, 95);
    return filtered_img;
}

/// Add a green/blue mixed hue to an image.
pub fn marine(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::alter_two_channels(img, 1, 14, 2, 119);
    return filtered_img;
}

/// Dark green hue, with tones of blue.
pub fn seagreen(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::alter_two_channels(img, 1, 68, 2, 62);
    return filtered_img;
}

/// Royal blue tint
pub fn flagblue(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::alter_blue_channel(img, 131);
    return filtered_img;
}

pub fn diamante(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::alter_two_channels(img, 1, 82, 2, 87);
    return filtered_img;
}

pub fn liquid(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::channels::alter_two_channels(img, 1, 10, 2, 75);
    return filtered_img;
}

pub fn solange(mut img: DynamicImage) -> DynamicImage {

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if 200 as i32 - px.data[0] as i32 > 0 {
                px.data[0] = 200 - px.data[0];
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

pub fn neue(mut img: DynamicImage) -> DynamicImage {

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if 255 as i32 - px.data[2] as i32 > 0 {
                px.data[2] = 255 - px.data[2];
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

pub fn lix(mut img: DynamicImage) -> DynamicImage {

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            
            px.data[0] = 255 - px.data[0];
            px.data[1] = 255 - px.data[1];
            
            img.put_pixel(x, y, px);
        }
    }
    return img;
}


pub fn ryo(mut img: DynamicImage) -> DynamicImage {

    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if 255 as i32 - px.data[2] as i32 > 0 {
                px.data[0] = 255 - px.data[0];
                px.data[2] = 255 - px.data[2];
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

pub fn radio(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome(img, 5, 40, 20);
    return filtered_img;
}

pub fn twenties(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome(img, 18, 12, 20);
    return filtered_img;
}

pub fn rosetint(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome(img, 80, 20, 31);
    return filtered_img;
}

pub fn mauve(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome(img, 90, 40, 80);
    return filtered_img;
}

pub fn bluechrome(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::monochrome(img, 20, 30, 60);
    return filtered_img;
}

pub fn vintage(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::tint(img, 120, 70, 13);
    return filtered_img;
}

pub fn perfume(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::tint(img, 80, 40, 120);
    return filtered_img;
}

pub fn serenity(img: DynamicImage) -> DynamicImage {
    let filtered_img = crate::tint(img, 10, 40, 90);
    return filtered_img;
}