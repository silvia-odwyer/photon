#[cfg(test)]
mod test {

    use image::ImageBuffer;

    use crate::adjustments::*;
    use crate::channels::*;
    use crate::colour_spaces::*;
    use crate::corrections::*;
    use crate::effects::{bayer_dither, vignette};
    use crate::noise::film_grain;
    use crate::transform::{resample, seam_carve};
    use crate::PhotonImage;
    #[test]
    fn test_alter_red_channel() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let altered_r_channel_pix = vec![
            174, 122, 131, 255, 171, 131, 139, 255, 175, 134, 137, 255, 178, 134, 130,
            255, 166, 125, 119, 255, 171, 134, 129, 255, 177, 134, 132, 255, 170, 126,
            130, 255, 172, 125, 132, 255, 162, 142, 129, 255, 174, 135, 128, 255, 178,
            120, 125, 255, 165, 134, 110, 255, 161, 122, 137, 255, 181, 140, 141, 255,
            165, 144, 120, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        alter_red_channel(&mut photon_image, 40);
        assert_eq!(photon_image.raw_pixels, altered_r_channel_pix);
    }

    #[test]
    fn test_alter_blue_channel() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let altered_b_channel_pix = vec![
            134, 122, 171, 255, 131, 131, 179, 255, 135, 134, 177, 255, 138, 134, 170,
            255, 126, 125, 159, 255, 131, 134, 169, 255, 137, 134, 172, 255, 130, 126,
            170, 255, 132, 125, 172, 255, 122, 142, 169, 255, 134, 135, 168, 255, 138,
            120, 165, 255, 125, 134, 150, 255, 121, 122, 177, 255, 141, 140, 181, 255,
            125, 144, 160, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        alter_blue_channel(&mut photon_image, 40);
        assert_eq!(photon_image.raw_pixels, altered_b_channel_pix);
    }

    #[test]
    fn test_alter_green_channel() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let altered_g_channel_pix = vec![
            134, 162, 131, 255, 131, 171, 139, 255, 135, 174, 137, 255, 138, 174, 130,
            255, 126, 165, 119, 255, 131, 174, 129, 255, 137, 174, 132, 255, 130, 166,
            130, 255, 132, 165, 132, 255, 122, 182, 129, 255, 134, 175, 128, 255, 138,
            160, 125, 255, 125, 174, 110, 255, 121, 162, 137, 255, 141, 180, 141, 255,
            125, 184, 120, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        alter_green_channel(&mut photon_image, 40);
        assert_eq!(photon_image.raw_pixels, altered_g_channel_pix);
    }

    #[test]
    fn test_swap_blue_green_channels() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let correct_pix = vec![
            134, 131, 122, 255, 131, 139, 131, 255, 135, 137, 134, 255, 138, 130, 134,
            255, 126, 119, 125, 255, 131, 129, 134, 255, 137, 132, 134, 255, 130, 130,
            126, 255, 132, 132, 125, 255, 122, 129, 142, 255, 134, 128, 135, 255, 138,
            125, 120, 255, 125, 110, 134, 255, 121, 137, 122, 255, 141, 141, 140, 255,
            125, 120, 144, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        swap_channels(&mut photon_image, 1, 2);
        assert_eq!(photon_image.raw_pixels, correct_pix);
    }

    #[test]
    fn test_swap_blue_red_channels() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let correct_pix = vec![
            131, 122, 134, 255, 139, 131, 131, 255, 137, 134, 135, 255, 130, 134, 138,
            255, 119, 125, 126, 255, 129, 134, 131, 255, 132, 134, 137, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 129, 142, 122, 255, 128, 135, 134, 255, 125,
            120, 138, 255, 110, 134, 125, 255, 137, 122, 121, 255, 141, 140, 141, 255,
            120, 144, 125, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        swap_channels(&mut photon_image, 0, 2);
        assert_eq!(photon_image.raw_pixels, correct_pix);
    }

