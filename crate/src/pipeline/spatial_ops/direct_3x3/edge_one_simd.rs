use crate::pipeline::PlanarImage;

use crate::pipeline::spatial_ops::common::restore_alpha_if_filter_zeroed_it;

const EDGE_ONE_DIVISOR: f32 = f32::from_bits(0x3e4cccb0);

#[target_feature(enable = "simd128")]
pub(in crate::pipeline::spatial_ops) unsafe fn edge_one_simd(
    src: &PlanarImage,
    dst: &mut PlanarImage,
) {
    let width = src.width() as usize;
    let height = src.height() as usize;

    dst.r.fill(0);
    dst.g.fill(0);
    dst.b.fill(0);
    dst.a.fill(0);

    edge_one_channel_simd(&src.r, &mut dst.r, width, height);
    edge_one_channel_simd(&src.g, &mut dst.g, width, height);
    edge_one_channel_simd(&src.b, &mut dst.b, width, height);
    edge_one_channel_simd(&src.a, &mut dst.a, width, height);
    restore_alpha_if_filter_zeroed_it(src, dst, width, height);
}

#[target_feature(enable = "simd128")]
unsafe fn edge_one_channel_simd(
    src: &[u8],
    dst: &mut [u8],
    width: usize,
    height: usize,
) {
    use core::arch::wasm32::*;

    if width < 3 || height < 3 {
        return;
    }

    for y in 1..height - 1 {
        let top = (y - 1) * width;
        let mid = y * width;
        let bot = (y + 1) * width;
        let mut x = 1;

        while x + 16 <= width - 1 {
            let top_center = v128_load(src.as_ptr().add(top + x) as *const v128);
            let top_right = v128_load(src.as_ptr().add(top + x + 1) as *const v128);
            let mid_left = v128_load(src.as_ptr().add(mid + x - 1) as *const v128);
            let mid_center = v128_load(src.as_ptr().add(mid + x) as *const v128);
            let mid_right = v128_load(src.as_ptr().add(mid + x + 1) as *const v128);
            let bot_left = v128_load(src.as_ptr().add(bot + x - 1) as *const v128);
            let bot_center = v128_load(src.as_ptr().add(bot + x) as *const v128);
            let bot_right = v128_load(src.as_ptr().add(bot + x + 1) as *const v128);

            let out0 = edge_one_lanes::<0>(
                top_center, top_right, mid_left, mid_center, mid_right, bot_left,
                bot_center, bot_right,
            );
            let out1 = edge_one_lanes::<1>(
                top_center, top_right, mid_left, mid_center, mid_right, bot_left,
                bot_center, bot_right,
            );
            let out2 = edge_one_lanes::<2>(
                top_center, top_right, mid_left, mid_center, mid_right, bot_left,
                bot_center, bot_right,
            );
            let out3 = edge_one_lanes::<3>(
                top_center, top_right, mid_left, mid_center, mid_right, bot_left,
                bot_center, bot_right,
            );

            let out_lo = u16x8_narrow_i32x4(out0, out1);
            let out_hi = u16x8_narrow_i32x4(out2, out3);
            let out = u8x16_narrow_i16x8(out_lo, out_hi);
            v128_store(dst.as_mut_ptr().add(mid + x) as *mut v128, out);

            x += 16;
        }

        while x < width - 1 {
            let value = src[top + x] as f32 * -2.2
                + src[top + x + 1] as f32 * -0.6
                + src[mid + x - 1] as f32 * -0.4
                + src[mid + x] as f32 * 2.8
                + src[mid + x + 1] as f32 * -0.3
                + src[bot + x - 1] as f32 * -0.8
                + src[bot + x] as f32 * -1.0
                + src[bot + x + 1] as f32 * 2.7;

            dst[mid + x] = (value / EDGE_ONE_DIVISOR).clamp(0.0, 255.0) as u8;
            x += 1;
        }
    }
}

#[target_feature(enable = "simd128")]
unsafe fn edge_one_lanes<const CHUNK: usize>(
    top_center: core::arch::wasm32::v128,
    top_right: core::arch::wasm32::v128,
    mid_left: core::arch::wasm32::v128,
    mid_center: core::arch::wasm32::v128,
    mid_right: core::arch::wasm32::v128,
    bot_left: core::arch::wasm32::v128,
    bot_center: core::arch::wasm32::v128,
    bot_right: core::arch::wasm32::v128,
) -> core::arch::wasm32::v128 {
    use core::arch::wasm32::*;

    let mut acc =
        f32x4_mul(u8x16_chunk_to_f32x4::<CHUNK>(top_center), f32x4_splat(-2.2));
    acc = f32x4_add(
        acc,
        f32x4_mul(u8x16_chunk_to_f32x4::<CHUNK>(top_right), f32x4_splat(-0.6)),
    );
    acc = f32x4_add(
        acc,
        f32x4_mul(u8x16_chunk_to_f32x4::<CHUNK>(mid_left), f32x4_splat(-0.4)),
    );
    acc = f32x4_add(
        acc,
        f32x4_mul(u8x16_chunk_to_f32x4::<CHUNK>(mid_center), f32x4_splat(2.8)),
    );
    acc = f32x4_add(
        acc,
        f32x4_mul(u8x16_chunk_to_f32x4::<CHUNK>(mid_right), f32x4_splat(-0.3)),
    );
    acc = f32x4_add(
        acc,
        f32x4_mul(u8x16_chunk_to_f32x4::<CHUNK>(bot_left), f32x4_splat(-0.8)),
    );
    acc = f32x4_add(
        acc,
        f32x4_mul(u8x16_chunk_to_f32x4::<CHUNK>(bot_center), f32x4_splat(-1.0)),
    );
    acc = f32x4_add(
        acc,
        f32x4_mul(u8x16_chunk_to_f32x4::<CHUNK>(bot_right), f32x4_splat(2.7)),
    );

    acc = f32x4_div(acc, f32x4_splat(EDGE_ONE_DIVISOR));
    acc = f32x4_max(acc, f32x4_splat(0.0));
    acc = f32x4_min(acc, f32x4_splat(255.0));

    u32x4_trunc_sat_f32x4(acc)
}

#[target_feature(enable = "simd128")]
unsafe fn u8x16_chunk_to_f32x4<const CHUNK: usize>(
    values: core::arch::wasm32::v128,
) -> core::arch::wasm32::v128 {
    use core::arch::wasm32::*;

    let values = match CHUNK {
        0 => u32x4_extend_low_u16x8(u16x8_extend_low_u8x16(values)),
        1 => u32x4_extend_high_u16x8(u16x8_extend_low_u8x16(values)),
        2 => u32x4_extend_low_u16x8(u16x8_extend_high_u8x16(values)),
        3 => u32x4_extend_high_u16x8(u16x8_extend_high_u8x16(values)),
        _ => unreachable!(),
    };

    f32x4_convert_u32x4(values)
}
