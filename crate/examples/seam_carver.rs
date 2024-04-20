extern crate image;
extern crate photon_rs;

use instant::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "crate/examples/input_images/daisies_fuji.jpg";
    println!("file name = {}", file_name);

    // // Open the image
    let img = photon_rs::native::open_image(file_name)?;
    let start = Instant::now();
    // Seam Carver
    let (w, h) = (img.get_width(), img.get_height());
    println!("original = w: {}, h: {}", w, h);
    let w = w - 60;
    let h = h - 10;
    let res = photon_rs::transform::seam_carve(&img, w, h);
    println!("after = w: {}, h: {}", w, h);

    // Write the contents of this image in JPEG format.
    photon_rs::native::save_image(res, "output_seam_carver.jpg")?;
    let end = Instant::now();
    println!(
        "Took {} seconds to seam carve image.",
        (end - start).as_secs_f64()
    );

    Ok(())
}
