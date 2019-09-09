extern crate image;
use image::{GenericImageView, GenericImage, ImageBuffer};
extern crate photon;
use photon::Rgb;
use std::io::Read;
use std::string::String;
use photon::native::{open_image, save_image};
use time::{PreciseTime};

fn main() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("examples/input_images/fruit.jpg");
    let img2 = open_image("examples/input_images/fruit_med.jpg");
    let watermark = photon::native::open_image("examples/input_images/watermark.jpg");

    // Apply a filter to the pixels
    let rgb1 = Rgb{r: 120, g: 130, b: 54};
    let rgb2 = Rgb{r: 44, g: 155, b: 244};
    let start = PreciseTime::now();
    photon::channels::alter_red_channel(&mut img, 70);
    let end = PreciseTime::now();
    println!("RAW PIXELS: Took {} seconds to process image.", start.to(end));
    save_image(img, "raw_image.png");    


    let mut clone_img = open_image("examples/input_images/fruit.jpg");
    let start = PreciseTime::now();
    photon::channels::alter_red_channel_dyn(&mut clone_img, 70);
    let end = PreciseTime::now();
    println!("DYNAMIC IMAGE: Took {} seconds to process image.", start.to(end));
    save_image(clone_img, "new_image.png");    
    // Write the contents of this image in JPG format.

    
}