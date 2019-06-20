//! Convolution effects such as sharpening, blurs, sobel filters, etc.,

extern crate image;
use wasm_bindgen::prelude::*;
use crate::{PhotonImage};
use crate::helpers;

fn conv(mut photon_image: &mut PhotonImage, kernel: Vec<f32>) {
    let img = helpers::dyn_image_from_raw(&photon_image);

    let filtered_img = img.filter3x3(&kernel);
    photon_image.raw_pixels = filtered_img.raw_pixels();
}

/// Noise reduction. 
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to sharpen an image:
/// use photon::conv::sharpen;
/// photon::channels::sharpen(img);
/// ```
/// Adds a constant to a select R, G, or B channel's value.
#[wasm_bindgen]
pub fn noise_reduction(photon_image: &mut PhotonImage) {
    let kernel = vec![0.0f32, -1.0, 7.0, -1.0, 5.0, 9.0, 0.0, 7.0, 9.0];
    return conv(photon_image, kernel);
}

/// Sharpen an image. 
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to sharpen an image:
/// use photon::conv::sharpen;
/// photon::channels::sharpen(img);
/// ```
/// Adds a constant to a select R, G, or B channel's value.
#[wasm_bindgen]
pub fn sharpen(photon_image: &mut PhotonImage) {
    let kernel = vec![0.0f32, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];
    return conv(photon_image, kernel);
}

