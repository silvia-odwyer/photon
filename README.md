# photon
An image processing library in Rust, that also compiles to Web Assembly.

The library provides low-level access to pixel and channel manipulation, and provides presets 
for common image processing functions. 

Standard functions include:
- thresholding
- convolutions
- edge-detection
- Sobel filters
- altering channels
- altering R, G, B channel values.
- greyscaling 
- increasing saturation
- brightness

View the [official website](https://silvia-odwyer.github.io/photon).

## Install The Crate via Cargo
`photon` can be installed via Cargo by declaring the following dependency in your Cargo.toml file:
```toml
[dependencies]
photon-rs = "*"
```

## Use 
```rust
extern crate photon;
extern crate image;

fn main() {
    let img = image::open("background3.JPG").unwrap();
    
    let filtered_img = photon::effects::threshold(img);
    
    // Write the contents of this image in PNG format.
    filtered_img.save("test.png").unwrap();
}
```

## Documentation
View the official [Cargo documentation here](https://cargo.io/photon-rs). 

## Issues/New Filters
Photon is always ready for new filters and functions, so if you'd like to contribute, just submit a Pull Request. :)

### Maintainers
Creator and Maintainer - [Silvia O'Dwyer](https://github.com/silvia-odwyer)