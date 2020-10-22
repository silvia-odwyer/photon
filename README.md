# photon

<p align="center">
  <a href="" rel="noopener">
 <img src="https://i.imgur.com/GxrKNOb.png" alt="Photon banner, showing the Photon logo on a dark background"></a>
</p>

<h3 align="center">Photon</h3>

<div align="center">

  [![Status](https://img.shields.io/badge/status-active-success.svg)]()
  [![GitHub Issues](https://img.shields.io/github/issues/silvia-odwyer/photon.svg)](https://github.com/silvia-odwyer/photon/issues)
  [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/silvia-odwyer/photon.svg)](https://github.com/silvia-odwyer/photon/pulls)
  [![Gitter chat](https://badges.gitter.im/silvia-odwyer/photon.png)](https://gitter.im/photonlibrary/community "Gitter chat")
  [![NPM Monthly Downloads](https://img.shields.io/npm/dm/@silvia-odwyer/photon.png)]()

</div>

---

<p align="center"> High-performance, cross-platform Rust/WebAssembly image processing library
    <br>
</p>

## 📝 Table of Contents
- [Get Started with WebAssembly](https://github.com/silvia-odwyer/photon#get-started-with-webassembly)
- [Get Started Natively](https://github.com/silvia-odwyer/photon#get-started-natively)
- [Documentation](https://docs.rs/photon-rs/)
- [Official Website](https://silvia-odwyer.github.io/photon/)
- [All Available Functions](https://silvia-odwyer.github.io/photon/docs/photon/all.html)
- [Got Questions? Ask Here!](https://github.com/silvia-odwyer/photon#got-questions)

Photon is a high-performance Rust image processing library, which compiles to WebAssembly, allowing for
safe, blazing-fast image processing both natively and on the web.

You can run Photon:
- natively
- in the browser with WebAssembly
- on Node with WebAssembly

### Features
- **Fast:** Photon outperforms even the fastest of libraries, including ImageMagick. On the web, its high-performance allows it to run at near-native speed. Benchmarks can be found [here](https://github.com/silvia-odwyer/photon/wiki/Benchmarks).
- **Call with JS:** Want to use Photon on the web or with Node? Using a simple npm package, you're good to go. Get all the benefits of WebAssembly
with zero-cost abstraction.
- **Use Natively:** For command-line apps, native photo editing apps, and so forth, Photon's core codebase is in Rust, allowing for cross-platform
development.
- **Pure Rust** - Unlike other libraries, 100% of the library's codebase is written in Rust, so security and safety is guaranteed.

### Live Demo
View the [official demo of WASM in action](https://silvia-odwyer.github.io/photon/demo.html).

### Photon In Action

![Imgur](https://i.imgur.com/PShSZ6k.png)

### Get Started
#### Getting Started Guide
Check out Photon's [getting started guide, complete with tutorials, installation instructions, and more](https://silvia-odwyer.github.io/photon/guide)

### 📚 Documentation
View the [official documentation](https://docs.rs/photon-rs/).

### Functions
96 customisable functions are available, for varying image effects.

Functions include:
- **Image correction**: Hue rotation, sharpening, brightness adjustment, adjusting saturation, lightening/darkening all within various colour spaces.
- **Resizing**: Resize images both natively and on the web.
- **Convolutions**: Sobel filters, blurs, Laplace effects, edge detection, etc.,
- **Channel manipulation**: Increasing/decreasing RGB channel values, swapping channels, removing channels, etc.
- **Monochrome effects**: Monochrome tints, greyscaling of various forms, thresholding, sepia, averaging RGB values
- **Colour manipulation**: Work with the image in various colour spaces such as HSL, LCh, and sRGB, and adjust the colours accordingly.
- **Filters**: Over 30 pre-set filters available, incorporating various effects and transformations.
- **Watermarking**: Watermark images in multiple formats.
- **Blending**: Blend images together using 10 different techniques, change image backgrounds.

## Get Started Natively

### Install
Add the following line to the dependencies section of your Rust project's Cargo.toml:

###### Cargo.toml
```toml
[dependencies]
photon-rs = "0.2.0"
```

#### Using Photon Natively
The following code opens an image from the filesystem, applies an effect, and outputs it as a PNG.

Here is a code sample to get you started:

```rust
extern crate photon_rs;
use photon_rs::native::{open_image, save_image};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("test_image.PNG")?;

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    // Write file to filesystem.
    save_image(img, "raw_image.JPG")?;

    Ok(())
}
```

##### See More Examples
[For more examples, check out the guide on how to get started with Photon natively.](https://silvia-odwyer.github.io/photon/guide/using-photon-natively/)

## Get Started With WebAssembly
### Using a Bundler?
#### Installing Photon
If you're using Webpack or a bundler to build your project, install Photon via npm:

```bash
npm install @silvia-odwyer/photon
```

You can run Photon directly in any web browser that supports WebAssembly, which includes Chrome, Firefox, Safari, and Edge.

##### Get Started with Photon on The Web
To get started, [check out the guide](https://silvia-odwyer.github.io/photon/guide/using-photon-web/).

#### Using NodeJS?
If you're intending to use Photon with NodeJS, you can install the NodeJS version of the library:

```bash
npm install @silvia-odwyer/photon-node
```

<!-- ##### NodeJS Tutorial
See the [NodeJS tutorial, which shows how to use Photon with NodeJS](). -->

## Modules
Photon contains a series of modules, which include:

- `effects`: Various image effects, including adding offsets, thresholding, duotoning, solarization, etc.,
- `channels`: Functions related to increasing/decreasing the red, green, and blue channels of the image data.
- `filters`: Preset filters, which alter the rgb channels of the image. Contains over 20.
- `conv`: Laplace, Sobel, emboss; image proc functions which require image convolution.
-  `noise`: Noise generation of varying tints and hues.
- `multiple`: A module for dealing with multiple images, such as watermarking images, etc.,
- `correction`: Hue rotation, adjusting saturation, lightening/darkening: all techniques available in multiple colour spaces, which lead to varying effects.

### Quick Start Example
Clone this repo:
```sh
git clone https://github.com/silvia-odwyer/photon
```

Run the binary, which will perform an image processing function on an image:
```sh
cd crate
cargo run --release
```

Compare the original image with the outputted image, and you'll see the desired effect has been applied.

## Got Questions?
If you have further questions about this library, you can ask them on [Gitter](https://gitter.im/photonlibrary/community) or [Spectrum](https://spectrum.chat/photonlibrary), and I'll get back to you!

If there are any issues involving running/using the library, make sure to open an issue, it would be greatly appreciated,
and will help improve the library.

- [Spectrum](https://spectrum.chat/photonlibrary)
- [Gitter](https://gitter.im/photonlibrary/community)

## Additional Notes
Functions have been designed with flexibility in mind, so that full customization of effects and filters can be utilised; for every function, hundreds of differing image effects/tints/hues can be created, just by changing parameters slightly, so with every function comes the ability to fully experiment.

For developers who would like to work with high-level constructs can do so, such as applying effects to imagery (eg: Laplace or Sobel)
or filters; this library provides a complete suite of functions to do so, as well as in-built filters and presets.

`photon` can be thought of as a high-level wrapper to the Rust `image` crate, but conversely also includes functions which provide low-level access to pixel and channel manipulation, perfect for developers who wish to work with this data directly.

## Contributing

Photon is always ready for new filters and functions, so if you'd like to contribute, we're always ready to accept new Pull Requests or investigate new issues.

## To Do
- Selective colorization
- Fade
- Pixelisation
- Blend images using browser-specific functions for WASM version of library.
- Vintage images with light leaks, grains, etc.,
- Normalisation
- Gamma correction
- Duotone filtering
- Tests in a headless web browser for WebAssembly version of library

## Contributors

* **Silvia O'Dwyer** - [@silvia-odwyer](https://github.com/silvia-odwyer)
* **Ivan Zvonimir Horvat** - [@Horki](https://github.com/Horki)
* **Future You(?)** - (See Contributing above ;)

## License
This project is licensed under the Apache 2.0 License - see the [LICENSE.md](LICENSE.md) file for details.
