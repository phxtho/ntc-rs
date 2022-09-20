mod colour;
mod utils;

use colour::{Colour, ALL_COLOURS};
use utils::hex_to_i32;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hex_color_to_rgb(hex_colour: &str) -> Vec<i32> {
    let r = hex_to_i32(&hex_colour[0..=1]);
    let g = hex_to_i32(&hex_colour[2..=3]);
    let b = hex_to_i32(&hex_colour[4..=5]);
    return vec![r, g, b];
}

#[test]
fn test_hex_color_to_rgb() {
    assert_eq!(vec![0, 0, 0], hex_color_to_rgb("000000")); // Black
    assert_eq!(vec![128, 128, 128], hex_color_to_rgb("808080")); // Grey
    assert_eq!(vec![255, 255, 255], hex_color_to_rgb("FFFFFF")); // White
}

#[wasm_bindgen]
pub fn rgb_to_hsl(rgb: Vec<i32>) -> Vec<i32> {
    if rgb.len() != 3 {
        return vec![0, 0, 0];
    }

    let normalised_rgb: Vec<f32> = rgb.iter().map(|&x| x as f32 / 255.).collect();

    let min = *rgb.iter().min().unwrap() as f32 / 255.;
    let max = *rgb.iter().max().unwrap() as f32 / 255.;
    let delta = max - min;
    let l = (min + max) / 2.;
    let mut s = 0.0;
    if l > 0. && l < 1. {
        let denom = if l < 0.5 { 2. * l } else { 2. - 2. * l };
        s = delta / denom;
    }

    let mut h = 0.0;
    if delta > 0. {
        let rf = normalised_rgb[0] as f32;
        let gf = normalised_rgb[1] as f32;
        let bf = normalised_rgb[2] as f32;

        if max == rf && max != gf {
            h += (gf - bf) / delta
        }
        if max == gf && max != bf {
            h += 2. + (bf - rf) / delta;
        }

        if max == bf && max != rf {
            h += 4. + (rf - gf) / delta
        }
        h /= 6.;
    }

    return vec![(h * 255.) as i32, (s * 255.) as i32, (l * 255.) as i32];
}

#[test]
fn test_rgb_to_hsl() {
    assert_eq!(vec![0, 0, 0], rgb_to_hsl(vec![0, 0, 0])); // Black
    assert_eq!(vec![0, 0, 128], rgb_to_hsl(vec![128, 128, 128])); // Grey
    assert_eq!(vec![0, 0, 255], rgb_to_hsl(vec![255, 255, 255])); // White
}

#[wasm_bindgen]
pub fn closest_colour(rgb: Vec<i32>) -> Colour {
    let mut _ndf = 0.;
    let mut cl = Colour {
        name: "Black",
        hex: "000000",
        rgb: &[0, 0, 0],
        hsl: &[0, 0, 0],
        shade: "Black",
    };
    let mut df = -1.;

    for colour in ALL_COLOURS.iter() {
        _ndf = redmean_distance(colour.rgb_vec(), rgb.clone());

        if df < 0. || df > _ndf {
            df = _ndf;
            cl = Colour {
                name: colour.name,
                hex: colour.hex,
                rgb: &[0, 0, 0],
                hsl: &[0, 0, 0],
                shade: colour.shade,
            }
        }
    }
    return cl;
}

#[test]
fn test_closest_colour() {
    assert_eq!("White Ice", closest_colour(vec![215, 238, 228]).name);
}

fn redmean_distance(a: Vec<i32>, b: Vec<i32>) -> f32 {
    let r_bar: f32 = (a[0] as f32 + b[0] as f32) / 2.;
    let r_coeff = 2. + (r_bar / 256.);
    let g_coeff = 4.;
    let b_coeff = 2. + ((255. - r_bar) / 256.);

    let distance = (r_coeff * ((a[0] - b[0]) as f32).powf(2.)
        + g_coeff * ((a[1] - b[1]) as f32).powf(2.)
        + b_coeff * ((a[2] - b[2]) as f32).powf(2.))
    .sqrt();
    return distance;
}
