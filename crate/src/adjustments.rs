//! Creative and corrective image modifications.
//! This module provides user-facing edits that photographers apply, such as exposure,
//! contrast, highlights, shadows, clarity, vibrance, and other creative adjustments.

use crate::PhotonImage;

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// Convert sRGB gamma-encoded value to linear
#[inline(always)]
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert linear value to sRGB gamma-encoded
#[inline(always)]
fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// Apply exposure compensation in linear space (scene-referred)
/// 
/// Exposure is applied as a multiplier in linear space before gamma encoding.
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process  
/// * `ev` - Exposure value in EV (stops). Range: -5.0 to +5.0
///          Each +1 EV doubles the light, -1 EV halves it
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_exposure;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_exposure(&mut img, 1.5); // Brighten by 1.5 stops
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_exposure(photon_image: &mut PhotonImage, ev: f32) {
    if ev == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let pixel_count = pixels.len() / 4;
    
    // Calculate multiplier from EV
    let multiplier = 2.0_f32.powf(ev);
    
    // Pre-build lookup table for efficiency
    // This converts sRGB -> linear -> multiply -> sRGB in one LUT lookup
    let mut lut: [u8; 256] = [0; 256];
    for (i, item) in lut.iter_mut().enumerate() {
        let linear = srgb_to_linear(i as f32 / 255.0);
        let adjusted = (linear * multiplier).clamp(0.0, 1.0);
        *item = (linear_to_srgb(adjusted) * 255.0).clamp(0.0, 255.0) as u8;
    }
    
    // Apply LUT
    for i in 0..pixel_count {
        let idx = i * 4;
        pixels[idx] = lut[pixels[idx] as usize];
        pixels[idx + 1] = lut[pixels[idx + 1] as usize];
        pixels[idx + 2] = lut[pixels[idx + 2] as usize];
    }
}

/// Bradford transformation matrix
const BRADFORD_M: [[f32; 3]; 3] = [
    [0.8951, 0.2664, -0.1614],
    [-0.7502, 1.7135, 0.0367],
    [0.0389, -0.0685, 1.0296],
];

/// Inverse Bradford matrix
const BRADFORD_M_INV: [[f32; 3]; 3] = [
    [0.9869929, -0.1470543, 0.1599627],
    [0.4323053, 0.5183603, 0.0492912],
    [-0.0085287, 0.0400428, 0.9684867],
];

/// sRGB to XYZ matrix (D65)
const SRGB_TO_XYZ: [[f32; 3]; 3] = [
    [0.412_456_4, 0.357_576_1, 0.180_437_5],
    [0.212_672_9, 0.715_152_2, 0.072_175],
    [0.019_333_9, 0.119_192, 0.950_304_1],
];

/// XYZ to sRGB matrix (D65)
const XYZ_TO_SRGB: [[f32; 3]; 3] = [
    [3.240_454_2, -1.537_138_5, -0.498_531_4],
    [-0.969_266, 1.876_010_8, 0.041_556],
    [0.055_643_4, -0.204_025_9, 1.057_225_2],
];

/// Convert color temperature in Kelvin to xy chromaticity coordinates
fn kelvin_to_xy(kelvin: f32) -> (f32, f32) {
    let k = kelvin.clamp(1000.0, 40000.0);
    
    let x = if k < 4000.0 {
        let t = 1000.0 / k;
        -0.2661239 * t * t * t - 0.2343589 * t * t + 0.8776956 * t + 0.179910
    } else if k <= 7000.0 {
        let k2 = k * k;
        let k3 = k2 * k;
        -4.6070e9 / k3 + 2.9678e6 / k2 + 0.09911e3 / k + 0.244063
    } else {
        let k2 = k * k;
        let k3 = k2 * k;
        -2.0064e9 / k3 + 1.9018e6 / k2 + 0.24748e3 / k + 0.237040
    };
    
    // Planckian locus approximation for y
    let y = -3.0 * x * x + 2.87 * x - 0.275;
    
    (x, y)
}

/// Convert xy chromaticity to XYZ with Y=1
#[inline(always)]
fn xy_to_xyz(x: f32, y: f32) -> [f32; 3] {
    let y_val = 1.0;
    let x_val = (y_val / y) * x;
    let z_val = (y_val / y) * (1.0 - x - y);
    [x_val, y_val, z_val]
}

/// Matrix-vector multiplication for 3x3 matrix and 3-element vector
#[inline(always)]
fn mat_vec_mult(m: &[[f32; 3]; 3], v: &[f32; 3]) -> [f32; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

/// Create Bradford chromatic adaptation matrix from source to destination white point
fn create_bradford_matrix(src_white: &[f32; 3], dst_white: &[f32; 3]) -> [[f32; 3]; 3] {
    // Convert white points to cone response domain
    let src_cone = mat_vec_mult(&BRADFORD_M, src_white);
    let dst_cone = mat_vec_mult(&BRADFORD_M, dst_white);
    
    // Diagonal scaling factors
    let scale = [
        dst_cone[0] / src_cone[0],
        dst_cone[1] / src_cone[1],
        dst_cone[2] / src_cone[2],
    ];
    
    // Build combined matrix: M^-1 * diag(scale) * M
    // First: diag(scale) * M
    let temp: [[f32; 3]; 3] = [
        [scale[0] * BRADFORD_M[0][0], scale[0] * BRADFORD_M[0][1], scale[0] * BRADFORD_M[0][2]],
        [scale[1] * BRADFORD_M[1][0], scale[1] * BRADFORD_M[1][1], scale[1] * BRADFORD_M[1][2]],
        [scale[2] * BRADFORD_M[2][0], scale[2] * BRADFORD_M[2][1], scale[2] * BRADFORD_M[2][2]],
    ];
    
    // Then: M^-1 * temp (3x3 matrix multiplication)
    let mut result = [[0.0f32; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += BRADFORD_M_INV[i][k] * temp[k][j];
            }
        }
    }
    
    result
}

