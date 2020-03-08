extern crate photon_rs;
use photon_rs::native::{open_image, save_image};
use photon_rs::base64_to_image;
use photon_rs::helpers;

fn main() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("examples/input_images/daisies_fuji.jpg");

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    let output_img_path = "raw_image.jpg";
    

    // Write file to filesystem.
    save_image(img, output_img_path);    

    println!("Saved image: {}. Please check this directory for the image.", output_img_path);
}