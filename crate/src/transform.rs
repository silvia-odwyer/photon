//! Image transformations, ie: scale, crop, resize, etc.,

use crate::helpers;
use crate::iter::ImageIterator;
use crate::{PhotonImage, Rgba};
use image::imageops::FilterType;
use image::DynamicImage::ImageRgba8;
use image::{GenericImageView, ImageBuffer, Pixel, RgbaImage};
use std::cmp::min;

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;
#[cfg(all(feature = "enable_wasm", target_arch = "wasm32"))]
use wasm_bindgen::{Clamped, JsCast};
#[cfg(all(feature = "enable_wasm", target_arch = "wasm32"))]
use web_sys::{HtmlCanvasElement, ImageData};

/// Crop an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to crop an image at (0, 0) to (500, 800)
/// use photon_rs::native::{open_image};
/// use photon_rs::transform::crop;
/// use photon_rs::PhotonImage;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let cropped_img: PhotonImage = crop(&img, 0_u32, 0_u32, 500_u32, 800_u32);
/// // Write the contents of this image in JPG format.
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn crop(
    photon_image: &PhotonImage,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
) -> PhotonImage {
    let img = helpers::dyn_image_from_raw(photon_image);

    let mut cropped_img: RgbaImage = ImageBuffer::new(x2 - x1, y2 - y1);

    for (x, y) in ImageIterator::with_dimension(&cropped_img.dimensions()) {
        let px = img.get_pixel(x1 + x, y1 + y);
        cropped_img.put_pixel(x, y, px);
    }
    let dynimage = ImageRgba8(cropped_img);

    let width = dynimage.width();
    let height = dynimage.height();

    let raw_pixels = dynimage.into_bytes();
    PhotonImage {
        raw_pixels,
        width,
        height,
    }
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn crop_img_browser(
    source_canvas: HtmlCanvasElement,
    width: f64,
    height: f64,
    left: f64,
    top: f64,
) -> HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let dest_canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    dest_canvas.set_width(width as u32);
    dest_canvas.set_height(height as u32);

    let ctx = dest_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        &source_canvas,
        left,
        top,
        width,
        height,
        0.0,
        0.0,
        width,
        height,
    )
    .unwrap();

    dest_canvas
}

/// Flip an image horizontally.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to flip an image horizontally:
/// use photon_rs::native::open_image;
/// use photon_rs::transform::fliph;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// fliph(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn fliph(photon_image: &mut PhotonImage) {
    let img = helpers::dyn_image_from_raw(photon_image);

    let width = img.width();
    let height = img.height();
    let mut flipped_img: RgbaImage = ImageBuffer::new(width, height);

    for (x, y) in ImageIterator::new(width, height) {
        let px = img.get_pixel(x, y);
        flipped_img.put_pixel(width - x - 1, y, px);
    }

    let dynimage = ImageRgba8(flipped_img);
    let raw_pixels = dynimage.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

/// Flip an image vertically.
///
/// # Arguments
/// * `img` - A PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to flip an image vertically:
/// use photon_rs::native::open_image;
/// use photon_rs::transform::flipv;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// flipv(&mut img);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn flipv(photon_image: &mut PhotonImage) {
    let img = helpers::dyn_image_from_raw(photon_image);

    let width = img.width();
    let height = img.height();

    let mut flipped_img: RgbaImage = ImageBuffer::new(width, height);

    for (x, y) in ImageIterator::new(width, height) {
        let px = img.get_pixel(x, y);
        flipped_img.put_pixel(x, height - y - 1, px);
    }

    let dynimage = ImageRgba8(flipped_img);
    let raw_pixels = dynimage.into_bytes();
    photon_image.raw_pixels = raw_pixels;
}

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub enum SamplingFilter {
    Nearest = 1,
    Triangle = 2,
    CatmullRom = 3,
    Gaussian = 4,
    Lanczos3 = 5,
}

