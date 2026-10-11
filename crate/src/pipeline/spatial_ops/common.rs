use crate::pipeline::PlanarImage;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub(super) const fn magic_u16_divisor(divisor: u32) -> u32 {
    ((1u32 << 16) + divisor - 1) / divisor
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub(super) const fn magic_u16_divisor_with_shift(divisor: u32, shift: u32) -> u32 {
    ((1u32 << shift) + divisor - 1) / divisor
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
pub(super) const fn magic_u16_shift_for_range(divisor: u32, max_value: u32) -> u32 {
    let mut shift = 16;

    while shift < 32 {
        let magic = magic_u16_divisor_with_shift(divisor, shift);

        if max_value.saturating_mul(magic) <= u32::MAX
            && magic_u16_division_is_exact(divisor, max_value, magic, shift)
        {
            return shift;
        }

        shift += 1;
    }

    31
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
const fn magic_u16_division_is_exact(
    divisor: u32,
    max_value: u32,
    magic: u32,
    shift: u32,
) -> bool {
    let mut value = 0;

    while value <= max_value {
        if ((value * magic) >> shift) != value / divisor {
            return false;
        }

        value += 1;
    }

    true
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! div_const_u16x8_simd {
    ($values:expr, $divisor:literal) => {{
        const MAGIC: u32 =
            crate::pipeline::spatial_ops::common::magic_u16_divisor($divisor);
        crate::pipeline::spatial_ops::common::div_u16x8_by_magic_simd($values, MAGIC)
    }};
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
pub(super) unsafe fn div_u16x8_by_magic_simd(
    values: core::arch::wasm32::v128,
    magic: u32,
) -> core::arch::wasm32::v128 {
    div_u16x8_by_magic_shift_simd(values, magic, 16)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
pub(super) unsafe fn div_u16x8_by_magic_shift_simd(
    values: core::arch::wasm32::v128,
    magic: u32,
    shift: u32,
) -> core::arch::wasm32::v128 {
    use core::arch::wasm32::*;

    // For bounded unsigned values we can replace division by a constant with a
    // fixed-point multiply: floor(x / d) == (x * ceil(2^shift / d)) >> shift.
    // The caller must only use this for value ranges where that identity has been checked.
    let magic = u32x4_splat(magic);

    // wasm SIMD has no u16x8 multiply that gives the high half of the product,
    // so widen to u32x4, multiply there, shift, then narrow back to u16x8.
    let lo = u32x4_extend_low_u16x8(values);
    let hi = u32x4_extend_high_u16x8(values);
    let div_lo = u32x4_shr(u32x4_mul(lo, magic), shift);
    let div_hi = u32x4_shr(u32x4_mul(hi, magic), shift);

    u16x8_narrow_i32x4(div_lo, div_hi)
}

/// We have added this function to stay compatible with the library
pub(super) fn restore_alpha_if_filter_zeroed_it(
    src: &PlanarImage,
    dst: &mut PlanarImage,
    width: usize,
    height: usize,
) {
    if !dst.a.iter().all(|alpha| *alpha == 0) {
        return;
    }

    for y in 0..height.saturating_sub(1) {
        let row = y * width;

        for x in 0..width.saturating_sub(1) {
            dst.a[row + x] = src.a[row + x];
        }
    }
}
