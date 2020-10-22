use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use photon_rs::channels::alter_red_channel;
use photon_rs::conv::gaussian_blur;
use photon_rs::native::{open_image, save_image};
use photon_rs::transform::{resize, SamplingFilter};
use photon_rs::PhotonImage;
use std::time::Duration;

fn gaussian_blur_3x3(img: &mut PhotonImage) {
    gaussian_blur(img, 3);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("invert_image", |b| b.iter(|| invert_image()));

    c.bench_function("resize_png", |b| b.iter(|| resize_png()));

    c.bench_function("resize_jpg", |b| b.iter(|| resize_jpg()));
}

fn invert_image() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("examples/input_images/underground.jpg").expect("File should open");

    // Invert the image
    photon_rs::channels::invert(&mut img);

    let output_img_path = "output.jpg";

    // Write to filesystem
    save_image(img, output_img_path).expect("File should be saved");
}

fn resize_png() {
    let mut img = open_image("examples/input_images/underground.png").expect("File should open");

    let resized_img = resize(&mut img, 800, 600, SamplingFilter::Lanczos3);

    let output_img_path = "output.png";

    save_image(resized_img, output_img_path).expect("File should be saved");
}

fn resize_jpg() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("examples/input_images/underground.jpg").expect("File should open");

    let resized_img = resize(&mut img, 800, 600, SamplingFilter::Lanczos3);

    let output_img_path = "output.jpg";

    save_image(resized_img, output_img_path).expect("File should be saved");
}

fn alter_sample_size() -> Criterion {
    Criterion::default()
        .sample_size(10_usize)
        .measurement_time(Duration::from_secs(10_u64))
}

criterion_group! { name = benches; config = alter_sample_size(); targets = criterion_benchmark }
criterion_main!(benches);
