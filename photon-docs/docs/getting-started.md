# Getting Started

## Installing Photon
### Photon for the Web
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