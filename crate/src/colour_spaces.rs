//! Image manipulation effects in HSL, LCh and HSV.

extern crate image;
extern crate rand;
use image::{GenericImageView};
use palette::{Hsl, Lch, Shade, Pixel, Saturate, Srgba, Hue, Hsv};
use crate::{PhotonImage, helpers};
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/// Image manipulation effects in the LCh colour space
/// 
/// Effects include:
/// * **saturate** - Saturation increase.
/// * **desaturate** - Desaturate the image.
/// * **shift_hue** - Hue rotation by a specified number of degrees.
/// * **darken** - Decrease the brightness.
/// * **lighten** - Increase the brightness.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
/// * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
/// # Example
/// ```
/// // For example to increase the saturation by 10%:
/// use photon::color_spaces::lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// lch(&mut img, "saturate", 0.1);
/// ```
#[wasm_bindgen]
pub fn lch(mut photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let (width, height) = img.dimensions();
    let mut img = img.to_rgba();
    for x in 0..width {
        for y in 0..height {
            let px_data = img.get_pixel(x, y).data;
            let lch_colour: Lch = Srgba::from_raw(&px_data)
                .into_format()
                .into_linear()
                .into();

            let new_color = match mode {
                // Match a single value
                "desaturate" => lch_colour.desaturate(amt),
                "saturate" => lch_colour.saturate(amt),
                "lighten" => lch_colour.lighten(amt), 
                "darken" => lch_colour.darken(amt),
                "shift_hue" => lch_colour.shift_hue(amt * 360.0),
                _ => lch_colour.saturate(amt),
            };
            
            img.put_pixel(x, y, image::Rgba {
                data: Srgba::from_linear(new_color.into()).into_format().into_raw()
            });
            }
        }
    photon_image.raw_pixels = img.to_vec();
}

/// Image manipulation effects in the HSL colour space.
/// 
/// Effects include:
/// * **saturate** - Saturation increase.
/// * **desaturate** - Desaturate the image.
/// * **shift_hue** - Hue rotation by a specified number of degrees.
/// * **darken** - Decrease the brightness.
/// * **lighten** - Increase the brightness.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
/// * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
/// # Example
/// ```
/// // For example to increase the saturation by 10%:
/// use photon::color_spaces::hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hsl(&mut img, "saturate", 0.1);
/// ``` 
#[wasm_bindgen]
pub fn hsl(mut photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    // The function logic is kept separate from other colour spaces for now, 
    // since other HSL-specific logic may be implemented here, which isn't available in other colour spaces
    let mut img = helpers::dyn_image_from_raw(&photon_image).to_rgba();
    let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let colour = Srgba::from_raw(&px_data).into_format();

                let hsl_colour = Hsl::from(colour);
                
                let new_color = match mode {
                    // Match a single value
                    "desaturate" => hsl_colour.desaturate(amt),
                    "saturate" => hsl_colour.saturate(amt),
                    "lighten" => hsl_colour.lighten(amt), 
                    "darken" => hsl_colour.darken(amt),
                    "shift_hue" => hsl_colour.shift_hue(amt * 360.0),
                    _ => hsl_colour.saturate(amt),
                };

                img.put_pixel(x, y, image::Rgba {
                    data: Srgba::from_linear(new_color.into()).into_format().into_raw()
                });
            }
        }

    photon_image.raw_pixels = img.to_vec();
}

