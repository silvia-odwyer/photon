
let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
const { TextDecoder, TextEncoder } = require(`util`);

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
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

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedUint8ClampedArrayMemory0 = null;

function getUint8ClampedArrayMemory0() {
    if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.byteLength === 0) {
        cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedArrayMemory0;
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

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
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
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

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
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
module.exports.noise_reduction = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.noise_reduction(photon_image.__wbg_ptr);
};

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
module.exports.sharpen = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sharpen(photon_image.__wbg_ptr);
};

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
module.exports.edge_detection = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.edge_detection(photon_image.__wbg_ptr);
};

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
module.exports.identity = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.identity(photon_image.__wbg_ptr);
};

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
module.exports.box_blur = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.box_blur(photon_image.__wbg_ptr);
};

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
module.exports.gaussian_blur = function(photon_image, radius) {
    _assertClass(photon_image, PhotonImage);
    wasm.gaussian_blur(photon_image.__wbg_ptr, radius);
};

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
module.exports.detect_horizontal_lines = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_horizontal_lines(photon_image.__wbg_ptr);
};

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
module.exports.detect_vertical_lines = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_vertical_lines(photon_image.__wbg_ptr);
};

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
module.exports.detect_45_deg_lines = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_45_deg_lines(photon_image.__wbg_ptr);
};

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
module.exports.detect_135_deg_lines = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.detect_135_deg_lines(photon_image.__wbg_ptr);
};

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
module.exports.laplace = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.laplace(photon_image.__wbg_ptr);
};

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
module.exports.edge_one = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.edge_one(photon_image.__wbg_ptr);
};

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
module.exports.emboss = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.emboss(photon_image.__wbg_ptr);
};

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
module.exports.sobel_horizontal = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sobel_horizontal(photon_image.__wbg_ptr);
};

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
module.exports.prewitt_horizontal = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.prewitt_horizontal(photon_image.__wbg_ptr);
};

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
module.exports.sobel_vertical = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sobel_vertical(photon_image.__wbg_ptr);
};

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
module.exports.sobel_global = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.sobel_global(photon_image.__wbg_ptr);
};

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
module.exports.watermark = function(img, watermark, x, y) {
    _assertClass(img, PhotonImage);
    _assertClass(watermark, PhotonImage);
    wasm.watermark(img.__wbg_ptr, watermark.__wbg_ptr, x, y);
};

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
module.exports.blend = function(photon_image, photon_image2, blend_mode) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(photon_image2, PhotonImage);
    const ptr0 = passStringToWasm0(blend_mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.blend(photon_image.__wbg_ptr, photon_image2.__wbg_ptr, ptr0, len0);
};

/**
 * @param {number} width
 * @param {number} height
 * @returns {PhotonImage}
 */
module.exports.create_gradient = function(width, height) {
    const ret = wasm.create_gradient(width, height);
    return PhotonImage.__wrap(ret);
};

/**
 * Apply a gradient to an image.
 * @param {PhotonImage} image
 */
