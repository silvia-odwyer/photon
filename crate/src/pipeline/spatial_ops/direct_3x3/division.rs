#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! positive_coeff_u32 {
    ($k:expr) => {{
        const K: i16 = $k;
        if K > 0 {
            K as u32
        } else {
            0
        }
    }};
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! kernel_divisor_u32 {
    (
        [$k0:expr, $k1:expr, $k2:expr,
         $k3:expr, $k4:expr, $k5:expr,
         $k6:expr, $k7:expr, $k8:expr $(,)?]
    ) => {{
        const SUM: i32 = $k0 as i32
            + $k1 as i32
            + $k2 as i32
            + $k3 as i32
            + $k4 as i32
            + $k5 as i32
            + $k6 as i32
            + $k7 as i32
            + $k8 as i32;
        if SUM == 0 {
            1
        } else {
            SUM as u32
        }
    }};
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! kernel_max_positive_value_u32 {
    (
        [$k0:expr, $k1:expr, $k2:expr,
         $k3:expr, $k4:expr, $k5:expr,
         $k6:expr, $k7:expr, $k8:expr $(,)?]
    ) => {{
        (positive_coeff_u32!($k0)
            + positive_coeff_u32!($k1)
            + positive_coeff_u32!($k2)
            + positive_coeff_u32!($k3)
            + positive_coeff_u32!($k4)
            + positive_coeff_u32!($k5)
            + positive_coeff_u32!($k6)
            + positive_coeff_u32!($k7)
            + positive_coeff_u32!($k8))
            * 255
    }};
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! divide_and_pack_i16x8 {
    ($lo:expr, $hi:expr, $divisor:expr, $max_value:expr) => {{
        let zero = i16x8_splat(0);
        let lo = i16x8_max($lo, zero);
        let hi = i16x8_max($hi, zero);

        const PACK_DIVISOR: u32 = $divisor;
        if PACK_DIVISOR == 1 {
            u8x16_narrow_i16x8(lo, hi)
        } else {
            const PACK_MAX_VALUE: u32 = $max_value;
            const SHIFT: u32 =
                common::magic_u16_shift_for_range(PACK_DIVISOR, PACK_MAX_VALUE);
            const MAGIC: u32 = common::magic_u16_divisor_with_shift(PACK_DIVISOR, SHIFT);

            let lo = common::div_u16x8_by_magic_shift_simd(lo, MAGIC, SHIFT);
            let hi = common::div_u16x8_by_magic_shift_simd(hi, MAGIC, SHIFT);

            u8x16_narrow_i16x8(lo, hi)
        }
    }};
}