/// Image manipulation in the HSV colour space. 
/// 
/// Effects include:
/// * **saturate** - Saturation increase.
/// * **desaturate** - Desaturate the image.
/// * **shift_hue** - Hue rotation by a specified number of degrees.
/// * **darken** - Decrease the brightness.
/// * **lighten** - Increase the brightness.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage.
/// * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
/// * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
/// 
/// # Example
/// ```
/// // For example to increase the saturation by 10%:
/// use photon::color_spaces::hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hsv(&mut img, "saturate", 0.1);
/// ```
#[wasm_bindgen]
pub fn hsv(photon_image: &mut PhotonImage, mode: &str, amt: f32) {
    let img = helpers::dyn_image_from_raw(&photon_image);
    let mut img  = img.to_rgba();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgba::from_raw(&px_data).into_format();

                let hsv_colour = Hsv::from(color);

                let new_color = match mode {
                    // Match a single value
                    "desaturate" => hsv_colour.desaturate(amt),
                    "saturate" => hsv_colour.saturate(amt),
                    "lighten" => hsv_colour.lighten(amt), 
                    "darken" => hsv_colour.darken(amt),
                    "shift_hue" => hsv_colour.shift_hue(amt * 360.0),
                    _ => hsv_colour.saturate(amt),
                };

                img.put_pixel(x, y, image::Rgba {
                    data: Srgba::from_linear(new_color.into()).into_format().into_raw()
                });
            }
        }
    photon_image.raw_pixels = img.to_vec();
}

/// Shift hue by a specified number of degrees in the HSL colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - The number of degrees to shift the hue by, or hue rotate by.
/// 
/// # Example
/// ```
/// // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
/// use photon::color_spaces::hue_rotate_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hue_rotate_hsl(&mut img, 120);
/// ``` 
#[wasm_bindgen]
pub fn hue_rotate_hsl(img: &mut PhotonImage, degrees: f32) {
    hsl(img, "shift_hue", degrees);
}

/// Shift hue by a specified number of degrees in the HSV colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - The number of degrees to shift the hue by, or hue rotate by.
/// 
/// # Example
/// ```
/// // For example to hue rotate/shift the hue by 120 degrees in the HSV colour space:
/// use photon::color_spaces::hue_rotate_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hue_rotate_hsv(&mut img, 120);
/// ``` 
#[wasm_bindgen]
pub fn hue_rotate_hsv(img: &mut PhotonImage, degrees: f32) {
    hsv(img, "shift_hue", degrees);
}

/// Shift hue by a specified number of degrees in the LCh colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `mode` - The number of degrees to shift the hue by, or hue rotate by.
/// 
/// # Example
/// ```
/// // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
/// use photon::color_spaces::hue_rotate_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// hue_rotate_lch(&mut img, 120);
/// ``` 
#[wasm_bindgen]
pub fn hue_rotate_lch(img: &mut PhotonImage, degrees: f32) {
    lch(img, "shift_hue", degrees)
}

/// Increase the image's saturation by converting each pixel's colour to the HSL colour space
/// and increasing the colour's saturation. 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Increasing saturation by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to increase saturation by 10% in the HSL colour space:
/// use photon::color_spaces::saturate_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// saturate_hsl(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn saturate_hsl(img: &mut PhotonImage, level: f32) {

    return hsl(img, "saturate", level);
}

/// Increase the image's saturation in the LCh colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Increasing saturation by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to increase saturation by 40% in the Lch colour space:
/// use photon::color_spaces::saturate_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// saturate_lch(&mut img, 0.4);
/// ``` 
#[wasm_bindgen]
pub fn saturate_lch(img: &mut PhotonImage, level: f32) {
    return lch(img, "saturate", level);
}

/// Increase the image's saturation in the HSV colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level by which to increase the saturation by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Increasing saturation by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to increase saturation by 30% in the HSV colour space:
/// use photon::color_spaces::saturate_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// saturate_hsv(&mut img, 0.3);
/// ``` 
#[wasm_bindgen]
pub fn saturate_hsv(img: &mut PhotonImage, level: f32) {
    return hsv(img, "saturate", level);
}

/// Lighten an image by a specified amount in the LCh colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Lightening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to lighten an image by 10% in the LCh colour space:
/// use photon::color_spaces::lighten_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// lighten_lch(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn lighten_lch(img: &mut PhotonImage, level: f32) {
    return lch(img, "lighten", level);
}

/// Lighten an image by a specified amount in the HSL colour space.
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Lightening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to lighten an image by 10% in the HSL colour space:
/// use photon::color_spaces::lighten_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// lighten_hsl(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn lighten_hsl(img: &mut PhotonImage, level: f32) {
    return hsl(img, "lighten", level);
}

