let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
const { TextEncoder, TextDecoder } = require(`util`);

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
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
    if (builtInMatches.length > 1) {
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

let WASM_VECTOR_LEN = 0;

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
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
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}
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
* let cropped_img: PhotonImage = crop(&mut img, 0_u32, 0_u32, 500_u32, 800_u32);
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
    var ret = wasm.crop(photon_image.ptr, x1, y1, x2, y2);
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
    var ret = wasm.crop_img_browser(addHeapObject(source_canvas), width, height, left, top);
    return takeObject(ret);
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
    wasm.fliph(photon_image.ptr);
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
    wasm.flipv(photon_image.ptr);
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
* @param {number} sampling_filter
* @returns {HTMLCanvasElement}
*/
module.exports.resize_img_browser = function(photon_img, width, height, sampling_filter) {
    _assertClass(photon_img, PhotonImage);
    var ret = wasm.resize_img_browser(photon_img.ptr, width, height, sampling_filter);
    return takeObject(ret);
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
* @param {number} sampling_filter
* @returns {PhotonImage}
*/
module.exports.resize = function(photon_img, width, height, sampling_filter) {
    _assertClass(photon_img, PhotonImage);
    var ret = wasm.resize(photon_img.ptr, width, height, sampling_filter);
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
    var ret = wasm.seam_carve(img.ptr, width, height);
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
    var ptr0 = padding_rgba.ptr;
    padding_rgba.ptr = 0;
    var ret = wasm.padding_uniform(img.ptr, padding, ptr0);
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
    var ptr0 = padding_rgba.ptr;
    padding_rgba.ptr = 0;
    var ret = wasm.padding_left(img.ptr, padding, ptr0);
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
    var ptr0 = padding_rgba.ptr;
    padding_rgba.ptr = 0;
    var ret = wasm.padding_right(img.ptr, padding, ptr0);
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
    var ptr0 = padding_rgba.ptr;
    padding_rgba.ptr = 0;
    var ret = wasm.padding_top(img.ptr, padding, ptr0);
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
    var ptr0 = padding_rgba.ptr;
    padding_rgba.ptr = 0;
    var ret = wasm.padding_bottom(img.ptr, padding, ptr0);
    return PhotonImage.__wrap(ret);
};

/**
* Rotate the PhotonImage on an arbitrary angle
* A rotated PhotonImage is returned.
* # NOTE: This is a naive implementation. Paeth rotation should be faster.
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
* let rotated_img = rotate(&img, 30);
* ```
* @param {PhotonImage} img
* @param {number} angle
* @returns {PhotonImage}
*/
module.exports.rotate = function(img, angle) {
    _assertClass(img, PhotonImage);
    var ret = wasm.rotate(img.ptr, angle);
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
    var ret = wasm.resample(img.ptr, dst_width, dst_height);
    return PhotonImage.__wrap(ret);
};

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*! [temp] Check if WASM is supported.
*/
module.exports.run = function() {
    wasm.run();
};

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* Get the ImageData from a 2D canvas context
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @returns {ImageData}
*/
module.exports.get_image_data = function(canvas, ctx) {
    try {
        var ret = wasm.get_image_data(addBorrowedObject(canvas), addBorrowedObject(ctx));
        return takeObject(ret);
    } finally {
        heap[stack_pointer++] = undefined;
        heap[stack_pointer++] = undefined;
    }
};

/**
* Place a PhotonImage onto a 2D canvas.
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @param {PhotonImage} new_image
*/
module.exports.putImageData = function(canvas, ctx, new_image) {
    _assertClass(new_image, PhotonImage);
    var ptr0 = new_image.ptr;
    new_image.ptr = 0;
    wasm.putImageData(addHeapObject(canvas), addHeapObject(ctx), ptr0);
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
    var ret = wasm.open_image(addHeapObject(canvas), addHeapObject(ctx));
    return PhotonImage.__wrap(ret);
};

/**
* Convert ImageData to a raw pixel vec of u8s.
* @param {ImageData} imgdata
* @returns {Uint8Array}
*/
module.exports.to_raw_pixels = function(imgdata) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.to_raw_pixels(retptr, addHeapObject(imgdata));
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v0 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v0;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
};