/// Apply white balance using Bradford chromatic adaptation
/// 
/// Converts from the current illuminant to D65 (standard daylight).
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `temperature` - Color temperature shift (-100 to 100, where 0 = 6500K/D65)
///                   Negative = warmer (lower Kelvin), Positive = cooler (higher Kelvin)
/// * `tint` - Green-magenta axis (-100 to 100, where negative = green, positive = magenta)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_white_balance;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_white_balance(&mut img, -20.0, 5.0); // Warm up the image slightly with magenta tint
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_white_balance(
    photon_image: &mut PhotonImage,
    temperature: f32,
    tint: f32,
) {
    if temperature == 0.0 && tint == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let pixel_count = pixels.len() / 4;
    
    // Base temperature is D65 (6500K)
    let base_temp = 6500.0_f32;
    
    // Map temperature slider to Kelvin
    // 0 = 6500K, -100 = 3000K (warm), +100 = 12000K (cool)
    let target_temp = if temperature <= 0.0 {
        base_temp + (temperature / 100.0) * (base_temp - 3000.0)
    } else {
        base_temp + (temperature / 100.0) * (12000.0 - base_temp)
    };
    
    // Get white points
    let (src_x, src_y) = kelvin_to_xy(base_temp);
    let (dst_x, dst_y) = kelvin_to_xy(target_temp);
    let src_white = xy_to_xyz(src_x, src_y);
    let dst_white = xy_to_xyz(dst_x, dst_y);
    
    // Create Bradford adaptation matrix
    let adapt_matrix = create_bradford_matrix(&src_white, &dst_white);
    
    // Tint adjustment factor (green-magenta axis)
    let tint_factor = tint / 100.0;
    
    // Process each pixel
    for i in 0..pixel_count {
        let idx = i * 4;
        
        // Convert sRGB to linear
        let lin_r = srgb_to_linear(pixels[idx] as f32 / 255.0);
        let lin_g = srgb_to_linear(pixels[idx + 1] as f32 / 255.0);
        let lin_b = srgb_to_linear(pixels[idx + 2] as f32 / 255.0);
        
        // Convert to XYZ
        let rgb = [lin_r, lin_g, lin_b];
        let xyz = mat_vec_mult(&SRGB_TO_XYZ, &rgb);
        
        // Apply Bradford adaptation
        let adapted_xyz = mat_vec_mult(&adapt_matrix, &xyz);
        
        // Convert back to linear RGB
        let mut adapted_rgb = mat_vec_mult(&XYZ_TO_SRGB, &adapted_xyz);
        
        // Apply tint (green-magenta adjustment)
        if tint_factor > 0.0 {
            // Magenta: reduce green, boost red+blue
            adapted_rgb[1] *= 1.0 - tint_factor * 0.5;
            adapted_rgb[0] *= 1.0 + tint_factor * 0.6;
            adapted_rgb[2] *= 1.0 + tint_factor * 0.6;
        } else if tint_factor < 0.0 {
            // Green: boost green, reduce red+blue
            adapted_rgb[1] *= 1.0 + tint_factor.abs() * 0.5;
            adapted_rgb[0] *= 1.0 - tint_factor.abs() * 0.6;
            adapted_rgb[2] *= 1.0 - tint_factor.abs() * 0.6;
        }
        
        // Convert back to sRGB
        let out_r = (linear_to_srgb(adapted_rgb[0].clamp(0.0, 1.0)) * 255.0).clamp(0.0, 255.0);
        let out_g = (linear_to_srgb(adapted_rgb[1].clamp(0.0, 1.0)) * 255.0).clamp(0.0, 255.0);
        let out_b = (linear_to_srgb(adapted_rgb[2].clamp(0.0, 1.0)) * 255.0).clamp(0.0, 255.0);
        
        pixels[idx] = out_r as u8;
        pixels[idx + 1] = out_g as u8;
        pixels[idx + 2] = out_b as u8;
    }
}

/// Convert RGB to HSL
fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = r / 255.0;
    let g = g / 255.0;
    let b = b / 255.0;
    
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;
    
    // Lightness
    let l = (max + min) / 2.0;
    
    // Saturation
    let s = if delta == 0.0 {
        0.0
    } else if l < 0.5 {
        delta / (max + min)
    } else {
        delta / (2.0 - max - min)
    };
    
    // Hue
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        (((g - b) / delta) % 6.0) * 60.0
    } else if max == g {
        (((b - r) / delta) + 2.0) * 60.0
    } else {
        (((r - g) / delta) + 4.0) * 60.0
    };
    
    (h, s, l)
}

/// Convert HSL to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let h = h / 360.0;
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    
    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    (
        ((r + m) * 255.0).clamp(0.0, 255.0),
        ((g + m) * 255.0).clamp(0.0, 255.0),
        ((b + m) * 255.0).clamp(0.0, 255.0),
    )
}

/// Apply vibrance adjustment
/// Vibrance boosts less-saturated colors more and protects skin tones
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `amount` - Vibrance amount (-100 to 100)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_vibrance;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_vibrance(&mut img, 30.0); // Increase vibrance
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_vibrance(
    photon_image: &mut PhotonImage,
    amount: f32,
) {
    if amount == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let pixel_count = pixels.len() / 4;
    let strength = amount / 100.0;
    
    for i in 0..pixel_count {
        let idx = i * 4;
        let r = pixels[idx] as f32;
        let g = pixels[idx + 1] as f32;
        let b = pixels[idx + 2] as f32;
        
        // Convert to HSL (rgb_to_hsl expects 0-255 range)
        let (h, s, l) = rgb_to_hsl(r, g, b);
        
        // Vibrance: asymmetric saturation adjustment
        // Positive vibrance: boosts less-saturated colors more (protects already-saturated colors)
        // Negative vibrance: reduces more-saturated colors more (protects already-muted colors)
        // This asymmetry prevents negative vibrance from making muted colors completely gray
        let sat_factor = if strength >= 0.0 {
            1.0 - s  // Positive: affect less-saturated colors more
        } else {
            s        // Negative: affect more-saturated colors more
        };
        
        // Skin tone protection (orange-red hues around 20-60°)
        // Always provide some protection across the range, strongest at center (35°)
        let mut skin_protection = 1.0;
        if (20.0..=60.0).contains(&h) {
            let dist_from_center = (h - 35.0).abs();
            let t = (dist_from_center / 20.0).min(1.0); // Normalize to 0-1
            // Protection ranges from 0.3 (at center) to 0.7 (at edges), never full effect
            skin_protection = 0.3 + 0.4 * t;
        }
        
        // Calculate boost (positive strength increases saturation)
        let boost = strength * sat_factor * skin_protection;
        let new_s = (s + boost).clamp(0.0, 1.0);
        
        // Convert back to RGB
        let (new_r, new_g, new_b) = hsl_to_rgb(h, new_s, l);
        
        // Explicitly clamp to avoid any wrapping issues
        pixels[idx] = new_r.clamp(0.0, 255.0) as u8;
        pixels[idx + 1] = new_g.clamp(0.0, 255.0) as u8;
        pixels[idx + 2] = new_b.clamp(0.0, 255.0) as u8;
    }
}

