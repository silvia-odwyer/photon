extern crate photon_rs;
use photon_rs::base64_to_image;
use photon_rs::helpers;
use photon_rs::native::{open_image, save_image};

fn main() {
        // Start time
        let start = PreciseTime::now();
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("fruit_1920_1080.jpg");

    // let distance_vec: Vec<f32> = photon_rs::channels::distance_runtime_select(row, row2, row3, row4);
    // println!("Distance: {:?}", distance_vec);

    // Increment the red channel by 40
    // let resized_img = resize(&mut img, 800, 600, SamplingFilter::Triangle);
    photon_rs::channels::alter_red_channel(&mut img, 30);

    let output_img_path = "output.jpg";

    let output_img_path = "raw_image.jpg";

    // Write file to filesystem.
    save_image(img, output_img_path);

    println!(
        "Saved image: {}. Please check this directory for the image.",
        output_img_path
    );
}