fn filter_type_from_sampling_filter(sampling_filter: SamplingFilter) -> FilterType {
    match sampling_filter {
        SamplingFilter::Nearest => FilterType::Nearest,
        SamplingFilter::Triangle => FilterType::Triangle,
        SamplingFilter::CatmullRom => FilterType::CatmullRom,
        SamplingFilter::Gaussian => FilterType::Gaussian,
        SamplingFilter::Lanczos3 => FilterType::Lanczos3,
    }
}

/// Resize an image on the web.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `width` - New width.
/// * `height` - New height.
/// * `sampling_filter` - Nearest = 1, Triangle = 2, CatmullRom = 3, Gaussian = 4, Lanczos3 = 5
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn resize_img_browser(
    photon_img: &PhotonImage,
    width: u32,
    height: u32,
    sampling_filter: SamplingFilter,
) -> HtmlCanvasElement {
    let sampling_filter = filter_type_from_sampling_filter(sampling_filter);
    let dyn_img = helpers::dyn_image_from_raw(photon_img);
    let resized_img = ImageRgba8(image::imageops::resize(
        &dyn_img,
        width,
        height,
        sampling_filter,
    ));

    // TODO Check if in browser or Node.JS
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let width = resized_img.width();
    let height = resized_img.height();

    canvas.set_width(width);
    canvas.set_height(width);

    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut resized_img.into_bytes()),
        width,
        height,
    );

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // Place the new imagedata onto the canvas
    ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0)
        .unwrap();

    canvas
}

/// Resize an image.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `width` - New width.
/// * `height` - New height.
/// * `sampling_filter` - Nearest = 1, Triangle = 2, CatmullRom = 3, Gaussian = 4, Lanczos3 = 5
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn resize(
    photon_img: &PhotonImage,
    width: u32,
    height: u32,
    sampling_filter: SamplingFilter,
) -> PhotonImage {
    let sampling_filter = filter_type_from_sampling_filter(sampling_filter);

    let dyn_img = helpers::dyn_image_from_raw(photon_img);
    let resized_img = ImageRgba8(image::imageops::resize(
        &dyn_img,
        width,
        height,
        sampling_filter,
    ));

    let width = resized_img.width();
    let height = resized_img.height();

    PhotonImage {
        raw_pixels: resized_img.into_bytes(),
        width,
        height,
    }
}

/// Resize image using seam carver.
/// Resize only if new dimensions are smaller, than original image.
/// # NOTE: This is still experimental feature, and pretty slow.
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `width` - New width.
/// * `height` - New height.
///
/// # Example
///
/// ```no_run
/// // For example, resize image using seam carver:
/// use photon_rs::native::open_image;
/// use photon_rs::transform::seam_carve;
/// use photon_rs::PhotonImage;
///
/// let img = open_image("img.jpg").expect("File should open");
/// let result: PhotonImage = seam_carve(&img, 100_u32, 100_u32);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn seam_carve(img: &PhotonImage, width: u32, height: u32) -> PhotonImage {
    let mut img: RgbaImage = ImageBuffer::from_raw(
        img.get_width(),
        img.get_height(),
        img.raw_pixels.to_vec(),
    )
    .unwrap();
    let (w, h) = img.dimensions();
    let (diff_w, diff_h) = (w - w.min(width), h - h.min(height));

    for _ in 0..diff_w {
        let vec_steam = imageproc::seam_carving::find_vertical_seam(&img);
        img = imageproc::seam_carving::remove_vertical_seam(&img, &vec_steam);
    }
    if diff_h.ne(&0_u32) {
        img = image::imageops::rotate90(&img);
        for _ in 0..diff_h {
            let vec_steam = imageproc::seam_carving::find_vertical_seam(&img);
            img = imageproc::seam_carving::remove_vertical_seam(&img, &vec_steam);
        }
        img = image::imageops::rotate270(&img);
    }

    let img = ImageRgba8(img);

    let width = img.width();
    let height = img.height();

    PhotonImage {
        raw_pixels: img.into_bytes(),
        width,
        height,
    }
}

