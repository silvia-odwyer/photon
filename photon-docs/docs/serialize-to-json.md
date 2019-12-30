# Serialize/Deserialize to JSON
You can serialize and deserialize to and from JSON, if required. 

Here is a minimum example:

#### Cargo.toml
```
[package]
authors = ["Name <email>"]
name = "serialize-example"
edition = "2018"

[dependencies]
photon="0.0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### bin.rs
    #!rust 
    extern crate image;
    extern crate photon;
    use photon::native::{open_image, save_image};
    use photon::{Rgb, PhotonImage};
    use serde::{Serialize, Deserialize};

    fn main() {
        let photon_image = PhotonImage::new(134, 145);

        let serialized = serde_json::to_string(&photon_image).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: PhotonImage = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
    }