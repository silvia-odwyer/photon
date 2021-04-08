//! Padding PhotonImages

use crate::PhotonImage;

/// Apply uniform padding around the PhotonImage
/// A padded PhotonImage is returned.
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `padding` - The amount of padding to be applied to the PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels around a PhotonImage:
/// use photon_rs::padding::uniform;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// uniform(&img, 10_u32);
/// ```

pub fn uniform(img: &PhotonImage, padding: u32) -> PhotonImage {
    let image_buffer = img.get_raw_pixels();
    let img_width = img.get_width();
    let img_height = img.get_height();

    let mut img_padded_buffer = Vec::<u8>::new();
    let width_padded: u32 = img_width + 2 * padding;
    let height_padded: u32 = img_height + 2 * padding;

    for _ in 0..((width_padded + 1) * padding) {
        img_padded_buffer.push(0);
        img_padded_buffer.push(0);
        img_padded_buffer.push(0);
        img_padded_buffer.push(255);
    }

    for i in (0..image_buffer.len()).step_by(4) {
        if (i / 4) % img_width as usize == 0 && i != 0 {
            for _ in (0..2 * padding).step_by(1) {
                img_padded_buffer.push(0);
                img_padded_buffer.push(0);
                img_padded_buffer.push(0);
                img_padded_buffer.push(255);
            }
        }
        img_padded_buffer.push(image_buffer[i]);
        img_padded_buffer.push(image_buffer[i + 1]);
        img_padded_buffer.push(image_buffer[i + 2]);
        img_padded_buffer.push(image_buffer[i + 3]);
    }

    for _ in 0..((width_padded + 1) * padding) {
        img_padded_buffer.push(0);
        img_padded_buffer.push(0);
        img_padded_buffer.push(0);
        img_padded_buffer.push(255);
    }

    PhotonImage::new(img_padded_buffer, width_padded, height_padded)
}

/// Apply padding on the left side of the PhotonImage
/// A padded PhotonImage is returned.
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `padding` - The amount of padding to be applied to the PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels on the left side of a PhotonImage:
/// use photon_rs::padding::left;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// left(&img, 10_u32);
/// ```

pub fn left(img: &PhotonImage, padding: u32) -> PhotonImage {
    let image_buffer = img.get_raw_pixels();
    let img_width = img.get_width();
    let img_height = img.get_height();

    let mut img_padded_buffer = Vec::<u8>::new();
    let width_padded: u32 = img_width + padding;

    for i in 0..img_height as usize {
        let img_slice =
            image_buffer[(i * 4 * img_width as usize)..(i + 1) * 4 * img_width as usize].to_owned();

        for _ in 0..padding {
            img_padded_buffer.push(0);
            img_padded_buffer.push(0);
            img_padded_buffer.push(0);
            img_padded_buffer.push(255);
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
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels on the right side of a PhotonImage:
/// use photon_rs::padding::right;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// right(&img, 10_u32);
/// ```

pub fn right(img: &PhotonImage, padding: u32) -> PhotonImage {
    let image_buffer = img.get_raw_pixels();
    let img_width = img.get_width();
    let img_height = img.get_height();

    let mut img_padded_buffer = Vec::<u8>::new();
    let width_padded: u32 = img_width + padding;

    for i in 0..img_height as usize {
        let img_slice =
            image_buffer[(i * 4 * img_width as usize)..(i + 1) * 4 * img_width as usize].to_owned();
        for x in img_slice {
            img_padded_buffer.push(x);
        }
        for _ in 0..padding {
            img_padded_buffer.push(0);
            img_padded_buffer.push(0);
            img_padded_buffer.push(0);
            img_padded_buffer.push(255);
        }
    }
    PhotonImage::new(img_padded_buffer, width_padded, img_height)
}

/// Apply padding on the left side of the PhotonImage
/// A padded PhotonImage is returned.
/// # Arguments
/// * `img` - A PhotonImage. See the PhotonImage struct for details.
/// * `padding` - The amount of padding to be applied to the PhotonImage.
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels on the top of a PhotonImage:
/// use photon_rs::padding::top;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// top(&img, 10_u32);
/// ```

pub fn top(img: &PhotonImage, padding: u32) -> PhotonImage {
    let image_buffer = img.get_raw_pixels();
    let img_width = img.get_width();
    let img_height = img.get_height();

    let height_padded: u32 = img_height + padding;
    let mut img_padded_buffer: Vec<u8> = Vec::<u8>::new();

    for _ in 0..(padding * img_width) {
        img_padded_buffer.push(0);
        img_padded_buffer.push(0);
        img_padded_buffer.push(0);
        img_padded_buffer.push(255);
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
///
/// # Example
///
/// ```no_run
/// // For example, to apply a padding of 10 pixels on the bottom of a PhotonImage:
/// use photon_rs::padding::bottom;
/// use photon_rs::native::open_image;
///
/// let mut img = open_image("img.jpg").expect("File should open");
/// bottom(&img, 10_u32);
/// ```

pub fn bottom(img: &PhotonImage, padding: u32) -> PhotonImage {
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
        img_padded_buffer.push(0);
        img_padded_buffer.push(0);
        img_padded_buffer.push(0);
        img_padded_buffer.push(255);
    }

    PhotonImage::new(img_padded_buffer, img_width, height_padded)
}
