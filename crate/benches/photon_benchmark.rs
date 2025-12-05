use criterion::{criterion_group, criterion_main, Criterion};
use photon_rs::native::{open_image, save_image};
use photon_rs::transform::{resize, SamplingFilter};
use photon_rs::adjustments::*;
use photon_rs::corrections::*;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("invert_image", |b| b.iter(invert_image));

    c.bench_function("resize_png", |b| b.iter(resize_png));

    c.bench_function("resize_jpg", |b| b.iter(resize_jpg));

    // Benchmarks for adjustment functions
    let mut group = c.benchmark_group("adjustments");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    // Load test image for benchmarking (same as other benchmarks)
    let test_image = open_image("examples/input_images/underground.jpg")
        .expect("Test image should load");
    
    group.bench_function("apply_exposure", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_exposure(&mut img, 1.5))
    });

    group.bench_function("apply_white_balance", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_white_balance(&mut img, -20.0, 5.0))
    });

    group.bench_function("apply_vibrance", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_vibrance(&mut img, 30.0))
    });

    group.bench_function("apply_clarity", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_clarity(&mut img, 25.0))
    });

    group.bench_function("apply_texture", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_texture(&mut img, 30.0))
    });

    group.bench_function("apply_dehaze", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_dehaze(&mut img, 50.0))
    });

    group.bench_function("apply_vignette", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_vignette(&mut img, 50.0, 30.0, 50.0))
    });

    group.bench_function("apply_tone_zones", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_tone_zones(&mut img, 10, 20, -10, 5))
    });

    group.bench_function("apply_color_grading", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_color_grading(&mut img, 200.0, 20.0, -10.0, 0.0, 0.0, 0.0, 30.0, 15.0, 5.0, 50.0, 0.0))
    });

    group.bench_function("apply_sharpening", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_sharpening(&mut img, 100.0, 1.0, 2.0, 50.0))
    });

    group.bench_function("apply_noise_reduction", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_noise_reduction(&mut img, 40.0, 50.0, 50.0))
    });

    group.bench_function("apply_noise_reduction_bilateral", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_noise_reduction_bilateral(&mut img, 40.0, 50.0, 50.0))
    });

    group.bench_function("apply_noise_reduction_wavelets", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_noise_reduction_wavelets(&mut img, 50.0, 30.0))
    });

    group.bench_function("apply_noise_reduction_median", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_noise_reduction_median(&mut img, 2))
    });

    group.bench_function("apply_noise_reduction_nlm", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_noise_reduction_nlm(&mut img, 50.0, 3, 5))
    });

    group.bench_function("apply_tone_curve", |b| {
        let lut: Vec<u8> = (0..256).map(|i| i as u8).collect();
        b.iter_with_setup(
            || test_image.clone(),
            |mut img| apply_tone_curve(&mut img, lut.clone())
        )
    });

    group.bench_function("apply_chromatic_aberration", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_chromatic_aberration(&mut img, 50.0, 30.0))
    });

    group.bench_function("apply_lens_correction", |b| {
        let mut img = test_image.clone();
        b.iter(|| apply_lens_correction(&mut img, -20.0, 30.0))
    });

    group.finish();
}

fn invert_image() {
    // Open the image (a PhotonImage is returned)
    let mut img =
        open_image("examples/input_images/underground.jpg").expect("File should open");

    // Invert the image
    photon_rs::channels::invert(&mut img);

    let output_img_path = "output.jpg";

    // Write to filesystem
    save_image(img, output_img_path).unwrap();
}

fn resize_png() {
    let img =
        open_image("examples/input_images/underground.png").expect("File should open");

    let resized_img = resize(&img, 800, 600, SamplingFilter::Lanczos3);

    let output_img_path = "output.png";

    save_image(resized_img, output_img_path).unwrap();
}

fn resize_jpg() {
    // Open the image (a PhotonImage is returned)
    let img =
        open_image("examples/input_images/underground.jpg").expect("File should open");

    let resized_img = resize(&img, 800, 600, SamplingFilter::Lanczos3);

    let output_img_path = "output.jpg";

    save_image(resized_img, output_img_path).unwrap();
}

fn alter_sample_size() -> Criterion {
    Criterion::default()
        .sample_size(10_usize)
        .measurement_time(Duration::from_secs(10_u64))
}

criterion_group! { name = benches; config = alter_sample_size(); targets = criterion_benchmark }
criterion_main!(benches);
