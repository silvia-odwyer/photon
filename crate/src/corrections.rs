//! Fixing technical and optical problems.
//! This module provides automated fixes for camera and lens defects, such as
//! lens distortion, chromatic aberration, vignetting, and perspective correction.

use crate::PhotonImage;

#[cfg(feature = "enable_wasm")]
use wasm_bindgen::prelude::*;

/// Remove chromatic aberration (purple and green fringing)
/// Common in high-contrast edges from lens imperfections
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `purple_amount` - Purple fringing removal amount (0 to 100)
/// * `green_amount` - Green fringing removal amount (0 to 100)
/// 
/// # Example
/// ```no_run
/// use photon_rs::corrections::apply_chromatic_aberration;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_chromatic_aberration(&mut img, 50.0, 30.0); // Remove purple and green fringing
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_chromatic_aberration(
    photon_image: &mut PhotonImage,
    purple_amount: f32,
    green_amount: f32,
) {
    if purple_amount == 0.0 && green_amount == 0.0 {
        return;
    }
    
    let pixels = &mut photon_image.raw_pixels;
    
    for i in (0..pixels.len()).step_by(4) {
        let r = pixels[i] as f32;
        let g = pixels[i + 1] as f32;
        let b = pixels[i + 2] as f32;
        
        // Detect purple fringe
        if purple_amount > 0.0 {
            let is_purplish = r > 100.0 && b > 100.0 && g < r.min(b) * 0.8;
            if is_purplish {
                let purple_strength = ((r + b - g * 2.0) / 255.0).min(1.0) * (purple_amount / 100.0);
                let avg = (r + g + b) / 3.0;
                pixels[i] = (r * (1.0 - purple_strength) + avg * purple_strength) as u8;
                pixels[i + 2] = (b * (1.0 - purple_strength) + avg * purple_strength) as u8;
            }
        }
        
        // Detect green fringe
        if green_amount > 0.0 {
            let is_greenish = g > 100.0 && r < g * 0.8 && b < g * 0.8;
            if is_greenish {
                let green_strength = ((g * 2.0 - r - b) / 255.0).min(1.0) * (green_amount / 100.0);
                let avg = (r + g + b) / 3.0;
                pixels[i + 1] = (g * (1.0 - green_strength) + avg * green_strength) as u8;
            }
        }
    }
}

/// Apply lens distortion correction (barrel/pincushion) with optional vignette removal
/// Uses bilinear interpolation for smooth results
/// 
/// # Arguments
/// * `photon_image` - A PhotonImage to process
/// * `distortion` - Distortion amount (-100 to 100, negative for barrel, positive for pincushion)
/// * `vignette` - Vignette correction amount (0 to 100)
/// 
/// # Example
/// ```no_run
/// use photon_rs::corrections::apply_lens_correction;
/// use photon_rs::native::open_image;
/// 
/// let mut img = open_image("img.jpg").expect("File should open");
/// apply_lens_correction(&mut img, -20.0, 30.0); // Correct barrel distortion and vignette
/// ```
#[cfg_attr(feature = "enable_wasm", wasm_bindgen)]
pub fn apply_lens_correction(
    photon_image: &mut PhotonImage,
    distortion: f32,
    vignette: f32,
) {
    if distortion == 0.0 && vignette == 0.0 {
        return;
    }
    
    let width = photon_image.width as usize;
    let height = photon_image.height as usize;
    
    // Clone the original pixels for reading
    let source_pixels = photon_image.raw_pixels.clone();
    let dest_pixels = &mut photon_image.raw_pixels;
    
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_radius = (center_x * center_x + center_y * center_y).sqrt();
    
    let distortion_k = distortion / 1000.0;
    let vignette_amount = vignette / 100.0;
    
    for y in 0..height {
        for x in 0..width {
            let dest_idx = (y * width + x) * 4;
            
            let nx = (x as f32 - center_x) / max_radius;
            let ny = (y as f32 - center_y) / max_radius;
            let r = (nx * nx + ny * ny).sqrt();
            
            // Apply distortion
            let (src_x, src_y) = if distortion_k != 0.0 {
                let distortion_factor = 1.0 + distortion_k * r * r;
                (
                    center_x + nx * max_radius * distortion_factor,
                    center_y + ny * max_radius * distortion_factor,
                )
            } else {
                (x as f32, y as f32)
            };
            
            // Bilinear interpolation
            let sx = src_x.floor() as i32;
            let sy = src_y.floor() as i32;
            let fx = src_x - sx as f32;
            let fy = src_y - sy as f32;
            
            if sx >= 0 && sx < (width - 1) as i32 && sy >= 0 && sy < (height - 1) as i32 {
                let idx00 = (sy as usize * width + sx as usize) * 4;
                let idx01 = (sy as usize * width + (sx + 1) as usize) * 4;
                let idx10 = ((sy + 1) as usize * width + sx as usize) * 4;
                let idx11 = ((sy + 1) as usize * width + (sx + 1) as usize) * 4;
                
                for c in 0..3 {
                    let v00 = source_pixels[idx00 + c] as f32;
                    let v01 = source_pixels[idx01 + c] as f32;
                    let v10 = source_pixels[idx10 + c] as f32;
                    let v11 = source_pixels[idx11 + c] as f32;
                    
                    let mut value = v00 * (1.0 - fx) * (1.0 - fy)
                        + v01 * fx * (1.0 - fy)
                        + v10 * (1.0 - fx) * fy
                        + v11 * fx * fy;
                    
                    // Apply vignette correction
                    if vignette_amount != 0.0 {
                        let vignette_factor = r * r * vignette_amount * 0.5;
                        value = (value + vignette_factor * 255.0).clamp(0.0, 255.0);
                    }
                    
                    dest_pixels[dest_idx + c] = value as u8;
                }
                dest_pixels[dest_idx + 3] = 255;
            } else {
                // Out of bounds - black
                dest_pixels[dest_idx] = 0;
                dest_pixels[dest_idx + 1] = 0;
                dest_pixels[dest_idx + 2] = 0;
                dest_pixels[dest_idx + 3] = 255;
            }
        }
    }
}
