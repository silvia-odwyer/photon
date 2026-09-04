#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! kernel_3x3_i16x8 {
    (
        [$($kernel:tt)+],
        [$($pixel:expr),+ $(,)?]
    ) => {{
        let mut acc = i16x8_splat(0);
        kernel_3x3_i16x8_impl!(acc; [$($kernel)+,]; [$($pixel,)+]);
        acc
    }};
}

// Recursive function which skips all zero values in the kernel
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
macro_rules! kernel_3x3_i16x8_impl {
    ($acc:ident; []; []) => {};
    ($acc:ident; [0, $($kernel:tt)*]; [$pixel:expr, $($pixels:expr,)*]) => {
        kernel_3x3_i16x8_impl!($acc; [$($kernel)*]; [$($pixels,)*]);
    };
    ($acc:ident; [-$k:literal, $($kernel:tt)*]; [$pixel:expr, $($pixels:expr,)*]) => {
        add_coeff_i16x8!($acc, $pixel, -$k);
        kernel_3x3_i16x8_impl!($acc; [$($kernel)*]; [$($pixels,)*]);
    };
    ($acc:ident; [$k:literal, $($kernel:tt)*]; [$pixel:expr, $($pixels:expr,)*]) => {
        add_coeff_i16x8!($acc, $pixel, $k);
        kernel_3x3_i16x8_impl!($acc; [$($kernel)*]; [$($pixels,)*]);
    };
}
