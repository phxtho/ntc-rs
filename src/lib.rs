mod utils;

use wasm_bindgen::prelude::*;
use std::cmp;

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
    let r = hex_to_u32(&hex_colour[1..3]);
    let g = hex_to_u32(&hex_colour[3..5]);
    let b = hex_to_u32(&hex_colour[5..7]);
    return vec![r, g, b]
}

fn hex_to_u32(hex_string: &str) -> u32 {
    // Hex string to 4-bytes, aka. u32
    let parsed_int:u32 = u32::from_str_radix(hex_string, 16).unwrap();
    return parsed_int;
}

#[wasm_bindgen]
pub fn rgb_to_hsl(rgb: Vec<u32>) -> Vec<u32> {
    if rgb.len() != 3 {
        return vec![0,0,0]
    }

    let min = *rgb.iter().min().unwrap() as f32;
    let max = *rgb.iter().max().unwrap() as f32;
    let delta  = max - min;
    let l = min + max/2.;
    
    let mut s = 0.0;
    if l > 0. && l < 1. {
        let denom = if l < 0.5 {2. * l} else {2. - 2. * l};
        s = delta / denom;
    }

    let mut h = 0.0;
    if delta > 0. {
        let rf = rgb[0] as f32;
        let gf = rgb[1] as f32;
        let bf = rgb[2] as f32;

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

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, ntc!");
}
