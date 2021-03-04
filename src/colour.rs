use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Colour {
    pub name: String,
    pub hex: String,
    pub rgb: Vec<i32>,
    pub hsl: Vec<i32>,
    pub shade: String
}

impl Colour {
    pub fn new(name: String, hex: String,rgb: Vec<i32>, hsl: Vec<i32>, shade: String) -> Colour {
        return Colour {
            name,hex,rgb,hsl,shade
        };
    }
}

pub fn get_colour_list () -> Vec<Colour> {
    // Read the JSON contents of the file
    let list_of_colours: Vec<Colour> = serde_json::from_str(crate::consts::COLOURS_JSON).expect("Couldn't parse JSON");
    return list_of_colours;
}

#[test]
fn test_get_colour_list () {
    let colour_list = get_colour_list();
    assert_eq!(colour_list.len(),1639);
}