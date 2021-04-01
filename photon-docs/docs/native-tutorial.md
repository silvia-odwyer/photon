# Native Tutorial

In this tutorial, we're going to write a program that resizes an image and applies a filter to it.
You'll get a feel for how to use Photon, and will be able to build upon this to use Photon in your own projects.

## Getting Started
Ensuring you have Rust installed, create a new Rust project:

```bash
cargo new photon-demo --bin
cd photon-demo
```

Once you've moved into the new directory, take a look at the source files generated.

## Add Photon as A Dependency

Add Photon as a dependency to your project:

    #!toml hl_lines="8"
    [dependencies]
    photon-rs="0.2.0"

Your Cargo.toml should look like this:

##### Cargo.toml
    #!toml
    [package]
    name = "photon-demo"
    version = "0.1.0"
    authors = ["your_name <your_email>"]
    edition = "2018"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    photon-rs="0.2.0"


## Writing The Program
Next up, open your `bin.rs` file. You'll find a sample function in there, remove that since we won't be using it.

#### Open An Image

To open an image:

##### bin.rs
    #!rust hl_lines="5"
    extern crate photon_rs;
    use photon_rs::native::{open_image};

    fn main() {
        let mut img = open_image("image.jpg");
    }

#### Apply a Filter Effect

To apply a filter effect to the opened image, we need to pass in our image and a filter name.

    #!rust
    photon_rs::filters::filter(&mut img, "twenties");

Notice that we're passing a mutable reference to the image. This allows the function to modify the image, rather than return a new image.
There are a variety of filter effects we can pass. Once you get the program compiled, try passing in "radio" instead of the filter above.
For a full list, see the documentation.

#### Save To The Filesystem
Then, to write the image to the filesystem:

    #!rust
    save_image(img, "new_image.jpg");

Notice here we're saving it as a JPG image, but we could also save it as a PNG or a different output format, by including a different file extension.

#### Get An Image
Next up, you'll need an image to work with. You can use an image from your own collection, or try out the images available at [Unsplash](https://unsplash.com/),
which are also available in the Public Domain.

Name it `image.jpg`, and save it in the same directory as your rust project.

### Final Program
The final code looks like this:

##### bin.rs
    #!rust
    extern crate photon_rs;
    use photon_rs::{filters};
    use photon_rs::native::{open_image, save_image};

    fn main() {
        // Open the image (a PhotonImage is returned)
        let mut img = open_image("image.jpg");

        // Apply a filter to the pixels
        filters::filter(&mut img, "twenties");

        // Write the new image to the filesystem.
        save_image(img, "new_image.jpg");

    }

##### Run The Code
To run the program in release mode, run:

```bash
cargo run --release
```

!!! warning
    Make sure you run in **release** mode for optimum performance, by adding the --release flag to your command.
    Otherwise, performance will be greatly affected.

#### Bonus: Add Timing
If you'd like to find out how long it takes to process your image, you can add some code to capture this.

Add the `time` dependency to your Cargo.toml:

###### Cargo.toml
```
[dependencies]
time="0.2.1"
```

Then in your code:

    #!rust hl_lines="10"
    extern crate photon_rs;
    use photon_rs::native::{open_image, save_image};
    use time::{PreciseTime};

    fn main() {
        // Open the image (a PhotonImage is returned)
        let mut img = open_image("image.jpg");

        // Start time
        let start = PreciseTime::now();

        // Process image
        photon_rs::channels::alter_channel(&mut img, 1, -20);
        save_image(img, "raw_image.png");

        // Output time taken.
        let end = PreciseTime::now();
        println!("Took {} seconds to process image.", start.to(end));
    }

### Want More Examples?

To view more examples for native-use, check out the [`/examples`](https://github.com/silvia-odwyer/photon/tree/master/crate/examples) folder in Photon's repository.
You'll find full instructions on how to run these in the README.

### Working with the Web
If you'd like to get started with Photon for the web, see the [accompanying web tutorial](web-tutorial.md).
