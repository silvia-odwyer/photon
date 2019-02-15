extern crate image;
extern crate rand;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
use image::Pixel;
use rand::Rng;

struct rgb {
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

pub mod channels {
    pub fn alter_channel(mut img: DynamicImage, channel: usize, offset: u8) -> DynamicImage {
        let (width, height) = img.dimensions();
        let mut rng = rand::thread_rng();

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
}