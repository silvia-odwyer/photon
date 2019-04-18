//! An image processing crate that provides a set of functions for image filtering, convolution, colour manipulation, and more.
//! ## Example 
//! ```rust
//! extern crate photon;

//! fn main() {
//!     let img = photon::helpers::open_image("valley.PNG");
//!     let filtered_img = photon::effects::solarize(img);
//!     // Write the contents of this image in PNG format.
//!     photon::helpers::save_image(filtered_img, "new_image.PNG");
//! }
//! ```
//! 
//! This crate contains built-in preset functions, which provide default image processing functionality, as well as functions
//! that allow for direct, low-level access to channel manipulation.

use image::{GenericImage, DynamicImage, GenericImageView};

pub mod filters;
pub mod channels;
pub mod noise;
pub mod effects;
pub mod conv;
pub mod monochrome;
pub mod helpers;
