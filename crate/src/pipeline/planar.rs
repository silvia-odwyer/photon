use crate::PhotonImage;
use serde::{Deserialize, Serialize};

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanarImage {
    pub(super) r: Vec<u8>,
    pub(super) g: Vec<u8>,
    pub(super) b: Vec<u8>,
    pub(super) a: Vec<u8>,
    width: u32,
    height: u32,
}

impl PlanarImage {
    pub(super) fn len(&self) -> usize {
        self.r.len()
    }

    pub(super) fn width(&self) -> u32 {
        self.width
    }

    pub(super) fn height(&self) -> u32 {
        self.height
    }

    pub(super) fn empty_like(other: &PlanarImage) -> Self {
        let len = other.len();
        PlanarImage {
            width: other.width,
            height: other.height,
            r: vec![0; len],
            g: vec![0; len],
            b: vec![0; len],
            a: vec![0; len],
        }
    }

    pub fn from_photon_image(img: &PhotonImage) -> PlanarImage {
        let pixels = (img.width * img.height) as usize;

        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            PlanarImage::from_photon_image_simd(img, pixels)
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            PlanarImage::from_photon_image_scalar(img, pixels)
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    fn from_photon_image_scalar(img: &PhotonImage, pixels: usize) -> PlanarImage {
        let mut r = vec![0; pixels];
        let mut g = vec![0; pixels];
        let mut b = vec![0; pixels];
        let mut a = vec![0; pixels];

        for (i, px) in img.raw_pixels.chunks_exact(4).enumerate() {
            r[i] = px[0];
            g[i] = px[1];
            b[i] = px[2];
            a[i] = px[3];
        }

        PlanarImage {
            r,
            g,
            b,
            a,
            width: img.width,
            height: img.height,
        }
    }

    pub fn to_photon_image(&self) -> PhotonImage {
        let pixels = (self.width * self.height) as usize;

        #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
        unsafe {
            self.to_photon_image_simd(pixels)
        }

        #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
        {
            self.to_photon_image_scalar(pixels)
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    fn to_photon_image_scalar(&self, pixels: usize) -> PhotonImage {
        let mut raw_pixels = vec![0; pixels * 4];

        for i in 0..pixels {
            let base = i * 4;
            raw_pixels[base] = self.r[i];
            raw_pixels[base + 1] = self.g[i];
            raw_pixels[base + 2] = self.b[i];
            raw_pixels[base + 3] = self.a[i];
        }

        PhotonImage {
            raw_pixels,
            width: self.width,
            height: self.height,
        }
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[target_feature(enable = "simd128")]
    unsafe fn from_photon_image_simd(img: &PhotonImage, pixels: usize) -> PlanarImage {
        use core::arch::wasm32::*;

        let mut r = vec![0; pixels];
        let mut g = vec![0; pixels];
        let mut b = vec![0; pixels];
        let mut a = vec![0; pixels];

        let simd_pixels = pixels - (pixels % 16);

        for i in (0..simd_pixels).step_by(16) {
            let base = i * 4;
            let px0 = v128_load(img.raw_pixels.as_ptr().add(base) as *const v128);
            let px1 = v128_load(img.raw_pixels.as_ptr().add(base + 16) as *const v128);
            let px2 = v128_load(img.raw_pixels.as_ptr().add(base + 32) as *const v128);
            let px3 = v128_load(img.raw_pixels.as_ptr().add(base + 48) as *const v128);

            // We must divide lanes first into two separate vectors
            // This is because there is no operations for shuffling more than 2 vectors at a time
            let r01 = u8x16_shuffle::<0, 4, 8, 12, 16, 20, 24, 28, 0, 0, 0, 0, 0, 0, 0, 0>(
                px0, px1,
            );
            let r23 = u8x16_shuffle::<0, 4, 8, 12, 16, 20, 24, 28, 0, 0, 0, 0, 0, 0, 0, 0>(
                px2, px3,
            );
            let g01 = u8x16_shuffle::<1, 5, 9, 13, 17, 21, 25, 29, 0, 0, 0, 0, 0, 0, 0, 0>(
                px0, px1,
            );
            let g23 = u8x16_shuffle::<1, 5, 9, 13, 17, 21, 25, 29, 0, 0, 0, 0, 0, 0, 0, 0>(
                px2, px3,
            );
            let b01 =
                u8x16_shuffle::<2, 6, 10, 14, 18, 22, 26, 30, 0, 0, 0, 0, 0, 0, 0, 0>(
                    px0, px1,
                );
            let b23 =
                u8x16_shuffle::<2, 6, 10, 14, 18, 22, 26, 30, 0, 0, 0, 0, 0, 0, 0, 0>(
                    px2, px3,
                );
            let a01 =
                u8x16_shuffle::<3, 7, 11, 15, 19, 23, 27, 31, 0, 0, 0, 0, 0, 0, 0, 0>(
                    px0, px1,
                );
            let a23 =
                u8x16_shuffle::<3, 7, 11, 15, 19, 23, 27, 31, 0, 0, 0, 0, 0, 0, 0, 0>(
                    px2, px3,
                );

            // Take first 8 pixels from {r,g,b,a}01 and first 8 pixels from {r,g,b,a}23
            // to create 16 pixels for each channel
            let r_vec =
                u8x16_shuffle::<0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23>(
                    r01, r23,
                );
            let g_vec =
                u8x16_shuffle::<0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23>(
                    g01, g23,
                );
            let b_vec =
                u8x16_shuffle::<0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23>(
                    b01, b23,
                );
            let a_vec =
                u8x16_shuffle::<0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23>(
                    a01, a23,
                );

            v128_store(r.as_mut_ptr().add(i) as *mut v128, r_vec);
            v128_store(g.as_mut_ptr().add(i) as *mut v128, g_vec);
            v128_store(b.as_mut_ptr().add(i) as *mut v128, b_vec);
            v128_store(a.as_mut_ptr().add(i) as *mut v128, a_vec);
        }

        // Scalar tail
        for (offset, px) in img.raw_pixels[simd_pixels * 4..]
            .chunks_exact(4)
            .enumerate()
        {
            let i = simd_pixels + offset;
            r[i] = px[0];
            g[i] = px[1];
            b[i] = px[2];
            a[i] = px[3];
        }

        PlanarImage {
            r,
            g,
            b,
            a,
            width: img.width,
            height: img.height,
        }
    }

    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    #[target_feature(enable = "simd128")]
    unsafe fn to_photon_image_simd(&self, pixels: usize) -> PhotonImage {
        use core::arch::wasm32::*;

        let mut raw_pixels = vec![0; pixels * 4];
        let simd_pixels = pixels - (pixels % 16);

        for i in (0..simd_pixels).step_by(16) {
            let r = v128_load(self.r.as_ptr().add(i) as *const v128);
            let g = v128_load(self.g.as_ptr().add(i) as *const v128);
            let b = v128_load(self.b.as_ptr().add(i) as *const v128);
            let a = v128_load(self.a.as_ptr().add(i) as *const v128);

            #[rustfmt::skip]
            let rg_lo = u8x16_shuffle::<0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23>(r, g);
            #[rustfmt::skip]
            let rg_hi =
                u8x16_shuffle::<8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31, >(r, g);

            #[rustfmt::skip]
            let ba_lo =
                u8x16_shuffle::<0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23>(b, a);

            #[rustfmt::skip]
            let ba_hi =
                u8x16_shuffle::<8, 24, 9, 25, 10, 26, 11, 27, 12, 28, 13, 29, 14, 30, 15, 31, >(b, a);

            #[rustfmt::skip]
            let out0 = u8x16_shuffle::<0, 1, 16, 17, 2, 3, 18, 19, 4, 5, 20, 21, 6, 7, 22, 23>(rg_lo, ba_lo);

            #[rustfmt::skip]
            let out1 =
                u8x16_shuffle::<8, 9, 24, 25, 10, 11, 26, 27, 12, 13, 28, 29, 14, 15, 30, 31>(rg_lo, ba_lo);
            #[rustfmt::skip]
            let out2 =
                u8x16_shuffle::<0, 1, 16, 17, 2, 3, 18, 19, 4, 5, 20, 21, 6, 7, 22, 23>(rg_hi, ba_hi);
            #[rustfmt::skip]
            let out3 = u8x16_shuffle::<8, 9, 24, 25, 10, 11, 26, 27, 12, 13, 28, 29, 14, 15, 30, 31>(rg_hi, ba_hi);

            let base = i * 4;
            v128_store(raw_pixels.as_mut_ptr().add(base) as *mut v128, out0);
            v128_store(raw_pixels.as_mut_ptr().add(base + 16) as *mut v128, out1);
            v128_store(raw_pixels.as_mut_ptr().add(base + 32) as *mut v128, out2);
            v128_store(raw_pixels.as_mut_ptr().add(base + 48) as *mut v128, out3);
        }

        // Scalar tail
        for i in simd_pixels..pixels {
            let base = i * 4;
            raw_pixels[base] = self.r[i];
            raw_pixels[base + 1] = self.g[i];
            raw_pixels[base + 2] = self.b[i];
            raw_pixels[base + 3] = self.a[i];
        }

        PhotonImage {
            raw_pixels,
            width: self.width,
            height: self.height,
        }
    }
}
