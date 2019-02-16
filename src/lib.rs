extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use image::Pixel;
use rand::Rng;
use std::cmp;

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

pub mod channels {
    extern crate image;
    use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
    use image::Pixel;
    pub fn alter_channel(mut img: DynamicImage, channel: usize, offset: u8) -> DynamicImage {
        let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let mut px = img.get_pixel(x, y);
                if px.data[channel] <= 255 - offset {
                    px.data[channel] += offset;
                }
                else {
                    px.data[channel] = 255;
                }
                img.put_pixel(x, y, px)
            }
        }
        return img;
    }

    pub fn alter_red_channel(mut img: DynamicImage, offset: u8) -> DynamicImage {
        let res_img = alter_channel(img, 0, offset);
        return res_img;
    }

    pub fn alter_green_channel(mut img: DynamicImage, offset: u8) -> DynamicImage {
        let res_img = alter_channel(img, 1, offset);
        return res_img;
    }

    pub fn alter_blue_channel(mut img: DynamicImage, offset: u8) -> DynamicImage {
        let res_img = alter_channel(img, 2, offset);
        return res_img;
    }

    pub fn alter_two_channels(mut img: DynamicImage, channel1: usize, offset1: u8, channel2: usize, offset2: u8) -> DynamicImage {
        let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let mut px = img.get_pixel(x, y);
                if px.data[channel1] <= 255 - offset1 {
                    px.data[channel1] += offset1;
                }
                else {
                    px.data[channel1] = 255;
                }
                
                if px.data[channel2] <= 255 - offset2 {
                    px.data[channel2] += offset2;
                }
                else {
                    px.data[channel2] = 255
                }
                img.put_pixel(x, y, px);
            }
        }
        return img;
    }

}

pub mod filters {
    extern crate image;
    use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
    use image::Pixel;

    // In-built image filters
    pub fn oceanic(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 9, 2, 173);
        return filtered_img;
    }

    pub fn islands(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 24, 2, 95);
        return filtered_img;
    }

    pub fn marine(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 14, 2, 119);
        return filtered_img;
    }

    pub fn seagreen(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 68, 2, 62);
        return filtered_img;
    }

    pub fn flagblue(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_blue_channel(img, 131);
        return filtered_img;
    }

    pub fn diamante(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 82, 2, 87);
        return filtered_img;
    }

    pub fn liquid(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 10, 2, 75);
        return filtered_img;
    }
}

pub mod noise {
    extern crate image;
    extern crate rand;
    use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
    use image::Pixel;
    use rand::Rng;
    // pub fn add_noise(mut img: DynamicImage, offset: u8) -> DynamicImage {
    //     // Add Gaussian Noise Sample with offset specified by the user.
    //     let (width, height) = img.dimensions();

    //     for x in 0..width {
    //         for y in 0..height {
    //             let px = img.get_pixel(x, y).map(|ch| if ch <= 255 - offset { ch + offset } else { 255});
    //             img.put_pixel(x, y, px);
    //     }
    //     }
    //     return img;
    // }

    pub fn add_noise_rand(mut img: DynamicImage) -> DynamicImage {
        // Add Gaussian Noise Sample by including a random offset to each channel in each pixel.
        let (width, height) = img.dimensions();
        let mut rng = rand::thread_rng();

        for x in 0..width {
            for y in 0..height {
                let offset = rng.gen_range(0, 150);
                let px = img.get_pixel(x, y).map(|ch| if ch <= 255 - offset { ch + offset } else { 255});
                img.put_pixel(x, y, px);
        }
        }
        return img;
    } 
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
