# photon

Photon is a high-performance Rust image processing library, which compiles to WebAssembly, allowing for 
safe, blazing-fast image processing both natively and on the web. 

The library provides low-level access to pixel and channel manipulation, as well as high-level functions for image correction, filtering, watermarking, and convolutions. 

<!-- ### Features:
- **Pure Rust** - Unlike other libraries, this library is built with 100% pure Rust, so security and safety is guaranteed. 
- **WebAssembly friendly** - For web-based image processing, Photon is 4-10x faster than JS, leading to faster results, and less lag. 
- **Call WASM with JS** - This library's WASM functions can be called via JS, allowing for zero-cost abstraction and faster development.
- **Over 90 functions** - Photon provides functions for every domain of image processing. 

#### Photon vs Other Libraries:
- **ImageMagick** - Benchmarks show that Photon is approximately 3x faster than ImageMagick.  
- **ImageFlow** - ImageFlow is 60% Rust, and still relies on C++ for its image encoding and decoding, however Photon is 100% Rust, guaranteeing full safety and security. ImageFlow also does not contain JS functions for front-end development. 
- **CamanJS** - Photon is approximately 8x faster than CamanJS, and contains over 50 more functions.  -->

### Functions
96 customisable functions are available, for varying image effects.

Functions include:
- **Image correction**: Hue rotation, sharpening, brightness adjustment, adjusting saturation, lightening/darkening all within various colour spaces. 
- **Convolutions**: Sobel filters, blurs, Laplace effects, edge detection, etc., 
- **Channel manipulation**: Increasing/decreasing RGB channel values, swapping channels, removing channels, etc.
- **Monochrome effects**: Duotoning, greyscaling of various forms, thresholding, sepia, averaging RGB values
- **Colour manipulation**: Work with the image in various colour spaces such as HSL, LCh, and sRGB, and adjust the colours accordingly. 
- **Filters**: Over 30 pre-set filters available, incorporating various effects and transformations. 
- **Text**: Apply text to imagery in artistic ways, or to watermark, etc.,
- **Watermarking**: Watermark images in multiple formats. 
- **Blending**: Blend images together using 10 different techniques, change image backgrounds. 

