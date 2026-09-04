#![cfg(target_arch = "wasm32")]
#![allow(dead_code)]
use photon_rs::pipeline::Pipeline;
use photon_rs::PhotonImage;
use std::sync::Arc;
use wasm_bindgen::prelude::wasm_bindgen;

// `println!` doesn't work with Wasm so we make a macro for console logging (with web_sys)
macro_rules! log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

// Node.js fs binding and timer
#[wasm_bindgen(inline_js = r#"
import { writeFileSync } from 'fs';

export function write_file(path, content) {
    writeFileSync(path, content, 'utf8');
}
export function now() {
    return performance.now();
}
"#)]
extern "C" {
    fn write_file(path: &str, content: &str);
    fn now() -> f64;
}
const DEFAULT_NAME: &str = "default";

pub const DEFAULT_IMAGES: &[(&str, &[u8])] = &[
    ("Lena 512x512", include_bytes!("../assets/lena.png")),
    ("Perlin 1280x720", include_bytes!("../assets/1280x720.png")),
    (
        "Perlin 1920x1080",
        include_bytes!("../assets/1920x1080.png"),
    ),
];

pub const DEFAULT_ITERS: u32 = 100;
pub const DEFAULT_WARMUP: u32 = 20;

pub const DEFAULT_BENCH_CONFIG: BenchConfig = BenchConfig {
    name: DEFAULT_NAME,
    images: DEFAULT_IMAGES,
    iterations: DEFAULT_ITERS,
    warmups: DEFAULT_WARMUP,
};

#[derive(Clone, Copy)]
pub struct BenchConfig {
    pub name: &'static str,
    pub images: &'static [(&'static str, &'static [u8])],
    pub iterations: u32,
    pub warmups: u32,
}

pub struct Bench {
    pub name: &'static str,
    pub original: Box<dyn Fn(&mut PhotonImage)>,
    // This is the builder chain used for
    // 1. correctness checking
    // 2. full pipeline timing,
    // 3. isolated timing (without conversion of PhotonImage -> PlanarImage taken into account)
    // It is an Arc so it can be called multiple times across the three measurement phases.
    pub pipeline: Arc<dyn Fn(Pipeline) -> Pipeline>,
}

struct Samples {
    original: Vec<f64>,
    pipeline: Vec<f64>,
    isolated: Vec<f64>,
}

fn load_image(bytes: &[u8]) -> PhotonImage {
    let rgba = image::load_from_memory(bytes)
        .expect("Failed to load Lena image")
        .to_rgba8();

    let (width, height) = rgba.dimensions();
    PhotonImage::new(rgba.into_raw(), width, height)
}

