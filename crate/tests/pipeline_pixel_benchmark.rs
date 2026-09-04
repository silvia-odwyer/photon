// Wasm benchmarks can be run with `wasm-pack test --node --release crate --test pipeline_benchmark -- --nocapture`
#![cfg(target_arch = "wasm32")]

mod common;

use common::*;
use photon_rs::channels::{alter_channels, invert, swap_channels};
use photon_rs::conv::box_blur;
use photon_rs::monochrome::{grayscale, monochrome};
use std::sync::Arc;
use wasm_bindgen_test::*;

const PIXEL_BENCH_CONFIG: BenchConfig = BenchConfig {
    name: "pixel",
    images: DEFAULT_IMAGES,
    iterations: 300,
    warmups: 50,
};

#[wasm_bindgen_test]
fn run_pipeline_pixel_benchmarks() {
    let benches = vec![
        // Single operations.
        Bench {
            name: "invert",
            original: Box::new(|img| invert(img)),
            pipeline: Arc::new(|p| p.invert()),
        },
        Bench {
            name: "monochrome",
            original: Box::new(|img| monochrome(img, 40, 50, 100)),
            pipeline: Arc::new(|p| p.monochrome(40, 50, 100)),
        },
        Bench {
            name: "swap_channels",
            original: Box::new(|img| swap_channels(img, 0, 2)),
            pipeline: Arc::new(|p| p.swap_channels(0, 2)),
        },
        Bench {
            name: "box_blur_3x3",
            original: Box::new(|img| box_blur(img)),
            pipeline: Arc::new(|p| p.box_blur()),
        },
        // Multiple chained operations.
        Bench {
            name: "chain_of_2",
            original: Box::new(|img| {
                invert(img);
                alter_channels(img, 20, -10, 5);
            }),
            pipeline: Arc::new(|p| p.invert().alter_channels(20, -10, 5)),
        },
        Bench {
            name: "chain_of_3",
            original: Box::new(|img| {
                grayscale(img);
                invert(img);
                alter_channels(img, 10, -20, 30);
            }),
            pipeline: Arc::new(|p| p.grayscale().invert().alter_channels(10, -20, 30)),
        },
        Bench {
            name: "chain_of_4",
            original: Box::new(|img| {
                grayscale(img);
                alter_channels(img, 10, -20, 30);
                swap_channels(img, 0, 2);
                invert(img);
            }),
            pipeline: Arc::new(|p| {
                p.grayscale()
                    .alter_channels(10, -20, 30)
                    .swap_channels(0, 2)
                    .invert()
            }),
        },
        Bench {
            name: "chain_of_12",
            original: Box::new(|img| {
                grayscale(img);
                invert(img);
                alter_channels(img, 10, -20, 30);
                monochrome(img, 40, 50, 100);
                invert(img);
                alter_channels(img, -5, 25, -10);
                monochrome(img, 0, 20, 40);
                invert(img);
                alter_channels(img, 12, -8, 4);
                monochrome(img, 5, 15, 25);
                invert(img);
                alter_channels(img, -30, 5, 10);
            }),
            pipeline: Arc::new(|p| {
                p.grayscale()
                    .invert()
                    .alter_channels(10, -20, 30)
                    .monochrome(40, 50, 100)
                    .invert()
                    .alter_channels(-5, 25, -10)
                    .monochrome(0, 20, 40)
                    .invert()
                    .alter_channels(12, -8, 4)
                    .monochrome(5, 15, 25)
                    .invert()
                    .alter_channels(-30, 5, 10)
            }),
        },
    ];

    bench_with_config(benches, PIXEL_BENCH_CONFIG);
}
