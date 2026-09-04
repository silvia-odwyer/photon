use crate::pipeline::spatial_ops::common::restore_alpha_if_filter_zeroed_it;
use crate::pipeline::PlanarImage;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub mod box_blur_simd;

pub fn convolve_separable_3x3(
    src: &PlanarImage,
    dst: &mut PlanarImage,
    scratch: &mut [f32],
    horizontal: [f32; 3],
    vertical: [f32; 3],
) {
    let width = src.width() as usize;
    let height = src.height() as usize;

    dst.r.fill(0);
    dst.g.fill(0);
    dst.b.fill(0);
    dst.a.fill(0);

    convolve_separable_3x3_channel(
        &src.r, &mut dst.r, scratch, width, height, horizontal, vertical,
    );
    convolve_separable_3x3_channel(
        &src.g, &mut dst.g, scratch, width, height, horizontal, vertical,
    );
    convolve_separable_3x3_channel(
        &src.b, &mut dst.b, scratch, width, height, horizontal, vertical,
    );
    convolve_separable_3x3_channel(
        &src.a, &mut dst.a, scratch, width, height, horizontal, vertical,
    );
    restore_alpha_if_filter_zeroed_it(src, dst, width, height);
}

fn convolve_separable_3x3_channel(
    src: &[u8],
    dst: &mut [u8],
    scratch: &mut [f32],
    width: usize,
    height: usize,
    horizontal: [f32; 3],
    vertical: [f32; 3],
) {
    if width < 3 || height < 3 {
        return;
    }

    let divisor = {
        let sum = horizontal.iter().sum::<f32>() * vertical.iter().sum::<f32>();
        if sum == 0.0 {
            1.0
        } else {
            sum
        }
    };

    for y in 0..height {
        let row = y * width;

        for x in 1..width - 1 {
            scratch[row + x] = src[row + x - 1] as f32 * horizontal[0]
                + src[row + x] as f32 * horizontal[1]
                + src[row + x + 1] as f32 * horizontal[2];
        }
    }

    for y in 1..height - 1 {
        let top = (y - 1) * width;
        let mid = y * width;
        let bot = (y + 1) * width;

        for x in 1..width - 1 {
            let value = scratch[top + x] * vertical[0]
                + scratch[mid + x] * vertical[1]
                + scratch[bot + x] * vertical[2];

            dst[mid + x] = (value / divisor).clamp(0.0, 255.0) as u8;
        }
    }
}
