extern crate photon_rs;
extern crate time;

use instant::Instant;
use photon_rs::channels::alter_red_channel;
use photon_rs::native::{open_image, save_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("examples/input_images/daisies_fuji.jpg")?;

    let start = Instant::now();
    // Increment the red channel by 40
    alter_red_channel(&mut img, 40_i16);

    let output_img_path = "output.jpg";

    // Write file to filesystem.
    save_image(img, output_img_path)?;
    let end = Instant::now();
    println!(
        "Took {} seconds to increment red channel by 40 on image.",
        (end - start).as_secs_f64()
    );

    println!(
        "Saved image: {}. Please check this directory for the image.",
        output_img_path
    );

    Ok(())
}
