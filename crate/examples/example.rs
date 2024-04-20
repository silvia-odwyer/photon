extern crate image;
extern crate photon_rs as photon;
use instant::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace the variable file_name with whatever image you would like to apply filters to
    // Ensure it is in the example_output directory, which can be found one sub-dir inside the photon dir.
    // However the image referenced below, along with sample images, have been included in the dir.
    let file_name = "crate/examples/input_images/underground.jpg";

    // Open the image
    let effects: [&str; 5] =
        ["saturate", "desaturate", "lighten", "darken", "shift_hue"];

    for &effect in effects.iter() {
        let mut img = photon::native::open_image(file_name)?;
        let start = Instant::now();

        // Apply the effect in the HSV colour space
        photon::colour_spaces::hsl(&mut img, effect, 0.2_f32);

        // Write the contents of this image in JPG format.
        photon::native::save_image(img, &format!("output_{}.jpg", effect)[..])?;

        let end = Instant::now();
        println!(
            "Took {} seconds to {} image.",
            (end - start).as_secs_f64(),
            effect
        );
    }
    println!("Check example_output dir for filtered images.\nYou can compare them with the original in {}", file_name);

    Ok(())
}
