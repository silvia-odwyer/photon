//! Image transformations, ie: scale, crop, resize, etc.,

use crate::helpers;
use crate::iter::ImageIterator;
use crate::{PhotonImage, Rgba};
use image::imageops::FilterType;
use image::DynamicImage::ImageRgba8;
use image::RgbaImage;
use image::{GenericImageView, ImageBuffer};
use std::cmp::max;
use std::cmp::min;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{Clamped, JsCast};
#[cfg(target_arch = "wasm32")]
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
/// let cropped_img: PhotonImage = crop(&mut img, 0_u32, 0_u32, 500_u32, 800_u32);
/// // Write the contents of this image in JPG format.
/// ```
#[wasm_bindgen]
pub fn crop(
    photon_image: &mut PhotonImage,
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
    let raw_pixels = dynimage.to_bytes();
    PhotonImage {
        raw_pixels,
        width: dynimage.width(),
        height: dynimage.height(),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
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
#[wasm_bindgen]
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
    let raw_pixels = dynimage.to_bytes();
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
#[wasm_bindgen]
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
    let raw_pixels = dynimage.to_bytes();
    photon_image.raw_pixels = raw_pixels;
}

#[wasm_bindgen]
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
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
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

    canvas.set_width(resized_img.width());
    canvas.set_height(resized_img.height());

    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut resized_img.to_bytes()),
        canvas.width(),
        canvas.height(),
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
#[wasm_bindgen]
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

    PhotonImage {
        raw_pixels: resized_img.to_bytes(),
        width: resized_img.width(),
        height: resized_img.height(),
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
#[wasm_bindgen]
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

    PhotonImage {
        raw_pixels: img.to_bytes(),
        width: img.width(),
        height: img.height(),
    }
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
#[wasm_bindgen]
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
#[wasm_bindgen]
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
#[wasm_bindgen]
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
#[wasm_bindgen]
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
#[wasm_bindgen]
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
/// # NOTE: This is a naive implementation. Paeth rotation should be faster.
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
/// let rotated_img = rotate(&img, 30);
/// ```
#[wasm_bindgen]
pub fn rotate(img: &PhotonImage, angle: i32) -> PhotonImage {
    // 390, 750 and 30 degrees represent the same angle. Trim 360.
    let full_circle_count = angle / 360;
    let normalized_angle = angle - full_circle_count * 360;
    if normalized_angle == 0 {
        return img.clone();
    }

    // Handle negative angles. -80 describes the same angle as 360 - 80 = 280.
    let positive_angle = if normalized_angle < 0 {
        normalized_angle + 360
    } else {
        normalized_angle
    };

    // Count the number of rotations by right angle and apply them via rotate90 if necessary.
    let right_angle_count = positive_angle / 90;
    let mut rgba_img: RgbaImage = ImageBuffer::from_raw(
        img.get_width(),
        img.get_height(),
        img.raw_pixels.to_vec(),
    )
    .unwrap();
    for _ in 0..right_angle_count {
        rgba_img = image::imageops::rotate90(&rgba_img);
    }

    let dynimage = ImageRgba8(rgba_img);
    let raw_pixels = dynimage.to_bytes();
    let src_width = dynimage.width();
    let src_height = dynimage.height();

    let angle_deg = positive_angle - right_angle_count * 90;
    if angle_deg == 0 {
        return PhotonImage {
            raw_pixels,
            width: src_width,
            height: src_height,
        };
    }

    // Convert degrees to radians and calculate sine and cosine parts.
    let angle_rad = angle_deg as f64 * std::f64::consts::PI / 180.0;
    let cosine = angle_rad.cos();
    let sine = angle_rad.sin();

    // Move (0, 0) point to (w / 2, h / 2) in order to rotate around the centre.
    let src_centre_x = src_width / 2;
    let src_centre_y = src_height / 2;
    let src_centre_x_f64 = src_centre_x as f64;
    let src_centre_y_f64 = src_centre_y as f64;

    // (-cx, cy) corner will contain the leftmost x after the rotation.
    let leftmost_pos = -1.0 * src_centre_x_f64 * cosine - src_centre_y_f64 * sine;
    let leftmost_pos = leftmost_pos.floor() as i32;

    // (cx, -cy) corner will contain the rightmost x after the rotation.
    let rightmost_pos = src_centre_x_f64 * cosine + src_centre_y_f64 as f64 * sine;
    let rightmost_pos = rightmost_pos.floor() as i32;

    // (-cx, -cy) corner will contain the least y after the rotation.
    let bottom_pos = -1.0 * src_centre_x_f64 * sine - src_centre_y_f64 * cosine;
    let bottom_pos = bottom_pos.floor() as i32;

    // (cx, cy) corner will contain the largest y after the rotation.
    let top_pos = src_centre_x_f64 * sine + src_centre_y_f64 * cosine;
    let top_pos = top_pos.floor() as i32;

    // Move (0, 0) point to (w / 2, h / 2) target image as well.
    let dst_width = rightmost_pos - leftmost_pos;
    let dst_width = dst_width as u32;
    let dst_height = top_pos - bottom_pos;
    let dst_height = dst_height as u32;
    let dst_centre_x = dst_width / 2;
    let dst_centre_y = dst_height / 2;

    // Allocate destination buffer.
    let channel_count = 4;
    let mut result = Vec::<u8>::new();
    let total_dst_size = (dst_width * dst_height * channel_count)
        .try_into()
        .expect("Failed to calculate destination size");
    result.resize(total_dst_size, 0);

    // Calculate source and target strides.
    let stride_chan = 1;
    let src_stride_col = channel_count * stride_chan;
    let src_stride_row = src_width * src_stride_col;
    let dst_stride_col = channel_count * stride_chan;
    let dst_stride_row = dst_width * dst_stride_col;

    for row in 0..src_height {
        for col in 0..src_width {
            // Rows and columns are counted from 0 to width and height.
            // In order to get coordinates relative to (w / 2, h / 2) subtract centre point.
            let src_x = (col as i32 - src_centre_x as i32) as f64;
            let src_y = (row as i32 - src_centre_y as i32) as f64;

            // Rotation.
            let dst_x = src_x * cosine - src_y * sine + dst_centre_x as f64;
            let dst_x = dst_x.floor() as u32;
            let dst_x = max(0, dst_x);
            let dst_x = min(dst_width - 1, dst_x);

            let dst_y = src_x * sine + src_y * cosine + dst_centre_y as f64;
            let dst_y = dst_y.floor() as u32;
            let dst_y = max(0, dst_y);
            let dst_y = min(dst_height - 1, dst_y);

            // Translate coordinates to the buffer offsets.
            let src_idx = (row * src_stride_row + col * src_stride_col) as usize;
            let dst_idx = (dst_y * dst_stride_row + dst_x * dst_stride_col) as usize;

            for chan in 0..channel_count {
                let chan = chan as usize;
                result[dst_idx + chan] = raw_pixels[src_idx + chan];

                // FIXME apply proper interpolation
                result[dst_idx + 4 + chan] = raw_pixels[src_idx + chan];
            }
        }
    }

    PhotonImage::new(result, dst_width, dst_height)
}
