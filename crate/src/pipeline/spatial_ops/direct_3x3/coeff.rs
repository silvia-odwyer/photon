#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! coeff_i16x8 {
    ($v:expr, $k:expr) => {{
        const K: i16 = $k;
        match K {
            1 => $v,
            -1 => i16x8_neg($v),
            2 => i16x8_shl($v, 1),
            -2 => i16x8_neg(i16x8_shl($v, 1)),
            3 => i16x8_add(i16x8_shl($v, 1), $v),
            -3 => i16x8_neg(i16x8_add(i16x8_shl($v, 1), $v)),
            4 => i16x8_shl($v, 2),
            -4 => i16x8_neg(i16x8_shl($v, 2)),
            5 => i16x8_add(i16x8_shl($v, 2), $v),
            -5 => i16x8_neg(i16x8_add(i16x8_shl($v, 2), $v)),
            6 => i16x8_add(i16x8_shl($v, 2), i16x8_shl($v, 1)),
            -6 => i16x8_neg(i16x8_add(i16x8_shl($v, 2), i16x8_shl($v, 1))),
            7 => i16x8_sub(i16x8_shl($v, 3), $v),
            -7 => i16x8_neg(i16x8_sub(i16x8_shl($v, 3), $v)),
            8 => i16x8_shl($v, 3),
            -8 => i16x8_neg(i16x8_shl($v, 3)),
            9 => i16x8_add(i16x8_shl($v, 3), $v),
            -9 => i16x8_neg(i16x8_add(i16x8_shl($v, 3), $v)),
            _ => i16x8_mul($v, i16x8_splat(K)),
        }
    }};
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! add_coeff_i16x8 {
    ($acc:ident, $v:expr, 0) => {};
    ($acc:ident, $v:expr, $k:expr) => {
        $acc = i16x8_add($acc, coeff_i16x8!($v, $k));
    };
}
