extern crate photon;
extern crate time;
extern crate image;
use time::PreciseTime;
use image::{GenericImage, DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::fs::File;

// use photon::effects::Rgb;

fn main() {
    let start = PreciseTime::now();

    let img = photon::helpers::open_image("noir.JPG");
    
    let filtered_img = photon::filters::islands(img);
    
    // get an image's raw pixels as a vec of u8s
    // this is useful for direct raw pixel manipulation 
    let raw_pixels = photon::helpers::get_pixels("noir.JPG");

    // Create an image buffer from a vec of u8s
    let start_dyn_raw = PreciseTime::now();
    test_dyn_image_from_raw();
    let end_dyn_raw = PreciseTime::now();
    let total_dyn_raw = start_dyn_raw.to(end_dyn_raw);
    println!("RAW PIXEL VEC to DYNAMICIMAGE: Took {} second to process image.", total_dyn_raw);
    
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(filtered_img, "new_image.PNG");

    let end = PreciseTime::now();
    println!("Regular. Took {} seconds to process image.", start.to(end));

    //testDuration();
}

fn testDuration() {
    let start = PreciseTime::now();
    let img = photon::helpers::open_image("original.JPG");

    let filt_img = dynImage(img);
    
    // let filtered_img = photon::noise::noise_gen(img);
    
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(filt_img, "new_image.JPG");

    let end = PreciseTime::now();

    println!("Took {} seconds to process image.", start.to(end));
}

fn noiseTest() {
    let start = PreciseTime::now();
    let mut img: DynamicImage = photon::helpers::open_image("original.JPG");
    
    let (width, height) = img.dimensions();

    for x in 0 .. width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if px.data[0] < 253 {
                px.data[0] += 100;
            }
            img.put_pixel(x, y, px);
        }
    }

     // Write the contents of this image in PNG format.
    photon::helpers::save_image(img, "new_image.JPG");

    let end = PreciseTime::now();
    println!("Took {} seconds to process image.", start.to(end));

}

fn dynImage(mut img: DynamicImage) -> DynamicImage {
 let (width, height) = img.dimensions();

    for x in 0 .. width {
        for y in 0..height {
            let mut px = img.get_pixel(x, y);
            if px.data[0] < 253 {
                px.data[0] += 100;
            }
            img.put_pixel(x, y, px);
        }
    }
    return img;
}

fn test_dyn_image_from_raw() {
    // save image from a raw pixel vec of u8s
    let mut raw_pixels = photon::helpers::get_pixels("train.JPG");

    let len_vec = raw_pixels.len() as u128;

    // for i in 0..len_vec.step_by(2) {
    //     raw_pixels[i] = 189;
    // }
    // println!("{:?}", raw_pixels);
    let image = photon::helpers::open_image("train.JPG");

    let (width, height) = image.dimensions();
    let dynimage = photon::helpers::dyn_image_from_raw(raw_pixels, width, height);

    let new_img = photon::filters::islands(dynimage);
    photon::helpers::save_image(new_img, "dynimage.JPG");

    //benchmark results
    // raw pixels to dynamic image:
    // dynamic image immediate: 0.14

    // image::save_buffer("image.png", &raw_pixels, width, height, image::RGB(8)).unwrap();

}