/**
* Convert a base64 string to a PhotonImage.
* @param {string} base64
* @returns {PhotonImage}
*/
module.exports.base64_to_image = function(base64) {
    var ptr0 = passStringToWasm0(base64, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.base64_to_image(ptr0, len0);
    return PhotonImage.__wrap(ret);
};

/**
* Convert a base64 string to a Vec of u8s.
* @param {string} base64
* @returns {Uint8Array}
*/
module.exports.base64_to_vec = function(base64) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        var ptr0 = passStringToWasm0(base64, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.base64_to_vec(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
};

/**
* Convert a PhotonImage to JS-compatible ImageData.
* @param {PhotonImage} photon_image
* @returns {ImageData}
*/
module.exports.to_image_data = function(photon_image) {
    _assertClass(photon_image, PhotonImage);
    var ptr0 = photon_image.ptr;
    photon_image.ptr = 0;
    var ret = wasm.to_image_data(ptr0);
    return takeObject(ret);
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
* draw_text_with_border(&mut img, "Welcome to Photon!", 10_u32, 10_u32);
* ```
* @param {PhotonImage} photon_img
* @param {string} text
* @param {number} x
* @param {number} y
*/
module.exports.draw_text_with_border = function(photon_img, text, x, y) {
    _assertClass(photon_img, PhotonImage);
    var ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.draw_text_with_border(photon_img.ptr, ptr0, len0, x, y);
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
* draw_text(&mut img, "Welcome to Photon!", 10_u32, 10_u32);
* ```
* @param {PhotonImage} photon_img
* @param {string} text
* @param {number} x
* @param {number} y
*/
module.exports.draw_text = function(photon_img, text, x, y) {
    _assertClass(photon_img, PhotonImage);
    var ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.draw_text(photon_img.ptr, ptr0, len0, x, y);
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
    wasm.offset(photon_image.ptr, channel_index, offset);
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
    wasm.offset_red(img.ptr, offset_amt);
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
    wasm.offset_green(img.ptr, offset_amt);
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
    wasm.offset_blue(img.ptr, offset_amt);
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
    wasm.multiple_offsets(photon_image.ptr, offset, channel_index, channel_index2);
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
    wasm.primary(img.ptr);
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
    wasm.colorize(photon_image.ptr);
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
    wasm.solarize(photon_image.ptr);
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
    var ret = wasm.solarize_retimg(photon_image.ptr);
    return PhotonImage.__wrap(ret);
};

/**
* Increase the brightness of an image by a factor.
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
    wasm.inc_brightness(photon_image.ptr, brightness);
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
    wasm.adjust_contrast(photon_image.ptr, contrast);
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
    wasm.tint(photon_image.ptr, r_offset, g_offset, b_offset);
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
    wasm.horizontal_strips(photon_image.ptr, num_strips);
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
    var ptr0 = color.ptr;
    color.ptr = 0;
    wasm.color_horizontal_strips(photon_image.ptr, num_strips, ptr0);
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
    wasm.vertical_strips(photon_image.ptr, num_strips);
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
    var ptr0 = color.ptr;
    color.ptr = 0;
    wasm.color_vertical_strips(photon_image.ptr, num_strips, ptr0);
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
    wasm.oil(photon_image.ptr, radius, intensity);
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
    wasm.frosted_glass(photon_image.ptr);
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
    wasm.pixelize(photon_image.ptr, pixel_size);
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
    wasm.normalize(photon_image.ptr);
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
    wasm.dither(photon_image.ptr, depth);
};

/**
* @param {PhotonImage} photon_image
* @param {Rgb} color_a
* @param {Rgb} color_b
*/
module.exports.duotone = function(photon_image, color_a, color_b) {
    _assertClass(photon_image, PhotonImage);
    _assertClass(color_a, Rgb);
    var ptr0 = color_a.ptr;
    color_a.ptr = 0;
    _assertClass(color_b, Rgb);
    var ptr1 = color_b.ptr;
    color_b.ptr = 0;
    wasm.duotone(photon_image.ptr, ptr0, ptr1);
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
    wasm.alter_channel(img.ptr, channel, amt);
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
    wasm.alter_red_channel(photon_image.ptr, amt);
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
    wasm.alter_green_channel(img.ptr, amt);
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
    wasm.alter_blue_channel(img.ptr, amt);
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
    wasm.alter_two_channels(img.ptr, channel1, amt1, channel2, amt2);
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
    wasm.alter_channels(img.ptr, r_amt, g_amt, b_amt);
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
    wasm.remove_channel(img.ptr, channel, min_filter);
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
    wasm.remove_red_channel(img.ptr, min_filter);
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
    wasm.remove_green_channel(img.ptr, min_filter);
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
    wasm.remove_blue_channel(img.ptr, min_filter);
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
    wasm.swap_channels(img.ptr, channel1, channel2);
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
    wasm.invert(photon_image.ptr);
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
    var ptr0 = ref_color.ptr;
    ref_color.ptr = 0;
    wasm.selective_hue_rotate(photon_image.ptr, ptr0, degrees);
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
    var ptr0 = ref_color.ptr;
    ref_color.ptr = 0;
    _assertClass(new_color, Rgb);
    var ptr1 = new_color.ptr;
    new_color.ptr = 0;
    wasm.selective_color_convert(photon_image.ptr, ptr0, ptr1, fraction);
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
    var ptr0 = ref_color.ptr;
    ref_color.ptr = 0;
    wasm.selective_lighten(img.ptr, ptr0, amt);
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
    var ptr0 = ref_color.ptr;
    ref_color.ptr = 0;
    wasm.selective_desaturate(img.ptr, ptr0, amt);
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
    var ptr0 = ref_color.ptr;
    ref_color.ptr = 0;
    wasm.selective_saturate(img.ptr, ptr0, amt);
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
    var ptr0 = photon_image.ptr;
    photon_image.ptr = 0;
    _assertClass(ref_color, Rgb);
    var ptr1 = ref_color.ptr;
    ref_color.ptr = 0;
    wasm.selective_greyscale(ptr0, ptr1);
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
    wasm.neue(photon_image.ptr);
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
    wasm.lix(photon_image.ptr);
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
    wasm.ryo(photon_image.ptr);
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
    var ptr0 = passStringToWasm0(filter_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.filter(img.ptr, ptr0, len0);
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
    wasm.lofi(img.ptr);
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
    wasm.pastel_pink(img.ptr);
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
    wasm.golden(img.ptr);
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
    wasm.cali(img.ptr);
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
    wasm.dramatic(img.ptr);
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
    var ptr0 = rgb_color.ptr;
    rgb_color.ptr = 0;
    wasm.monochrome_tint(img.ptr, ptr0);
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
    wasm.duotone_violette(img.ptr);
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
    wasm.duotone_horizon(img.ptr);
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
    var ptr0 = rgb_color.ptr;
    rgb_color.ptr = 0;
    wasm.duotone_tint(img.ptr, ptr0);
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
    wasm.duotone_lilac(img.ptr);
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
    wasm.duotone_ochre(img.ptr);
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
    wasm.firenze(img.ptr);
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
    wasm.obsidian(img.ptr);
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
    wasm.monochrome(img.ptr, r_offset, g_offset, b_offset);
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
    wasm.sepia(img.ptr);
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
    wasm.grayscale(img.ptr);
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
    wasm.grayscale_human_corrected(img.ptr);
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
    wasm.desaturate(img.ptr);
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
    wasm.decompose_min(img.ptr);
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
    wasm.decompose_max(img.ptr);
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
    wasm.grayscale_shades(photon_image.ptr, num_shades);
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
    wasm.r_grayscale(photon_image.ptr);
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
    wasm.g_grayscale(photon_image.ptr);
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
    wasm.b_grayscale(photon_image.ptr);
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
    wasm.single_channel_grayscale(photon_image.ptr, channel);
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
    wasm.threshold(img.ptr, threshold);
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
    wasm.gamma_correction(photon_image.ptr, red, green, blue);
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
    var ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.hsluv(photon_image.ptr, ptr0, len0, amt);
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
    var ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.lch(photon_image.ptr, ptr0, len0, amt);
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
    var ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.hsl(photon_image.ptr, ptr0, len0, amt);
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
    var ptr0 = passStringToWasm0(mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.hsv(photon_image.ptr, ptr0, len0, amt);
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
    wasm.hue_rotate_hsl(img.ptr, degrees);
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
    wasm.hue_rotate_hsv(img.ptr, degrees);
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
    wasm.hue_rotate_lch(img.ptr, degrees);
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
    wasm.hue_rotate_hsluv(img.ptr, degrees);
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
    wasm.saturate_hsl(img.ptr, level);
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
    wasm.saturate_lch(img.ptr, level);
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
    wasm.saturate_hsluv(img.ptr, level);
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
    wasm.saturate_hsv(img.ptr, level);
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
    wasm.lighten_lch(img.ptr, level);
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
    wasm.lighten_hsluv(img.ptr, level);
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
    wasm.lighten_hsl(img.ptr, level);
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
    wasm.lighten_hsv(img.ptr, level);
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
    wasm.darken_lch(img.ptr, level);
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
    wasm.darken_hsluv(img.ptr, level);
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
    wasm.darken_hsl(img.ptr, level);
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
    wasm.darken_hsv(img.ptr, level);
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
    wasm.desaturate_hsv(img.ptr, level);
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
    wasm.desaturate_hsl(img.ptr, level);
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
    wasm.desaturate_lch(img.ptr, level);
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
    wasm.desaturate_hsluv(img.ptr, level);
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
    var ptr0 = mix_colour.ptr;
    mix_colour.ptr = 0;
    wasm.mix_with_colour(photon_image.ptr, ptr0, opacity);
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
* watermark(&mut img, &water_mark, 30_u32, 40_u32);
* ```
* @param {PhotonImage} img
* @param {PhotonImage} watermark
* @param {number} x
* @param {number} y
*/
module.exports.watermark = function(img, watermark, x, y) {
    _assertClass(img, PhotonImage);
    _assertClass(watermark, PhotonImage);
    wasm.watermark(img.ptr, watermark.ptr, x, y);
};

/**
* Blend two images together.
*
* The `blend_mode` (3rd param) determines which blending mode to use; change this for varying effects.
* The blend modes available include: `overlay`, `over`, `atop`, `xor`, `multiply`, `burn`, `soft_light`, `hard_light`,
* `difference`, `lighten`, `darken`, `dodge`, `plus`, `exclusion` (more to come)
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
    var ptr0 = passStringToWasm0(blend_mode, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.blend(photon_image.ptr, photon_image2.ptr, ptr0, len0);
};

/**
* @param {number} width
* @param {number} height
* @returns {PhotonImage}
*/
module.exports.create_gradient = function(width, height) {
    var ret = wasm.create_gradient(width, height);
    return PhotonImage.__wrap(ret);
};

/**
* Apply a gradient to an image.
* @param {PhotonImage} image
*/
module.exports.apply_gradient = function(image) {
    _assertClass(image, PhotonImage);
    wasm.apply_gradient(image.ptr);
};

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
    wasm.noise_reduction(photon_image.ptr);
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
    wasm.sharpen(photon_image.ptr);
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
    wasm.edge_detection(photon_image.ptr);
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
    wasm.identity(photon_image.ptr);
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
    wasm.box_blur(photon_image.ptr);
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
    wasm.gaussian_blur(photon_image.ptr, radius);
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
    wasm.detect_horizontal_lines(photon_image.ptr);
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
    wasm.detect_vertical_lines(photon_image.ptr);
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
    wasm.detect_45_deg_lines(photon_image.ptr);
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
    wasm.detect_135_deg_lines(photon_image.ptr);
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
    wasm.laplace(photon_image.ptr);
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
    wasm.edge_one(photon_image.ptr);
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
    wasm.emboss(photon_image.ptr);
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
    wasm.sobel_horizontal(photon_image.ptr);
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
    wasm.prewitt_horizontal(photon_image.ptr);
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
    wasm.sobel_vertical(photon_image.ptr);
};

function isLikeNone(x) {
    return x === undefined || x === null;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

let cachegetUint8ClampedMemory0 = null;
function getUint8ClampedMemory0() {
    if (cachegetUint8ClampedMemory0 === null || cachegetUint8ClampedMemory0.buffer !== wasm.memory.buffer) {
        cachegetUint8ClampedMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachegetUint8ClampedMemory0;
}

function getClampedArrayU8FromWasm0(ptr, len) {
    return getUint8ClampedMemory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
module.exports.SamplingFilter = Object.freeze({ Nearest:1,"1":"Nearest",Triangle:2,"2":"Triangle",CatmullRom:3,"3":"CatmullRom",Gaussian:4,"4":"Gaussian",Lanczos3:5,"5":"Lanczos3", });
/**
* Provides the image's height, width, and contains the image's raw pixels.
* For use when communicating between JS and WASM, and also natively.
*/
class PhotonImage {

    static __wrap(ptr) {
        const obj = Object.create(PhotonImage.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_photonimage_free(ptr);
    }
    /**
    * Create a new PhotonImage from a Vec of u8s, which represent raw pixels.
    * @param {Uint8Array} raw_pixels
    * @param {number} width
    * @param {number} height
    */
    constructor(raw_pixels, width, height) {
        var ptr0 = passArray8ToWasm0(raw_pixels, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.photonimage_new(ptr0, len0, width, height);
        return PhotonImage.__wrap(ret);
    }
    /**
    * Create a new PhotonImage from a base64 string.
    * @param {string} base64
    * @returns {PhotonImage}
    */
    static new_from_base64(base64) {
        var ptr0 = passStringToWasm0(base64, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.base64_to_image(ptr0, len0);
        return PhotonImage.__wrap(ret);
    }
    /**
    * Create a new PhotonImage from a byteslice.
    * @param {Uint8Array} vec
    * @returns {PhotonImage}
    */
    static new_from_byteslice(vec) {
        var ptr0 = passArray8ToWasm0(vec, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.photonimage_new_from_byteslice(ptr0, len0);
        return PhotonImage.__wrap(ret);
    }
    /**
    * Get the width of the PhotonImage.
    * @returns {number}
    */
    get_width() {
        var ret = wasm.photonimage_get_width(this.ptr);
        return ret >>> 0;
    }
    /**
    * Get the PhotonImage's pixels as a Vec of u8s.
    * @returns {Uint8Array}
    */
    get_raw_pixels() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_raw_pixels(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Get the height of the PhotonImage.
    * @returns {number}
    */
    get_height() {
        var ret = wasm.photonimage_get_height(this.ptr);
        return ret >>> 0;
    }
    /**
    * Convert the PhotonImage to base64.
    * @returns {string}
    */
    get_base64() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_base64(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(r0, r1);
        }
    }
    /**
    * Convert the PhotonImage to raw bytes. Returns JPEG.
    * @returns {Uint8Array}
    */
    get_bytes() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_bytes(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Convert the PhotonImage to raw bytes. Returns a JPEG.
    * @param {number} quality
    * @returns {Uint8Array}
    */
    get_bytes_jpeg(quality) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.photonimage_get_bytes_jpeg(retptr, this.ptr, quality);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayU8FromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 1);
            return v0;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Convert the PhotonImage's raw pixels to JS-compatible ImageData.
    * @returns {ImageData}
    */
    get_image_data() {
        var ret = wasm.photonimage_get_image_data(this.ptr);
        return takeObject(ret);
    }
    /**
    * Convert ImageData to raw pixels, and update the PhotonImage's raw pixels to this.
    * @param {ImageData} img_data
    */
    set_imgdata(img_data) {
        wasm.photonimage_set_imgdata(this.ptr, addHeapObject(img_data));
    }
}
module.exports.PhotonImage = PhotonImage;
/**
* RGB color type.
*/
class Rgb {

    static __wrap(ptr) {
        const obj = Object.create(Rgb.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgb_free(ptr);
    }
    /**
    * Create a new RGB struct.
    * @param {number} r
    * @param {number} g
    * @param {number} b
    */
    constructor(r, g, b) {
        var ret = wasm.rgb_new(r, g, b);
        return Rgb.__wrap(ret);
    }
    /**
    * Set the Red value.
    * @param {number} r
    */
    set_red(r) {
        wasm.rgb_set_red(this.ptr, r);
    }
    /**
    * Get the Green value.
    * @param {number} g
    */
    set_green(g) {
        wasm.rgb_set_green(this.ptr, g);
    }
    /**
    * Set the Blue value.
    * @param {number} b
    */
    set_blue(b) {
        wasm.rgb_set_blue(this.ptr, b);
    }
    /**
    * Get the Red value.
    * @returns {number}
    */
    get_red() {
        var ret = wasm.rgb_get_red(this.ptr);
        return ret;
    }
    /**
    * Get the Green value.
    * @returns {number}
    */
    get_green() {
        var ret = wasm.rgb_get_green(this.ptr);
        return ret;
    }
    /**
    * Get the Blue value.
    * @returns {number}
    */
    get_blue() {
        var ret = wasm.rgb_get_blue(this.ptr);
        return ret;
    }
}
module.exports.Rgb = Rgb;
/**
* RGBA color type.
*/
class Rgba {

    static __wrap(ptr) {
        const obj = Object.create(Rgba.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rgba_free(ptr);
    }
    /**
    * Create a new RGBA struct.
    * @param {number} r
    * @param {number} g
    * @param {number} b
    * @param {number} a
    */
    constructor(r, g, b, a) {
        var ret = wasm.rgba_new(r, g, b, a);
        return Rgba.__wrap(ret);
    }
    /**
    * Set the Red value.
    * @param {number} r
    */
    set_red(r) {
        wasm.rgb_set_red(this.ptr, r);
    }
    /**
    * Get the Green value.
    * @param {number} g
    */
    set_green(g) {
        wasm.rgb_set_green(this.ptr, g);
    }
    /**
    * Set the Blue value.
    * @param {number} b
    */
    set_blue(b) {
        wasm.rgb_set_blue(this.ptr, b);
    }
    /**
    * Set the alpha value.
    * @param {number} a
    */
    set_alpha(a) {
        wasm.rgba_set_alpha(this.ptr, a);
    }
    /**
    * Get the Red value.
    * @returns {number}
    */
    get_red() {
        var ret = wasm.rgb_get_red(this.ptr);
        return ret;
    }
    /**
    * Get the Green value.
    * @returns {number}
    */
    get_green() {
        var ret = wasm.rgb_get_green(this.ptr);
        return ret;
    }
    /**
    * Get the Blue value.
    * @returns {number}
    */
    get_blue() {
        var ret = wasm.rgb_get_blue(this.ptr);
        return ret;
    }
    /**
    * Get the alpha value for this color.
    * @returns {number}
    */
    get_alpha() {
        var ret = wasm.rgba_get_alpha(this.ptr);
        return ret;
    }
}
module.exports.Rgba = Rgba;

module.exports.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

module.exports.__wbg_new_693216e109162396 = function() {
    var ret = new Error();
    return addHeapObject(ret);
};

module.exports.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

module.exports.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

module.exports.__wbg_instanceof_Window_c4b70662a0d2c5ec = function(arg0) {
    var ret = getObject(arg0) instanceof Window;
    return ret;
};

module.exports.__wbg_document_1c64944725c0d81d = function(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

module.exports.__wbg_body_78ae4fd43b446013 = function(arg0) {
    var ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

module.exports.__wbg_createElement_86c152812a141a62 = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_width_16bd64d09cbf5661 = function(arg0) {
    var ret = getObject(arg0).width;
    return ret;
};

module.exports.__wbg_height_368bb86c37d51bc9 = function(arg0) {
    var ret = getObject(arg0).height;
    return ret;
};

module.exports.__wbg_data_1ae7496c58caf755 = function(arg0, arg1) {
    var ret = getObject(arg1).data;
    var ptr0 = passArray8ToWasm0(ret, wasm.__wbindgen_malloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

module.exports.__wbg_newwithu8clampedarrayandsh_1b8c6e1bede43657 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_instanceof_CanvasRenderingContext2d_3abbe7ec7af32cae = function(arg0) {
    var ret = getObject(arg0) instanceof CanvasRenderingContext2D;
    return ret;
};

module.exports.__wbg_drawImage_9e2d13329d92a0a3 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).drawImage(getObject(arg1), arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
}, arguments) };

module.exports.__wbg_getImageData_9ffc3df78ca3dbc9 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var ret = getObject(arg0).getImageData(arg1, arg2, arg3, arg4);
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_putImageData_b9544b271e569392 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).putImageData(getObject(arg1), arg2, arg3);
}, arguments) };

module.exports.__wbg_settextContent_799ebbf96e16265d = function(arg0, arg1, arg2) {
    getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
};

module.exports.__wbg_appendChild_d318db34c4559916 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_instanceof_HtmlCanvasElement_25d964a0dde6717e = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLCanvasElement;
    return ret;
};

module.exports.__wbg_width_555f63ab09ba7d3f = function(arg0) {
    var ret = getObject(arg0).width;
    return ret;
};

module.exports.__wbg_setwidth_c1a7061891b71f25 = function(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
};

module.exports.__wbg_height_7153faec70fbaf7b = function(arg0) {
    var ret = getObject(arg0).height;
    return ret;
};

module.exports.__wbg_setheight_88894b05710ff752 = function(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
};

module.exports.__wbg_getContext_f701d0231ae22393 = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

module.exports.__wbg_newnoargs_be86524d73f67598 = function(arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_call_888d259a5fefc347 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_self_c6fbdfc2918d5e58 = function() { return handleError(function () {
    var ret = self.self;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_window_baec038b5ab35c54 = function() { return handleError(function () {
    var ret = window.window;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_globalThis_3f735a5746d41fbd = function() { return handleError(function () {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_global_1bc0b39582740e95 = function() { return handleError(function () {
    var ret = global.global;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};

module.exports.__wbindgen_debug_string = function(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_rethrow = function(arg0) {
    throw takeObject(arg0);
};

const path = require('path').join(__dirname, 'photon-node_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

