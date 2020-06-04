use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use photon_rs::PhotonImage;
use photon_rs::native::{open_image, save_image};
use photon_rs::conv::{gaussian_blur};
use photon_rs::channels::{alter_red_channel};

fn gaussian_blur_3x3(img: &mut PhotonImage) {
    gaussian_blur(img, 3);
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("invert_image", 
        |b| b.iter(|| invert_image()));
}

fn invert_image() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("examples/input_images/input.jpg");

    // Invert the image
    photon_rs::channels::invert(&mut img);

    let output_img_path = "output.jpg";

    // Write to filesystem
    save_image(img, output_img_path);

}

fn alter_sample_size() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! { name = benches; config = alter_sample_size(); targets = criterion_benchmark }
criterion_main!(benches);
