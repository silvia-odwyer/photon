extern crate image;
extern crate photon;
use photon::native::{open_image, save_image};
use time::{PreciseTime};

fn main() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("examples/input_images/daisies_fuji.jpg");
    let start = PreciseTime::now();

    // Increment the red channel by 40
    photon::channels::alter_red_channel(&mut img, 40);

    // Write file to filesystem.
    save_image(img, "raw_image.jpg");    

    // Print time taken to process image
    let end = PreciseTime::now();
    println!("Took {} seconds to process image.", start.to(end));

}