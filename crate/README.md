<p align="center">
  <a href="" rel="noopener">
 <img src="https://i.imgur.com/GxrKNOb.png" alt="Photon banner, showing the Photon logo on a dark background"></a>
</p>

<div align="center">

  [![Status](https://img.shields.io/badge/status-active-success.svg)]() 
  [![GitHub Issues](https://img.shields.io/github/issues/silvia-odwyer/photon.svg)](https://github.com/silvia-odwyer/photon/issues)
  [![GitHub Pull Requests](https://img.shields.io/github/issues-pr/silvia-odwyer/photon.svg)](https://github.com/silvia-odwyer/p/pulls)
  [![Gitter chat](https://badges.gitter.im/silvia-odwyer/photon.png)](https://gitter.im/photonlibrary/community "Gitter chat")
  [![Crates.io](https://img.shields.io/crates/v/photon-rs)](https://crates.io/crates/photon-rs)

</div>

---

<p align="center"> High-performance, cross-platform Rust/WebAssembly image processing library
    <br> 
</p>

## 📝 Table of Contents
- [Get Started with WebAssembly](https://github.com/silvia-odwyer/photon#-get-started-with-webassembly)
- [Get Started Natively](https://github.com/silvia-odwyer/photon#getting-started)
- [Documentation](https://docs.rs/photon-rs/0.1.0/)
- [Official Website.](https://silvia-odwyer.github.io/photon/)
- [All Available Functions.](https://silvia-odwyer.github.io/photon/docs/photon/all.html)
- [Got Questions? Ask Here!](https://github.com/silvia-odwyer/photon#got-questions)

Photon is a high-performance Rust image processing library, which compiles to WebAssembly, allowing for 
safe, blazing-fast image processing both natively and on the web. 

### Features 
- **Fast:** Photon outperforms even the fastest of libraries, including ImageMagick. On the web, its high-performance allows for near-native-speed image processing on the web. Benchmarks coming soon.
- **Call with JS:** Want to use Photon on the web or with Node? Using a simple npm package, you're good to go. Get all the benefits of WebAssembly
with zero-cost abstraction. 
- **Use Natively:** For command-line apps, native photo editing apps, and so forth, Photon's core codebase is in Rust, allowing for cross-platform
development.

### Live Demo
View the [official demo of WASM in action](https://silvia-odwyer.github.io/photon/demo.html).

### Get Started
#### Getting Started Guide
Check out Photon's [getting started guide, complete with tutorials, installation instructions, and more](https://silvia-odwyer.github.io/photon/guide)

#### Documentation
### 📚 Documentation
View the [official documentation](https://docs.rs/photon-rs/).

### Photon In Action

![Imgur](https://i.imgur.com/PShSZ6k.png)

### Functionality
96+ customisable functions are available. 

Resize, transform, correct, and filter images: apply as many effects as desired. 

Functions include:
- **Image correction**: Hue rotation, sharpening, brightness adjustment, adjusting saturation, lightening/darkening all within various colour spaces. 
- **Convolutions**: Sobel filters, blurs, Laplace effects, edge detection, etc., 
- **Channel manipulation**: Increasing/decreasing RGB channel values, swapping channels, removing channels, etc.
- **Transform**: Resize, crop, rotate and flip images.
- **Monochrome effects**: Duotoning, greyscaling of various forms, thresholding, sepia, averaging RGB values
- **Colour manipulation**: Work with the image in various colour spaces such as HSL, LCh, and sRGB, and adjust the colours accordingly. 
- **Filters**: Over 30 pre-set filters available, incorporating various effects and transformations.
- **Text**: Apply text to imagery in artistic ways, or to watermark, etc.,
- **Watermarking**: Watermark images in multiple formats. 
- **Blending**: Blend images together using 10 different techniques, change image backgrounds. 

## Install
### Native
Add the following line to the dependencies section of your Rust project's Cargo.toml:

###### Cargo.toml
```toml
[dependencies]
photon-rs = "0.3.2"
``` 

### Web
Install Photon as an npm module:

```bash
npm install --save @silvia-odwyer/photon
```

### Node.JS
To install Photon as an npm module for Node.JS use:

```bash
npm install --save @silvia-odwyer/photon-node
```

#### Using Photon Natively 
The following code opens an image from the filesystem, applies an effect, and saves it.

Here is a code sample to get you started:

```rs
extern crate photon_rs;
use photon_rs::native::{open_image, save_image};

fn main() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("test_image.png");

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    // Write file to filesystem.
    save_image(img, "raw_image.jpg");    

}
```

[For more examples, check out the guide on how to get started with Photon natively.](https://silvia-odwyer.github.io/photon/guide/using-photon-natively/)

#### Using Photon On The Web 

## Modules 
Photon contains a series of modules, which include:

- `effects`: Various image effects, including adding offsets, thresholding, duotoning, solarization, etc.,
- `channels`: Functions related to increasing/decreasing the red, green, and blue channels of the image data.
- `transform`: Resize, crop, flip, and rotate images.
- `filters`: Preset filters, which alter the RGB channels of the image. Contains over 20. 
- `conv`: Laplace, Sobel, emboss; image proc functions which require image convolution. 
-  `noise`: Noise generation of varying tints and hues. 
- `multiple`: A module for dealing with multiple images, such as watermarking images, etc.,
- `correction`: Hue rotation, adjusting saturation, lightening/darkening: all techniques available in multiple colour spaces, which lead to varying effects.

All effects and filters can be viewed below and on the official website.

### Run Examples
Clone this crate's official GitHub repo:
```sh
git clone https://github.com/silvia-odwyer/photon
```

Run the binary, which will perform an image processing function on an image:
```sh
cd crate
cargo run --release 
```

Compare the original image with the outputted image, and you'll see the desired effect has been applied.

### Got Questions? 
If you'd like to chat to the developer about your potential use case, or have further questions about this library, 
just submit them here, and I'll get back to you!

- [Spectrum Chat](https://spectrum.chat/photonlibrary)
- [Gitter](https://gitter.im/photonlibrary/community)

## License
This project is licensed under the Apache 2.0 License - see the [LICENSE.md](LICENSE.md) file for details.