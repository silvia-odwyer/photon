extern crate image;
use image::{GenericImageView, GenericImage, ImageBuffer};
extern crate photon;
use photon::Rgb;
use std::io::Read;
use std::string::String;
use photon::native::{open_image, save_image};

fn main() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("examples/input_images/tetris.png");
    let img2 = open_image("examples/input_images/fruit_med.jpg");
    let watermark = photon::native::open_image("examples/input_images/watermark.jpg");

    // Apply a filter to the pixels
    let rgb1 = Rgb{r: 120, g: 130, b: 54};
    let rgb2 = Rgb{r: 44, g: 155, b: 244};
    photon::channels::alter_two_channels(&mut img, 0, 60, 2, 0);

    // Write the contents of this image in JPG format.
    save_image(img, "new_image.png");
}