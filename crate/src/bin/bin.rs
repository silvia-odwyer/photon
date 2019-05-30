extern crate image;
use image::{GenericImageView, GenericImage, ImageBuffer};
extern crate photon;
use photon::Rgb;

fn main() {
    // Open the image (a PhotonImage is returned) and get dimensions
    let mut img = photon::helpers::open_image("fruit.jpg");

    // Apply a filter to the pixels
    let rgb1 = Rgb{r: 120, g: 130, b: 54};
    let rgb2 = Rgb{r: 123, g: 12, b: 10};
    let new_img = photon::conv::sharpen(img);

    // Write the contents of this image in JPG format.
    photon::helpers::save_image(new_img, "blended.JPG");
}