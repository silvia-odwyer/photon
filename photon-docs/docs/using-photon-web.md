# Using Photon on the Web

## Installing Photon
### Prerequisites
Ensure you have NodeJS installed on your machine. 

### Sample Code

    #!javascript
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