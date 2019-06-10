extern crate photon;
extern crate time;
extern crate image;
use time::PreciseTime;
use image::{GenericImageView};
use photon::{Rgb};

fn main() {
    // Replace the variable file_name with whatever image you would like to apply filters to
    // Ensure it is in the example_output directory, which can be found one sub-dir inside the photon dir.
    // However the image referenced below, along with sample images, have been included in the dir.
    let file_name = "examples/input_images/underground.JPG";

    // Open the image
    let effects = ["saturate", "desaturate", "lighten", "darken", "shift_hue"];

    for i in 0..effects.len() {
        let img = photon::helpers::open_image(file_name);
        let start = PreciseTime::now();

        // Apply the effect in the HSV colour space
        let filtered_img = photon::colour_spaces::hsl(img, effects[i], 0.2);

        // Write the contents of this image in JPG format.
        photon::helpers::save_image(filtered_img, &("examples/example_output/".to_owned() + &effects[i].to_owned() + ".JPG"));
    
        let end = PreciseTime::now();
        println!("Took {} seconds to {} image.", start.to(end), effects[i]);
    }
    println!("Check example_output dir for filtered images.\nYou can compare them with the original in {}", file_name);
}
