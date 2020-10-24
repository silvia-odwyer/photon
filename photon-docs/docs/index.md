# Photon 

Photon is a high-performance image processing library, written in Rust and compilable to WebAssembly, which can be used both natively and on the web.

It allows developers to apply filters, effects and manipulate channels of their images, as well as apply standard image processing operations such as blurring, cropping, resizing, etc.,

## Features <small></small>
- **High-performance**: Photon outperforms even the fastest of libraries, including ImageMagick and the Python Imaging Library.
- **Safety and Security**: Written in Rust, this library guarantees memory safety. 
- Over **80 effects**: Boasting over 80 effects.
- Use on the **web** or **natively**.
- **Cross-platform**

This guide will help you install Photon and start using it either natively or on the web.

## Image Effects Available 
- **Transformations** - Crop, resize, flip, and rotate images. 
- **Filters** - Apply image filters. 
- **Channel Manipulation** - Alter channels to create new tinted images, channel-related effects.
- **Special Effects** - From solarization to offset fx to blending, over 20 special fx available. 
- **Correction** - Sharpen, brighten, darken, saturate, color correction

## Web Demo
To view a demo of Photon in action, [click here](https://silvia-odwyer.github.io/photon/demo.html).

### Supported Image Formats
The following image formats are supported:

- PNG
- JPEG
- BMP
- ICO 
- TIFF

## Documentation
[See the documentation here.](https://docs.rs/photon-rs/0.1.0/)

<!-- ## Tutorials
To create your own applications using Photon, check out our two tutorials, one which is for the web, 
and the other which runs Rust natively. -->

<!-- 
## Browse Effects
| Original             |  Retro | Twenties | Sharpen | Hi|
:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:|:-------------------------:
![](https://i.imgur.com/7J5fkBq.png)  |  ![](https://i.imgur.com/7J5fkBq.png) Saturate HSL| ![](https://i.imgur.com/qdQRHla.png) Box Blur| ![](https://i.imgur.com/orhUDz0.png) Swap GB channels | ![](https://i.imgur.com/YU8rruS.png) Sharpen | ![](https://i.imgur.com/c3MsXWI.png) Saturate LCh | ![](https://i.imgur.com/Uw3DcVe.png) Saturate HSL| ![]()
![Imgur](https://i.imgur.com/g3Vfs0f.jpg) | ![](https://i.imgur.com/VBdyTmE.png) Saturate HSV |  ![](https://i.imgur.com/ZeJfSV8.png) Sharpen | ![](https://i.imgur.com/Uw3DcVe.png) Saturate HSL
![Original](https://i.imgur.com/9JLtvUC.jpg) | ![](https://i.imgur.com/ZLOyQ0y.png) Perfume |  ![](https://i.imgur.com/Zc0Vgr3.png) Serenity | ![](https://i.imgur.com/Z3eVyP6.png) Gradient Overlay

## Photon vs Other Libraries

These benchmarks were carried out on an 8GB RAM w/ i5 Processor laptop.

| Operation | Photon | Python Imaging Library | ImageMagick    | arcu | sed |
| ------------------------ | ----------- | ---------- | ------- | ---- | --- |
| Flip horizontally      | 0.2s         | 34.4s        | 2.8s     | yes  | yes |
| Ornare viverra ex        | yes         | yes        | yes     | yes  | yes |
| Mauris a ullamcorper     | yes         | yes        | partial | yes  | yes |
| Nullam urna elit         | yes         | yes        | yes     | yes  | yes |
| Malesuada eget finibus   | yes         | yes        | yes     | yes  | yes |
| Ullamcorper              | yes         | yes        | yes     | yes  | yes |
| Vestibulum sodales       | yes         | -          | yes     | -    | yes |
| Pulvinar nisl            | yes         | yes        | yes     | -    | -   |
| Pharetra aliquet est     | yes         | yes        | yes     | yes  | yes |
| Sed suscipit             | yes         | yes        | yes     | yes  | yes |
| Orci non pretium         | yes         | partial    | -       | -    | -   |

##### Apply Effect, then Save as PNG

| Operation | Photon | Python Imaging Library | ImageMagick    | arcu | sed |
| ------------------------ | ----------- | ---------- | ------- | ---- | --- |
| Flip horizontally      | 1.2s         | 34.4s        | 42.8s     | yes  | yes |
| Ornare viverra ex        | yes         | yes        | yes     | yes  | yes |
| Mauris a ullamcorper     | yes         | yes        | partial | yes  | yes |
| Nullam urna elit         | yes         | yes        | yes     | yes  | yes |
| Malesuada eget finibus   | yes         | yes        | yes     | yes  | yes |
| Ullamcorper              | yes         | yes        | yes     | yes  | yes |
| Vestibulum sodales       | yes         | -          | yes     | -    | yes |
| Pulvinar nisl            | yes         | yes        | yes     | -    | -   |
| Pharetra aliquet est     | yes         | yes        | yes     | yes  | yes |
| Sed suscipit             | yes         | yes        | yes     | yes  | yes |
| Orci non pretium         | yes         | partial    | -       | -    | -   | -->


## Featured In 
See what others have to say about Photon:

- [Stephen Downes' Note on Photon](https://www.downes.ca/cgi-bin/page.cgi?post=70217)

## Ready To Get Started?

- [Get started with WebAssembly.](https://silvia-odwyer.github.io/photon/guide/using-photon-web/)
- [Get started with Rust for native use.](https://silvia-odwyer.github.io/photon/guide/using-photon-natively/)