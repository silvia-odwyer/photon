extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use rand::Rng;
use std::f64;
use std::cmp;

#[derive(Debug)]
pub struct Rgb {
    pub r: u32,
    pub g: u32,
    pub b: u32
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
pub fn offset(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();
    let offset = rng.gen_range(0, 150);
    for x in 0..width {
        for y in 0..height {

            let mut px = img.get_pixel(x, y);

            if x + offset < width - 1 && y + offset < height - 1  {
                let offset_px = img.get_pixel(x + offset, y + offset);
                px = offset_px;
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
            
            px.data[0] = cmp::max(0, cmp::min(255, r as u32)) as u8;
            px.data[1] = cmp::max(0, cmp::min(255, g as u32)) as u8;
            px.data[2] = cmp::max(0, cmp::min(255, b as u32)) as u8;
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

pub fn createGradientMap(colorA : Rgb, colorB: Rgb) -> Vec<Rgb> {
    println!("hi");
    println!("{}", colorA.r);
    let mut gradient_map = vec![];

    let maxVal = 255;
    let mut r_val = 0;

    for i in 0..maxVal + 1{
        let intensityB = maxVal - i;

        r_val = (i * colorA.r + intensityB * colorB.r) / maxVal as u32;
        println!("r_val {}", r_val);
        gradient_map.push(Rgb {
            r: r_val , 
            g: (i * colorA.g + intensityB * colorB.g) / maxVal as u32 ,
            b: (i * colorA.b + intensityB * colorB.b) / maxVal as u32
        });
    }
    println!("{:?}", gradient_map);
    return gradient_map;
}

pub fn duotone(mut img: DynamicImage, colorA : Rgb, colorB : Rgb) -> DynamicImage {
    let (width, height) = img.dimensions();
    let gradient_map = createGradientMap(colorA, colorB);
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
                g_val = 0;
            }

            px.data[0] = r_val;
            px.data[1] = g_val;
            px.data[2] = b_val;

            img.put_pixel(x, y, px);
         }
    }
    return img;
}
