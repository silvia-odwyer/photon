use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData, HtmlCanvasElement, HtmlImageElement};
use wasm_bindgen::Clamped;
use web_sys::console;
use image::{DynamicImage, GenericImageView, GenericImage};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Describes the image's height, width, and contains the image's raw pixels.
/// For use when communicating between JS and Rust, and also natively. 
#[wasm_bindgen]
#[derive(Debug)]
pub struct PhotonImage {
    raw_pixels: Vec<u8>,
    width: u32, 
    height: u32,
}

#[wasm_bindgen]
impl PhotonImage {
    pub fn new(&mut self, img_data: ImageData, width: u32, height: u32) -> PhotonImage {
        let raw_pixels = to_raw_pixels(img_data);
        let new_vec = Vec::new();
        return PhotonImage {raw_pixels: new_vec, width: width, height: height}
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

// Called by the JS entry point to ensure that everything is working as expected
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("No Window found, should have a Window");
    let document = window.document().expect("No Document found, should have a Document");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("You're successfully running WASM!"));

    let body = document.body().expect("ERR: No body found, should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;
    Ok(())
}

// #[wasm_bindgen]
// pub fn editContext(ctx: CanvasRenderingContext2d, img: HtmlImageElement, mode: &str) -> Result<(), JsValue> {
//     set_panic_hook();
//     let width = 3104;
//     let height = 4656;

//     // draw image to canvas
//     //ctx.draw_image_with_html_image_element(&img, 0.0, 0.0);

//     // let data: ImageData = ctx.get_image_data(0.0, 0.0, 100.0, 100.0).unwrap();
//     let mut data = ctx.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap().data();
//     let vec_data = data.to_vec();

//     // // Convert ImageData to the Rust type, ie: DynamicImage
//     // let dyn_image = helpers::dyn_image_from_raw(data, width, height);

//     // Call a Photon function to manipulate the image data/apply the effect
//     let mut raw_pixels = match mode {
//         "hue_rotate" => inc_channel_raw(vec_data, 1, 20, width, height),
//         "saturate" => inc_channel_raw(vec_data, 2, 40, width, height),
//         _ => inc_channel_raw(vec_data, 0, 45, width, height),
//     };


//     // Convert the raw pixels back to an ImageData object.
//     let newData = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height)?;

//     // Place the new imagedata onto the canvas
//     ctx.put_image_data(&newData, 0.0, 0.0);
//     Ok(())
// }

#[wasm_bindgen]
#[no_mangle]
pub fn open_image(imgdata: ImageData, width: u32, height: u32) -> PhotonImage {
    let raw_pixels = to_raw_pixels(imgdata);
    return PhotonImage {raw_pixels: raw_pixels, width: width, height: height }
}

// #[wasm_bindgen]
// pub fn inc_channel_raw(imgData: ImageData, channel: usize, offset: u32, width: u32, height: u32) -> Result<(ImageData), JsValue> {
    
//     let img_vec = to_raw_pixels(imgData);
//     let mut img = helpers::dyn_image_from_raw(img_vec, width, height);
//     let (width, height) = img.dimensions();

//     for x in 0..width {
//         for y in 0..height {
//             let mut px = img.get_pixel(x, y);
            
//             if px.data[channel] <= 255 - offset as u8 {
//                 let px_data = px.data[channel];
//                 let final_px_data = px_data + offset as u8;
//                 px.data[channel] = final_px_data as u8;
//             }
//             else {
//                 px.data[channel] = 255;
//             }
//             img.put_pixel(x, y, px);
//         }
//     }
//     let mut raw_pixels = img.raw_pixels();
//     let newData = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height)?;
//     return Ok(newData);
// }

#[wasm_bindgen]
pub fn to_raw_pixels(imgdata: ImageData) -> Vec<u8> {
    let mut img_vec = imgdata.data().to_vec();
    return img_vec;
}

#[wasm_bindgen]
pub fn to_image_data(photon_image: PhotonImage) -> ImageData {
    let mut raw_pixels = photon_image.raw_pixels;
    let width = photon_image.width;
    let height = photon_image.height;
    let newData = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height).unwrap();

    return newData;
}


// #[wasm_bindgen]
// pub fn apply_filter(ctx: CanvasRenderingContext2d, img: HtmlImageElement, filter_name: &str) -> Result<(), JsValue> {
//     set_panic_hook();
//     let width = 3104;
//     let height = 4656;

//     // draw image to canvas
//     ctx.draw_image_with_html_image_element(&img, 0.0, 0.0);

//     // let data: ImageData = ctx.get_image_data(0.0, 0.0, 100.0, 100.0).unwrap();
//     let mut data = ctx.get_image_data(0.0, 0.0, 3104.0, 4656.0).unwrap().data().to_vec();

//     // Convert ImageData to the Rust type, ie: DynamicImage
//     let dyn_image = photon::helpers::dyn_image_from_raw(data, width, height);

//     // Call a Photon function to manipulate the image data/apply the effect
//     let new_dyn_image = match filter_name {
//         "vintage" => photon::filters::vintage(dyn_image),
//         "twenties" => photon::filters::twenties(dyn_image),
//         "perfume" => photon::filters::perfume(dyn_image),
//         "oceanic" => photon::filters::oceanic(dyn_image),
//         "marine" => photon::filters::marine(dyn_image),
//         "islands" => photon::filters::islands(dyn_image),
//         _ => photon::filters::perfume(dyn_image),
//     };

//     // Convert the DynamicImage back to a vec 
//     let mut raw_pixels = new_dyn_image.raw_pixels();

//     // Convert the raw pixels back to an ImageData object.
//     let newData = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height)?;

//     // Place the new imagedata onto the canvas
//     ctx.put_image_data(&newData, 0.0, 0.0);
//     Ok(())
// }


fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub mod channels;
pub mod effects;
pub mod conv;
pub mod filters;
pub mod monochrome;
pub mod noise;
pub mod helpers;