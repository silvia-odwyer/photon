use crate::pipeline::Pipeline;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use std::arch::wasm32::v128;

#[allow(dead_code)]
#[derive(Clone)]
pub(super) enum PixelOp {
    GrayScale,
    Monochrome {
        r_offset: u8,
        g_offset: u8,
        b_offset: u8,
    },
    Invert,
    AlterChannels {
        r: i16,
        g: i16,
        b: i16,
    },
}

impl Pipeline {
    pub fn grayscale(mut self) -> Self {
        self.pending.push(PixelOp::GrayScale);
        self
    }

    pub fn monochrome(mut self, r_offset: u8, g_offset: u8, b_offset: u8) -> Self {
        self.pending.push(PixelOp::Monochrome {
            r_offset,
            g_offset,
            b_offset,
        });
        self
    }

    pub fn invert(mut self) -> Self {
        self.pending.push(PixelOp::Invert);
        self
    }
}

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
pub(super) fn apply_pixel_op_scalar(op: &PixelOp, r: &mut u8, g: &mut u8, b: &mut u8) {
    match op {
        PixelOp::GrayScale => {
            let avg = ((*r as u32 + *g as u32 + *b as u32) / 3) as u8;
            *r = avg;
            *g = avg;
            *b = avg;
        }
        PixelOp::Monochrome {
            r_offset,
            g_offset,
            b_offset,
        } => {
            let avg = (*r as u32 + *g as u32 + *b as u32) / 3;
            *r = (avg + *r_offset as u32).min(255) as u8;
            *g = (avg + *g_offset as u32).min(255) as u8;
            *b = (avg + *b_offset as u32).min(255) as u8;
        }
        PixelOp::Invert => {
            *r = 255 - *r;
            *g = 255 - *g;
            *b = 255 - *b;
        }
        PixelOp::AlterChannels {
            r: dr,
            g: dg,
            b: db,
        } => {
            *r = (*r as i16 + dr).clamp(0, 255) as u8;
            *g = (*g as i16 + dg).clamp(0, 255) as u8;
            *b = (*b as i16 + db).clamp(0, 255) as u8;
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
pub(super) unsafe fn invert_planes_simd(r: &mut [u8], g: &mut [u8], b: &mut [u8]) {
    use core::arch::wasm32::*;

    let simd_len = r.len() - (r.len() % 16);
    let mask = u8x16_splat(255);

    for i in (0..simd_len).step_by(16) {
        let rv = v128_load(r.as_ptr().add(i) as *const v128);
        let gv = v128_load(g.as_ptr().add(i) as *const v128);
        let bv = v128_load(b.as_ptr().add(i) as *const v128);

        v128_store(r.as_mut_ptr().add(i) as *mut v128, v128_xor(rv, mask));
        v128_store(g.as_mut_ptr().add(i) as *mut v128, v128_xor(gv, mask));
        v128_store(b.as_mut_ptr().add(i) as *mut v128, v128_xor(bv, mask));
    }

    for i in simd_len..r.len() {
        r[i] = 255 - r[i];
        g[i] = 255 - g[i];
        b[i] = 255 - b[i];
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
pub(super) unsafe fn alter_channels_planes_simd(
    r: &mut [u8],
    g: &mut [u8],
    b: &mut [u8],
    r_amt: i16,
    g_amt: i16,
    b_amt: i16,
) {
    alter_plane_simd(r, r_amt);
    alter_plane_simd(g, g_amt);
    alter_plane_simd(b, b_amt);
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn alter_plane_simd(channel: &mut [u8], amt: i16) {
    use core::arch::wasm32::*;

    let simd_len = channel.len() - (channel.len() % 16);
    let magnitude = amt.unsigned_abs().min(255) as u8;
    let delta = u8x16_splat(magnitude);

    for i in (0..simd_len).step_by(16) {
        let values = v128_load(channel.as_ptr().add(i) as *const v128);
        let output = if amt >= 0 {
            u8x16_add_sat(values, delta)
        } else {
            u8x16_sub_sat(values, delta)
        };
        v128_store(channel.as_mut_ptr().add(i) as *mut v128, output);
    }

    for value in &mut channel[simd_len..] {
        *value = (*value as i16 + amt).clamp(0, 255) as u8;
    }
}

// Helper function for `grayscale_planes_simd` and `monochrome_planes_simd` below
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
unsafe fn average_rgb_planes_simd(r: v128, g: v128, b: v128) -> v128 {
    use core::arch::wasm32::*;

    // (r+g+b)/3 needs 32-bit lanes (sum up to 765), so each 16-lane u8 chunk
    // is split into 4 groups of 4 lanes via u8->u16->u32 zero-extension.
    let div3 = u32x4_splat(21846); // ceil(2^16 / 3), exact for sums 0..765

    let r16_lo = u16x8_extend_low_u8x16(r);
    let r16_hi = u16x8_extend_high_u8x16(r);
    let g16_lo = u16x8_extend_low_u8x16(g);
    let g16_hi = u16x8_extend_high_u8x16(g);
    let b16_lo = u16x8_extend_low_u8x16(b);
    let b16_hi = u16x8_extend_high_u8x16(b);

    let avg32 = |rv, gv, bv| {
        let sum = i32x4_add(i32x4_add(rv, gv), bv);
        u32x4_shr(i32x4_mul(sum, div3), 16)
    };

    let avg_a = avg32(
        u32x4_extend_low_u16x8(r16_lo),
        u32x4_extend_low_u16x8(g16_lo),
        u32x4_extend_low_u16x8(b16_lo),
    );
    let avg_b = avg32(
        u32x4_extend_high_u16x8(r16_lo),
        u32x4_extend_high_u16x8(g16_lo),
        u32x4_extend_high_u16x8(b16_lo),
    );
    let avg_c = avg32(
        u32x4_extend_low_u16x8(r16_hi),
        u32x4_extend_low_u16x8(g16_hi),
        u32x4_extend_low_u16x8(b16_hi),
    );
    let avg_d = avg32(
        u32x4_extend_high_u16x8(r16_hi),
        u32x4_extend_high_u16x8(g16_hi),
        u32x4_extend_high_u16x8(b16_hi),
    );

    let avg16_lo = u16x8_narrow_i32x4(avg_a, avg_b);
    let avg16_hi = u16x8_narrow_i32x4(avg_c, avg_d);
    u8x16_narrow_i16x8(avg16_lo, avg16_hi)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
pub(super) unsafe fn grayscale_planes_simd(r: &mut [u8], g: &mut [u8], b: &mut [u8]) {
    use core::arch::wasm32::*;

    let simd_len = r.len() - (r.len() % 16);

    for i in (0..simd_len).step_by(16) {
        let rv = v128_load(r.as_ptr().add(i) as *const v128);
        let gv = v128_load(g.as_ptr().add(i) as *const v128);
        let bv = v128_load(b.as_ptr().add(i) as *const v128);

        let avg = average_rgb_planes_simd(rv, gv, bv);

        v128_store(r.as_mut_ptr().add(i) as *mut v128, avg);
        v128_store(g.as_mut_ptr().add(i) as *mut v128, avg);
        v128_store(b.as_mut_ptr().add(i) as *mut v128, avg);
    }

    for i in simd_len..r.len() {
        let avg = ((r[i] as u32 + g[i] as u32 + b[i] as u32) / 3) as u8;
        r[i] = avg;
        g[i] = avg;
        b[i] = avg;
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[target_feature(enable = "simd128")]
pub(super) unsafe fn monochrome_planes_simd(
    r: &mut [u8],
    g: &mut [u8],
    b: &mut [u8],
    r_offset: u8,
    g_offset: u8,
    b_offset: u8,
) {
    use core::arch::wasm32::*;

    let simd_len = r.len() - (r.len() % 16);
    let r_off = u8x16_splat(r_offset);
    let g_off = u8x16_splat(g_offset);
    let b_off = u8x16_splat(b_offset);

    for i in (0..simd_len).step_by(16) {
        let rv = v128_load(r.as_ptr().add(i) as *const v128);
        let gv = v128_load(g.as_ptr().add(i) as *const v128);
        let bv = v128_load(b.as_ptr().add(i) as *const v128);

        let avg = average_rgb_planes_simd(rv, gv, bv);

        // avg + offset, clamped to 255, matches the scalar `.min(255)`.
        v128_store(
            r.as_mut_ptr().add(i) as *mut v128,
            u8x16_add_sat(avg, r_off),
        );
        v128_store(
            g.as_mut_ptr().add(i) as *mut v128,
            u8x16_add_sat(avg, g_off),
        );
        v128_store(
            b.as_mut_ptr().add(i) as *mut v128,
            u8x16_add_sat(avg, b_off),
        );
    }

    for i in simd_len..r.len() {
        let avg = (r[i] as u32 + g[i] as u32 + b[i] as u32) / 3;
        r[i] = (avg + r_offset as u32).min(255) as u8;
        g[i] = (avg + g_offset as u32).min(255) as u8;
        b[i] = (avg + b_offset as u32).min(255) as u8;
    }
}
