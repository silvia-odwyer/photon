# photon

A high-performance Rust image processing library.

The library provides low-level access to pixel and channel manipulation, as well as high-level functions for convolution, thresholding, and edge-detection.

Benchmarks show that Photon is approximately 3x faster than ImageMagick.  

Functions include:
- thresholding
- convolutions
- sharpening
- edge-detection
- Sobel filters
- Laplace effects
- altering channels
- altering R, G, B channel values.
- greyscaling 
- increasing saturation
- brightness adjustment 

View all [functions here](https://silvia-odwyer.github.io/photon/docs/photon/all.html).

`photon` can be thought of as a high-level wrapper to the Rust `image` crate, but conversely also includes functions which provide low-level access to pixel and channel manipulation, perfect for developers who wish to work with this data directly.

## WebAssembly Support
Photon will also support WebAssembly in the near-future, compilable to wasm via Emscripten/wasm32-unknown-unknown. 

This will allow for universal image processing, including native-speed image processing within the browser. 
To support this, raw pixel vectors (of `u8s`) will be converted to `DynamicImage`s and can be processed via the regular Rust functions already found 
within this library. Raw pixel vectors can be fetched using the Canvas API with the `getPixelData` function (see PixelsJS for more info).

Benchmarks show that converting raw pixel vecs to `DynamicImage`s and writing to disk is similar in performance (faster in some cases) to operating on `DynamicImage`s directly.

View the [official website](https://silvia-odwyer.github.io/photon).

Documentation can be found [here](https://silvia-odwyer.github.io/photon/docs/photon/index.html).

## Examples
![](https://github.com/silvia-odwyer/photon/blob/master/docs/img_examples/streetlamp_collage.png)

## Live Demo
The GIF below shows an image cycling through various effects available. Don't take heed of the quality, since this is a GIF and 
merely for demonstration purposes. You should run the library's binary for a more thorough analysis.

![](https://github.com/silvia-odwyer/photon/blob/master/docs/img_examples/cube_demo.gif)

<!-- ## Cargo Status -->
<!-- `photon` can be installed via Cargo by declaring the following dependency in your Cargo.toml file:
```toml
[dependencies]
photon-rs = "*"
``` -->

## Install 
Clone this repo, then run:
```bash
cargo run --release 
```
which will run the binary file. Ensure you have an image with the same name as that in the bin file. 

## Use 
```rust
extern crate photon;
fn main() {
    let img = photon::helpers::open_image("daisies.JPG");
    
    let filtered_img = photon::conv::sobel_vertical(img);
    
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(filtered_img, "new_image.PNG");
}

```

## Modules 
Photon contains a series of modules, which include:

- `effects`: Various image effects, including adding offsets, thresholding, duotoning, solarization, etc.,
- `channels`: Functions related to increasing/decreasing the red, green, and blue channels of the image data.
- `filters`: Preset filters, which alter the rgb channels of the image. Contains over 20. 
- `conv`: Laplace, Sobel, emboss; image proc functions which require image convolution. 
-  `noise`: Noise generation of varying tints and hues. 
- `multiple`: A module for dealing with multiple images, such as watermarking images, etc.,

All effects and filters can be viewed below and on the official website.

## Documentation
View the official [documentation here](https://silvia-odwyer.github.io/photon/docs/photon/index.html). 

## To Do 
- Error detection and exception handling.
- wasm support
- New website

## Additional Notes
Functions have been designed with flexibility in mind, so that full customization of effects and filters can be utilised; for every function, hundreds of differing image effects/tints/hues can be created, just by changing parameters slightly, so with every function comes the ability to fully experiment. 

For developers who would like to work with high-level constructs can do so, such as applying effects to imagery (eg: Laplace or Sobel)
or filters; this library provides a complete suite of functions to do so, as well as in-built filters and presets. 

## Issues/New Filters
Photon is always ready for new filters and functions, so if you'd like to contribute, just submit a Pull Request. :)