/// Shear the image along the X axis.
/// A sheared PhotonImage is returned.
///
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `shear` - Amount to shear.
///
/// # Example
///
/// ```no_run
/// // For example, to shear an image by 0.5:
/// use photon_rs::native::open_image;
/// use photon_rs::transform::shearx;
///
/// let img = open_image("img.jpg").expect("File should open");
/// let sheared_img = shearx(&img, 0.5);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn shearx(photon_img: &PhotonImage, shear: f32) -> PhotonImage {
    let img = helpers::dyn_image_from_raw(photon_img);
    let src_width = img.width();
    let src_height = img.height();

    let maxskew = shear * (src_height as f32);
    let dst_width = maxskew.floor().abs() as u32 + src_width;

    let mut delta = 0;
    if shear < 0. {
        delta = dst_width - src_width;
    }

    let mut sheared_image: RgbaImage = ImageBuffer::new(dst_width, src_height);

    for old_y in 0..src_height {
        let skew = shear * (old_y as f32 + 0.5);
        let skewi = skew.floor() as i32 + delta as i32;
        let skewf = skew.fract().abs();
        let mut oleft = image::Rgba([0_u8, 0_u8, 0_u8, 0_u8]);
        for old_x in (0..src_width).rev() {
            let mut pixel = img.get_pixel(old_x, old_y);
            let mut left = pixel.map(|val| (val as f32 * skewf) as u8);
            if shear >= 0. {
                left = pixel.map2(&left, |val1, val2| val1 - val2);
            }
            pixel = pixel.map2(&left, |val1, val2| val1 - val2);
            pixel = pixel.map2(&oleft, |val1, val2| {
                min(val1 as u16 + val2 as u16, 255_u16) as u8
            });
            let new_x = (old_x as i32 + skewi) as u32;
            sheared_image.put_pixel(new_x, old_y, pixel);
            oleft = left;
        }
        sheared_image.put_pixel(skewi as u32, old_y, oleft);
    }

    let dynimage = ImageRgba8(sheared_image);
    let width = dynimage.width();
    let height = dynimage.height();
    let raw_pixels = dynimage.into_bytes();

    PhotonImage::new(raw_pixels, width, height)
}

/// Shear the image along the Y axis.
/// A sheared PhotonImage is returned.
///
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `shear` - Amount to shear.
///
/// # Example
///
/// ```no_run
/// // For example, to shear an image by 0.5:
/// use photon_rs::native::open_image;
/// use photon_rs::transform::sheary;
///
/// let img = open_image("img.jpg").expect("File should open");
/// let sheared_img = sheary(&img, 0.5);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn sheary(photon_img: &PhotonImage, shear: f32) -> PhotonImage {
    let img = helpers::dyn_image_from_raw(photon_img);
    let src_width = img.width();
    let src_height = img.height();

    let maxskew = shear * (src_width as f32);
    let dst_height = maxskew.floor().abs() as u32 + src_height;

    let mut delta = 0;
    if shear < 0. {
        delta = dst_height - src_height;
    }

    let mut sheared_image: RgbaImage = ImageBuffer::new(src_width, dst_height);

    for old_x in 0..src_width {
        let skew = shear * (old_x as f32 + 0.5);
        let skewi = skew.floor() as i32 + delta as i32;
        let skewf = skew.fract().abs();
        let mut oleft = image::Rgba([0_u8, 0_u8, 0_u8, 0_u8]);
        for old_y in (0..src_height).rev() {
            let mut pixel = img.get_pixel(old_x, old_y);
            let mut left = pixel.map(|val| (val as f32 * skewf).floor() as u8);
            if shear >= 0. {
                left = pixel.map2(&left, |val1, val2| val1 - val2);
            }
            pixel = pixel.map2(&left, |val1, val2| val1 - val2);
            pixel = pixel.map2(&oleft, |val1, val2| {
                min(val1 as u16 + val2 as u16, 255_u16) as u8
            });
            let new_y = (old_y as i32 + skewi) as u32;
            sheared_image.put_pixel(old_x, new_y, pixel);
            oleft = left;
        }
        sheared_image.put_pixel(old_x, skewi as u32, oleft);
    }

    let dynimage = ImageRgba8(sheared_image);
    let width = dynimage.width();
    let height = dynimage.height();
    let raw_pixels = dynimage.into_bytes();

    PhotonImage::new(raw_pixels, width, height)
}

