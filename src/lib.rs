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

fn hex_color_to_rgb (hex_colour: &str) -> [u32;3] {
    let r = hex_to_u32(&hex_colour[1..3]);
    let g = hex_to_u32(&hex_colour[3..5]);
    let b = hex_to_u32(&hex_colour[5..7]);
    return [r, g, b]
}

fn hex_to_u32(hex_string: &str) -> u32 {
    // Hex string to 4-bytes, aka. u32
    let parsed_int:u32 = u32::from_str_radix(hex_string, 16).unwrap();
    return parsed_int;
}

fn rgb_to_hsl(rgb: [u32;3]) -> [u32;3] {
    let [r,g,b] =rgb;
    let min = cmp::min(r, cmp::min(g,b)) as f32;
    let max = cmp::max(r, cmp::max(g,b)) as f32;
    let delta  = max - min;
    let l = min + max/2.;
    
    let mut s = 0.0;
    if l > 0. && l < 1. {
        let denom = if l < 0.5 {2. * l} else {2. - 2. * l};
        s = delta / denom;
    }

    let mut h = 0.0;
    if delta > 0. {
        let rf = r as f32;
        let gf = g as f32;
        let bf = b as f32;

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

    return [(h * 255.) as u32, (s * 255.) as u32, (l * 255.) as u32 ];
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, ntc!");
}
