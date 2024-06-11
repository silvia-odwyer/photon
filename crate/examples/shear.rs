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
    let shear = 1.;

    let res1 = photon_rs::transform::shearx(&img, shear);
    println!(
        "after shearx({}) = w: {}, h: {}",
        shear,
        res1.get_width(),
        res1.get_height()
    );
    let res2 = photon_rs::transform::sheary(&img, shear);
    println!(
        "after sheary({}) = w: {}, h: {}",
        shear,
        res2.get_width(),
        res2.get_height()
    );

    // Write both outputs in JPEG format.
    photon_rs::native::save_image(res1, "output_shearx.jpg")?;
    photon_rs::native::save_image(res2, "output_sheary.jpg")?;
    let end = Instant::now();
    println!(
        "Took {} seconds to shear 2 images.",
        (end - start).as_secs_f64()
    );

    Ok(())
}
