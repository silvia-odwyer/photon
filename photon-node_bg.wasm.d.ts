/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function alter_channel(a: number, b: number, c: number): void;
export function alter_red_channel(a: number, b: number): void;
export function alter_green_channel(a: number, b: number): void;
export function alter_blue_channel(a: number, b: number): void;
export function alter_two_channels(a: number, b: number, c: number, d: number, e: number): void;
export function alter_channels(a: number, b: number, c: number, d: number): void;
export function remove_channel(a: number, b: number, c: number): void;
export function remove_red_channel(a: number, b: number): void;
export function remove_green_channel(a: number, b: number): void;
export function remove_blue_channel(a: number, b: number): void;
export function swap_channels(a: number, b: number, c: number): void;
export function invert(a: number): void;
export function selective_hue_rotate(a: number, b: number, c: number): void;
export function selective_color_convert(a: number, b: number, c: number, d: number): void;
export function selective_lighten(a: number, b: number, c: number): void;
export function selective_desaturate(a: number, b: number, c: number): void;
export function selective_saturate(a: number, b: number, c: number): void;
export function selective_greyscale(a: number, b: number): void;
export function noise_reduction(a: number): void;
export function sharpen(a: number): void;
export function edge_detection(a: number): void;
export function identity(a: number): void;
export function box_blur(a: number): void;
export function gaussian_blur(a: number, b: number): void;
export function detect_horizontal_lines(a: number): void;
export function detect_vertical_lines(a: number): void;
export function detect_45_deg_lines(a: number): void;
export function detect_135_deg_lines(a: number): void;
export function laplace(a: number): void;
export function edge_one(a: number): void;
export function emboss(a: number): void;
export function sobel_horizontal(a: number): void;
export function prewitt_horizontal(a: number): void;
export function sobel_vertical(a: number): void;
export function monochrome(a: number, b: number, c: number, d: number): void;
export function sepia(a: number): void;
export function grayscale(a: number): void;
export function grayscale_human_corrected(a: number): void;
export function desaturate(a: number): void;
export function decompose_min(a: number): void;
export function decompose_max(a: number): void;
export function grayscale_shades(a: number, b: number): void;
export function r_grayscale(a: number): void;
export function g_grayscale(a: number): void;
export function b_grayscale(a: number): void;
export function single_channel_grayscale(a: number, b: number): void;
export function threshold(a: number, b: number): void;
export function neue(a: number): void;
export function lix(a: number): void;
export function ryo(a: number): void;
export function filter(a: number, b: number, c: number): void;
export function lofi(a: number): void;
export function pastel_pink(a: number): void;
export function golden(a: number): void;
export function cali(a: number): void;
export function dramatic(a: number): void;
export function monochrome_tint(a: number, b: number): void;
export function duotone_violette(a: number): void;
export function duotone_horizon(a: number): void;
export function duotone_tint(a: number, b: number): void;
export function duotone_lilac(a: number): void;
export function duotone_ochre(a: number): void;
export function firenze(a: number): void;
export function obsidian(a: number): void;
export function crop(a: number, b: number, c: number, d: number, e: number): number;
export function crop_img_browser(a: number, b: number, c: number, d: number, e: number): number;
export function fliph(a: number): void;
export function flipv(a: number): void;
export function resize_img_browser(a: number, b: number, c: number, d: number): number;
export function resize(a: number, b: number, c: number, d: number): number;
export function seam_carve(a: number, b: number, c: number): number;
export function padding_uniform(a: number, b: number, c: number): number;
export function padding_left(a: number, b: number, c: number): number;
export function padding_right(a: number, b: number, c: number): number;
export function padding_top(a: number, b: number, c: number): number;
export function padding_bottom(a: number, b: number, c: number): number;
export function rotate(a: number, b: number): number;
export function resample(a: number, b: number, c: number): number;
export function gamma_correction(a: number, b: number, c: number, d: number): void;
export function hsluv(a: number, b: number, c: number, d: number): void;
export function lch(a: number, b: number, c: number, d: number): void;
export function hsl(a: number, b: number, c: number, d: number): void;
export function hsv(a: number, b: number, c: number, d: number): void;
export function hue_rotate_hsl(a: number, b: number): void;
export function hue_rotate_hsv(a: number, b: number): void;
export function hue_rotate_lch(a: number, b: number): void;
export function hue_rotate_hsluv(a: number, b: number): void;
export function saturate_hsl(a: number, b: number): void;
export function saturate_lch(a: number, b: number): void;
export function saturate_hsluv(a: number, b: number): void;
export function saturate_hsv(a: number, b: number): void;
export function lighten_lch(a: number, b: number): void;
export function lighten_hsluv(a: number, b: number): void;
export function lighten_hsl(a: number, b: number): void;
export function lighten_hsv(a: number, b: number): void;
export function darken_lch(a: number, b: number): void;
export function darken_hsluv(a: number, b: number): void;
export function darken_hsl(a: number, b: number): void;
export function darken_hsv(a: number, b: number): void;
export function desaturate_hsv(a: number, b: number): void;
export function desaturate_hsl(a: number, b: number): void;
export function desaturate_lch(a: number, b: number): void;
export function desaturate_hsluv(a: number, b: number): void;
export function mix_with_colour(a: number, b: number, c: number): void;
export function draw_text_with_border(a: number, b: number, c: number, d: number, e: number): void;
export function draw_text(a: number, b: number, c: number, d: number, e: number): void;
export function offset(a: number, b: number, c: number): void;
export function offset_red(a: number, b: number): void;
export function offset_green(a: number, b: number): void;
export function offset_blue(a: number, b: number): void;
export function multiple_offsets(a: number, b: number, c: number, d: number): void;
export function primary(a: number): void;
export function colorize(a: number): void;
export function solarize(a: number): void;
export function solarize_retimg(a: number): number;
export function inc_brightness(a: number, b: number): void;
export function adjust_contrast(a: number, b: number): void;
export function tint(a: number, b: number, c: number, d: number): void;
export function horizontal_strips(a: number, b: number): void;
export function color_horizontal_strips(a: number, b: number, c: number): void;
export function vertical_strips(a: number, b: number): void;
export function color_vertical_strips(a: number, b: number, c: number): void;
export function oil(a: number, b: number, c: number): void;
export function frosted_glass(a: number): void;
export function pixelize(a: number, b: number): void;
export function normalize(a: number): void;
export function dither(a: number, b: number): void;
export function duotone(a: number, b: number, c: number): void;
export function add_noise_rand(a: number): void;
export function pink_noise(a: number): void;
export function __wbg_photonimage_free(a: number): void;
export function photonimage_new(a: number, b: number, c: number, d: number): number;
export function photonimage_new_from_byteslice(a: number, b: number): number;
export function photonimage_new_from_blob(a: number): number;
export function photonimage_new_from_image(a: number): number;
export function photonimage_get_width(a: number): number;
export function photonimage_get_raw_pixels(a: number, b: number): void;
export function photonimage_get_height(a: number): number;
export function photonimage_get_base64(a: number, b: number): void;
export function photonimage_get_bytes(a: number, b: number): void;
export function photonimage_get_bytes_jpeg(a: number, b: number, c: number): void;
export function photonimage_get_bytes_webp(a: number, b: number): void;
export function photonimage_get_image_data(a: number): number;
export function photonimage_set_imgdata(a: number, b: number): void;
export function __wbg_rgb_free(a: number): void;
export function rgb_new(a: number, b: number, c: number): number;
export function rgb_set_red(a: number, b: number): void;
export function rgb_set_green(a: number, b: number): void;
export function rgb_set_blue(a: number, b: number): void;
export function rgb_get_red(a: number): number;
export function rgb_get_green(a: number): number;
export function rgb_get_blue(a: number): number;
export function rgba_new(a: number, b: number, c: number, d: number): number;
export function rgba_set_alpha(a: number, b: number): void;
export function rgba_get_alpha(a: number): number;
export function run(a: number): void;
export function get_image_data(a: number, b: number): number;
export function putImageData(a: number, b: number, c: number): void;
export function open_image(a: number, b: number): number;
export function to_raw_pixels(a: number, b: number): void;
export function base64_to_image(a: number, b: number): number;
export function base64_to_vec(a: number, b: number, c: number): void;
export function to_image_data(a: number): number;
export function rgba_get_red(a: number): number;
export function rgba_get_green(a: number): number;
export function rgba_get_blue(a: number): number;
export function photonimage_new_from_base64(a: number, b: number): number;
export function rgba_set_red(a: number, b: number): void;
export function __wbg_rgba_free(a: number): void;
export function rgba_set_green(a: number, b: number): void;
export function rgba_set_blue(a: number, b: number): void;
export function watermark(a: number, b: number, c: number, d: number): void;
export function blend(a: number, b: number, c: number, d: number): void;
export function create_gradient(a: number, b: number): number;
export function apply_gradient(a: number): void;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
export function __wbindgen_exn_store(a: number): void;
