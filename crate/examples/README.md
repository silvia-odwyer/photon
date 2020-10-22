## Examples
Examples for running Photon natively.

Note: If you're here to see WebAssembly in action, this is the wrong path, turn back.
To see WebAssembly in action, clone this repo and run `npm start` after `cd`ing into photon.
More details can be found in the main repo README.

### Running These Examples
Clone this repo:
```sh
git clone https://github.com/silvia-odwyer/photon
```

Several examples are included, for this demo, we'll run the Rust file called `example.rs`:

```sh
cargo run --example example --release
```

Make sure the `--release` flag is added.

You'll find the outputted images in `example_output`.

To change the images being inputted, add your image to `input_images` and change the filename in `example.rs`.

#### Add Text Example
To run the example which adds text to an image:

```sh
cargo run --example add_text
```