module.exports.apply_gradient = function(image) {
    _assertClass(image, PhotonImage);
    wasm.apply_gradient(image.__wbg_ptr);
};

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
module.exports.offset = function(photon_image, channel_index, offset) {
    _assertClass(photon_image, PhotonImage);
    wasm.offset(photon_image.__wbg_ptr, channel_index, offset);
};

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
module.exports.offset_red = function(img, offset_amt) {
    _assertClass(img, PhotonImage);
    wasm.offset_red(img.__wbg_ptr, offset_amt);
};

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
module.exports.offset_green = function(img, offset_amt) {
    _assertClass(img, PhotonImage);
    wasm.offset_green(img.__wbg_ptr, offset_amt);
};

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
module.exports.offset_blue = function(img, offset_amt) {
    _assertClass(img, PhotonImage);
    wasm.offset_blue(img.__wbg_ptr, offset_amt);
};

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
module.exports.multiple_offsets = function(photon_image, offset, channel_index, channel_index2) {
    _assertClass(photon_image, PhotonImage);
    wasm.multiple_offsets(photon_image.__wbg_ptr, offset, channel_index, channel_index2);
};

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
module.exports.halftone = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.halftone(photon_image.__wbg_ptr);
};

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
module.exports.primary = function(img) {
    _assertClass(img, PhotonImage);
    wasm.primary(img.__wbg_ptr);
};

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
module.exports.colorize = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.colorize(photon_image.__wbg_ptr);
};

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
module.exports.solarize = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.solarize(photon_image.__wbg_ptr);
};

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
module.exports.solarize_retimg = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    const ret = wasm.solarize_retimg(photon_image.__wbg_ptr);
    return PhotonImage.__wrap(ret);
};

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
module.exports.adjust_brightness = function(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.adjust_brightness(photon_image.__wbg_ptr, brightness);
};

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
module.exports.inc_brightness = function(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.inc_brightness(photon_image.__wbg_ptr, brightness);
};

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
module.exports.dec_brightness = function(photon_image, brightness) {
    _assertClass(photon_image, PhotonImage);
    wasm.dec_brightness(photon_image.__wbg_ptr, brightness);
};

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
module.exports.adjust_contrast = function(photon_image, contrast) {
    _assertClass(photon_image, PhotonImage);
    wasm.adjust_contrast(photon_image.__wbg_ptr, contrast);
};

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
module.exports.tint = function(photon_image, r_offset, g_offset, b_offset) {
    _assertClass(photon_image, PhotonImage);
    wasm.tint(photon_image.__wbg_ptr, r_offset, g_offset, b_offset);
};

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
module.exports.horizontal_strips = function(photon_image, num_strips) {
    _assertClass(photon_image, PhotonImage);
    wasm.horizontal_strips(photon_image.__wbg_ptr, num_strips);
};

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
module.exports.color_horizontal_strips = function(photon_image, num_strips, color) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color, Rgb);
    var ptr0 = color.__destroy_into_raw();
    wasm.color_horizontal_strips(photon_image.__wbg_ptr, num_strips, ptr0);
};

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
module.exports.vertical_strips = function(photon_image, num_strips) {
    _assertClass(photon_image, PhotonImage);
    wasm.vertical_strips(photon_image.__wbg_ptr, num_strips);
};

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
module.exports.color_vertical_strips = function(photon_image, num_strips, color) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color, Rgb);
    var ptr0 = color.__destroy_into_raw();
    wasm.color_vertical_strips(photon_image.__wbg_ptr, num_strips, ptr0);
};

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
module.exports.oil = function(photon_image, radius, intensity) {
    _assertClass(photon_image, PhotonImage);
    wasm.oil(photon_image.__wbg_ptr, radius, intensity);
};

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
module.exports.frosted_glass = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.frosted_glass(photon_image.__wbg_ptr);
};

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
module.exports.pixelize = function(photon_image, pixel_size) {
    _assertClass(photon_image, PhotonImage);
    wasm.pixelize(photon_image.__wbg_ptr, pixel_size);
};

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
module.exports.normalize = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.normalize(photon_image.__wbg_ptr);
};

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
module.exports.dither = function(photon_image, depth) {
    _assertClass(photon_image, PhotonImage);
    wasm.dither(photon_image.__wbg_ptr, depth);
};

/**
 * @param {PhotonImage} photon_image
 * @param {Rgb} color_a
 * @param {Rgb} color_b
 */
module.exports.duotone = function(photon_image, color_a, color_b) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color_a, Rgb);
    var ptr0 = color_a.__destroy_into_raw();
    _assertClass(color_b, Rgb);
    var ptr1 = color_b.__destroy_into_raw();
    wasm.duotone(photon_image.__wbg_ptr, ptr0, ptr1);
};

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
module.exports.add_noise_rand = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.add_noise_rand(photon_image.__wbg_ptr);
};

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
module.exports.pink_noise = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.pink_noise(photon_image.__wbg_ptr);
};

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
module.exports.crop = function(photon_image, x1, y1, x2, y2) {
    _assertClass(photon_image, PhotonImage);
    const ret = wasm.crop(photon_image.__wbg_ptr, x1, y1, x2, y2);
    return PhotonImage.__wrap(ret);
};

/**
 * @param {HTMLCanvasElement} source_canvas
 * @param {number} width
 * @param {number} height
 * @param {number} left
 * @param {number} top
 * @returns {HTMLCanvasElement}
 */
