//! Special effects.

extern crate image;
use image::{GenericImage, GenericImageView};
use std::f64;
extern crate imageproc;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
extern crate rusttype;
use crate::helpers;
use crate::{PhotonImage, Rgb};
use image::Rgba;
use wasm_bindgen::prelude::*;
use crate::iter::ImageIterator;

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
pub fn offset(photon_image: &mut PhotonImage, channel_index: usize, offset: u32) {
    if channel_index > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }

    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in 0..width - 10 {
        for y in 0..height - 10 {
            let mut px = img.get_pixel(x, y);

            if x + offset < width - 1 && y + offset < height - 1 {
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
///
/// ```
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
pub fn multiple_offsets(
    mut photon_image: &mut PhotonImage,
    offset: u32,
    channel_index: usize,
    channel_index2: usize,
) {
    if channel_index > 2 {
        panic!("Invalid channel index passed. Channel1 must be equal to 0, 1, or 2.");
    }
    if channel_index2 > 2 {
        panic!("Invalid channel index passed. Channel2 must be equal to 0, 1, or 2.");
    }
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for (x, y) in ImageIterator::new(width, height) {
        let mut px = img.get_pixel(x, y);

        if x + offset < width - 1 && y + offset < height - 1 {
            let offset_px = img.get_pixel(x + offset, y);

            px.data[channel_index] = offset_px.data[channel_index];
        }

        if x as i32 - offset as i32 > 0 && y as i32 - offset as i32 > 0 {
            let offset_px2 = img.get_pixel(x - offset, y);

            px.data[channel_index2] = offset_px2.data[channel_index2];
        }

        img.put_pixel(x, y, px);
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Halftoning effect.
pub fn halftone(mut photon_image: PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    for x in (0..width).step_by(2 as usize) {
        for y in (0..height).step_by(2 as usize) {
            let mut px1 = img.get_pixel(x, y);
            let mut px2 = img.get_pixel(x, y + 1);
            let mut px3 = img.get_pixel(x + 1, y);
            let mut px4 = img.get_pixel(x + 1, y + 1);

            let gray1 = (px1[0] as f64 * 0.299)
                + (px1[1] as f64 * 0.587)
                + (px1[2] as f64 * 0.114);
            let gray2 = (px2[0] as f64 * 0.299)
                + (px2[1] as f64 * 0.587)
                + (px2[2] as f64 * 0.114);
            let gray3 = (px3[0] as f64 * 0.299)
                + (px3[1] as f64 * 0.587)
                + (px3[2] as f64 * 0.114);
            let gray4 = (px4[0] as f64 * 0.299)
                + (px4[1] as f64 * 0.587)
                + (px4[2] as f64 * 0.114);

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
            } else if sat > 159.0 {
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
            } else if sat > 95.0 {
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
            } else if sat > 32.0 {
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
            } else {
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
pub fn primary(img: &mut PhotonImage) {
    let end = img.raw_pixels.len() - 4;

    for i in (0..end).step_by(4) {
        let mut r_val = img.raw_pixels[0];
        let mut g_val = img.raw_pixels[1];
        let mut b_val = img.raw_pixels[2];

        if r_val > 128 {
            r_val = 255;
        } else {
            r_val = 0;
        }

        if g_val > 128 {
            g_val = 255;
        } else {
            g_val = 0;
        }

        if b_val > 128 {
            g_val = 255;
        } else {
            b_val = 0;
        }

        img.raw_pixels[i] = r_val;
        img.raw_pixels[i + 1] = g_val;
        img.raw_pixels[i + 2] = b_val;
    }
}

/// Colorizes the green channels of the image.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to colorize an image of type `PhotonImage`:
/// use photon::effects;
/// photon::effects::colorize(img);
/// ```
#[wasm_bindgen]
pub fn colorize(mut photon_image: &mut PhotonImage) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let threshold = 220;

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let px_as_rgb = Rgb {
            r: px.data[0],
            g: px.data[1],
            b: px.data[2],
        };

        let baseline_color = Rgb {
            r: 0,
            g: 255,
            b: 255,
        };

        let square_distance =
            crate::helpers::square_distance(baseline_color, px_as_rgb);

        let mut r = px.data[0] as f32;
        let mut g = px.data[1] as f32;
        let mut b = px.data[2] as f32;

        if square_distance < i32::pow(threshold, 2) {
            r *= 0.5;
            g *= 1.25;
            b *= 0.5;
        }

        px.data[0] = r as u8;
        px.data[1] = g as u8;
        px.data[2] = b as u8;

        img.put_pixel(x, y, px);
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
/// // For example, to colorize an image of type `PhotonImage`:
/// use photon::effects;
/// photon::effects::solarize(img);
/// ```
#[wasm_bindgen]
pub fn solarize(photon_image: &mut PhotonImage) {
    let end = photon_image.get_raw_pixels().len() - 4;

    for i in (0..end).step_by(4) {
        let r_val = photon_image.raw_pixels[i];

        if 200 as i32 - r_val as i32 > 0 {
            photon_image.raw_pixels[i] = 200 - r_val;
        }
    }
}

/// Applies a solarizing effect to an image and returns the resulting PhotonImage.
///
/// # Arguments
/// * `img` - A PhotonImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to solarize an image of type `PhotonImage`:
/// use photon::effects;
/// let img = photon::effects::solarize(img);
/// ```
#[wasm_bindgen]
pub fn solarize_retimg(photon_image: &PhotonImage) -> PhotonImage {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        if 200 as i32 - px.data[0] as i32 > 0 {
            px.data[0] = 200 - px.data[0];
        }
        img.put_pixel(x, y, px);
    }
    PhotonImage {
        raw_pixels: img.raw_pixels(),
        width: img.width(),
        height: img.height(),
    }
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
pub fn inc_brightness(photon_image: &mut PhotonImage, brightness: u8) {
    let end = photon_image.get_raw_pixels().len() - 4;

    for i in (0..end).step_by(4) {
        let r_val = photon_image.raw_pixels[i];
        let g_val = photon_image.raw_pixels[i + 1];
        let b_val = photon_image.raw_pixels[i + 2];

        if r_val <= 255 - brightness {
            photon_image.raw_pixels[i] += brightness;
        } else {
            photon_image.raw_pixels[i] = 255;
        }
        if g_val <= 255 - brightness {
            photon_image.raw_pixels[i + 1] += brightness;
        } else {
            photon_image.raw_pixels[1] = 255
        }

        if b_val <= 255 - brightness {
            photon_image.raw_pixels[i + 2] += brightness;
        } else {
            photon_image.raw_pixels[i + 2] = 255
        }
    }
}

/// Adjust the contrast of an image by a factor.
///
/// # Arguments
/// * `photon_image` - A PhotonImage that contains a view into the image.
/// * `contrast` - An f32 factor used to adjust contrast. Between [-255.0, 255.0]. The algorithm will
/// clamp results if passed factor is out of range.
/// # Example
///
/// ```
/// photon::effects::adjust_contrast(photon_image, 30.0);
/// ```
#[wasm_bindgen]
pub fn adjust_contrast(mut photon_image: &mut PhotonImage, contrast: f32) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    let clamped_contrast = num::clamp(contrast, -255.0, 255.0);

    // Some references:
    // https://math.stackexchange.com/questions/906240/algorithms-to-increase-or-decrease-the-contrast-of-an-image
    // https://www.dfstudios.co.uk/articles/programming/image-programming-algorithms/image-processing-algorithms-part-5-contrast-adjustment/
    let factor =
        (259.0 * (clamped_contrast + 255.0)) / (255.0 * (259.0 - clamped_contrast));
    let mut lookup_table: Vec<u8> = vec![0; 256];
    let offset = -128.0 * factor + 128.0;
    for i in 0..256 {
        let new_val = i as f32 * factor + offset;
        lookup_table[i] = num::clamp(new_val, 0.0, 255.0) as u8;
    }
    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        px.data[0] = lookup_table[px.data[0] as usize];
        px.data[1] = lookup_table[px.data[1] as usize];
        px.data[2] = lookup_table[px.data[2] as usize];

        img.put_pixel(x, y, px);
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
/// // For example, to tint an image of type `PhotonImage`:
/// photon::tint(img, 10, 20, 15);
/// ```
///
#[wasm_bindgen]
pub fn tint(
    mut photon_image: &mut PhotonImage,
    r_offset: u32,
    g_offset: u32,
    b_offset: u32,
) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);

    for (x, y) in ImageIterator::with_dimension(&img.dimensions()) {
        let mut px = img.get_pixel(x, y);
        let (r_val, g_val, b_val) =
            (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);

        px.data[0] = if r_val as u32 + r_offset < 255 {
            r_val as u8 + r_offset as u8
        } else {
            255
        };
        px.data[1] = if g_val as u32 + g_offset < 255 {
            g_val as u8 + g_offset as u8
        } else {
            255
        };
        px.data[2] = if b_val as u32 + b_offset < 255 {
            b_val as u8 + b_offset as u8
        } else {
            255
        };

        img.put_pixel(x, y, px);
    }
    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Horizontal strips. Divide an image into a series of equal-height strips, for an artistic effect.
#[wasm_bindgen]
pub fn horizontal_strips(mut photon_image: &mut PhotonImage, num_strips: u8) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    let total_strips = (num_strips * 2) - 1;
    let height_strip = height / total_strips as u32;
    let background_color = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    let mut y_pos: u32 = 0;
    for i in 1..num_strips {
        draw_filled_rect_mut(
            &mut img,
            Rect::at(0, (y_pos + height_strip) as i32).of_size(width, height_strip),
            Rgba([
                background_color.r,
                background_color.g,
                background_color.b,
                255u8,
            ]),
        );
        y_pos = i as u32 * (height_strip * 2);
    }

    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

/// Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect.
#[wasm_bindgen]
pub fn vertical_strips(mut photon_image: &mut PhotonImage, num_strips: u8) {
    let mut img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();

    let total_strips = (num_strips * 2) - 1;
    let width_strip = width / total_strips as u32;
    let background_color = Rgb {
        r: 255,
        g: 255,
        b: 255,
    };
    let mut x_pos: u32 = 0;
    for i in 1..num_strips {
        draw_filled_rect_mut(
            &mut img,
            Rect::at((x_pos + width_strip) as i32, 0).of_size(width_strip, height),
            Rgba([
                background_color.r,
                background_color.g,
                background_color.b,
                255u8,
            ]),
        );
        x_pos = i as u32 * (width_strip * 2);
    }

    let raw_pixels = img.raw_pixels();
    photon_image.raw_pixels = raw_pixels;
}

// pub fn create_gradient_map(color_a : Rgb, color_b: Rgb) -> Vec<Rgb> {
//     println!("hi");
//     println!("{}", color_a.get_red());
//     let mut gradient_map = vec![];

//     let max_val = 255;
//     let mut r_val = 0;

//     let end: i32 = 256 * 4;

//     for i in (0..end).step_by(4){
//         let i: u8 = i as u8;
//         let intensity_b = max_val - i;

//         let res1 = (i * color_a.get_red() + intensity_b * color_b.get_red());
//         let res2 = res1 / max_val;
//         println!("res 1 {}", res1);
//         println!("res 2 {}", res2);

//         r_val = (i * color_a.get_red() + intensity_b * color_b.get_red()) / max_val;
//         println!("r_val {}", r_val);
//         gradient_map.push(Rgb {
//             r: (256 - (i / 4) * color_a.get_red() + (i / 4) * color_b.r) / 256 ,
//             g: (i * color_a.get_green() + intensity_b * color_b.get_green()) / max_val ,
//             b: (i * color_a.get_blue() + intensity_b * color_b.get_blue()) / max_val
//         });

//     }
//     println!("{:?}", gradient_map);

//     return gradient_map;
// }

// pub fn duotone(mut img: DynamicImage, color_a : Rgb, color_b : Rgb) -> DynamicImage {
//     let (width, height) = img.dimensions();
//     let gradient_map = create_gradient_map(color_a, color_b);
//     println!("entering for loop");

//     for x in 0..width {
//         for y in 0..height {

//             let mut px = img.get_pixel(x, y);

//             let r = px.data[0];
//             let g = px.data[1];
//             let b = px.data[2];

//             px.data[0] = gradient_map[r as usize].r as u8;
//             px.data[1] = gradient_map[g as usize].g as u8;
//             px.data[2] = gradient_map[b as usize].b as u8;

//             img.put_pixel(x, y, px);
//         }
//     }
//     return img;
// }
