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
    let file_name = "examples/input_images/daisies_fuji.jpg";

    // Open the image
    let img = photon::helpers::open_image(file_name);

    let start = PreciseTime::now();

    // Add text to the image
    let new_img = photon::text::draw_text_with_border(img, "Welcome to Photon!", 10, 20);

    // Write the contents of this image in PNG format.
    photon::helpers::save_image(new_img, "new_image.png");    
    let end = PreciseTime::now();
    println!("Took {} seconds to add text to image.", start.to(end));
    
    println!("Check example_output dir for image with text applied.\nYou can compare them with the original in {}", file_name);
}