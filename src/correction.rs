extern crate image;
extern crate rand;
extern crate num;
use image::{DynamicImage, GenericImageView};
use palette::{Hsl, Lch, Shade, Pixel, Saturate, Srgb, Hue, Hsv, Hwb, Lab, Xyz, Yxy};

use image::GenericImage;


// Increase the image's saturation by converting each pixel's colour to the HSL colour space
// and increasing the colour's saturation. 
// The level must be from 0 to 1 in floating-point, `f32` format.
// Increasing saturation by 80% would be represented by a level of 0.8
pub fn saturate_hsl(img: DynamicImage, level: f32) -> DynamicImage {
    let mut img = img.to_rgb();
    {

        let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let hsl_colour: Hsl = Srgb::from_raw(&px_data)
                    .into_format()
                    .into_linear()
                    .into();

                let saturated = hsl_colour.saturate(level);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(saturated.into()).into_format().into_raw()
                });
            }
        }
    }
    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

// Increase the image's saturation by converting each pixel's colour to the LCh colour space
// and increasing the colour's saturation. 
// The level must be from 0 to 1 in floating-point, `f32` format.
// Increasing saturation by 80% would be represented by a level of 0.8
pub fn saturate_lch(img: DynamicImage, level: f32) -> DynamicImage {
    let mut img = img.to_rgb();
     {
        let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let lch_colour: Lch = Srgb::from_raw(&px_data)
                    .into_format()
                    .into_linear()
                    .into();

                let saturated = lch_colour.saturate(level);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(saturated.into()).into_format().into_raw()
                });
            }
        }
    }
    let dynimage = image::ImageRgb8(img);
    return dynimage;

}

// Shift hue by a specified number of degrees in the HSL colour space.
pub fn hue_rotate_hsl(mut img: DynamicImage, degrees: f32) -> DynamicImage {
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
pub fn hue_rotate_lch(mut img: DynamicImage, degrees: f32) -> DynamicImage {
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

// Lighten a colour by a specified amount in the LCh colour space.
pub fn lighten_lch(mut img: DynamicImage, level: f32) -> DynamicImage {
    let mut img  = img.to_rgb();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgb::from_raw(&px_data).into_format();

                let lightened = Lch::from(color).lighten(0.1);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(lightened.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

// Lighten a colour by a specified amount in the HSL colour space.
pub fn lighten_hsl(mut img: DynamicImage, level: f32) -> DynamicImage {
    let mut img  = img.to_rgb();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgb::from_raw(&px_data).into_format();

                let lightened = Hsl::from(color).lighten(0.1);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(lightened.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

// Lighten a colour by a specified amount in the HSV colour space.
pub fn lighten_hsv(mut img: DynamicImage, level: f32) -> DynamicImage {
    let mut img  = img.to_rgb();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgb::from_raw(&px_data).into_format();

                let lightened = Hsv::from(color).lighten(0.1);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(lightened.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}


// Darken the image's colours by a specified amount in the LCh colour space.
pub fn darken_lch(mut img: DynamicImage, level: f32) -> DynamicImage {
    let mut img  = img.to_rgb();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgb::from_raw(&px_data).into_format();

                let darkened = Lch::from(color).darken(0.1);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(darkened.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

// Darken the image's colours by a specified amount in the HSL colour space.
pub fn darken_hsl(mut img: DynamicImage, level: f32) -> DynamicImage {
    let mut img  = img.to_rgb();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgb::from_raw(&px_data).into_format();

                let darkened = Hsl::from(color).darken(0.1);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(darkened.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}

// Darken the image's colours by a specified amount in the HSV colour space.
pub fn darken_hsv(mut img: DynamicImage, level: f32) -> DynamicImage {
    let mut img  = img.to_rgb();

    let (width, height) = img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let px_data = img.get_pixel(x, y).data;

                let color = Srgb::from_raw(&px_data).into_format();

                let darkened = Hsv::from(color).darken(0.1);
                img.put_pixel(x, y, image::Rgb {
                    data: Srgb::from_linear(darkened.into()).into_format().into_raw()
                });
            }
        }

    let dynimage = image::ImageRgb8(img);
    return dynimage;
}