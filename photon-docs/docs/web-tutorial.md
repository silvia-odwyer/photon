# Web Tutorial

In this tutorial, we're going to create an image processing application for the web, 
which will allow users to upload images and perform various image processing operations on them
using Photon.

Since this app will use Photon, it'll take advantage of Web Assembly's near-native performance, 
leading to a high-performance application. 

Here's a GIF of what we'll be making:

## Setting Up

Photon is available as an npm module, which calls the underlying WebAssembly code. 

Ensuring you have Node.JS installed on your machine, run:

```bash
npm install photon-wasm
```

You can use Photon as you would a standard JavaScript library, since it’s available as an npm module, meaning you can use Photon for the browser, in electron apps, with Vue, React, other JS frameworks, etc., 

### Photon Natively
If you’d like to use Photon with a Rust project, you can install the `photon` cargo package by adding the following line to your Cargo.toml dependencies:

#### Cargo.toml
```
[dependencies]
photon 1.2
```