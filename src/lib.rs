mod utils;
mod colour;

use wasm_bindgen::prelude::*;
use utils::hex_to_i32;
use colour::{Colour, ALL_COLOURS};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hex_color_to_rgb (hex_colour: &str) -> Vec<i32> {
    let r = hex_to_i32(&hex_colour[0..=1]);
    let g = hex_to_i32(&hex_colour[2..=3]);
    let b = hex_to_i32(&hex_colour[4..=5]);
    return vec![r, g , b ];
}

#[test]
fn test_hex_color_to_rgb () {
    assert_eq!(vec![0,0,0], hex_color_to_rgb("000000")); // Black
    assert_eq!(vec![128,128,128], hex_color_to_rgb("808080")); // Grey
    assert_eq!(vec![255,255,255], hex_color_to_rgb("FFFFFF")); // White
}

#[wasm_bindgen]
pub fn rgb_to_hsl(rgb: Vec<i32>) -> Vec<i32> {
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

    return vec![(h * 255.) as i32, (s * 255.) as i32, (l * 255.) as i32 ];
}

#[test]
fn test_rgb_to_hsl () {
    assert_eq!(vec![0,0,0], rgb_to_hsl(vec![0,0,0])); // Black
    assert_eq!(vec![0,0,128], rgb_to_hsl(vec![128,128,128])); // Grey
    assert_eq!(vec![0,0,255], rgb_to_hsl(vec![255,255,255])); // White
}

#[wasm_bindgen]
pub fn closest_colour(rgb : Vec<i32>) -> Colour {
    let mut _ndf = 0;
    let mut cl = Colour { name: "Black",hex: "000000",rgb: &[0, 0, 0],hsl: &[0, 0, 0],shade: "Black"};
    let mut df = -1;
    let hsl = rgb_to_hsl(rgb.clone());

    for colour in ALL_COLOURS.iter() {
        let rgb_distance = (rgb[0] - colour.rgb[0]).pow(2) 
            + (rgb[1] - colour.rgb[1]).pow(2) 
            + (rgb[2] - colour.rgb[2]).pow(2);

        let hsl_distance = (hsl[0] - colour.hsl[0]).pow(2) 
            + (hsl[1] - colour.hsl[1]).pow(2) 
            + (hsl[2] - colour.hsl[2]).pow(2);

        _ndf = rgb_distance + hsl_distance * 2;

        if df < 0 || df > _ndf {
            df = _ndf;
            cl = Colour {name: colour.name, hex: colour.hex, rgb: colour.rgb, hsl: colour.hsl, shade: colour.shade}
        }
    }
    return cl;
}

#[test]
fn test_closest_colour() {
    assert_eq!("White Ice", closest_colour(vec![215, 238, 228]).name);
}


