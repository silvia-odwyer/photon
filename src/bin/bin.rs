extern crate photon;
extern crate time;
extern crate image;
use time::PreciseTime;
use image::{GenericImage, DynamicImage, GenericImageView};

// use photon::effects::Rgb;

fn main() {
    let start = PreciseTime::now();

    let img = photon::helpers::open_image("noir.JPG");
    
    let filtered_img = photon::filters::islands(img);
    
    // get an image's raw pixels as a vec of u8s
    // this is useful for direct raw pixel manipulation 
    let raw_pixels = photon::helpers::get_pixels("noir.JPG");
    
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(filtered_img, "new_image.PNG");

    let end = PreciseTime::now();
    println!("Took {} seconds to process image.", start.to(end));

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

fn get_raw_pixels() {
    // get the image's pixels as a vector
    let raw_pixels = photon::helpers::get_pixels("forev.JPG");
}