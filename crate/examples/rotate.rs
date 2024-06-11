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
    let angles = vec![
        60.0,  //   60.0 = q1:60.0
        135.0, //  135.0 = q2:45.0
        562.5, //  517.5 = q3:22.5
        -30.0, //  -30.0 = q4:60.0
    ];
    let operations = angles.len();
    let mut results = Vec::new();
    for i in 0..operations {
        let angle = angles[i];
        let result = photon_rs::transform::rotate(&img, angles[i]);
        println!(
            "after rotate({}) = w: {}, h: {}",
            angle,
            result.get_width(),
            result.get_height()
        );
        results.push(result);
    }

    // Write all outputs in JPEG format.
    for i in (0..operations).rev() {
        photon_rs::native::save_image(
            results.remove(i),
            &format!("output_rotate_{}.jpg", i + 1),
        )?;
    }
    let end = Instant::now();
    println!(
        "Took {} seconds to rotate {} images.",
        (end - start).as_secs_f64(),
        operations
    );

    Ok(())
}
