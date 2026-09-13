use crate::pipeline::{Pipeline, PlanarImage};

impl Pipeline {
    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    pub(crate) fn apply_sobel_global_scalar(&mut self) {
        self.flush_pixel_ops();
        self.ensure_scratch();

        let scratch = self.scratch.as_mut().unwrap();
        sobel_global_scalar(&self.image, scratch);
        std::mem::swap(&mut self.image, scratch);
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[target_feature(enable = "simd128")]
    pub(crate) unsafe fn apply_sobel_global_simd(&mut self) {
        self.flush_pixel_ops();
        self.ensure_scratch();

        let scratch = self.scratch.as_mut().unwrap();
        sobel_global_simd(&self.image, scratch);
        std::mem::swap(&mut self.image, scratch);
    }
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
fn sobel_global_scalar(src: &PlanarImage, dst: &mut PlanarImage) {
    let width = src.width() as usize;
    let height = src.height() as usize;

    dst.r.fill(0);
    dst.g.fill(0);
    dst.b.fill(0);
    dst.a.fill(0);

    sobel_global_channel_scalar(&src.r, &mut dst.r, width, height);
    sobel_global_channel_scalar(&src.g, &mut dst.g, width, height);
    sobel_global_channel_scalar(&src.b, &mut dst.b, width, height);
    sobel_global_alpha_scalar(&src.a, &mut dst.a, width, height);
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
fn sobel_global_channel_scalar(src: &[u8], dst: &mut [u8], width: usize, height: usize) {
    if width < 3 || height < 3 {
        return;
    }

    for y in 1..height - 1 {
        let top = (y - 1) * width;
        let mid = y * width;
        let bot = (y + 1) * width;

        for x in 1..width - 1 {
            dst[mid + x] = sobel_global_pixel(
                src[top + x - 1],
                src[top + x],
                src[top + x + 1],
                src[mid + x - 1],
                src[mid + x + 1],
                src[bot + x - 1],
                src[bot + x],
                src[bot + x + 1],
            );
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
fn sobel_global_alpha_scalar(src: &[u8], dst: &mut [u8], width: usize, height: usize) {
    if src
        .first()
        .is_some_and(|alpha| src.iter().all(|value| value == alpha))
    {
        restore_uniform_sobel_alpha(src, dst, width, height);
        return;
    }

    sobel_global_channel_scalar(src, dst, width, height);
}

fn restore_uniform_sobel_alpha(src: &[u8], dst: &mut [u8], width: usize, height: usize) {
    if width == 0 || height == 0 {
        return;
    }

    for y in 0..height.saturating_sub(1) {
        let row = y * width;

        for x in 0..width.saturating_sub(1) {
            let alpha = src[row + x] as u32;
            dst[row + x] = ((alpha * alpha + alpha * alpha) as f64).sqrt() as u8;
        }
    }
}

fn sobel_global_pixel(
    top_left: u8,
    top_center: u8,
    top_right: u8,
    mid_left: u8,
    mid_right: u8,
    bot_left: u8,
    bot_center: u8,
    bot_right: u8,
) -> u8 {
    let horizontal = -(top_left as i32) - 2 * top_center as i32 - top_right as i32
        + bot_left as i32
        + 2 * bot_center as i32
        + bot_right as i32;
    let vertical = -(top_left as i32) + top_right as i32 - 2 * mid_left as i32
        + 2 * mid_right as i32
        - bot_left as i32
        + bot_right as i32;

    let horizontal = horizontal.clamp(0, 255) as u32;
    let vertical = vertical.clamp(0, 255) as u32;

    ((horizontal * horizontal + vertical * vertical) as f64).sqrt() as u8
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn sobel_global_simd(src: &PlanarImage, dst: &mut PlanarImage) {
    let width = src.width() as usize;
    let height = src.height() as usize;

    dst.r.fill(0);
    dst.g.fill(0);
    dst.b.fill(0);
    dst.a.fill(0);

    sobel_global_channel_simd(&src.r, &mut dst.r, width, height);
    sobel_global_channel_simd(&src.g, &mut dst.g, width, height);
    sobel_global_channel_simd(&src.b, &mut dst.b, width, height);
    sobel_global_alpha_simd(&src.a, &mut dst.a, width, height);
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn sobel_global_channel_simd(
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
            let top_left = v128_load(src.as_ptr().add(top + x - 1) as *const v128);
            let top_center = v128_load(src.as_ptr().add(top + x) as *const v128);
            let top_right = v128_load(src.as_ptr().add(top + x + 1) as *const v128);
            let mid_left = v128_load(src.as_ptr().add(mid + x - 1) as *const v128);
            let mid_right = v128_load(src.as_ptr().add(mid + x + 1) as *const v128);
            let bot_left = v128_load(src.as_ptr().add(bot + x - 1) as *const v128);
            let bot_center = v128_load(src.as_ptr().add(bot + x) as *const v128);
            let bot_right = v128_load(src.as_ptr().add(bot + x + 1) as *const v128);

            let horizontal_lo = sobel_horizontal_lanes(
                i16x8_extend_low_u8x16(top_left),
                i16x8_extend_low_u8x16(top_center),
                i16x8_extend_low_u8x16(top_right),
                i16x8_extend_low_u8x16(bot_left),
                i16x8_extend_low_u8x16(bot_center),
                i16x8_extend_low_u8x16(bot_right),
            );
            let horizontal_hi = sobel_horizontal_lanes(
                i16x8_extend_high_u8x16(top_left),
                i16x8_extend_high_u8x16(top_center),
                i16x8_extend_high_u8x16(top_right),
                i16x8_extend_high_u8x16(bot_left),
                i16x8_extend_high_u8x16(bot_center),
                i16x8_extend_high_u8x16(bot_right),
            );
            let vertical_lo = sobel_vertical_lanes(
                i16x8_extend_low_u8x16(top_left),
                i16x8_extend_low_u8x16(top_right),
                i16x8_extend_low_u8x16(mid_left),
                i16x8_extend_low_u8x16(mid_right),
                i16x8_extend_low_u8x16(bot_left),
                i16x8_extend_low_u8x16(bot_right),
            );
            let vertical_hi = sobel_vertical_lanes(
                i16x8_extend_high_u8x16(top_left),
                i16x8_extend_high_u8x16(top_right),
                i16x8_extend_high_u8x16(mid_left),
                i16x8_extend_high_u8x16(mid_right),
                i16x8_extend_high_u8x16(bot_left),
                i16x8_extend_high_u8x16(bot_right),
            );

            let out_lo = sobel_magnitude_lanes(horizontal_lo, vertical_lo);
            let out_hi = sobel_magnitude_lanes(horizontal_hi, vertical_hi);
            let out = u8x16_narrow_i16x8(out_lo, out_hi);
            v128_store(dst.as_mut_ptr().add(mid + x) as *mut v128, out);

            x += 16;
        }

        while x < width - 1 {
            dst[mid + x] = sobel_global_pixel(
                src[top + x - 1],
                src[top + x],
                src[top + x + 1],
                src[mid + x - 1],
                src[mid + x + 1],
                src[bot + x - 1],
                src[bot + x],
                src[bot + x + 1],
            );
            x += 1;
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn sobel_global_alpha_simd(
    src: &[u8],
    dst: &mut [u8],
    width: usize,
    height: usize,
) {
    if src
        .first()
        .is_some_and(|alpha| src.iter().all(|value| value == alpha))
    {
        restore_uniform_sobel_alpha(src, dst, width, height);
        return;
    }

    sobel_global_channel_simd(src, dst, width, height);
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn sobel_horizontal_lanes(
    top_left: core::arch::wasm32::v128,
    top_center: core::arch::wasm32::v128,
    top_right: core::arch::wasm32::v128,
    bot_left: core::arch::wasm32::v128,
    bot_center: core::arch::wasm32::v128,
    bot_right: core::arch::wasm32::v128,
) -> core::arch::wasm32::v128 {
    use core::arch::wasm32::*;

    let top = i16x8_add(i16x8_add(top_left, i16x8_shl(top_center, 1)), top_right);
    let bot = i16x8_add(i16x8_add(bot_left, i16x8_shl(bot_center, 1)), bot_right);
    i16x8_sub(bot, top)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn sobel_vertical_lanes(
    top_left: core::arch::wasm32::v128,
    top_right: core::arch::wasm32::v128,
    mid_left: core::arch::wasm32::v128,
    mid_right: core::arch::wasm32::v128,
    bot_left: core::arch::wasm32::v128,
    bot_right: core::arch::wasm32::v128,
) -> core::arch::wasm32::v128 {
    use core::arch::wasm32::*;

    let left = i16x8_add(i16x8_add(top_left, i16x8_shl(mid_left, 1)), bot_left);
    let right = i16x8_add(i16x8_add(top_right, i16x8_shl(mid_right, 1)), bot_right);
    i16x8_sub(right, left)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn sobel_magnitude_lanes(
    horizontal: core::arch::wasm32::v128,
    vertical: core::arch::wasm32::v128,
) -> core::arch::wasm32::v128 {
    use core::arch::wasm32::*;

    let zero = i16x8_splat(0);
    let max = i16x8_splat(255);
    let horizontal = i16x8_min(i16x8_max(horizontal, zero), max);
    let vertical = i16x8_min(i16x8_max(vertical, zero), max);

    let out_lo = sobel_magnitude_i32x4(
        u32x4_extend_low_u16x8(horizontal),
        u32x4_extend_low_u16x8(vertical),
    );
    let out_hi = sobel_magnitude_i32x4(
        u32x4_extend_high_u16x8(horizontal),
        u32x4_extend_high_u16x8(vertical),
    );

    u16x8_narrow_i32x4(out_lo, out_hi)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn sobel_magnitude_i32x4(
    horizontal: core::arch::wasm32::v128,
    vertical: core::arch::wasm32::v128,
) -> core::arch::wasm32::v128 {
    use core::arch::wasm32::*;

    let horizontal = f32x4_convert_u32x4(horizontal);
    let vertical = f32x4_convert_u32x4(vertical);
    let magnitude = f32x4_sqrt(f32x4_add(
        f32x4_mul(horizontal, horizontal),
        f32x4_mul(vertical, vertical),
    ));

    u32x4_trunc_sat_f32x4(magnitude)
}