/// Apply clarity (local contrast / midtone contrast)
/// Uses unsharp mask with large radius on luminance channel
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `amount` - Clarity amount (-100 to 100)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_clarity;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_clarity(&mut img, 25.0); // Increase clarity
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_clarity(
    photon_image: &mut PhotonImage,
    amount: f32,
) {
    if amount == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    let pixel_count = width * height;
    
    let strength = amount / 100.0;
    
    // Large radius for clarity (affects midtones/local contrast)
    // Typically 30-50 pixels for clarity vs 0.5-3 for sharpening
    let radius: usize = 15;
    
    // Extract luminance channel
    let mut luminance: Vec<f32> = vec![0.0; pixel_count];
    for (i, lum) in luminance.iter_mut().enumerate() {
        let idx = i * 4;
        let r = pixels[idx] as f32;
        let g = pixels[idx + 1] as f32;
        let b = pixels[idx + 2] as f32;
        // Rec. 601 luminance
        *lum = 0.299 * r + 0.587 * g + 0.114 * b;
    }
    
    // Box blur the luminance (approximates Gaussian for large radius)
    // Do horizontal pass
    let mut blurred: Vec<f32> = vec![0.0; pixel_count];
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            let mut count = 0;
            let x_start = x.saturating_sub(radius);
            let x_end = (x + radius + 1).min(width);
            
            for bx in x_start..x_end {
                sum += luminance[y * width + bx];
                count += 1;
            }
            blurred[y * width + x] = sum / count as f32;
        }
    }
    
    // Vertical pass
    let mut blurred2: Vec<f32> = vec![0.0; pixel_count];
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            let mut count = 0;
            let y_start = y.saturating_sub(radius);
            let y_end = (y + radius + 1).min(height);
            
            for by in y_start..y_end {
                sum += blurred[by * width + x];
                count += 1;
            }
            blurred2[y * width + x] = sum / count as f32;
        }
    }
    
    // Apply high-pass filter to each pixel
    // Clarity = original + (original - blurred) * strength
    for i in 0..pixel_count {
        let idx = i * 4;
        let r = pixels[idx] as f32;
        let g = pixels[idx + 1] as f32;
        let b = pixels[idx + 2] as f32;
        
        let orig_lum = luminance[i];
        let blur_lum = blurred2[i];
        
        // High-pass component
        let detail = orig_lum - blur_lum;
        
        // Midtone mask: apply more to midtones, less to shadows/highlights
        let midtone_mask = 1.0 - (2.0 * (orig_lum / 255.0 - 0.5)).abs().powf(2.0);
        
        // Calculate adjustment
        let adjustment = detail * strength * midtone_mask * 0.5;
        
        // Apply to each channel proportionally
        if orig_lum > 0.001 {
            let factor = (orig_lum + adjustment) / orig_lum;
            pixels[idx] = (r * factor).clamp(0.0, 255.0) as u8;
            pixels[idx + 1] = (g * factor).clamp(0.0, 255.0) as u8;
            pixels[idx + 2] = (b * factor).clamp(0.0, 255.0) as u8;
        }
    }
}

/// Apply texture enhancement (medium-frequency detail enhancement)
/// Similar to clarity but uses smaller radius for medium-frequency details
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `amount` - Texture amount (-100 to 100)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_texture;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_texture(&mut img, 30.0); // Increase texture
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_texture(
    photon_image: &mut PhotonImage,
    amount: f32,
) {
    if amount == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    let pixel_count = width * height;
    
    let strength = amount / 100.0;
    
    // Medium radius for texture (smaller than clarity, larger than sharpening)
    let radius: usize = 5;
    
    // Create blurred copy using box blur
    let mut blurred = pixels.clone();
    
    // Horizontal blur pass
    for y in 0..height {
        for x in 0..width {
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;
            let mut count = 0u32;
            
            let x_start = x.saturating_sub(radius);
            let x_end = (x + radius + 1).min(width);
            
            for bx in x_start..x_end {
                let idx = (y * width + bx) * 4;
                sum_r += pixels[idx] as u32;
                sum_g += pixels[idx + 1] as u32;
                sum_b += pixels[idx + 2] as u32;
                count += 1;
            }
            
            let idx = (y * width + x) * 4;
            blurred[idx] = (sum_r / count) as u8;
            blurred[idx + 1] = (sum_g / count) as u8;
            blurred[idx + 2] = (sum_b / count) as u8;
        }
    }
    
    // Vertical blur pass
    let mut blurred2 = blurred.clone();
    for y in 0..height {
        for x in 0..width {
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;
            let mut count = 0u32;
            
            let y_start = y.saturating_sub(radius);
            let y_end = (y + radius + 1).min(height);
            
            for by in y_start..y_end {
                let idx = (by * width + x) * 4;
                sum_r += blurred[idx] as u32;
                sum_g += blurred[idx + 1] as u32;
                sum_b += blurred[idx + 2] as u32;
                count += 1;
            }
            
            let idx = (y * width + x) * 4;
            blurred2[idx] = (sum_r / count) as u8;
            blurred2[idx + 1] = (sum_g / count) as u8;
            blurred2[idx + 2] = (sum_b / count) as u8;
        }
    }
    
    // Apply texture enhancement (high-pass filter)
    for i in 0..pixel_count {
        let idx = i * 4;
        
        // Calculate detail (difference from blur)
        let detail_r = pixels[idx] as f32 - blurred2[idx] as f32;
        let detail_g = pixels[idx + 1] as f32 - blurred2[idx + 1] as f32;
        let detail_b = pixels[idx + 2] as f32 - blurred2[idx + 2] as f32;
        
        // Apply texture amount
        pixels[idx] = ((pixels[idx] as f32 + detail_r * strength).clamp(0.0, 255.0)) as u8;
        pixels[idx + 1] = ((pixels[idx + 1] as f32 + detail_g * strength).clamp(0.0, 255.0)) as u8;
        pixels[idx + 2] = ((pixels[idx + 2] as f32 + detail_b * strength).clamp(0.0, 255.0)) as u8;
    }
}

