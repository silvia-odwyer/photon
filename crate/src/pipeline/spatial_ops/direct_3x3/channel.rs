#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! direct_3x3_simd_channel {
    (
        $src:expr,
        $dst:expr,
        $width:expr,
        $height:expr,
        raw [$($kernel:tt)+],
        expr [$k0:expr, $k1:expr, $k2:expr,
              $k3:expr, $k4:expr, $k5:expr,
              $k6:expr, $k7:expr, $k8:expr $(,)?]
    ) => {{
        #[allow(unused_variables)]
        {
        use core::arch::wasm32::*;

        let src = $src;
        let dst = $dst;
        let width = $width;
        let height = $height;
        const DIVISOR: u32 =
            kernel_divisor_u32!([$k0, $k1, $k2, $k3, $k4, $k5, $k6, $k7, $k8]);
        const MAX_VALUE: u32 = kernel_max_positive_value_u32!([
            $k0, $k1, $k2, $k3, $k4, $k5, $k6, $k7, $k8
        ]);

        if width >= 3 && height >= 3 {
            for y in 1..height - 1 {
                let top = (y - 1) * width;
                let mid = y * width;
                let bot = (y + 1) * width;
                let mut x = 1;

                while x + 16 <= width - 1 {
                    let top_left =
                        v128_load(src.as_ptr().add(top + x - 1) as *const v128);
                    let top_center = v128_load(src.as_ptr().add(top + x) as *const v128);
                    let top_right =
                        v128_load(src.as_ptr().add(top + x + 1) as *const v128);

                    let mid_left =
                        v128_load(src.as_ptr().add(mid + x - 1) as *const v128);
                    let mid_center = v128_load(src.as_ptr().add(mid + x) as *const v128);
                    let mid_right =
                        v128_load(src.as_ptr().add(mid + x + 1) as *const v128);

                    let bot_left =
                        v128_load(src.as_ptr().add(bot + x - 1) as *const v128);
                    let bot_center = v128_load(src.as_ptr().add(bot + x) as *const v128);
                    let bot_right =
                        v128_load(src.as_ptr().add(bot + x + 1) as *const v128);

                    let lo = kernel_3x3_i16x8!(
                        [$($kernel)+],
                        [
                            i16x8_extend_low_u8x16(top_left),
                            i16x8_extend_low_u8x16(top_center),
                            i16x8_extend_low_u8x16(top_right),
                            i16x8_extend_low_u8x16(mid_left),
                            i16x8_extend_low_u8x16(mid_center),
                            i16x8_extend_low_u8x16(mid_right),
                            i16x8_extend_low_u8x16(bot_left),
                            i16x8_extend_low_u8x16(bot_center),
                            i16x8_extend_low_u8x16(bot_right)
                        ]
                    );

                    let hi = kernel_3x3_i16x8!(
                        [$($kernel)+],
                        [
                            i16x8_extend_high_u8x16(top_left),
                            i16x8_extend_high_u8x16(top_center),
                            i16x8_extend_high_u8x16(top_right),
                            i16x8_extend_high_u8x16(mid_left),
                            i16x8_extend_high_u8x16(mid_center),
                            i16x8_extend_high_u8x16(mid_right),
                            i16x8_extend_high_u8x16(bot_left),
                            i16x8_extend_high_u8x16(bot_center),
                            i16x8_extend_high_u8x16(bot_right)
                        ]
                    );

                    let out = divide_and_pack_i16x8!(lo, hi, DIVISOR, MAX_VALUE);
                    v128_store(dst.as_mut_ptr().add(mid + x) as *mut v128, out);

                    x += 16;
                }

                while x < width - 1 {
                    let value = src[top + x - 1] as i32 * $k0
                        + src[top + x] as i32 * $k1
                        + src[top + x + 1] as i32 * $k2
                        + src[mid + x - 1] as i32 * $k3
                        + src[mid + x] as i32 * $k4
                        + src[mid + x + 1] as i32 * $k5
                        + src[bot + x - 1] as i32 * $k6
                        + src[bot + x] as i32 * $k7
                        + src[bot + x + 1] as i32 * $k8;

                    let value = value / DIVISOR as i32;
                    dst[mid + x] = value.clamp(0, 255) as u8;
                    x += 1;
                }
            }
        }
        }
    }};
}
