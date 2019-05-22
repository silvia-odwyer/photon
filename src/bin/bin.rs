extern crate photon;
extern crate time;
extern crate image;
use time::PreciseTime;
use image::{GenericImage, DynamicImage, GenericImageView};
use photon::effects::{Rgb};

fn main() {
    let start = PreciseTime::now();

    let img = photon::helpers::open_image("daisies.jpg");
       
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(filtered_img, "new_image.PNG");

    let end = PreciseTime::now();
    println!("DYNAMICIMAGE (prev impl): Took {} seconds to process image.", start.to(end));

    selective_color_change();
    
    bench();
}

// Compare two methods of pixel manipulation; one involves creating a DynamicImage from a raw vec of u8s, the other involves 
// working with the raw vec directly. Performance metrics are printed to the console.
fn bench() {
    // Create an image buffer from a vec of u8s
    test_dyn_image_from_raw();

    test_raw_pixel_vec();
}

fn selective_color_change() {
    let color = Rgb{r: 10, g: 50, b: 70};
    let new_color = Rgb{r: 90, g: 50, b: 20};
    let filtered_img = photon::channels::selective_color_change(img, color, new_color);
    photon::helpers::save_image(filtered_img, "selective_color_change.PNG");

}

fn test_duration() {
    let start = PreciseTime::now();
    let img = photon::helpers::open_image("original.JPG");

    let filt_img = dyn_image(img);
    
    // let filtered_img = photon::noise::noise_gen(img);
    
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(filt_img, "new_image.JPG");

    let end = PreciseTime::now();

    println!("Took {} seconds to process image.", start.to(end));
}

fn noise_test() {
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

fn dyn_image(mut img: DynamicImage) -> DynamicImage {
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
    let start_dyn_raw = PreciseTime::now();
    let raw_pixels = photon::helpers::get_pixels("newValley.PNG");

    let image = photon::helpers::open_image("newValley.PNG");

    let (width, height) = image.dimensions();
    let dynimage = photon::helpers::dyn_image_from_raw(raw_pixels, width, height);

    let new_img = photon::channels::inc_red_channel(dynimage, 40);
    photon::helpers::save_image(new_img, "dynimage.PNG");
    let end_dyn_raw = PreciseTime::now();
    let total_dyn_raw = start_dyn_raw.to(end_dyn_raw);
    println!("RAW PIXEL VEC to DYNAMICIMAGE: Took {} seconds to process image.", total_dyn_raw);

    // image::save_buffer("image.png", &raw_pixels, width, height, image::RGB(8)).unwrap();
}

fn test_raw_pixel_vec() {
    // save image from a raw pixel vec of u8s
    let start_dyn_raw = PreciseTime::now();
    let mut raw_pixels = photon::helpers::get_pixels("newValley.PNG");
    let image = photon::helpers::open_image("newValley.PNG");
    let (width, height) = image.dimensions();

    for pixel in raw_pixels.iter_mut() {
        if *pixel + 30 <= 255 {
            *pixel += 30;
        }
    }
    let dynimage = photon::helpers::dyn_image_from_raw(raw_pixels, width, height);
    photon::helpers::save_image(dynimage, "vec_image.PNG");

    let end_dyn_raw = PreciseTime::now();
    let total_dyn_raw = start_dyn_raw.to(end_dyn_raw);
    println!("RAW PIXEL VEC ONLY: Took {} seconds to process image.", total_dyn_raw);

    // image::save_buffer("image.png", &raw_pixels, width, height, image::RGB(8)).unwrap();
}