use crate::pipeline::Pipeline;

#[macro_use]
pub(crate) mod common;

#[macro_use]
mod direct_3x3;
mod separable;
mod sobel_global;

use common::restore_alpha_if_filter_zeroed_it;

use crate::pipeline::spatial_ops::direct_3x3::convolve_3x3;
use crate::pipeline::spatial_ops::separable::convolve_separable_3x3;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use crate::pipeline::spatial_ops::separable::box_blur_simd::box_blur_3x3_simd;
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use direct_3x3::edge_one_simd::edge_one_simd;

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
const NOISE_REDUCTION: [f32; 9] = [0.0, -1.0, 7.0, -1.0, 5.0, 9.0, 0.0, 7.0, 9.0];
#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
const SHARPEN: [f32; 9] = [0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0];
#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
const EDGE_DETECTION: [f32; 9] = [-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
const DETECT_45_DEG_LINES: [f32; 9] =
    [-1.0, -1.0, 2.0, -1.0, 2.0, -1.0, 2.0, -1.0, -1.0];
#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
const DETECT_135_DEG_LINES: [f32; 9] =
    [2.0, -1.0, -1.0, -1.0, 2.0, -1.0, -1.0, -1.0, 2.0];
#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
const LAPLACE: [f32; 9] = [0.0, -1.0, 0.0, -1.0, 4.0, -1.0, 0.0, -1.0, 0.0];
#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
const EDGE_ONE: [f32; 9] = [0.0, -2.2, -0.6, -0.4, 2.8, -0.3, -0.8, -1.0, 2.7];

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
const EMBOSS: [f32; 9] = [-2.0, -1.0, 0.0, -1.0, 1.0, 1.0, 0.0, 1.0, 2.0];
#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
const PREWITT_HORIZONTAL: [f32; 9] = [5.0, -3.0, -3.0, 5.0, 0.0, -3.0, 5.0, -3.0, -3.0];

impl Pipeline {
    pub fn convolve_3x3(mut self, kernel: [f32; 9]) -> Self {
        self.apply_direct_3x3(kernel);
        self
    }

    pub fn convolve_separable_3x3(
        mut self,
        horizontal: [f32; 3],
        vertical: [f32; 3],
    ) -> Self {
        self.apply_separable_3x3(horizontal, vertical);
        self
    }

    pub fn box_blur(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            self.flush_pixel_ops();
            self.ensure_scratch();
            self.ensure_i16_scratch();

            let scratch = self.scratch.as_mut().unwrap();
            let i16_scratch = self.i16_scratch.as_mut().unwrap();

            box_blur_3x3_simd(&self.image, scratch, i16_scratch);
            std::mem::swap(&mut self.image, scratch);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.apply_separable_3x3([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]);
        }

        self
    }

    pub fn noise_reduction(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [0, -1, 7, -1, 5, 9, 0, 7, 9]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self = self.convolve_3x3(NOISE_REDUCTION);
        }

        self
    }

    pub fn sharpen(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [0, -1, 0, -1, 5, -1, 0, -1, 0]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self = self.convolve_3x3(SHARPEN);
        }

        self
    }

    pub fn edge_detection(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [-1, -1, -1, -1, 8, -1, -1, -1, -1]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self = self.convolve_3x3(EDGE_DETECTION);
        }

        self
    }

    pub fn identity(self) -> Self {
        self
    }

    pub fn detect_horizontal_lines(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [-1, -1, -1, 2, 2, 2, -1, -1, -1]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.apply_separable_3x3([1.0, 1.0, 1.0], [-1.0, 2.0, -1.0]);
        }

        self
    }

    pub fn detect_vertical_lines(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [-1, 2, -1, -1, 2, -1, -1, 2, -1]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.apply_separable_3x3([-1.0, 2.0, -1.0], [1.0, 1.0, 1.0]);
        }

        self
    }

    pub fn detect_45_deg_lines(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [-1, -1, 2, -1, 2, -1, 2, -1, -1]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self = self.convolve_3x3(DETECT_45_DEG_LINES);
        }

        self
    }

    pub fn detect_135_deg_lines(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [2, -1, -1, -1, 2, -1, -1, -1, 2]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self = self.convolve_3x3(DETECT_135_DEG_LINES);
        }

        self
    }

    pub fn laplace(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [0, -1, 0, -1, 4, -1, 0, -1, 0]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self = self.convolve_3x3(LAPLACE);
        }

        self
    }

    pub fn edge_one(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            self.flush_pixel_ops();
            self.ensure_scratch();

            let scratch = self.scratch.as_mut().unwrap();

            edge_one_simd(&self.image, scratch);
            std::mem::swap(&mut self.image, scratch);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self = self.convolve_3x3(EDGE_ONE);
        }

        self
    }

    pub fn emboss(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [-2, -1, 0, -1, 1, 1, 0, 1, 2]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self = self.convolve_3x3(EMBOSS);
        }

        self
    }

    pub fn sobel_horizontal(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [-1, -2, -1, 0, 0, 0, 1, 2, 1]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.apply_separable_3x3([1.0, 2.0, 1.0], [-1.0, 0.0, 1.0]);
        }

        self
    }

    pub fn prewitt_horizontal(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [5, -3, -3, 5, 0, -3, 5, -3, -3]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.apply_direct_3x3(PREWITT_HORIZONTAL);
        }

        self
    }

    pub fn sobel_vertical(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            apply_direct_3x3_simd!(self, [-1, 0, 1, -2, 0, 2, -1, 0, 1]);
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.apply_separable_3x3([-1.0, 0.0, 1.0], [1.0, 2.0, 1.0]);
        }

        self
    }

    pub fn sobel_global(mut self) -> Self {
        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            self.apply_sobel_global_simd();
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.apply_sobel_global_scalar();
        }

        self
    }

    fn apply_direct_3x3(&mut self, kernel: [f32; 9]) {
        self.flush_pixel_ops();
        self.ensure_scratch();

        let scratch = self.scratch.as_mut().unwrap();
        convolve_3x3(&self.image, scratch, kernel);
        std::mem::swap(&mut self.image, scratch);
    }

    fn apply_separable_3x3(&mut self, horizontal: [f32; 3], vertical: [f32; 3]) {
        self.flush_pixel_ops();
        self.ensure_scratch();
        self.ensure_f32_scratch();

        let scratch = self.scratch.as_mut().unwrap();
        let f32_scratch = self.f32_scratch.as_mut().unwrap();

        convolve_separable_3x3(&self.image, scratch, f32_scratch, horizontal, vertical);
        std::mem::swap(&mut self.image, scratch);
    }
}
