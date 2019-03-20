use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};

pub mod filters;
pub mod channels;
pub mod noise;
pub mod effects;
pub mod conv;

struct Rgb {
    r: u32,
    g: u32,
    b: u32
}

/// Threshold an image using a standard thresholding algorithm.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `threshold` - The amount the image should be thresholded by.
/// # Example
///
/// ```
/// // For example, to threshold an image of type `DynamicImage`:
/// use photon::channels;
/// photon::channels::threshold(img);
/// ```
pub fn threshold(mut img: DynamicImage, threshold: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let r: f32 = px.data[0].into();
            let g: f32 = px.data[1].into();
            let b: f32 = px.data[2].into();

            let mut v = 0.2126 * r + 0.7152 * g + 0.072 * b;

            if v >= threshold as f32 {
                v = 255.0;
            }
            else {
                v = 0.0;
            }
            px.data[0] = v as u8;
            px.data[1] = v as u8;
            px.data[2] = v as u8;

            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Tint an image by adding an offset to averaged RGB channel values.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
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
pub fn tint(mut img: DynamicImage, r_offset: u32, g_offset: u32, b_offset: u32) -> DynamicImage {
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
    return img;
}

pub fn monochrome(mut img: DynamicImage, r_offset: u32, g_offset: u32, b_offset: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            let mut avg = (r_val + g_val + b_val) / 3;
            if avg >= 255 {
                avg = 255
            }
            
            px.data[0] = if avg as u32 + r_offset < 255 { avg as u8 + r_offset as u8} else { 255 };
            px.data[1] = if avg as u32 + g_offset < 255 { avg as u8 + g_offset as u8} else { 255 };
            px.data[2] = if avg as u32 + b_offset < 255 { avg as u8 + b_offset as u8} else { 255 };

            img.put_pixel(x, y, px);
        }
    }
    return img;
}


/// Convert an image to sepia.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to tint an image of type `DynamicImage`:
/// use photon::channels;
/// photon::channels::sepia(img);
/// ```
/// 
pub fn sepia(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as f32, px.data[1] as f32, px.data[2] as f32);
            let avg = 0.3 * r_val + 0.59 * g_val + 0.11 * b_val;

            px.data[0] = if avg as u32 + 100 < 255 { avg as u8 + 100} else { 255 };
            px.data[1] = if avg as u32 + 30 < 255 { avg as u8 + 50 } else { 255 };
      
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Convert an image to grayscale using the conventional averaging algorithm.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.

/// # Example
///
/// ```
/// // For example, to convert an image of type `DynamicImage` to greyscale:
/// use photon::channels;
/// photon::channels::grayscale(img);
/// ```
pub fn grayscale(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            let mut avg = (r_val + g_val + b_val) / 3;
            if avg >= 255 {
                avg = 255
            }
            px.data[0] = avg as u8;
            px.data[1] = avg as u8;
            px.data[2] = avg as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Convert an image to grayscale with a human corrected factor, to account for human vision.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.

/// # Example
///
/// ```
/// // For example, to convert an image of type `DynamicImage` to greyscale with a human corrected factor:
/// use photon::channels;
/// photon::channels::grayscale_human_corrected(img);
/// ```
pub fn grayscale_human_corrected(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as f32, px.data[1] as f32, px.data[2] as f32);

            let mut avg = (r_val * 0.3 + g_val * 0.59 + b_val * 0.11);
            
            if avg >= 255.0 {
                avg = 255.0
            }
            
            px.data[0] = avg as u8;
            px.data[1] = avg as u8;
            px.data[2] = avg as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Desaturate an image by getting the min/max of each pixel's RGB values.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.

/// # Example
///
/// ```
/// // For example, to desaturate an image:
/// use photon::channels;
/// photon::channels::desaturate(img);
/// ```
pub fn desaturate(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            // get the max and min vals of all 3 rgb values by sorting a vec of these
            let mut rgb_vals = vec![r_val, g_val, b_val];
            rgb_vals.sort();

            let mut gray = (rgb_vals[0] + rgb_vals[2]) / 2;

            if gray >= 255 {
                gray = 255
            }
            
            px.data[0] = gray as u8;
            px.data[1] = gray as u8;
            px.data[2] = gray as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Uses a min. decomposition algorithm to convert an image to greyscale.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.

/// # Example
///
/// ```
/// // For example, to decompose an image:
/// photon::channels::decompose_min(img);
/// ```
pub fn decompose_min(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            // get the max and min vals of all 3 rgb values by sorting a vec of these
            let mut rgb_vals = vec![r_val, g_val, b_val];
            rgb_vals.sort();

            let mut gray = rgb_vals[0];

            if gray >= 255 {
                gray = 255
            }
            
            px.data[0] = gray as u8;
            px.data[1] = gray as u8;
            px.data[2] = gray as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Uses a max. decomposition algorithm to convert an image to greyscale.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.

/// # Example
///
/// ```
/// // For example, to decompose an image with max decomposition:
/// photon::channels::decompose_max(img);
/// ```
pub fn decompose_max(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            // get the max and min vals of all 3 rgb values by sorting a vec of these
            let mut rgb_vals = vec![r_val, g_val, b_val];
            rgb_vals.sort();

            let mut gray = rgb_vals[2];
            
            px.data[0] = gray as u8;
            px.data[1] = gray as u8;
            px.data[2] = gray as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Employ only a limited number of gray shades in an image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.

/// # Example
///
/// ```
/// // For example, to limit an image to four shades of gray only:
/// photon::channels::grayscale_shades(img, 4);
/// ```
pub fn grayscale_shades(mut img: DynamicImage, num_shades: u8) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let conversion: f32 = 255.0 / (num_shades as f32 - 1.0);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            
            let avg: f32 = (r_val + g_val + b_val) as f32 / 3.0;
            
            let dividend = avg / conversion as f32;

            let gray = (dividend + 0.5) * conversion;
            let mut px = img.get_pixel(x, y);
            
            px.data[0] = gray as u8;
            px.data[1] = gray as u8;
            px.data[2] = gray as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Convert an image to grayscale by setting all 3 RGB values to the Red channel only.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.

/// # Example
///
/// ```
/// photon::channels::r_grayscale(img);
/// ```
pub fn r_grayscale(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let mut red = px.data[0];
            
            px.data[0] = red as u8;
            px.data[1] = red as u8;
            px.data[2] = red as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Convert an image to grayscale by setting all 3 RGB values to the Green channel only.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.

/// # Example
///
/// ```
/// photon::channels::g_grayscale(img);
/// ```
pub fn g_grayscale(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let mut green = px.data[1];
            
            px.data[0] = green as u8;
            px.data[1] = green as u8;
            px.data[2] = green as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Convert an image to grayscale by setting all 3 RGB values to the Blue channel only.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.

/// # Example
///
/// ```
/// photon::channels::b_grayscale(img);
/// ```
pub fn b_grayscale(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let mut blue = px.data[2];
            
            px.data[0] = blue as u8;
            px.data[1] = blue as u8;
            px.data[2] = blue as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

/// Increase the brightness of an image by a factor.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `brightness` - A u8 to add to the brightness.
/// # Example
///
/// ```
/// photon::channels::g_grayscale(img);
/// ```
pub fn inc_brightness(mut img: DynamicImage, brightness: u8) -> DynamicImage {
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
    return img;
}