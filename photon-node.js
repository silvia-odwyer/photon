/* @ts-self-types="./photon-node.d.ts" */

/**
 * Provides the image's height, width, and contains the image's raw pixels.
 * For use when communicating between JS and WASM, and also natively.
 */
class PhotonImage {
    static __wrap(ptr) {
        const obj = Object.create(PhotonImage.prototype);
        obj.__wbg_ptr = ptr;
        PhotonImageFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PhotonImageFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_photonimage_free(ptr, 0);
    }
    /**
     * Convert the PhotonImage to base64.
     * @returns {string}
     */
    get_base64() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.photonimage_get_base64(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Convert the PhotonImage to raw bytes. Returns PNG.
     * @returns {Uint8Array}
     */
    get_bytes() {
        const ret = wasm.photonimage_get_bytes(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * Convert the PhotonImage to raw bytes. Returns a JPEG.
     * @param {number} quality
     * @returns {Uint8Array}
     */
    get_bytes_jpeg(quality) {
        const ret = wasm.photonimage_get_bytes_jpeg(this.__wbg_ptr, quality);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * Convert the PhotonImage to raw bytes. Returns a WEBP.
     * @returns {Uint8Array}
     */
    get_bytes_webp() {
        const ret = wasm.photonimage_get_bytes_webp(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * Calculates estimated filesize and returns number of bytes
     * @returns {bigint}
     */
    get_estimated_filesize() {
        const ret = wasm.photonimage_get_estimated_filesize(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * Get the height of the PhotonImage.
     * @returns {number}
     */
    get_height() {
        const ret = wasm.photonimage_get_height(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Convert the PhotonImage's raw pixels to JS-compatible ImageData.
     * @returns {ImageData}
     */
    get_image_data() {
        const ret = wasm.photonimage_get_image_data(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the PhotonImage's pixels as a Vec of u8s.
     * @returns {Uint8Array}
     */
    get_raw_pixels() {
        const ret = wasm.photonimage_get_raw_pixels(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * Get the width of the PhotonImage.
     * @returns {number}
     */
    get_width() {
        const ret = wasm.photonimage_get_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Create a new PhotonImage from a Vec of u8s, which represent raw pixels.
     * @param {Uint8Array} raw_pixels
     * @param {number} width
     * @param {number} height
     */
    constructor(raw_pixels, width, height) {
        const ptr0 = passArray8ToWasm0(raw_pixels, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.photonimage_new(ptr0, len0, width, height);
        this.__wbg_ptr = ret;
        PhotonImageFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Create a new PhotonImage from a base64 string.
     * @param {string} base64
     * @returns {PhotonImage}
     */
    static new_from_base64(base64) {
        const ptr0 = passStringToWasm0(base64, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.photonimage_new_from_base64(ptr0, len0);
        return PhotonImage.__wrap(ret);
    }
    /**
     * Create a new PhotonImage from a Blob/File.
     * @param {Blob} blob
     * @returns {PhotonImage}
     */
    static new_from_blob(blob) {
        const ret = wasm.photonimage_new_from_blob(blob);
        return PhotonImage.__wrap(ret);
    }
    /**
     * Create a new PhotonImage from a byteslice.
     * @param {Uint8Array} vec
     * @returns {PhotonImage}
     */
    static new_from_byteslice(vec) {
        const ptr0 = passArray8ToWasm0(vec, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.photonimage_new_from_byteslice(ptr0, len0);
        return PhotonImage.__wrap(ret);
    }
    /**
     * Create a new PhotonImage from a HTMLImageElement
     * @param {HTMLImageElement} image
     * @returns {PhotonImage}
     */
    static new_from_image(image) {
        const ret = wasm.photonimage_new_from_image(image);
        return PhotonImage.__wrap(ret);
    }
    /**
     * Convert ImageData to raw pixels, and update the PhotonImage's raw pixels to this.
     * @param {ImageData} img_data
     */
    set_imgdata(img_data) {
        wasm.photonimage_set_imgdata(this.__wbg_ptr, img_data);
    }
}
if (Symbol.dispose) PhotonImage.prototype[Symbol.dispose] = PhotonImage.prototype.free;
exports.PhotonImage = PhotonImage;

/**
 * RGB color type.
 */
class Rgb {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RgbFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgb_free(ptr, 0);
    }
    /**
     * Get the Blue value.
     * @returns {number}
     */
    get_blue() {
        const ret = wasm.rgb_get_blue(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Green value.
     * @returns {number}
     */
    get_green() {
        const ret = wasm.rgb_get_green(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Red value.
     * @returns {number}
     */
    get_red() {
        const ret = wasm.rgb_get_red(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create a new RGB struct.
     * @param {number} r
     * @param {number} g
     * @param {number} b
     */
    constructor(r, g, b) {
        const ret = wasm.rgb_new(r, g, b);
        this.__wbg_ptr = ret;
        RgbFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Set the Blue value.
     * @param {number} b
     */
    set_blue(b) {
        wasm.rgb_set_blue(this.__wbg_ptr, b);
    }
    /**
     * Get the Green value.
     * @param {number} g
     */
    set_green(g) {
        wasm.rgb_set_green(this.__wbg_ptr, g);
    }
    /**
     * Set the Red value.
     * @param {number} r
     */
    set_red(r) {
        wasm.rgb_set_red(this.__wbg_ptr, r);
    }
}
if (Symbol.dispose) Rgb.prototype[Symbol.dispose] = Rgb.prototype.free;
exports.Rgb = Rgb;

/**
 * RGBA color type.
 */
class Rgba {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RgbaFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgba_free(ptr, 0);
    }
    /**
     * Get the alpha value for this color.
     * @returns {number}
     */
    get_alpha() {
        const ret = wasm.rgba_get_alpha(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Blue value.
     * @returns {number}
     */
    get_blue() {
        const ret = wasm.rgba_get_blue(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Green value.
     * @returns {number}
     */
    get_green() {
        const ret = wasm.rgba_get_green(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Red value.
     * @returns {number}
     */
    get_red() {
        const ret = wasm.rgba_get_red(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create a new RGBA struct.
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} a
     */
    constructor(r, g, b, a) {
        const ret = wasm.rgba_new(r, g, b, a);
        this.__wbg_ptr = ret;
        RgbaFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Set the alpha value.
     * @param {number} a
     */
    set_alpha(a) {
        wasm.rgba_set_alpha(this.__wbg_ptr, a);
    }
    /**
     * Set the Blue value.
     * @param {number} b
     */
    set_blue(b) {
        wasm.rgba_set_blue(this.__wbg_ptr, b);
    }
    /**
     * Get the Green value.
     * @param {number} g
     */
    set_green(g) {
        wasm.rgba_set_green(this.__wbg_ptr, g);
    }
    /**
     * Set the Red value.
     * @param {number} r
     */
    set_red(r) {
        wasm.rgba_set_red(this.__wbg_ptr, r);
    }
}
if (Symbol.dispose) Rgba.prototype[Symbol.dispose] = Rgba.prototype.free;
exports.Rgba = Rgba;

/**
 * @enum {1 | 2 | 3 | 4 | 5}
 */
const SamplingFilter = Object.freeze({
    Nearest: 1, "1": "Nearest",
    Triangle: 2, "2": "Triangle",
    CatmullRom: 3, "3": "CatmullRom",
    Gaussian: 4, "4": "Gaussian",
    Lanczos3: 5, "5": "Lanczos3",
});
exports.SamplingFilter = SamplingFilter;

/**
 * Add randomized noise to an image.
 * This function adds a Gaussian Noise Sample to each pixel through incrementing each channel by a randomized offset.
 * This randomized offset is generated by creating a randomized thread pool.
 * **[WASM SUPPORT IS AVAILABLE]**: Randomized thread pools cannot be created with WASM, but
 * a workaround using js_sys::Math::random works now.
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example:
 * use photon_rs::native::open_image;
 * use photon_rs::noise::add_noise_rand;
 * use photon_rs::PhotonImage;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * add_noise_rand(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function add_noise_rand(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.add_noise_rand(photon_image.__wbg_ptr);
}
exports.add_noise_rand = add_noise_rand;

/**
 * Adjust the brightness of an image by a factor.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `brightness` - A u8 to add or subtract to the brightness. To increase
 * the brightness, pass a positive number (up to 255). To decrease the brightness,
 * pass a negative number instead.
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::adjust_brightness;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * adjust_brightness(&mut img, 10_i16);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} brightness
 */
function adjust_brightness(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.adjust_brightness(photon_image.__wbg_ptr, brightness);
}
exports.adjust_brightness = adjust_brightness;

/**
 * Adjust the contrast of an image by a factor.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `contrast` - An f32 factor used to adjust contrast. Between [-255.0, 255.0]. The algorithm will
 * clamp results if passed factor is out of range.
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::adjust_contrast;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * adjust_contrast(&mut img, 30_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} contrast
 */
function adjust_contrast(photon_image, contrast) {
    _assertClass(photon_image, PhotonImage);
    wasm.adjust_contrast(photon_image.__wbg_ptr, contrast);
}
exports.adjust_contrast = adjust_contrast;

/**
 * Increment or decrement every pixel's Blue channel by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `amt` - The amount to increment or decrement the channel's value by for that pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the Blue channel for all pixels by 10:
 * use photon_rs::channels::alter_blue_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_blue_channel(&mut img, 10_i16);
 * ```
 * @param {PhotonImage} img
 * @param {number} amt
 */
function alter_blue_channel(img, amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_blue_channel(img.__wbg_ptr, amt);
}
exports.alter_blue_channel = alter_blue_channel;

/**
 * Alter a select channel by incrementing or decrementing its value by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `channel` - The channel you wish to alter, it should be either 0, 1 or 2,
 * representing R, G, or B respectively. (O=Red, 1=Green, 2=Blue)
 * * `amount` - The amount to increment/decrement the channel's value by for that pixel.
 * A positive value will increment/decrement the channel's value, a negative value will decrement the channel's value.
 *
 * ## Example
 *
 * ```no_run
 * // For example, to increase the Red channel for all pixels by 10:
 * use photon_rs::channels::alter_channel;
 * use photon_rs::native::{open_image};
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_channel(&mut img, 0_usize, 10_i16);
 * ```
 *
 * Adds a constant to a select R, G, or B channel's value.
 *
 * ### Decrease a channel's value
 * // For example, to decrease the Green channel for all pixels by 20:
 * ```no_run
 * use photon_rs::channels::alter_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_channel(&mut img, 1_usize, -20_i16);
 * ```
 * **Note**: Note the use of a minus symbol when decreasing the channel.
 * @param {PhotonImage} img
 * @param {number} channel
 * @param {number} amt
 */
function alter_channel(img, channel, amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_channel(img.__wbg_ptr, channel, amt);
}
exports.alter_channel = alter_channel;

/**
 * Increment all 3 channels' values by adding an amt to each channel per pixel.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `r_amt` - The amount to increment/decrement the Red channel by.
 * * `g_amt` - The amount to increment/decrement the Green channel by.
 * * `b_amt` - The amount to increment/decrement the Blue channel by.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the values of the Red channel by 10, the Green channel by 20,
 * // and the Blue channel by 50:
 * use photon_rs::channels::alter_channels;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_channels(&mut img, 10_i16, 20_i16, 50_i16);
 * ```
 * @param {PhotonImage} img
 * @param {number} r_amt
 * @param {number} g_amt
 * @param {number} b_amt
 */
function alter_channels(img, r_amt, g_amt, b_amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_channels(img.__wbg_ptr, r_amt, g_amt, b_amt);
}
exports.alter_channels = alter_channels;

/**
 * Increment or decrement every pixel's Green channel by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `amt` - The amount to increment/decrement the channel's value by for that pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the Green channel for all pixels by 20:
 * use photon_rs::channels::alter_green_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_green_channel(&mut img, 20_i16);
 * ```
 * @param {PhotonImage} img
 * @param {number} amt
 */
function alter_green_channel(img, amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_green_channel(img.__wbg_ptr, amt);
}
exports.alter_green_channel = alter_green_channel;

/**
 * Increment or decrement every pixel's Red channel by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `amt` - The amount to increment or decrement the channel's value by for that pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the Red channel for all pixels by 10:
 * use photon_rs::channels::alter_red_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_red_channel(&mut img, 10_i16);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} amt
 */
function alter_red_channel(photon_image, amt) {
    _assertClass(photon_image, PhotonImage);
    wasm.alter_red_channel(photon_image.__wbg_ptr, amt);
}
exports.alter_red_channel = alter_red_channel;

/**
 * Increment/decrement two channels' values simultaneously by adding an amt to each channel per pixel.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `channel1` - A usize from 0 to 2 that represents either the R, G or B channels.
 * * `amt1` - The amount to increment/decrement the channel's value by for that pixel.
 * * `channel2` -A usize from 0 to 2 that represents either the R, G or B channels.
 * * `amt2` - The amount to increment/decrement the channel's value by for that pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the values of the Red and Blue channels per pixel:
 * use photon_rs::channels::alter_two_channels;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * alter_two_channels(&mut img, 0_usize, 10_i16, 2_usize, 20_i16);
 * ```
 * @param {PhotonImage} img
 * @param {number} channel1
 * @param {number} amt1
 * @param {number} channel2
 * @param {number} amt2
 */
function alter_two_channels(img, channel1, amt1, channel2, amt2) {
    _assertClass(img, PhotonImage);
    wasm.alter_two_channels(img.__wbg_ptr, channel1, amt1, channel2, amt2);
}
exports.alter_two_channels = alter_two_channels;

/**
 * Remove chromatic aberration (purple and green fringing)
 * Common in high-contrast edges from lens imperfections
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `purple_amount` - Purple fringing removal amount (0 to 100)
 * * `green_amount` - Green fringing removal amount (0 to 100)
 *
 * # Example
 * ```no_run
 * use photon_rs::corrections::apply_chromatic_aberration;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_chromatic_aberration(&mut img, 50.0, 30.0); // Remove purple and green fringing
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} purple_amount
 * @param {number} green_amount
 */
function apply_chromatic_aberration(photon_image, purple_amount, green_amount) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_chromatic_aberration(photon_image.__wbg_ptr, purple_amount, green_amount);
}
exports.apply_chromatic_aberration = apply_chromatic_aberration;

/**
 * Apply clarity (local contrast / midtone contrast)
 * Uses unsharp mask with large radius on luminance channel
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `amount` - Clarity amount (-100 to 100)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_clarity;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_clarity(&mut img, 25.0); // Increase clarity
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} amount
 */
function apply_clarity(photon_image, amount) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_clarity(photon_image.__wbg_ptr, amount);
}
exports.apply_clarity = apply_clarity;

/**
 * Apply 3-way color grading (shadows/midtones/highlights)
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `shadow_hue` - Shadow hue adjustment (0-360 degrees)
 * * `shadow_sat` - Shadow saturation adjustment (-100 to 100)
 * * `shadow_lum` - Shadow luminance adjustment (-100 to 100)
 * * `midtone_hue` - Midtone hue adjustment (0-360 degrees)
 * * `midtone_sat` - Midtone saturation adjustment (-100 to 100)
 * * `midtone_lum` - Midtone luminance adjustment (-100 to 100)
 * * `highlight_hue` - Highlight hue adjustment (0-360 degrees)
 * * `highlight_sat` - Highlight saturation adjustment (-100 to 100)
 * * `highlight_lum` - Highlight luminance adjustment (-100 to 100)
 * * `blending` - Blending factor (0 to 100)
 * * `balance` - Balance between shadows and highlights (-100 to 100)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_color_grading;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_color_grading(&mut img, 200.0, 20.0, -10.0, 0.0, 0.0, 0.0, 30.0, 15.0, 5.0, 50.0, 0.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} shadow_hue
 * @param {number} shadow_sat
 * @param {number} shadow_lum
 * @param {number} midtone_hue
 * @param {number} midtone_sat
 * @param {number} midtone_lum
 * @param {number} highlight_hue
 * @param {number} highlight_sat
 * @param {number} highlight_lum
 * @param {number} blending
 * @param {number} balance
 */
function apply_color_grading(photon_image, shadow_hue, shadow_sat, shadow_lum, midtone_hue, midtone_sat, midtone_lum, highlight_hue, highlight_sat, highlight_lum, blending, balance) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_color_grading(photon_image.__wbg_ptr, shadow_hue, shadow_sat, shadow_lum, midtone_hue, midtone_sat, midtone_lum, highlight_hue, highlight_sat, highlight_lum, blending, balance);
}
exports.apply_color_grading = apply_color_grading;

/**
 * Apply dehaze effect (atmospheric haze removal)
 * Uses dark channel prior algorithm for haze removal
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `amount` - Dehaze amount (-100 to 100, positive removes haze, negative adds haze)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_dehaze;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_dehaze(&mut img, 50.0); // Remove haze
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} amount
 */
function apply_dehaze(photon_image, amount) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_dehaze(photon_image.__wbg_ptr, amount);
}
exports.apply_dehaze = apply_dehaze;

/**
 * Apply exposure compensation in linear space (scene-referred)
 *
 * Exposure is applied as a multiplier in linear space before gamma encoding.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `ev` - Exposure value in EV (stops). Range: -5.0 to +5.0
 *          Each +1 EV doubles the light, -1 EV halves it
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_exposure;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_exposure(&mut img, 1.5); // Brighten by 1.5 stops
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} ev
 */
function apply_exposure(photon_image, ev) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_exposure(photon_image.__wbg_ptr, ev);
}
exports.apply_exposure = apply_exposure;

/**
 * Apply a gradient to an image.
 * @param {PhotonImage} image
 */
function apply_gradient(image) {
    _assertClass(image, PhotonImage);
    wasm.apply_gradient(image.__wbg_ptr);
}
exports.apply_gradient = apply_gradient;

/**
 * Apply lens distortion correction (barrel/pincushion) with optional vignette removal
 * Uses bilinear interpolation for smooth results
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `distortion` - Distortion amount (-100 to 100, negative for barrel, positive for pincushion)
 * * `vignette` - Vignette correction amount (0 to 100)
 *
 * # Example
 * ```no_run
 * use photon_rs::corrections::apply_lens_correction;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_lens_correction(&mut img, -20.0, 30.0); // Correct barrel distortion and vignette
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} distortion
 * @param {number} vignette
 */
function apply_lens_correction(photon_image, distortion, vignette) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_lens_correction(photon_image.__wbg_ptr, distortion, vignette);
}
exports.apply_lens_correction = apply_lens_correction;

/**
 * Apply noise reduction (default: bilateral filtering)
 *
 * Convenience function that calls bilateral filtering
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `luminance` - Luminance noise reduction (0 to 100)
 * * `color` - Color noise reduction (0 to 100)
 * * `detail` - Detail preservation (0 to 100, higher = preserve more detail)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_noise_reduction;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_noise_reduction(&mut img, 40.0, 50.0, 50.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} luminance
 * @param {number} color
 * @param {number} detail
 */
function apply_noise_reduction(photon_image, luminance, color, detail) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_noise_reduction(photon_image.__wbg_ptr, luminance, color, detail);
}
exports.apply_noise_reduction = apply_noise_reduction;

/**
 * Apply noise reduction using bilateral filtering (edge-preserving)
 *
 * Reduces noise while preserving edges
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `luminance` - Luminance noise reduction (0 to 100)
 * * `color` - Color noise reduction (0 to 100)
 * * `detail` - Detail preservation (0 to 100, higher = preserve more detail)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_noise_reduction_bilateral;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_noise_reduction_bilateral(&mut img, 40.0, 50.0, 50.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} luminance
 * @param {number} color
 * @param {number} detail
 */
function apply_noise_reduction_bilateral(photon_image, luminance, color, detail) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_noise_reduction_bilateral(photon_image.__wbg_ptr, luminance, color, detail);
}
exports.apply_noise_reduction_bilateral = apply_noise_reduction_bilateral;

/**
 * Apply noise reduction using median filter
 *
 * Effective for salt-and-pepper noise, preserves edges well
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `radius` - Filter radius (1 to 3, typically 1-2)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_noise_reduction_median;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_noise_reduction_median(&mut img, 2);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} radius
 */
function apply_noise_reduction_median(photon_image, radius) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_noise_reduction_median(photon_image.__wbg_ptr, radius);
}
exports.apply_noise_reduction_median = apply_noise_reduction_median;

/**
 * Apply noise reduction using non-local means algorithm
 *
 * Advanced algorithm that compares similar patches across the image
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `strength` - Noise reduction strength (0 to 100)
 * * `patch_size` - Patch size for comparison (3 or 5)
 * * `search_radius` - Search radius for similar patches (5 to 15)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_noise_reduction_nlm;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_noise_reduction_nlm(&mut img, 50.0, 3, 10);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} strength
 * @param {number} patch_size
 * @param {number} search_radius
 */
function apply_noise_reduction_nlm(photon_image, strength, patch_size, search_radius) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_noise_reduction_nlm(photon_image.__wbg_ptr, strength, patch_size, search_radius);
}
exports.apply_noise_reduction_nlm = apply_noise_reduction_nlm;

/**
 * Apply noise reduction using wavelet denoising
 *
 * Uses wavelet transform to separate signal from noise, then reconstructs with reduced noise
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `strength` - Noise reduction strength (0 to 100)
 * * `threshold` - Wavelet threshold (0 to 100, higher = more aggressive)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_noise_reduction_wavelets;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_noise_reduction_wavelets(&mut img, 50.0, 30.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} strength
 * @param {number} threshold
 */
function apply_noise_reduction_wavelets(photon_image, strength, threshold) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_noise_reduction_wavelets(photon_image.__wbg_ptr, strength, threshold);
}
exports.apply_noise_reduction_wavelets = apply_noise_reduction_wavelets;

/**
 * Apply unsharp mask sharpening
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `amount` - Sharpening strength (0 to 150, typical 50-150)
 * * `radius` - Edge detection radius in pixels (0.5 to 3.0, typical 0.8-1.5)
 * * `threshold` - Minimum brightness difference to sharpen (0 to 255, typical 0-5)
 * * `masking` - Edge masking (0 to 100, 0 = sharpen everywhere, 100 = edges only)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_sharpening;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_sharpening(&mut img, 100.0, 1.0, 2.0, 50.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} amount
 * @param {number} radius
 * @param {number} threshold
 * @param {number} masking
 */
function apply_sharpening(photon_image, amount, radius, threshold, masking) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_sharpening(photon_image.__wbg_ptr, amount, radius, threshold, masking);
}
exports.apply_sharpening = apply_sharpening;

/**
 * Apply texture enhancement (medium-frequency detail enhancement)
 * Similar to clarity but uses smaller radius for medium-frequency details
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `amount` - Texture amount (-100 to 100)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_texture;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_texture(&mut img, 30.0); // Increase texture
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} amount
 */
function apply_texture(photon_image, amount) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_texture(photon_image.__wbg_ptr, amount);
}
exports.apply_texture = apply_texture;

/**
 * Apply tone curve lookup table
 * Applies a tone curve to the image's luminance while preserving color relationships
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `lookup_table` - A Vec<u8> containing 256 values representing the tone curve mapping.
 *                    Each index represents an input value, and the value at that index is the output.
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_tone_curve;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * // Create a lookup table (e.g., linear curve: [0, 1, 2, ..., 255])
 * let mut lut = Vec::new();
 * for i in 0..256 {
 *     lut.push(i as u8);
 * }
 * apply_tone_curve(&mut img, lut);
 * ```
 * @param {PhotonImage} photon_image
 * @param {Uint8Array} lookup_table
 */
function apply_tone_curve(photon_image, lookup_table) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passArray8ToWasm0(lookup_table, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.apply_tone_curve(photon_image.__wbg_ptr, ptr0, len0);
}
exports.apply_tone_curve = apply_tone_curve;

/**
 * Apply tone zone adjustments (darks, shadows, highlights, whites)
 * Uses luminance-based masking
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `darks` - Darks adjustment (-100 to 100)
 * * `shadows` - Shadows adjustment (-100 to 100)
 * * `highlights` - Highlights adjustment (-100 to 100)
 * * `whites` - Whites adjustment (-100 to 100)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_tone_zones;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_tone_zones(&mut img, 10, 20, -10, 5); // Adjust tone zones
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} darks
 * @param {number} shadows
 * @param {number} highlights
 * @param {number} whites
 */
function apply_tone_zones(photon_image, darks, shadows, highlights, whites) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_tone_zones(photon_image.__wbg_ptr, darks, shadows, highlights, whites);
}
exports.apply_tone_zones = apply_tone_zones;

/**
 * Apply vibrance adjustment
 * Vibrance boosts less-saturated colors more and protects skin tones
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `amount` - Vibrance amount (-100 to 100)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_vibrance;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_vibrance(&mut img, 30.0); // Increase vibrance
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} amount
 */
function apply_vibrance(photon_image, amount) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_vibrance(photon_image.__wbg_ptr, amount);
}
exports.apply_vibrance = apply_vibrance;

/**
 * Apply vignette effect (edge darkening or lightening)
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `strength` - Vignette strength (-100 to 100, positive darkens edges, negative lightens)
 * * `radius` - Vignette radius (0 to 100, percentage of image where vignette starts)
 * * `softness` - Vignette softness (0 to 100, controls falloff curve)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_vignette;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_vignette(&mut img, 50.0, 30.0, 50.0); // Darken edges
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} strength
 * @param {number} radius
 * @param {number} softness
 */
function apply_vignette(photon_image, strength, radius, softness) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_vignette(photon_image.__wbg_ptr, strength, radius, softness);
}
exports.apply_vignette = apply_vignette;

/**
 * Apply white balance using Bradford chromatic adaptation
 *
 * Converts from the current illuminant to D65 (standard daylight).
 *
 * # Arguments
 * * `photon_image` - A PhotonImage to process
 * * `temperature` - Color temperature shift (-100 to 100, where 0 = 6500K/D65)
 *                   Negative = warmer (lower Kelvin), Positive = cooler (higher Kelvin)
 * * `tint` - Green-magenta axis (-100 to 100, where negative = green, positive = magenta)
 *
 * # Example
 * ```no_run
 * use photon_rs::adjustments::apply_white_balance;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * apply_white_balance(&mut img, -20.0, 5.0); // Warm up the image slightly with magenta tint
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} temperature
 * @param {number} tint
 */
function apply_white_balance(photon_image, temperature, tint) {
    _assertClass(photon_image, PhotonImage);
    wasm.apply_white_balance(photon_image.__wbg_ptr, temperature, tint);
}
exports.apply_white_balance = apply_white_balance;

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to the Blue channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::monochrome::b_grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * b_grayscale(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function b_grayscale(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.b_grayscale(photon_image.__wbg_ptr);
}
exports.b_grayscale = b_grayscale;

/**
 * Convert a base64 string to a PhotonImage.
 * @param {string} base64
 * @returns {PhotonImage}
 */
function base64_to_image(base64) {
    const ptr0 = passStringToWasm0(base64, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.base64_to_image(ptr0, len0);
    return PhotonImage.__wrap(ret);
}
exports.base64_to_image = base64_to_image;

/**
 * Convert a base64 string to a Vec of u8s.
 * @param {string} base64
 * @returns {Uint8Array}
 */
function base64_to_vec(base64) {
    const ptr0 = passStringToWasm0(base64, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.base64_to_vec(ptr0, len0);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}
exports.base64_to_vec = base64_to_vec;

/**
 * Apply Bayer ordered dithering to an image.
 *
 * Ordered dithering quantizes each pixel's colour channels independently by
 * comparing the channel value (plus a spatially-varying threshold from the
 * 8×8 Bayer matrix) against the nearest quantization level.  Unlike
 * Floyd-Steinberg error diffusion, every pixel is processed in isolation,
 * making this algorithm branch-free and cache-friendly.
 *
 * # Arguments
 * * `photon_image` - A mutable reference to the [`PhotonImage`] to process.
 * * `bit_depth`    - Target bits per channel (clamped to 1–8).
 *                    `1` → 2 levels (pure black/white per channel),
 *                    `4` → 16 levels, `8` → no quantization.
 * * `spread`       - Dithering spread in the range `[0.0, 1.0]`.
 *                    `1.0` is the canonical Bayer threshold; lower values
 *                    reduce the visible halftone pattern for a subtler look.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::bayer_dither;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * // 2-bit depth, full Bayer spread — strong ordered dither
 * bayer_dither(&mut img, 2, 1.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} bit_depth
 * @param {number} spread
 */
function bayer_dither(photon_image, bit_depth, spread) {
    _assertClass(photon_image, PhotonImage);
    wasm.bayer_dither(photon_image.__wbg_ptr, bit_depth, spread);
}
exports.bayer_dither = bayer_dither;

/**
 * Blend two images together.
 *
 * The `blend_mode` (3rd param) determines which blending mode to use; change this for varying effects.
 * The blend modes available include: `overlay`, `over`, `atop`, `xor`, `plus`, `multiply`, `burn`,
 * `difference`, `soft_light`, `screen`, `hard_light`, `dodge`, `exclusion`, `lighten`, `darken` (more to come)
 * NOTE: The first image must be smaller than the second image passed as params.
 * If the first image were larger than the second, then there would be overflowing pixels which would have no corresponding pixels
 * in the second image.
 * # Arguments
 * * `img` - A DynamicImage that contains a view into the image.
 * * `img2` - The 2nd DynamicImage to be blended with the first.
 * * `blend_mode` - The blending mode to use. See above for complete list of blend modes available.
 * # Example
 *
 * ```no_run
 * // For example, to blend two images with the `multiply` blend mode:
 * use photon_rs::multiple::blend;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let img2 = open_image("img2.jpg").expect("File should open");
 * blend(&mut img, &img2, "multiply");
 * ```
 * @param {PhotonImage} photon_image
 * @param {PhotonImage} photon_image2
 * @param {string} blend_mode
 */
function blend(photon_image, photon_image2, blend_mode) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(photon_image2, PhotonImage);
    const ptr0 = passStringToWasm0(blend_mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.blend(photon_image.__wbg_ptr, photon_image2.__wbg_ptr, ptr0, len0);
}
exports.blend = blend;

/**
 * Apply a box blur effect.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a box blur effect:
 * use photon_rs::conv::box_blur;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * box_blur(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function box_blur(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.box_blur(photon_image.__wbg_ptr);
}
exports.box_blur = box_blur;

/**
 * Increased contrast filter effect.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::cali;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * cali(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function cali(img) {
    _assertClass(img, PhotonImage);
    wasm.cali(img.__wbg_ptr);
}
exports.cali = cali;

/**
 * Apply a cinematic film look to an image.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::cinematic;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * cinematic(&mut img);
 * ```
 *
 * The same effect is also available through the generic dispatcher:
 *
 * ```no_run
 * use photon_rs::filters::filter;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * filter(&mut img, "cinematic");
 * ```
 * @param {PhotonImage} img
 */
function cinematic(img) {
    _assertClass(img, PhotonImage);
    wasm.cinematic(img.__wbg_ptr);
}
exports.cinematic = cinematic;

/**
 * Horizontal strips. Divide an image into a series of equal-width strips, for an artistic effect. Sepcify a color as well.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `num_strips` - The numbder of strips
 * * `color` - Color of strips.
 * # Example
 *
 * ```no_run
 * // For example, to draw blue horizontal strips on a `PhotonImage`:
 * use photon_rs::effects::color_horizontal_strips;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgb;
 *
 * let color = Rgb::new(255u8, 0u8, 0u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * color_horizontal_strips(&mut img, 8u8, color);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_strips
 * @param {Rgb} color
 */
function color_horizontal_strips(photon_image, num_strips, color) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color, Rgb);
    var ptr0 = color.__destroy_into_raw();
    wasm.color_horizontal_strips(photon_image.__wbg_ptr, num_strips, ptr0);
}
exports.color_horizontal_strips = color_horizontal_strips;

/**
 * Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect. Sepcify a color as well.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `num_strips` - The numbder of strips
 * * `color` - Color of strips.
 * # Example
 *
 * ```no_run
 * // For example, to draw red vertical strips on a `PhotonImage`:
 * use photon_rs::effects::color_vertical_strips;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgb;
 *
 * let color = Rgb::new(255u8, 0u8, 0u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * color_vertical_strips(&mut img, 8u8, color);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_strips
 * @param {Rgb} color
 */
function color_vertical_strips(photon_image, num_strips, color) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color, Rgb);
    var ptr0 = color.__destroy_into_raw();
    wasm.color_vertical_strips(photon_image.__wbg_ptr, num_strips, ptr0);
}
exports.color_vertical_strips = color_vertical_strips;

/**
 * Colorizes the green channels of the image.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to colorize an image of type `PhotonImage`:
 * use photon_rs::effects::colorize;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * colorize(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function colorize(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.colorize(photon_image.__wbg_ptr);
}
exports.colorize = colorize;

/**
 * @param {number} width
 * @param {number} height
 * @returns {PhotonImage}
 */
function create_gradient(width, height) {
    const ret = wasm.create_gradient(width, height);
    return PhotonImage.__wrap(ret);
}
exports.create_gradient = create_gradient;

/**
 * Crop an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to crop an image at (0, 0) to (500, 800)
 * use photon_rs::native::{open_image};
 * use photon_rs::transform::crop;
 * use photon_rs::PhotonImage;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let cropped_img: PhotonImage = crop(&img, 0_u32, 0_u32, 500_u32, 800_u32);
 * // Write the contents of this image in JPG format.
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} x1
 * @param {number} y1
 * @param {number} x2
 * @param {number} y2
 * @returns {PhotonImage}
 */
function crop(photon_image, x1, y1, x2, y2) {
    _assertClass(photon_image, PhotonImage);
    const ret = wasm.crop(photon_image.__wbg_ptr, x1, y1, x2, y2);
    return PhotonImage.__wrap(ret);
}
exports.crop = crop;

/**
 * @param {HTMLCanvasElement} source_canvas
 * @param {number} width
 * @param {number} height
 * @param {number} left
 * @param {number} top
 * @returns {HTMLCanvasElement}
 */
function crop_img_browser(source_canvas, width, height, left, top) {
    const ret = wasm.crop_img_browser(source_canvas, width, height, left, top);
    return ret;
}
exports.crop_img_browser = crop_img_browser;

/**
 * Darken the image by a specified amount in the HSL colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Darkening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to darken an image by 10% in the HSL colour space:
 * use photon_rs::colour_spaces::darken_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * darken_hsl(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function darken_hsl(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_hsl(img.__wbg_ptr, level);
}
exports.darken_hsl = darken_hsl;

/**
 * Darken the image by a specified amount in the HSLuv colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Darkening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to darken an image by 10% in the HSLuv colour space:
 * use photon_rs::colour_spaces::darken_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * darken_hsluv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function darken_hsluv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_hsluv(img.__wbg_ptr, level);
}
exports.darken_hsluv = darken_hsluv;

/**
 * Darken the image's colours by a specified amount in the HSV colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Darkening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to darken an image by 10% in the HSV colour space:
 * use photon_rs::colour_spaces::darken_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * darken_hsv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function darken_hsv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_hsv(img.__wbg_ptr, level);
}
exports.darken_hsv = darken_hsv;

/**
 * Darken the image by a specified amount in the LCh colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to darken the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Darkening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to darken an image by 10% in the LCh colour space:
 * use photon_rs::colour_spaces::darken_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * darken_lch(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function darken_lch(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_lch(img.__wbg_ptr, level);
}
exports.darken_lch = darken_lch;

/**
 * Decrease the brightness of an image by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `brightness` - A u8 to subtract from the brightness. It should be a positive number,
 * and this value will then be subtracted from the brightness.
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::dec_brightness;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * dec_brightness(&mut img, 10_u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} brightness
 */
function dec_brightness(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.dec_brightness(photon_image.__wbg_ptr, brightness);
}
exports.dec_brightness = dec_brightness;

/**
 * Uses a max. decomposition algorithm to convert an image to greyscale.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to decompose an image with max decomposition:
 * use photon_rs::monochrome::decompose_max;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * decompose_max(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function decompose_max(img) {
    _assertClass(img, PhotonImage);
    wasm.decompose_max(img.__wbg_ptr);
}
exports.decompose_max = decompose_max;

/**
 * Uses a min. decomposition algorithm to convert an image to greyscale.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to decompose an image with min decomposition:
 * use photon_rs::monochrome::decompose_min;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * decompose_min(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function decompose_min(img) {
    _assertClass(img, PhotonImage);
    wasm.decompose_min(img.__wbg_ptr);
}
exports.decompose_min = decompose_min;

/**
 * Desaturate an image by getting the min/max of each pixel's RGB values.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * // For example, to desaturate an image:
 * use photon_rs::monochrome::desaturate;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function desaturate(img) {
    _assertClass(img, PhotonImage);
    wasm.desaturate(img.__wbg_ptr);
}
exports.desaturate = desaturate;

/**
 * Desaturate the image by a specified amount in the HSL colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Desaturating by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to desaturate an image by 10% in the LCh colour space:
 * use photon_rs::colour_spaces::desaturate_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate_hsl(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function desaturate_hsl(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_hsl(img.__wbg_ptr, level);
}
exports.desaturate_hsl = desaturate_hsl;

/**
 * Desaturate the image by a specified amount in the HSLuv colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Desaturating by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to desaturate an image by 10% in the HSLuv colour space:
 * use photon_rs::colour_spaces::desaturate_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate_hsluv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function desaturate_hsluv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_hsluv(img.__wbg_ptr, level);
}
exports.desaturate_hsluv = desaturate_hsluv;

/**
 * Desaturate the image by a specified amount in the HSV colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Desaturating by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to desaturate an image by 10% in the HSV colour space:
 * use photon_rs::colour_spaces::desaturate_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate_hsv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function desaturate_hsv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_hsv(img.__wbg_ptr, level);
}
exports.desaturate_hsv = desaturate_hsv;

/**
 * Desaturate the image by a specified amount in the LCh colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to desaturate the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Desaturating by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to desaturate an image by 10% in the LCh colour space:
 * use photon_rs::colour_spaces::desaturate_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * desaturate_lch(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function desaturate_lch(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_lch(img.__wbg_ptr, level);
}
exports.desaturate_lch = desaturate_lch;

/**
 * Detect lines at a 135 degree angle in an image, and highlight these only.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to display the lines at a 135 degree angle in an image:
 * use photon_rs::conv::detect_135_deg_lines;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * detect_135_deg_lines(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function detect_135_deg_lines(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_135_deg_lines(photon_image.__wbg_ptr);
}
exports.detect_135_deg_lines = detect_135_deg_lines;

/**
 * Detect lines at a forty five degree angle in an image, and highlight these only.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to display the lines at a forty five degree angle in an image:
 * use photon_rs::conv::detect_45_deg_lines;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * detect_45_deg_lines(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function detect_45_deg_lines(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_45_deg_lines(photon_image.__wbg_ptr);
}
exports.detect_45_deg_lines = detect_45_deg_lines;

/**
 * Detect horizontal lines in an image, and highlight these only.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to display the horizontal lines in an image:
 * use photon_rs::conv::detect_horizontal_lines;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * detect_horizontal_lines(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function detect_horizontal_lines(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_horizontal_lines(photon_image.__wbg_ptr);
}
exports.detect_horizontal_lines = detect_horizontal_lines;

/**
 * Detect vertical lines in an image, and highlight these only.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to display the vertical lines in an image:
 * use photon_rs::conv::detect_vertical_lines;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * detect_vertical_lines(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function detect_vertical_lines(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_vertical_lines(photon_image.__wbg_ptr);
}
exports.detect_vertical_lines = detect_vertical_lines;

/**
 * Applies Floyd-Steinberg dithering to an image.
 * Only RGB channels are processed, alpha remains unchanged.
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `depth` - bits per channel. Clamped between 1 and 8.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into a dithered image:
 * use photon_rs::effects::dither;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let depth = 1;
 * dither(&mut img, depth);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} depth
 */
function dither(photon_image, depth) {
    _assertClass(photon_image, PhotonImage);
    wasm.dither(photon_image.__wbg_ptr, depth);
}
exports.dither = dither;

/**
 * Greyscale effect with increased contrast.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::dramatic;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * dramatic(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function dramatic(img) {
    _assertClass(img, PhotonImage);
    wasm.dramatic(img.__wbg_ptr);
}
exports.dramatic = dramatic;

/**
 * Add text to an image.
 * The only font available as of now is Roboto.
 * Note: A graphic design/text-drawing library is currently being developed, so stay tuned.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `text` - Text string to be drawn to the image.
 * * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
 * * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
 * * `font_size` - Font size in pixels of the text to be drawn.
 *
 * # Example
 *
 * ```no_run
 * // For example to draw the string "Welcome to Photon!" at 10, 10:
 * use photon_rs::native::open_image;
 * use photon_rs::text::draw_text;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * draw_text(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32);
 * ```
 * @param {PhotonImage} photon_img
 * @param {string} text
 * @param {number} x
 * @param {number} y
 * @param {number} font_size
 */
function draw_text(photon_img, text, x, y, font_size) {
    _assertClass(photon_img, PhotonImage);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.draw_text(photon_img.__wbg_ptr, ptr0, len0, x, y, font_size);
}
exports.draw_text = draw_text;

/**
 * Add bordered-text to an image.
 * The only font available as of now is Roboto.
 * Note: A graphic design/text-drawing library is currently being developed, so stay tuned.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `text` - Text string to be drawn to the image.
 * * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
 * * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
 * * `font_size` - Font size in pixels of the text to be drawn.
 *
 * # Example
 *
 * ```no_run
 * // For example to draw the string "Welcome to Photon!" at 10, 10:
 * use photon_rs::native::open_image;
 * use photon_rs::text::draw_text_with_border;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * draw_text_with_border(&mut img, "Welcome to Photon!", 10_i32, 10_i32, 90_f32);
 * ```
 * @param {PhotonImage} photon_img
 * @param {string} text
 * @param {number} x
 * @param {number} y
 * @param {number} font_size
 */
function draw_text_with_border(photon_img, text, x, y, font_size) {
    _assertClass(photon_img, PhotonImage);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.draw_text_with_border(photon_img.__wbg_ptr, ptr0, len0, x, y, font_size);
}
exports.draw_text_with_border = draw_text_with_border;

/**
 * Add text to an image with a custom color.
 * The only font available as of now is Roboto.
 * Note: A graphic design/text-drawing library is currently being developed, so stay tuned.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `text` - Text string to be drawn to the image.
 * * `x` - x-coordinate of where first letter's 1st pixel should be drawn.
 * * `y` - y-coordinate of where first letter's 1st pixel should be drawn.
 * * `font_size` - Font size in pixels of the text to be drawn.
 * * `color` - RGBA color value for the text.
 *
 * # Example
 *
 * ```no_run
 * // For example to draw the string "Welcome to Photon!" at 10, 10:
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 * use photon_rs::text::draw_text_with_color;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * draw_text_with_color(
 *     &mut img,
 *     "Welcome to Photon!",
 *     10_i32,
 *     10_i32,
 *     90_f32,
 *     Rgba::new(100u8, 100u8, 100u8, 255u8),
 * );
 * ```
 * @param {PhotonImage} photon_img
 * @param {string} text
 * @param {number} x
 * @param {number} y
 * @param {number} font_size
 * @param {Rgba} color
 */
function draw_text_with_color(photon_img, text, x, y, font_size, color) {
    _assertClass(photon_img, PhotonImage);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(color, Rgba);
    var ptr1 = color.__destroy_into_raw();
    wasm.draw_text_with_color(photon_img.__wbg_ptr, ptr0, len0, x, y, font_size, ptr1);
}
exports.draw_text_with_color = draw_text_with_color;

/**
 * @param {PhotonImage} photon_image
 * @param {Rgb} color_a
 * @param {Rgb} color_b
 */
function duotone(photon_image, color_a, color_b) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color_a, Rgb);
    var ptr0 = color_a.__destroy_into_raw();
    _assertClass(color_b, Rgb);
    var ptr1 = color_b.__destroy_into_raw();
    wasm.duotone(photon_image.__wbg_ptr, ptr0, ptr1);
}
exports.duotone = duotone;

/**
 * Duotone effect with purple tones.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_horizon;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * duotone_horizon(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function duotone_horizon(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_horizon(img.__wbg_ptr);
}
exports.duotone_horizon = duotone_horizon;

/**
 * Duotone effect with a lilac hue
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_lilac;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * duotone_lilac(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function duotone_lilac(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_lilac(img.__wbg_ptr);
}
exports.duotone_lilac = duotone_lilac;

/**
 * A duotone ochre tint effect
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_ochre;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * duotone_ochre(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function duotone_ochre(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_ochre(img.__wbg_ptr);
}
exports.duotone_ochre = duotone_ochre;

/**
 * A duotone filter with a user-specified color and a gray color
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `rgb_color` - RGB color
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_tint;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgb;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgb_color = Rgb::new(12, 12, 10);
 * duotone_tint(&mut img, rgb_color);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} rgb_color
 */
function duotone_tint(img, rgb_color) {
    _assertClass(img, PhotonImage);
    _assertClass(rgb_color, Rgb);
    var ptr0 = rgb_color.__destroy_into_raw();
    wasm.duotone_tint(img.__wbg_ptr, ptr0);
}
exports.duotone_tint = duotone_tint;

/**
 * Duotone effect with blue and purple tones.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::duotone_violette;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * duotone_violette(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function duotone_violette(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_violette(img.__wbg_ptr);
}
exports.duotone_violette = duotone_violette;

/**
 * Apply edge detection to an image, to create a dark version with its edges highlighted.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to increase the Red channel for all pixels by 10:
 * use photon_rs::conv::edge_detection;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * edge_detection(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function edge_detection(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.edge_detection(photon_image.__wbg_ptr);
}
exports.edge_detection = edge_detection;

/**
 * Preset edge effect.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply this effect:
 * use photon_rs::conv::edge_one;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * edge_one(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function edge_one(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.edge_one(photon_image.__wbg_ptr);
}
exports.edge_one = edge_one;

/**
 * Apply an emboss effect to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply an emboss effect:
 * use photon_rs::conv::emboss;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * emboss(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function emboss(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.emboss(photon_image.__wbg_ptr);
}
exports.emboss = emboss;

/**
 * Apply a cinematic film grain effect to an image.
 *
 * Simulates analog photographic grain by adding spatially-varying noise that is weighted by each
 * pixel's perceptual luminance: grain is strongest in the midtones and naturally falls off toward the
 * shadows and highlights matching the characteristic response of real photographic emulsions.
 *
 * # Arguments
 * * `photon_image` - A mutable reference to the [`PhotonImage`] to process.
 * * `intensity`    - Grain strength in the range `[0.0, 1.0]`.
 *                    `0.1` – `0.3` is a realistic film look; `1.0` is extreme.
 * * `monochrome`   - When `true`, a single noise sample is shared across R, G and B (silver-halide style, one PRNG call per pixel).
 *                    When `false`, each channel gets an independent sample, producing the subtle colour fringing of
 *                    multi-layer film stocks (three PRNG calls per pixel).
 *
 * * `seed`         - Initial PRNG seed. Use a fixed value for reproducible results or any non-zero runtime value for variation.
 *                    Supplying `0` falls back to an internal safe constant.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::noise::film_grain;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * film_grain(&mut img, 0.15, true, 42);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} intensity
 * @param {boolean} monochrome
 * @param {number} seed
 */
function film_grain(photon_image, intensity, monochrome, seed) {
    _assertClass(photon_image, PhotonImage);
    wasm.film_grain(photon_image.__wbg_ptr, intensity, monochrome, seed);
}
exports.film_grain = film_grain;

/**
 * Apply a filter to an image. Over 20 filters are available.
 * The filters are as follows:
 * * **oceanic**: Add an aquamarine-tinted hue to an image.
 * * **islands**: Aquamarine tint.
 * * **marine**: Add a green/blue mixed hue to an image.
 * * **seagreen**: Dark green hue, with tones of blue.
 * * **flagblue**: Royal blue tint
 * * **liquid**: Blue-inspired tint.
 * * **diamante**: Custom filter with a blue/turquoise tint.
 * * **radio**: Fallout-style radio effect.
 * * **twenties**: Slight-blue tinted historical effect.
 * * **rosetint**: Rose-tinted filter.
 * * **mauve**: Purple-infused filter.
 * * **bluechrome**: Blue monochrome effect.
 * * **vintage**: Vintage filter with a red tint.
 * * **perfume**: Increase the blue channel, with moderate increases in the Red and Green channels.
 * * **serenity**: Custom filter with an increase in the Blue channel's values.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `filter_name` - The filter's name. Choose from the selection above, eg: "oceanic"
 * # Example
 *
 * ```no_run
 * // For example, to add a filter called "vintage" to an image:
 * use photon_rs::filters::filter;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * filter(&mut img, "vintage");
 * ```
 * @param {PhotonImage} img
 * @param {string} filter_name
 */
function filter(img, filter_name) {
    _assertClass(img, PhotonImage);
    const ptr0 = passStringToWasm0(filter_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.filter(img.__wbg_ptr, ptr0, len0);
}
exports.filter = filter;

/**
 * Apply a red hue, with increased contrast and brightness.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::firenze;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * firenze(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function firenze(img) {
    _assertClass(img, PhotonImage);
    wasm.firenze(img.__wbg_ptr);
}
exports.firenze = firenze;

/**
 * Flip an image horizontally.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to flip an image horizontally:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::fliph;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * fliph(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function fliph(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.fliph(photon_image.__wbg_ptr);
}
exports.fliph = fliph;

/**
 * Flip an image vertically.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to flip an image vertically:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::flipv;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * flipv(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function flipv(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.flipv(photon_image.__wbg_ptr);
}
exports.flipv = flipv;

/**
 * Turn an image into an frosted glass see through
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into frosted glass see through:
 * use photon_rs::effects::frosted_glass;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * frosted_glass(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function frosted_glass(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.frosted_glass(photon_image.__wbg_ptr);
}
exports.frosted_glass = frosted_glass;

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to the Green channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::monochrome::g_grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * g_grayscale(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function g_grayscale(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.g_grayscale(photon_image.__wbg_ptr);
}
exports.g_grayscale = g_grayscale;

/**
 * Applies gamma correction to an image.
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `red` - Gamma value for red channel.
 * * `green` - Gamma value for green channel.
 * * `blue` - Gamma value for blue channel.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into a gamma corrected image:
 * use photon_rs::colour_spaces::gamma_correction;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * gamma_correction(&mut img, 2.2, 2.2, 2.2);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} red
 * @param {number} green
 * @param {number} blue
 */
function gamma_correction(photon_image, red, green, blue) {
    _assertClass(photon_image, PhotonImage);
    wasm.gamma_correction(photon_image.__wbg_ptr, red, green, blue);
}
exports.gamma_correction = gamma_correction;

/**
 * Gaussian blur in linear time.
 *
 * Reference: http://blog.ivank.net/fastest-gaussian-blur.html
 *
 * # Arguments
 * * `photon_image` - A PhotonImage
 * * `radius` - blur radius
 * # Example
 *
 * ```no_run
 * use photon_rs::conv::gaussian_blur;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * gaussian_blur(&mut img, 3_i32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} radius
 */
function gaussian_blur(photon_image, radius) {
    _assertClass(photon_image, PhotonImage);
    wasm.gaussian_blur(photon_image.__wbg_ptr, radius);
}
exports.gaussian_blur = gaussian_blur;

/**
 * Get the ImageData from a 2D canvas context
 * @param {HTMLCanvasElement} canvas
 * @param {CanvasRenderingContext2D} ctx
 * @returns {ImageData}
 */
function get_image_data(canvas, ctx) {
    const ret = wasm.get_image_data(canvas, ctx);
    return ret;
}
exports.get_image_data = get_image_data;

/**
 * Apply a vintage, golden hue to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::golden;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * golden(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function golden(img) {
    _assertClass(img, PhotonImage);
    wasm.golden(img.__wbg_ptr);
}
exports.golden = golden;

/**
 * Convert an image to grayscale using the conventional averaging algorithm.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * // For example, to convert an image of type `PhotonImage` to grayscale:
 * use photon_rs::monochrome::grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * grayscale(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function grayscale(img) {
    _assertClass(img, PhotonImage);
    wasm.grayscale(img.__wbg_ptr);
}
exports.grayscale = grayscale;

/**
 * Convert an image to grayscale with a human corrected factor, to account for human vision.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * // For example, to convert an image of type `PhotonImage` to grayscale with a human corrected factor:
 * use photon_rs::monochrome::grayscale_human_corrected;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * grayscale_human_corrected(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function grayscale_human_corrected(img) {
    _assertClass(img, PhotonImage);
    wasm.grayscale_human_corrected(img.__wbg_ptr);
}
exports.grayscale_human_corrected = grayscale_human_corrected;

/**
 * Employ only a limited number of gray shades in an image.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `num_shades` - The number of grayscale shades to be displayed in the image.
 *
 * # Example
 *
 * ```no_run
 * // For example, to limit an image to four shades of gray only:
 * use photon_rs::monochrome::grayscale_shades;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * grayscale_shades(&mut img, 4_u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_shades
 */
function grayscale_shades(photon_image, num_shades) {
    _assertClass(photon_image, PhotonImage);
    wasm.grayscale_shades(photon_image.__wbg_ptr, num_shades);
}
exports.grayscale_shades = grayscale_shades;

/**
 * Halftoning effect.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example:
 * use photon_rs::effects::halftone;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * halftone(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function halftone(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.halftone(photon_image.__wbg_ptr);
}
exports.halftone = halftone;

/**
 * Horizontal strips. Divide an image into a series of equal-height strips, for an artistic effect.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `num_strips` - The number of strips
 * # Example
 *
 * ```no_run
 * // For example, to draw horizontal strips on a `PhotonImage`:
 * use photon_rs::effects::horizontal_strips;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * horizontal_strips(&mut img, 8u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_strips
 */
function horizontal_strips(photon_image, num_strips) {
    _assertClass(photon_image, PhotonImage);
    wasm.horizontal_strips(photon_image.__wbg_ptr, num_strips);
}
exports.horizontal_strips = horizontal_strips;

/**
 * Image manipulation effects in the HSL colour space.
 *
 * Effects include:
 * * **saturate** - Saturation increase.
 * * **desaturate** - Desaturate the image.
 * * **shift_hue** - Hue rotation by a specified number of degrees.
 * * **darken** - Decrease the brightness.
 * * **lighten** - Increase the brightness.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
 * * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
 * # Example
 * ```no_run
 * // For example to increase the saturation by 10%:
 * use photon_rs::colour_spaces::hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hsl(&mut img, "saturate", 0.1_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
function hsl(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsl(photon_image.__wbg_ptr, ptr0, len0, amt);
}
exports.hsl = hsl;

/**
 * Image manipulation effects in the HSLuv colour space
 *
 * Effects include:
 * * **saturate** - Saturation increase.
 * * **desaturate** - Desaturate the image.
 * * **shift_hue** - Hue rotation by a specified number of degrees.
 * * **darken** - Decrease the brightness.
 * * **lighten** - Increase the brightness.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
 * * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
 * # Example
 * ```no_run
 * // For example to increase the saturation by 10%:
 * use photon_rs::colour_spaces::hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hsluv(&mut img, "saturate", 0.1_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
function hsluv(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsluv(photon_image.__wbg_ptr, ptr0, len0, amt);
}
exports.hsluv = hsluv;

/**
 * Image manipulation in the HSV colour space.
 *
 * Effects include:
 * * **saturate** - Saturation increase.
 * * **desaturate** - Desaturate the image.
 * * **shift_hue** - Hue rotation by a specified number of degrees.
 * * **darken** - Decrease the brightness.
 * * **lighten** - Increase the brightness.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
 * * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
 *
 * # Example
 * ```no_run
 * // For example to increase the saturation by 10%:
 * use photon_rs::colour_spaces::hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hsv(&mut img, "saturate", 0.1_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
function hsv(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsv(photon_image.__wbg_ptr, ptr0, len0, amt);
}
exports.hsv = hsv;

/**
 * Shift hue by a specified number of degrees in the HSL colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
 *
 * # Example
 * ```no_run
 * // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
 * use photon_rs::colour_spaces::hue_rotate_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hue_rotate_hsl(&mut img, 120_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} degrees
 */
function hue_rotate_hsl(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_hsl(img.__wbg_ptr, degrees);
}
exports.hue_rotate_hsl = hue_rotate_hsl;

/**
 * Shift hue by a specified number of degrees in the HSLuv colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
 *
 * # Example
 * ```no_run
 * // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
 * use photon_rs::colour_spaces::hue_rotate_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hue_rotate_hsluv(&mut img, 120_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} degrees
 */
function hue_rotate_hsluv(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_hsluv(img.__wbg_ptr, degrees);
}
exports.hue_rotate_hsluv = hue_rotate_hsluv;

/**
 * Shift hue by a specified number of degrees in the HSV colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
 *
 * # Example
 * ```no_run
 * // For example to hue rotate/shift the hue by 120 degrees in the HSV colour space:
 * use photon_rs::colour_spaces::hue_rotate_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hue_rotate_hsv(&mut img, 120_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} degrees
 */
function hue_rotate_hsv(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_hsv(img.__wbg_ptr, degrees);
}
exports.hue_rotate_hsv = hue_rotate_hsv;

/**
 * Shift hue by a specified number of degrees in the LCh colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `mode` - A float value from 0 to 1 which is the amount to shift the hue by, or hue rotate by.
 *
 * # Example
 * ```no_run
 * // For example to hue rotate/shift the hue by 120 degrees in the HSL colour space:
 * use photon_rs::colour_spaces::hue_rotate_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * hue_rotate_lch(&mut img, 120_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} degrees
 */
function hue_rotate_lch(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_lch(img.__wbg_ptr, degrees);
}
exports.hue_rotate_lch = hue_rotate_lch;

/**
 * Apply an identity kernel convolution to an image.
 *
 * # Arguments
 * * `img` -A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply an identity kernel convolution:
 * use photon_rs::conv::identity;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * identity(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function identity(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.identity(photon_image.__wbg_ptr);
}
exports.identity = identity;

/**
 * Increase the brightness of an image by a constant.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `brightness` - A u8 to add to the brightness.
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::inc_brightness;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * inc_brightness(&mut img, 10_u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} brightness
 */
function inc_brightness(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.inc_brightness(photon_image.__wbg_ptr, brightness);
}
exports.inc_brightness = inc_brightness;

/**
 * Invert RGB value of an image.
 *
 * # Arguments
 * * `photon_image` - A DynamicImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * use photon_rs::channels::invert;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * invert(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function invert(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.invert(photon_image.__wbg_ptr);
}
exports.invert = invert;

/**
 * Apply a standard laplace convolution.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a laplace effect:
 * use photon_rs::conv::laplace;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * laplace(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function laplace(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.laplace(photon_image.__wbg_ptr);
}
exports.laplace = laplace;

/**
 * Image manipulation effects in the LCh colour space
 *
 * Effects include:
 * * **saturate** - Saturation increase.
 * * **desaturate** - Desaturate the image.
 * * **shift_hue** - Hue rotation by a specified number of degrees.
 * * **darken** - Decrease the brightness.
 * * **lighten** - Increase the brightness.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `mode` - The effect desired to be applied. Choose from: `saturate`, `desaturate`, `shift_hue`, `darken`, `lighten`
 * * `amt` - A float value from 0 to 1 which represents the amount the effect should be increased by.
 * # Example
 * ```no_run
 * // For example to increase the saturation by 10%:
 * use photon_rs::colour_spaces::lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lch(&mut img, "saturate", 0.1_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {string} mode
 * @param {number} amt
 */
function lch(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.lch(photon_image.__wbg_ptr, ptr0, len0, amt);
}
exports.lch = lch;

/**
 * Lighten an image by a specified amount in the HSL colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Lightening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to lighten an image by 10% in the HSL colour space:
 * use photon_rs::colour_spaces::lighten_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lighten_hsl(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function lighten_hsl(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_hsl(img.__wbg_ptr, level);
}
exports.lighten_hsl = lighten_hsl;

/**
 * Lighten an image by a specified amount in the HSLuv colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Lightening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to lighten an image by 10% in the HSLuv colour space:
 * use photon_rs::colour_spaces::lighten_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lighten_hsluv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function lighten_hsluv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_hsluv(img.__wbg_ptr, level);
}
exports.lighten_hsluv = lighten_hsluv;

/**
 * Lighten an image by a specified amount in the HSV colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Lightening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to lighten an image by 10% in the HSV colour space:
 * use photon_rs::colour_spaces::lighten_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lighten_hsv(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function lighten_hsv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_hsv(img.__wbg_ptr, level);
}
exports.lighten_hsv = lighten_hsv;

/**
 * Lighten an image by a specified amount in the LCh colour space.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to lighten the image by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Lightening by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to lighten an image by 10% in the LCh colour space:
 * use photon_rs::colour_spaces::lighten_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * lighten_lch(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function lighten_lch(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_lch(img.__wbg_ptr, level);
}
exports.lighten_lch = lighten_lch;

/**
 * Solarization on the Red and Green channels.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::lix;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * lix(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function lix(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.lix(photon_image.__wbg_ptr);
}
exports.lix = lix;

/**
 * Apply a lofi effect to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::lofi;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * lofi(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function lofi(img) {
    _assertClass(img, PhotonImage);
    wasm.lofi(img.__wbg_ptr);
}
exports.lofi = lofi;

/**
 * Mix image with a single color, supporting passing `opacity`.
 * The algorithm comes from Jimp. See `function mix` and `function colorFn` at following link:
 * https://github.com/oliver-moran/jimp/blob/29679faa597228ff2f20d34c5758e4d2257065a3/packages/plugin-color/src/index.js
 * Specifically, result_value = (mix_color_value - origin_value) * opacity + origin_value =
 * mix_color_value * opacity + (1 - opacity) * origin_value for each
 * of RGB channel.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `mix_color` - the color to be mixed in, as an RGB value.
 * * `opacity` - the opacity of color when mixed to image. Float value from 0 to 1.
 * # Example
 *
 * ```no_run
 * // For example, to mix an image with rgb (50, 255, 254) and opacity 0.4:
 * use photon_rs::Rgb;
 * use photon_rs::colour_spaces::mix_with_colour;
 * use photon_rs::native::open_image;
 *
 * let mix_colour = Rgb::new(50_u8, 255_u8, 254_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * mix_with_colour(&mut img, mix_colour, 0.4_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {Rgb} mix_colour
 * @param {number} opacity
 */
function mix_with_colour(photon_image, mix_colour, opacity) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(mix_colour, Rgb);
    var ptr0 = mix_colour.__destroy_into_raw();
    wasm.mix_with_colour(photon_image.__wbg_ptr, ptr0, opacity);
}
exports.mix_with_colour = mix_with_colour;

/**
 * Apply a monochrome effect of a certain colour.
 *
 * It does so by averaging the R, G, and B values of a pixel, and then adding a
 * separate value to that averaged value for each channel to produce a tint.
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `r_offset` - The value to add to the Red channel per pixel.
 * * `g_offset` - The value to add to the Green channel per pixel.
 * * `b_offset` - The value to add to the Blue channel per pixel.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a monochrome effect to an image:
 * use photon_rs::monochrome::monochrome;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * monochrome(&mut img, 40_u32, 50_u32, 100_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} r_offset
 * @param {number} g_offset
 * @param {number} b_offset
 */
function monochrome(img, r_offset, g_offset, b_offset) {
    _assertClass(img, PhotonImage);
    wasm.monochrome(img.__wbg_ptr, r_offset, g_offset, b_offset);
}
exports.monochrome = monochrome;

/**
 * Monochrome tint effect with increased contrast
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `rgb_color` - RGB color
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::monochrome_tint;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgb;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgb_color = Rgb::new(12, 12, 10);
 * monochrome_tint(&mut img, rgb_color);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} rgb_color
 */
function monochrome_tint(img, rgb_color) {
    _assertClass(img, PhotonImage);
    _assertClass(rgb_color, Rgb);
    var ptr0 = rgb_color.__destroy_into_raw();
    wasm.monochrome_tint(img.__wbg_ptr, ptr0);
}
exports.monochrome_tint = monochrome_tint;

/**
 * Adds multiple offsets to the image by a certain number of pixels (on two channels).
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `offset` - The offset is added to the pixels in the image.
 * # Example
 *
 * ```no_run
 * // For example, to add a 30-pixel offset to both the red and blue channels:
 * use photon_rs::effects::multiple_offsets;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * multiple_offsets(&mut img, 30_u32, 0_usize, 2_usize);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} offset
 * @param {number} channel_index
 * @param {number} channel_index2
 */
function multiple_offsets(photon_image, offset, channel_index, channel_index2) {
    _assertClass(photon_image, PhotonImage);
    wasm.multiple_offsets(photon_image.__wbg_ptr, offset, channel_index, channel_index2);
}
exports.multiple_offsets = multiple_offsets;

/**
 * Solarization on the Blue channel.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::neue;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * neue(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function neue(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.neue(photon_image.__wbg_ptr);
}
exports.neue = neue;

/**
 * Noise reduction.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to noise reduct an image:
 * use photon_rs::conv::noise_reduction;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * noise_reduction(&mut img);
 * ```
 * Adds a constant to a select R, G, or B channel's value.
 * @param {PhotonImage} photon_image
 */
function noise_reduction(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.noise_reduction(photon_image.__wbg_ptr);
}
exports.noise_reduction = noise_reduction;

/**
 * Normalizes an image by remapping its range of pixels values. Only RGB
 * channels are processed and each channel is stretched to \[0, 255\] range
 * independently. This process is also known as contrast stretching.
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into a normalized image:
 * use photon_rs::effects::normalize;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * normalize(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function normalize(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.normalize(photon_image.__wbg_ptr);
}
exports.normalize = normalize;

/**
 * Apply a greyscale effect with increased contrast.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::obsidian;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * obsidian(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function obsidian(img) {
    _assertClass(img, PhotonImage);
    wasm.obsidian(img.__wbg_ptr);
}
exports.obsidian = obsidian;

/**
 * Adds an offset to the image by a certain number of pixels.
 *
 * This creates an RGB shift effect.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `channel_index`: The index of the channel to increment. 0 for red, 1 for green and 2 for blue.
 * * `offset` - The offset is added to the pixels in the image.
 * # Example
 *
 * ```no_run
 * // For example, to offset pixels by 30 pixels on the red channel:
 * use photon_rs::effects::offset;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * offset(&mut img, 0_usize, 30_u32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} channel_index
 * @param {number} offset
 */
function offset(photon_image, channel_index, offset) {
    _assertClass(photon_image, PhotonImage);
    wasm.offset(photon_image.__wbg_ptr, channel_index, offset);
}
exports.offset = offset;

/**
 * Adds an offset to the blue channel by a certain number of pixels.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `offset_amt` - The offset you want to move the blue channel by.
 * # Example
 * // For example, to add an offset to the green channel by 40 pixels.
 *
 * ```no_run
 * use photon_rs::effects::offset_blue;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * offset_blue(&mut img, 40_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} offset_amt
 */
function offset_blue(img, offset_amt) {
    _assertClass(img, PhotonImage);
    wasm.offset_blue(img.__wbg_ptr, offset_amt);
}
exports.offset_blue = offset_blue;

/**
 * Adds an offset to the green channel by a certain number of pixels.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `offset` - The offset you want to move the green channel by.
 * # Example
 *
 * ```no_run
 * // For example, to add an offset to the green channel by 30 pixels.
 * use photon_rs::effects::offset_green;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * offset_green(&mut img, 30_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} offset_amt
 */
function offset_green(img, offset_amt) {
    _assertClass(img, PhotonImage);
    wasm.offset_green(img.__wbg_ptr, offset_amt);
}
exports.offset_green = offset_green;

/**
 * Adds an offset to the red channel by a certain number of pixels.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `offset` - The offset you want to move the red channel by.
 * # Example
 *
 * ```no_run
 * // For example, to add an offset to the red channel by 30 pixels.
 * use photon_rs::effects::offset_red;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * offset_red(&mut img, 30_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} offset_amt
 */
function offset_red(img, offset_amt) {
    _assertClass(img, PhotonImage);
    wasm.offset_red(img.__wbg_ptr, offset_amt);
}
exports.offset_red = offset_red;

/**
 * Turn an image into an oil painting
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `radius` - Radius of each paint particle
 * * `intesnity` - How artsy an Image should be
 * # Example
 *
 * ```no_run
 * // For example, to oil an image of type `PhotonImage`:
 * use photon_rs::effects::oil;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * oil(&mut img, 4i32, 55.0);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} radius
 * @param {number} intensity
 */
function oil(photon_image, radius, intensity) {
    _assertClass(photon_image, PhotonImage);
    wasm.oil(photon_image.__wbg_ptr, radius, intensity);
}
exports.oil = oil;

/**
 * Convert a HTML5 Canvas Element to a PhotonImage.
 *
 * This converts the ImageData found in the canvas context to a PhotonImage,
 * which can then have effects or filters applied to it.
 * @param {HTMLCanvasElement} canvas
 * @param {CanvasRenderingContext2D} ctx
 * @returns {PhotonImage}
 */
function open_image(canvas, ctx) {
    const ret = wasm.open_image(canvas, ctx);
    return PhotonImage.__wrap(ret);
}
exports.open_image = open_image;

/**
 * Apply padding on the left side of the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels on the bottom of a PhotonImage:
 * use photon_rs::transform::padding_bottom;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_bottom(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
function padding_bottom(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_bottom(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}
exports.padding_bottom = padding_bottom;

/**
 * Apply padding on the left side of the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels on the left side of a PhotonImage:
 * use photon_rs::transform::padding_left;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_left(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
function padding_left(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_left(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}
exports.padding_left = padding_left;

/**
 * Apply padding on the left side of the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels on the right side of a PhotonImage:
 * use photon_rs::transform::padding_right;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_right(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
function padding_right(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_right(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}
exports.padding_right = padding_right;

/**
 * Apply padding on the left side of the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels on the top of a PhotonImage:
 * use photon_rs::transform::padding_top;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_top(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
function padding_top(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_top(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}
exports.padding_top = padding_top;

/**
 * Apply uniform padding around the PhotonImage
 * A padded PhotonImage is returned.
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `padding` - The amount of padding to be applied to the PhotonImage.
 * * `padding_rgba` - Tuple containing the RGBA code for padding color.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a padding of 10 pixels around a PhotonImage:
 * use photon_rs::transform::padding_uniform;
 * use photon_rs::native::open_image;
 * use photon_rs::Rgba;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let rgba = Rgba::new(200_u8, 100_u8, 150_u8, 255_u8);
 * padding_uniform(&img, 10_u32, rgba);
 * ```
 * @param {PhotonImage} img
 * @param {number} padding
 * @param {Rgba} padding_rgba
 * @returns {PhotonImage}
 */
function padding_uniform(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_uniform(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
}
exports.padding_uniform = padding_uniform;

/**
 * Apply a rose tint to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::pastel_pink;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * pastel_pink(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function pastel_pink(img) {
    _assertClass(img, PhotonImage);
    wasm.pastel_pink(img.__wbg_ptr);
}
exports.pastel_pink = pastel_pink;

/**
 * Add pink-tinted noise to an image.
 *
 * **[WASM SUPPORT IS AVAILABLE]**: Randomized thread pools cannot be created with WASM, but
 * a workaround using js_sys::Math::random works now.
 * # Arguments
 * * `name` - A PhotonImage that contains a view into the image.
 *
 * # Example
 *
 * ```no_run
 * // For example, to add pink-tinted noise to an image:
 * use photon_rs::native::open_image;
 * use photon_rs::noise::pink_noise;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * pink_noise(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function pink_noise(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.pink_noise(photon_image.__wbg_ptr);
}
exports.pink_noise = pink_noise;

/**
 * Pixelize an image.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage that contains a view into the image.
 * * `pixel_size` - Targeted pixel size of generated image.
 * # Example
 *
 * ```no_run
 * // For example, to turn an image of type `PhotonImage` into a pixelized image with 50 pixels blocks:
 * use photon_rs::effects::pixelize;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * pixelize(&mut img, 50);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} pixel_size
 */
function pixelize(photon_image, pixel_size) {
    _assertClass(photon_image, PhotonImage);
    wasm.pixelize(photon_image.__wbg_ptr, pixel_size);
}
exports.pixelize = pixelize;

/**
 * Apply a horizontal Prewitt convolution to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a horizontal Prewitt convolution effect:
 * use photon_rs::conv::prewitt_horizontal;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * prewitt_horizontal(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function prewitt_horizontal(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.prewitt_horizontal(photon_image.__wbg_ptr);
}
exports.prewitt_horizontal = prewitt_horizontal;

/**
 * Reduces an image to the primary colours.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to add a primary colour effect to an image of type `DynamicImage`:
 * use photon_rs::effects::primary;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * primary(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function primary(img) {
    _assertClass(img, PhotonImage);
    wasm.primary(img.__wbg_ptr);
}
exports.primary = primary;

/**
 * Place a PhotonImage onto a 2D canvas.
 * @param {HTMLCanvasElement} canvas
 * @param {CanvasRenderingContext2D} ctx
 * @param {PhotonImage} new_image
 */
function putImageData(canvas, ctx, new_image) {
    _assertClass(new_image, PhotonImage);
    var ptr0 = new_image.__destroy_into_raw();
    wasm.putImageData(canvas, ctx, ptr0);
}
exports.putImageData = putImageData;

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to the Red channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::monochrome::r_grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * r_grayscale(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function r_grayscale(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.r_grayscale(photon_image.__wbg_ptr);
}
exports.r_grayscale = r_grayscale;

/**
 * Remove the Blue channel's influence in an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter.
 *
 * # Example
 *
 * ```no_run
 * // For example, to remove the blue channel for blue channel pixel values less than 50:
 * use photon_rs::channels::remove_blue_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * remove_blue_channel(&mut img, 50_u8);
 * ```
 * @param {PhotonImage} img
 * @param {number} min_filter
 */
function remove_blue_channel(img, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_blue_channel(img.__wbg_ptr, min_filter);
}
exports.remove_blue_channel = remove_blue_channel;

/**
 * Set a certain channel to zero, thus removing the channel's influence in the pixels' final rendered colour.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `channel` - The channel to be removed; must be a usize from 0 to 2, with 0 representing Red, 1 representing Green, and 2 representing Blue.
 * * `min_filter` - Minimum filter. Value between 0 and 255. Only remove the channel if the current pixel's channel value is less than this minimum filter. To completely
 * remove the channel, set this value to 255, to leave the channel as is, set to 0, and to set a channel to zero for a pixel whose red value is greater than 50,
 * then channel would be 0 and min_filter would be 50.
 *
 * # Example
 *
 * ```no_run
 * // For example, to remove the Red channel with a min_filter of 100:
 * use photon_rs::channels::remove_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * remove_channel(&mut img, 0_usize, 100_u8);
 * ```
 * @param {PhotonImage} img
 * @param {number} channel
 * @param {number} min_filter
 */
function remove_channel(img, channel, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_channel(img.__wbg_ptr, channel, min_filter);
}
exports.remove_channel = remove_channel;

/**
 * Remove the Green channel's influence in an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter.
 *
 * # Example
 *
 * ```no_run
 * // For example, to remove the green channel for green channel pixel values less than 50:
 * use photon_rs::channels::remove_green_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * remove_green_channel(&mut img, 50_u8);
 * ```
 * @param {PhotonImage} img
 * @param {number} min_filter
 */
function remove_green_channel(img, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_green_channel(img.__wbg_ptr, min_filter);
}
exports.remove_green_channel = remove_green_channel;

/**
 * Remove the Red channel's influence in an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `min_filter` - Only remove the channel if the current pixel's channel value is less than this minimum filter.
 *
 * # Example
 *
 * ```no_run
 * // For example, to remove the red channel for red channel pixel values less than 50:
 * use photon_rs::channels::remove_red_channel;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * remove_red_channel(&mut img, 50_u8);
 * ```
 * @param {PhotonImage} img
 * @param {number} min_filter
 */
function remove_red_channel(img, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_red_channel(img.__wbg_ptr, min_filter);
}
exports.remove_red_channel = remove_red_channel;

/**
 * Resample the PhotonImage.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `dst_width` - Target width.
 * * `dst_height` - Target height.
 *
 * # Example
 *
 * ```no_run
 * // For example, to resample a PhotonImage to 1920x1080 size:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::resample;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let rotated_img = resample(&img, 1920, 1080);
 * ```
 * @param {PhotonImage} img
 * @param {number} dst_width
 * @param {number} dst_height
 * @returns {PhotonImage}
 */
function resample(img, dst_width, dst_height) {
    _assertClass(img, PhotonImage);
    const ret = wasm.resample(img.__wbg_ptr, dst_width, dst_height);
    return PhotonImage.__wrap(ret);
}
exports.resample = resample;

/**
 * Resize an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `width` - New width.
 * * `height` - New height.
 * * `sampling_filter` - Nearest = 1, Triangle = 2, CatmullRom = 3, Gaussian = 4, Lanczos3 = 5
 * @param {PhotonImage} photon_img
 * @param {number} width
 * @param {number} height
 * @param {SamplingFilter} sampling_filter
 * @returns {PhotonImage}
 */
function resize(photon_img, width, height, sampling_filter) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.resize(photon_img.__wbg_ptr, width, height, sampling_filter);
    return PhotonImage.__wrap(ret);
}
exports.resize = resize;

/**
 * Resize an image on the web.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `width` - New width.
 * * `height` - New height.
 * * `sampling_filter` - Nearest = 1, Triangle = 2, CatmullRom = 3, Gaussian = 4, Lanczos3 = 5
 * @param {PhotonImage} photon_img
 * @param {number} width
 * @param {number} height
 * @param {SamplingFilter} sampling_filter
 * @returns {HTMLCanvasElement}
 */
function resize_img_browser(photon_img, width, height, sampling_filter) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.resize_img_browser(photon_img.__wbg_ptr, width, height, sampling_filter);
    return ret;
}
exports.resize_img_browser = resize_img_browser;

/**
 * Rotate the PhotonImage on an arbitrary angle
 * A rotated PhotonImage is returned.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `angle` - Rotation angle in degrees.
 *
 * # Example
 *
 * ```no_run
 * // For example, to rotate a PhotonImage by 30 degrees:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::rotate;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let rotated_img = rotate(&img, 30.0);
 * ```
 * @param {PhotonImage} photon_img
 * @param {number} angle
 * @returns {PhotonImage}
 */
function rotate(photon_img, angle) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.rotate(photon_img.__wbg_ptr, angle);
    return PhotonImage.__wrap(ret);
}
exports.rotate = rotate;

/**
 * ! [temp] Check if WASM is supported.
 */
function run() {
    const ret = wasm.run();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}
exports.run = run;

/**
 * Solarization on the Red and Blue channels.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * use photon_rs::filters::ryo;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * ryo(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function ryo(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.ryo(photon_image.__wbg_ptr);
}
exports.ryo = ryo;

/**
 * Increase the image's saturation by converting each pixel's colour to the HSL colour space
 * and increasing the colour's saturation.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Increasing saturation by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to increase saturation by 10% in the HSL colour space:
 * use photon_rs::colour_spaces::saturate_hsl;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * saturate_hsl(&mut img, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function saturate_hsl(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_hsl(img.__wbg_ptr, level);
}
exports.saturate_hsl = saturate_hsl;

/**
 * Increase the image's saturation in the HSLuv colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Increasing saturation by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to increase saturation by 40% in the HSLuv colour space:
 * use photon_rs::colour_spaces::saturate_hsluv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * saturate_hsluv(&mut img, 0.4_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function saturate_hsluv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_hsluv(img.__wbg_ptr, level);
}
exports.saturate_hsluv = saturate_hsluv;

/**
 * Increase the image's saturation in the HSV colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level by which to increase the saturation by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Increasing saturation by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to increase saturation by 30% in the HSV colour space:
 * use photon_rs::colour_spaces::saturate_hsv;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * saturate_hsv(&mut img, 0.3_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function saturate_hsv(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_hsv(img.__wbg_ptr, level);
}
exports.saturate_hsv = saturate_hsv;

/**
 * Increase the image's saturation in the LCh colour space.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `level` - Float value from 0 to 1 representing the level to which to increase the saturation by.
 * The `level` must be from 0 to 1 in floating-point, `f32` format.
 * Increasing saturation by 80% would be represented by a `level` of 0.8
 *
 * # Example
 * ```no_run
 * // For example to increase saturation by 40% in the Lch colour space:
 * use photon_rs::colour_spaces::saturate_lch;
 * use photon_rs::native::open_image;
 *
 * // Open the image. A PhotonImage is returned.
 * let mut img = open_image("img.jpg").expect("File should open");
 * saturate_lch(&mut img, 0.4_f32);
 * ```
 * @param {PhotonImage} img
 * @param {number} level
 */
function saturate_lch(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_lch(img.__wbg_ptr, level);
}
exports.saturate_lch = saturate_lch;

/**
 * Resize image using seam carver.
 * Resize only if new dimensions are smaller, than original image.
 * # NOTE: This is still experimental feature, and pretty slow.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `width` - New width.
 * * `height` - New height.
 *
 * # Example
 *
 * ```no_run
 * // For example, resize image using seam carver:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::seam_carve;
 * use photon_rs::PhotonImage;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let result: PhotonImage = seam_carve(&img, 100_u32, 100_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} width
 * @param {number} height
 * @returns {PhotonImage}
 */
function seam_carve(img, width, height) {
    _assertClass(img, PhotonImage);
    const ret = wasm.seam_carve(img.__wbg_ptr, width, height);
    return PhotonImage.__wrap(ret);
}
exports.seam_carve = seam_carve;

/**
 * Selectively change pixel colours which are similar to the reference colour provided.
 *
 * Similarity between two colours is calculated via the CIE76 formula.
 * Only changes the color of a pixel if its similarity to the reference colour is within the range in the algorithm.
 * For example, with this function, a user can change the color of all blue pixels by mixing them with red by 10%.
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `new_color` - The `RGB` value of the new color (to be mixed with the matched pixels)
 * * `fraction` - The amount of mixing the new colour with the matched pixels
 *
 * # Example
 *
 * ```no_run
 * // For example, to only change the color of pixels that are similar to the RGB value RGB{200, 120, 30} by mixing RGB{30, 120, 200} with 25%:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_color_convert;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(200, 120, 30);
 * let new_color = Rgb::new(30, 120, 200);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_color_convert(&mut img, ref_color, new_color, 0.25);
 * ```
 * @param {PhotonImage} photon_image
 * @param {Rgb} ref_color
 * @param {Rgb} new_color
 * @param {number} fraction
 */
function selective_color_convert(photon_image, ref_color, new_color, fraction) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    _assertClass(new_color, Rgb);
    var ptr1 = new_color.__destroy_into_raw();
    wasm.selective_color_convert(photon_image.__wbg_ptr, ptr0, ptr1, fraction);
}
exports.selective_color_convert = selective_color_convert;

/**
 * Selectively desaturate pixel colours which are similar to the reference colour provided.
 *
 * Similarity between two colours is calculated via the CIE76 formula.
 * Only desaturates the hue of a pixel if its similarity to the reference colour is within the range in the algorithm.
 * For example, if a user wishes all pixels that are blue to be desaturated by 0.1, they can selectively specify  only the blue pixels to be changed.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `amt` - The amount of desaturate the colour by.
 *
 * # Example
 *
 * ```no_run
 * // For example, to only desaturate the pixels that are similar to the RGB value RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_desaturate;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_desaturate(&mut img, ref_color, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} ref_color
 * @param {number} amt
 */
function selective_desaturate(img, ref_color, amt) {
    _assertClass(img, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_desaturate(img.__wbg_ptr, ptr0, amt);
}
exports.selective_desaturate = selective_desaturate;

/**
 * Selectively changes a pixel to greyscale if it is *not* visually similar or close to the colour specified.
 * Only changes the colour of a pixel if its RGB values are within a specified range.
 *
 * (Similarity between two colours is calculated via the CIE76 formula.)
 * For example, if a user wishes all pixels that are *NOT* blue to be displayed in greyscale, they can selectively specify only the blue pixels to be
 * kept in the photo.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 *
 * # Example
 *
 * ```no_run
 * // For example, to greyscale all pixels that are *not* visually similar to the RGB colour RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_greyscale;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_greyscale(img, ref_color);
 * ```
 * @param {PhotonImage} photon_image
 * @param {Rgb} ref_color
 */
function selective_greyscale(photon_image, ref_color) {
    _assertClass(photon_image, PhotonImage);
    var ptr0 = photon_image.__destroy_into_raw();
    _assertClass(ref_color, Rgb);
    var ptr1 = ref_color.__destroy_into_raw();
    wasm.selective_greyscale(ptr0, ptr1);
}
exports.selective_greyscale = selective_greyscale;

/**
 * Selective hue rotation.
 *
 * Only rotate the hue of a pixel if its RGB values are within a specified range.
 * This function only rotates a pixel's hue to another  if it is visually similar to the colour specified.
 * For example, if a user wishes all pixels that are blue to be changed to red, they can selectively specify  only the blue pixels to be changed.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `degrees` - The amount of degrees to hue rotate by.
 *
 * # Example
 *
 * ```no_run
 * // For example, to only rotate the pixels that are of RGB value RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_hue_rotate;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_hue_rotate(&mut img, ref_color, 180_f32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {Rgb} ref_color
 * @param {number} degrees
 */
function selective_hue_rotate(photon_image, ref_color, degrees) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_hue_rotate(photon_image.__wbg_ptr, ptr0, degrees);
}
exports.selective_hue_rotate = selective_hue_rotate;

/**
 * Selectively lighten an image.
 *
 * Only lighten the hue of a pixel if its colour matches or is similar to the RGB colour specified.
 * For example, if a user wishes all pixels that are blue to be lightened, they can selectively specify  only the blue pixels to be changed.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `amt` - The level from 0 to 1 to lighten the hue by. Increasing by 10% would have an `amt` of 0.1
 *
 * # Example
 *
 * ```no_run
 * // For example, to only lighten the pixels that are of or similar to RGB value RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_lighten;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_lighten(&mut img, ref_color, 0.2_f32);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} ref_color
 * @param {number} amt
 */
function selective_lighten(img, ref_color, amt) {
    _assertClass(img, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_lighten(img.__wbg_ptr, ptr0, amt);
}
exports.selective_lighten = selective_lighten;

/**
 * Selectively saturate pixel colours which are similar to the reference colour provided.
 *
 * Similarity between two colours is calculated via the CIE76 formula.
 * Only saturates the hue of a pixel if its similarity to the reference colour is within the range in the algorithm.
 * For example, if a user wishes all pixels that are blue to have an increase in saturation by 10%, they can selectively specify only the blue pixels to be changed.
 * # Arguments
 * * `img` - A PhotonImage.
 * * `ref_color` - The `RGB` value of the reference color (to be compared to)
 * * `amt` - The amount of saturate the colour by.
 *
 * # Example
 *
 * ```no_run
 * // For example, to only increase the saturation of pixels that are similar to the RGB value RGB{20, 40, 60}:
 * use photon_rs::Rgb;
 * use photon_rs::channels::selective_saturate;
 * use photon_rs::native::open_image;
 *
 * let ref_color = Rgb::new(20_u8, 40_u8, 60_u8);
 * let mut img = open_image("img.jpg").expect("File should open");
 * selective_saturate(&mut img, ref_color, 0.1_f32);
 * ```
 * @param {PhotonImage} img
 * @param {Rgb} ref_color
 * @param {number} amt
 */
function selective_saturate(img, ref_color, amt) {
    _assertClass(img, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_saturate(img.__wbg_ptr, ptr0, amt);
}
exports.selective_saturate = selective_saturate;

/**
 * Convert an image to sepia.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * # Example
 *
 * ```no_run
 * // For example, to sepia an image of type `PhotonImage`:
 * use photon_rs::monochrome::sepia;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sepia(&mut img);
 * ```
 * @param {PhotonImage} img
 */
function sepia(img) {
    _assertClass(img, PhotonImage);
    wasm.sepia(img.__wbg_ptr);
}
exports.sepia = sepia;

/**
 * Sharpen an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to sharpen an image:
 * use photon_rs::conv::sharpen;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sharpen(&mut img);
 * ```
 * Adds a constant to a select R, G, or B channel's value.
 * @param {PhotonImage} photon_image
 */
function sharpen(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sharpen(photon_image.__wbg_ptr);
}
exports.sharpen = sharpen;

/**
 * Shear the image along the X axis.
 * A sheared PhotonImage is returned.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `shear` - Amount to shear.
 *
 * # Example
 *
 * ```no_run
 * // For example, to shear an image by 0.5:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::shearx;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let sheared_img = shearx(&img, 0.5);
 * ```
 * @param {PhotonImage} photon_img
 * @param {number} shear
 * @returns {PhotonImage}
 */
function shearx(photon_img, shear) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.shearx(photon_img.__wbg_ptr, shear);
    return PhotonImage.__wrap(ret);
}
exports.shearx = shearx;

/**
 * Shear the image along the Y axis.
 * A sheared PhotonImage is returned.
 *
 * # Arguments
 * * `img` - A PhotonImage. See the PhotonImage struct for details.
 * * `shear` - Amount to shear.
 *
 * # Example
 *
 * ```no_run
 * // For example, to shear an image by 0.5:
 * use photon_rs::native::open_image;
 * use photon_rs::transform::sheary;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let sheared_img = sheary(&img, 0.5);
 * ```
 * @param {PhotonImage} photon_img
 * @param {number} shear
 * @returns {PhotonImage}
 */
function sheary(photon_img, shear) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.sheary(photon_img.__wbg_ptr, shear);
    return PhotonImage.__wrap(ret);
}
exports.sheary = sheary;

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to a chosen channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `channel` - A usize representing the channel from 0 to 2. O represents the Red channel, 1 the Green channel, and 2 the Blue channel.
 *
 * # Example
 * To grayscale using only values from the Red channel:
 * ```no_run
 * use photon_rs::monochrome::single_channel_grayscale;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * single_channel_grayscale(&mut img, 0_usize);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} channel
 */
function single_channel_grayscale(photon_image, channel) {
    _assertClass(photon_image, PhotonImage);
    wasm.single_channel_grayscale(photon_image.__wbg_ptr, channel);
}
exports.single_channel_grayscale = single_channel_grayscale;

/**
 * Apply a global Sobel filter to an image
 *
 * Each pixel is calculated as the magnitude of the horizontal and vertical components of the Sobel filter,
 * ie if X is the horizontal sobel and Y is the vertical, for each pixel, we calculate sqrt(X^2 + Y^2)
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a global Sobel filter:
 * use photon_rs::conv::sobel_global;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sobel_global(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function sobel_global(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sobel_global(photon_image.__wbg_ptr);
}
exports.sobel_global = sobel_global;

/**
 * Apply a horizontal Sobel filter to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a horizontal Sobel filter:
 * use photon_rs::conv::sobel_horizontal;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sobel_horizontal(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function sobel_horizontal(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sobel_horizontal(photon_image.__wbg_ptr);
}
exports.sobel_horizontal = sobel_horizontal;

/**
 * Apply a vertical Sobel filter to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 *
 * # Example
 *
 * ```no_run
 * // For example, to apply a vertical Sobel filter:
 * use photon_rs::conv::sobel_vertical;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * sobel_vertical(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function sobel_vertical(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sobel_vertical(photon_image.__wbg_ptr);
}
exports.sobel_vertical = sobel_vertical;

/**
 * Applies a solarizing effect to an image.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to colorize an image of type `PhotonImage`:
 * use photon_rs::effects::solarize;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * solarize(&mut img);
 * ```
 * @param {PhotonImage} photon_image
 */
function solarize(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.solarize(photon_image.__wbg_ptr);
}
exports.solarize = solarize;

/**
 * Applies a solarizing effect to an image and returns the resulting PhotonImage.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * # Example
 *
 * ```no_run
 * // For example, to solarize "retimg" an image of type `PhotonImage`:
 * use photon_rs::effects::solarize_retimg;
 * use photon_rs::native::open_image;
 * use photon_rs::PhotonImage;
 *
 * let img = open_image("img.jpg").expect("File should open");
 * let result: PhotonImage = solarize_retimg(&img);
 * ```
 * @param {PhotonImage} photon_image
 * @returns {PhotonImage}
 */
function solarize_retimg(photon_image) {
    _assertClass(photon_image, PhotonImage);
    const ret = wasm.solarize_retimg(photon_image.__wbg_ptr);
    return PhotonImage.__wrap(ret);
}
exports.solarize_retimg = solarize_retimg;

/**
 * Swap two channels.
 *
 * # Arguments
 * * `img` - A PhotonImage.
 * * `channel1` - An index from 0 to 2, representing the Red, Green or Blue channels respectively. Red would be represented by 0, Green by 1, and Blue by 2.
 * * `channel2` - An index from 0 to 2, representing the Red, Green or Blue channels respectively. Same as above.
 *
 * # Example
 *
 * ```no_run
 * // For example, to swap the values of the Red channel with the values of the Blue channel:
 * use photon_rs::channels::swap_channels;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * swap_channels(&mut img, 0_usize, 2_usize);
 * ```
 * @param {PhotonImage} img
 * @param {number} channel1
 * @param {number} channel2
 */
function swap_channels(img, channel1, channel2) {
    _assertClass(img, PhotonImage);
    wasm.swap_channels(img.__wbg_ptr, channel1, channel2);
}
exports.swap_channels = swap_channels;

/**
 * Threshold an image using a standard thresholding algorithm.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `threshold` - The amount the image should be thresholded by from 0 to 255.
 * # Example
 *
 * ```no_run
 * // For example, to threshold an image of type `PhotonImage`:
 * use photon_rs::monochrome::threshold;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * threshold(&mut img, 30_u32);
 * ```
 * @param {PhotonImage} img
 * @param {number} threshold
 */
function threshold(img, threshold) {
    _assertClass(img, PhotonImage);
    wasm.threshold(img.__wbg_ptr, threshold);
}
exports.threshold = threshold;

/**
 * Tint an image by adding an offset to averaged RGB channel values.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `r_offset` - The amount the R channel should be incremented by.
 * * `g_offset` - The amount the G channel should be incremented by.
 * * `b_offset` - The amount the B channel should be incremented by.
 * # Example
 *
 * ```no_run
 * // For example, to tint an image of type `PhotonImage`:
 * use photon_rs::effects::tint;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * tint(&mut img, 10_u32, 20_u32, 15_u32);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} r_offset
 * @param {number} g_offset
 * @param {number} b_offset
 */
function tint(photon_image, r_offset, g_offset, b_offset) {
    _assertClass(photon_image, PhotonImage);
    wasm.tint(photon_image.__wbg_ptr, r_offset, g_offset, b_offset);
}
exports.tint = tint;

/**
 * Convert a PhotonImage to JS-compatible ImageData.
 * @param {PhotonImage} photon_image
 * @returns {ImageData}
 */
function to_image_data(photon_image) {
    _assertClass(photon_image, PhotonImage);
    var ptr0 = photon_image.__destroy_into_raw();
    const ret = wasm.to_image_data(ptr0);
    return ret;
}
exports.to_image_data = to_image_data;

/**
 * Convert ImageData to a raw pixel vec of u8s.
 * @param {ImageData} imgdata
 * @returns {Uint8Array}
 */
function to_raw_pixels(imgdata) {
    const ret = wasm.to_raw_pixels(imgdata);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}
exports.to_raw_pixels = to_raw_pixels;

/**
 * Vertical strips. Divide an image into a series of equal-width strips, for an artistic effect.
 *
 * # Arguments
 * * `img` - A PhotonImage that contains a view into the image.
 * * `num_strips` - The numbder of strips
 * # Example
 *
 * ```no_run
 * // For example, to draw vertical strips on a `PhotonImage`:
 * use photon_rs::effects::vertical_strips;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * vertical_strips(&mut img, 8u8);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} num_strips
 */
function vertical_strips(photon_image, num_strips) {
    _assertClass(photon_image, PhotonImage);
    wasm.vertical_strips(photon_image.__wbg_ptr, num_strips);
}
exports.vertical_strips = vertical_strips;

/**
 * Apply a radial vignette effect to an image.
 *
 * # Example
 *
 * ```no_run
 * use photon_rs::effects::vignette;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * // Moderate vignette — corners darken by ~60 %
 * vignette(&mut img, 0.6);
 * ```
 * @param {PhotonImage} photon_image
 * @param {number} intensity
 */
function vignette(photon_image, intensity) {
    _assertClass(photon_image, PhotonImage);
    wasm.vignette(photon_image.__wbg_ptr, intensity);
}
exports.vignette = vignette;

/**
 * Add a watermark to an image.
 *
 * # Arguments
 * * `img` - A DynamicImage that contains a view into the image.
 * * `watermark` - The watermark to be placed onto the `img` image.
 * * `x` - The x coordinate where the watermark's top corner should be positioned.
 * * `y` - The y coordinate where the watermark's top corner should be positioned.
 * # Example
 *
 * ```no_run
 * // For example, to add a watermark to an image at x: 30, y: 40:
 * use photon_rs::multiple::watermark;
 * use photon_rs::native::open_image;
 *
 * let mut img = open_image("img.jpg").expect("File should open");
 * let water_mark = open_image("watermark.jpg").expect("File should open");
 * watermark(&mut img, &water_mark, 30_i64, 40_i64);
 * ```
 * @param {PhotonImage} img
 * @param {PhotonImage} watermark
 * @param {bigint} x
 * @param {bigint} y
 */
function watermark(img, watermark, x, y) {
    _assertClass(img, PhotonImage);
    _assertClass(watermark, PhotonImage);
    wasm.watermark(img.__wbg_ptr, watermark.__wbg_ptr, x, y);
}
exports.watermark = watermark;
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_debug_string_a57024b9c6e4a48b: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_undefined_6cff064c44e0d823: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_throw_bb96b2010945f0bc: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_appendChild_d5cbce3d5fa81471: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.appendChild(arg1);
            return ret;
        }, arguments); },
        __wbg_body_d6eca0586d628e3c: function(arg0) {
            const ret = arg0.body;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_createElement_7f42344eee7bb810: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_data_51774c2dcd0a0e9f: function(arg0, arg1) {
            const ret = arg1.data;
            const ptr1 = passArray8ToWasm0(ret, wasm.__wbindgen_malloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_document_ac38448dbfd31a57: function(arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_drawImage_426c25f11858e3bd: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.drawImage(arg1, arg2, arg3);
        }, arguments); },
        __wbg_drawImage_87a05b54f458ec06: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
            arg0.drawImage(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
        }, arguments); },
        __wbg_error_757e9472f8410341: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_getContext_71c33f14b63da593: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_getImageData_8700a8b301ed5f06: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            const ret = arg0.getImageData(arg1, arg2, arg3, arg4);
            return ret;
        }, arguments); },
        __wbg_height_89544b3a2329e1e4: function(arg0) {
            const ret = arg0.height;
            return ret;
        },
        __wbg_height_b0594a7850e20673: function(arg0) {
            const ret = arg0.height;
            return ret;
        },
        __wbg_height_e56f6fb197710e09: function(arg0) {
            const ret = arg0.height;
            return ret;
        },
        __wbg_instanceof_CanvasRenderingContext2d_d23139c3ef7651a3: function(arg0) {
            let result;
            try {
                result = arg0 instanceof CanvasRenderingContext2D;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlCanvasElement_327e7f7530c72bbd: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLCanvasElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_5625ff9937037a38: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_length_36bd29c6848c2144: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_77cc4f4f472aeb81: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_new_with_u8_clamped_array_and_sh_d9a3bf9abac17f51: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
            return ret;
        }, arguments); },
        __wbg_prototypesetcall_de8e0d9553586985: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_putImageData_17fd10517d5503a3: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.putImageData(arg1, arg2, arg3);
        }, arguments); },
        __wbg_random_b0d98802be10ff20: function() {
            const ret = Math.random();
            return ret;
        },
        __wbg_set_height_d72f2b76484a44de: function(arg0, arg1) {
            arg0.height = arg1 >>> 0;
        },
        __wbg_set_textContent_e027901c7bc836b5: function(arg0, arg1, arg2) {
            arg0.textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_width_36ef6630b22fc519: function(arg0, arg1) {
            arg0.width = arg1 >>> 0;
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_static_accessor_GLOBAL_THIS_466428f93b4eaa76: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_c7aea38d4de089bc: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_42d4fae05e59267a: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_e0db14a0eba6a812: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_width_1952934caca67137: function(arg0) {
            const ret = arg0.width;
            return ret;
        },
        __wbg_width_4bb073b449891b57: function(arg0) {
            const ret = arg0.width;
            return ret;
        },
        __wbg_width_534fef037caa5f9b: function(arg0) {
            const ret = arg0.width;
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./photon-node_bg.js": import0,
    };
}

const PhotonImageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_photonimage_free(ptr, 1));
const RgbFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rgb_free(ptr, 1));
const RgbaFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rgba_free(ptr, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedUint8ClampedArrayMemory0 = null;
function getUint8ClampedArrayMemory0() {
    if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.byteLength === 0) {
        cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

const wasmPath = `${__dirname}/photon-node_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
let wasmInstance = new WebAssembly.Instance(wasmModule, __wbg_get_imports());
let wasm = wasmInstance.exports;
wasm.__wbindgen_start();