### Get Started
- [Get Started with WebAssembly](https://github.com/silvia-odwyer/photon#-get-started-with-webassembly)
- [Get Started Natively](https://github.com/silvia-odwyer/photon#getting-started)
- [Check Out The Documentation](https://silvia-odwyer.github.io/photon/docs/photon/all.html)
- [View the Official Website.](https://silvia-odwyer.github.io/photon/)
- [See All Available Functions.](https://silvia-odwyer.github.io/photon/docs/photon/all.html)

`photon` can be thought of as a high-level wrapper to the Rust `image` crate, but conversely also includes functions which provide low-level access to pixel and channel manipulation, perfect for developers who wish to work with this data directly.

### 🧐 Repo Organisation
This repo can be thought of as a hybrid library, divided into 2 major components:
1. **Core Rust library**: This provides universal image processing functions, which can be used either natively or in the browser. The WASM component of this repo relies on this for all image processing functionality. 
2. **WebAssembly helpers and starter demo**: This section of the library contains helpers for exporting pixel data to and from the browser, and communicates directly with the core Rust library. The starter demo demonstrates calling the compiled WebAssembly code using JS functions and hooks into a webpack build pipeline. 

### Live Demo
View the [official demo of WASM in action](https://silvia-odwyer.github.io/photon/demo.html).

### 📚 Documentation
View the [official documentation](https://silvia-odwyer.github.io/photon/docs/photon/index.html).

## 🚴Getting Started

These instructions will get you a copy of Photon up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

To use Photon, you must have Rust and Node installed. Builds of Photon are compiled using the rust nightly toolchain.

### Installing
<!-- ## Cargo Status -->
<!-- `photon` can be installed via Cargo by declaring the following dependency in your Cargo.toml file:
```toml
[dependencies]
photon-rs = "*"
``` -->

Clone this repo:
```sh
git clone https://github.com/silvia-odwyer/photon
```

### Run Natively
Run the binary, which will perform an image processing function on an image:
```sh
cd crate
cargo run --release 
```

Compare the original image with the outputted image, and you'll see the desired effect has been applied.

### Native Use
Photon contains native-only functions for opening files from the filesystem. 

When an image is opened, it is converted to a `PhotonImage` type, which can then be passed into any image processing function, and the `PhotonImage` value is accordingly edited.

Getting started is relatively straightforward, this code snippet is all you need to get started:
```rust
extern crate photon;
fn main() {
    // Open the image as a PhotonImage type
    let img = photon::helpers::open_image("daisies.JPG");
    
    // Apply a Sobel effect to the image 
    photon::conv::sobel_vertical(&mut img);
    
    // Write the contents of this image in PNG format.
    photon::helpers::save_image(img, "new_image.PNG");
}
```

See the documentation for a full list of effects which you can apply. All functions take in a `PhotonImage` similar to above.

### 🚴 Get Started With WebAssembly 

#### 🔋 Batteries Included

This repo comes pre-configured with a quick-start demo, which hooks into a Webpack build pipeline, and provides all WASM-friendly functions.

<!-- 
***WARNING***: Running WASM code in development mode is ***significantly*** slower than in production mode (often up to 10 times),
so don't be disheartened if the JS alternatives outperform WASM. For the blazing speeds promised, make sure to build the 
project under production mode with `npm run build` and visit `dist/index.html`.  -->

To get started:

Clone this repo:
```sh
git clone https://github.com/silvia-odwyer/photon
```

Install the dependencies:
```sh
npm install
```

To compile the lib's Rust code to wasm, run:
```sh
npm run build
```

Serve the project locally for development at http://localhost:8080
```sh
npm run start 
# This serves the project locally for development at http://localhost:8080
```

Then navigate to http://localhost:8080 and you'll see a demo in action. 

To build the demo:
* `npm run build` -- Bundle the project (in production mode).

#### WebAssembly Use
To allow for universal communication between the core Rust library and WebAssembly, the functions have been generalised to allow for both native and in-browser use. 

Due to this, image data from the browser must first be converted to a PhotonImage before being passed to the image processing functions. 

The PhotonImage can then be converted back to JS-compatible ImageData so that it can be displayed in-browser.

See the code snippet below:
```js
function filterImage(event) {
    // Create a canvas and get a 2D context from the canvas
    var canvas = document.getElementById("canvas");
    var ctx = canvas.getContext("2d");
    
    // Draw the image element onto the canvas
    ctx.drawImage(newimg, 0, 0);
    
    // Convert the ImageData found in the canvas to a PhotonImage (so that it can communicate with the core Rust library)
    let rust_image = module.open_image(canvas, ctx);

    // Filter the image, the PhotonImage's raw pixels are modified
    module.filter(rust_image, "radio");
    
    // Place the pixels back on the canvas
    ctx.putImageData(rust_image, 0, 0)
  }
```

Not all functions available in the core Rust library are available in WebAssembly (currently investigating this). Only WASM-friendly functions have been annotated with #[wasm_bindgen]. All supported WASM functions are displayed in the starter demo. 

## Modules 
Photon contains a series of modules, which include:

- `effects`: Various image effects, including adding offsets, thresholding, duotoning, solarization, etc.,
- `channels`: Functions related to increasing/decreasing the red, green, and blue channels of the image data.
- `filters`: Preset filters, which alter the rgb channels of the image. Contains over 20. 
- `conv`: Laplace, Sobel, emboss; image proc functions which require image convolution. 
-  `noise`: Noise generation of varying tints and hues. 
- `multiple`: A module for dealing with multiple images, such as watermarking images, etc.,
- `correction`: Hue rotation, adjusting saturation, lightening/darkening: all techniques available in multiple colour spaces, which lead to varying effects.

All effects and filters can be viewed below and on the official website.

## Building For Production

#### Native
You can edit the binary file in the `bin` dir to create an executable or a command-line app, for example. 

Then, to build this executable for native use in release mode:

```sh
cd crate 
cargo build --release
```

#### WebAssembly
To build the example under production mode:

```sh
npm run build
```

Check the `dist` folder for the outputted static files, which can be deployed to a live server.

## To Do 
- Error detection and exception handling.

## Additional Notes
Functions have been designed with flexibility in mind, so that full customization of effects and filters can be utilised; for every function, hundreds of differing image effects/tints/hues can be created, just by changing parameters slightly, so with every function comes the ability to fully experiment. 

For developers who would like to work with high-level constructs can do so, such as applying effects to imagery (eg: Laplace or Sobel)
or filters; this library provides a complete suite of functions to do so, as well as in-built filters and presets. 

## Contributing

Photon is always ready for new filters and functions, so if you'd like to contribute, we're always ready to accept new Pull Requests or investigate new issues. 

## Authors

* **Silvia O'Dwyer** - [GitHub Profile](https://github.com/silvia-odwyer)
* **Future You(?)** - (See Contributing above ;) 

## License

This project is licensed under the Apache 2.0 License - see the [LICENSE.md](LICENSE.md) file for details.