use super::pixel_ops::PixelOp;
use super::planar::PlanarImage;
use crate::PhotonImage;

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
use super::pixel_ops::apply_pixel_op_scalar;

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use super::pixel_ops::{
    alter_channels_planes_simd, grayscale_planes_simd, invert_planes_simd,
    monochrome_planes_simd,
};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub struct Pipeline {
    pub(super) image: PlanarImage,
    pub(super) scratch: Option<PlanarImage>,
    pub(super) f32_scratch: Option<Vec<f32>>,
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    pub(super) i16_scratch: Option<Vec<i16>>,
    pub(super) pending: Vec<PixelOp>,
}

impl Pipeline {
    pub fn from_photon_image(img: &PhotonImage) -> Self {
        let planar_image = PlanarImage::from_photon_image(img);
        Pipeline {
            image: planar_image,
            scratch: None,
            f32_scratch: None,
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            i16_scratch: None,
            pending: Vec::new(),
        }
    }

    pub fn from_planar_image(img: PlanarImage) -> Self {
        Pipeline {
            image: img,
            scratch: None,
            f32_scratch: None,
            #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
            i16_scratch: None,
            pending: Vec::new(),
        }
    }

    // This function assumes that the dimensions of the image never change inside the pipeline.
    // If we want to be able to change the dimensions of the image inside the pipeline,
    // then this function must be changed to validate whether the size is still the same for the scratch image
    pub(super) fn ensure_scratch(&mut self) {
        if !self.scratch.is_none() {
            return;
        }

        self.scratch = Some(PlanarImage::empty_like(&self.image))
    }

    pub(super) fn ensure_f32_scratch(&mut self) {
        if self.f32_scratch.is_none() {
            self.f32_scratch = Some(vec![0.0; self.image.len()]);
        }
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    pub(super) fn ensure_i16_scratch(&mut self) {
        if self.i16_scratch.is_none() {
            self.i16_scratch = Some(vec![0; self.image.len()]);
        }
    }

    pub fn finish(mut self) -> PhotonImage {
        self.flush_pixel_ops();
        self.image.to_photon_image()
    }

    pub fn finish_to_planar(mut self) -> PlanarImage {
        self.flush_pixel_ops();
        self.image
    }

    pub fn alter_channels(mut self, r: i16, g: i16, b: i16) -> Self {
        self.pending.push(PixelOp::AlterChannels { r, g, b });
        self
    }

    pub fn swap_channels(mut self, channel1: usize, channel2: usize) -> Self {
        self.swap_channels_in_place(channel1, channel2);
        self
    }

    fn swap_channels_in_place(&mut self, mut channel1: usize, mut channel2: usize) {
        if channel1 > 2 {
            panic!(
                "Invalid channel index passed. Channel1 must be equal to 0, 1, or 2."
            );
        }
        if channel2 > 2 {
            panic!(
                "Invalid channel index passed. Channel2 must be equal to 0, 1, or 2."
            );
        }

        self.flush_pixel_ops();

        if channel1 == channel2 {
            return;
        }

        if channel1 > channel2 {
            std::mem::swap(&mut channel1, &mut channel2);
        }

        match (channel1, channel2) {
            (0, 1) => std::mem::swap(&mut self.image.r, &mut self.image.g),
            (0, 2) => std::mem::swap(&mut self.image.r, &mut self.image.b),
            (1, 2) => std::mem::swap(&mut self.image.g, &mut self.image.b),
            _ => unreachable!(),
        }
    }

    pub(super) fn flush_pixel_ops(&mut self) {
        if self.pending.is_empty() {
            return;
        }

        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            self.flush_pixel_ops_simd();
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.flush_pixel_ops_scalar();
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    fn flush_pixel_ops_scalar(&mut self) {
        for i in 0..self.image.r.len() {
            let mut r = self.image.r[i];
            let mut g = self.image.g[i];
            let mut b = self.image.b[i];

            for op in &self.pending {
                apply_pixel_op_scalar(op, &mut r, &mut g, &mut b);
            }

            self.image.r[i] = r;
            self.image.g[i] = g;
            self.image.b[i] = b;
        }

        self.pending.clear();
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[target_feature(enable = "simd128")]
    unsafe fn flush_pixel_ops_simd(&mut self) {
        for op in &self.pending {
            match op {
                PixelOp::Invert => invert_planes_simd(
                    &mut self.image.r,
                    &mut self.image.g,
                    &mut self.image.b,
                ),
                PixelOp::AlterChannels { r, g, b } => alter_channels_planes_simd(
                    &mut self.image.r,
                    &mut self.image.g,
                    &mut self.image.b,
                    *r,
                    *g,
                    *b,
                ),
                PixelOp::GrayScale => grayscale_planes_simd(
                    &mut self.image.r,
                    &mut self.image.g,
                    &mut self.image.b,
                ),
                PixelOp::Monochrome {
                    r_offset,
                    g_offset,
                    b_offset,
                } => monochrome_planes_simd(
                    &mut self.image.r,
                    &mut self.image.g,
                    &mut self.image.b,
                    *r_offset,
                    *g_offset,
                    *b_offset,
                ),
            }
        }

        self.pending.clear();
    }
}
