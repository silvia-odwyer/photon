# Using Photon Natively

## Prerequisites
Add Photon as a dependency to your project's Cargo.toml:

##### Cargo.toml
```
photon-rs = "0.2.0"
```

## Open an Image 
To open an image:

##### bin.rs
    #!rust hl_lines="4"
    extern crate photon_rs;
    use photon_rs::native::{open_image};
    fn main() {
        let mut img = open_image("image.jpg");
    }

## Process The Image
To apply a filter effect to the opened image, we need to pass in our image and a filter name. 

    #!rust
    photon_rs::filters::filter(&mut img, "twenties");

Notice that we're passing a mutable reference to the image. This allows the function to modify the image, rather than return a new image.
There are a variety of filter effects we can pass. Once you get the program compiled, try passing in "radio" instead of the filter above.
For a full list, see the [documentation](https://docs.rs/photon-rs). 

## Write to the Filesystem
Then, to write the image to the filesystem:

    #!rust
    save_image(img, "new_image.jpg");

Notice here we're saving it as a JPG image, but we could also save it as a PNG or a different output format, by including a different file extension.

## Sample Program
This program adds a sepia effect to an image:

    #!rust hl_lines="10"
    extern crate photon_rs;
    use photon_rs::{monochrome};
    use photon_rs::native::{open_image, save_image};

    fn main() {
        // Open the image (a PhotonImage is returned)
        let mut img = open_image("image.jpg");

        // Apply a sepia effect to the image.
        monochrome::sepia(&mut img);

        save_image(img, "raw_image.png");    
    }