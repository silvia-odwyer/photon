extern crate photon;
extern crate image;
use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};

fn main() {
    let img = image::open("test.JPG").unwrap();

    let filtered_img = photon::threshold(img, 210);
    
    // Write the contents of this image in PNG format.
    filtered_img.save("test.png").unwrap();
}