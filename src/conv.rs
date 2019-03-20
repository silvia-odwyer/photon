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
pub fn sharpen(mut img: DynamicImage) -> DynamicImage {
    let kernel = [0.0f32, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
}


pub fn edge_detection(mut img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
}

pub fn identity(mut img: DynamicImage) -> DynamicImage {
    let kernel = [0.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
}

pub fn box_blur(mut img: DynamicImage) -> DynamicImage {
    let kernel = [1.0f32, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
}

pub fn gaussian_blur(mut img: DynamicImage) -> DynamicImage {
    let kernel = [1.0f32, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
}

pub fn detect_horizontal_lines(img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, -1.0, -1.0, 2.0, 2.0, 2.0, -1.0, -1.0, -1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
}

pub fn detect_vertictal_lines(img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, 2.0, -1.0, -1.0, 2.0, -1.0, -1.0, 2.0, -1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
}

pub fn detect_fortyfivedeg_lines(img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, -1.0, 2.0, -1.0, 2.0, -1.0, 2.0, -1.0, -1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
}