/// Apply dehaze effect (atmospheric haze removal)
/// Uses dark channel prior algorithm for haze removal
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `amount` - Dehaze amount (-100 to 100, positive removes haze, negative adds haze)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_dehaze;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_dehaze(&mut img, 50.0); // Remove haze
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_dehaze(
    photon_image: &mut PhotonImage,
    amount: f32,
) {
    if amount == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    let pixel_count = width * height;
    
    let dehaze_amount = amount / 100.0;
    
    // Find atmospheric light (brightest region, likely haze)
    let mut max_brightness = 0.0f32;
    let mut atmospheric_light = [255u8, 255u8, 255u8];
    
    // Sample brightest pixels (every 100th pixel for performance)
    for i in (0..pixel_count).step_by(100) {
        let idx = i * 4;
        let brightness = (pixels[idx] as f32 + pixels[idx + 1] as f32 + pixels[idx + 2] as f32) / 3.0;
        if brightness > max_brightness {
            max_brightness = brightness;
            atmospheric_light = [pixels[idx], pixels[idx + 1], pixels[idx + 2]];
        }
    }
    
    // Apply dark channel prior dehaze
    for i in 0..pixel_count {
        let idx = i * 4;
        let r = pixels[idx] as f32;
        let g = pixels[idx + 1] as f32;
        let b = pixels[idx + 2] as f32;
        
        // Estimate transmission using dark channel prior
        let dark_channel = (r / atmospheric_light[0] as f32)
            .min(g / atmospheric_light[1] as f32)
            .min(b / atmospheric_light[2] as f32);
        
        let transmission = 1.0 - dehaze_amount * dark_channel;
        let t = transmission.max(0.1); // Prevent division by zero
        
        // Recover scene radiance
        let new_r = ((r - atmospheric_light[0] as f32 * (1.0 - t)) / t + atmospheric_light[0] as f32 * (1.0 - t)).clamp(0.0, 255.0);
        let new_g = ((g - atmospheric_light[1] as f32 * (1.0 - t)) / t + atmospheric_light[1] as f32 * (1.0 - t)).clamp(0.0, 255.0);
        let new_b = ((b - atmospheric_light[2] as f32 * (1.0 - t)) / t + atmospheric_light[2] as f32 * (1.0 - t)).clamp(0.0, 255.0);
        
        pixels[idx] = new_r as u8;
        pixels[idx + 1] = new_g as u8;
        pixels[idx + 2] = new_b as u8;
        
        // Boost contrast slightly for positive dehaze
        if dehaze_amount > 0.0 {
            let contrast = 1.0 + dehaze_amount * 0.2;
            pixels[idx] = (((pixels[idx] as f32 - 128.0) * contrast + 128.0).clamp(0.0, 255.0)) as u8;
            pixels[idx + 1] = (((pixels[idx + 1] as f32 - 128.0) * contrast + 128.0).clamp(0.0, 255.0)) as u8;
            pixels[idx + 2] = (((pixels[idx + 2] as f32 - 128.0) * contrast + 128.0).clamp(0.0, 255.0)) as u8;
        }
    }
}

/// Apply vignette effect (edge darkening or lightening)
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `strength` - Vignette strength (-100 to 100, positive darkens edges, negative lightens)
/// * `radius` - Vignette radius (0 to 100, percentage of image where vignette starts)
/// * `softness` - Vignette softness (0 to 100, controls falloff curve)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_vignette;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_vignette(&mut img, 50.0, 30.0, 50.0); // Darken edges
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_vignette(
    photon_image: &mut PhotonImage,
    strength: f32,
    radius: f32,
    softness: f32,
) {
    if strength == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    
    let strength_val = strength / 100.0;
    let radius_val = radius / 100.0;
    let softness_val = softness / 100.0;
    
    // Center of image
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    
    // Aspect ratio normalization for circular vignettes on non-square images
    let aspect_ratio = width as f32 / height as f32;
    let max_radius = (center_x * center_x + center_y * center_y).sqrt();
    
    // Inner and outer radius
    let inner_radius = max_radius * radius_val;
    let outer_radius = max_radius;
    
    // Smoothstep function for smooth transitions
    let smootherstep = |t: f32| -> f32 {
        let t = t.clamp(0.0, 1.0);
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    };
    
    // Strength multiplier: scales vignette intensity to prevent over-darkening
    // Typical range 0.3-0.5; lower = subtle, higher = dramatic
    const STRENGTH_MULTIPLIER: f32 = 0.35;
    
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            
            // Calculate normalized distance from center (circular on non-square images)
            let dx = (x as f32 - center_x) / aspect_ratio;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            
            // Calculate vignette factor
            let vignette_factor = if distance <= inner_radius {
                0.0
            } else if distance >= outer_radius {
                1.0
            } else {
                let t = (distance - inner_radius) / (outer_radius - inner_radius);
                smootherstep(t)
            };
            
            // Apply softness
            let vignette_factor = vignette_factor.powf(1.0 / (1.0 + softness_val * 2.0));
            
            // Calculate adjustment
            let adjustment = vignette_factor * strength_val * STRENGTH_MULTIPLIER;
            
            // Apply vignette effect
            for c in 0..3 {
                let pixel_val = pixels[idx + c] as f32;
                let new_val = if adjustment >= 0.0 {
                    // Darken: multiply toward black
                    pixel_val * (1.0 - adjustment)
                } else {
                    // Lighten: add toward white
                    pixel_val + (255.0 - pixel_val) * adjustment.abs()
                };
                pixels[idx + c] = new_val.clamp(0.0, 255.0) as u8;
            }
        }
    }
}

/// Apply tone zone adjustments (darks, shadows, highlights, whites)
/// Uses luminance-based masking
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `darks` - Darks adjustment (-100 to 100)
/// * `shadows` - Shadows adjustment (-100 to 100)
/// * `highlights` - Highlights adjustment (-100 to 100)
/// * `whites` - Whites adjustment (-100 to 100)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_tone_zones;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_tone_zones(&mut img, 10, 20, -10, 5); // Adjust tone zones
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_tone_zones(
    photon_image: &mut PhotonImage,
    darks: i32,
    shadows: i32,
    highlights: i32,
    whites: i32,
) {
    if darks == 0 && shadows == 0 && highlights == 0 && whites == 0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    
    for i in (0..pixels.len()).step_by(4) {
        let r = pixels[i] as f32;
        let g = pixels[i + 1] as f32;
        let b = pixels[i + 2] as f32;
        
        let luminance = 0.299 * r + 0.587 * g + 0.114 * b;
        let normalized_lum = luminance / 255.0;
        
        let mut adjustment = 0.0;
        
        if darks != 0 && normalized_lum < 0.25 {
            let mask = 1.0 - (normalized_lum / 0.25);
            adjustment += (darks as f32 / 100.0) * mask * 0.5;
        }
        
        if shadows != 0 && (0.25..0.5).contains(&normalized_lum) {
            let mask = 1.0 - ((normalized_lum - 0.375).abs() / 0.125);
            adjustment += (shadows as f32 / 100.0) * mask * 0.5;
        }
        
        if highlights != 0 && (0.5..0.75).contains(&normalized_lum) {
            let mask = 1.0 - ((normalized_lum - 0.625).abs() / 0.125);
            adjustment += (highlights as f32 / 100.0) * mask * 0.5;
        }
        
        if whites != 0 && normalized_lum >= 0.75 {
            let mask = (normalized_lum - 0.75) / 0.25;
            adjustment += (whites as f32 / 100.0) * mask * 0.5;
        }
        
        if adjustment != 0.0 {
            let adj_amount = adjustment * 255.0;
            pixels[i] = (r + adj_amount).clamp(0.0, 255.0) as u8;
            pixels[i + 1] = (g + adj_amount).clamp(0.0, 255.0) as u8;
            pixels[i + 2] = (b + adj_amount).clamp(0.0, 255.0) as u8;
        }
    }
}