/// Apply edge detection to an image, to create a dark version with its edges highlighted.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon::channels;
/// photon::conv::edge_detection(img);
/// ```
#[wasm_bindgen]
pub fn edge_detection(photon_image: &mut PhotonImage) {
    let kernel = vec![-1.0f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
    return conv(photon_image, kernel);
}

/// Apply an identity kernel convolution to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to apply an identity kernel convolution:
/// use photon::channels;
/// photon::conv::identity(img);
/// ```
#[wasm_bindgen]
pub fn identity(photon_image: &mut PhotonImage) {
    let kernel = vec![0.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    return conv(photon_image, kernel);
}

/// Apply a box blur effect.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to apply a box blur effect:
/// use photon::channels;
/// photon::conv::box_blur(img);
/// ```
#[wasm_bindgen]
pub fn box_blur(photon_image: &mut PhotonImage) {
    let kernel = vec![1.0f32, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    return conv(photon_image, kernel);
}

/// Apply a gaussian blur effect.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
///
/// # Example
///
/// ```
/// // For example, to apply a gaussian blur effect to an image:
/// use photon::channels;
/// photon::conv::gaussian_blur(img);
/// ```
#[wasm_bindgen]
pub fn gaussian_blur(photon_image: &mut PhotonImage) {
    let kernel = vec![1.0f32, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0];
    return conv(photon_image, kernel);
}

/// Detect horizontal lines in an image, and highlight these only.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to display the horizontal lines in an image:
/// use photon::channels;
/// photon::conv::detect_horizontal_lines(img);
/// ```
#[wasm_bindgen]
pub fn detect_horizontal_lines(photon_image: &mut PhotonImage) {
    let kernel = vec![-1.0f32, -1.0, -1.0, 2.0, 2.0, 2.0, -1.0, -1.0, -1.0];
    return conv(photon_image, kernel);
}

/// Detect vertical lines in an image, and highlight these only.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to display the vertical lines in an image:
/// use photon::channels;
/// photon::conv::detect_vertical_lines(img);
/// ```
#[wasm_bindgen]
pub fn detect_vertical_lines(photon_image: &mut PhotonImage) {
    let kernel = vec![-1.0f32, 2.0, -1.0, -1.0, 2.0, -1.0, -1.0, 2.0, -1.0];
    return conv(photon_image, kernel);
}

/// Detect lines at a forty five degree angle in an image, and highlight these only.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to display the lines at a forty five degree angle in an image:
/// use photon::channels;
/// photon::conv::detect_fortyfivedeg_lines(img);
/// ```
#[wasm_bindgen]
pub fn detect_45_deg_lines(photon_image: &mut PhotonImage) {
    let kernel = vec![-1.0f32, -1.0, 2.0, -1.0, 2.0, -1.0, 2.0, -1.0, -1.0];
    return conv(photon_image, kernel);
}

/// Detect lines at a 135 degree angle in an image, and highlight these only.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to display the lines at a 135 degree angle in an image:
/// use photon::channels;
/// photon::conv::detect_hundredthirtyfive_deg_lines(img);
/// ```
#[wasm_bindgen]
pub fn detect_135_deg_lines(photon_image: &mut PhotonImage) {
    let kernel = vec![2.0f32, -1.0, -1.0, -1.0, 2.0, -1.0, -1.0, -1.0, 2.0];
    return conv(photon_image, kernel);
}

/// Apply a standard laplace convolution.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to apply a laplace effect:
/// use photon::conv;
/// photon::conv::laplace(img);
/// ```
#[wasm_bindgen]
pub fn laplace(photon_image: &mut PhotonImage) {
    let kernel = vec![0.0f32, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0];
    return conv(photon_image, kernel);
}

/// Preset edge effect.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to apply this effect:
/// use photon::conv;
/// photon::conv::edge_one(img);
/// ```
#[wasm_bindgen]
pub fn edge_one(photon_image: &mut PhotonImage) {
    let kernel = vec![0.0f32, -2.2, -0.6, -0.4, 2.8, -0.3, -0.8, -1.0, 2.7];
    return conv(photon_image, kernel);
}

/// Apply an emboss effect to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to apply an emboss effect:
/// use photon::conv;
/// photon::conv::emboss(img);
/// ```
#[wasm_bindgen]
pub fn emboss(photon_image: &mut PhotonImage) {
    let kernel = vec![-2.0f32, -1.0, 0.0, -1.0, 1.0, 1.0, 0.0, 1.0, 2.0];
    return conv(photon_image, kernel);
}

/// Apply a horizontal Sobel filter to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to apply a horizontal Sobel filter:
/// use photon::conv;
/// photon::conv::sobel_horizontal(img);
/// ```
#[wasm_bindgen]
pub fn sobel_horizontal(photon_image: &mut PhotonImage) {
    let kernel = vec![-1.0f32, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];
    return conv(photon_image, kernel);
}

/// Apply a horizontal Prewitt convolution to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to apply a horizontal Prewitt convolution effect:
/// use photon::conv;
/// photon::conv::prewitt_horizontal(img);
/// ```
#[wasm_bindgen]
pub fn prewitt_horizontal(photon_image: &mut PhotonImage) {
    let kernel = vec![5.0f32, -3.0, -3.0, 5.0, 0.0, -3.0, 5.0, -3.0, -3.0];
    return conv(photon_image, kernel);
}

/// Apply a vertical Sobel filter to an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// 
/// # Example
///
/// ```
/// // For example, to apply a vertical Sobel filter:
/// use photon::conv;
/// photon::conv::sobel_vertical(img);
/// ```
#[wasm_bindgen]
pub fn sobel_vertical(photon_image: &mut PhotonImage) {
    let kernel = vec![-1.0f32, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
    return conv(photon_image, kernel);
}


// // pub fn sobel_color(img: DynamicImage) -> DynamicImage {

// //     let kernel = [-1.0f32, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];

// //     let mut filtered_img = img.filter3x3(&kernel);

// //     let (width, height) = filtered_img.dimensions();

// //     for x in 0..width {
// //         for y in 0..height {
// //             let mut px = filtered_img.get_pixel(x, y);
            
// //             let r_val = px.data[0];
// //             let g_val = px.data[1];
// //             let b_val = px.data[2];
// //             if r_val > 150 {
// //                 let addition = 90;
// //                 px.data[0] = if r_val as u32 + addition < 255 { r_val as u8 + addition as u8} else { 255 };
// //             }
// //             if g_val > 150 {
// //                 let addition = 60;
// //                 px.data[1] = if g_val as u32 + addition < 255 { g_val as u8 + addition as u8} else { 255 };
// //             }
// //             if b_val > 150 {
// //                 let addition = 110;
// //                 px.data[2] = if b_val as u32 + addition < 255 { b_val as u8 + addition as u8} else { 255 };
// //             }

// //             filtered_img.put_pixel(x, y, px);
// //         }
// //     }
// //     return filtered_img;
// // }