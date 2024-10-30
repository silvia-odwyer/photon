//! Convolution effects such as sharpening, blurs, sobel filters, etc.,

use crate::helpers;
use crate::PhotonImage;
use image::DynamicImage::ImageRgba8;
use image::{GenericImage, GenericImageView, Pixel};
use std::cmp::min;

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

type Kernel = [f32; 9];

fn conv(photon_image: &mut PhotonImage, kernel: Kernel) {
    let mut img = helpers::dyn_image_from_raw(photon_image);
    img = ImageRgba8(img.to_rgba8());

    let mut filtered_img = img.filter3x3(&kernel);
    filtered_img = ImageRgba8(filtered_img.to_rgba8());

    if filtered_img.pixels().all(|p| p.2[3] == 0) {
        for x in 0..GenericImageView::width(&img) - 1 {
            for y in 0..GenericImageView::height(&img) - 1 {
                let mut pixel = GenericImageView::get_pixel(&filtered_img, x, y);
                let original_alpha =
                    GenericImageView::get_pixel(&img, x, y).channels()[3];
                pixel.channels_mut()[3] = original_alpha;
                filtered_img.put_pixel(x, y, pixel);
            }
        }
    }

    photon_image.raw_pixels = filtered_img.into_bytes();
}

/// Noise reduction.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to noise reduct an image:
/// use photon_rs::conv::noise_reduction;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// noise_reduction(&mut img);
/// ```
/// Adds a constant to a select R, G, or B channel's value.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn noise_reduction(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [0.0_f32, -1.0, 7.0, -1.0, 5.0, 9.0, 0.0, 7.0, 9.0],
    );
}

/// Sharpen an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to sharpen an image:
/// use photon_rs::conv::sharpen;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// sharpen(&mut img);
/// ```
/// Adds a constant to a select R, G, or B channel's value.
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn sharpen(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [0.0_f32, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0],
    );
}

/// Apply edge detection to an image, to create a dark version with its edges highlighted.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to increase the Red channel for all pixels by 10:
/// use photon_rs::conv::edge_detection;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// edge_detection(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn edge_detection(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [-1.0_f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0],
    );
}

/// Apply an identity kernel convolution to an image.
///
/// # Arguments
/// * `img` -A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply an identity kernel convolution:
/// use photon_rs::conv::identity;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// identity(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn identity(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [0.0_f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
    );
}

/// Apply a box blur effect.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a box blur effect:
/// use photon_rs::conv::box_blur;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// box_blur(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn box_blur(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [1.0_f32, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    );
}

/// Gaussian blur in linear time.
///
/// Reference: http://blog.ivank.net/fastest-gaussian-blur.html
///
/// # Arguments
/// * `photon_image` - A PhotonImage
/// * `radius` - blur radius
/// # Example
///
/// ```no_run
/// use photon_rs::conv::gaussian_blur;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// gaussian_blur(&mut img, 3_i32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn gaussian_blur(photon_image: &mut PhotonImage, radius: i32) {
    // construct pixel data
    let img = helpers::dyn_image_from_raw(photon_image);
    let mut src = img.into_bytes();

    let width = photon_image.get_width();
    let height = photon_image.get_height();
    let mut target: Vec<u8> = src.clone();

    // Clamp radius value when it exceeds width or height.
    // Divide by 2 since maximal radius must satisfy these conditions:
    // rad + ((w - 1) * h) + rad < w * h
    // rad + ((h - 1) * w) + rad < w * h
    // After all transformations they become:
    // rad < h / 2
    // rad < w / 2
    // Subtract 1 because the inequalies are strict.
    let radius = min(width as i32 / 2 - 1, radius);
    let radius = min(height as i32 / 2 - 1, radius);

    let bxs = boxes_for_gauss(radius as f32, 3);
    box_blur_inner(&mut src, &mut target, width, height, (bxs[0] - 1) / 2);
    box_blur_inner(&mut target, &mut src, width, height, (bxs[1] - 1) / 2);
    box_blur_inner(&mut src, &mut target, width, height, (bxs[2] - 1) / 2);

    // manipulate back
    photon_image.raw_pixels = target;
}

fn boxes_for_gauss(sigma: f32, n: usize) -> Vec<i32> {
    let n_float = n as f32;

    let w_ideal = (12.0 * sigma * sigma / n_float).sqrt() + 1.0;
    let mut wl: i32 = w_ideal.floor() as i32;

    if wl % 2 == 0 {
        wl -= 1;
    };

    let wu = wl + 2;

    let wl_float = wl as f32;

    let m_ideal = (12.0 * sigma * sigma
        - n_float * wl_float * wl_float
        - 4.0 * n_float * wl_float
        - 3.0 * n_float)
        / (-4.0 * wl_float - 4.0);

    let m: usize = m_ideal.round() as usize;

    let mut sizes = Vec::<i32>::new();
    for i in 0..n {
        if i < m {
            sizes.push(wl);
        } else {
            sizes.push(wu);
        }
    }

    sizes
}

