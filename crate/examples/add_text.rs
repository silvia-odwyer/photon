extern crate image;
extern crate photon_rs as photon;
extern crate time;
use time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace the variable file_name with whatever image you would like to apply filters to
    // Ensure it is in the example_output directory, which can be found one sub-dir inside the photon dir.
    // However the image referenced below, along with sample images, have been included in the dir.
    let file_name = "crate/examples/input_images/daisies_fuji.jpg";

    // Open the image
    let mut img = photon::native::open_image(file_name)?;

    let start = Instant::now();

    // Add text to the image
    photon::text::draw_text_with_border(&mut img, "Welcome to Photon!", 10, 20);

    // Write the contents of this image in PNG format.
    photon::native::save_image(img, "output_new_image.png")?;
    let end = Instant::now();
    println!(
        "Took {} seconds to add text to image.",
        (end - start).as_seconds_f64()
    );

    println!("Check example_output dir for image with text applied.\nYou can compare them with the original in {}", file_name);

    Ok(())
}