    #[test]
    fn test_swap_green_red_channels() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let correct_pix = vec![
            122, 134, 131, 255, 131, 131, 139, 255, 134, 135, 137, 255, 134, 138, 130,
            255, 125, 126, 119, 255, 134, 131, 129, 255, 134, 137, 132, 255, 126, 130,
            130, 255, 125, 132, 132, 255, 142, 122, 129, 255, 135, 134, 128, 255, 120,
            138, 125, 255, 134, 125, 110, 255, 122, 121, 137, 255, 140, 141, 141, 255,
            144, 125, 120, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        swap_channels(&mut photon_image, 1, 0);
        assert_eq!(photon_image.raw_pixels, correct_pix);
    }

    #[test]
    fn test_hsluv_bypass() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let correct_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        hue_rotate_hsluv(&mut photon_image, 0.0);
        let photon_result: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, photon_image.get_raw_pixels())
                .expect("Test failed");
        let correct_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, correct_pix).expect("Test failed");
        imageproc::assert_pixels_eq_within!(photon_result, correct_image, 1);
    }

    #[test]
    fn test_hsl_bypass() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let correct_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        hue_rotate_hsl(&mut photon_image, 0.0);
        let photon_result: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, photon_image.get_raw_pixels())
                .expect("Test failed");
        let correct_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, correct_pix).expect("Test failed");
        imageproc::assert_pixels_eq_within!(photon_result, correct_image, 1);
    }

    #[test]
    fn test_hsv_bypass() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let correct_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        hue_rotate_hsv(&mut photon_image, 0.0);
        let photon_result: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, photon_image.get_raw_pixels())
                .expect("Test failed");
        let correct_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, correct_pix).expect("Test failed");
        imageproc::assert_pixels_eq_within!(photon_result, correct_image, 1);
    }

    #[test]
    fn test_lch_bypass() {
        let width = 4;
        let height = 4;
        // Create an image from a vec of pixels
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let correct_pix: Vec<u8> = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let mut photon_image = PhotonImage::new(raw_pix, width, height);
        hue_rotate_lch(&mut photon_image, 0.0);
        let photon_result: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, photon_image.get_raw_pixels())
                .expect("Test failed");
        let correct_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, correct_pix).expect("Test failed");
        imageproc::assert_pixels_eq_within!(photon_result, correct_image, 1);
    }

    #[test]
    fn test_seam_carve() {
        let width = 4_u32;
        let height = 4_u32;
        // Create an image from a vec of pixels
        let raw_pix: Vec<u8> = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];

        let correct_pix: Vec<u8> = vec![
            132, 125, 132, 255, 131, 134, 129, 255, 134, 135, 128, 255, 125, 134, 110,
            255, 121, 122, 137, 255, 125, 144, 120, 255,
        ];

        let photon_image: PhotonImage = PhotonImage::new(raw_pix.clone(), width, height);
        {
            // Original image
            assert_eq!(photon_image.get_width(), width);
            assert_eq!(photon_image.get_height(), height);
        }
        {
            // Un-carved image
            // Will return the same image
            let result: PhotonImage = seam_carve(&photon_image, 100_u32, 100_u32);
            assert_eq!(result.get_width(), width);
            assert_eq!(result.get_height(), height);
            assert_eq!(result.get_raw_pixels(), raw_pix);
        }
        {
            // Carved Image, from 4x4 --> 3x2
            let new_w = 3_u32;
            let new_h = 2_u32;
            let result: PhotonImage = seam_carve(&photon_image, new_w, new_h);
            assert_eq!(result.get_width(), new_w);
            assert_eq!(result.get_height(), new_h);
            assert_eq!(result.get_raw_pixels(), correct_pix);
        }
    }

    #[test]
    fn test_resample() {
        let width = 320;
        let height = 240;
        let channels = 4;
        // Create an image from a vec of pixels
        let total_size = width * height * channels;
        let raw_pix: Vec<u8> = std::iter::repeat(127)
            .take(total_size as usize)
            .collect::<Vec<_>>();

        let photon_image: PhotonImage = PhotonImage::new(raw_pix.clone(), width, height);
        {
            // Resample to the same size.
            // Will return the same image.
            let result: PhotonImage =
                resample(&photon_image, width as usize, height as usize);
            assert_eq!(result.get_width(), width);
            assert_eq!(result.get_height(), height);
            assert_eq!(result.get_raw_pixels(), raw_pix);
        }
        {
            // Upsample width and upsample height.
            let new_w: usize = 640;
            let new_h: usize = 480;
            let channels = 4;
            let new_size = new_w * new_h * channels;
            let correct_pix: Vec<u8> =
                std::iter::repeat(127).take(new_size).collect::<Vec<_>>();
            let result: PhotonImage = resample(&photon_image, new_w, new_h);
            assert_eq!(result.get_width(), new_w as u32);
            assert_eq!(result.get_height(), new_h as u32);
            assert_eq!(result.get_raw_pixels(), correct_pix);
        }
        {
            // Downsample width and downsample height.
            let new_w: usize = 160;
            let new_h: usize = 120;
            let channels = 4;
            let new_size = new_w * new_h * channels;
            let correct_pix: Vec<u8> =
                std::iter::repeat(127).take(new_size).collect::<Vec<_>>();
            let result: PhotonImage = resample(&photon_image, new_w, new_h);
            assert_eq!(result.get_width(), new_w as u32);
            assert_eq!(result.get_height(), new_h as u32);
            assert_eq!(result.get_raw_pixels(), correct_pix);
        }
        {
            let new_w: usize = 160;
            let new_h: usize = 320;
            let channels = 4;
            let new_size = new_w * new_h * channels;
            let correct_pix: Vec<u8> =
                std::iter::repeat(127).take(new_size).collect::<Vec<_>>();
            let result: PhotonImage = resample(&photon_image, new_w, new_h);
            assert_eq!(result.get_width(), new_w as u32);
            assert_eq!(result.get_height(), new_h as u32);
            assert_eq!(result.get_raw_pixels(), correct_pix);
        }
        {
            let new_w: usize = 320;
            let new_h: usize = 120;
            let channels = 4;
            let new_size = new_w * new_h * channels;
            let correct_pix: Vec<u8> =
                std::iter::repeat(127).take(new_size).collect::<Vec<_>>();
            let result: PhotonImage = resample(&photon_image, new_w, new_h);
            assert_eq!(result.get_width(), new_w as u32);
            assert_eq!(result.get_height(), new_h as u32);
            assert_eq!(result.get_raw_pixels(), correct_pix);
        }
    }

    // Standard test image used across tests (4x4 pixels)
    fn get_test_image() -> PhotonImage {
        let width = 4;
        let height = 4;
        let raw_pix = vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
        ];
        PhotonImage::new(raw_pix, width, height)
    }

    #[test]
    fn test_apply_exposure() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_exposure(&mut img, 1.0);

        // After +1 EV exposure, pixels should be brighter
        // We can't do exact comparison due to gamma correction, but we can check it changed
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero exposure (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_exposure(&mut img2, 0.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_white_balance() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_white_balance(&mut img, 20.0, 10.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero adjustment (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_white_balance(&mut img2, 0.0, 0.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_vibrance() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_vibrance(&mut img, 30.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero vibrance (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_vibrance(&mut img2, 0.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_clarity() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_clarity(&mut img, 25.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero clarity (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_clarity(&mut img2, 0.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_texture() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_texture(&mut img, 30.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero texture (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_texture(&mut img2, 0.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_dehaze() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_dehaze(&mut img, 50.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero dehaze (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_dehaze(&mut img2, 0.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_vignette() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_vignette(&mut img, 50.0, 30.0, 50.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero strength (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_vignette(&mut img2, 0.0, 30.0, 50.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_tone_zones() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_tone_zones(&mut img, 10, 20, -10, 5);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero adjustments (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_tone_zones(&mut img2, 0, 0, 0, 0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_color_grading() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        // Use parameters that will definitely change the image (non-zero saturation and luminance)
        apply_color_grading(
            &mut img, 200.0, 50.0, -20.0, 0.0, 30.0, 10.0, 30.0, 40.0, 15.0, 50.0, 0.0,
        );

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);
    }

    #[test]
    fn test_apply_sharpening() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_sharpening(&mut img, 100.0, 1.0, 2.0, 50.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero amount (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_sharpening(&mut img2, 0.0, 1.0, 2.0, 50.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_noise_reduction() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_noise_reduction(&mut img, 40.0, 50.0, 50.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero noise reduction (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_noise_reduction(&mut img2, 0.0, 0.0, 50.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_noise_reduction_bilateral() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_noise_reduction_bilateral(&mut img, 40.0, 50.0, 50.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero noise reduction (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_noise_reduction_bilateral(&mut img2, 0.0, 0.0, 50.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_noise_reduction_wavelets() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_noise_reduction_wavelets(&mut img, 50.0, 30.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero strength (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_noise_reduction_wavelets(&mut img2, 0.0, 30.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_noise_reduction_median() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_noise_reduction_median(&mut img, 2);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero radius (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_noise_reduction_median(&mut img2, 0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_noise_reduction_nlm() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_noise_reduction_nlm(&mut img, 50.0, 3, 5);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero strength (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_noise_reduction_nlm(&mut img2, 0.0, 3, 5);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_tone_curve() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        // Create a non-linear tone curve (brightening curve) that will definitely change the image
        let mut lut = Vec::new();
        for i in 0..256 {
            // Brightening curve: output is brighter than input
            lut.push((i as f32 * 1.2).min(255.0) as u8);
        }

        apply_tone_curve(&mut img, lut);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);
    }

    #[test]
    fn test_apply_chromatic_aberration() {
        // Create an image with purple/green pixels to test chromatic aberration
        let width = 4;
        let height = 4;
        let mut raw_pix = vec![
            150, 50, 150, 255, // Purple pixel (high R and B, low G)
            50, 150, 50, 255, // Green pixel (high G, low R and B)
            131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119,
            255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125,
            132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125,
            134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255,
        ];
        let mut img = PhotonImage::new(raw_pix.clone(), width, height);
        let original_pixels = img.raw_pixels.clone();

        apply_chromatic_aberration(&mut img, 50.0, 30.0);

        // Should change the image (purple and green pixels should be corrected)
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero amounts (should not change)
        let mut img2 = PhotonImage::new(raw_pix, width, height);
        let original_pixels2 = img2.raw_pixels.clone();
        apply_chromatic_aberration(&mut img2, 0.0, 0.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_apply_lens_correction() {
        let mut img = get_test_image();
        let original_pixels = img.raw_pixels.clone();

        apply_lens_correction(&mut img, -20.0, 30.0);

        // Should change the image
        assert_ne!(img.raw_pixels, original_pixels);

        // Test zero amounts (should not change)
        let mut img2 = get_test_image();
        let original_pixels2 = img2.raw_pixels.clone();
        apply_lens_correction(&mut img2, 0.0, 0.0);
        assert_eq!(img2.raw_pixels, original_pixels2);
    }

    #[test]
    fn test_bayer_dither_depth1_binary_output() {
        let width = 8_u32;
        let height = 8_u32;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128_u8, 128_u8, 255_u8])
            .take((width * height) as usize)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width, height);
        bayer_dither(&mut img, 1, 1.0);

        let out = img.get_raw_pixels();
        for i in (0..out.len()).step_by(4) {
            assert!(
                out[i] == 0 || out[i] == 255,
                "R channel value {} is not binary at depth=1",
                out[i]
            );
            assert!(
                out[i + 1] == 0 || out[i + 1] == 255,
                "G channel value {} is not binary at depth=1",
                out[i + 1]
            );
            assert!(
                out[i + 2] == 0 || out[i + 2] == 255,
                "B channel value {} is not binary at depth=1",
                out[i + 2]
            );
        }
    }

    #[test]
    fn test_film_grain_preserves_dimensions() {
        let width = 4_u32;
        let height = 4_u32;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128_u8, 128_u8, 255_u8])
            .take((width * height) as usize)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width, height);
        film_grain(&mut img, 0.5, true, 42);

        assert_eq!(
            img.get_width(),
            width,
            "width must be unchanged after film_grain"
        );
        assert_eq!(
            img.get_height(),
            height,
            "height must be unchanged after film_grain"
        );
        assert_eq!(
            img.get_raw_pixels().len(),
            (width * height * 4) as usize,
            "pixel buffer length must be unchanged after film_grain"
        );
    }
    #[test]
    fn test_film_grain_deterministic_with_fixed_seed() {
        let width = 4_u32;
        let height = 4_u32;
        let raw_pix: Vec<u8> = std::iter::repeat([100_u8, 150_u8, 200_u8, 255_u8])
            .take((width * height) as usize)
            .flatten()
            .collect();

        let mut img_a = PhotonImage::new(raw_pix.clone(), width, height);
        let mut img_b = PhotonImage::new(raw_pix, width, height);

        film_grain(&mut img_a, 0.3, false, 12345);
        film_grain(&mut img_b, 0.3, false, 12345);

        assert_eq!(
            img_a.get_raw_pixels(),
            img_b.get_raw_pixels(),
            "film_grain must produce identical output for the same seed"
        );
    }
    #[test]
    fn test_vignette_corners_darker_than_centre() {
        let width = 6_u32;
        let height = 6_u32;
        let fill = 200_u8;
        let raw_pix: Vec<u8> = std::iter::repeat([fill, fill, fill, 255_u8])
            .take((width * height) as usize)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width, height);
        vignette(&mut img, 0.8);

        let out = img.get_raw_pixels();

        let corner_r = out[0] as i32;
        let centre_idx = (3 * width as usize + 3) * 4;
        let centre_r = out[centre_idx] as i32;

        assert!(
            corner_r < centre_r,
            "corner pixel ({}) should be darker than centre pixel ({}) after vignette",
            corner_r,
            centre_r
        );
    }

    #[test]
    fn test_cinematic_filter_with_vignette_smoke() {
        use crate::filters::cinematic;

        let width = 6_u32;
        let height = 6_u32;
        let raw_pix: Vec<u8> = std::iter::repeat([120_u8, 80_u8, 60_u8, 255_u8])
            .take((width * height) as usize)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width, height);
        cinematic(&mut img);

        assert_eq!(img.get_width(), width);
        assert_eq!(img.get_height(), height);
        assert_eq!(img.get_raw_pixels().len(), (width * height * 4) as usize);
    }
    #[test]
    fn test_cinematic_filter_smoke() {
        use crate::filters::cinematic;

        let width = 4_u32;
        let height = 4_u32;
        let raw_pix: Vec<u8> = std::iter::repeat([120_u8, 80_u8, 60_u8, 255_u8])
            .take((width * height) as usize)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width, height);
        cinematic(&mut img);

        assert_eq!(img.get_width(), width);
        assert_eq!(img.get_height(), height);
        assert_eq!(img.get_raw_pixels().len(), (width * height * 4) as usize);
    }

    // ============================================
    // P0 级别测试用例 - 已补充
    // ============================================

    #[test]
    fn test_invert() {
        let width = 4;
        let height = 1;
        let raw_pix = vec![
            255, 0, 0, 255, // 纯红 → 反色：青
            0, 255, 0, 255, // 纯绿 → 反色：品红
            0, 0, 255, 255, // 纯蓝 → 反色：黄
            255, 255, 0, 255, // 纯黄 → 反色：蓝
        ];
        let expected = vec![
            0, 255, 255, 255, 255, 0, 255, 255, 255, 255, 0, 255, 0, 0, 255, 255,
        ];

        let mut img = PhotonImage::new(raw_pix, width, height);
        invert(&mut img);
        assert_eq!(img.raw_pixels, expected);
    }

    #[test]
    fn test_grayscale() {
        let width = 2;
        let height = 2;
        let raw_pix = vec![
            255, 0, 0, 255, // 红
            0, 255, 0, 255, // 绿
            0, 0, 255, 255, // 蓝
            255, 255, 255, 255, // 白
        ];

        let mut img = PhotonImage::new(raw_pix, width, height);
        grayscale(&mut img);

        // 验证所有通道都变成相同的灰度值
        for i in (0..img.raw_pixels.len()).step_by(4) {
            let r = img.raw_pixels[i];
            let g = img.raw_pixels[i + 1];
            let b = img.raw_pixels[i + 2];
            assert_eq!(r, g, "R and G channel should be equal for grayscale");
            assert_eq!(g, b, "G and B channel should be equal for grayscale");
        }
    }

    #[test]
    fn test_grayscale_human_corrected() {
        let width = 1;
        let height = 1;
        let raw_pix = vec![100, 100, 100, 255];

        let mut img = PhotonImage::new(raw_pix, width, height);
        grayscale_human_corrected(&mut img);

        // 验证灰度值使用正确的系数: 0.3*R + 0.59*G + 0.11*B
        let expected_gray = (100.0 * 0.3 + 100.0 * 0.59 + 100.0 * 0.11) as u8;
        assert_eq!(img.raw_pixels[0], expected_gray);
    }

    #[test]
    fn test_threshold() {
        let width = 4;
        let height = 1;
        let raw_pix = vec![
            50, 100, 150, 255, // 平均 ~100 < 128 → 0
            200, 50, 100, 255, // 平均 ~117 < 128 → 0
            10, 20, 30, 255, // 平均 ~20 < 128 → 0
            240, 250, 255, 255, // 平均 ~248 > 128 → 255
        ];

        let mut img = PhotonImage::new(raw_pix, width, height);
        threshold(&mut img, 128);

        // 前三个像素应变为黑色 (0)，最后一个应变为白色 (255)
        assert_eq!(img.raw_pixels[0], 0);
        assert_eq!(img.raw_pixels[4], 0);
        assert_eq!(img.raw_pixels[8], 0);
        assert_eq!(img.raw_pixels[12], 255);
    }

    #[test]
    fn test_sepia() {
        let width = 1;
        let height = 1;
        let raw_pix = vec![128, 128, 128, 255]; // 中灰

        let mut img = PhotonImage::new(raw_pix, width, height);
        sepia(&mut img);

        // 棕褐算法: new_r = avg + 100, new_g = avg + 50, new_b = avg
        let avg = 128.0;
        let expected_r = if avg as u32 + 100 < 255 {
            (avg + 100.0) as u8
        } else {
            255
        };
        let expected_g = if avg as u32 + 50 < 255 {
            (avg + 50.0) as u8
        } else {
            255
        };
        assert_eq!(img.raw_pixels[0], expected_r);
        assert_eq!(img.raw_pixels[1], expected_g);
        assert_eq!(img.raw_pixels[2], 128); // b 不变
    }

    #[test]
    fn test_solarize_preserves_dimensions() {
        let width = 4;
        let height = 4;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        solarize(&mut img);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_dec_brightness_preserves_dimensions() {
        let width = 4;
        let height = 4;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        dec_brightness(&mut img, 30);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_inc_brightness_preserves_dimensions() {
        let width = 4;
        let height = 4;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        monochrome(&mut img, 10, 20, 30);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_desaturate_preserves_dimensions() {
        let width = 4;
        let height = 4;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        desaturate(&mut img);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_pixelize_preserves_dimensions() {
        let width = 8;
        let height = 8;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        pixelize(&mut img, 4);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_normalize_min_max_tracking() {
        let width = 2;
        let height = 2;
        let raw_pix = vec![
            50, 50, 50, 255, 200, 200, 200, 255, 100, 100, 100, 255, 150, 150, 150, 255,
        ];

        let mut img = PhotonImage::new(raw_pix, width, height);
        normalize(&mut img);

        // 最暗的(50)应该变成0，最亮的(200)应该变成255
        assert_eq!(img.raw_pixels[0], 0); // min = 50 → 0
        assert_eq!(img.raw_pixels[4], 255); // max = 200 → 255
    }

    #[test]
    fn test_adjust_contrast_preserves_dimensions() {
        let width = 4;
        let height = 1;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        adjust_contrast(&mut img, 30.0);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_tint_preserves_dimensions() {
        let width = 4;
        let height = 1;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        tint(&mut img, 10, 20, 30);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_halftone_preserves_dimensions() {
        let width = 8;
        let height = 8;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        halftone(&mut img);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_colorize_preserves_dimensions() {
        let width = 4;
        let height = 4;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        colorize(&mut img);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_offset_red_preserves_dimensions() {
        let width = 10;
        let height = 10;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        offset_red(&mut img, 2);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
    }

    #[test]
    fn test_multiple_offsets_preserves_dimensions() {
        let width = 10;
        let height = 10;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        multiple_offsets(&mut img, 2, 0, 2);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
    }

    #[test]
    fn test_frosted_glass_preserves_dimensions() {
        let width = 8;
        let height = 8;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        frosted_glass(&mut img);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    // ============================================
    // P1 级别测试用例 - 已补充
    // ============================================

    #[test]
    fn test_sobel_horizontal_preserves_dimensions() {
        let width = 3;
        let height = 3;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        sobel_horizontal(&mut img);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
    }

    #[test]
    fn test_sobel_vertical_preserves_dimensions() {
        let width = 3;
        let height = 3;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        sobel_vertical(&mut img);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
    }

    #[test]
    fn test_gaussian_blur_preserves_dimensions() {
        let width = 5;
        let height = 5;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        gaussian_blur(&mut img, 2);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_mix_with_colour_alpha_preserved() {
        let width = 1;
        let height = 1;
        let raw_pix = vec![128, 128, 128, 128]; // 半透明

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        let mix_color = Rgb::new(255, 0, 0);
        mix_with_colour(&mut img, mix_color, 0.5);

        // Alpha 通道应该保持不变
        assert_eq!(img.raw_pixels[3], 128);
    }

    #[test]
    fn test_gamma_correction_preserves_dimensions() {
        let width = 4;
        let height = 4;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        gamma_correction(&mut img, 2.2, 2.2, 2.2);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_primary_preserves_dimensions() {
        let width = 4;
        let height = 4;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        primary(&mut img);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_dither_preserves_dimensions() {
        let width = 4;
        let height = 4;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        dither(&mut img, 2);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_oil_preserves_dimensions() {
        let width = 8;
        let height = 8;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        oil(&mut img, 2, 50.0);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_duotone_preserves_dimensions() {
        let width = 4;
        let height = 4;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        let color_a = Rgb::new(0, 0, 0);
        let color_b = Rgb::new(255, 255, 255);
        duotone(&mut img, color_a, color_b);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_vertical_strips_preserves_dimensions() {
        let width = 8;
        let height = 8;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        vertical_strips(&mut img, 4);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_horizontal_strips_preserves_dimensions() {
        let width = 8;
        let height = 8;
        let raw_pix: Vec<u8> = std::iter::repeat([128_u8, 128, 128, 255])
            .take(width * height)
            .flatten()
            .collect();

        let mut img = PhotonImage::new(raw_pix, width as u32, height as u32);
        horizontal_strips(&mut img, 4);

        assert_eq!(img.get_width(), width as u32);
        assert_eq!(img.get_height(), height as u32);
        assert_eq!(img.raw_pixels.len(), (width * height * 4) as usize);
    }

    #[test]
    fn test_lens_correction_ignores_degenerate_images() {
        for (width, height) in [(0, 0), (1, 1), (1, 2), (2, 1)] {
            let raw_pixels = vec![128; width * height * 4];
            let original = raw_pixels.clone();
            let mut img = PhotonImage::new(raw_pixels, width as u32, height as u32);

            apply_lens_correction(&mut img, 10.0, 10.0);

            assert_eq!(img.raw_pixels, original);
        }
    }

    // ============================================
    // 补充导入缺失的函数
    // ============================================
    use crate::channels::invert;
    use crate::colour_spaces::{gamma_correction, mix_with_colour};
    use crate::conv::{gaussian_blur, sobel_horizontal, sobel_vertical};
    use crate::effects::{
        adjust_contrast, colorize, dec_brightness, dither, duotone, frosted_glass,
        halftone, horizontal_strips, inc_brightness, multiple_offsets, normalize,
        offset_red, oil, pixelize, primary, solarize, tint, vertical_strips,
    };
    use crate::monochrome::monochrome;
    use crate::monochrome::{
        desaturate, grayscale, grayscale_human_corrected, sepia, threshold,
    };
    use crate::Rgb;
}
