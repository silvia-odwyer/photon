# photon

![Photon is a Rust image processing library.](https://github.com/silvia-odwyer/photon/blob/master/photon_banner.JPG)

A Rust image processing library, which provides a suite of functions for low-level pixel manipulation, as well as functions to apply effects to imagery 
through convolution, thresholding, and edge-detection. 

The library provides low-level access to pixel and channel manipulation, and provides presets for common image processing functions. 

Functions include:
- thresholding
- convolutions
- edge-detection
- Sobel filters
- Laplace effects
- altering channels
- altering R, G, B channel values.
- greyscaling 
- increasing saturation
- brightness adjustment 

`photon` can be thought of as a high-level wrapper to the Rust image crate, but conversely also includes functions which provide low-level access to pixel and channel manipulation to developers who wish to do work with this data directly.

View the [official website](https://silvia-odwyer.github.io/photon).

## Examples
![](https://github.com/silvia-odwyer/photon/blob/master/img_examples/laplace.JPG)

![](https://github.com/silvia-odwyer/photon/blob/master/img_examples/remove_red_150.JPG)


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

## Modules 
Photon contains a series of modules, which include:

- `effects`: Various image effects, including adding offsets, thresholding, duotoning, solarization, etc.,
- `channels`: Functions related to increasing/decreasing the red, green, and blue channels of the image data.
- `filters`: Preset filters, which alter the rgb channels of the image. Contains over 20. 
- `conv`: Laplace, Sobel, emboss; image proc functions which require image convolution. 
-  `noise`: Noise generation of varying tints and hues. 

All effects and filters can be viewed below and on the official website.

## Documentation
View the official [Cargo documentation here](https://cargo.io/photon-rs). 

## To Do 
- Error detection and exception handling.

## Additional Notes
Functions have been designed with flexibility in mind, so that full customization of effects and filters can be utilised; for every function, hundreds of differing image effects/tints/hues can be created, just by changing parameters slightly, so with every function comes the ability to fully experiment. 

For developers who would like to work with high-level constructs can do so, such as applying effects to imagery (eg: Laplace or Sobel)
or filters; this library provides a complete suite of functions to do so, as well as in-built filters and presets. 

## Issues/New Filters
Photon is always ready for new filters and functions, so if you'd like to contribute, just submit a Pull Request. :)