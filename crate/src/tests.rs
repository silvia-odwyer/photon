#[cfg(test)]
mod test {

    use image::ImageBuffer;

    use crate::adjustments::*;
    use crate::channels::*;
    use crate::colour_spaces::*;
    use crate::corrections::*;
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
            // Downsample width and upsample height.
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
            // Upsample width and downsample height.
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
        apply_color_grading(&mut img, 200.0, 50.0, -20.0, 0.0, 30.0, 10.0, 30.0, 40.0, 15.0, 50.0, 0.0);
        
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
            150, 50, 150, 255,  // Purple pixel (high R and B, low G)
            50, 150, 50, 255,   // Green pixel (high G, low R and B)
            131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130,
            255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126,
            130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138,
            120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255,
            125, 144, 120, 255,
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
}