/// Apply uniform padding around the PhotonImage
/// A padded PhotonImage is returned.
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `padding` - The amount of padding to be applied to the PhotonImage.
/// * `padding_rgba` - Tuple containing the RGBA code for padding color.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels around a PhotonImage:
/// use photon_rs::transform::padding_uniform;
/// use photon_rs::native::open_image;
/// use photon_rs::Rgba;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
/// padding_uniform(&img, 10_u32, rgba);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn padding_uniform(
    img: &PhotonImage,
    padding: u32,
    padding_rgba: Rgba,
) -> PhotonImage {
    let image_buffer = img.get_raw_pixels();
    let img_width = img.get_width();
    let img_height = img.get_height();

    let mut img_padded_buffer = Vec::<u8>::new();
    let width_padded: u32 = img_width + 2 * padding;
    let height_padded: u32 = img_height + 2 * padding;

    for _ in 0..((width_padded + 1) * padding) {
        img_padded_buffer.push(padding_rgba.get_red());
        img_padded_buffer.push(padding_rgba.get_green());
        img_padded_buffer.push(padding_rgba.get_blue());
        img_padded_buffer.push(padding_rgba.get_alpha());
    }

    for i in (0..image_buffer.len()).step_by(4) {
        if (i / 4) % img_width as usize == 0 && i != 0 {
            for _ in (0..2 * padding).step_by(1) {
                img_padded_buffer.push(padding_rgba.get_red());
                img_padded_buffer.push(padding_rgba.get_green());
                img_padded_buffer.push(padding_rgba.get_blue());
                img_padded_buffer.push(padding_rgba.get_alpha());
            }
        }
        img_padded_buffer.push(image_buffer[i]);
        img_padded_buffer.push(image_buffer[i + 1]);
        img_padded_buffer.push(image_buffer[i + 2]);
        img_padded_buffer.push(image_buffer[i + 3]);
    }

    for _ in 0..((width_padded + 1) * padding) {
        img_padded_buffer.push(padding_rgba.get_red());
        img_padded_buffer.push(padding_rgba.get_green());
        img_padded_buffer.push(padding_rgba.get_blue());
        img_padded_buffer.push(padding_rgba.get_alpha());
    }

    PhotonImage::new(img_padded_buffer, width_padded, height_padded)
}

/// Apply padding on the left side of the PhotonImage
/// A padded PhotonImage is returned.
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `padding` - The amount of padding to be applied to the PhotonImage.
/// * `padding_rgba` - Tuple containing the RGBA code for padding color.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels on the left side of a PhotonImage:
/// use photon_rs::transform::padding_left;
/// use photon_rs::native::open_image;
/// use photon_rs::Rgba;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
/// padding_left(&img, 10_u32, rgba);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn padding_left(img: &PhotonImage, padding: u32, padding_rgba: Rgba) -> PhotonImage {
    let image_buffer = img.get_raw_pixels();
    let img_width = img.get_width();
    let img_height = img.get_height();

    let mut img_padded_buffer = Vec::<u8>::new();
    let width_padded: u32 = img_width + padding;

    for i in 0..img_height as usize {
        let img_slice = image_buffer
            [(i * 4 * img_width as usize)..(i + 1) * 4 * img_width as usize]
            .to_owned();

        for _ in 0..padding {
            img_padded_buffer.push(padding_rgba.get_red());
            img_padded_buffer.push(padding_rgba.get_green());
            img_padded_buffer.push(padding_rgba.get_blue());
            img_padded_buffer.push(padding_rgba.get_alpha());
        }
        for x in img_slice {
            img_padded_buffer.push(x);
        }
    }
    PhotonImage::new(img_padded_buffer, width_padded, img_height)
}

