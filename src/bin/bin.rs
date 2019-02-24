extern crate photon;
extern crate image;
use photon::effects::Rgb;

fn main() {
    let img = image::open("test.JPG").unwrap();
    let colorA: Rgb = Rgb {r: 120, g: 100, b: 30 };
    let colorB: Rgb = Rgb {r: 110, g: 30, b: 90 };
    
    let filtered_img = photon::effects::primary(img);
    
    // Write the contents of this image in PNG format.
    filtered_img.save("test.png").unwrap();
}