module.exports.crop_img_browser = function(source_canvas, width, height, left, top) {
    const ret = wasm.crop_img_browser(source_canvas, width, height, left, top);
    return ret;
};

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
module.exports.fliph = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.fliph(photon_image.__wbg_ptr);
};

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
module.exports.flipv = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.flipv(photon_image.__wbg_ptr);
};

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
module.exports.resize_img_browser = function(photon_img, width, height, sampling_filter) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.resize_img_browser(photon_img.__wbg_ptr, width, height, sampling_filter);
    return ret;
};

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
module.exports.resize = function(photon_img, width, height, sampling_filter) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.resize(photon_img.__wbg_ptr, width, height, sampling_filter);
    return PhotonImage.__wrap(ret);
};

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
module.exports.seam_carve = function(img, width, height) {
    _assertClass(img, PhotonImage);
    const ret = wasm.seam_carve(img.__wbg_ptr, width, height);
    return PhotonImage.__wrap(ret);
};

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
module.exports.shearx = function(photon_img, shear) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.shearx(photon_img.__wbg_ptr, shear);
    return PhotonImage.__wrap(ret);
};

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
module.exports.sheary = function(photon_img, shear) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.sheary(photon_img.__wbg_ptr, shear);
    return PhotonImage.__wrap(ret);
};

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
module.exports.padding_uniform = function(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_uniform(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
};

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
module.exports.padding_left = function(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_left(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
};

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
module.exports.padding_right = function(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_right(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
};

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
module.exports.padding_top = function(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_top(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
};

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
module.exports.padding_bottom = function(img, padding, padding_rgba) {
    _assertClass(img, PhotonImage);
    _assertClass(padding_rgba, Rgba);
    var ptr0 = padding_rgba.__destroy_into_raw();
    const ret = wasm.padding_bottom(img.__wbg_ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
};

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
module.exports.rotate = function(photon_img, angle) {
    _assertClass(photon_img, PhotonImage);
    const ret = wasm.rotate(photon_img.__wbg_ptr, angle);
    return PhotonImage.__wrap(ret);
};

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
module.exports.resample = function(img, dst_width, dst_height) {
    _assertClass(img, PhotonImage);
    const ret = wasm.resample(img.__wbg_ptr, dst_width, dst_height);
    return PhotonImage.__wrap(ret);
};

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
module.exports.gamma_correction = function(photon_image, red, green, blue) {
    _assertClass(photon_image, PhotonImage);
    wasm.gamma_correction(photon_image.__wbg_ptr, red, green, blue);
};

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
module.exports.hsluv = function(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsluv(photon_image.__wbg_ptr, ptr0, len0, amt);
};

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
module.exports.lch = function(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.lch(photon_image.__wbg_ptr, ptr0, len0, amt);
};

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
module.exports.hsl = function(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsl(photon_image.__wbg_ptr, ptr0, len0, amt);
};

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
module.exports.hsv = function(photon_image, mode, amt) {
    _assertClass(photon_image, PhotonImage);
    const ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.hsv(photon_image.__wbg_ptr, ptr0, len0, amt);
};

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
module.exports.hue_rotate_hsl = function(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_hsl(img.__wbg_ptr, degrees);
};

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
module.exports.hue_rotate_hsv = function(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_hsv(img.__wbg_ptr, degrees);
};

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
module.exports.hue_rotate_lch = function(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_lch(img.__wbg_ptr, degrees);
};

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
module.exports.hue_rotate_hsluv = function(img, degrees) {
    _assertClass(img, PhotonImage);
    wasm.hue_rotate_hsluv(img.__wbg_ptr, degrees);
};

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
module.exports.saturate_hsl = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_hsl(img.__wbg_ptr, level);
};

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
module.exports.saturate_lch = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_lch(img.__wbg_ptr, level);
};

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
module.exports.saturate_hsluv = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_hsluv(img.__wbg_ptr, level);
};

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
module.exports.saturate_hsv = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.saturate_hsv(img.__wbg_ptr, level);
};

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
module.exports.lighten_lch = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_lch(img.__wbg_ptr, level);
};

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
module.exports.lighten_hsluv = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_hsluv(img.__wbg_ptr, level);
};

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
module.exports.lighten_hsl = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_hsl(img.__wbg_ptr, level);
};

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
module.exports.lighten_hsv = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.lighten_hsv(img.__wbg_ptr, level);
};

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
module.exports.darken_lch = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_lch(img.__wbg_ptr, level);
};

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
module.exports.darken_hsluv = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_hsluv(img.__wbg_ptr, level);
};

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
module.exports.darken_hsl = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_hsl(img.__wbg_ptr, level);
};

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
module.exports.darken_hsv = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.darken_hsv(img.__wbg_ptr, level);
};

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
module.exports.desaturate_hsv = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_hsv(img.__wbg_ptr, level);
};

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
module.exports.desaturate_hsl = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_hsl(img.__wbg_ptr, level);
};

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
module.exports.desaturate_lch = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_lch(img.__wbg_ptr, level);
};

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
module.exports.desaturate_hsluv = function(img, level) {
    _assertClass(img, PhotonImage);
    wasm.desaturate_hsluv(img.__wbg_ptr, level);
};

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
module.exports.mix_with_colour = function(photon_image, mix_colour, opacity) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(mix_colour, Rgb);
    var ptr0 = mix_colour.__destroy_into_raw();
    wasm.mix_with_colour(photon_image.__wbg_ptr, ptr0, opacity);
};

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
module.exports.monochrome = function(img, r_offset, g_offset, b_offset) {
    _assertClass(img, PhotonImage);
    wasm.monochrome(img.__wbg_ptr, r_offset, g_offset, b_offset);
};

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
module.exports.sepia = function(img) {
    _assertClass(img, PhotonImage);
    wasm.sepia(img.__wbg_ptr);
};

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
module.exports.grayscale = function(img) {
    _assertClass(img, PhotonImage);
    wasm.grayscale(img.__wbg_ptr);
};

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
module.exports.grayscale_human_corrected = function(img) {
    _assertClass(img, PhotonImage);
    wasm.grayscale_human_corrected(img.__wbg_ptr);
};

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
module.exports.desaturate = function(img) {
    _assertClass(img, PhotonImage);
    wasm.desaturate(img.__wbg_ptr);
};

