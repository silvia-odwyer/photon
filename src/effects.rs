extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use rand::Rng;
use std::f64;
use std::cmp;

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}
 
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
pub fn offset(mut img: DynamicImage, channel_index: usize, offset: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let rng = rand::thread_rng();

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
    return img;
}

/// Adds an offset to the red channel by a certain number of pixels. 
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `offset` - The offset you want to move the red channel by. 
/// # Example
///
/// ```
/// // For example, to threshold an image of type `DynamicImage`:
/// use photon::channels;
/// photon::channels::threshold(img);
/// ```
pub fn offset_red(img: DynamicImage, offset_amt: u32) -> DynamicImage {
    offset(img, 0, offset_amt)
}

/// Adds an offset to the green channel by a certain number of pixels. 
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `offset` - The offset you want to move the green channel by.
/// # Example
///
/// ```
/// // For example, to threshold an image of type `DynamicImage`:
/// use photon::channels;
/// photon::channels::threshold(img);
/// ```
pub fn offset_green(img: DynamicImage, offset_amt: u32) -> DynamicImage {
    offset(img, 1, offset_amt)
}

/// Adds an offset to the blue channel by a certain number of pixels. 
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// * `offset_amt` - The offset you want to move the blue channel by.
/// # Example
///
pub fn offset_blue(img: DynamicImage, offset_amt: u32) -> DynamicImage {
    offset(img, 2, offset_amt)
}

/// Adds multiple offsets to the image by a certain number of pixels. 
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
pub fn multiple_offsets(mut img: DynamicImage, offset: u32, channel_index: usize, channel_index2: usize) -> DynamicImage {
    let (width, height) = img.dimensions();
    let rng = rand::thread_rng();

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
    return img;
}

/// Add a sine wave animation to the pixels by distributing the pixels along a sine curve.
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
            
            px.data[0] = cmp::max(0, cmp::min(255, r as u8)) as u8;
            px.data[1] = cmp::max(0, cmp::min(255, g as u8)) as u8;
            px.data[2] = cmp::max(0, cmp::min(255, b as u8)) as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

pub fn create_gradient_map(color_a : Rgb, color_b: Rgb) -> Vec<Rgb> {
    println!("hi");
    println!("{}", color_a.r);
    let mut gradient_map = vec![];

    let max_val = 255;
    let mut r_val = 0;

    for i in 0..max_val + 1{
        let intensity_b = max_val - i;

        r_val = (i * color_a.r + intensity_b * color_b.r) / max_val as u8;
        println!("r_val {}", r_val);
        gradient_map.push(Rgb {
            r: r_val , 
            g: (i * color_a.g + intensity_b * color_b.g) / max_val as u8 ,
            b: (i * color_a.b + intensity_b * color_b.b) / max_val as u8
        });
    }
    println!("{:?}", gradient_map);
    return gradient_map;
}

pub fn duotone(mut img: DynamicImage, color_a : Rgb, color_b : Rgb) -> DynamicImage {
    let (width, height) = img.dimensions();
    let gradient_map = create_gradient_map(color_a, color_b);
    println!("entering for loop");

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
    return img;
}

pub fn halftone(mut img: DynamicImage) -> DynamicImage {
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
            img.put_pixel(x, y + 1, px2);
         }
    }
    return img;
}
/// Reduces an image to the primary colours.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to add a primary colour effect to an image of type `DynamicImage`:
/// use photon::effects;
/// photon::effects::primary(img);
/// ```
pub fn primary(mut img: DynamicImage) -> DynamicImage {
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
    return img;
}

/// Colorizes the green channels of the image.
/// 
/// # Arguments
/// * `img` - A DynamicImage that contains a view into the image.
/// # Example
///
/// ```
/// // For example, to colorize an image of type `DynamicImage`:
/// use photon::effects;
/// photon::effects::colorize(img);
/// ```
pub fn colorize(mut img: DynamicImage) -> DynamicImage {
    let threshold = 220;
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let px_as_rgb = Rgb{r: px.data[0], g: px.data[1], b: px.data[2]};

            let baseline_color = Rgb{r: 0, g: 255, b: 255};

            let square_distance = square_distance(baseline_color, px_as_rgb);

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
    return img;

}

pub fn inc_luminosity(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut min_intensity = 255;
    let mut max_intensity = 0;

    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);
            let intensity = (px.data[0] as u32 + px.data[1] as u32 + px.data[2] as u32) / 3;
            if intensity > 0{
                min_intensity = cmp::min(min_intensity, intensity);
                max_intensity = cmp::max(max_intensity, intensity);
            }
            
        }
    }

    // we have found the max and min intensities in the image

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            // let px_as_rgb = Rgb{r: px.data[0], g: px.data[1], b: px.data[2]};

            let mut r = px.data[0] as f32;
            let mut g = px.data[1] as f32;
            let mut b = px.data[2] as f32;

            let lum = (r + g + b) / 3.0;

            let new_lum = 255.0 * (lum - min_intensity as f32) / (max_intensity / min_intensity) as f32;

            r = r * new_lum / lum;
            g = g * new_lum / lum;
            b = b * new_lum / lum;
            


            px.data[0] = r as u8;
            px.data[1] = g as u8;
            px.data[2] = b as u8;

            img.put_pixel(x, y, px);
        }
    }
    return img;
}

// Gets the square distance between two colours
pub fn square_distance(color1 : Rgb, color2 : Rgb) -> i32{
    let (r1, g1, b1) = (color1.r as i32, color1.g as i32, color1.b as i32);
    let (r2, g2, b2) = (color2.r as i32, color2.g as i32, color2.b as i32);
    return i32::pow(r1 - r2, 2) + i32::pow(g1 - g2, 2) + i32::pow(b1 - b2, 2);
}