/// Apply 3-way color grading (shadows/midtones/highlights)
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `shadow_hue` - Shadow hue adjustment (0-360 degrees)
/// * `shadow_sat` - Shadow saturation adjustment (-100 to 100)
/// * `shadow_lum` - Shadow luminance adjustment (-100 to 100)
/// * `midtone_hue` - Midtone hue adjustment (0-360 degrees)
/// * `midtone_sat` - Midtone saturation adjustment (-100 to 100)
/// * `midtone_lum` - Midtone luminance adjustment (-100 to 100)
/// * `highlight_hue` - Highlight hue adjustment (0-360 degrees)
/// * `highlight_sat` - Highlight saturation adjustment (-100 to 100)
/// * `highlight_lum` - Highlight luminance adjustment (-100 to 100)
/// * `blending` - Blending factor (0 to 100)
/// * `balance` - Balance between shadows and highlights (-100 to 100)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_color_grading;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_color_grading(&mut img, 200.0, 20.0, -10.0, 0.0, 0.0, 0.0, 30.0, 15.0, 5.0, 50.0, 0.0);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
#[allow(clippy::too_many_arguments)]
pub fn apply_color_grading(
    photon_image: &mut PhotonImage,
    shadow_hue: f32,
    shadow_sat: f32,
    shadow_lum: f32,
    midtone_hue: f32,
    midtone_sat: f32,
    midtone_lum: f32,
    highlight_hue: f32,
    highlight_sat: f32,
    highlight_lum: f32,
    blending: f32,
    balance: f32,
) {
    let pixels = &mut photon_image.raw_pixels;
    
    // Convert HSL to RGB offset
    let hsl_to_rgb_offset = |hue: f32, sat: f32| -> (f32, f32, f32) {
        if sat == 0.0 {
            return (0.0, 0.0, 0.0);
        }
        
        let h = (hue % 360.0) / 60.0;
        let s = sat / 100.0;
        let c = s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        
        let (r1, g1, b1) = if h < 1.0 {
            (c, x, 0.0)
        } else if h < 2.0 {
            (x, c, 0.0)
        } else if h < 3.0 {
            (0.0, c, x)
        } else if h < 4.0 {
            (0.0, x, c)
        } else if h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        ((r1 - 0.5) * 2.0 * s, (g1 - 0.5) * 2.0 * s, (b1 - 0.5) * 2.0 * s)
    };
    
    let shadow_color = hsl_to_rgb_offset(shadow_hue, shadow_sat);
    let midtone_color = hsl_to_rgb_offset(midtone_hue, midtone_sat);
    let highlight_color = hsl_to_rgb_offset(highlight_hue, highlight_sat);
    
    let color_strength = 0.8;
    let balance_shift = balance / 200.0;
    let blend_factor = blending / 100.0;
    let shadow_end = 0.25 + blend_factor * 0.15;
    let highlight_start = 0.75 - blend_factor * 0.15;
    
    for i in (0..pixels.len()).step_by(4) {
        let r = pixels[i] as f32 / 255.0;
        let g = pixels[i + 1] as f32 / 255.0;
        let b = pixels[i + 2] as f32 / 255.0;
        
        let lum = 0.299 * r + 0.587 * g + 0.114 * b;
        let adjusted_lum = (lum + balance_shift).clamp(0.0, 1.0);
        
        let (shadow_weight, midtone_weight, highlight_weight) = if adjusted_lum < shadow_end {
            let sw = 1.0 - (adjusted_lum / shadow_end);
            (sw, 1.0 - sw, 0.0)
        } else if adjusted_lum > highlight_start {
            let hw = (adjusted_lum - highlight_start) / (1.0 - highlight_start);
            (0.0, 1.0 - hw, hw)
        } else {
            (0.0, 1.0, 0.0)
        };
        
        let total_weight = shadow_weight + midtone_weight + highlight_weight;
        let (sw, mw, hw) = if total_weight > 0.0 {
            (shadow_weight / total_weight, midtone_weight / total_weight, highlight_weight / total_weight)
        } else {
            (shadow_weight, midtone_weight, highlight_weight)
        };
        
        let r_shift = (shadow_color.0 * sw + midtone_color.0 * mw + highlight_color.0 * hw) * color_strength;
        let g_shift = (shadow_color.1 * sw + midtone_color.1 * mw + highlight_color.1 * hw) * color_strength;
        let b_shift = (shadow_color.2 * sw + midtone_color.2 * mw + highlight_color.2 * hw) * color_strength;
        
        let lum_adjust = ((shadow_lum / 100.0) * sw + (midtone_lum / 100.0) * mw + (highlight_lum / 100.0) * hw) * 0.5;
        
        let new_r = (r + r_shift + lum_adjust).clamp(0.0, 1.0);
        let new_g = (g + g_shift + lum_adjust).clamp(0.0, 1.0);
        let new_b = (b + b_shift + lum_adjust).clamp(0.0, 1.0);
        
        pixels[i] = (new_r * 255.0) as u8;
        pixels[i + 1] = (new_g * 255.0) as u8;
        pixels[i + 2] = (new_b * 255.0) as u8;
    }
}

/// Apply tone curve lookup table
/// Applies a tone curve to the image's luminance while preserving color relationships
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `lookup_table` - A Vec<u8> containing 256 values representing the tone curve mapping.
///                    Each index represents an input value, and the value at that index is the output.
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_tone_curve;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// // Create a lookup table (e.g., linear curve: [0, 1, 2, ..., 255])
/// let mut lut = Vec::new();
/// for i in 0..256 {
///     lut.push(i as u8);
/// }
/// apply_tone_curve(&mut img, lut);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_tone_curve(photon_image: &mut PhotonImage, lookup_table: Vec<u8>) {
    if lookup_table.len() != 256 {
        panic!("Lookup table must contain exactly 256 values");
    }

    // Work directly with raw pixels for maximum performance
    let buf = photon_image.raw_pixels.as_mut_slice();
    
    // Apply tone curve to luminance
    // This preserves color relationships better than applying to RGB channels separately
    for i in (0..buf.len()).step_by(4) {
        if i + 2 < buf.len() {
            let r = buf[i] as f32;
            let g = buf[i + 1] as f32;
            let b = buf[i + 2] as f32;
            
            // Calculate luminance using standard formula (ITU-R BT.601)
            // Y = 0.299*R + 0.587*G + 0.114*B
            let luminance = (0.299 * r + 0.587 * g + 0.114 * b).round() as usize;
            let luminance_clamped = luminance.min(255);
            
            // Apply tone curve to luminance
            let new_luminance = lookup_table[luminance_clamped] as f32;
            
            // Calculate ratio to preserve color relationships
            let ratio = if luminance > 0 {
                new_luminance / luminance as f32
            } else {
                // Handle black pixels
                if new_luminance > 0.0 {
                    new_luminance / 255.0
                } else {
                    1.0
                }
            };
            
            // Apply ratio to each RGB channel
            let new_r = (r * ratio).clamp(0.0, 255.0) as u8;
            let new_g = (g * ratio).clamp(0.0, 255.0) as u8;
            let new_b = (b * ratio).clamp(0.0, 255.0) as u8;
            
            buf[i] = new_r;
            buf[i + 1] = new_g;
            buf[i + 2] = new_b;
            // Alpha channel (buf[i + 3]) remains unchanged
        }
    }
}

