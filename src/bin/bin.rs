extern crate photon;
extern crate image;

fn main() {
    let img = image::open("test.JPG").unwrap();

    let filtered_img = photon::tint(img, 50, 100, 20);
    
    // Write the contents of this image in PNG format.
    filtered_img.save("test.png").unwrap();
}