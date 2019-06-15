use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData, HtmlCanvasElement, HtmlImageElement};
use wasm_bindgen::Clamped;
use web_sys::console;
use image::{DynamicImage, GenericImageView, GenericImage, ImageBuffer};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Provides the image's height, width, and contains the image's raw pixels.
/// For use when communicating between JS and WASM, and also natively. 
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

/// RGB struct, containing values for Red, Green, and Blue channels.
#[wasm_bindgen]
#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[wasm_bindgen]
impl Rgb {
    pub fn new(&mut self, r: u8, g: u8, b: u8) -> Rgb {
        return Rgb {r: r, g: g, b: b}
    }
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

/// Get the ImageData from a 2D canvas context
#[wasm_bindgen]
pub fn getImageData(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) -> ImageData {
    set_panic_hook();
    let width = canvas.width();
    let height = canvas.height();

    // let data: ImageData = ctx.get_image_data(0.0, 0.0, 100.0, 100.0).unwrap();
    let mut data = ctx.get_image_data(0.0, 0.0, width as f64, height as f64).unwrap();
    let vec_data = data.data().to_vec();
    return data;
}

/// Place the ImageData onto the 2D context.
#[wasm_bindgen]
pub fn putImageData(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d, mut new_image: PhotonImage) {
    // Convert the raw pixels back to an ImageData object.
    let newData = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut new_image.raw_pixels), canvas.width(), canvas.height());

    // Place the new imagedata onto the canvas
    ctx.put_image_data(&newData.unwrap(), 0.0, 0.0);
}

/// Convert a HTML5 Canvas Element to a PhotonImage.
/// 
/// This converts the ImageData found in the canvas context to a PhotonImage,
/// which can then have effects or filters applied to it.
#[wasm_bindgen]
#[no_mangle]
pub fn open_image(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> PhotonImage {
    let imgdata = getImageData(&canvas, &ctx);
    let raw_pixels = to_raw_pixels(imgdata);
    return PhotonImage {raw_pixels: raw_pixels, width: canvas.width(), height: canvas.height() }
}

/// Create a new RGB colour. TODO Will be using struct impl soon. 
#[wasm_bindgen]
pub fn new_rgb(imgdata: ImageData, r:u8, g:u8, b:u8) -> Rgb {
    let rgb = Rgb{r, g, b};
    return rgb;
}

#[wasm_bindgen]
pub fn to_raw_pixels(imgdata: ImageData) -> Vec<u8> {
    let mut img_vec = imgdata.data().to_vec();
    return img_vec;
}

/// Convert a PhotonImage to JS-compatible ImageData
#[wasm_bindgen]
pub fn to_image_data(photon_image: PhotonImage) -> ImageData {
    let mut raw_pixels = photon_image.raw_pixels;
    let width = photon_image.width;
    let height = photon_image.height;
    let newData = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut raw_pixels), width, height).unwrap();

    return newData;
}

// draw image to canvas
//ctx.draw_image_with_html_image_element(&img, 0.0, 0.0);

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
pub mod native;
pub mod text;
pub mod colour_spaces;
pub mod multiple;
pub mod noise;
pub mod helpers;