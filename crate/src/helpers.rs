extern crate image;
use image::{DynamicImage, ImageBuffer, GenericImageView};
use crate::{PhotonImage, Rgb};

// Gets the square distance between two colours
pub fn square_distance(color1 : Rgb, color2 : Rgb) -> i32{
    let (r1, g1, b1) = (color1.r as i32, color1.g as i32, color1.b as i32);
    let (r2, g2, b2) = (color2.r as i32, color2.g as i32, color2.b as i32);
    return i32::pow(r1 - r2, 2) + i32::pow(g1 - g2, 2) + i32::pow(b1 - b2, 2);
}

pub fn open_image(img_path: &'static str) -> PhotonImage {
    let img = image::open(img_path).unwrap();

    let (width, height) = img.dimensions();

    // Convert the DynamicImage type to raw vec representing RGBA pixels (not RGB)
    let raw_pixels = img.to_rgba().to_vec();

    let photon_image: PhotonImage = PhotonImage {raw_pixels: raw_pixels, width: width, height: height};
    return photon_image;
}

pub fn save_image(img: PhotonImage, filtered_img_path: &str) {
    let raw_pixels = img.raw_pixels;
    let width = img.width;
    let height = img.height;

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = image::ImageRgba8(img_buffer);
    
    dynimage.save(filtered_img_path).unwrap();
}

pub fn get_pixels(img: DynamicImage) -> Vec<u8>{
    // get an image's raw pixels, and return as a vec of u8s
    let raw_pixels: Vec<u8> = img.raw_pixels();
    raw_pixels
}

pub fn dyn_image_from_raw(photon_image: &PhotonImage) -> DynamicImage {
    // convert a vec of raw pixels (as u8s) to a DynamicImage type 
    let _len_vec = photon_image.raw_pixels.len() as u128;
    let raw_pixels = &photon_image.raw_pixels;
    let img_buffer = ImageBuffer::from_vec(photon_image.width, photon_image.height, raw_pixels.to_vec()).unwrap();
    let dynimage = image::ImageRgba8(img_buffer);
    dynimage
}