/**
 * Uses a min. decomposition algorithm to convert an image to greyscale.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
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
module.exports.decompose_min = function(img) {
    _assertClass(img, PhotonImage);
    wasm.decompose_min(img.__wbg_ptr);
};

/**
 * Uses a max. decomposition algorithm to convert an image to greyscale.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
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
module.exports.decompose_max = function(img) {
    _assertClass(img, PhotonImage);
    wasm.decompose_max(img.__wbg_ptr);
};

/**
 * Employ only a limited number of gray shades in an image.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `num_shades` - The number of grayscale shades to be displayed in the image.
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
module.exports.grayscale_shades = function(photon_image, num_shades) {
    _assertClass(photon_image, PhotonImage);
    wasm.grayscale_shades(photon_image.__wbg_ptr, num_shades);
};

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to the Red channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
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
module.exports.r_grayscale = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.r_grayscale(photon_image.__wbg_ptr);
};

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to the Green channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
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
module.exports.g_grayscale = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.g_grayscale(photon_image.__wbg_ptr);
};

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to the Blue channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
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
module.exports.b_grayscale = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.b_grayscale(photon_image.__wbg_ptr);
};

/**
 * Convert an image to grayscale by setting a pixel's 3 RGB values to a chosen channel's value.
 *
 * # Arguments
 * * `photon_image` - A PhotonImage.
 * * `channel` - A usize representing the channel from 0 to 2. O represents the Red channel, 1 the Green channel, and 2 the Blue channel.
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
module.exports.single_channel_grayscale = function(photon_image, channel) {
    _assertClass(photon_image, PhotonImage);
    wasm.single_channel_grayscale(photon_image.__wbg_ptr, channel);
};

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
module.exports.threshold = function(img, threshold) {
    _assertClass(img, PhotonImage);
    wasm.threshold(img.__wbg_ptr, threshold);
};

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
 * draw_text_with_border(&mut img, "Welcome to Photon!", 10_i32, 10_i32);
 * ```
 * @param {PhotonImage} photon_img
 * @param {string} text
 * @param {number} x
 * @param {number} y
 */