/// Apply unsharp mask sharpening
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `amount` - Sharpening strength (0 to 150, typical 50-150)
/// * `radius` - Edge detection radius in pixels (0.5 to 3.0, typical 0.8-1.5)
/// * `threshold` - Minimum brightness difference to sharpen (0 to 255, typical 0-5)
/// * `masking` - Edge masking (0 to 100, 0 = sharpen everywhere, 100 = edges only)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_sharpening;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_sharpening(&mut img, 100.0, 1.0, 2.0, 50.0);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_sharpening(
    photon_image: &mut PhotonImage,
    amount: f32,
    radius: f32,
    threshold: f32,
    masking: f32,
) {
    if amount == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    
    // Algorithm: sharpened = original + (original - blurred) * amount
    // This is the standard unsharp mask formula
    
    let blur_radius = (radius.clamp(0.5, 3.0) * 2.0) as usize;
    let amount_factor = amount / 100.0;
    let threshold_val = threshold;
    
    // Create luminance channel for edge detection
    let mut luminance: Vec<f32> = vec![0.0; width * height];
    for (i, lum) in luminance.iter_mut().enumerate() {
        let idx = i * 4;
        let r = pixels[idx] as f32;
        let g = pixels[idx + 1] as f32;
        let b = pixels[idx + 2] as f32;
        *lum = 0.299 * r + 0.587 * g + 0.114 * b;
    }
    
    // Box blur for "unsharp" part (horizontal pass)
    let mut blurred = luminance.clone();
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            let mut count = 0;
            for bx in x.saturating_sub(blur_radius)..=(x + blur_radius).min(width - 1) {
                sum += luminance[y * width + bx];
                count += 1;
            }
            blurred[y * width + x] = sum / count as f32;
        }
    }
    
    // Vertical pass
    let mut blurred2 = blurred.clone();
    for y in 0..height {
        for x in 0..width {
            let mut sum = 0.0;
            let mut count = 0;
            for by in y.saturating_sub(blur_radius)..=(y + blur_radius).min(height - 1) {
                sum += blurred[by * width + x];
                count += 1;
            }
            blurred2[y * width + x] = sum / count as f32;
        }
    }
    
    // Calculate edge mask if masking > 0
    let mut edge_mask = vec![1.0f32; width * height];
    if masking > 0.0 {
        // Sobel edge detection for masking
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let gx = luminance[(y - 1) * width + (x + 1)] 
                       - luminance[(y - 1) * width + (x - 1)]
                       + 2.0 * luminance[y * width + (x + 1)]
                       - 2.0 * luminance[y * width + (x - 1)]
                       + luminance[(y + 1) * width + (x + 1)]
                       - luminance[(y + 1) * width + (x - 1)];
                
                let gy = luminance[(y + 1) * width + (x - 1)]
                       + 2.0 * luminance[(y + 1) * width + x]
                       + luminance[(y + 1) * width + (x + 1)]
                       - luminance[(y - 1) * width + (x - 1)]
                       - 2.0 * luminance[(y - 1) * width + x]
                       - luminance[(y - 1) * width + (x + 1)];
                
                let edge_strength = (gx * gx + gy * gy).sqrt() / 255.0;
                edge_mask[y * width + x] = edge_strength.powf(1.0 - masking / 100.0);
            }
        }
    }
    
    // Apply unsharp mask
    for i in 0..(width * height) {
        let idx = i * 4;
        let orig_lum = luminance[i];
        let blur_lum = blurred2[i];
        
        // High-pass component (detail)
        let detail = orig_lum - blur_lum;
        
        // Apply threshold
        if detail.abs() >= threshold_val {
            // Apply masking
            let mask_factor = edge_mask[i];
            let adjustment = detail * amount_factor * mask_factor;
            
            // Apply to each channel proportionally
            if orig_lum > 0.001 {
                let factor = (orig_lum + adjustment) / orig_lum;
                pixels[idx] = (pixels[idx] as f32 * factor).clamp(0.0, 255.0) as u8;
                pixels[idx + 1] = (pixels[idx + 1] as f32 * factor).clamp(0.0, 255.0) as u8;
                pixels[idx + 2] = (pixels[idx + 2] as f32 * factor).clamp(0.0, 255.0) as u8;
            }
        }
    }
}

/// Apply noise reduction using bilateral filtering (edge-preserving)
/// 
/// Reduces noise while preserving edges
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `luminance` - Luminance noise reduction (0 to 100)
/// * `color` - Color noise reduction (0 to 100)
/// * `detail` - Detail preservation (0 to 100, higher = preserve more detail)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_noise_reduction_bilateral;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_noise_reduction_bilateral(&mut img, 40.0, 50.0, 50.0);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_noise_reduction_bilateral(
    photon_image: &mut PhotonImage,
    luminance: f32,
    color: f32,
    detail: f32,
) {
    if luminance == 0.0 && color == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    
    // Bilateral filter parameters
    let spatial_sigma = 2.0; // Spatial domain falloff
    let lum_intensity_sigma = 25.0 * (1.0 - detail / 100.0); // Intensity domain falloff
    let color_intensity_sigma = 15.0;
    
    let lum_strength = luminance / 100.0;
    let color_strength = color / 100.0;
    
    let original_pixels = pixels.clone();
    
    // Bilateral filter implementation
    for y in 0..height {
        for x in 0..width {
            let center_idx = (y * width + x) * 4;
            
            let center_r = original_pixels[center_idx] as f32;
            let center_g = original_pixels[center_idx + 1] as f32;
            let center_b = original_pixels[center_idx + 2] as f32;
            let center_lum = 0.299 * center_r + 0.587 * center_g + 0.114 * center_b;
            
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut sum_weight = 0.0;
            
            // Neighborhood size based on noise reduction strength
            let radius = 2 + ((luminance.max(color) / 50.0) as usize);
            
            for ny in y.saturating_sub(radius)..=(y + radius).min(height - 1) {
                for nx in x.saturating_sub(radius)..=(x + radius).min(width - 1) {
                    let neighbor_idx = (ny * width + nx) * 4;
                    
                    let n_r = original_pixels[neighbor_idx] as f32;
                    let n_g = original_pixels[neighbor_idx + 1] as f32;
                    let n_b = original_pixels[neighbor_idx + 2] as f32;
                    let n_lum = 0.299 * n_r + 0.587 * n_g + 0.114 * n_b;
                    
                    // Spatial distance weight
                    let dx = (x as i32 - nx as i32) as f32;
                    let dy = (y as i32 - ny as i32) as f32;
                    let spatial_dist = dx * dx + dy * dy;
                    let spatial_weight = (-spatial_dist / (2.0 * spatial_sigma * spatial_sigma)).exp();
                    
                    // Intensity difference weight (for luminance)
                    let lum_diff = (center_lum - n_lum).abs();
                    let intensity_weight = (-lum_diff * lum_diff / (2.0 * lum_intensity_sigma * lum_intensity_sigma)).exp();
                    
                    // Color difference weight
                    let color_diff = ((center_r - n_r).powi(2) + (center_g - n_g).powi(2) + (center_b - n_b).powi(2)).sqrt();
                    let color_weight = (-color_diff / (2.0 * color_intensity_sigma * color_intensity_sigma)).exp();
                    
                    // Combined weight
                    // When strength is 0, use full weight (no filtering). When strength is 1, use intensity/color weights.
                    let lum_weight = if lum_strength > 0.0 {
                        intensity_weight * lum_strength + (1.0 - lum_strength)
                    } else {
                        1.0
                    };
                    let col_weight = if color_strength > 0.0 {
                        color_weight * color_strength + (1.0 - color_strength)
                    } else {
                        1.0
                    };
                    let weight = spatial_weight * lum_weight * col_weight;
                    
                    sum_r += n_r * weight;
                    sum_g += n_g * weight;
                    sum_b += n_b * weight;
                    sum_weight += weight;
                }
            }
            
            if sum_weight > 0.0 {
                pixels[center_idx] = (sum_r / sum_weight).clamp(0.0, 255.0) as u8;
                pixels[center_idx + 1] = (sum_g / sum_weight).clamp(0.0, 255.0) as u8;
                pixels[center_idx + 2] = (sum_b / sum_weight).clamp(0.0, 255.0) as u8;
            }
        }
    }
}

