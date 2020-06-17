# Getting Started

## Installing Photon
### Photon for the Web
Photon is available as an npm module, which calls the underlying WebAssembly code. 

Ensuring you have Node.JS installed on your machine, install [Photon's npm module](https://www.npmjs.com/package/@silvia-odwyer/photon):

```bash
npm install @silvia-odwyer/photon
```

You can use Photon as you would a standard JavaScript library, since it’s available as an npm module, meaning you can use Photon for the browser, in Electron apps, with Vue, React, other JS frameworks, etc., 

### Photon with NodeJS
If you'd like to install Photon for use with NodeJS, install the [relevant npm module](https://www.npmjs.com/package/@silvia-odwyer/photon-node):

```bash
npm install @silvia-odwyer/photon-node 
```

### Using Photon On the Web
Check out our accompanying article on [how to use Photon on the Web.](using-photon-web.md)

### Photon Natively
If you’d like to use Photon with a Rust project, you can install the `photon-rs` [Cargo crate](https://crates.io/crates/photon-rs) by adding the following line to your Cargo.toml dependencies:

#### Cargo.toml
```
[dependencies]
photon_rs = "0.2.0"
```

### Using Photon Natively 
Once you've added Photon as a dependency, you can [start using it, see this article for more](using-photon-natively.md)