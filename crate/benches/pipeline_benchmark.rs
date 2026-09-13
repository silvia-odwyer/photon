use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use photon_rs::channels::{alter_channels, invert, swap_channels};
use photon_rs::monochrome::{grayscale, monochrome};
use photon_rs::pipeline::Pipeline;
use photon_rs::PhotonImage;
use std::time::Duration;

fn synthetic_image(width: u32, height: u32) -> PhotonImage {
    let pixels = (width * height) as usize;
    let mut raw = Vec::with_capacity(pixels * 4);

    for i in 0..pixels {
        let x = (i % width as usize) as u32;
        let y = (i / width as usize) as u32;

        raw.push(((x * 13 + y * 3) & 0xff) as u8);
        raw.push(((x * 5 + y * 11) & 0xff) as u8);
        raw.push(((x * 17 + y * 7) & 0xff) as u8);
        raw.push(255);
    }

    PhotonImage::new(raw, width, height)
}

fn bench_pipeline(c: &mut Criterion) {
    let image = synthetic_image(1920, 1080);

    let mut group = c.benchmark_group("pipeline");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("conversion_only", |b| {
        b.iter_batched(
            || image.clone(),
            |img| black_box(Pipeline::from_photon_image(&img).finish()),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("scalar_grayscale_invert_alter_channels", |b| {
        b.iter_batched(
            || image.clone(),
            |mut img| {
                grayscale(&mut img);
                invert(&mut img);
                alter_channels(&mut img, 10, -20, 30);
                black_box(img)
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("pipeline_grayscale_invert_alter_channels", |b| {
        b.iter_batched(
            || image.clone(),
            |img| {
                black_box(
                    Pipeline::from_photon_image(&img)
                        .grayscale()
                        .invert()
                        .alter_channels(10, -20, 30)
                        .finish(),
                )
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("scalar_monochrome", |b| {
        b.iter_batched(
            || image.clone(),
            |mut img| {
                monochrome(&mut img, 40, 50, 100);
                black_box(img)
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("pipeline_monochrome", |b| {
        b.iter_batched(
            || image.clone(),
            |img| {
                black_box(
                    Pipeline::from_photon_image(&img)
                        .monochrome(40, 50, 100)
                        .finish(),
                )
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("scalar_swap_channels", |b| {
        b.iter_batched(
            || image.clone(),
            |mut img| {
                swap_channels(&mut img, 0, 2);
                black_box(img)
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("pipeline_swap_channels", |b| {
        b.iter_batched(
            || image.clone(),
            |img| {
                black_box(
                    Pipeline::from_photon_image(&img)
                        .swap_channels(0, 2)
                        .finish(),
                )
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("scalar_mixed_chain_with_swap", |b| {
        b.iter_batched(
            || image.clone(),
            |mut img| {
                grayscale(&mut img);
                alter_channels(&mut img, 10, -20, 30);
                swap_channels(&mut img, 0, 2);
                invert(&mut img);
                black_box(img)
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("pipeline_mixed_chain_with_swap", |b| {
        b.iter_batched(
            || image.clone(),
            |img| {
                black_box(
                    Pipeline::from_photon_image(&img)
                        .grayscale()
                        .alter_channels(10, -20, 30)
                        .swap_channels(0, 2)
                        .invert()
                        .finish(),
                )
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("scalar_long_fusible_chain", |b| {
        b.iter_batched(
            || image.clone(),
            |mut img| {
                grayscale(&mut img);
                invert(&mut img);
                alter_channels(&mut img, 10, -20, 30);
                monochrome(&mut img, 40, 50, 100);
                invert(&mut img);
                alter_channels(&mut img, -5, 25, -10);
                monochrome(&mut img, 0, 20, 40);
                invert(&mut img);
                alter_channels(&mut img, 12, -8, 4);
                monochrome(&mut img, 5, 15, 25);
                invert(&mut img);
                alter_channels(&mut img, -30, 5, 10);
                black_box(img)
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("pipeline_long_fusible_chain", |b| {
        b.iter_batched(
            || image.clone(),
            |img| {
                black_box(
                    Pipeline::from_photon_image(&img)
                        .grayscale()
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
                        .finish(),
                )
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(10))
}

criterion_group! { name = benches; config = criterion_config(); targets = bench_pipeline }
criterion_main!(benches);