fn box_blur_inner(
    src: &mut [u8],
    target: &mut [u8],
    width: u32,
    height: u32,
    radius: i32,
) {
    let length = (width * height * 4) as usize;
    target[..length].clone_from_slice(&src[..length]);
    box_blur_horizontal(target, src, width, height, radius);
    box_blur_vertical(src, target, width, height, radius);
}

fn box_blur_horizontal(
    src: &[u8],
    target: &mut [u8],
    width: u32,
    height: u32,
    radius: i32,
) {
    let iarr = 1.0 / (radius + radius + 1) as f32;
    for i in 0..height {
        let mut ti: usize = (i * width) as usize * 4;
        let mut li: usize = ti;
        let mut ri: usize = ti + radius as usize * 4;

        let fv_r = src[ti] as i32;
        let fv_g = src[ti + 1] as i32;
        let fv_b = src[ti + 2] as i32;

        let lv_r = src[ti + (width - 1) as usize * 4];
        let lv_g = src[ti + (width - 1) as usize * 4 + 1];
        let lv_b = src[ti + (width - 1) as usize * 4 + 2];

        let mut val_r = (radius + 1) * fv_r;
        let mut val_g = (radius + 1) * fv_g;
        let mut val_b = (radius + 1) * fv_b;

        for j in 0..radius {
            val_r += src[ti + j as usize * 4] as i32;
            val_g += src[ti + j as usize * 4 + 1] as i32;
            val_b += src[ti + j as usize * 4 + 2] as i32;
        }

        for _ in 0..radius + 1 {
            val_r += src[ri] as i32 - fv_r;
            val_g += src[ri + 1] as i32 - fv_g;
            val_b += src[ri + 2] as i32 - fv_b;
            ri += 4;

            target[ti] = (val_r as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 1] = (val_g as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 2] = (val_b as f32 * iarr).clamp(0.0, 255.0) as u8;
            ti += 4;
        }

        for _ in (radius + 1)..(width as i32 - radius) {
            val_r += src[ri] as i32 - src[li] as i32;
            val_g += src[ri + 1] as i32 - src[li + 1] as i32;
            val_b += src[ri + 2] as i32 - src[li + 2] as i32;
            ri += 4;
            li += 4;

            target[ti] = (val_r as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 1] = (val_g as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 2] = (val_b as f32 * iarr).clamp(0.0, 255.0) as u8;
            ti += 4;
        }

        for _ in (width as i32 - radius)..width as i32 {
            val_r += lv_r as i32 - src[li] as i32;
            val_g += lv_g as i32 - src[li + 1] as i32;
            val_b += lv_b as i32 - src[li + 2] as i32;
            li += 4;

            target[ti] = (val_r as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 1] = (val_g as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 2] = (val_b as f32 * iarr).clamp(0.0, 255.0) as u8;
            ti += 4;
        }
    }
}

fn box_blur_vertical(
    src: &[u8],
    target: &mut [u8],
    width: u32,
    height: u32,
    radius: i32,
) {
    let iarr = 1.0 / (radius + radius + 1) as f32;

    for i in 0..width {
        let mut ti: usize = i as usize * 4;
        let mut li: usize = ti;
        let mut ri: usize = ti + (radius * width as i32) as usize * 4;

        let fv_r = src[ti] as i32;
        let fv_g = src[ti + 1] as i32;
        let fv_b = src[ti + 2] as i32;

        let lv_r = src[ti + ((height - 1) * width) as usize * 4];
        let lv_g = src[ti + ((height - 1) * width) as usize * 4 + 1];
        let lv_b = src[ti + ((height - 1) * width) as usize * 4 + 2];

        let mut val_r = (radius + 1) * fv_r;
        let mut val_g = (radius + 1) * fv_g;
        let mut val_b = (radius + 1) * fv_b;

        for j in 0..radius {
            val_r += src[ti + (j * width as i32) as usize * 4] as i32;
            val_g += src[ti + (j * width as i32) as usize * 4 + 1] as i32;
            val_b += src[ti + (j * width as i32) as usize * 4 + 2] as i32;
        }

        for _ in 0..radius + 1 {
            val_r += src[ri] as i32 - fv_r;
            val_g += src[ri + 1] as i32 - fv_g;
            val_b += src[ri + 2] as i32 - fv_b;
            ri += width as usize * 4;

            target[ti] = (val_r as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 1] = (val_g as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 2] = (val_b as f32 * iarr).clamp(0.0, 255.0) as u8;
            ti += width as usize * 4;
        }

        for _ in (radius + 1)..(height as i32 - radius) {
            val_r += src[ri] as i32 - src[li] as i32;
            val_g += src[ri + 1] as i32 - src[li + 1] as i32;
            val_b += src[ri + 2] as i32 - src[li + 2] as i32;
            ri += width as usize * 4;
            li += width as usize * 4;

            target[ti] = (val_r as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 1] = (val_g as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 2] = (val_b as f32 * iarr).clamp(0.0, 255.0) as u8;
            ti += width as usize * 4;
        }

        for _ in (height as i32 - radius)..height as i32 {
            val_r += lv_r as i32 - src[li] as i32;
            val_g += lv_g as i32 - src[li + 1] as i32;
            val_b += lv_b as i32 - src[li + 2] as i32;
            li += width as usize * 4;

            target[ti] = (val_r as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 1] = (val_g as f32 * iarr).clamp(0.0, 255.0) as u8;
            target[ti + 2] = (val_b as f32 * iarr).clamp(0.0, 255.0) as u8;
            ti += width as usize * 4;
        }
    }
}

