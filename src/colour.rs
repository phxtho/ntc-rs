use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;
use std::path::Path; 

#[derive(Debug, Deserialize, Clone)]
pub struct Colour {
    pub name: String,
    pub hex: String,
    pub rgb: Vec<i32>,
    pub hsl: Vec<i32>
}

impl Colour {
    pub fn new(name: String, hex: String,rgb: Vec<i32>, hsl: Vec<i32>) -> Colour {
        return Colour {
            name,hex,rgb,hsl
        };
    }
}

pub fn get_colour_list () -> Vec<Colour> {
    // Read the JSON contents of the file
    let list_of_colours: Vec<Colour> = serde_json::from_str(crate::colour_list::colour_list_json).expect("Couldn't parse JSON");
    return list_of_colours;
}

#[test]
fn test_get_colour_map () {
    let colour_list = get_colour_list();
    assert_eq!(colour_list.len(),1566);
}