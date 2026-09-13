// Wasm benchmarks can be run with:
// `wasm-pack test --node --release crate --test pipeline_combined_benchmark -- --nocapture`
// Scalar Wasm variant:
// `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS='' RUSTFLAGS='' wasm-pack test --node --release crate --test pipeline_combined_benchmark -- --nocapture`
#![cfg(target_arch = "wasm32")]

mod common;

use common::*;
use photon_rs::channels::{alter_channels, invert, swap_channels};
use photon_rs::conv::{
    box_blur, edge_detection, emboss, sharpen, sobel_global, sobel_horizontal,
};
use photon_rs::monochrome::{grayscale, monochrome};
use std::sync::Arc;
use wasm_bindgen_test::*;

const COMBINED_BENCH_CONFIG: BenchConfig = BenchConfig {
    name: "combined",
    images: DEFAULT_IMAGES,
    iterations: 50,
    warmups: 10,
};

#[wasm_bindgen_test]
fn run_pipeline_combined_benchmarks() {
    let benches = vec![
        Bench {
            name: "pixels_then_box_blur",
            original: Box::new(|img| {
                grayscale(img);
                invert(img);
                alter_channels(img, 10, -20, 30);
                box_blur(img);
            }),
            pipeline: Arc::new(|p| {
                p.grayscale()
                    .invert()
                    .alter_channels(10, -20, 30)
                    .box_blur()
            }),
        },
        Bench {
            name: "pixels_then_sharpen",
            original: Box::new(|img| {
                grayscale(img);
                monochrome(img, 40, 50, 100);
                invert(img);
                sharpen(img);
            }),
            pipeline: Arc::new(|p| {
                p.grayscale().monochrome(40, 50, 100).invert().sharpen()
            }),
        },
        Bench {
            name: "pixels_spatial_pixels",
            original: Box::new(|img| {
                grayscale(img);
                alter_channels(img, 20, -10, 5);
                box_blur(img);
                invert(img);
                alter_channels(img, -5, 15, -20);
            }),
            pipeline: Arc::new(|p| {
                p.grayscale()
                    .alter_channels(20, -10, 5)
                    .box_blur()
                    .invert()
                    .alter_channels(-5, 15, -20)
            }),
        },
        Bench {
            name: "pixels_then_sobel",
            original: Box::new(|img| {
                grayscale(img);
                invert(img);
                alter_channels(img, 10, -20, 30);
                sobel_horizontal(img);
            }),
            pipeline: Arc::new(|p| {
                p.grayscale()
                    .invert()
                    .alter_channels(10, -20, 30)
                    .sobel_horizontal()
            }),
        },
        Bench {
            name: "pixels_then_sobel_global",
            original: Box::new(|img| {
                grayscale(img);
                alter_channels(img, 10, -20, 30);
                invert(img);
                sobel_global(img);
            }),
            pipeline: Arc::new(|p| {
                p.grayscale()
                    .alter_channels(10, -20, 30)
                    .invert()
                    .sobel_global()
            }),
        },
        Bench {
            name: "mixed_chain",
            original: Box::new(|img| {
                grayscale(img);
                invert(img);
                alter_channels(img, 10, -20, 30);
                box_blur(img);
                monochrome(img, 40, 50, 100);
                sharpen(img);
                swap_channels(img, 0, 2);
                edge_detection(img);
                invert(img);
                emboss(img);
            }),
            pipeline: Arc::new(|p| {
                p.grayscale()
                    .invert()
                    .alter_channels(10, -20, 30)
                    .box_blur()
                    .monochrome(40, 50, 100)
                    .sharpen()
                    .swap_channels(0, 2)
                    .edge_detection()
                    .invert()
                    .emboss()
            }),
        },
    ];

    bench_with_config(benches, COMBINED_BENCH_CONFIG);
}