/// Apply noise reduction using wavelet denoising
/// 
/// Uses wavelet transform to separate signal from noise, then reconstructs with reduced noise
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `strength` - Noise reduction strength (0 to 100)
/// * `threshold` - Wavelet threshold (0 to 100, higher = more aggressive)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_noise_reduction_wavelets;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_noise_reduction_wavelets(&mut img, 50.0, 30.0);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_noise_reduction_wavelets(
    photon_image: &mut PhotonImage,
    strength: f32,
    threshold: f32,
) {
    if strength == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    
    let strength_val = strength / 100.0;
    // Threshold controls aggressiveness independently from strength
    // Strength controls blending between original and denoised
    let threshold_val = threshold / 100.0 * 30.0;
    
    // Check if dimensions are powers of two (required for proper Haar wavelets)
    // For non-power-of-two, use simplified blur-based approach
    let width_pow2 = width.is_power_of_two();
    let height_pow2 = height.is_power_of_two();
    
    if !width_pow2 || !height_pow2 {
        // Simplified approach for non-power-of-two dimensions
        let original_pixels = pixels.clone();
        for channel in 0..3 {
            let mut channel_data: Vec<f32> = vec![0.0; width * height];
            for i in 0..(width * height) {
                channel_data[i] = original_pixels[i * 4 + channel] as f32;
            }
            
            // Apply box blur for smoothing
            let mut smoothed = channel_data.clone();
            let blur_radius = 2i32;
            for y in 0..height {
                for x in 0..width {
                    let mut sum = 0.0;
                    let mut count = 0;
                    for dy in -blur_radius..=blur_radius {
                        for dx in -blur_radius..=blur_radius {
                            let ny = (y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                            let nx = (x as i32 + dx).clamp(0, width as i32 - 1) as usize;
                            sum += channel_data[ny * width + nx];
                            count += 1;
                        }
                    }
                    smoothed[y * width + x] = sum / count as f32;
                }
            }
            
            // Calculate detail and apply soft thresholding
            for i in 0..(width * height) {
                let detail = channel_data[i] - smoothed[i];
                let thresholded_detail = if detail.abs() > threshold_val {
                    detail.signum() * (detail.abs() - threshold_val)
                } else {
                    0.0
                };
                pixels[i * 4 + channel] = (smoothed[i] + thresholded_detail * strength_val).clamp(0.0, 255.0) as u8;
            }
        }
    } else {
        // Proper Haar wavelet transform for power-of-two dimensions
        let original_pixels = pixels.clone();
        for channel in 0..3 {
            // Extract channel data
            let mut channel_data: Vec<f32> = vec![0.0; width * height];
            for i in 0..(width * height) {
                channel_data[i] = original_pixels[i * 4 + channel] as f32;
            }
            
            // Forward Haar wavelet transform
            // Step 1: Horizontal pass on each row
            let mut temp_h = vec![0.0f32; width * height];
            for y in 0..height {
                for x in 0..(width / 2) {
                    let idx1 = y * width + x * 2;
                    let idx2 = y * width + x * 2 + 1;
                    let avg = (channel_data[idx1] + channel_data[idx2]) / 2.0;
                    let diff = (channel_data[idx1] - channel_data[idx2]) / 2.0;
                    
                    // Store in separate buffer to avoid overwriting input
                    temp_h[y * width + x] = avg;                    // Left half: averages
                    temp_h[y * width + (width / 2) + x] = diff;     // Right half: horizontal details
                }
            }
            
            // Step 2: Vertical pass on each column
            // Produces 4 quadrants: LL (top-left), LH (top-right), HL (bottom-left), HH (bottom-right)
            let mut transformed = vec![0.0f32; width * height];
            for x in 0..width {
                for y in 0..(height / 2) {
                    let idx1 = y * 2 * width + x;
                    let idx2 = (y * 2 + 1) * width + x;
                    let avg = (temp_h[idx1] + temp_h[idx2]) / 2.0;
                    let diff = (temp_h[idx1] - temp_h[idx2]) / 2.0;
                    
                    // Write to correct quadrants
                    transformed[y * width + x] = avg;
                    transformed[(height / 2 + y) * width + x] = diff;
                }
            }
            
            // Apply soft thresholding to detail coefficients only
            // Preserve LL quadrant (top-left: y < height/2 && x < width/2), threshold LH, HL, HH
            for y in 0..height {
                for x in 0..width {
                    let i = y * width + x;
                    
                    // Preserve LL quadrant (approximation) - top-left quadrant
                    if y < height / 2 && x < width / 2 {
                        continue; // Skip LL approximation coefficients
                    }
                    
                    // Standard soft thresholding for detail coefficients (LH, HL, HH)
                    let val = transformed[i];
                    if val.abs() > threshold_val {
                        transformed[i] = val.signum() * (val.abs() - threshold_val);
                    } else {
                        transformed[i] = 0.0;
                    }
                }
            }
            
            // Inverse Haar wavelet transform
            // Step 1: Vertical inverse pass
            let mut temp_v = vec![0.0f32; width * height];
            for x in 0..width {
                for y in 0..(height / 2) {
                    let avg = transformed[y * width + x];
                    let diff = transformed[(height / 2 + y) * width + x];
                    temp_v[y * 2 * width + x] = avg + diff;
                    temp_v[(y * 2 + 1) * width + x] = avg - diff;
                }
            }
            
            // Step 2: Horizontal inverse pass
            let mut reconstructed = vec![0.0f32; width * height];
            for y in 0..height {
                for x in 0..(width / 2) {
                    let avg = temp_v[y * width + x];
                    let diff = temp_v[y * width + (width / 2) + x];
                    reconstructed[y * width + x * 2] = avg + diff;
                    reconstructed[y * width + x * 2 + 1] = avg - diff;
                }
            }
            
            // Blend original and denoised based on strength
            for i in 0..(width * height) {
                let original = channel_data[i];
                let denoised = reconstructed[i];
                let blended = original * (1.0 - strength_val) + denoised * strength_val;
                pixels[i * 4 + channel] = blended.clamp(0.0, 255.0) as u8;
            }
        }
    }
}

/// Apply noise reduction using median filter
/// 
/// Effective for salt-and-pepper noise, preserves edges well
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `radius` - Filter radius (1 to 3, typically 1-2)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_noise_reduction_median;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_noise_reduction_median(&mut img, 2);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_noise_reduction_median(
    photon_image: &mut PhotonImage,
    radius: usize,
) {
    if radius == 0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    
    let radius = radius.min(3);
    let original_pixels = pixels.clone();
    
    // Apply median filter to each channel
    for channel in 0..3 {
        for y in 0..height {
            for x in 0..width {
                let mut neighbors = Vec::new();
                
                // Collect neighbors
                for ny in y.saturating_sub(radius)..=(y + radius).min(height - 1) {
                    for nx in x.saturating_sub(radius)..=(x + radius).min(width - 1) {
                        let idx = (ny * width + nx) * 4;
                        neighbors.push(original_pixels[idx + channel] as u32);
                    }
                }
                
                // Calculate median
                neighbors.sort();
                let median = if neighbors.len() % 2 == 0 {
                    (neighbors[neighbors.len() / 2 - 1] + neighbors[neighbors.len() / 2]) / 2
                } else {
                    neighbors[neighbors.len() / 2]
                };
                
                let idx = (y * width + x) * 4;
                pixels[idx + channel] = median as u8;
            }
        }
    }
}

/// Apply noise reduction using non-local means algorithm
/// 
/// Advanced algorithm that compares similar patches across the image
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `strength` - Noise reduction strength (0 to 100)
/// * `patch_size` - Patch size for comparison (3 or 5)
/// * `search_radius` - Search radius for similar patches (5 to 15)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_noise_reduction_nlm;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_noise_reduction_nlm(&mut img, 50.0, 3, 10);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_noise_reduction_nlm(
    photon_image: &mut PhotonImage,
    strength: f32,
    patch_size: usize,
    search_radius: usize,
) {
    if strength == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    
    let strength_val = strength / 100.0;
    let patch_size = patch_size.clamp(3, 5);
    let search_radius = search_radius.clamp(5, 15);
    let h = 10.0 * (1.0 - strength_val) + 2.0; // Filter parameter
    
    let original_pixels = pixels.clone();
    
    // Process each pixel
    for y in 0..height {
        for x in 0..width {
            let center_idx = (y * width + x) * 4;
            
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut sum_weight = 0.0;
            
            // Search window
            let y_start = y.saturating_sub(search_radius);
            let y_end = (y + search_radius + 1).min(height);
            let x_start = x.saturating_sub(search_radius);
            let x_end = (x + search_radius + 1).min(width);
            
            // Compare with each pixel in search window
            for ny in y_start..y_end {
                for nx in x_start..x_end {
                    let neighbor_idx = (ny * width + nx) * 4;
                    
                    // Compute patch distance (simplified - just center pixel difference for performance)
                    let patch_dist = {
                        let mut dist = 0.0;
                        let patch_half = patch_size / 2;
                        
                        for py in 0..patch_size {
                            for px in 0..patch_size {
                                let cy = y.saturating_sub(patch_half).saturating_add(py);
                                let cx = x.saturating_sub(patch_half).saturating_add(px);
                                let ny_p = ny.saturating_sub(patch_half).saturating_add(py);
                                let nx_p = nx.saturating_sub(patch_half).saturating_add(px);
                                
                                if cy < height && cx < width && ny_p < height && nx_p < width {
                                    let idx1 = (cy * width + cx) * 4;
                                    let idx2 = (ny_p * width + nx_p) * 4;
                                    
                                    let dr = (original_pixels[idx1] as f32 - original_pixels[idx2] as f32).powi(2);
                                    let dg = (original_pixels[idx1 + 1] as f32 - original_pixels[idx2 + 1] as f32).powi(2);
                                    let db = (original_pixels[idx1 + 2] as f32 - original_pixels[idx2 + 2] as f32).powi(2);
                                    dist += dr + dg + db;
                                }
                            }
                        }
                        dist
                    };
                    
                    // Weight based on patch similarity
                    let weight = (-patch_dist / (h * h)).exp();
                    
                    sum_r += original_pixels[neighbor_idx] as f32 * weight;
                    sum_g += original_pixels[neighbor_idx + 1] as f32 * weight;
                    sum_b += original_pixels[neighbor_idx + 2] as f32 * weight;
                    sum_weight += weight;
                }
            }
            
            if sum_weight > 0.0 {
                let denoised_r = sum_r / sum_weight;
                let denoised_g = sum_g / sum_weight;
                let denoised_b = sum_b / sum_weight;
                
                // Blend with original based on strength
                let original_r = original_pixels[center_idx] as f32;
                let original_g = original_pixels[center_idx + 1] as f32;
                let original_b = original_pixels[center_idx + 2] as f32;
                
                pixels[center_idx] = ((original_r * (1.0 - strength_val) + denoised_r * strength_val).clamp(0.0, 255.0)) as u8;
                pixels[center_idx + 1] = ((original_g * (1.0 - strength_val) + denoised_g * strength_val).clamp(0.0, 255.0)) as u8;
                pixels[center_idx + 2] = ((original_b * (1.0 - strength_val) + denoised_b * strength_val).clamp(0.0, 255.0)) as u8;
            }
        }
    }
}

/// Apply noise reduction (default: bilateral filtering)
/// 
/// Convenience function that calls bilateral filtering
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `luminance` - Luminance noise reduction (0 to 100)
/// * `color` - Color noise reduction (0 to 100)
/// * `detail` - Detail preservation (0 to 100, higher = preserve more detail)
/// 
/// # Example
/// ```no_run
/// use photon_rs::adjustments::apply_noise_reduction;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_noise_reduction(&mut img, 40.0, 50.0, 50.0);
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_noise_reduction(
    photon_image: &mut PhotonImage,
    luminance: f32,
    color: f32,
    detail: f32,
) {
    apply_noise_reduction_bilateral(photon_image, luminance, color, detail);
}