/// Apply padding on the left side of the PhotonImage
/// A padded PhotonImage is returned.
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `padding` - The amount of padding to be applied to the PhotonImage.
/// * `padding_rgba` - Tuple containing the RGBA code for padding color.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels on the right side of a PhotonImage:
/// use photon_rs::transform::padding_right;
/// use photon_rs::native::open_image;
/// use photon_rs::Rgba;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
/// padding_right(&img, 10_u32, rgba);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn padding_right(
    img: &PhotonImage,
    padding: u32,
    padding_rgba: Rgba,
) -> PhotonImage {
    let image_buffer = img.get_raw_pixels();
    let img_width = img.get_width();
    let img_height = img.get_height();

    let mut img_padded_buffer = Vec::<u8>::new();
    let width_padded: u32 = img_width + padding;

    for i in 0..img_height as usize {
        let img_slice = image_buffer
            [(i * 4 * img_width as usize)..(i + 1) * 4 * img_width as usize]
            .to_owned();
        for x in img_slice {
            img_padded_buffer.push(x);
        }
        for _ in 0..padding {
            img_padded_buffer.push(padding_rgba.get_red());
            img_padded_buffer.push(padding_rgba.get_green());
            img_padded_buffer.push(padding_rgba.get_blue());
            img_padded_buffer.push(padding_rgba.get_alpha());
        }
    }
    PhotonImage::new(img_padded_buffer, width_padded, img_height)
}

/// Apply padding on the left side of the PhotonImage
/// A padded PhotonImage is returned.
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `padding` - The amount of padding to be applied to the PhotonImage.
/// * `padding_rgba` - Tuple containing the RGBA code for padding color.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels on the top of a PhotonImage:
/// use photon_rs::transform::padding_top;
/// use photon_rs::native::open_image;
/// use photon_rs::Rgba;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
/// padding_top(&img, 10_u32, rgba);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn padding_top(img: &PhotonImage, padding: u32, padding_rgba: Rgba) -> PhotonImage {
    let image_buffer = img.get_raw_pixels();
    let img_width = img.get_width();
    let img_height = img.get_height();

    let height_padded: u32 = img_height + padding;
    let mut img_padded_buffer: Vec<u8> = Vec::<u8>::new();

    for _ in 0..(padding * img_width) {
        img_padded_buffer.push(padding_rgba.get_red());
        img_padded_buffer.push(padding_rgba.get_green());
        img_padded_buffer.push(padding_rgba.get_blue());
        img_padded_buffer.push(padding_rgba.get_alpha());
    }

    for i in (0..image_buffer.len()).step_by(4) {
        img_padded_buffer.push(image_buffer[i]);
        img_padded_buffer.push(image_buffer[i + 1]);
        img_padded_buffer.push(image_buffer[i + 2]);
        img_padded_buffer.push(image_buffer[i + 3]);
    }

    PhotonImage::new(img_padded_buffer, img_width, height_padded)
}

