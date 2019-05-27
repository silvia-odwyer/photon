extern crate photon;
extern crate time;
extern crate image;
use time::PreciseTime;
use image::{GenericImageView};
use photon::effects::{Rgb};

fn main() {
    // Replace the variable file_name with whatever image you would like to apply filters to
    // Ensure it is in the example_output directory, which can be found one sub-dir inside the photon dir.
    // However the image referenced below, along with sample images, have been included in the dir.
    let file_name = "example_output/fruit_demo.jpg";

    // Open the image
    let img = photon::helpers::open_image(file_name);
    let effects = ["saturate", "desaturate", "lighten", "darken", "shift_hue"];

    for i in 0..effects.len() {
        let start = PreciseTime::now();

        // Apply the effect in the HSV colour space
        let filtered_img = photon::correction::hsl(&img, effects[i], 0.2);

        // Write the contents of this image in JPG format.
        photon::helpers::save_image(filtered_img, &("example_output/".to_owned() + &effects[i].to_owned() + ".JPG"));
    
        let end = PreciseTime::now();
        println!("Took {} seconds to {} image.", start.to(end), effects[i]);
    }

    let filtered_img = photon::correction::hue_rotate_lch(&img, 180.0);
    photon::helpers::save_image(filtered_img,  "hue_rotated.JPG");

    println!("Check example_output dir for filtered images.\nYou can compare them with the original called fruit_demo.jpg");
}
