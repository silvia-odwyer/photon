extern crate image;
use crate::effects::Rgb;
use crate::filters;
use image::{DynamicImage, ImageBuffer};

// Gets the square distance between two colours
pub fn square_distance(color1 : Rgb, color2 : Rgb) -> i32{
    let (r1, g1, b1) = (color1.r as i32, color1.g as i32, color1.b as i32);
    let (r2, g2, b2) = (color2.r as i32, color2.g as i32, color2.b as i32);
    return i32::pow(r1 - r2, 2) + i32::pow(g1 - g2, 2) + i32::pow(b1 - b2, 2);
}

pub fn open_image(img_path: &'static str) -> DynamicImage {
    let img = image::open(img_path).unwrap();
    
    return img;
}

pub fn save_image(img: DynamicImage, filtered_img_path: &'static str) {
    img.save(filtered_img_path).unwrap();
}

pub fn get_pixels(img_path: &'static str) -> Vec<u8>{
    // get an image's raw pixels, and return as a vec of u8s
    let image = image::open(img_path).unwrap();
    let raw_pixels: Vec<u8> = image.raw_pixels();
    raw_pixels
}

pub fn dyn_image_from_raw(raw_pixels: Vec<u8>, width: u32, height: u32) -> DynamicImage {
    // convert a vec of raw pixels (as u8s) to a DynamicImage type 
    let len_vec = raw_pixels.len() as u128;

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = image::ImageRgb8(img_buffer);
    dynimage
}