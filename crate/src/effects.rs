extern crate image;
use image::{GenericImage, GenericImageView};
use std::f64;
extern crate imageproc;
extern crate rusttype;
use crate::{PhotonImage, Rgb};
use crate::helpers;
use wasm_bindgen::prelude::*;
 
/// Adds an offset to the image by a certain number of pixels. 
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset is added to the pixels in the image.  
/// # Example
///
/// ```
/// // For example, to offset pixels by 30 pixels on the red channel:
/// use photon::effects;
/// photon::effects::offset(img, 0, 30);
/// ```
#[wasm_bindgen]
pub fn offset(mut photon_image: &mut PhotonImage, channel_index: usize, offset: u32) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {

            let mut px = img.get_pixel(x, y);

            if x + offset < width - 1 && y + offset < height - 1  {

                let offset_px = img.get_pixel(x + offset, y + offset);

                px.data[channel_index] = offset_px.data[channel_index];
                
            }
            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Adds an offset to the red channel by a certain number of pixels. 
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset you want to move the red channel by. 
/// # Example
///
/// ```
/// // For example, to add an offset to the red channel by 30 pixels.
/// use photon::effects;
/// photon::effects::offset_red(img, 30);
/// ```
#[wasm_bindgen]
pub fn offset_red(img: &mut PhotonImage, offset_amt: u32) {
    offset(img, 0, offset_amt)
}

/// Adds an offset to the green channel by a certain number of pixels. 
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset you want to move the green channel by.
/// # Example
///
/// ```
/// // For example, to add an offset to the green channel by 30 pixels.
/// use photon::effects;
/// photon::effects::offset_green(img, 40);
/// ```
#[wasm_bindgen]
pub fn offset_green(img: &mut PhotonImage, offset_amt: u32) {
    offset(img, 1, offset_amt)
}

/// Adds an offset to the blue channel by a certain number of pixels. 
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset_amt` - The offset you want to move the blue channel by.
/// # Example
/// // For example, to add an offset to the green channel by 40 pixels.
/// use photon::effects;
/// photon::effects::offset_blue(img, 40);
/// ```
#[wasm_bindgen]
pub fn offset_blue(img: &mut PhotonImage, offset_amt: u32) {
    offset(img, 2, offset_amt)
}

/// Adds multiple offsets to the image by a certain number of pixels (on two channels).
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `offset` - The offset is added to the pixels in the image.  
/// # Example
///
/// ```
/// // For example, to add a 30-pixel offset to both the red and blue channels:
/// use photon::effects;
/// photon::effects::multiple_offsets(img, 30, 0, 2);
/// ```
#[wasm_bindgen]
pub fn multiple_offsets(mut photon_image: &mut PhotonImage, offset: u32, channel_index: usize, channel_index2: usize) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {

            let mut px = img.get_pixel(x, y);

            if x + offset < width - 1 && y + offset < height - 1  {

                let offset_px = img.get_pixel(x + offset, y);

                px.data[channel_index] = offset_px.data[channel_index];
                
            }
            
            if x as i32 - offset as i32 > 0 && y as i32 - offset as i32 > 0  {
                let offset_px2 = img.get_pixel(x - offset, y );

                px.data[channel_index2] = offset_px2.data[channel_index2];
                
            }


            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Add a sine wave animation to the pixels by distributing the pixels along a sine curve.
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to threshold an image of type `DynamicImage`:
/// use photon::effects;
/// photon::effects::ripple(img);
/// ```
// pub fn ripple(mut img: DynamicImage) -> DynamicImage {
//     let (width, height) = img.dimensions();
    
//     let xoff = width / 3; 
//     let yoff = height / 3;

//     for x in 0..width {
//         for y in 0..height {

//             let mut px = img.get_pixel(x, y);
//             // // calculate sine based on distance
//             // x2 = x - xoff;
//             // y2 = y - yoff;
//             // d = Math.sqrt(x2*x2 + y2*y2);
//             // t = Math.sin(d/6.0);
//             let x2: f64 = x as f64 - xoff as f64;
//             let y2: f64 = y as f64 - yoff as f64;

//             let res: f64 = x2*x2  + y2*y2;
//             let d = (res).sqrt();
// 		    let t = (d/6.0).sin();

//             let r = t * 200.0;
// 		    let g = 125.0 + t * 80.0;
// 		    let b = 235.0 + t * 20.0;
            
//             px.data[0] = cmp::max(0, cmp::min(255, r as u8)) as u8;
//             px.data[1] = cmp::max(0, cmp::min(255, g as u8)) as u8;
//             px.data[2] = cmp::max(0, cmp::min(255, b as u8)) as u8;
//             img.put_pixel(x, y, px);
//         }
//     }
//     return img;
// }

/// Create a gradient map between two RGB colours.
/// 
/// # Arguments
/// * `color_a`: An RGB color
/// * `color_b`: An RGB color
pub fn create_gradient_map(color_a : Rgb, color_b: Rgb) -> Vec<Rgb> {
    let mut gradient_map = vec![];

    let max_val = 255;
    let mut r_val = 0;

    for i in 0..max_val + 1{
        let intensity_b = max_val - i;

        r_val = (i * color_a.r + intensity_b * color_b.r) / max_val as u8;
        gradient_map.push(Rgb {
            r: r_val , 
            g: (i * color_a.g + intensity_b * color_b.g) / max_val as u8,
            b: (i * color_a.b + intensity_b * color_b.b) / max_val as u8
        });
    }
    return gradient_map;
}

/// Pass the image through a duotone filter (comprising two colours, and their gradient from one colour to the other).
/// This is similar to greyscaling an image, but rather than having the gradient transition from black to white, it should
/// be between two other colours, red to green, for example. 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `color_a` - An RGB color. 
/// * `color_b` - An RGB color.
/// # Example
///
/// ```
/// // For example, to pass an image through a duotone filter:
/// use photon::effects;
/// photon::effects::duotone(img, colour_a, colour_b);
/// ```
#[wasm_bindgen]
pub fn duotone(mut photon_image: &mut PhotonImage, color_a : Rgb, color_b : Rgb) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    let gradient_map = create_gradient_map(color_a, color_b);

    for x in 0..width {
        for y in 0..height {

            let mut px = img.get_pixel(x, y);

            let r = px.data[0];
            let g = px.data[1];
            let b = px.data[2];
            
            px.data[0] = gradient_map[r as usize].r as u8;
            px.data[1] = gradient_map[g as usize].g as u8;
            px.data[2] = gradient_map[b as usize].b as u8;

            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

pub fn halftone(mut photon_image: PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    

    for x in (0..width).step_by(2 as usize) {
        for y in (0..height).step_by(2 as usize) {

            let mut px1 = img.get_pixel(x, y);
            let mut px2 = img.get_pixel(x, y + 1);
            let mut px3 = img.get_pixel(x + 1, y);
            let mut px4 = img.get_pixel(x + 1, y + 1);

            let gray1 = (px1[0] as f64 * 0.299) + (px1[1] as f64 * 0.587) + (px1[2] as f64 * 0.114);
            let gray2 = (px2[0] as f64 * 0.299) + (px2[1] as f64 * 0.587) + (px2[2] as f64 * 0.114);
            let gray3 = (px3[0] as f64 * 0.299) + (px3[1] as f64 * 0.587) + (px3[2] as f64 * 0.114);            
            let gray4 = (px4[0] as f64 * 0.299) + (px4[1] as f64 * 0.587) + (px4[2] as f64 * 0.114);

            let sat = (gray1 + gray2 + gray3 + gray4) / 4.0;

            if sat > 200.0 {
                px1.data[0] = 255;
                px1.data[1] = 255;
                px1.data[2] = 255;

                px2.data[0] = 255;
                px2.data[1] = 255;
                px2.data[2] = 255;

                px3.data[0] = 255;
                px3.data[1] = 255;
                px3.data[2] = 255;

                px4.data[0] = 255;
                px4.data[1] = 255;
                px4.data[2] = 255;

            }

            else if sat > 159.0 {
                px1.data[0] = 255;
                px1.data[1] = 255;
                px1.data[2] = 255;

                px2.data[0] = 0;
                px2.data[1] = 0;
                px2.data[2] = 0;

                px3.data[0] = 255;
                px3.data[1] = 255;
                px3.data[2] = 255;

                px4.data[0] = 255;
                px4.data[1] = 255;
                px4.data[2] = 255;
            }

            else if sat > 95.0 {
                px1.data[0] = 255;
                px1.data[1] = 255;
                px1.data[2] = 255;

                px2.data[0] = 0;
                px2.data[1] = 0;
                px2.data[2] = 0;

                px3.data[0] = 0;
                px3.data[1] = 0;
                px3.data[2] = 0;

                px4.data[0] = 255;
                px4.data[1] = 255;
                px4.data[2] = 255;
            }

            else if sat > 32.0 {
                px1.data[0] = 0;
                px1.data[1] = 0;
                px1.data[2] = 0;

                px2.data[0] = 255;
                px2.data[0] = 255;
                px2.data[0] = 255;

                px3.data[0] = 0;
                px3.data[1] = 0;
                px3.data[2] = 0;                
                
                px4.data[0] = 0;
                px4.data[1] = 0;
                px4.data[2] = 0;
            }

            else {
                px1.data[0] = 0;
                px1.data[1] = 0;
                px1.data[2] = 0;                
                
                px2.data[0] = 0;
                px2.data[1] = 0;
                px2.data[2] = 0;                
                
                px3.data[0] = 0;
                px3.data[1] = 0;
                px3.data[2] = 0;

                px4.data[0] = 0;
                px4.data[1] = 0;
                px4.data[2] = 0;
            }


            img.put_pixel(x, y, px1);
            // img.put_pixel(x, y + 1, px2);
         }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Reduces an image to the primary colours.
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to add a primary colour effect to an image of type `DynamicImage`:
/// use photon::effects;
/// photon::effects::primary(img);
/// ```
#[wasm_bindgen]
pub fn primary(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width{
        for y in 0..height {

            let mut px = img.get_pixel(x, y);

            let mut r_val = px.data[0];
            let mut g_val = px.data[1];
            let mut b_val = px.data[2];

            if r_val > 128 {
                r_val = 255;
            }

            else {
                r_val = 0;
            }

            if g_val > 128 {
                g_val = 255;
            }
            else {
                g_val = 0;
            }

            if b_val > 128 {
                g_val = 255;
            }
            else {
                b_val = 0;
            }

            px.data[0] = r_val;
            px.data[1] = g_val;
            px.data[2] = b_val;

            img.put_pixel(x, y, px);
         }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Colorizes the green channels of the image.
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to colorize an image of type `DynamicImage`:
/// use photon::effects;
/// photon::effects::colorize(img);
/// ```
#[wasm_bindgen]
pub fn colorize(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let threshold = 220;
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let px_as_rgb = Rgb{r: px.data[0], g: px.data[1], b: px.data[2]};

            let baseline_color = Rgb{r: 0, g: 255, b: 255};

            let square_distance = crate::helpers::square_distance(baseline_color, px_as_rgb);

            let mut r = px.data[0] as f32;
            let mut g = px.data[1] as f32;
            let mut b = px.data[2] as f32;

            if square_distance < i32::pow(threshold, 2) {
                r = r * 0.5;
                g = g * 1.25;
                b = b * 0.5;
            }

            px.data[0] = r as u8;
            px.data[1] = g as u8;
            px.data[2] = b as u8;

            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

// #[wasm_bindgen]
// pub fn inc_luminosity(mut photon_image: PhotonImage) -> PhotonImage {
//     let mut img = helpers::dyn_image_from_raw(&photon_image);
//     let (width, height) = img.dimensions();
//     let mut min_intensity = 255;
//     let mut max_intensity = 0;

//     // find the max and min intensities in the image
//     for x in 0..width {
//         for y in 0..height {
//             let px = img.get_pixel(x, y);
//             let intensity = (px.data[0] as u32 + px.data[1] as u32 + px.data[2] as u32) / 3;
//             if intensity > 0{
//                 min_intensity = cmp::min(min_intensity, intensity);
//                 max_intensity = cmp::max(max_intensity, intensity);
//             }
            
//         }
//     }

//     for x in 0..width {
//         for y in 0..height {
//             let mut px = img.get_pixel(x, y);
//             // let px_as_rgb = Rgb{r: px.data[0], g: px.data[1], b: px.data[2]};

//             let mut r = px.data[0] as f32;
//             let mut g = px.data[1] as f32;
//             let mut b = px.data[2] as f32;

//             let lum = (r + g + b) / 3.0;

//             let new_lum = 255.0 * (lum - min_intensity as f32) / (max_intensity / min_intensity) as f32;

//             r = r * new_lum / lum;
//             g = g * new_lum / lum;
//             b = b * new_lum / lum;

//             px.data[0] = r as u8;
//             px.data[1] = g as u8;
//             px.data[2] = b as u8;

//             img.put_pixel(x, y, px);
//         }
//     }
//     let mut raw_pixels = img.raw_pixels();
//     photon_image.raw_pixels = raw_pixels;
//     return photon_image;
// }

/// Applies a solarizing effect to an image.
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to colorize an image of type `DynamicImage`:
/// use photon::effects;
/// photon::effects::solarize(img);
/// ```
#[wasm_bindgen]
pub fn solarize(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
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
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}


/// Increase the brightness of an image by a factor.
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `brightness` - A u8 to add to the brightness.
/// # Example
///
/// ```
/// photon::effects::inc_brightness(img, 10);
/// ```
#[wasm_bindgen]
pub fn inc_brightness(mut photon_image: &mut PhotonImage, brightness: u8) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if px.data[0] <= 255 - brightness {
                px.data[0] += brightness;
            }
            else {
                px.data[0] = 255;
            }            
            if px.data[1] <= 255 - brightness {
                px.data[1] += brightness;
            }

            else {
                px.data[1] = 255
            }

            if px.data[2] <= 255 - brightness {
                px.data[2] += brightness;
            }

            else {
                px.data[2] = 255
            }

            img.put_pixel(x, y, px);
        }
    }
    photon_image.raw_pixels = img.raw_pixels();
}

/// Tint an image by adding an offset to averaged RGB channel values.
/// 
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// * `r_offset` - The amount the  R channel should be incremented by.
/// * `g_offset` - The amount the G channel should be incremented by.
/// * `b_offset` - The amount the B channel should be incremented by.
/// # Example
///
/// ```
/// // For example, to tint an image of type `DynamicImage`:
/// photon::tint(img, 10, 20, 15);
/// ```
/// 
#[wasm_bindgen]
pub fn tint(mut photon_image: &mut PhotonImage, r_offset: u32, g_offset: u32, b_offset: u32) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            px.data[0] = if r_val as u32 + r_offset < 255 { r_val as u8 + r_offset as u8} else { 255 };
            px.data[1] = if g_val as u32 + g_offset < 255 { g_val as u8 + g_offset as u8} else { 255 };
            px.data[2] = if b_val as u32 + b_offset < 255 { b_val as u8 + b_offset as u8} else { 255 };

            img.put_pixel(x, y, px);
        }
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}