/// Detect horizontal lines in an image, and highlight these only.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to display the horizontal lines in an image:
/// use photon_rs::conv::detect_horizontal_lines;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// detect_horizontal_lines(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn detect_horizontal_lines(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [-1.0_f32, -1.0, -1.0, 2.0, 2.0, 2.0, -1.0, -1.0, -1.0],
    );
}

/// Detect vertical lines in an image, and highlight these only.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to display the vertical lines in an image:
/// use photon_rs::conv::detect_vertical_lines;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// detect_vertical_lines(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn detect_vertical_lines(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [-1.0_f32, 2.0, -1.0, -1.0, 2.0, -1.0, -1.0, 2.0, -1.0],
    );
}

/// Detect lines at a forty five degree angle in an image, and highlight these only.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to display the lines at a forty five degree angle in an image:
/// use photon_rs::conv::detect_45_deg_lines;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// detect_45_deg_lines(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn detect_45_deg_lines(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [-1.0_f32, -1.0, 2.0, -1.0, 2.0, -1.0, 2.0, -1.0, -1.0],
    );
}

/// Detect lines at a 135 degree angle in an image, and highlight these only.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to display the lines at a 135 degree angle in an image:
/// use photon_rs::conv::detect_135_deg_lines;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// detect_135_deg_lines(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn detect_135_deg_lines(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [2.0_f32, -1.0, -1.0, -1.0, 2.0, -1.0, -1.0, -1.0, 2.0],
    );
}

/// Apply a standard laplace convolution.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a laplace effect:
/// use photon_rs::conv::laplace;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// laplace(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn laplace(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [0.0_f32, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0],
    );
}

/// Preset edge effect.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply this effect:
/// use photon_rs::conv::edge_one;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// edge_one(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn edge_one(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [0.0_f32, -2.2, -0.6, -0.4, 2.8, -0.3, -0.8, -1.0, 2.7],
    );
}

/// Apply an emboss effect to an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply an emboss effect:
/// use photon_rs::conv::emboss;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// emboss(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn emboss(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [-2.0_f32, -1.0, 0.0, -1.0, 1.0, 1.0, 0.0, 1.0, 2.0],
    );
}

/// Apply a horizontal Sobel filter to an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a horizontal Sobel filter:
/// use photon_rs::conv::sobel_horizontal;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// sobel_horizontal(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn sobel_horizontal(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [-1.0_f32, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0],
    );
}

/// Apply a horizontal Prewitt convolution to an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a horizontal Prewitt convolution effect:
/// use photon_rs::conv::prewitt_horizontal;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// prewitt_horizontal(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn prewitt_horizontal(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [5.0_f32, -3.0, -3.0, 5.0, 0.0, -3.0, 5.0, -3.0, -3.0],
    );
}

/// Apply a vertical Sobel filter to an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a vertical Sobel filter:
/// use photon_rs::conv::sobel_vertical;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// sobel_vertical(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn sobel_vertical(photon_image: &mut PhotonImage) {
    conv(
        photon_image,
        [-1.0_f32, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
    );
}

/// Apply a global Sobel filter to an image
///
/// Each pixel is calculated as the magnitude of the horizontal and vertical components of the Sobel filter,
/// ie if X is the horizontal sobel and Y is the vertical, for each pixel, we calculate sqrt(X^2 + Y^2)
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a global Sobel filter:
/// use photon_rs::conv::sobel_global;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// sobel_global(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn sobel_global(photon_image: &mut PhotonImage) {
    let mut sobel_x = photon_image.clone();
    let sobel_y = photon_image;

    sobel_horizontal(&mut sobel_x);
    sobel_vertical(sobel_y);

    let sob_x_values = sobel_x.get_raw_pixels();
    let sob_y_values = sobel_y.get_raw_pixels();

    let mut sob_xy_values = vec![];

    for i in 0..(sob_x_values.len()) {
        let kx = (sob_x_values[i]) as u32;
        let ky = (sob_y_values[i]) as u32; // this could panic if for some reason the sobel_y doesn't have the same size as the sobel_x
        let kxy_2 = kx * kx + ky * ky; // u8 * u8 is u16 and we sum two so we need u32
        sob_xy_values.push((kxy_2 as f64).sqrt() as u8); // f64::max is bigger than u32::max so no problem with conversion
    }

    sobel_y.raw_pixels = sob_xy_values;
}
