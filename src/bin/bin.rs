extern crate photon;
extern crate time;
use time::PreciseTime;
// use photon::effects::Rgb;

fn main() {
    let img = photon::helpers::open_image("background3.JPG");
    
    let filtered_img = photon::conv::sobel_vertical(img);
    
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(filtered_img, "new_image.PNG");
}

fn testDuration() {
    let start = PreciseTime::now();
    let img = photon::helpers::open_image("background3.JPG");
    
    let filtered_img = photon::conv::sobel_vertical(img);
    
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(filtered_img, "new_image.PNG");

    let end = PreciseTime::now();

    println!("Took {} seconds to process image.", start.to(end));
}