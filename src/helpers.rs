extern crate image;
use crate::effects::Rgb;
use image::{GenericImage, DynamicImage, GenericImageView};

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