// Returns (original_ms, full_pipeline_ms, isolated_ms).
fn validate_and_measure(
    bench: &Bench,
    img: &PhotonImage,
    config: BenchConfig,
) -> (f64, f64, f64, Samples) {
    // Correctness check: original and pipeline must have the same output
    let mut original_out = img.clone();
    (bench.original)(&mut original_out);
    let pipeline_out = (bench.pipeline)(Pipeline::from_photon_image(img)).finish();
    if bench.name != "identity" {
        let original_pixels = original_out.get_raw_pixels();
        let pipeline_pixels = pipeline_out.get_raw_pixels();
        for (i, (&a, &b)) in original_pixels
            .iter()
            .zip(pipeline_pixels.iter())
            .enumerate()
        {
            if a == b {
                continue;
            }
            let pixel_index = i / 4;
            let x = pixel_index as u32 % img.get_width();
            let y = pixel_index as u32 / img.get_width();
            log!(
                "{}: first mismatch at pixel ({}, {}), original={}, pipeline={}",
                bench.name,
                x,
                y,
                a,
                b
            );
            panic!();
        }
    }

    // Benchmark: original vs pipeline

    // Timing stage 1: original
    // Not timed, warm-up
    for _ in 0..config.warmups {
        let mut img_clone = img.clone();
        (bench.original)(&mut img_clone);
    }

    // Timed
    let mut sum = 0.0;
    let mut original_samples = Vec::with_capacity(config.iterations as usize);
    for _ in 0..config.iterations {
        let mut img_clone = img.clone();
        let start = now();
        (bench.original)(&mut img_clone);
        let time = now() - start;
        sum += time;
        original_samples.push(time);
    }
    let original_ms = sum / config.iterations as f64;

    // Timing stage 2: full pipeline (conversion + ops)
    // Not timed, warm-up
    for _ in 0..config.warmups {
        let img_clone = img.clone();
        std::hint::black_box(
            (bench.pipeline)(Pipeline::from_photon_image(&img_clone)).finish(),
        );
    }

    let mut sum = 0.0;
    let mut pipeline_samples = Vec::with_capacity(config.iterations as usize);
    for _ in 0..config.iterations {
        let img_clone = img.clone();
        let start = now();
        std::hint::black_box(
            (bench.pipeline)(Pipeline::from_photon_image(&img_clone)).finish(),
        );
        let time = now() - start;
        sum += time;
        pipeline_samples.push(time);
    }
    let pipeline_ms = sum / config.iterations as f64;

    // Timing stage 3: isolated (convert once and time ops only)
    let input_planar = Pipeline::from_photon_image(img).finish_to_planar();
    for _ in 0..config.warmups {
        let pipeline = Pipeline::from_planar_image(input_planar.clone());
        std::hint::black_box((bench.pipeline)(pipeline).finish_to_planar());
    }

    let mut sum = 0.0;
    let mut isolated_samples = Vec::with_capacity(config.iterations as usize);
    let input_planar = Pipeline::from_photon_image(img).finish_to_planar();
    for _ in 0..config.iterations {
        let pipeline = Pipeline::from_planar_image(input_planar.clone());
        let start = now();
        std::hint::black_box((bench.pipeline)(pipeline).finish_to_planar());
        let time = now() - start;
        sum += time;
        isolated_samples.push(time);
    }
    let isolated_ms = sum / config.iterations as f64;

    (
        original_ms,
        pipeline_ms,
        isolated_ms,
        Samples {
            original: original_samples,
            pipeline: pipeline_samples,
            isolated: isolated_samples,
        },
    )
}

pub fn bench(benches: Vec<Bench>) {
    bench_with_config(benches, DEFAULT_BENCH_CONFIG);
}

pub fn bench_with_config(benches: Vec<Bench>, config: BenchConfig) {
    let variant = if cfg!(all(target_arch = "wasm32", target_feature = "simd128")) {
        "SIMD"
    } else {
        "scalar"
    };
    log!("RUNNING BENCHMARK (variant: {})", variant);

    log!(
        "| {:<24} | {:<16} | {:>13} | {:>13} | {:>13} | {:>8} | {:>8} |",
        "benchmark",
        "image",
        "original (ms)",
        "pipeline (ms)",
        "isolated (ms)",
        "full speedup",
        "ops speedup"
    );
    log!("|--------------------------|------------------|---------------|---------------|---------------|--------------|-------------|");

    let mut csv = String::from("benchmark,image,variant,stage,ms\n");

    for bench in &benches {
        for (image_name, bytes) in config.images {
            let img = load_image(bytes);
            let (original_ms, pipeline_ms, isolated_ms, samples) =
                validate_and_measure(bench, &img, config);
            log!(
                "| {:<24} | {:<16} | {:>13.4} | {:>13.4} | {:>13.4} | {:>11.2}x | {:>10.2}x |",
                bench.name,
                image_name,
                original_ms,
                pipeline_ms,
                isolated_ms,
                original_ms / pipeline_ms,
                original_ms / isolated_ms,
            );

            for (_, &ms) in samples.original.iter().enumerate() {
                csv.push_str(&format!(
                    "{},{},{},original,{:.6}\n",
                    bench.name, image_name, variant, ms
                ));
            }
            for (_, &ms) in samples.pipeline.iter().enumerate() {
                csv.push_str(&format!(
                    "{},{},{},pipeline,{:.6}\n",
                    bench.name, image_name, variant, ms
                ));
            }
            for (_, &ms) in samples.isolated.iter().enumerate() {
                csv.push_str(&format!(
                    "{},{},{},isolated,{:.6}\n",
                    bench.name, image_name, variant, ms
                ));
            }
        }
        log!("|--------------------------|------------------|---------------|---------------|---------------|--------------|-------------|");
    }

    let filename = format!("../benchmark_{}_{}.csv", config.name, variant);
    write_file(&filename, &csv);
    log!("CSV written to {}", filename);
}
