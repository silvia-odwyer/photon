extern crate image;
extern crate rand;
extern crate num;
use image::{DynamicImage, GenericImageView};
use palette::{Hsl, Lch, Shade, Pixel, Saturate, Srgb, Hue, Hsv};

// Image manipulation effects in the LCh colour space
pub fn lch(img: DynamicImage, mode: &'static str, amt: f32) -> DynamicImage {
   
    let (width, height) = img.dimensions();
    let mut img = img.to_rgb();
    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y);

            let px_data = img.get_pixel(x, y).data;
            let lch_colour: Lch = Srgb::from_raw(&px_data)
                .into_format()
                .into_linear()
                .into();

            let new_color = match mode {
                // Match a single value
                "desaturate" => lch_colour.desaturate(amt),
                "saturate" => lch_colour.saturate(amt),
                "lighten" => lch_colour.lighten(amt), 
                "darken" => lch_colour.darken(amt),
                _ => lch_colour.saturate(amt),
            };
            
            img.put_pixel(x, y, image::Rgb {
                data: Srgb::from_linear(new_color.into()).into_format().into_raw()
            });

            }
        }
    
    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

// Image manipulation effects in the HSL colour space
// The function logic is kept separate from other colour spaces for now, 
// since other HSL-specific logic may be implemented here, which isn't available in other colour spaces
pub fn hsl(img: DynamicImage, mode: &'static str, amt: f32) -> DynamicImage {
    let mut img  = img.to_rgb();
    let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let colour = Srgb::from_raw(&px_data).into_format();

                let hsl_colour = Hsl::from(colour);
                
                let new_color = match mode {
                    // Match a single value
                    "desaturate" => hsl_colour.desaturate(amt),
                    "saturate" => hsl_colour.saturate(amt),
                    "lighten" => hsl_colour.lighten(amt), 
                    "darken" => hsl_colour.darken(amt),
                    _ => hsl_colour.saturate(amt),
                };

                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(new_color.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

pub fn hsv(img: DynamicImage, mode: &'static str, amt: f32) -> DynamicImage {
    let mut img  = img.to_rgb();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgb::from_raw(&px_data).into_format();

                let hsv_colour = Hsv::from(color);

                let new_color = match mode {
                    // Match a single value
                    "desaturate" => hsv_colour.desaturate(amt),
                    "saturate" => hsv_colour.saturate(amt),
                    "lighten" => hsv_colour.lighten(amt), 
                    "darken" => hsv_colour.darken(amt),
                    _ => hsv_colour.saturate(amt),
                };

                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(new_color.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

// Shift hue by a specified number of degrees in the HSL colour space.
pub fn hue_rotate_hsl(img: DynamicImage, degrees: f32) -> DynamicImage {
    let mut img  = img.to_rgb();

       let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgb::from_raw(&px_data).into_format();

                let hue_rotated_color = Hsl::from(color).shift_hue(degrees);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(hue_rotated_color.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

// Shift hue by a specified number of degrees in the LCh colour space.
pub fn hue_rotate_lch(img: DynamicImage, degrees: f32) -> DynamicImage {
    let mut img  = img.to_rgb();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgb::from_raw(&px_data).into_format();

                let hue_rotated_color = Lch::from(color).shift_hue(degrees);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(hue_rotated_color.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

// Increase the image's saturation by converting each pixel's colour to the HSL colour space
// and increasing the colour's saturation. 
// The level must be from 0 to 1 in floating-point, `f32` format.
// Increasing saturation by 80% would be represented by a level of 0.8
pub fn saturate_hsl(img: DynamicImage, level: f32) -> DynamicImage {
    return hsl(img, "saturate", level);
}

// Increase the image's saturation by converting each pixel's colour to the LCh colour space
// and increasing the colour's saturation. 
// The level must be from 0 to 1 in floating-point, `f32` format.
// Increasing saturation by 80% would be represented by a level of 0.8
pub fn saturate_lch(img: DynamicImage, level: f32) -> DynamicImage {
    return lch(img, "saturate", level);
}

pub fn saturate_hsv(img: DynamicImage, level: f32) -> DynamicImage {
    return hsv(img, "saturate", level);
}

// Lighten a colour by a specified amount in the LCh colour space.
pub fn lighten_lch(img: DynamicImage, level: f32) -> DynamicImage {
    return lch(img, "lighten", level);
}

// Lighten a colour by a specified amount in the HSL colour space.
pub fn lighten_hsl(img: DynamicImage, level: f32) -> DynamicImage {
    return hsl(img, "lighten", level);
}

// Lighten a colour by a specified amount in the HSV colour space.
pub fn lighten_hsv(img: DynamicImage, level: f32) -> DynamicImage {
    return hsv(img, "lighten", level);
}


// Darken the image's colours by a specified amount in the LCh colour space.
pub fn darken_lch(img: DynamicImage, level: f32) -> DynamicImage {
    return lch(img, "darken", level);
}

// Darken the image's colours by a specified amount in the HSL colour space.
pub fn darken_hsl(img: DynamicImage, level: f32) -> DynamicImage {
    return hsl(img, "darken", level);
}

// Darken the image's colours by a specified amount in the HSV colour space.
pub fn darken_hsv(img: DynamicImage, level: f32) -> DynamicImage {
    return hsv(img, "darken", level);
}
