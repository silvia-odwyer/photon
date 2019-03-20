extern crate photon;
extern crate image;
use photon::effects::Rgb;

fn main() {
    let img = image::open("background3.JPG").unwrap();
    // let color_a: Rgb = Rgb {r: 120, g: 100, b: 30 };
    // let color_b: Rgb = Rgb {r: 110, g: 30, b: 90 };
    
    let filtered_img = photon::conv::emboss(img);
    
    // Write the contents of this image in PNG format.
    filtered_img.save("test.png").unwrap();
}