#### photon + webpack demo
Here you'll find instructions for running Photon with a webpack build pipeline. 

For a React demo, see the `react_app_demo` dir in the root of this repo.

To get started:

Clone this repo:
```sh
git clone https://github.com/silvia-odwyer/photon
```

Navigate to the webpack_demo dir:
```sh
cd webpack_demo
```

Install the dependencies:
```sh
npm install
```
<!-- 
To compile the lib's Rust code to wasm, run:
```sh
npm run build
``` -->

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