/// Apply padding on the left side of the PhotonImage
/// A padded PhotonImage is returned.
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `padding` - The amount of padding to be applied to the PhotonImage.
/// * `padding_rgba` - Tuple containing the RGBA code for padding color.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels on the bottom of a PhotonImage:
/// use photon_rs::transform::padding_bottom;
/// use photon_rs::native::open_image;
/// use photon_rs::Rgba;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
/// padding_bottom(&img, 10_u32, rgba);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn padding_bottom(
    img: &PhotonImage,
    padding: u32,
    padding_rgba: Rgba,
) -> PhotonImage {
    let image_buffer = img.get_raw_pixels();
    let img_width = img.get_width();
    let img_height = img.get_height();

    let height_padded: u32 = img_height + padding;
    let mut img_padded_buffer: Vec<u8> = Vec::<u8>::new();

    for i in (0..image_buffer.len()).step_by(4) {
        img_padded_buffer.push(image_buffer[i]);
        img_padded_buffer.push(image_buffer[i + 1]);
        img_padded_buffer.push(image_buffer[i + 2]);
        img_padded_buffer.push(image_buffer[i + 3]);
    }

    for _ in 0..(padding * img_width) {
        img_padded_buffer.push(padding_rgba.get_red());
        img_padded_buffer.push(padding_rgba.get_green());
        img_padded_buffer.push(padding_rgba.get_blue());
        img_padded_buffer.push(padding_rgba.get_alpha());
    }

    PhotonImage::new(img_padded_buffer, img_width, height_padded)
}

/// Rotate the PhotonImage on an arbitrary angle
/// A rotated PhotonImage is returned.
///
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `angle` - Rotation angle in degrees.
///
/// # Example
///
/// ```no_run
/// // For example, to rotate a PhotonImage by 30 degrees:
/// use photon_rs::native::open_image;
/// use photon_rs::transform::rotate;
///
/// let img = open_image("img.jpg").expect("File should open");
/// let rotated_img = rotate(&img, 30.0);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[must_use]
pub fn rotate(photon_img: &PhotonImage, angle: f32) -> PhotonImage {
    // 390, 750 and 30 degrees represent the same angle. Trim 360.
    let full_circle_count = angle as i32 / 360;
    let normalized_angle = angle as i32 - full_circle_count * 360;
    if normalized_angle == 0 {
        return photon_img.clone();
    }

    // Handle negative angles. -80 describes the same angle as 360 - 80 = 280.
    let positive_angle = if normalized_angle < 0 {
        normalized_angle + 360
    } else {
        normalized_angle
    };

    let right_angle_count = positive_angle / 90;
    let mut rgba_img: RgbaImage = ImageBuffer::from_raw(
        photon_img.get_width(),
        photon_img.get_height(),
        photon_img.get_raw_pixels().to_vec(),
    )
    .unwrap();
    for _ in 0..right_angle_count {
        rgba_img = image::imageops::rotate90(&rgba_img);
    }

    let dynimage = ImageRgba8(rgba_img);
    let src_width = dynimage.width();
    let src_height = dynimage.height();
    let raw_pixels = dynimage.into_bytes();

    let mut img_out = PhotonImage::new(raw_pixels, src_width, src_height);

    let theta = ((angle % 360.) - (right_angle_count * 90) as f32).to_radians();
    let beta = theta.sin();
    let alpha = -1. * ((theta / 2.).tan());

    img_out = shearx(&img_out, alpha);
    img_out = sheary(&img_out, beta);
    img_out = shearx(&img_out, alpha);

    img_out
}

fn greatest_common_divisor(left_val: usize, right_val: usize) -> usize {
    let mut a = left_val;
    let mut b = right_val;
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn copy_row(buf: &[u8], row_pos: usize, row_stride: usize) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    for byte_idx in 0..row_stride {
        let src_idx = (row_pos * row_stride) + byte_idx;
        result.push(buf[src_idx]);
    }

    result
}