/// Lighten an image by a specified amount in the HSV colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Lightening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to lighten an image by 10% in the HSV colour space:
/// use photon::color_spaces::lighten_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// lighten_hsv(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn lighten_hsv(img: &mut PhotonImage, level: f32) {
    return hsv(img, "lighten", level);
}

/// Darken the image by a specified amount in the LCh colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Darkening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to darken an image by 10% in the LCh colour space:
/// use photon::color_spaces::darken_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// darken_lch(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn darken_lch(img: &mut PhotonImage, level: f32) {
    return lch(img, "darken", level);
}

/// Darken the image by a specified amount in the HSL colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Darkening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to darken an image by 10% in the HSL colour space:
/// use photon::color_spaces::darken_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// darken_hsl(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn darken_hsl(img: &mut PhotonImage, level: f32) {
    return hsl(img, "darken", level);
}

/// Darken the image's colours by a specified amount in the HSV colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Darkening by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to darken an image by 10% in the HSV colour space:
/// use photon::color_spaces::darken_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// darken_hsv(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn darken_hsv(img: &mut PhotonImage, level: f32) {
    return hsv(img, "darken", level);
}

/// Desaturate the image by a specified amount in the HSV colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Desaturating by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to desaturate an image by 10% in the HSV colour space:
/// use photon::color_spaces::desaturate_hsv;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/mountains.PNG");
/// 
/// desaturate_hsv(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn desaturate_hsv(img: &mut PhotonImage, level: f32) {
    return hsv(img, "desaturate", level);
}

/// Desaturate the image by a specified amount in the HSL colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Desaturating by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to desaturate an image by 10% in the LCh colour space:
/// use photon::color_spaces::desaturate_hsl;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// desaturate_hsl(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn desaturate_hsl(img: &mut PhotonImage, level: f32) {
    return hsl(img, "desaturate", level);
}

/// Desaturate the image by a specified amount in the LCh colour space.
/// 
/// # Arguments
/// * `img` - A PhotonImage.
/// * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
/// The `level` must be from 0 to 1 in floating-point, `f32` format. 
/// Desaturating by 80% would be represented by a `level` of 0.8
/// 
/// # Example
/// ```
/// // For example to desaturate an image by 10% in the LCh colour space:
/// use photon::color_spaces::desaturate_lch;
/// 
/// // Open the image. A PhotonImage is returned.
/// let img: PhotonImage = open_image("images/flowers.PNG");
/// 
/// desaturate_lch(&mut img, 0.1);
/// ``` 
#[wasm_bindgen]
pub fn desaturate_lch(img: &mut PhotonImage, level: f32) {
    return lch(img, "desaturate", level);
}

// pub fn correct(img: &DynamicImage, mode: &'static str, colour_space: &'static str, amt: f32) -> DynamicImage {
//     let mut img  = img.to_rgb();

//     let (width, height) = img.dimensions();

//         for x in 0..width {
//             for y in 0..height {
//                 let px_data = img.get_pixel(x, y).data;

//                 let colour_to_cspace;
//                 if colour_space == "hsv" {
//                     colour_to_cspace: Hsv = Srgb::from_raw(&px_data).into_format();
//                 }
//                 else if colour_space == "hsl" {
//                     colour_to_cspace = Hsl::from(color);
//                 }
//                 else {
//                     colour_to_cspace = Lch::from(color);
//                 }
            
//                 let new_color  = match mode {
//                     // Match a single value
//                     "desaturate" => colour_to_cspace.desaturate(amt),
//                     "saturate" => colour_to_cspace.saturate(amt),
//                     "lighten" => colour_to_cspace.lighten(amt), 
//                     "darken" => colour_to_cspace.darken(amt),
//                     _ => colour_to_cspace.saturate(amt),
//                 };

//                 img.put_pixel(x, y, image::Rgb {
//                     data: Srgb::from_linear(new_color.into()).into_format().into_raw()
//                 });
//             }
//         }

//     let dynimage = image::ImageRgb8(img);
//     return dynimage;
// }