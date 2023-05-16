use criterion::{criterion_group, criterion_main, Criterion};
use photon_rs::native::{open_image, save_image};
use photon_rs::transform::{resize, SamplingFilter};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("invert_image", |b| b.iter(invert_image));

    c.bench_function("resize_png", |b| b.iter(resize_png));

    c.bench_function("resize_jpg", |b| b.iter(resize_jpg));
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
