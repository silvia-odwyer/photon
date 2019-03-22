extern crate image;
use image::{GenericImage, DynamicImage, GenericImageView};

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
pub fn sharpen(img: DynamicImage) -> DynamicImage {
    let kernel = [0.0f32, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn edge_detection(img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
/// photon::conv::edge_detection(img);
/// ```
pub fn identity(img: DynamicImage) -> DynamicImage {
    let kernel = [0.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn box_blur(img: DynamicImage) -> DynamicImage {
    let kernel = [1.0f32, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn gaussian_blur(img: DynamicImage) -> DynamicImage {
    let kernel = [1.0f32, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn detect_horizontal_lines(img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, -1.0, -1.0, 2.0, 2.0, 2.0, -1.0, -1.0, -1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn detect_vertictal_lines(img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, 2.0, -1.0, -1.0, 2.0, -1.0, -1.0, 2.0, -1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn detect_fortyfivedeg_lines(img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, -1.0, 2.0, -1.0, 2.0, -1.0, 2.0, -1.0, -1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn detect_hundredthirtyfive_deg_lines(img: DynamicImage) -> DynamicImage {
    let kernel = [2.0f32, -1.0, -1.0, -1.0, 2.0, -1.0, -1.0, -1.0, 2.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn laplace(img: DynamicImage) -> DynamicImage {
    let kernel = [0.0f32, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn edge_one(img: DynamicImage) -> DynamicImage {
    let kernel = [0.0f32, -2.2, -0.6, -0.4, 2.8, -0.3, -0.8, -1.0, 2.7];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn emboss(img: DynamicImage) -> DynamicImage {
    let kernel = [-2.0f32, -1.0, 0.0, -1.0, 1.0, 1.0, 0.0, 1.0, 2.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn sobel_horizontal(img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
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
pub fn sobel_vertical(img: DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];

    let filtered_img = img.filter3x3(&kernel);
    return filtered_img;
}

