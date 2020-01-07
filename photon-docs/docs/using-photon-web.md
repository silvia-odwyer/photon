# Using Photon on the Web

## Installing Photon
### Prerequisites
Install via npm:

```bash
npm install @silvia-odwyer/photon
```

## Importing 
```javascript
import("@silvia-odwyer/photon").then(photon => {
    // Module has now been imported. 
    // All image processing logic w/ Photon goes here.
    // See sample code below.
}
```

### Usage

Working with Photon involves the use of the HTML5 Canvas element, so you'll need to create 
a canvas and draw your desired image onto the canvas. 

You can then convert this canvas to a PhotonImage, and apply effects to it. 

See the sample code below.

### Sample Code

    #!javascript
    function filterImage() {
        // Create a canvas and get a 2D context from the canvas
        var canvas = document.getElementById("canvas");
        var ctx = canvas.getContext("2d");
        
        // Draw the image element onto the canvas
        ctx.drawImage(newimg, 0, 0);
        
        // Convert the ImageData found in the canvas to a PhotonImage (so that it can communicate with the core Rust library)
        let image = photon.open_image(canvas, ctx);

        // Filter the image, the PhotonImage's raw pixels are modified
        photon.filter(image, "radio");
        
        // Place the modified image back on the canvas
        photon.putImageData(canvas, ctx, rust_image);
    }

### Apply Multiple Effects 
To apply multiple effects to a single image, you can pass the same image into consecutive calls. 

For example, to add a "radio" filter effect, then convert to greyscale, and finally increment the Red channel:

    #!javascript
    function filterImage() {
        // Create a canvas and get a 2D context from the canvas
        var canvas = document.getElementById("canvas");
        var ctx = canvas.getContext("2d");
        
        // Draw the image element onto the canvas
        ctx.drawImage(newimg, 0, 0);
        
        // Convert the ImageData found in the canvas to a PhotonImage (so that it can communicate with the core Rust library)
        let image = photon.open_image(canvas, ctx);

        // Filter the image, the PhotonImage's raw pixels are modified
        photon.filter(image, "radio");

        photon.grayscale(image);

        photon.alter_red_channel(image, 20);
        
        // Place the pixels back on the canvas
        photon.putImageData(canvas, ctx, rust_image);
    }