# Using Photon Natively

## Prerequisites
Ensure you have added Photon as a dependency to your Cargo.toml project. 

## Open an Image 
To open an image:

##### bin.rs
    #!rust hl_lines="4"
    extern crate photon;
    use photon::native::{open_image};
    fn main() {
        let mut img = open_image("image.jpg");
    }

## Process The Image
To apply a filter effect to the opened image, we need to pass in our image and a filter name. 

    #!rust
    photon::filters::filter(&mut img, "twenties");

Notice that we're passing a mutable reference to the image. This allows the function to modify the image, rather than return a new image.
There are a variety of filter effects we can pass. Once you get the program compiled, try passing in "radio" instead of the filter above.
For a full list, see the documentation. 

## Write to the Filesystem
Then, to write the image to the filesystem:

    #!rust
    save_image(img, "new_image.jpg");

Notice here we're saving it as a JPG image, but we could also save it as a PNG or a different output format, by including a different file extension.

## Sample Program
This program increases the red channel of the image, and then applies a sepia effect to the image.

    #!rust hl_lines="11"
    extern crate photon;
    use photon::{filters, channels};
    use photon::native::{open_image, save_image};

    fn main() {
        // Open the image (a PhotonImage is returned)
        let mut img = open_image("image.jpg");

        // Apply a filter to the pixels
        channels::alter_red_channel(&mut img, 10);
        filters::filter(&mut img, "sepia");
        save_image(img, "raw_image.png");    
    }