/// Resample the PhotonImage.
///
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `dst_width` - Target width.
/// * `dst_height` - Target height.
///
/// # Example
///
/// ```no_run
/// // For example, to resample a PhotonImage to 1920x1080 size:
/// use photon_rs::native::open_image;
/// use photon_rs::transform::resample;
///
/// let img = open_image("img.jpg").expect("File should open");
/// let rotated_img = resample(&img, 1920, 1080);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn resample(img: &PhotonImage, dst_width: usize, dst_height: usize) -> PhotonImage {
    let mut pix_buf = Vec::<u8>::new();
    if dst_width == 0 || dst_height == 0 {
        return PhotonImage::new(pix_buf, dst_width as u32, dst_height as u32);
    }

    let src_width = img.get_width() as usize;
    let src_height = img.get_height() as usize;

    // The idea is to upsample source width to the greatest commond divisor, then downsample to
    // target width. For example: resample 240 to 320. The greatest common divisor is 80.
    // At first, upsample 240 to 960 (960 is 240 * 320 / 80)
    // Next downsample 960 to 320 (320 is 960 / (240 / 80)).
    // Thus, upsampling rate is 320 / 80 = 4, downsampling rate is 240 / 80 = 3.
    let width_gcd = greatest_common_divisor(src_width, dst_width);
    let height_gcd = greatest_common_divisor(src_height, dst_height);
    let upsample_x = dst_width / width_gcd;
    let downsample_x = src_width / width_gcd;
    let upsample_y = dst_height / height_gcd;
    let downsample_y = src_height / height_gcd;

    // Upsampling and downsampling are performed in the same loop.
    // Upsample the image while the size is indivisible by downsampling rate.
    // Then downsample and clear the buffer.
    // For example, upsampling rate is 3 and downsampling rate is 4.
    // After processing 4 pixels, buffer gets 12 pixels (repeats each pixel 3 times).
    // 12 pixels can be downsampled by 4, so downsampling takes every 4th pixel.
    // The result contains 3 pixels. That approach is somewhat slower but requires less memory.
    let img_pixels = &img.raw_pixels;
    let src_chan = 4;

    // Resample width.
    let mut resampled_width = Vec::<u8>::new();
    for row in 0..src_height {
        // Upsample pixels and put them to temporary buffer.
        let mut upsampled_width = Vec::<u8>::new();
        for col in 0..src_width {
            for _i in 0..upsample_x {
                for chan in 0..src_chan {
                    let src_idx = (row * src_width * src_chan) + col * src_chan + chan;
                    upsampled_width.push(img_pixels[src_idx]);
                }
            }

            // When the temporary buffer can be downsampled, downsample and clear it.
            let upsampled_pix_count = upsampled_width.len() / src_chan;
            if (upsampled_pix_count % downsample_x) == 0 {
                for i in 0..upsampled_pix_count / downsample_x {
                    for chan in 0..src_chan {
                        let src_idx = (i * downsample_x) * src_chan + chan;
                        resampled_width.push(upsampled_width[src_idx]);
                    }
                }
                upsampled_width.clear();
            }
        }
    }

    // Resample height.
    let mut upsampled_height = Vec::<u8>::new();
    for row in 0..src_height {
        // Upsample rows and put them to temporary buffer.
        for _i in 0..upsample_y {
            let mut row_copy = copy_row(&resampled_width, row, dst_width * src_chan);
            upsampled_height.append(&mut row_copy);
        }

        // When the temporary buffer can be downsampled, downsample and clear it.
        let upsampled_rows_count = upsampled_height.len() / src_chan / dst_width;
        if (upsampled_rows_count % downsample_y) == 0 {
            for i in 0..upsampled_rows_count / downsample_y {
                let mut row_copy =
                    copy_row(&upsampled_height, i * downsample_y, dst_width * src_chan);
                pix_buf.append(&mut row_copy);
            }
            upsampled_height.clear();
        }
    }

    PhotonImage::new(pix_buf, dst_width as u32, dst_height as u32)
}

/// Compression
///
/// # Arguments
/// * `img` - A PhotonImage.
/// * `quality` - The Quality of the returned PhotonImage.
///
/// # Example
///
/// ```no_run
/// use photon_rs::native::open_image;
/// use photon_rs::transform::compress;
///
/// let image = open_image("img.jpg").expect("File should open");
///
/// let compressed_image = compress(&image, 50);
/// ```
/// Adds a constant to a select R, G, or B channel's value.
pub fn compress(img: &PhotonImage, quality: u8) -> PhotonImage {
    let bytes = img.get_bytes_jpeg(quality);

    PhotonImage::new_from_byteslice(bytes)
}
