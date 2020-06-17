extern crate photon_rs;
use photon_rs::helpers;
use photon_rs::native::{open_image, save_image};

fn main() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("fruit_1920_1080.jpg");

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 30);

    let output_img_path = "output.jpg";

    // Write file to filesystem.
    save_image(img, output_img_path);

    println!(
        "Saved image: {}. Please check this directory for the image.",
        output_img_path
    );
}
