extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use image::Pixel;
use rand::Rng;
use std::cmp;

pub mod filters;
pub mod channels;
pub mod noise;

struct Rgb {
    r: u32,
    g: u32,
    b: u32
}

pub fn threshold(mut img: DynamicImage, threshold: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);

            let r: f32 = px.data[0].into();
            let g: f32 = px.data[1].into();
            let b: f32 = px.data[2].into();

            let mut v = (0.2126 * r + 0.7152 * g + 0.072 * b);

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

pub fn grayscale(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as u32, px.data[1] as u32, px.data[2] as u32);
            let mut avg = (r_val + g_val + b_val) / 3;
            if (avg >= 255) {
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

pub fn grayscale_human_corrected(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            let (r_val, g_val, b_val) = (px.data[0] as f32, px.data[1] as f32, px.data[2] as f32);

            let mut avg = (r_val * 0.3 + g_val * 0.59 + b_val * 0.11);
            
            if (avg >= 255.0) {
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

            if (gray >= 255) {
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

            if (gray >= 255) {
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

pub fn grayscale_shades(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            // ConversionFactor = 255 / (NumberOfShades - 1)
            // AverageValue = (Red + Green + Blue) / 3
            // Gray = Integer((AverageValue / ConversionFactor) + 0.5) * ConversionFactor
            let mut px = img.get_pixel(x, y);

            let shade_num = 2.0;
            let conversion: f32 = 255.0 / (shade_num - 1.0);
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

pub mod effects {
    extern crate image;
    extern crate rand;
    use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
    use rand::Rng;
    
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
}