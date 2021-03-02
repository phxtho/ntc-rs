mod utils;
mod colour;

use wasm_bindgen::prelude::*;
use utils::hex_to_u32;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hex_color_to_rgb (hex_colour: &str) -> Vec<u32> {
    let r = hex_to_u32(&hex_colour[0..=1]);
    let g = hex_to_u32(&hex_colour[2..=3]);
    let b = hex_to_u32(&hex_colour[4..=5]);
    return vec![r, g, b]
}

#[wasm_bindgen]
pub fn rgb_to_hsl(rgb: Vec<u32>) -> Vec<u32> {
    if rgb.len() != 3 {
        return vec![0,0,0]
    }

    let normalised_rgb: Vec<f32> = rgb.iter().map(|&x| x as f32 / 255. ).collect();

    let min = *rgb.iter().min().unwrap() as f32 / 255.;
    let max = *rgb.iter().max().unwrap() as f32 / 255.;
    let delta  = max - min;
    let l = (min + max)/2.;
    
    let mut s = 0.0;
    if l > 0. && l < 1. {
        let denom = if l < 0.5 {2. * l} else {2. - 2. * l};
        s = delta / denom;
    }

    let mut h = 0.0;
    if delta > 0. {
        let rf = normalised_rgb[0] as f32;
        let gf = normalised_rgb[1] as f32;
        let bf = normalised_rgb[2] as f32;

        if max == rf && max != gf  {
            h += (gf-bf)/delta
        }
        
        if max == gf && max != bf {
            h += 2. + (bf - rf) / delta;
        } 

        if max == bf && max != rf {
            h+= 4. + (rf - gf) / delta
        }
        h /= 6.;
    }

    return vec![(h * 255.) as u32, (s * 255.) as u32, (l * 255.) as u32 ];
}

#[test]
fn test_hex_color_to_rgb () {
    assert_eq!(vec![0,0,0], hex_color_to_rgb("000000")); // Black
    assert_eq!(vec![128,128,128], hex_color_to_rgb("808080")); // Grey
    assert_eq!(vec![255,255,255], hex_color_to_rgb("FFFFFF")); // White
}

#[test]
fn test_rgb_to_hsl () {
    assert_eq!(vec![0,0,0], rgb_to_hsl(vec![0,0,0])); // Black
    assert_eq!(vec![0,0,128], rgb_to_hsl(vec![128,128,128])); // Grey
    assert_eq!(vec![0,0,255], rgb_to_hsl(vec![255,255,255])); // White
}