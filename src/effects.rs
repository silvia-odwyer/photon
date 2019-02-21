extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use rand::Rng;
use std::f64;
use std::cmp;
 
/// Adds an offset to the image by a certain number of pixels. 
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `offset` - The offset is added to the pixels in the image.  
/// # Example
///
/// ```
/// // For example, to threshold an image of type `DynamicImage`:
/// use photon::channels;
/// photon::channels::threshold(img);
/// ```
pub fn offset(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();

    for x in 0..width {
        for y in 0..height {
            let offset = rng.gen_range(0, 150);
            let mut px = img.get_pixel(x, y);

            if x + 10 < width - 1 && y + 10 < height - 1  {
                let offset_px = img.get_pixel(x + 10, y + 10);
                px = offset_px;
            }
        }
    }
    return img;
}

/// Add a sine wave animation to the pixels by getting a sine wave. 
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to threshold an image of type `DynamicImage`:
/// use photon::effects;
/// photon::effects::ripple(img);
/// ```
pub fn ripple(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    
    let xoff = width / 3; 
    let yoff = height / 3;

    for x in 0..width {
        for y in 0..height {

            let mut px = img.get_pixel(x, y);
            // // calculate sine based on distance
            // x2 = x - xoff;
            // y2 = y - yoff;
            // d = Math.sqrt(x2*x2 + y2*y2);
            // t = Math.sin(d/6.0);
            let x2: f64 = x as f64 - xoff as f64;
            let y2: f64 = y as f64 - yoff as f64;

            let res: f64 = x2*x2  + y2*y2;
            let d = (res).sqrt();
		    let t = (d/6.0).sin();

            let r = t * 200.0;
		    let g = 125.0 + t * 80.0;
		    let b = 235.0 + t * 20.0;
            
            px.data[0] = cmp::max(0, cmp::min(255, r as u32)) as u8;
            px.data[1] = cmp::max(0, cmp::min(255, g as u32)) as u8;
            px.data[2] = cmp::max(0, cmp::min(255, b as u32)) as u8;
        }
    }
    return img;
}