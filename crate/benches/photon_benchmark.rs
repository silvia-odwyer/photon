use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use photon_rs::PhotonImage;
use photon_rs::native::{open_image};
use photon_rs::conv::{fast_gaussian_blur, gaussian_blur};

fn gaussian_blur_3x3_1(img: &mut PhotonImage) {
    fast_gaussian_blur(img, 1);
}

fn gaussian_blur_3x3_2(img: &mut PhotonImage) {
    gaussian_blur(img);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Gaussian Blur");

    let mut img = open_image("examples/input_images/daisies_fuji.jpg");
    group.bench_function(BenchmarkId::new("fast_gaussian_blur", 0),
        |b| b.iter(|| gaussian_blur_3x3_1(&mut img)));

    let mut img = open_image("examples/input_images/underground.jpg");
    group.bench_function(BenchmarkId::new("fast_gaussian_blur", 1),
        |b| b.iter(|| gaussian_blur_3x3_1(&mut img)));

    img = open_image("examples/input_images/daisies_fuji.jpg");
    group.bench_function(BenchmarkId::new("normal_gaussian_blur", 2), 
        |b| b.iter(|| gaussian_blur_3x3_2(&mut img)));

    img = open_image("examples/input_images/underground.jpg");
    group.bench_function(BenchmarkId::new("normal_gaussian_blur", 3), 
        |b| b.iter(|| gaussian_blur_3x3_2(&mut img)));


    group.finish();
}

fn alter_sample_size() -> Criterion {
    Criterion::default().sample_size(50)
}

criterion_group! { name = benches; config = alter_sample_size(); targets = criterion_benchmark }
criterion_main!(benches);
