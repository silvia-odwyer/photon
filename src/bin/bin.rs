extern crate photon;
use photon::effects::Rgb;

fn main() {
    let img = photon::helpers::open_image("background3.JPG");
    // let color_a: Rgb = Rgb {r: 120, g: 100, b: 30 };
    // let color_b: Rgb = Rgb {r: 110, g: 30, b: 90 };
    
    let filtered_img = photon::conv::emboss(img);
    
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(filtered_img, "new_image.PNG")
}