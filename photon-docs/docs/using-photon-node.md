# Using Photon with Node.js

Photon also can be executed on Node.js via WebAssembly. In this guide, we're going to 
take a look at how to read, filter and save images on Node.js using Photon and the `fs`
module.

## Installing Photon
### Prerequisites
Install via npm:

```bash
npm install @silvia-odwyer/photon-node
```

## Importing 
```javascript
var photon = require("@silvia-odwyer/photon-node");
```

### Usage

For a quick overview, see the sample code snippet below.

#### Creating A PhotonImage
To convert your image to a Photon-compatible image, you'll need to encode your image as base64 
and then create a PhotonImage from the encoded base64:

    #!javascript
    // read file, then convert to base64
    var base64 = fs.readFileSync(`input.png`, { encoding: 'base64' });
    let data = base64.replace(/^data:image\/(png|jpg);base64,/, "");
        
    // convert base64 to PhotonImage
    var phtn_img = photon.PhotonImage.new_from_base64(data);

#### Applying Effects and Filters

Now that you have your PhotonImage, you can apply effects and filters to this image:

    #!javascript
    photon.grayscale(phtn_img);


#### Save Image

To save and write your image, you'll need to convert the PhotonImage back to base64, 
this base64 can then be saved as an image.

    #!javascript
    // get base64 from filtered image, and write 
    let output_base64 = phtn_img.get_base64();
    let output_image_name = "output.png";
    var output_data = output_base64.replace(/^data:image\/\w+;base64,/, '');

    fs.writeFile(output_image_name, output_data, {encoding: 'base64'}, function(err) {
    });

### Sample Code

The following code reads an image called `input.png`, converts it to grayscale, 
and then saves it as `output.png`. All imports required are also included.

    #!javascript
    var fs = require('fs');

    var photon = require("@silvia-odwyer/photon-node");

    const fetch = require('node-fetch');
    global.fetch = fetch;

    function grayscaleImage() {
        // read file, then convert to base64
        var base64 = fs.readFileSync(`input.png`, { encoding: 'base64' });
        let data = base64.replace(/^data:image\/(png|jpg);base64,/, "");
        
        // convert base64 to PhotonImage
        var phtn_img = photon.PhotonImage.new_from_base64(data);
            
        photon.grayscale(phtn_img);
        
        // get base64 from filtered image, and write 
        let output_base64 = phtn_img.get_base64();
        let output_image_name = "output.png";
        var output_data = output_base64.replace(/^data:image\/\w+;base64,/, '');

        fs.writeFile(output_image_name, output_data, {encoding: 'base64'}, function(err) {
        });
        console.log(`Saved ${output_image_name}`);
        
    }

    grayscaleImage();