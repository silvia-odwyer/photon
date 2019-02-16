    extern crate image;
    use image::{GenericImage, DynamicImage, ImageBuffer, GenericImageView};
    use image::Pixel;

    // In-built image filters
    pub fn oceanic(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 9, 2, 173);
        return filtered_img;
    }

    pub fn islands(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 24, 2, 95);
        return filtered_img;
    }

    pub fn marine(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 14, 2, 119);
        return filtered_img;
    }

    pub fn seagreen(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 68, 2, 62);
        return filtered_img;
    }

    pub fn flagblue(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_blue_channel(img, 131);
        return filtered_img;
    }

    pub fn diamante(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 82, 2, 87);
        return filtered_img;
    }

    pub fn liquid(mut img: DynamicImage) -> DynamicImage {
        let filtered_img = crate::channels::alter_two_channels(img, 1, 10, 2, 75);
        return filtered_img;
    }
