use crate::pipeline::PlanarImage;

use crate::pipeline::spatial_ops::common::restore_alpha_if_filter_zeroed_it;

#[target_feature(enable = "simd128")]
pub(in crate::pipeline::spatial_ops) unsafe fn box_blur_3x3_simd(
    src: &PlanarImage,
    dst: &mut PlanarImage,
    scratch: &mut [i16],
) {
    let width = src.width() as usize;
    let height = src.height() as usize;

    dst.r.fill(0);
    dst.g.fill(0);
    dst.b.fill(0);
    dst.a.fill(0);

    box_blur_3x3_channel_simd(&src.r, &mut dst.r, scratch, width, height);
    box_blur_3x3_channel_simd(&src.g, &mut dst.g, scratch, width, height);
    box_blur_3x3_channel_simd(&src.b, &mut dst.b, scratch, width, height);
    box_blur_3x3_channel_simd(&src.a, &mut dst.a, scratch, width, height);
    restore_alpha_if_filter_zeroed_it(src, dst, width, height);
}

#[target_feature(enable = "simd128")]
unsafe fn box_blur_3x3_channel_simd(
    src: &[u8],
    dst: &mut [u8],
    scratch: &mut [i16],
    width: usize,
    height: usize,
) {
    use core::arch::wasm32::*;

    if width < 3 || height < 3 {
        return;
    }

    for y in 0..height {
        let row = y * width;
        let mut x = 1;

        while x + 16 <= width - 1 {
            let left = v128_load(src.as_ptr().add(row + x - 1) as *const v128);
            let center = v128_load(src.as_ptr().add(row + x) as *const v128);
            let right = v128_load(src.as_ptr().add(row + x + 1) as *const v128);

            let sum_lo = i16x8_add(
                i16x8_add(i16x8_extend_low_u8x16(left), i16x8_extend_low_u8x16(center)),
                i16x8_extend_low_u8x16(right),
            );
            let sum_hi = i16x8_add(
                i16x8_add(
                    i16x8_extend_high_u8x16(left),
                    i16x8_extend_high_u8x16(center),
                ),
                i16x8_extend_high_u8x16(right),
            );

            v128_store(scratch.as_mut_ptr().add(row + x) as *mut v128, sum_lo);
            v128_store(scratch.as_mut_ptr().add(row + x + 8) as *mut v128, sum_hi);

            x += 16;
        }

        while x < width - 1 {
            scratch[row + x] =
                src[row + x - 1] as i16 + src[row + x] as i16 + src[row + x + 1] as i16;
            x += 1;
        }
    }

    for y in 1..height - 1 {
        let top = (y - 1) * width;
        let mid = y * width;
        let bot = (y + 1) * width;
        let mut x = 1;

        while x + 16 <= width - 1 {
            let top_lo = v128_load(scratch.as_ptr().add(top + x) as *const v128);
            let mid_lo = v128_load(scratch.as_ptr().add(mid + x) as *const v128);
            let bot_lo = v128_load(scratch.as_ptr().add(bot + x) as *const v128);
            let top_hi = v128_load(scratch.as_ptr().add(top + x + 8) as *const v128);
            let mid_hi = v128_load(scratch.as_ptr().add(mid + x + 8) as *const v128);
            let bot_hi = v128_load(scratch.as_ptr().add(bot + x + 8) as *const v128);

            let sum_lo = u16x8_add(u16x8_add(top_lo, mid_lo), bot_lo);
            let sum_hi = u16x8_add(u16x8_add(top_hi, mid_hi), bot_hi);
            let avg_lo = div_const_u16x8_simd!(sum_lo, 9);
            let avg_hi = div_const_u16x8_simd!(sum_hi, 9);
            let out = u8x16_narrow_i16x8(avg_lo, avg_hi);

            v128_store(dst.as_mut_ptr().add(mid + x) as *mut v128, out);

            x += 16;
        }

        while x < width - 1 {
            let sum = scratch[top + x] as u32
                + scratch[mid + x] as u32
                + scratch[bot + x] as u32;
            dst[mid + x] = (sum / 9) as u8;
            x += 1;
        }
    }
}
