use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Colour {
    name: String,
    hex: String,
    rgb: Vec<u32>,
    hsl: Vec<u32>
}

impl Colour {
    pub fn new(name: String, hex: String,rgb: Vec<u32>, hsl: Vec<u32>) -> Colour {
        return Colour {
            name,hex,rgb,hsl
        };
    }
}

pub fn get_colour_list () -> Vec<Colour> {
    // Open the file in read-only mode with buffer
    let path = Path::new("./data/colourMap.json");
    let file = File::open(path).expect("file should open read only");
    let reader = BufReader::new(file);

    // Read the JSON contents of the file
    let list_of_colours: Vec<Colour> = serde_json::from_reader(reader).expect("file should be proper JSON");
    return list_of_colours;
}

#[test]
fn test_get_colour_map () {
    let colour_list = get_colour_list();
    assert_eq!(colour_list.len(),1566);
}