module.exports.draw_text_with_border = function(photon_img, text, x, y) {
    _assertClass(photon_img, PhotonImage);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.draw_text_with_border(photon_img.__wbg_ptr, ptr0, len0, x, y);
};

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
 * draw_text(&mut img, "Welcome to Photon!", 10_i32, 10_i32);
 * ```
 * @param {PhotonImage} photon_img
 * @param {string} text
 * @param {number} x
 * @param {number} y
 */
module.exports.draw_text = function(photon_img, text, x, y) {
    _assertClass(photon_img, PhotonImage);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.draw_text(photon_img.__wbg_ptr, ptr0, len0, x, y);
};

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
module.exports.neue = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.neue(photon_image.__wbg_ptr);
};

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
module.exports.lix = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.lix(photon_image.__wbg_ptr);
};

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
module.exports.ryo = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.ryo(photon_image.__wbg_ptr);
};

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
module.exports.filter = function(img, filter_name) {
    _assertClass(img, PhotonImage);
    const ptr0 = passStringToWasm0(filter_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.filter(img.__wbg_ptr, ptr0, len0);
};

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
module.exports.lofi = function(img) {
    _assertClass(img, PhotonImage);
    wasm.lofi(img.__wbg_ptr);
};

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
module.exports.pastel_pink = function(img) {
    _assertClass(img, PhotonImage);
    wasm.pastel_pink(img.__wbg_ptr);
};

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
module.exports.golden = function(img) {
    _assertClass(img, PhotonImage);
    wasm.golden(img.__wbg_ptr);
};

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
module.exports.cali = function(img) {
    _assertClass(img, PhotonImage);
    wasm.cali(img.__wbg_ptr);
};

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
module.exports.dramatic = function(img) {
    _assertClass(img, PhotonImage);
    wasm.dramatic(img.__wbg_ptr);
};

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
module.exports.monochrome_tint = function(img, rgb_color) {
    _assertClass(img, PhotonImage);
    _assertClass(rgb_color, Rgb);
    var ptr0 = rgb_color.__destroy_into_raw();
    wasm.monochrome_tint(img.__wbg_ptr, ptr0);
};

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
module.exports.duotone_violette = function(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_violette(img.__wbg_ptr);
};

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
module.exports.duotone_horizon = function(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_horizon(img.__wbg_ptr);
};

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
module.exports.duotone_tint = function(img, rgb_color) {
    _assertClass(img, PhotonImage);
    _assertClass(rgb_color, Rgb);
    var ptr0 = rgb_color.__destroy_into_raw();
    wasm.duotone_tint(img.__wbg_ptr, ptr0);
};

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
module.exports.duotone_lilac = function(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_lilac(img.__wbg_ptr);
};

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
module.exports.duotone_ochre = function(img) {
    _assertClass(img, PhotonImage);
    wasm.duotone_ochre(img.__wbg_ptr);
};

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
module.exports.firenze = function(img) {
    _assertClass(img, PhotonImage);
    wasm.firenze(img.__wbg_ptr);
};

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
module.exports.obsidian = function(img) {
    _assertClass(img, PhotonImage);
    wasm.obsidian(img.__wbg_ptr);
};

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
module.exports.alter_channel = function(img, channel, amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_channel(img.__wbg_ptr, channel, amt);
};

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
module.exports.alter_red_channel = function(photon_image, amt) {
    _assertClass(photon_image, PhotonImage);
    wasm.alter_red_channel(photon_image.__wbg_ptr, amt);
};

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
module.exports.alter_green_channel = function(img, amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_green_channel(img.__wbg_ptr, amt);
};

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
module.exports.alter_blue_channel = function(img, amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_blue_channel(img.__wbg_ptr, amt);
};

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
module.exports.alter_two_channels = function(img, channel1, amt1, channel2, amt2) {
    _assertClass(img, PhotonImage);
    wasm.alter_two_channels(img.__wbg_ptr, channel1, amt1, channel2, amt2);
};

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
module.exports.alter_channels = function(img, r_amt, g_amt, b_amt) {
    _assertClass(img, PhotonImage);
    wasm.alter_channels(img.__wbg_ptr, r_amt, g_amt, b_amt);
};

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
module.exports.remove_channel = function(img, channel, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_channel(img.__wbg_ptr, channel, min_filter);
};

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
module.exports.remove_red_channel = function(img, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_red_channel(img.__wbg_ptr, min_filter);
};

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
module.exports.remove_green_channel = function(img, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_green_channel(img.__wbg_ptr, min_filter);
};

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
module.exports.remove_blue_channel = function(img, min_filter) {
    _assertClass(img, PhotonImage);
    wasm.remove_blue_channel(img.__wbg_ptr, min_filter);
};

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
module.exports.swap_channels = function(img, channel1, channel2) {
    _assertClass(img, PhotonImage);
    wasm.swap_channels(img.__wbg_ptr, channel1, channel2);
};

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
module.exports.invert = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    wasm.invert(photon_image.__wbg_ptr);
};

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
module.exports.selective_hue_rotate = function(photon_image, ref_color, degrees) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_hue_rotate(photon_image.__wbg_ptr, ptr0, degrees);
};

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
module.exports.selective_color_convert = function(photon_image, ref_color, new_color, fraction) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    _assertClass(new_color, Rgb);
    var ptr1 = new_color.__destroy_into_raw();
    wasm.selective_color_convert(photon_image.__wbg_ptr, ptr0, ptr1, fraction);
};

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
module.exports.selective_lighten = function(img, ref_color, amt) {
    _assertClass(img, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_lighten(img.__wbg_ptr, ptr0, amt);
};

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
module.exports.selective_desaturate = function(img, ref_color, amt) {
    _assertClass(img, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_desaturate(img.__wbg_ptr, ptr0, amt);
};

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
module.exports.selective_saturate = function(img, ref_color, amt) {
    _assertClass(img, PhotonImage);
    _assertClass(ref_color, Rgb);
    var ptr0 = ref_color.__destroy_into_raw();
    wasm.selective_saturate(img.__wbg_ptr, ptr0, amt);
};

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
module.exports.selective_greyscale = function(photon_image, ref_color) {
    _assertClass(photon_image, PhotonImage);
    var ptr0 = photon_image.__destroy_into_raw();
    _assertClass(ref_color, Rgb);
    var ptr1 = ref_color.__destroy_into_raw();
    wasm.selective_greyscale(ptr0, ptr1);
};

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_2.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
/**
 * ! [temp] Check if WASM is supported.
 */
module.exports.run = function() {
    const ret = wasm.run();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
};

/**
 * Get the ImageData from a 2D canvas context
 * @param {HTMLCanvasElement} canvas
 * @param {CanvasRenderingContext2D} ctx
 * @returns {ImageData}
 */
module.exports.get_image_data = function(canvas, ctx) {
    const ret = wasm.get_image_data(canvas, ctx);
    return ret;
};

/**
 * Place a PhotonImage onto a 2D canvas.
 * @param {HTMLCanvasElement} canvas
 * @param {CanvasRenderingContext2D} ctx
 * @param {PhotonImage} new_image
 */
module.exports.putImageData = function(canvas, ctx, new_image) {
    _assertClass(new_image, PhotonImage);
    var ptr0 = new_image.__destroy_into_raw();
    wasm.putImageData(canvas, ctx, ptr0);
};

/**
 * Convert a HTML5 Canvas Element to a PhotonImage.
 *
 * This converts the ImageData found in the canvas context to a PhotonImage,
 * which can then have effects or filters applied to it.
 * @param {HTMLCanvasElement} canvas
 * @param {CanvasRenderingContext2D} ctx
 * @returns {PhotonImage}
 */
module.exports.open_image = function(canvas, ctx) {
    const ret = wasm.open_image(canvas, ctx);
    return PhotonImage.__wrap(ret);
};

/**
 * Convert ImageData to a raw pixel vec of u8s.
 * @param {ImageData} imgdata
 * @returns {Uint8Array}
 */
module.exports.to_raw_pixels = function(imgdata) {
    const ret = wasm.to_raw_pixels(imgdata);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
};

/**
 * Convert a base64 string to a PhotonImage.
 * @param {string} base64
 * @returns {PhotonImage}
 */
module.exports.base64_to_image = function(base64) {
    const ptr0 = passStringToWasm0(base64, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.base64_to_image(ptr0, len0);
    return PhotonImage.__wrap(ret);
};

/**
 * Convert a base64 string to a Vec of u8s.
 * @param {string} base64
 * @returns {Uint8Array}
 */
module.exports.base64_to_vec = function(base64) {
    const ptr0 = passStringToWasm0(base64, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.base64_to_vec(ptr0, len0);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
};

/**
 * Convert a PhotonImage to JS-compatible ImageData.
 * @param {PhotonImage} photon_image
 * @returns {ImageData}
 */
module.exports.to_image_data = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    var ptr0 = photon_image.__destroy_into_raw();
    const ret = wasm.to_image_data(ptr0);
    return ret;
};

/**
 * @enum {1 | 2 | 3 | 4 | 5}
 */
module.exports.SamplingFilter = Object.freeze({
    Nearest: 1, "1": "Nearest",
    Triangle: 2, "2": "Triangle",
    CatmullRom: 3, "3": "CatmullRom",
    Gaussian: 4, "4": "Gaussian",
    Lanczos3: 5, "5": "Lanczos3",
});

const PhotonImageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_photonimage_free(ptr >>> 0, 1));
/**
 * Provides the image's height, width, and contains the image's raw pixels.
 * For use when communicating between JS and WASM, and also natively.
 */
class PhotonImage {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
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
     * Create a new PhotonImage from a Vec of u8s, which represent raw pixels.
     * @param {Uint8Array} raw_pixels
     * @param {number} width
     * @param {number} height
     */
    constructor(raw_pixels, width, height) {
        const ptr0 = passArray8ToWasm0(raw_pixels, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.photonimage_new(ptr0, len0, width, height);
        this.__wbg_ptr = ret >>> 0;
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
        const ret = wasm.base64_to_image(ptr0, len0);
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
     * Create a new PhotonImage from a Blob/File.
     * @param {Blob} blob
     * @returns {PhotonImage}
     */
    static new_from_blob(blob) {
        const ret = wasm.photonimage_new_from_blob(blob);
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
     * Get the width of the PhotonImage.
     * @returns {number}
     */
    get_width() {
        const ret = wasm.photonimage_get_width(this.__wbg_ptr);
        return ret >>> 0;
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
     * Get the height of the PhotonImage.
     * @returns {number}
     */
    get_height() {
        const ret = wasm.photonimage_get_height(this.__wbg_ptr);
        return ret >>> 0;
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
     * Convert the PhotonImage's raw pixels to JS-compatible ImageData.
     * @returns {ImageData}
     */
    get_image_data() {
        const ret = wasm.photonimage_get_image_data(this.__wbg_ptr);
        return ret;
    }
    /**
     * Convert ImageData to raw pixels, and update the PhotonImage's raw pixels to this.
     * @param {ImageData} img_data
     */
    set_imgdata(img_data) {
        wasm.photonimage_set_imgdata(this.__wbg_ptr, img_data);
    }
    /**
     * Calculates estimated filesize and returns number of bytes
     * @returns {bigint}
     */
    get_estimated_filesize() {
        const ret = wasm.photonimage_get_estimated_filesize(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
}
module.exports.PhotonImage = PhotonImage;

const RgbFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rgb_free(ptr >>> 0, 1));
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
     * Create a new RGB struct.
     * @param {number} r
     * @param {number} g
     * @param {number} b
     */
    constructor(r, g, b) {
        const ret = wasm.rgb_new(r, g, b);
        this.__wbg_ptr = ret >>> 0;
        RgbFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Set the Red value.
     * @param {number} r
     */
    set_red(r) {
        wasm.rgb_set_red(this.__wbg_ptr, r);
    }
    /**
     * Get the Green value.
     * @param {number} g
     */
    set_green(g) {
        wasm.rgb_set_green(this.__wbg_ptr, g);
    }
    /**
     * Set the Blue value.
     * @param {number} b
     */
    set_blue(b) {
        wasm.rgb_set_blue(this.__wbg_ptr, b);
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
     * Get the Green value.
     * @returns {number}
     */
    get_green() {
        const ret = wasm.rgb_get_green(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the Blue value.
     * @returns {number}
     */
    get_blue() {
        const ret = wasm.rgb_get_blue(this.__wbg_ptr);
        return ret;
    }
}
module.exports.Rgb = Rgb;

const RgbaFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rgba_free(ptr >>> 0, 1));
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
     * Create a new RGBA struct.
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @param {number} a
     */
    constructor(r, g, b, a) {
        const ret = wasm.rgba_new(r, g, b, a);
        this.__wbg_ptr = ret >>> 0;
        RgbaFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Set the Red value.
     * @param {number} r
     */
    set_red(r) {
        wasm.rgb_set_red(this.__wbg_ptr, r);
    }
    /**
     * Get the Green value.
     * @param {number} g
     */
    set_green(g) {
        wasm.rgb_set_green(this.__wbg_ptr, g);
    }
    /**
     * Set the Blue value.
     * @param {number} b
     */
    set_blue(b) {
        wasm.rgb_set_blue(this.__wbg_ptr, b);
    }
    /**
     * Set the alpha value.
     * @param {number} a
     */
    set_alpha(a) {
        wasm.rgba_set_alpha(this.__wbg_ptr, a);
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
     * Get the Green value.
     * @returns {number}
     */
    get_green() {
        const ret = wasm.rgb_get_green(this.__wbg_ptr);
        return ret;
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
     * Get the alpha value for this color.
     * @returns {number}
     */
    get_alpha() {
        const ret = wasm.rgba_get_alpha(this.__wbg_ptr);
        return ret;
    }
}
module.exports.Rgba = Rgba;

module.exports.__wbg_appendChild_d22bc7af6b96b3f1 = function() { return handleError(function (arg0, arg1) {
    const ret = arg0.appendChild(arg1);
    return ret;
}, arguments) };

module.exports.__wbg_body_8d7d8c4aa91dcad8 = function(arg0) {
    const ret = arg0.body;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

module.exports.__wbg_buffer_61b7ce01341d7f88 = function(arg0) {
    const ret = arg0.buffer;
    return ret;
};

module.exports.__wbg_call_b0d8e36992d9900d = function() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };

module.exports.__wbg_createElement_89923fcb809656b7 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
    return ret;
}, arguments) };

module.exports.__wbg_data_3ba00521691343fc = function(arg0, arg1) {
    const ret = arg1.data;
    const ptr1 = passArray8ToWasm0(ret, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

module.exports.__wbg_document_f11bc4f7c03e1745 = function(arg0) {
    const ret = arg0.document;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

module.exports.__wbg_drawImage_93dc15f8b94cd4b6 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.drawImage(arg1, arg2, arg3);
}, arguments) };

module.exports.__wbg_drawImage_bd47fc9aff0589c5 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    arg0.drawImage(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
}, arguments) };

module.exports.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_getContext_5eaf5645cd6acb46 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments) };

module.exports.__wbg_getImageData_eb8a47512c21d5f8 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    const ret = arg0.getImageData(arg1, arg2, arg3, arg4);
    return ret;
}, arguments) };

module.exports.__wbg_height_578a43e02f182163 = function(arg0) {
    const ret = arg0.height;
    return ret;
};

module.exports.__wbg_height_a9968d3f0e288300 = function(arg0) {
    const ret = arg0.height;
    return ret;
};

module.exports.__wbg_height_f36c36e27347cf38 = function(arg0) {
    const ret = arg0.height;
    return ret;
};

module.exports.__wbg_instanceof_CanvasRenderingContext2d_23b21317d73228be = function(arg0) {
    let result;
    try {
        result = arg0 instanceof CanvasRenderingContext2D;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

module.exports.__wbg_instanceof_HtmlCanvasElement_f764441ef5ddb63f = function(arg0) {
    let result;
    try {
        result = arg0 instanceof HTMLCanvasElement;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

module.exports.__wbg_instanceof_Window_d2514c6a7ee7ba60 = function(arg0) {
    let result;
    try {
        result = arg0 instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

module.exports.__wbg_length_65d1cd11729ced11 = function(arg0) {
    const ret = arg0.length;
    return ret;
};

module.exports.__wbg_new_3ff5b33b1ce712df = function(arg0) {
    const ret = new Uint8Array(arg0);
    return ret;
};

module.exports.__wbg_new_8a6f238a6ece86ea = function() {
    const ret = new Error();
    return ret;
};

module.exports.__wbg_newnoargs_fd9e4bf8be2bc16d = function(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return ret;
};

module.exports.__wbg_newwithu8clampedarrayandsh_9792f5d240418183 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
    return ret;
}, arguments) };

module.exports.__wbg_putImageData_32b5cf733605df83 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    arg0.putImageData(arg1, arg2, arg3);
}, arguments) };

module.exports.__wbg_set_23d69db4e5c66a6e = function(arg0, arg1, arg2) {
    arg0.set(arg1, arg2 >>> 0);
};

module.exports.__wbg_setheight_16d76e7fa9d506ea = function(arg0, arg1) {
    arg0.height = arg1 >>> 0;
};

module.exports.__wbg_settextContent_0eab7fce6c07d5c9 = function(arg0, arg1, arg2) {
    arg0.textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
};

module.exports.__wbg_setwidth_c588fe07a7982aca = function(arg0, arg1) {
    arg0.width = arg1 >>> 0;
};

module.exports.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

module.exports.__wbg_static_accessor_GLOBAL_0be7472e492ad3e3 = function() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

module.exports.__wbg_static_accessor_GLOBAL_THIS_1a6eb482d12c9bfb = function() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

module.exports.__wbg_static_accessor_SELF_1dc398a895c82351 = function() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

module.exports.__wbg_static_accessor_WINDOW_ae1c80c7eea8d64a = function() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

module.exports.__wbg_width_9927e6a7adb23d6d = function(arg0) {
    const ret = arg0.width;
    return ret;
};

module.exports.__wbg_width_c7e5cdd4c1f88e14 = function(arg0) {
    const ret = arg0.width;
    return ret;
};

module.exports.__wbg_width_ed3fd44e46b8c2c9 = function(arg0) {
    const ret = arg0.width;
    return ret;
};

module.exports.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

module.exports.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_export_2;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

module.exports.__wbindgen_is_undefined = function(arg0) {
    const ret = arg0 === undefined;
    return ret;
};

module.exports.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return ret;
};

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

const path = require('path').join(__dirname, 'photon-node_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

wasm.__wbindgen_start();

