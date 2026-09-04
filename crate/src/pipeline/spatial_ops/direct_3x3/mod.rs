use crate::pipeline::spatial_ops::common::restore_alpha_if_filter_zeroed_it;
use crate::pipeline::PlanarImage;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[macro_use]
mod coeff;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[macro_use]
mod division;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[macro_use]
mod kernel;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[macro_use]
mod channel;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub mod edge_one_simd;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! apply_direct_3x3_simd {
    (
        $pipeline:ident,
        [$($kernel:tt)+]
    ) => {{
        $pipeline.flush_pixel_ops();
        $pipeline.ensure_scratch();

        let width = $pipeline.image.width() as usize;
        let height = $pipeline.image.height() as usize;
        let scratch = $pipeline.scratch.as_mut().unwrap();

        scratch.r.fill(0);
        scratch.g.fill(0);
        scratch.b.fill(0);
        scratch.a.fill(0);

        direct_3x3_simd_channel!(
            &$pipeline.image.r,
            &mut scratch.r,
            width,
            height,
            raw [$($kernel)+],
            expr [$($kernel)+]
        );
        direct_3x3_simd_channel!(
            &$pipeline.image.g,
            &mut scratch.g,
            width,
            height,
            raw [$($kernel)+],
            expr [$($kernel)+]
        );
        direct_3x3_simd_channel!(
            &$pipeline.image.b,
            &mut scratch.b,
            width,
            height,
            raw [$($kernel)+],
            expr [$($kernel)+]
        );
        direct_3x3_simd_channel!(
            &$pipeline.image.a,
            &mut scratch.a,
            width,
            height,
            raw [$($kernel)+],
            expr [$($kernel)+]
        );

        restore_alpha_if_filter_zeroed_it(&$pipeline.image, scratch, width, height);

        std::mem::swap(&mut $pipeline.image, scratch);
    }};
}

pub fn convolve_3x3(src: &PlanarImage, dst: &mut PlanarImage, kernel: [f32; 9]) {
    let width = src.width() as usize;
    let height = src.height() as usize;

    // If we want to match the result from the photon library, then we skip the borders
    // So we must fill the lanes with 0 beforehand so that we do not have accidental stale data in there
    dst.r.fill(0);
    dst.g.fill(0);
    dst.b.fill(0);
    dst.a.fill(0);

    convolve_3x3_channel(&src.r, &mut dst.r, width, height, kernel);
    convolve_3x3_channel(&src.g, &mut dst.g, width, height, kernel);
    convolve_3x3_channel(&src.b, &mut dst.b, width, height, kernel);
    convolve_3x3_channel(&src.a, &mut dst.a, width, height, kernel);
    restore_alpha_if_filter_zeroed_it(src, dst, width, height);
}

fn convolve_3x3_channel(
    src: &[u8],
    dst: &mut [u8],
    width: usize,
    height: usize,
    kernel: [f32; 9],
) {
    if width < 3 || height < 3 {
        return;
    }

    let sum: f32 = kernel.iter().sum();
    let divisor = if sum == 0.0 { 1.0 } else { sum };

    for y in 1..height - 1 {
        let top = (y - 1) * width;
        let mid = y * width;
        let bot = (y + 1) * width;

        for x in 1..width - 1 {
            let value = src[top + x - 1] as f32 * kernel[0]
                + src[top + x] as f32 * kernel[1]
                + src[top + x + 1] as f32 * kernel[2]
                + src[mid + x - 1] as f32 * kernel[3]
                + src[mid + x] as f32 * kernel[4]
                + src[mid + x + 1] as f32 * kernel[5]
                + src[bot + x - 1] as f32 * kernel[6]
                + src[bot + x] as f32 * kernel[7]
                + src[bot + x + 1] as f32 * kernel[8];

            dst[mid + x] = (value / divisor).clamp(0.0, 255.0) as u8;
        }
    }
}
