use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Colour {
    pub name: &'static str,
    pub hex: &'static str,
    pub rgb: &'static [i32],
    pub hsl: &'static [i32],
    pub shade: &'static str
}

pub const  ALL_COLOURS: &[Colour; 1639] = &[
      Colour {
        name: "Acadia",
        hex: "35312C",
        rgb: &[53, 49, 44],
        hsl: &[23, 23, 48],
        shade: "Brown"
      },
      Colour {
        name: "Acapulco",
        hex: "75AA94",
        rgb: &[117, 170, 148],
        hsl: &[109, 60, 143],
        shade: "Green"
      },
      Colour {
        name: "Aero Blue",
        hex: "C0E8D5",
        rgb: &[192, 232, 213],
        hsl: &[107, 118, 211],
        shade: "Green"
      },
      Colour {
        name: "Affair",
        hex: "745085",
        rgb: &[116, 80, 133],
        hsl: &[198, 63, 106],
        shade: "Violet"
      },
      Colour {
        name: "Afghan Tan",
        hex: "905E26",
        rgb: &[144, 94, 38],
        hsl: &[22, 148, 91],
        shade: "Yellow"
      },
      Colour {
        name: "Air Force Blue",
        hex: "5D8AA8",
        rgb: &[93, 138, 168],
        hsl: &[144, 76, 130],
        shade: "Blue"
      },
      Colour {
        name: "Akaroa",
        hex: "BEB29A",
        rgb: &[190, 178, 154],
        hsl: &[28, 55, 172],
        shade: "Yellow"
      },
      Colour {
        name: "Alabaster",
        hex: "F2F0E6",
        rgb: &[242, 240, 230],
        hsl: &[35, 80, 236],
        shade: "Grey"
      },
      Colour {
        name: "Albescent White",
        hex: "E1DACB",
        rgb: &[225, 218, 203],
        hsl: &[28, 68, 213],
        shade: "Yellow"
      },
      Colour {
        name: "Alert Tan",
        hex: "954E2C",
        rgb: &[149, 78, 44],
        hsl: &[13, 138, 96],
        shade: "Orange"
      },
      Colour {
        name: "Alice Blue",
        hex: "F0F8FF",
        rgb: &[240, 248, 255],
        hsl: &[147, 255, 247],
        shade: "Blue"
      },
      Colour {
        name: "Alizarin",
        hex: "E32636",
        rgb: &[227, 38, 54],
        hsl: &[-3, 196, 132],
        shade: "Red"
      },
      Colour {
        name: "Allports",
        hex: "1F6A7D",
        rgb: &[31, 106, 125],
        hsl: &[136, 153, 77],
        shade: "Blue"
      },
      Colour {
        name: "Almond",
        hex: "EED9C4",
        rgb: &[238, 217, 196],
        hsl: &[21, 140, 217],
        shade: "Yellow"
      },
      Colour {
        name: "Almond Frost",
        hex: "9A8678",
        rgb: &[154, 134, 120],
        hsl: &[17, 36, 137],
        shade: "Brown"
      },
      Colour {
        name: "Alpine",
        hex: "AD8A3B",
        rgb: &[173, 138, 59],
        hsl: &[29, 125, 116],
        shade: "Yellow"
      },
      Colour {
        name: "Alto",
        hex: "CDC6C5",
        rgb: &[205, 198, 197],
        hsl: &[5, 18, 201],
        shade: "Grey"
      },
      Colour {
        name: "Aluminium",
        hex: "848789",
        rgb: &[132, 135, 137],
        hsl: &[144, 5, 134],
        shade: "Grey"
      },
      Colour {
        name: "Amaranth",
        hex: "E52B50",
        rgb: &[229, 43, 80],
        hsl: &[-8, 199, 136],
        shade: "Red"
      },
      Colour {
        name: "Amazon",
        hex: "387B54",
        rgb: &[56, 123, 84],
        hsl: &[102, 95, 89],
        shade: "Green"
      },
      Colour {
        name: "Amber",
        hex: "FFBF00",
        rgb: &[255, 191, 0],
        hsl: &[31, 255, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Americano",
        hex: "8A7D72",
        rgb: &[138, 125, 114],
        hsl: &[19, 24, 126],
        shade: "Brown"
      },
      Colour {
        name: "Amethyst",
        hex: "9966CC",
        rgb: &[153, 102, 204],
        hsl: &[191, 127, 153],
        shade: "Violet"
      },
      Colour {
        name: "Amethyst Smoke",
        hex: "95879C",
        rgb: &[149, 135, 156],
        hsl: &[198, 24, 145],
        shade: "Violet"
      },
      Colour {
        name: "Amour",
        hex: "F5E6EA",
        rgb: &[245, 230, 234],
        hsl: &[-11, 109, 237],
        shade: "Violet"
      },
      Colour {
        name: "Amulet",
        hex: "7D9D72",
        rgb: &[125, 157, 114],
        hsl: &[74, 45, 135],
        shade: "Green"
      },
      Colour {
        name: "Anakiwa",
        hex: "8CCEEA",
        rgb: &[140, 206, 234],
        hsl: &[140, 176, 187],
        shade: "Blue"
      },
      Colour {
        name: "Antique Brass",
        hex: "6C461F",
        rgb: &[108, 70, 31],
        hsl: &[21, 141, 69],
        shade: "Orange"
      },
      Colour {
        name: "Antique White",
        hex: "FAEBD7",
        rgb: &[250, 235, 215],
        hsl: &[24, 198, 232],
        shade: "White"
      },
      Colour {
        name: "Anzac",
        hex: "C68E3F",
        rgb: &[198, 142, 63],
        hsl: &[24, 138, 130],
        shade: "Yellow"
      },
      Colour {
        name: "Apache",
        hex: "D3A95C",
        rgb: &[211, 169, 92],
        hsl: &[27, 146, 151],
        shade: "Yellow"
      },
      Colour {
        name: "Apple",
        hex: "66B348",
        rgb: &[102, 179, 72],
        hsl: &[73, 108, 125],
        shade: "Green"
      },
      Colour {
        name: "Apple Blossom",
        hex: "A95249",
        rgb: &[169, 82, 73],
        hsl: &[3, 101, 121],
        shade: "Red"
      },
      Colour {
        name: "Apple Green",
        hex: "DEEADC",
        rgb: &[222, 234, 220],
        hsl: &[78, 63, 227],
        shade: "Green"
      },
      Colour {
        name: "Apricot",
        hex: "FBCEB1",
        rgb: &[251, 206, 177],
        hsl: &[16, 230, 213],
        shade: "Orange"
      },
      Colour {
        name: "Apricot White",
        hex: "F7F0DB",
        rgb: &[247, 240, 219],
        hsl: &[31, 162, 233],
        shade: "Yellow"
      },
      Colour {
        name: "Aqua",
        hex: "00FFFF",
        rgb: &[0, 255, 255],
        hsl: &[127, 255, 127],
        shade: "Blue"
      },
      Colour {
        name: "Aqua Haze",
        hex: "D9DDD5",
        rgb: &[217, 221, 213],
        hsl: &[63, 26, 217],
        shade: "Grey"
      },
      Colour {
        name: "Aqua Spring",
        hex: "E8F3E8",
        rgb: &[232, 243, 232],
        hsl: &[85, 80, 237],
        shade: "Green"
      },
      Colour {
        name: "Aqua Squeeze",
        hex: "DBE4DC",
        rgb: &[219, 228, 220],
        hsl: &[89, 36, 223],
        shade: "Grey"
      },
      Colour {
        name: "Aquamarine",
        hex: "7FFFD4",
        rgb: &[127, 255, 212],
        hsl: &[113, 255, 191],
        shade: "Blue"
      },
      Colour {
        name: "Arapawa",
        hex: "274A5D",
        rgb: &[39, 74, 93],
        hsl: &[142, 104, 65],
        shade: "Blue"
      },
      Colour {
        name: "Armadillo",
        hex: "484A46",
        rgb: &[72, 74, 70],
        hsl: &[63, 7, 72],
        shade: "Grey"
      },
      Colour {
        name: "Army green",
        hex: "4B5320",
        rgb: &[75, 83, 32],
        hsl: &[49, 113, 57],
        shade: "Green"
      },
      Colour {
        name: "Arrowtown",
        hex: "827A67",
        rgb: &[130, 122, 103],
        hsl: &[29, 29, 116],
        shade: "Yellow"
      },
      Colour {
        name: "Arsenic",
        hex: "3B444B",
        rgb: &[59, 68, 75],
        hsl: &[146, 30, 67],
        shade: "Grey"
      },
      Colour {
        name: "Ash",
        hex: "BEBAA7",
        rgb: &[190, 186, 167],
        hsl: &[35, 38, 178],
        shade: "Green"
      },
      Colour {
        name: "Asparagus",
        hex: "7BA05B",
        rgb: &[123, 160, 91],
        hsl: &[65, 70, 125],
        shade: "Green"
      },
      Colour {
        name: "Astra",
        hex: "EDD5A6",
        rgb: &[237, 213, 166],
        hsl: &[28, 169, 201],
        shade: "Yellow"
      },
      Colour {
        name: "Astral",
        hex: "376F89",
        rgb: &[55, 111, 137],
        hsl: &[140, 108, 96],
        shade: "Blue"
      },
      Colour {
        name: "Astronaut",
        hex: "445172",
        rgb: &[68, 81, 114],
        hsl: &[157, 64, 91],
        shade: "Blue"
      },
      Colour {
        name: "Astronaut Blue",
        hex: "214559",
        rgb: &[33, 69, 89],
        hsl: &[142, 117, 60],
        shade: "Blue"
      },
      Colour {
        name: "Athens Grey",
        hex: "DCDDDD",
        rgb: &[220, 221, 221],
        hsl: &[127, 3, 220],
        shade: "Grey"
      },
      Colour {
        name: "Aths Special",
        hex: "D5CBB2",
        rgb: &[213, 203, 178],
        hsl: &[30, 75, 195],
        shade: "Yellow"
      },
      Colour {
        name: "Atlantis",
        hex: "9CD03B",
        rgb: &[156, 208, 59],
        hsl: &[57, 156, 133],
        shade: "Green"
      },
      Colour {
        name: "Atoll",
        hex: "2B797A",
        rgb: &[43, 121, 122],
        hsl: &[128, 122, 82],
        shade: "Green"
      },
      Colour {
        name: "Atomic",
        hex: "3D4B52",
        rgb: &[61, 75, 82],
        hsl: &[141, 37, 71],
        shade: "Blue"
      },
      Colour {
        name: "Atomic Tangerine",
        hex: "FF9966",
        rgb: &[255, 153, 102],
        hsl: &[14, 254, 178],
        shade: "Orange"
      },
      Colour {
        name: "Au Chico",
        hex: "9E6759",
        rgb: &[158, 103, 89],
        hsl: &[8, 71, 123],
        shade: "Brown"
      },
      Colour {
        name: "Aubergine",
        hex: "372528",
        rgb: &[55, 37, 40],
        hsl: &[-7, 49, 46],
        shade: "Brown"
      },
      Colour {
        name: "Auburn",
        hex: "712F2C",
        rgb: &[113, 47, 44],
        hsl: &[1, 112, 78],
        shade: "Brown"
      },
      Colour {
        name: "Australian Mint",
        hex: "EFF8AA",
        rgb: &[239, 248, 170],
        hsl: &[47, 216, 209],
        shade: "Green"
      },
      Colour {
        name: "Avocado",
        hex: "95986B",
        rgb: &[149, 152, 107],
        hsl: &[45, 45, 129],
        shade: "Green"
      },
      Colour {
        name: "Axolotl",
        hex: "63775A",
        rgb: &[99, 119, 90],
        hsl: &[71, 35, 104],
        shade: "Green"
      },
      Colour {
        name: "Azalea",
        hex: "F9C0C4",
        rgb: &[249, 192, 196],
        hsl: &[-2, 210, 220],
        shade: "Red"
      },
      Colour {
        name: "Aztec",
        hex: "293432",
        rgb: &[41, 52, 50],
        hsl: &[119, 30, 46],
        shade: "Green"
      },
      Colour {
        name: "Azure",
        hex: "F0FFFF",
        rgb: &[240, 255, 255],
        hsl: &[127, 255, 247],
        shade: "Blue"
      },
      Colour {
        name: "Baby Blue",
        hex: "6FFFFF",
        rgb: &[111, 255, 255],
        hsl: &[127, 255, 183],
        shade: "Blue"
      },
      Colour {
        name: "Bahama Blue",
        hex: "25597F",
        rgb: &[37, 89, 127],
        hsl: &[145, 139, 82],
        shade: "Blue"
      },
      Colour {
        name: "Bahia",
        hex: "A9C01C",
        rgb: &[169, 192, 28],
        hsl: &[48, 190, 110],
        shade: "Green"
      },
      Colour {
        name: "Baker's Chocolate",
        hex: "5C3317",
        rgb: &[92, 51, 23],
        hsl: &[17, 153, 57],
        shade: "Brown"
      },
      Colour {
        name: "Bali Hai",
        hex: "849CA9",
        rgb: &[132, 156, 169],
        hsl: &[142, 45, 150],
        shade: "Blue"
      },
      Colour {
        name: "Baltic Sea",
        hex: "3C3D3E",
        rgb: &[60, 61, 62],
        hsl: &[148, 4, 60],
        shade: "Grey"
      },
      Colour {
        name: "Banana Mania",
        hex: "FBE7B2",
        rgb: &[251, 231, 178],
        hsl: &[30, 229, 214],
        shade: "Yellow"
      },
      Colour {
        name: "Bandicoot",
        hex: "878466",
        rgb: &[135, 132, 102],
        hsl: &[38, 35, 118],
        shade: "Green"
      },
      Colour {
        name: "Barberry",
        hex: "D2C61F",
        rgb: &[210, 198, 31],
        hsl: &[39, 189, 120],
        shade: "Green"
      },
      Colour {
        name: "Barley Corn",
        hex: "B6935C",
        rgb: &[182, 147, 92],
        hsl: &[25, 97, 137],
        shade: "Yellow"
      },
      Colour {
        name: "Barley White",
        hex: "F7E5B7",
        rgb: &[247, 229, 183],
        hsl: &[30, 204, 215],
        shade: "Yellow"
      },
      Colour {
        name: "Barossa",
        hex: "452E39",
        rgb: &[69, 46, 57],
        hsl: &[-20, 50, 57],
        shade: "Violet"
      },
      Colour {
        name: "Bastille",
        hex: "2C2C32",
        rgb: &[44, 44, 50],
        hsl: &[170, 16, 47],
        shade: "Blue"
      },
      Colour {
        name: "Battleship Grey",
        hex: "51574F",
        rgb: &[81, 87, 79],
        hsl: &[74, 12, 83],
        shade: "Grey"
      },
      Colour {
        name: "Bay Leaf",
        hex: "7BB18D",
        rgb: &[123, 177, 141],
        hsl: &[99, 65, 150],
        shade: "Green"
      },
      Colour {
        name: "Bay Of Many",
        hex: "353E64",
        rgb: &[53, 62, 100],
        hsl: &[161, 78, 76],
        shade: "Blue"
      },
      Colour {
        name: "Bazaar",
        hex: "8F7777",
        rgb: &[143, 119, 119],
        hsl: &[0, 24, 131],
        shade: "Brown"
      },
      Colour {
        name: "Beauty Bush",
        hex: "EBB9B3",
        rgb: &[235, 185, 179],
        hsl: &[4, 148, 207],
        shade: "Red"
      },
      Colour {
        name: "Beaver",
        hex: "926F5B",
        rgb: &[146, 111, 91],
        hsl: &[15, 59, 118],
        shade: "Brown"
      },
      Colour {
        name: "Beeswax",
        hex: "E9D7AB",
        rgb: &[233, 215, 171],
        hsl: &[30, 149, 202],
        shade: "Yellow"
      },
      Colour {
        name: "Beige",
        hex: "F5F5DC",
        rgb: &[245, 245, 220],
        hsl: &[42, 141, 232],
        shade: "Brown"
      },
      Colour {
        name: "Bermuda",
        hex: "86D2C1",
        rgb: &[134, 210, 193],
        hsl: &[117, 116, 172],
        shade: "Green"
      },
      Colour {
        name: "Bermuda Grey",
        hex: "6F8C9F",
        rgb: &[111, 140, 159],
        hsl: &[144, 51, 135],
        shade: "Blue"
      },
      Colour {
        name: "Beryl Green",
        hex: "BCBFA8",
        rgb: &[188, 191, 168],
        hsl: &[48, 38, 179],
        shade: "Green"
      },
      Colour {
        name: "Bianca",
        hex: "F4EFE0",
        rgb: &[244, 239, 224],
        hsl: &[31, 121, 234],
        shade: "Yellow"
      },
      Colour {
        name: "Big Stone",
        hex: "334046",
        rgb: &[51, 64, 70],
        hsl: &[140, 40, 60],
        shade: "Blue"
      },
      Colour {
        name: "Bilbao",
        hex: "3E8027",
        rgb: &[62, 128, 39],
        hsl: &[74, 135, 83],
        shade: "Green"
      },
      Colour {
        name: "Biloba Flower",
        hex: "AE99D2",
        rgb: &[174, 153, 210],
        hsl: &[185, 98, 181],
        shade: "Violet"
      },
      Colour {
        name: "Birch",
        hex: "3F3726",
        rgb: &[63, 55, 38],
        hsl: &[28, 63, 50],
        shade: "Yellow"
      },
      Colour {
        name: "Bird Flower",
        hex: "D0C117",
        rgb: &[208, 193, 23],
        hsl: &[39, 204, 115],
        shade: "Green"
      },
      Colour {
        name: "Biscay",
        hex: "2F3C53",
        rgb: &[47, 60, 83],
        hsl: &[154, 70, 65],
        shade: "Blue"
      },
      Colour {
        name: "Bismark",
        hex: "486C7A",
        rgb: &[72, 108, 122],
        hsl: &[139, 65, 97],
        shade: "Blue"
      },
      Colour {
        name: "Bison Hide",
        hex: "B5AC94",
        rgb: &[181, 172, 148],
        hsl: &[30, 46, 164],
        shade: "Yellow"
      },
      Colour {
        name: "Bisque",
        hex: "FFE4C4",
        rgb: &[255, 228, 196],
        hsl: &[23, 254, 225],
        shade: "Brown"
      },
      Colour {
        name: "Bistre",
        hex: "3D2B1F",
        rgb: &[61, 43, 31],
        hsl: &[17, 83, 46],
        shade: "Brown"
      },
      Colour {
        name: "Bitter",
        hex: "88896C",
        rgb: &[136, 137, 108],
        hsl: &[43, 30, 122],
        shade: "Green"
      },
      Colour {
        name: "Bitter Lemon",
        hex: "D2DB32",
        rgb: &[210, 219, 50],
        hsl: &[44, 178, 134],
        shade: "Green"
      },
      Colour {
        name: "Bittersweet",
        hex: "FE6F5E",
        rgb: &[254, 111, 94],
        hsl: &[4, 251, 174],
        shade: "Orange"
      },
      Colour {
        name: "Bizarre",
        hex: "E7D2C8",
        rgb: &[231, 210, 200],
        hsl: &[13, 100, 215],
        shade: "Orange"
      },
      Colour {
        name: "Black",
        hex: "000000",
        rgb: &[0, 0, 0],
        hsl: &[0, 0, 0],
        shade: "Black"
      },
      Colour {
        name: "Black Bean",
        hex: "232E26",
        rgb: &[35, 46, 38],
        hsl: &[96, 34, 40],
        shade: "Green"
      },
      Colour {
        name: "Black Forest",
        hex: "2C3227",
        rgb: &[44, 50, 39],
        hsl: &[65, 31, 44],
        shade: "Green"
      },
      Colour {
        name: "Black Haze",
        hex: "E0DED7",
        rgb: &[224, 222, 215],
        hsl: &[33, 32, 219],
        shade: "Grey"
      },
      Colour {
        name: "Black Magic",
        hex: "332C22",
        rgb: &[51, 44, 34],
        hsl: &[24, 51, 42],
        shade: "Brown"
      },
      Colour {
        name: "Black Marlin",
        hex: "383740",
        rgb: &[56, 55, 64],
        hsl: &[174, 19, 59],
        shade: "Blue"
      },
      Colour {
        name: "Black Pearl",
        hex: "1E272C",
        rgb: &[30, 39, 44],
        hsl: &[142, 48, 36],
        shade: "Blue"
      },
      Colour {
        name: "Black Rock",
        hex: "2C2D3C",
        rgb: &[44, 45, 60],
        hsl: &[167, 39, 52],
        shade: "Blue"
      },
      Colour {
        name: "Black Rose",
        hex: "532934",
        rgb: &[83, 41, 52],
        hsl: &[-11, 86, 62],
        shade: "Red"
      },
      Colour {
        name: "Black Russian",
        hex: "24252B",
        rgb: &[36, 37, 43],
        hsl: &[163, 22, 39],
        shade: "Grey"
      },
      Colour {
        name: "Black Squeeze",
        hex: "E5E6DF",
        rgb: &[229, 230, 223],
        hsl: &[48, 31, 226],
        shade: "Grey"
      },
      Colour {
        name: "Black White",
        hex: "E5E4DB",
        rgb: &[229, 228, 219],
        hsl: &[38, 41, 224],
        shade: "Grey"
      },
      Colour {
        name: "Blackberry",
        hex: "43182F",
        rgb: &[67, 24, 47],
        hsl: &[-22, 120, 45],
        shade: "Violet"
      },
      Colour {
        name: "Blackcurrant",
        hex: "2E183B",
        rgb: &[46, 24, 59],
        hsl: &[196, 107, 41],
        shade: "Violet"
      },
      Colour {
        name: "Blanc",
        hex: "D9D0C1",
        rgb: &[217, 208, 193],
        hsl: &[26, 61, 205],
        shade: "Yellow"
      },
      Colour {
        name: "Blanched Almond",
        hex: "FFEBCD",
        rgb: &[255, 235, 205],
        hsl: &[25, 255, 230],
        shade: "Brown"
      },
      Colour {
        name: "Bleach White",
        hex: "EBE1CE",
        rgb: &[235, 225, 206],
        hsl: &[27, 107, 220],
        shade: "Yellow"
      },
      Colour {
        name: "Blizzard Blue",
        hex: "A3E3ED",
        rgb: &[163, 227, 237],
        hsl: &[133, 171, 200],
        shade: "Blue"
      },
      Colour {
        name: "Blossom",
        hex: "DFB1B6",
        rgb: &[223, 177, 182],
        hsl: &[-4, 106, 200],
        shade: "Red"
      },
      Colour {
        name: "Blue",
        hex: "0000FF",
        rgb: &[0, 0, 255],
        hsl: &[170, 255, 127],
        shade: "Blue"
      },
      Colour {
        name: "Blue Bayoux",
        hex: "62777E",
        rgb: &[98, 119, 126],
        hsl: &[138, 31, 112],
        shade: "Blue"
      },
      Colour {
        name: "Blue Bell",
        hex: "9999CC",
        rgb: &[153, 153, 204],
        hsl: &[170, 85, 178],
        shade: "Blue"
      },
      Colour {
        name: "Blue Chalk",
        hex: "E3D6E9",
        rgb: &[227, 214, 233],
        hsl: &[199, 76, 223],
        shade: "Violet"
      },
      Colour {
        name: "Blue Charcoal",
        hex: "262B2F",
        rgb: &[38, 43, 47],
        hsl: &[146, 27, 42],
        shade: "Blue"
      },
      Colour {
        name: "Blue Chill",
        hex: "408F90",
        rgb: &[64, 143, 144],
        hsl: &[128, 98, 104],
        shade: "Green"
      },
      Colour {
        name: "Blue Diamond",
        hex: "4B2D72",
        rgb: &[75, 45, 114],
        hsl: &[188, 110, 79],
        shade: "Violet"
      },
      Colour {
        name: "Blue Dianne",
        hex: "35514F",
        rgb: &[53, 81, 79],
        hsl: &[124, 53, 67],
        shade: "Green"
      },
      Colour {
        name: "Blue Gem",
        hex: "4B3C8E",
        rgb: &[75, 60, 142],
        hsl: &[177, 103, 101],
        shade: "Violet"
      },
      Colour {
        name: "Blue Haze",
        hex: "BDBACE",
        rgb: &[189, 186, 206],
        hsl: &[176, 43, 195],
        shade: "Violet"
      },
      Colour {
        name: "Blue Lagoon",
        hex: "00626F",
        rgb: &[0, 98, 111],
        hsl: &[132, 255, 55],
        shade: "Green"
      },
      Colour {
        name: "Blue Marguerite",
        hex: "6A5BB1",
        rgb: &[106, 91, 177],
        hsl: &[177, 90, 134],
        shade: "Violet"
      },
      Colour {
        name: "Blue Romance",
        hex: "D8F0D2",
        rgb: &[216, 240, 210],
        hsl: &[76, 127, 225],
        shade: "Green"
      },
      Colour {
        name: "Blue Smoke",
        hex: "78857A",
        rgb: &[120, 133, 122],
        hsl: &[91, 13, 126],
        shade: "Green"
      },
      Colour {
        name: "Blue Stone",
        hex: "166461",
        rgb: &[22, 100, 97],
        hsl: &[125, 163, 60],
        shade: "Green"
      },
      Colour {
        name: "Blue Violet",
        hex: "8A2BE2",
        rgb: &[138, 43, 226],
        hsl: &[192, 193, 134],
        shade: "Violet"
      },
      Colour {
        name: "Blue Whale",
        hex: "1E3442",
        rgb: &[30, 52, 66],
        hsl: &[144, 95, 48],
        shade: "Blue"
      },
      Colour {
        name: "Blue Zodiac",
        hex: "3C4354",
        rgb: &[60, 67, 84],
        hsl: &[157, 42, 72],
        shade: "Blue"
      },
      Colour {
        name: "Blumine",
        hex: "305C71",
        rgb: &[48, 92, 113],
        hsl: &[141, 102, 80],
        shade: "Blue"
      },
      Colour {
        name: "Blush",
        hex: "B55067",
        rgb: &[181, 80, 103],
        hsl: &[-9, 103, 130],
        shade: "Red"
      },
      Colour {
        name: "Bokara Grey",
        hex: "2A2725",
        rgb: &[42, 39, 37],
        hsl: &[17, 16, 39],
        shade: "Grey"
      },
      Colour {
        name: "Bole",
        hex: "79443B",
        rgb: &[121, 68, 59],
        hsl: &[6, 87, 89],
        shade: "Brown"
      },
      Colour {
        name: "Bombay",
        hex: "AEAEAD",
        rgb: &[174, 174, 173],
        hsl: &[42, 1, 173],
        shade: "Grey"
      },
      Colour {
        name: "Bon Jour",
        hex: "DFD7D2",
        rgb: &[223, 215, 210],
        hsl: &[16, 43, 216],
        shade: "Grey"
      },
      Colour {
        name: "Bondi Blue",
        hex: "0095B6",
        rgb: &[0, 149, 182],
        hsl: &[135, 255, 91],
        shade: "Blue"
      },
      Colour {
        name: "Bone",
        hex: "DBC2AB",
        rgb: &[219, 194, 171],
        hsl: &[20, 101, 195],
        shade: "Orange"
      },
      Colour {
        name: "Bordeaux",
        hex: "4C1C24",
        rgb: &[76, 28, 36],
        hsl: &[-7, 117, 52],
        shade: "Red"
      },
      Colour {
        name: "Bossanova",
        hex: "4C3D4E",
        rgb: &[76, 61, 78],
        hsl: &[207, 31, 69],
        shade: "Violet"
      },
      Colour {
        name: "Boston Blue",
        hex: "438EAC",
        rgb: &[67, 142, 172],
        hsl: &[139, 112, 119],
        shade: "Blue"
      },
      Colour {
        name: "Botticelli",
        hex: "92ACB4",
        rgb: &[146, 172, 180],
        hsl: &[137, 47, 163],
        shade: "Blue"
      },
      Colour {
        name: "Bottle Green",
        hex: "254636",
        rgb: &[37, 70, 54],
        hsl: &[106, 78, 53],
        shade: "Green"
      },
      Colour {
        name: "Boulder",
        hex: "7C817C",
        rgb: &[124, 129, 124],
        hsl: &[85, 5, 126],
        shade: "Grey"
      },
      Colour {
        name: "Bouquet",
        hex: "A78199",
        rgb: &[167, 129, 153],
        hsl: &[-26, 45, 147],
        shade: "Violet"
      },
      Colour {
        name: "Bourbon",
        hex: "AF6C3E",
        rgb: &[175, 108, 62],
        hsl: &[17, 121, 118],
        shade: "Orange"
      },
      Colour {
        name: "Bracken",
        hex: "5B3D27",
        rgb: &[91, 61, 39],
        hsl: &[17, 102, 65],
        shade: "Brown"
      },
      Colour {
        name: "Brandy",
        hex: "DCB68A",
        rgb: &[220, 182, 138],
        hsl: &[22, 137, 179],
        shade: "Orange"
      },
      Colour {
        name: "Brandy Punch",
        hex: "C07C40",
        rgb: &[192, 124, 64],
        hsl: &[19, 128, 128],
        shade: "Orange"
      },
      Colour {
        name: "Brandy Rose",
        hex: "B6857A",
        rgb: &[182, 133, 122],
        hsl: &[7, 74, 152],
        shade: "Red"
      },
      Colour {
        name: "Brass",
        hex: "B5A642",
        rgb: &[181, 166, 66],
        hsl: &[36, 118, 123],
        shade: "Yellow"
      },
      Colour {
        name: "Breaker Bay",
        hex: "517B78",
        rgb: &[81, 123, 120],
        hsl: &[124, 52, 102],
        shade: "Green"
      },
      Colour {
        name: "Brick Red",
        hex: "C62D42",
        rgb: &[198, 45, 66],
        hsl: &[-5, 160, 121],
        shade: "Red"
      },
      Colour {
        name: "Bridal Heath",
        hex: "F8EBDD",
        rgb: &[248, 235, 221],
        hsl: &[22, 167, 234],
        shade: "Orange"
      },
      Colour {
        name: "Bridesmaid",
        hex: "FAE6DF",
        rgb: &[250, 230, 223],
        hsl: &[11, 186, 236],
        shade: "Orange"
      },
      Colour {
        name: "Bright Green",
        hex: "66FF00",
        rgb: &[102, 255, 0],
        hsl: &[68, 255, 127],
        shade: "Green"
      },
      Colour {
        name: "Bright Grey",
        hex: "57595D",
        rgb: &[87, 89, 93],
        hsl: &[155, 8, 89],
        shade: "Grey"
      },
      Colour {
        name: "Bright Red",
        hex: "922A31",
        rgb: &[146, 42, 49],
        hsl: &[-2, 141, 93],
        shade: "Red"
      },
      Colour {
        name: "Bright Sun",
        hex: "ECBD2C",
        rgb: &[236, 189, 44],
        hsl: &[32, 212, 140],
        shade: "Yellow"
      },
      Colour {
        name: "Bright Turquoise",
        hex: "08E8DE",
        rgb: &[8, 232, 222],
        hsl: &[125, 238, 120],
        shade: "Blue"
      },
      Colour {
        name: "Brilliant Rose",
        hex: "FF55A3",
        rgb: &[255, 85, 163],
        hsl: &[-19, 255, 170],
        shade: "Red"
      },
      Colour {
        name: "Brink Pink",
        hex: "FB607F",
        rgb: &[251, 96, 127],
        hsl: &[-8, 242, 173],
        shade: "Red"
      },
      Colour {
        name: "British Racing Green",
        hex: "004225",
        rgb: &[0, 66, 37],
        hsl: &[108, 255, 33],
        shade: "Green"
      },
      Colour {
        name: "Bronco",
        hex: "A79781",
        rgb: &[167, 151, 129],
        hsl: &[24, 45, 147],
        shade: "Brown"
      },
      Colour {
        name: "Bronze",
        hex: "CD7F32",
        rgb: &[205, 127, 50],
        hsl: &[21, 155, 127],
        shade: "Brown"
      },
      Colour {
        name: "Bronze Olive",
        hex: "584C25",
        rgb: &[88, 76, 37],
        hsl: &[32, 104, 62],
        shade: "Yellow"
      },
      Colour {
        name: "Bronzetone",
        hex: "434C28",
        rgb: &[67, 76, 40],
        hsl: &[53, 79, 58],
        shade: "Yellow"
      },
      Colour {
        name: "Broom",
        hex: "EECC24",
        rgb: &[238, 204, 36],
        hsl: &[35, 218, 137],
        shade: "Yellow"
      },
      Colour {
        name: "Brown",
        hex: "A52A2A",
        rgb: &[165, 42, 42],
        hsl: &[0, 151, 103],
        shade: "Brown"
      },
      Colour {
        name: "Brown Bramble",
        hex: "53331E",
        rgb: &[83, 51, 30],
        hsl: &[16, 119, 56],
        shade: "Brown"
      },
      Colour {
        name: "Brown Derby",
        hex: "594537",
        rgb: &[89, 69, 55],
        hsl: &[17, 60, 72],
        shade: "Brown"
      },
      Colour {
        name: "Brown Pod",
        hex: "3C241B",
        rgb: &[60, 36, 27],
        hsl: &[11, 96, 43],
        shade: "Brown"
      },
      Colour {
        name: "Bubbles",
        hex: "E6F2EA",
        rgb: &[230, 242, 234],
        hsl: &[99, 80, 236],
        shade: "Green"
      },
      Colour {
        name: "Buccaneer",
        hex: "6E5150",
        rgb: &[110, 81, 80],
        hsl: &[1, 40, 95],
        shade: "Red"
      },
      Colour {
        name: "Bud",
        hex: "A5A88F",
        rgb: &[165, 168, 143],
        hsl: &[47, 32, 155],
        shade: "Green"
      },
      Colour {
        name: "Buddha Gold",
        hex: "BC9B1B",
        rgb: &[188, 155, 27],
        hsl: &[33, 190, 107],
        shade: "Yellow"
      },
      Colour {
        name: "Buff",
        hex: "F0DC82",
        rgb: &[240, 220, 130],
        hsl: &[34, 200, 185],
        shade: "Yellow"
      },
      Colour {
        name: "Bulgarian Rose",
        hex: "482427",
        rgb: &[72, 36, 39],
        hsl: &[-3, 85, 54],
        shade: "Red"
      },
      Colour {
        name: "Bull Shot",
        hex: "75442B",
        rgb: &[117, 68, 43],
        hsl: &[14, 117, 80],
        shade: "Orange"
      },
      Colour {
        name: "Bunker",
        hex: "292C2F",
        rgb: &[41, 44, 47],
        hsl: &[148, 17, 44],
        shade: "Grey"
      },
      Colour {
        name: "Bunting",
        hex: "2B3449",
        rgb: &[43, 52, 73],
        hsl: &[157, 65, 58],
        shade: "Blue"
      },
      Colour {
        name: "Burgundy",
        hex: "800020",
        rgb: &[128, 0, 32],
        hsl: &[-10, 255, 64],
        shade: "Red"
      },
      Colour {
        name: "Burly Wood",
        hex: "DEB887",
        rgb: &[222, 184, 135],
        hsl: &[23, 145, 178],
        shade: "Brown"
      },
      Colour {
        name: "Burnham",
        hex: "234537",
        rgb: &[35, 69, 55],
        hsl: &[110, 83, 52],
        shade: "Green"
      },
      Colour {
        name: "Burning Sand",
        hex: "D08363",
        rgb: &[208, 131, 99],
        hsl: &[12, 136, 153],
        shade: "Orange"
      },
      Colour {
        name: "Burnt Crimson",
        hex: "582124",
        rgb: &[88, 33, 36],
        hsl: &[-2, 115, 60],
        shade: "Red"
      },
      Colour {
        name: "Burnt Orange",
        hex: "FF7034",
        rgb: &[255, 112, 52],
        hsl: &[12, 255, 153],
        shade: "Orange"
      },
      Colour {
        name: "Burnt Sienna",
        hex: "E97451",
        rgb: &[233, 116, 81],
        hsl: &[9, 197, 157],
        shade: "Brown"
      },
      Colour {
        name: "Burnt Umber",
        hex: "8A3324",
        rgb: &[138, 51, 36],
        hsl: &[6, 149, 87],
        shade: "Brown"
      },
      Colour {
        name: "Buttercup",
        hex: "DA9429",
        rgb: &[218, 148, 41],
        hsl: &[25, 179, 129],
        shade: "Yellow"
      },
      Colour {
        name: "Buttered Rum",
        hex: "9D702E",
        rgb: &[157, 112, 46],
        hsl: &[25, 139, 101],
        shade: "Yellow"
      },
      Colour {
        name: "Butterfly Bush",
        hex: "68578C",
        rgb: &[104, 87, 140],
        hsl: &[183, 59, 113],
        shade: "Violet"
      },
      Colour {
        name: "Buttermilk",
        hex: "F6E0A4",
        rgb: &[246, 224, 164],
        hsl: &[31, 209, 205],
        shade: "Yellow"
      },
      Colour {
        name: "Buttery White",
        hex: "F1EBDA",
        rgb: &[241, 235, 218],
        hsl: &[31, 114, 229],
        shade: "Yellow"
      },
      Colour {
        name: "Cab Sav",
        hex: "4A2E32",
        rgb: &[74, 46, 50],
        hsl: &[-6, 59, 60],
        shade: "Red"
      },
      Colour {
        name: "Cabaret",
        hex: "CD526C",
        rgb: &[205, 82, 108],
        hsl: &[-8, 140, 143],
        shade: "Red"
      },
      Colour {
        name: "Cabbage Pont",
        hex: "4C5544",
        rgb: &[76, 85, 68],
        hsl: &[65, 28, 76],
        shade: "Green"
      },
      Colour {
        name: "Cactus",
        hex: "5B6F55",
        rgb: &[91, 111, 85],
        hsl: &[75, 33, 97],
        shade: "Green"
      },
      Colour {
        name: "Cadet Blue",
        hex: "5F9EA0",
        rgb: &[95, 158, 160],
        hsl: &[128, 65, 127],
        shade: "Blue"
      },
      Colour {
        name: "Cadillac",
        hex: "984961",
        rgb: &[152, 73, 97],
        hsl: &[-12, 89, 112],
        shade: "Red"
      },
      Colour {
        name: "Cafe Royale",
        hex: "6A4928",
        rgb: &[106, 73, 40],
        hsl: &[21, 115, 73],
        shade: "Brown"
      },
      Colour {
        name: "Calico",
        hex: "D5B185",
        rgb: &[213, 177, 133],
        hsl: &[23, 124, 173],
        shade: "Brown"
      },
      Colour {
        name: "California",
        hex: "E98C3A",
        rgb: &[233, 140, 58],
        hsl: &[19, 203, 145],
        shade: "Orange"
      },
      Colour {
        name: "Calypso",
        hex: "3D7188",
        rgb: &[61, 113, 136],
        hsl: &[140, 97, 98],
        shade: "Blue"
      },
      Colour {
        name: "Camarone",
        hex: "206937",
        rgb: &[32, 105, 55],
        hsl: &[98, 135, 68],
        shade: "Green"
      },
      Colour {
        name: "Camelot",
        hex: "803A4B",
        rgb: &[128, 58, 75],
        hsl: &[-10, 95, 93],
        shade: "Red"
      },
      Colour {
        name: "Cameo",
        hex: "CCA483",
        rgb: &[204, 164, 131],
        hsl: &[19, 106, 167],
        shade: "Brown"
      },
      Colour {
        name: "Camouflage",
        hex: "4F4D32",
        rgb: &[79, 77, 50],
        hsl: &[39, 57, 64],
        shade: "Yellow"
      },
      Colour {
        name: "Camouflage Green",
        hex: "78866B",
        rgb: &[120, 134, 107],
        hsl: &[64, 28, 120],
        shade: "Green"
      },
      Colour {
        name: "Can Can",
        hex: "D08A9B",
        rgb: &[208, 138, 155],
        hsl: &[-10, 108, 173],
        shade: "Red"
      },
      Colour {
        name: "Canary",
        hex: "FFFF99",
        rgb: &[255, 255, 153],
        hsl: &[42, 255, 204],
        shade: "Yellow"
      },
      Colour {
        name: "Cannon Pink",
        hex: "8E5164",
        rgb: &[142, 81, 100],
        hsl: &[-13, 69, 111],
        shade: "Red"
      },
      Colour {
        name: "Cape Cod",
        hex: "4E5552",
        rgb: &[78, 85, 82],
        hsl: &[109, 10, 81],
        shade: "Grey"
      },
      Colour {
        name: "Cape Honey",
        hex: "FEE0A5",
        rgb: &[254, 224, 165],
        hsl: &[28, 249, 209],
        shade: "Yellow"
      },
      Colour {
        name: "Cape Palliser",
        hex: "75482F",
        rgb: &[117, 72, 47],
        hsl: &[15, 108, 81],
        shade: "Orange"
      },
      Colour {
        name: "Caper",
        hex: "AFC182",
        rgb: &[175, 193, 130],
        hsl: &[54, 85, 161],
        shade: "Green"
      },
      Colour {
        name: "Caput Mortuum",
        hex: "592720",
        rgb: &[89, 39, 32],
        hsl: &[5, 120, 60],
        shade: "Brown"
      },
      Colour {
        name: "Caramel",
        hex: "FFD59A",
        rgb: &[255, 213, 154],
        hsl: &[24, 255, 204],
        shade: "Yellow"
      },
      Colour {
        name: "Cararra",
        hex: "EBE5D5",
        rgb: &[235, 229, 213],
        hsl: &[30, 90, 224],
        shade: "Green"
      },
      Colour {
        name: "Cardin Green",
        hex: "1B3427",
        rgb: &[27, 52, 39],
        hsl: &[105, 80, 39],
        shade: "Green"
      },
      Colour {
        name: "Cardinal",
        hex: "C41E3A",
        rgb: &[196, 30, 58],
        hsl: &[-7, 187, 113],
        shade: "Red"
      },
      Colour {
        name: "Careys Pink",
        hex: "C99AA0",
        rgb: &[201, 154, 160],
        hsl: &[-5, 77, 177],
        shade: "Red"
      },
      Colour {
        name: "Caribbean Green",
        hex: "00CC99",
        rgb: &[0, 204, 153],
        hsl: &[116, 255, 102],
        shade: "Green"
      },
      Colour {
        name: "Carissma",
        hex: "E68095",
        rgb: &[230, 128, 149],
        hsl: &[-8, 171, 179],
        shade: "Red"
      },
      Colour {
        name: "Carla",
        hex: "F5F9CB",
        rgb: &[245, 249, 203],
        hsl: &[46, 202, 226],
        shade: "Green"
      },
      Colour {
        name: "Carmine",
        hex: "960018",
        rgb: &[150, 0, 24],
        hsl: &[-6, 255, 75],
        shade: "Red"
      },
      Colour {
        name: "Carnaby Tan",
        hex: "5B3A24",
        rgb: &[91, 58, 36],
        hsl: &[17, 110, 63],
        shade: "Brown"
      },
      Colour {
        name: "Carnation Pink",
        hex: "FFA6C9",
        rgb: &[255, 166, 201],
        hsl: &[-16, 254, 210],
        shade: "Red"
      },
      Colour {
        name: "Carousel Pink",
        hex: "F8DBE0",
        rgb: &[248, 219, 224],
        hsl: &[-7, 171, 233],
        shade: "Red"
      },
      Colour {
        name: "Carrot Orange",
        hex: "ED9121",
        rgb: &[237, 145, 33],
        hsl: &[23, 216, 135],
        shade: "Orange"
      },
      Colour {
        name: "Casablanca",
        hex: "F0B253",
        rgb: &[240, 178, 83],
        hsl: &[25, 214, 161],
        shade: "Yellow"
      },
      Colour {
        name: "Casal",
        hex: "3F545A",
        rgb: &[63, 84, 90],
        hsl: &[136, 44, 76],
        shade: "Blue"
      },
      Colour {
        name: "Cascade",
        hex: "8CA8A0",
        rgb: &[140, 168, 160],
        hsl: &[115, 35, 154],
        shade: "Green"
      },
      Colour {
        name: "Cashmere",
        hex: "D1B399",
        rgb: &[209, 179, 153],
        hsl: &[19, 96, 180],
        shade: "Brown"
      },
      Colour {
        name: "Casper",
        hex: "AAB5B8",
        rgb: &[170, 181, 184],
        hsl: &[136, 22, 177],
        shade: "Blue"
      },
      Colour {
        name: "Castro",
        hex: "44232F",
        rgb: &[68, 35, 47],
        hsl: &[-15, 81, 51],
        shade: "Red"
      },
      Colour {
        name: "Catalina Blue",
        hex: "273C5A",
        rgb: &[39, 60, 90],
        hsl: &[152, 100, 64],
        shade: "Blue"
      },
      Colour {
        name: "Catskill White",
        hex: "E0E4DC",
        rgb: &[224, 228, 220],
        hsl: &[63, 32, 224],
        shade: "Grey"
      },
      Colour {
        name: "Cavern Pink",
        hex: "E0B8B1",
        rgb: &[224, 184, 177],
        hsl: &[6, 109, 200],
        shade: "Red"
      },
      Colour {
        name: "Ce Soir",
        hex: "9271A7",
        rgb: &[146, 113, 167],
        hsl: &[195, 59, 140],
        shade: "Violet"
      },
      Colour {
        name: "Cedar",
        hex: "463430",
        rgb: &[70, 52, 48],
        hsl: &[7, 47, 59],
        shade: "Brown"
      },
      Colour {
        name: "Celadon",
        hex: "ACE1AF",
        rgb: &[172, 225, 175],
        hsl: &[87, 119, 198],
        shade: "Green"
      },
      Colour {
        name: "Celery",
        hex: "B4C04C",
        rgb: &[180, 192, 76],
        hsl: &[46, 122, 134],
        shade: "Green"
      },
      Colour {
        name: "Celeste",
        hex: "D2D2C0",
        rgb: &[210, 210, 192],
        hsl: &[42, 42, 201],
        shade: "Green"
      },
      Colour {
        name: "Cello",
        hex: "3A4E5F",
        rgb: &[58, 78, 95],
        hsl: &[147, 61, 76],
        shade: "Blue"
      },
      Colour {
        name: "Celtic",
        hex: "2B3F36",
        rgb: &[43, 63, 54],
        hsl: &[108, 48, 53],
        shade: "Green"
      },
      Colour {
        name: "Cement",
        hex: "857158",
        rgb: &[133, 113, 88],
        hsl: &[23, 51, 110],
        shade: "Brown"
      },
      Colour {
        name: "Cerise",
        hex: "DE3163",
        rgb: &[222, 49, 99],
        hsl: &[-12, 184, 135],
        shade: "Violet"
      },
      Colour {
        name: "Cerulean",
        hex: "007BA7",
        rgb: &[0, 123, 167],
        hsl: &[138, 255, 83],
        shade: "Blue"
      },
      Colour {
        name: "Cerulean Blue",
        hex: "2A52BE",
        rgb: &[42, 82, 190],
        hsl: &[158, 162, 116],
        shade: "Blue"
      },
      Colour {
        name: "Chablis",
        hex: "FDE9E0",
        rgb: &[253, 233, 224],
        hsl: &[13, 224, 238],
        shade: "Red"
      },
      Colour {
        name: "Chalet Green",
        hex: "5A6E41",
        rgb: &[90, 110, 65],
        hsl: &[61, 65, 87],
        shade: "Green"
      },
      Colour {
        name: "Chalky",
        hex: "DFC281",
        rgb: &[223, 194, 129],
        hsl: &[29, 151, 176],
        shade: "Yellow"
      },
      Colour {
        name: "Chambray",
        hex: "475877",
        rgb: &[71, 88, 119],
        hsl: &[154, 64, 95],
        shade: "Blue"
      },
      Colour {
        name: "Chamois",
        hex: "E8CD9A",
        rgb: &[232, 205, 154],
        hsl: &[27, 160, 193],
        shade: "Yellow"
      },
      Colour {
        name: "Champagne",
        hex: "EED9B6",
        rgb: &[238, 217, 182],
        hsl: &[26, 158, 210],
        shade: "Yellow"
      },
      Colour {
        name: "Chantilly",
        hex: "EDB8C7",
        rgb: &[237, 184, 199],
        hsl: &[-12, 151, 210],
        shade: "Red"
      },
      Colour {
        name: "Charade",
        hex: "394043",
        rgb: &[57, 64, 67],
        hsl: &[140, 20, 62],
        shade: "Blue"
      },
      Colour {
        name: "Charcoal",
        hex: "464646",
        rgb: &[70, 70, 70],
        hsl: &[0, 0, 70],
        shade: "Grey"
      },
      Colour {
        name: "Chardon",
        hex: "F8EADF",
        rgb: &[248, 234, 223],
        hsl: &[18, 163, 235],
        shade: "Orange"
      },
      Colour {
        name: "Chardonnay",
        hex: "FFC878",
        rgb: &[255, 200, 120],
        hsl: &[25, 255, 187],
        shade: "Yellow"
      },
      Colour {
        name: "Charlotte",
        hex: "A4DCE6",
        rgb: &[164, 220, 230],
        hsl: &[133, 145, 197],
        shade: "Blue"
      },
      Colour {
        name: "Charm",
        hex: "D0748B",
        rgb: &[208, 116, 139],
        hsl: &[-10, 126, 162],
        shade: "Red"
      },
      Colour {
        name: "Chartreuse",
        hex: "7FFF00",
        rgb: &[127, 255, 0],
        hsl: &[63, 255, 127],
        shade: "Green"
      },
      Colour {
        name: "Chartreuse Yellow",
        hex: "DFFF00",
        rgb: &[223, 255, 0],
        hsl: &[47, 255, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Chateau Green",
        hex: "419F59",
        rgb: &[65, 159, 89],
        hsl: &[95, 107, 112],
        shade: "Green"
      },
      Colour {
        name: "Chatelle",
        hex: "B3ABB6",
        rgb: &[179, 171, 182],
        hsl: &[200, 17, 176],
        shade: "Violet"
      },
      Colour {
        name: "Chathams Blue",
        hex: "2C5971",
        rgb: &[44, 89, 113],
        hsl: &[142, 112, 78],
        shade: "Blue"
      },
      Colour {
        name: "Chelsea Cucumber",
        hex: "88A95B",
        rgb: &[136, 169, 91],
        hsl: &[60, 79, 130],
        shade: "Green"
      },
      Colour {
        name: "Chelsea Gem",
        hex: "95532F",
        rgb: &[149, 83, 47],
        hsl: &[14, 132, 98],
        shade: "Orange"
      },
      Colour {
        name: "Chenin",
        hex: "DEC371",
        rgb: &[222, 195, 113],
        hsl: &[31, 158, 167],
        shade: "Yellow"
      },
      Colour {
        name: "Cherokee",
        hex: "F5CD82",
        rgb: &[245, 205, 130],
        hsl: &[27, 217, 187],
        shade: "Yellow"
      },
      Colour {
        name: "Cherry Pie",
        hex: "372D52",
        rgb: &[55, 45, 82],
        hsl: &[181, 74, 63],
        shade: "Violet"
      },
      Colour {
        name: "Cherub",
        hex: "F5D7DC",
        rgb: &[245, 215, 220],
        hsl: &[-7, 153, 230],
        shade: "Red"
      },
      Colour {
        name: "Chestnut",
        hex: "B94E48",
        rgb: &[185, 78, 72],
        hsl: &[2, 113, 128],
        shade: "Brown"
      },
      Colour {
        name: "Chetwode Blue",
        hex: "666FB4",
        rgb: &[102, 111, 180],
        hsl: &[165, 87, 141],
        shade: "Blue"
      },
      Colour {
        name: "Chicago",
        hex: "5B5D56",
        rgb: &[91, 93, 86],
        hsl: &[54, 9, 89],
        shade: "Grey"
      },
      Colour {
        name: "Chiffon",
        hex: "F0F5BB",
        rgb: &[240, 245, 187],
        hsl: &[46, 189, 216],
        shade: "Green"
      },
      Colour {
        name: "Chilean Fire",
        hex: "D05E34",
        rgb: &[208, 94, 52],
        hsl: &[11, 159, 130],
        shade: "Orange"
      },
      Colour {
        name: "Chilean Heath",
        hex: "F9F7DE",
        rgb: &[249, 247, 222],
        hsl: &[39, 176, 235],
        shade: "Green"
      },
      Colour {
        name: "China Ivory",
        hex: "FBF3D3",
        rgb: &[251, 243, 211],
        hsl: &[34, 212, 230],
        shade: "Green"
      },
      Colour {
        name: "Chino",
        hex: "B8AD8A",
        rgb: &[184, 173, 138],
        hsl: &[32, 62, 161],
        shade: "Yellow"
      },
      Colour {
        name: "Chinook",
        hex: "9DD3A8",
        rgb: &[157, 211, 168],
        hsl: &[93, 96, 184],
        shade: "Green"
      },
      Colour {
        name: "Chocolate",
        hex: "D2691E",
        rgb: &[210, 105, 30],
        hsl: &[17, 191, 120],
        shade: "Brown"
      },
      Colour {
        name: "Christalle",
        hex: "382161",
        rgb: &[56, 33, 97],
        hsl: &[185, 125, 65],
        shade: "Violet"
      },
      Colour {
        name: "Christi",
        hex: "71A91D",
        rgb: &[113, 169, 29],
        hsl: &[59, 180, 99],
        shade: "Green"
      },
      Colour {
        name: "Christine",
        hex: "BF652E",
        rgb: &[191, 101, 46],
        hsl: &[16, 156, 118],
        shade: "Orange"
      },
      Colour {
        name: "Chrome White",
        hex: "CAC7B7",
        rgb: &[202, 199, 183],
        hsl: &[35, 38, 192],
        shade: "Green"
      },
      Colour {
        name: "Cigar",
        hex: "7D4E38",
        rgb: &[125, 78, 56],
        hsl: &[13, 97, 90],
        shade: "Brown"
      },
      Colour {
        name: "Cinder",
        hex: "242A2E",
        rgb: &[36, 42, 46],
        hsl: &[144, 31, 40],
        shade: "Grey"
      },
      Colour {
        name: "Cinderella",
        hex: "FBD7CC",
        rgb: &[251, 215, 204],
        hsl: &[9, 217, 227],
        shade: "Red"
      },
      Colour {
        name: "Cinnabar",
        hex: "E34234",
        rgb: &[227, 66, 52],
        hsl: &[3, 193, 139],
        shade: "Red"
      },
      Colour {
        name: "Cioccolato",
        hex: "5D3B2E",
        rgb: &[93, 59, 46],
        hsl: &[11, 86, 69],
        shade: "Brown"
      },
      Colour {
        name: "Citron",
        hex: "8E9A21",
        rgb: &[142, 154, 33],
        hsl: &[46, 165, 93],
        shade: "Green"
      },
      Colour {
        name: "Citrus",
        hex: "9FB70A",
        rgb: &[159, 183, 10],
        hsl: &[48, 228, 96],
        shade: "Green"
      },
      Colour {
        name: "Clam Shell",
        hex: "D2B3A9",
        rgb: &[210, 179, 169],
        hsl: &[10, 79, 189],
        shade: "Orange"
      },
      Colour {
        name: "Claret",
        hex: "6E2233",
        rgb: &[110, 34, 51],
        hsl: &[-9, 134, 72],
        shade: "Red"
      },
      Colour {
        name: "Classic Rose",
        hex: "F4C8DB",
        rgb: &[244, 200, 219],
        hsl: &[-18, 170, 222],
        shade: "Violet"
      },
      Colour {
        name: "Clay Creek",
        hex: "897E59",
        rgb: &[137, 126, 89],
        hsl: &[32, 54, 113],
        shade: "Yellow"
      },
      Colour {
        name: "Clear Day",
        hex: "DFEFEA",
        rgb: &[223, 239, 234],
        hsl: &[114, 85, 231],
        shade: "Green"
      },
      Colour {
        name: "Clinker",
        hex: "463623",
        rgb: &[70, 54, 35],
        hsl: &[23, 85, 52],
        shade: "Brown"
      },
      Colour {
        name: "Cloud",
        hex: "C2BCB1",
        rgb: &[194, 188, 177],
        hsl: &[27, 31, 185],
        shade: "Yellow"
      },
      Colour {
        name: "Cloud Burst",
        hex: "353E4F",
        rgb: &[53, 62, 79],
        hsl: &[155, 50, 66],
        shade: "Blue"
      },
      Colour {
        name: "Cloudy",
        hex: "B0A99F",
        rgb: &[176, 169, 159],
        hsl: &[24, 24, 167],
        shade: "Brown"
      },
      Colour {
        name: "Clover",
        hex: "47562F",
        rgb: &[71, 86, 47],
        hsl: &[58, 74, 66],
        shade: "Green"
      },
      Colour {
        name: "Cobalt",
        hex: "0047AB",
        rgb: &[0, 71, 171],
        hsl: &[152, 255, 85],
        shade: "Blue"
      },
      Colour {
        name: "Cocoa Bean",
        hex: "4F3835",
        rgb: &[79, 56, 53],
        hsl: &[4, 50, 66],
        shade: "Red"
      },
      Colour {
        name: "Cocoa Brown",
        hex: "35281E",
        rgb: &[53, 40, 30],
        hsl: &[18, 70, 41],
        shade: "Brown"
      },
      Colour {
        name: "Coconut Cream",
        hex: "E1DABB",
        rgb: &[225, 218, 187],
        hsl: &[34, 98, 206],
        shade: "Green"
      },
      Colour {
        name: "Cod Grey",
        hex: "2D3032",
        rgb: &[45, 48, 50],
        hsl: &[144, 13, 47],
        shade: "Grey"
      },
      Colour {
        name: "Coffee",
        hex: "726751",
        rgb: &[114, 103, 81],
        hsl: &[28, 43, 97],
        shade: "Yellow"
      },
      Colour {
        name: "Coffee Bean",
        hex: "362D26",
        rgb: &[54, 45, 38],
        hsl: &[18, 44, 46],
        shade: "Brown"
      },
      Colour {
        name: "Cognac",
        hex: "9A463D",
        rgb: &[154, 70, 61],
        hsl: &[4, 110, 107],
        shade: "Red"
      },
      Colour {
        name: "Cola",
        hex: "3C2F23",
        rgb: &[60, 47, 35],
        hsl: &[20, 67, 47],
        shade: "Brown"
      },
      Colour {
        name: "Cold Purple",
        hex: "9D8ABF",
        rgb: &[157, 138, 191],
        hsl: &[185, 74, 164],
        shade: "Violet"
      },
      Colour {
        name: "Cold Turkey",
        hex: "CAB5B2",
        rgb: &[202, 181, 178],
        hsl: &[5, 47, 190],
        shade: "Red"
      },
      Colour {
        name: "Columbia Blue",
        hex: "9BDDFF",
        rgb: &[155, 221, 255],
        hsl: &[141, 255, 205],
        shade: "Blue"
      },
      Colour {
        name: "Comet",
        hex: "636373",
        rgb: &[99, 99, 115],
        hsl: &[170, 19, 107],
        shade: "Blue"
      },
      Colour {
        name: "Como",
        hex: "4C785C",
        rgb: &[76, 120, 92],
        hsl: &[100, 57, 97],
        shade: "Green"
      },
      Colour {
        name: "Conch",
        hex: "A0B1AE",
        rgb: &[160, 177, 174],
        hsl: &[120, 25, 168],
        shade: "Green"
      },
      Colour {
        name: "Concord",
        hex: "827F79",
        rgb: &[130, 127, 121],
        hsl: &[28, 9, 125],
        shade: "Grey"
      },
      Colour {
        name: "Concrete",
        hex: "D2D1CD",
        rgb: &[210, 209, 205],
        hsl: &[34, 13, 207],
        shade: "Grey"
      },
      Colour {
        name: "Confetti",
        hex: "DDCB46",
        rgb: &[221, 203, 70],
        hsl: &[37, 175, 145],
        shade: "Green"
      },
      Colour {
        name: "Congo Brown",
        hex: "654D49",
        rgb: &[101, 77, 73],
        hsl: &[6, 41, 87],
        shade: "Brown"
      },
      Colour {
        name: "Conifer",
        hex: "B1DD52",
        rgb: &[177, 221, 82],
        hsl: &[55, 171, 151],
        shade: "Green"
      },
      Colour {
        name: "Contessa",
        hex: "C16F68",
        rgb: &[193, 111, 104],
        hsl: &[3, 106, 148],
        shade: "Red"
      },
      Colour {
        name: "Copper",
        hex: "DA8A67",
        rgb: &[218, 138, 103],
        hsl: &[12, 155, 160],
        shade: "Red"
      },
      Colour {
        name: "Copper Canyon",
        hex: "77422C",
        rgb: &[119, 66, 44],
        hsl: &[12, 117, 81],
        shade: "Orange"
      },
      Colour {
        name: "Copper Rose",
        hex: "996666",
        rgb: &[153, 102, 102],
        hsl: &[0, 50, 127],
        shade: "Violet"
      },
      Colour {
        name: "Copper Rust",
        hex: "95524C",
        rgb: &[149, 82, 76],
        hsl: &[3, 82, 112],
        shade: "Red"
      },
      Colour {
        name: "Coral",
        hex: "FF7F50",
        rgb: &[255, 127, 80],
        hsl: &[11, 255, 167],
        shade: "Orange"
      },
      Colour {
        name: "Coral Candy",
        hex: "F5D0C9",
        rgb: &[245, 208, 201],
        hsl: &[6, 175, 223],
        shade: "Red"
      },
      Colour {
        name: "Coral Red",
        hex: "FF4040",
        rgb: &[255, 64, 64],
        hsl: &[0, 255, 159],
        shade: "Red"
      },
      Colour {
        name: "Coral Tree",
        hex: "AB6E67",
        rgb: &[171, 110, 103],
        hsl: &[4, 73, 137],
        shade: "Red"
      },
      Colour {
        name: "Corduroy",
        hex: "404D49",
        rgb: &[64, 77, 73],
        hsl: &[114, 23, 70],
        shade: "Green"
      },
      Colour {
        name: "Coriander",
        hex: "BBB58D",
        rgb: &[187, 181, 141],
        hsl: &[36, 64, 163],
        shade: "Green"
      },
      Colour {
        name: "Cork",
        hex: "5A4C42",
        rgb: &[90, 76, 66],
        hsl: &[17, 39, 78],
        shade: "Brown"
      },
      Colour {
        name: "Corn",
        hex: "FBEC5D",
        rgb: &[251, 236, 93],
        hsl: &[38, 242, 171],
        shade: "Yellow"
      },
      Colour {
        name: "Corn Field",
        hex: "F8F3C4",
        rgb: &[248, 243, 196],
        hsl: &[38, 200, 222],
        shade: "Green"
      },
      Colour {
        name: "Corn Flower Blue",
        hex: "42426F",
        rgb: &[66, 66, 111],
        hsl: &[170, 64, 88],
        shade: "Blue"
      },
      Colour {
        name: "Corn Harvest",
        hex: "8D702A",
        rgb: &[141, 112, 42],
        hsl: &[30, 137, 91],
        shade: "Yellow"
      },
      Colour {
        name: "Corn Silk",
        hex: "FFF8DC",
        rgb: &[255, 248, 220],
        hsl: &[33, 255, 237],
        shade: "Yellow"
      },
      Colour {
        name: "Cornflower",
        hex: "93CCEA",
        rgb: &[147, 204, 234],
        hsl: &[142, 171, 190],
        shade: "Blue"
      },
      Colour {
        name: "Cornflower Blue",
        hex: "6495ED",
        rgb: &[100, 149, 237],
        hsl: &[154, 201, 168],
        shade: "Blue"
      },
      Colour {
        name: "Corvette",
        hex: "E9BA81",
        rgb: &[233, 186, 129],
        hsl: &[23, 179, 180],
        shade: "Orange"
      },
      Colour {
        name: "Cosmic",
        hex: "794D60",
        rgb: &[121, 77, 96],
        hsl: &[-18, 56, 98],
        shade: "Violet"
      },
      Colour {
        name: "Cosmic Latte",
        hex: "E1F8E7",
        rgb: &[225, 248, 231],
        hsl: &[96, 158, 236],
        shade: "White"
      },
      Colour {
        name: "Cosmos",
        hex: "FCD5CF",
        rgb: &[252, 213, 207],
        hsl: &[5, 225, 229],
        shade: "Red"
      },
      Colour {
        name: "Costa Del Sol",
        hex: "625D2A",
        rgb: &[98, 93, 42],
        hsl: &[38, 102, 70],
        shade: "Green"
      },
      Colour {
        name: "Cotton Candy",
        hex: "FFB7D5",
        rgb: &[255, 183, 213],
        hsl: &[-17, 255, 219],
        shade: "Red"
      },
      Colour {
        name: "Cotton Seed",
        hex: "BFBAAF",
        rgb: &[191, 186, 175],
        hsl: &[29, 28, 183],
        shade: "Yellow"
      },
      Colour {
        name: "County Green",
        hex: "1B4B35",
        rgb: &[27, 75, 53],
        hsl: &[108, 120, 51],
        shade: "Green"
      },
      Colour {
        name: "Cowboy",
        hex: "443736",
        rgb: &[68, 55, 54],
        hsl: &[3, 29, 60],
        shade: "Brown"
      },
      Colour {
        name: "Crab Apple",
        hex: "87382F",
        rgb: &[135, 56, 47],
        hsl: &[4, 123, 91],
        shade: "Red"
      },
      Colour {
        name: "Crail",
        hex: "A65648",
        rgb: &[166, 86, 72],
        hsl: &[6, 100, 119],
        shade: "Red"
      },
      Colour {
        name: "Cranberry",
        hex: "DB5079",
        rgb: &[219, 80, 121],
        hsl: &[-12, 167, 149],
        shade: "Red"
      },
      Colour {
        name: "Crater Brown",
        hex: "4D3E3C",
        rgb: &[77, 62, 60],
        hsl: &[5, 31, 68],
        shade: "Brown"
      },
      Colour {
        name: "Cream",
        hex: "FFFDD0",
        rgb: &[255, 253, 208],
        hsl: &[40, 255, 231],
        shade: "White"
      },
      Colour {
        name: "Cream Brulee",
        hex: "FFE39B",
        rgb: &[255, 227, 155],
        hsl: &[30, 255, 205],
        shade: "Yellow"
      },
      Colour {
        name: "Cream Can",
        hex: "EEC051",
        rgb: &[238, 192, 81],
        hsl: &[30, 209, 159],
        shade: "Yellow"
      },
      Colour {
        name: "Creole",
        hex: "393227",
        rgb: &[57, 50, 39],
        hsl: &[25, 47, 48],
        shade: "Brown"
      },
      Colour {
        name: "Crete",
        hex: "77712B",
        rgb: &[119, 113, 43],
        hsl: &[39, 119, 81],
        shade: "Green"
      },
      Colour {
        name: "Crimson",
        hex: "DC143C",
        rgb: &[220, 20, 60],
        hsl: &[-8, 212, 120],
        shade: "Red"
      },
      Colour {
        name: "Crocodile",
        hex: "706950",
        rgb: &[112, 105, 80],
        hsl: &[33, 42, 96],
        shade: "Yellow"
      },
      Colour {
        name: "Crown Of Thorns",
        hex: "763C33",
        rgb: &[118, 60, 51],
        hsl: &[5, 101, 84],
        shade: "Red"
      },
      Colour {
        name: "Cruise",
        hex: "B4E2D5",
        rgb: &[180, 226, 213],
        hsl: &[115, 112, 203],
        shade: "Green"
      },
      Colour {
        name: "Crusoe",
        hex: "165B31",
        rgb: &[22, 91, 49],
        hsl: &[101, 155, 56],
        shade: "Green"
      },
      Colour {
        name: "Crusta",
        hex: "F38653",
        rgb: &[243, 134, 83],
        hsl: &[13, 221, 163],
        shade: "Orange"
      },
      Colour {
        name: "Cumin",
        hex: "784430",
        rgb: &[120, 68, 48],
        hsl: &[11, 109, 84],
        shade: "Orange"
      },
      Colour {
        name: "Cumulus",
        hex: "F5F4C1",
        rgb: &[245, 244, 193],
        hsl: &[41, 184, 219],
        shade: "Green"
      },
      Colour {
        name: "Cupid",
        hex: "F5B2C5",
        rgb: &[245, 178, 197],
        hsl: &[-12, 196, 211],
        shade: "Red"
      },
      Colour {
        name: "Curious Blue",
        hex: "3D85B8",
        rgb: &[61, 133, 184],
        hsl: &[145, 128, 122],
        shade: "Blue"
      },
      Colour {
        name: "Cutty Sark",
        hex: "5C8173",
        rgb: &[92, 129, 115],
        hsl: &[111, 42, 110],
        shade: "Green"
      },
      Colour {
        name: "Cyprus",
        hex: "0F4645",
        rgb: &[15, 70, 69],
        hsl: &[126, 164, 42],
        shade: "Green"
      },
      Colour {
        name: "Dairy Cream",
        hex: "EDD2A4",
        rgb: &[237, 210, 164],
        hsl: &[26, 170, 200],
        shade: "Yellow"
      },
      Colour {
        name: "Daisy Bush",
        hex: "5B3E90",
        rgb: &[91, 62, 144],
        hsl: &[185, 101, 103],
        shade: "Violet"
      },
      Colour {
        name: "Dallas",
        hex: "664A2D",
        rgb: &[102, 74, 45],
        hsl: &[21, 98, 73],
        shade: "Brown"
      },
      Colour {
        name: "Dandelion",
        hex: "FED85D",
        rgb: &[254, 216, 93],
        hsl: &[32, 251, 173],
        shade: "Yellow"
      },
      Colour {
        name: "Danube",
        hex: "5B89C0",
        rgb: &[91, 137, 192],
        hsl: &[150, 113, 141],
        shade: "Blue"
      },
      Colour {
        name: "Dark Blue",
        hex: "00008B",
        rgb: &[0, 0, 139],
        hsl: &[170, 255, 69],
        shade: "Blue"
      },
      Colour {
        name: "Dark Brown",
        hex: "654321",
        rgb: &[101, 67, 33],
        hsl: &[21, 129, 67],
        shade: "Brown"
      },
      Colour {
        name: "Dark Cerulean",
        hex: "08457E",
        rgb: &[8, 69, 126],
        hsl: &[148, 224, 67],
        shade: "Blue"
      },
      Colour {
        name: "Dark Chestnut",
        hex: "986960",
        rgb: &[152, 105, 96],
        hsl: &[6, 57, 124],
        shade: "Red"
      },
      Colour {
        name: "Dark Coral",
        hex: "CD5B45",
        rgb: &[205, 91, 69],
        hsl: &[6, 146, 137],
        shade: "Orange"
      },
      Colour {
        name: "Dark Cyan",
        hex: "008B8B",
        rgb: &[0, 139, 139],
        hsl: &[127, 255, 69],
        shade: "Green"
      },
      Colour {
        name: "Dark Goldenrod",
        hex: "B8860B",
        rgb: &[184, 134, 11],
        hsl: &[30, 226, 97],
        shade: "Yellow"
      },
      Colour {
        name: "Dark Gray",
        hex: "A9A9A9",
        rgb: &[169, 169, 169],
        hsl: &[0, 0, 169],
        shade: "Grey"
      },
      Colour {
        name: "Dark Green",
        hex: "013220",
        rgb: &[1, 50, 32],
        hsl: &[111, 244, 25],
        shade: "Green"
      },
      Colour {
        name: "Dark Green Copper",
        hex: "4A766E",
        rgb: &[74, 118, 110],
        hsl: &[119, 58, 96],
        shade: "Green"
      },
      Colour {
        name: "Dark Khaki",
        hex: "BDB76B",
        rgb: &[189, 183, 107],
        hsl: &[39, 97, 148],
        shade: "Yellow"
      },
      Colour {
        name: "Dark Magenta",
        hex: "8B008B",
        rgb: &[139, 0, 139],
        hsl: &[-42, 255, 69],
        shade: "Violet"
      },
      Colour {
        name: "Dark Olive Green",
        hex: "556B2F",
        rgb: &[85, 107, 47],
        hsl: &[58, 99, 77],
        shade: "Green"
      },
      Colour {
        name: "Dark Orange",
        hex: "FF8C00",
        rgb: &[255, 140, 0],
        hsl: &[23, 255, 127],
        shade: "Orange"
      },
      Colour {
        name: "Dark Orchid",
        hex: "9932CC",
        rgb: &[153, 50, 204],
        hsl: &[198, 154, 127],
        shade: "Violet"
      },
      Colour {
        name: "Dark Pastel Green",
        hex: "03C03C",
        rgb: &[3, 192, 60],
        hsl: &[97, 247, 97],
        shade: "Green"
      },
      Colour {
        name: "Dark Pink",
        hex: "E75480",
        rgb: &[231, 84, 128],
        hsl: &[-12, 192, 157],
        shade: "Red"
      },
      Colour {
        name: "Dark Purple",
        hex: "871F78",
        rgb: &[135, 31, 120],
        hsl: &[-36, 159, 83],
        shade: "Violet"
      },
      Colour {
        name: "Dark Red",
        hex: "8B0000",
        rgb: &[139, 0, 0],
        hsl: &[0, 255, 69],
        shade: "Red"
      },
      Colour {
        name: "Dark Rum",
        hex: "45362B",
        rgb: &[69, 54, 43],
        hsl: &[17, 59, 56],
        shade: "Brown"
      },
      Colour {
        name: "Dark Salmon",
        hex: "E9967A",
        rgb: &[233, 150, 122],
        hsl: &[10, 182, 177],
        shade: "Orange"
      },
      Colour {
        name: "Dark Sea Green",
        hex: "8FBC8F",
        rgb: &[143, 188, 143],
        hsl: &[85, 64, 165],
        shade: "Green"
      },
      Colour {
        name: "Dark Slate",
        hex: "465352",
        rgb: &[70, 83, 82],
        hsl: &[124, 21, 76],
        shade: "Green"
      },
      Colour {
        name: "Dark Slate Blue",
        hex: "483D8B",
        rgb: &[72, 61, 139],
        hsl: &[175, 99, 100],
        shade: "Blue"
      },
      Colour {
        name: "Dark Slate Grey",
        hex: "2F4F4F",
        rgb: &[47, 79, 79],
        hsl: &[127, 64, 63],
        shade: "Grey"
      },
      Colour {
        name: "Dark Spring Green",
        hex: "177245",
        rgb: &[23, 114, 69],
        hsl: &[106, 169, 68],
        shade: "Green"
      },
      Colour {
        name: "Dark Tan",
        hex: "97694F",
        rgb: &[151, 105, 79],
        hsl: &[15, 79, 115],
        shade: "Brown"
      },
      Colour {
        name: "Dark Tangerine",
        hex: "FFA812",
        rgb: &[255, 168, 18],
        hsl: &[26, 255, 136],
        shade: "Orange"
      },
      Colour {
        name: "Dark Turquoise",
        hex: "00CED1",
        rgb: &[0, 206, 209],
        hsl: &[128, 255, 104],
        shade: "Blue"
      },
      Colour {
        name: "Dark Violet",
        hex: "9400D3",
        rgb: &[148, 0, 211],
        hsl: &[199, 255, 105],
        shade: "Violet"
      },
      Colour {
        name: "Dark Wood",
        hex: "855E42",
        rgb: &[133, 94, 66],
        hsl: &[17, 85, 99],
        shade: "Brown"
      },
      Colour {
        name: "Davy's Grey",
        hex: "788878",
        rgb: &[120, 136, 120],
        hsl: &[85, 16, 128],
        shade: "Grey"
      },
      Colour {
        name: "Dawn",
        hex: "9F9D91",
        rgb: &[159, 157, 145],
        hsl: &[36, 17, 152],
        shade: "Green"
      },
      Colour {
        name: "Dawn Pink",
        hex: "E6D6CD",
        rgb: &[230, 214, 205],
        hsl: &[15, 85, 217],
        shade: "Orange"
      },
      Colour {
        name: "De York",
        hex: "85CA87",
        rgb: &[133, 202, 135],
        hsl: &[86, 100, 167],
        shade: "Green"
      },
      Colour {
        name: "Deco",
        hex: "CCCF82",
        rgb: &[204, 207, 130],
        hsl: &[44, 113, 168],
        shade: "Green"
      },
      Colour {
        name: "Deep Blush",
        hex: "E36F8A",
        rgb: &[227, 111, 138],
        hsl: &[-9, 171, 169],
        shade: "Red"
      },
      Colour {
        name: "Deep Bronze",
        hex: "51412D",
        rgb: &[81, 65, 45],
        hsl: &[23, 72, 63],
        shade: "Brown"
      },
      Colour {
        name: "Deep Cerise",
        hex: "DA3287",
        rgb: &[218, 50, 135],
        hsl: &[-21, 177, 134],
        shade: "Violet"
      },
      Colour {
        name: "Deep Fir",
        hex: "193925",
        rgb: &[25, 57, 37],
        hsl: &[100, 99, 41],
        shade: "Green"
      },
      Colour {
        name: "Deep Koamaru",
        hex: "343467",
        rgb: &[52, 52, 103],
        hsl: &[170, 83, 77],
        shade: "Violet"
      },
      Colour {
        name: "Deep Lilac",
        hex: "9955BB",
        rgb: &[153, 85, 187],
        hsl: &[198, 109, 136],
        shade: "Violet"
      },
      Colour {
        name: "Deep Magenta",
        hex: "CC00CC",
        rgb: &[204, 0, 204],
        hsl: &[-42, 255, 102],
        shade: "Violet"
      },
      Colour {
        name: "Deep Pink",
        hex: "FF1493",
        rgb: &[255, 20, 147],
        hsl: &[-22, 255, 137],
        shade: "Red"
      },
      Colour {
        name: "Deep Sea",
        hex: "167E65",
        rgb: &[22, 126, 101],
        hsl: &[117, 179, 74],
        shade: "Green"
      },
      Colour {
        name: "Deep Sky Blue",
        hex: "00BFFF",
        rgb: &[0, 191, 255],
        hsl: &[138, 255, 127],
        shade: "Blue"
      },
      Colour {
        name: "Deep Teal",
        hex: "19443C",
        rgb: &[25, 68, 60],
        hsl: &[119, 117, 46],
        shade: "Green"
      },
      Colour {
        name: "Del Rio",
        hex: "B5998E",
        rgb: &[181, 153, 142],
        hsl: &[11, 53, 161],
        shade: "Brown"
      },
      Colour {
        name: "Dell",
        hex: "486531",
        rgb: &[72, 101, 49],
        hsl: &[66, 88, 75],
        shade: "Green"
      },
      Colour {
        name: "Delta",
        hex: "999B95",
        rgb: &[153, 155, 149],
        hsl: &[56, 7, 152],
        shade: "Grey"
      },
      Colour {
        name: "Deluge",
        hex: "8272A4",
        rgb: &[130, 114, 164],
        hsl: &[183, 54, 139],
        shade: "Violet"
      },
      Colour {
        name: "Denim",
        hex: "1560BD",
        rgb: &[21, 96, 189],
        hsl: &[151, 203, 105],
        shade: "Blue"
      },
      Colour {
        name: "Derby",
        hex: "F9E4C6",
        rgb: &[249, 228, 198],
        hsl: &[25, 206, 223],
        shade: "Yellow"
      },
      Colour {
        name: "Desert",
        hex: "A15F3B",
        rgb: &[161, 95, 59],
        hsl: &[15, 118, 110],
        shade: "Orange"
      },
      Colour {
        name: "Desert Sand",
        hex: "EDC9AF",
        rgb: &[237, 201, 175],
        hsl: &[17, 161, 206],
        shade: "Brown"
      },
      Colour {
        name: "Desert Storm",
        hex: "EDE7E0",
        rgb: &[237, 231, 224],
        hsl: &[22, 67, 230],
        shade: "Grey"
      },
      Colour {
        name: "Dew",
        hex: "E7F2E9",
        rgb: &[231, 242, 233],
        hsl: &[92, 75, 236],
        shade: "Green"
      },
      Colour {
        name: "Diesel",
        hex: "322C2B",
        rgb: &[50, 44, 43],
        hsl: &[6, 19, 46],
        shade: "Grey"
      },
      Colour {
        name: "Dim Gray",
        hex: "696969",
        rgb: &[105, 105, 105],
        hsl: &[0, 0, 105],
        shade: "Grey"
      },
      Colour {
        name: "Dingley",
        hex: "607C47",
        rgb: &[96, 124, 71],
        hsl: &[64, 69, 97],
        shade: "Green"
      },
      Colour {
        name: "Disco",
        hex: "892D4F",
        rgb: &[137, 45, 79],
        hsl: &[-15, 128, 91],
        shade: "Red"
      },
      Colour {
        name: "Dixie",
        hex: "CD8431",
        rgb: &[205, 132, 49],
        hsl: &[22, 156, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Dodger Blue",
        hex: "1E90FF",
        rgb: &[30, 144, 255],
        hsl: &[148, 255, 142],
        shade: "Blue"
      },
      Colour {
        name: "Dolly",
        hex: "F5F171",
        rgb: &[245, 241, 113],
        hsl: &[41, 221, 179],
        shade: "Green"
      },
      Colour {
        name: "Dolphin",
        hex: "6A6873",
        rgb: &[106, 104, 115],
        hsl: &[177, 12, 109],
        shade: "Violet"
      },
      Colour {
        name: "Domino",
        hex: "6C5B4C",
        rgb: &[108, 91, 76],
        hsl: &[19, 44, 92],
        shade: "Brown"
      },
      Colour {
        name: "Don Juan",
        hex: "5A4F51",
        rgb: &[90, 79, 81],
        hsl: &[-7, 16, 84],
        shade: "Brown"
      },
      Colour {
        name: "Donkey Brown",
        hex: "816E5C",
        rgb: &[129, 110, 92],
        hsl: &[20, 42, 110],
        shade: "Brown"
      },
      Colour {
        name: "Dorado",
        hex: "6E5F56",
        rgb: &[110, 95, 86],
        hsl: &[15, 31, 98],
        shade: "Brown"
      },
      Colour {
        name: "Double Colonial White",
        hex: "E4CF99",
        rgb: &[228, 207, 153],
        hsl: &[30, 148, 190],
        shade: "Yellow"
      },
      Colour {
        name: "Double Pearl Lusta",
        hex: "E9DCBE",
        rgb: &[233, 220, 190],
        hsl: &[29, 126, 211],
        shade: "Yellow"
      },
      Colour {
        name: "Double Spanish White",
        hex: "D2C3A3",
        rgb: &[210, 195, 163],
        hsl: &[28, 87, 186],
        shade: "Yellow"
      },
      Colour {
        name: "Dove Grey",
        hex: "777672",
        rgb: &[119, 118, 114],
        hsl: &[34, 5, 116],
        shade: "Grey"
      },
      Colour {
        name: "Downy",
        hex: "6FD2BE",
        rgb: &[111, 210, 190],
        hsl: &[118, 133, 160],
        shade: "Green"
      },
      Colour {
        name: "Drover",
        hex: "FBEB9B",
        rgb: &[251, 235, 155],
        hsl: &[35, 235, 203],
        shade: "Yellow"
      },
      Colour {
        name: "Dune",
        hex: "514F4A",
        rgb: &[81, 79, 74],
        hsl: &[30, 11, 77],
        shade: "Grey"
      },
      Colour {
        name: "Dust Storm",
        hex: "E5CAC0",
        rgb: &[229, 202, 192],
        hsl: &[11, 106, 210],
        shade: "Orange"
      },
      Colour {
        name: "Dusty Grey",
        hex: "AC9B9B",
        rgb: &[172, 155, 155],
        hsl: &[0, 23, 163],
        shade: "Grey"
      },
      Colour {
        name: "Dutch White",
        hex: "F0DFBB",
        rgb: &[240, 223, 187],
        hsl: &[28, 162, 213],
        shade: "Yellow"
      },
      Colour {
        name: "Eagle",
        hex: "B0AC94",
        rgb: &[176, 172, 148],
        hsl: &[36, 38, 162],
        shade: "Green"
      },
      Colour {
        name: "Earls Green",
        hex: "B8A722",
        rgb: &[184, 167, 34],
        hsl: &[37, 175, 109],
        shade: "Green"
      },
      Colour {
        name: "Early Dawn",
        hex: "FBF2DB",
        rgb: &[251, 242, 219],
        hsl: &[30, 203, 235],
        shade: "Yellow"
      },
      Colour {
        name: "East Bay",
        hex: "47526E",
        rgb: &[71, 82, 110],
        hsl: &[158, 54, 90],
        shade: "Blue"
      },
      Colour {
        name: "East Side",
        hex: "AA8CBC",
        rgb: &[170, 140, 188],
        hsl: &[196, 67, 164],
        shade: "Violet"
      },
      Colour {
        name: "Eastern Blue",
        hex: "00879F",
        rgb: &[0, 135, 159],
        hsl: &[133, 255, 79],
        shade: "Blue"
      },
      Colour {
        name: "Ebb",
        hex: "E6D8D4",
        rgb: &[230, 216, 212],
        hsl: &[9, 67, 221],
        shade: "Red"
      },
      Colour {
        name: "Ebony",
        hex: "313337",
        rgb: &[49, 51, 55],
        hsl: &[155, 14, 52],
        shade: "Grey"
      },
      Colour {
        name: "Ebony Clay",
        hex: "323438",
        rgb: &[50, 52, 56],
        hsl: &[155, 14, 52],
        shade: "Grey"
      },
      Colour {
        name: "Echo Blue",
        hex: "A4AFCD",
        rgb: &[164, 175, 205],
        hsl: &[158, 74, 184],
        shade: "Blue"
      },
      Colour {
        name: "Eclipse",
        hex: "3F3939",
        rgb: &[63, 57, 57],
        hsl: &[0, 12, 60],
        shade: "Grey"
      },
      Colour {
        name: "Ecru",
        hex: "C2B280",
        rgb: &[194, 178, 128],
        hsl: &[32, 89, 161],
        shade: "Brown"
      },
      Colour {
        name: "Ecru White",
        hex: "D6D1C0",
        rgb: &[214, 209, 192],
        hsl: &[32, 53, 203],
        shade: "Green"
      },
      Colour {
        name: "Ecstasy",
        hex: "C96138",
        rgb: &[201, 97, 56],
        hsl: &[12, 146, 128],
        shade: "Orange"
      },
      Colour {
        name: "Eden",
        hex: "266255",
        rgb: &[38, 98, 85],
        hsl: &[118, 112, 68],
        shade: "Green"
      },
      Colour {
        name: "Edgewater",
        hex: "C1D8C5",
        rgb: &[193, 216, 197],
        hsl: &[92, 58, 204],
        shade: "Green"
      },
      Colour {
        name: "Edward",
        hex: "97A49A",
        rgb: &[151, 164, 154],
        hsl: &[94, 17, 157],
        shade: "Green"
      },
      Colour {
        name: "Egg Sour",
        hex: "F9E4C5",
        rgb: &[249, 228, 197],
        hsl: &[25, 207, 223],
        shade: "Yellow"
      },
      Colour {
        name: "Eggplant",
        hex: "990066",
        rgb: &[153, 0, 102],
        hsl: &[-28, 255, 76],
        shade: "Violet"
      },
      Colour {
        name: "Egyptian Blue",
        hex: "1034A6",
        rgb: &[16, 52, 166],
        hsl: &[159, 210, 91],
        shade: "Blue"
      },
      Colour {
        name: "El Paso",
        hex: "39392C",
        rgb: &[57, 57, 44],
        hsl: &[42, 32, 50],
        shade: "Green"
      },
      Colour {
        name: "El Salva",
        hex: "8F4E45",
        rgb: &[143, 78, 69],
        hsl: &[5, 89, 105],
        shade: "Red"
      },
      Colour {
        name: "Electric Blue",
        hex: "7DF9FF",
        rgb: &[125, 249, 255],
        hsl: &[129, 255, 190],
        shade: "Blue"
      },
      Colour {
        name: "Electric Indigo",
        hex: "6600FF",
        rgb: &[102, 0, 255],
        hsl: &[187, 255, 127],
        shade: "Violet"
      },
      Colour {
        name: "Electric Lime",
        hex: "CCFF00",
        rgb: &[204, 255, 0],
        hsl: &[50, 255, 127],
        shade: "Green"
      },
      Colour {
        name: "Electric Purple",
        hex: "BF00FF",
        rgb: &[191, 0, 255],
        hsl: &[201, 255, 127],
        shade: "Violet"
      },
      Colour {
        name: "Elephant",
        hex: "243640",
        rgb: &[36, 54, 64],
        hsl: &[142, 71, 50],
        shade: "Blue"
      },
      Colour {
        name: "Elf Green",
        hex: "1B8A6B",
        rgb: &[27, 138, 107],
        hsl: &[115, 171, 82],
        shade: "Green"
      },
      Colour {
        name: "Elm",
        hex: "297B76",
        rgb: &[41, 123, 118],
        hsl: &[124, 127, 82],
        shade: "Green"
      },
      Colour {
        name: "Emerald",
        hex: "50C878",
        rgb: &[80, 200, 120],
        hsl: &[99, 133, 140],
        shade: "Green"
      },
      Colour {
        name: "Eminence",
        hex: "6E3974",
        rgb: &[110, 57, 116],
        hsl: &[208, 86, 86],
        shade: "Violet"
      },
      Colour {
        name: "Emperor",
        hex: "50494A",
        rgb: &[80, 73, 74],
        hsl: &[-6, 11, 76],
        shade: "Grey"
      },
      Colour {
        name: "Empress",
        hex: "7C7173",
        rgb: &[124, 113, 115],
        hsl: &[-7, 11, 118],
        shade: "Grey"
      },
      Colour {
        name: "Endeavour",
        hex: "29598B",
        rgb: &[41, 89, 139],
        hsl: &[149, 138, 89],
        shade: "Blue"
      },
      Colour {
        name: "Energy Yellow",
        hex: "F5D752",
        rgb: &[245, 215, 82],
        hsl: &[34, 227, 163],
        shade: "Yellow"
      },
      Colour {
        name: "English Holly",
        hex: "274234",
        rgb: &[39, 66, 52],
        hsl: &[105, 65, 52],
        shade: "Green"
      },
      Colour {
        name: "Envy",
        hex: "8BA58F",
        rgb: &[139, 165, 143],
        hsl: &[91, 32, 152],
        shade: "Green"
      },
      Colour {
        name: "Equator",
        hex: "DAB160",
        rgb: &[218, 177, 96],
        hsl: &[28, 158, 157],
        shade: "Yellow"
      },
      Colour {
        name: "Espresso",
        hex: "4E312D",
        rgb: &[78, 49, 45],
        hsl: &[5, 68, 61],
        shade: "Red"
      },
      Colour {
        name: "Eternity",
        hex: "2D2F28",
        rgb: &[45, 47, 40],
        hsl: &[54, 20, 43],
        shade: "Green"
      },
      Colour {
        name: "Eucalyptus",
        hex: "329760",
        rgb: &[50, 151, 96],
        hsl: &[104, 128, 100],
        shade: "Green"
      },
      Colour {
        name: "Eunry",
        hex: "CDA59C",
        rgb: &[205, 165, 156],
        hsl: &[7, 83, 180],
        shade: "Red"
      },
      Colour {
        name: "Evening Sea",
        hex: "26604F",
        rgb: &[38, 96, 79],
        hsl: &[115, 110, 67],
        shade: "Green"
      },
      Colour {
        name: "Everglade",
        hex: "264334",
        rgb: &[38, 67, 52],
        hsl: &[105, 70, 52],
        shade: "Green"
      },
      Colour {
        name: "Fair Pink",
        hex: "F3E5DC",
        rgb: &[243, 229, 220],
        hsl: &[16, 124, 231],
        shade: "Orange"
      },
      Colour {
        name: "Falcon",
        hex: "6E5A5B",
        rgb: &[110, 90, 91],
        hsl: &[-2, 25, 100],
        shade: "Brown"
      },
      Colour {
        name: "Fallow",
        hex: "C19A6B",
        rgb: &[193, 154, 107],
        hsl: &[23, 104, 150],
        shade: "Brown"
      },
      Colour {
        name: "Falu Red",
        hex: "801818",
        rgb: &[128, 24, 24],
        hsl: &[0, 174, 76],
        shade: "Red"
      },
      Colour {
        name: "Fantasy",
        hex: "F2E6DD",
        rgb: &[242, 230, 221],
        hsl: &[18, 113, 231],
        shade: "Orange"
      },
      Colour {
        name: "Fedora",
        hex: "625665",
        rgb: &[98, 86, 101],
        hsl: &[204, 20, 93],
        shade: "Violet"
      },
      Colour {
        name: "Feijoa",
        hex: "A5D785",
        rgb: &[165, 215, 133],
        hsl: &[68, 129, 174],
        shade: "Green"
      },
      Colour {
        name: "Feldgrau",
        hex: "4D5D53",
        rgb: &[77, 93, 83],
        hsl: &[100, 24, 85],
        shade: "Grey"
      },
      Colour {
        name: "Feldspar",
        hex: "D19275",
        rgb: &[209, 146, 117],
        hsl: &[13, 127, 163],
        shade: "Red"
      },
      Colour {
        name: "Fern",
        hex: "63B76C",
        rgb: &[99, 183, 108],
        hsl: &[89, 93, 141],
        shade: "Green"
      },
      Colour {
        name: "Fern Green",
        hex: "4F7942",
        rgb: &[79, 121, 66],
        hsl: &[74, 74, 93],
        shade: "Green"
      },
      Colour {
        name: "Ferra",
        hex: "876A68",
        rgb: &[135, 106, 104],
        hsl: &[2, 33, 119],
        shade: "Brown"
      },
      Colour {
        name: "Festival",
        hex: "EACC4A",
        rgb: &[234, 204, 74],
        hsl: &[34, 201, 154],
        shade: "Yellow"
      },
      Colour {
        name: "Feta",
        hex: "DBE0D0",
        rgb: &[219, 224, 208],
        hsl: &[55, 52, 216],
        shade: "Green"
      },
      Colour {
        name: "Fiery Orange",
        hex: "B1592F",
        rgb: &[177, 89, 47],
        hsl: &[13, 147, 112],
        shade: "Orange"
      },
      Colour {
        name: "Fiji Green",
        hex: "636F22",
        rgb: &[99, 111, 34],
        hsl: &[49, 135, 72],
        shade: "Green"
      },
      Colour {
        name: "Finch",
        hex: "75785A",
        rgb: &[117, 120, 90],
        hsl: &[46, 36, 105],
        shade: "Green"
      },
      Colour {
        name: "Finlandia",
        hex: "61755B",
        rgb: &[97, 117, 91],
        hsl: &[75, 31, 104],
        shade: "Green"
      },
      Colour {
        name: "Finn",
        hex: "694554",
        rgb: &[105, 69, 84],
        hsl: &[-17, 52, 87],
        shade: "Violet"
      },
      Colour {
        name: "Fiord",
        hex: "4B5A62",
        rgb: &[75, 90, 98],
        hsl: &[142, 33, 86],
        shade: "Blue"
      },
      Colour {
        name: "Fire",
        hex: "8F3F2A",
        rgb: &[143, 63, 42],
        hsl: &[8, 139, 92],
        shade: "Orange"
      },
      Colour {
        name: "Fire Brick",
        hex: "B22222",
        rgb: &[178, 34, 34],
        hsl: &[0, 173, 105],
        shade: "Red"
      },
      Colour {
        name: "Fire Bush",
        hex: "E09842",
        rgb: &[224, 152, 66],
        hsl: &[23, 183, 145],
        shade: "Yellow"
      },
      Colour {
        name: "Fire Engine Red",
        hex: "CE1620",
        rgb: &[206, 22, 32],
        hsl: &[-2, 205, 114],
        shade: "Red"
      },
      Colour {
        name: "Firefly",
        hex: "314643",
        rgb: &[49, 70, 67],
        hsl: &[121, 45, 59],
        shade: "Green"
      },
      Colour {
        name: "Flame Pea",
        hex: "BE5C48",
        rgb: &[190, 92, 72],
        hsl: &[7, 121, 131],
        shade: "Orange"
      },
      Colour {
        name: "Flame Red",
        hex: "86282E",
        rgb: &[134, 40, 46],
        hsl: &[-2, 137, 87],
        shade: "Red"
      },
      Colour {
        name: "Flamenco",
        hex: "EA8645",
        rgb: &[234, 134, 69],
        hsl: &[16, 203, 151],
        shade: "Orange"
      },
      Colour {
        name: "Flamingo",
        hex: "E1634F",
        rgb: &[225, 99, 79],
        hsl: &[5, 180, 152],
        shade: "Orange"
      },
      Colour {
        name: "Flax",
        hex: "EEDC82",
        rgb: &[238, 220, 130],
        hsl: &[35, 193, 184],
        shade: "Yellow"
      },
      Colour {
        name: "Flint",
        hex: "716E61",
        rgb: &[113, 110, 97],
        hsl: &[34, 19, 105],
        shade: "Green"
      },
      Colour {
        name: "Flirt",
        hex: "7A2E4D",
        rgb: &[122, 46, 77],
        hsl: &[-17, 115, 84],
        shade: "Red"
      },
      Colour {
        name: "Floral White",
        hex: "FFFAF0",
        rgb: &[255, 250, 240],
        hsl: &[28, 255, 247],
        shade: "White"
      },
      Colour {
        name: "Foam",
        hex: "D0EAE8",
        rgb: &[208, 234, 232],
        hsl: &[124, 97, 221],
        shade: "Green"
      },
      Colour {
        name: "Fog",
        hex: "D5C7E8",
        rgb: &[213, 199, 232],
        hsl: &[188, 106, 215],
        shade: "Violet"
      },
      Colour {
        name: "Foggy Grey",
        hex: "A7A69D",
        rgb: &[167, 166, 157],
        hsl: &[38, 13, 162],
        shade: "Grey"
      },
      Colour {
        name: "Forest Green",
        hex: "228B22",
        rgb: &[34, 139, 34],
        hsl: &[85, 154, 86],
        shade: "Green"
      },
      Colour {
        name: "Forget Me Not",
        hex: "FDEFDB",
        rgb: &[253, 239, 219],
        hsl: &[25, 228, 236],
        shade: "Yellow"
      },
      Colour {
        name: "Fountain Blue",
        hex: "65ADB2",
        rgb: &[101, 173, 178],
        hsl: &[130, 84, 139],
        shade: "Blue"
      },
      Colour {
        name: "Frangipani",
        hex: "FFD7A0",
        rgb: &[255, 215, 160],
        hsl: &[24, 255, 207],
        shade: "Yellow"
      },
      Colour {
        name: "Free Speech Aquamarine",
        hex: "029D74",
        rgb: &[2, 157, 116],
        hsl: &[116, 248, 79],
        shade: "Green"
      },
      Colour {
        name: "Free Speech Blue",
        hex: "4156C5",
        rgb: &[65, 86, 197],
        hsl: &[163, 135, 131],
        shade: "Blue"
      },
      Colour {
        name: "Free Speech Green",
        hex: "09F911",
        rgb: &[9, 249, 17],
        hsl: &[86, 242, 129],
        shade: "Green"
      },
      Colour {
        name: "Free Speech Magenta",
        hex: "E35BD8",
        rgb: &[227, 91, 216],
        hsl: &[-39, 180, 159],
        shade: "Red"
      },
      Colour {
        name: "Free Speech Red",
        hex: "C00000",
        rgb: &[192, 0, 0],
        hsl: &[0, 255, 96],
        shade: "Red"
      },
      Colour {
        name: "French Grey",
        hex: "BFBDC1",
        rgb: &[191, 189, 193],
        hsl: &[191, 7, 191],
        shade: "Grey"
      },
      Colour {
        name: "French Lilac",
        hex: "DEB7D9",
        rgb: &[222, 183, 217],
        hsl: &[-37, 94, 202],
        shade: "Violet"
      },
      Colour {
        name: "French Pass",
        hex: "A4D2E0",
        rgb: &[164, 210, 224],
        hsl: &[137, 125, 194],
        shade: "Blue"
      },
      Colour {
        name: "French Rose",
        hex: "F64A8A",
        rgb: &[246, 74, 138],
        hsl: &[-15, 230, 160],
        shade: "Red"
      },
      Colour {
        name: "Friar Grey",
        hex: "86837A",
        rgb: &[134, 131, 122],
        hsl: &[31, 12, 128],
        shade: "Grey"
      },
      Colour {
        name: "Fringy Flower",
        hex: "B4E1BB",
        rgb: &[180, 225, 187],
        hsl: &[91, 109, 202],
        shade: "Green"
      },
      Colour {
        name: "Froly",
        hex: "E56D75",
        rgb: &[229, 109, 117],
        hsl: &[-2, 177, 169],
        shade: "Red"
      },
      Colour {
        name: "Frost",
        hex: "E1E4C5",
        rgb: &[225, 228, 197],
        hsl: &[46, 93, 212],
        shade: "Green"
      },
      Colour {
        name: "Frosted Mint",
        hex: "E2F2E4",
        rgb: &[226, 242, 228],
        hsl: &[90, 97, 234],
        shade: "Green"
      },
      Colour {
        name: "Frostee",
        hex: "DBE5D2",
        rgb: &[219, 229, 210],
        hsl: &[64, 68, 219],
        shade: "Green"
      },
      Colour {
        name: "Fruit Salad",
        hex: "4BA351",
        rgb: &[75, 163, 81],
        hsl: &[87, 94, 119],
        shade: "Green"
      },
      Colour {
        name: "Fuchsia",
        hex: "C154C1",
        rgb: &[193, 84, 193],
        hsl: &[-42, 119, 138],
        shade: "Violet"
      },
      Colour {
        name: "Fuchsia Pink",
        hex: "FF77FF",
        rgb: &[255, 119, 255],
        hsl: &[-42, 255, 187],
        shade: "Red"
      },
      Colour {
        name: "Fuego",
        hex: "C2D62E",
        rgb: &[194, 214, 46],
        hsl: &[47, 171, 130],
        shade: "Green"
      },
      Colour {
        name: "Fuel Yellow",
        hex: "D19033",
        rgb: &[209, 144, 51],
        hsl: &[25, 161, 130],
        shade: "Yellow"
      },
      Colour {
        name: "Fun Blue",
        hex: "335083",
        rgb: &[51, 80, 131],
        hsl: &[154, 112, 90],
        shade: "Blue"
      },
      Colour {
        name: "Fun Green",
        hex: "15633D",
        rgb: &[21, 99, 61],
        hsl: &[106, 165, 60],
        shade: "Green"
      },
      Colour {
        name: "Fuscous Grey",
        hex: "3C3B3C",
        rgb: &[60, 59, 60],
        hsl: &[-42, 2, 59],
        shade: "Grey"
      },
      Colour {
        name: "Fuzzy Wuzzy Brown",
        hex: "C45655",
        rgb: &[196, 86, 85],
        hsl: &[0, 123, 140],
        shade: "Brown"
      },
      Colour {
        name: "Gable Green",
        hex: "2C4641",
        rgb: &[44, 70, 65],
        hsl: &[119, 58, 57],
        shade: "Green"
      },
      Colour {
        name: "Gainsboro",
        hex: "DCDCDC",
        rgb: &[220, 220, 220],
        hsl: &[0, 0, 220],
        shade: "White"
      },
      Colour {
        name: "Gallery",
        hex: "DCD7D1",
        rgb: &[220, 215, 209],
        hsl: &[23, 34, 214],
        shade: "Grey"
      },
      Colour {
        name: "Galliano",
        hex: "D8A723",
        rgb: &[216, 167, 35],
        hsl: &[30, 183, 125],
        shade: "Yellow"
      },
      Colour {
        name: "Gamboge",
        hex: "E49B0F",
        rgb: &[228, 155, 15],
        hsl: &[27, 223, 121],
        shade: "Yellow"
      },
      Colour {
        name: "Geebung",
        hex: "C5832E",
        rgb: &[197, 131, 46],
        hsl: &[23, 158, 121],
        shade: "Yellow"
      },
      Colour {
        name: "Genoa",
        hex: "31796D",
        rgb: &[49, 121, 109],
        hsl: &[120, 108, 85],
        shade: "Green"
      },
      Colour {
        name: "Geraldine",
        hex: "E77B75",
        rgb: &[231, 123, 117],
        hsl: &[2, 179, 174],
        shade: "Red"
      },
      Colour {
        name: "Geyser",
        hex: "CBD0CF",
        rgb: &[203, 208, 207],
        hsl: &[119, 12, 205],
        shade: "Grey"
      },
      Colour {
        name: "Ghost",
        hex: "C0BFC7",
        rgb: &[192, 191, 199],
        hsl: &[175, 16, 195],
        shade: "Blue"
      },
      Colour {
        name: "Ghost White",
        hex: "F8F8FF",
        rgb: &[248, 248, 255],
        hsl: &[170, 255, 251],
        shade: "White"
      },
      Colour {
        name: "Gigas",
        hex: "564786",
        rgb: &[86, 71, 134],
        hsl: &[180, 78, 102],
        shade: "Violet"
      },
      Colour {
        name: "Gimblet",
        hex: "B9AD61",
        rgb: &[185, 173, 97],
        hsl: &[36, 98, 141],
        shade: "Green"
      },
      Colour {
        name: "Gin",
        hex: "D9DFCD",
        rgb: &[217, 223, 205],
        hsl: &[56, 55, 214],
        shade: "Green"
      },
      Colour {
        name: "Gin Fizz",
        hex: "F8EACA",
        rgb: &[248, 234, 202],
        hsl: &[29, 195, 225],
        shade: "Yellow"
      },
      Colour {
        name: "Givry",
        hex: "EBD4AE",
        rgb: &[235, 212, 174],
        hsl: &[26, 154, 204],
        shade: "Yellow"
      },
      Colour {
        name: "Glacier",
        hex: "78B1BF",
        rgb: &[120, 177, 191],
        hsl: &[135, 90, 155],
        shade: "Blue"
      },
      Colour {
        name: "Glade Green",
        hex: "5F8151",
        rgb: &[95, 129, 81],
        hsl: &[72, 58, 105],
        shade: "Green"
      },
      Colour {
        name: "Go Ben",
        hex: "786E4C",
        rgb: &[120, 110, 76],
        hsl: &[32, 57, 97],
        shade: "Yellow"
      },
      Colour {
        name: "Goblin",
        hex: "34533D",
        rgb: &[52, 83, 61],
        hsl: &[97, 58, 67],
        shade: "Green"
      },
      Colour {
        name: "Gold",
        hex: "FFD700",
        rgb: &[255, 215, 0],
        hsl: &[35, 255, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Gold Drop",
        hex: "D56C30",
        rgb: &[213, 108, 48],
        hsl: &[15, 168, 130],
        shade: "Orange"
      },
      Colour {
        name: "Gold Tips",
        hex: "E2B227",
        rgb: &[226, 178, 39],
        hsl: &[31, 194, 132],
        shade: "Yellow"
      },
      Colour {
        name: "Golden Bell",
        hex: "CA8136",
        rgb: &[202, 129, 54],
        hsl: &[21, 148, 128],
        shade: "Orange"
      },
      Colour {
        name: "Golden Brown",
        hex: "996515",
        rgb: &[153, 101, 21],
        hsl: &[25, 193, 87],
        shade: "Brown"
      },
      Colour {
        name: "Golden Dream",
        hex: "F1CC2B",
        rgb: &[241, 204, 43],
        hsl: &[34, 223, 142],
        shade: "Yellow"
      },
      Colour {
        name: "Golden Fizz",
        hex: "EBDE31",
        rgb: &[235, 222, 49],
        hsl: &[39, 209, 142],
        shade: "Green"
      },
      Colour {
        name: "Golden Glow",
        hex: "F9D77E",
        rgb: &[249, 215, 126],
        hsl: &[30, 232, 187],
        shade: "Yellow"
      },
      Colour {
        name: "Golden Poppy",
        hex: "FCC200",
        rgb: &[252, 194, 0],
        hsl: &[32, 255, 126],
        shade: "Yellow"
      },
      Colour {
        name: "Golden Sand",
        hex: "EACE6A",
        rgb: &[234, 206, 106],
        hsl: &[33, 191, 170],
        shade: "Yellow"
      },
      Colour {
        name: "Golden Tainoi",
        hex: "FFC152",
        rgb: &[255, 193, 82],
        hsl: &[27, 255, 168],
        shade: "Yellow"
      },
      Colour {
        name: "Golden Yellow",
        hex: "FFDF00",
        rgb: &[255, 223, 0],
        hsl: &[37, 255, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Goldenrod",
        hex: "DBDB70",
        rgb: &[219, 219, 112],
        hsl: &[42, 152, 165],
        shade: "Yellow"
      },
      Colour {
        name: "Gondola",
        hex: "373332",
        rgb: &[55, 51, 50],
        hsl: &[8, 12, 52],
        shade: "Grey"
      },
      Colour {
        name: "Gordons Green",
        hex: "29332B",
        rgb: &[41, 51, 43],
        hsl: &[93, 27, 46],
        shade: "Green"
      },
      Colour {
        name: "Gorse",
        hex: "FDE336",
        rgb: &[253, 227, 54],
        hsl: &[36, 249, 153],
        shade: "Green"
      },
      Colour {
        name: "Gossamer",
        hex: "399F86",
        rgb: &[57, 159, 134],
        hsl: &[117, 120, 108],
        shade: "Green"
      },
      Colour {
        name: "Gossip",
        hex: "9FD385",
        rgb: &[159, 211, 133],
        hsl: &[70, 119, 172],
        shade: "Green"
      },
      Colour {
        name: "Gothic",
        hex: "698890",
        rgb: &[105, 136, 144],
        hsl: &[136, 39, 124],
        shade: "Blue"
      },
      Colour {
        name: "Governor Bay",
        hex: "51559B",
        rgb: &[81, 85, 155],
        hsl: &[167, 79, 117],
        shade: "Blue"
      },
      Colour {
        name: "Grain Brown",
        hex: "CAB8A2",
        rgb: &[202, 184, 162],
        hsl: &[23, 69, 181],
        shade: "Yellow"
      },
      Colour {
        name: "Grandis",
        hex: "FFCD73",
        rgb: &[255, 205, 115],
        hsl: &[27, 255, 185],
        shade: "Yellow"
      },
      Colour {
        name: "Granite Green",
        hex: "8B8265",
        rgb: &[139, 130, 101],
        hsl: &[32, 40, 120],
        shade: "Yellow"
      },
      Colour {
        name: "Granny Apple",
        hex: "C5E7CD",
        rgb: &[197, 231, 205],
        hsl: &[95, 105, 214],
        shade: "Green"
      },
      Colour {
        name: "Granny Smith",
        hex: "7B948C",
        rgb: &[123, 148, 140],
        hsl: &[113, 26, 135],
        shade: "Green"
      },
      Colour {
        name: "Granny Smith Apple",
        hex: "9DE093",
        rgb: &[157, 224, 147],
        hsl: &[79, 141, 185],
        shade: "Green"
      },
      Colour {
        name: "Grape",
        hex: "413D4B",
        rgb: &[65, 61, 75],
        hsl: &[182, 26, 68],
        shade: "Violet"
      },
      Colour {
        name: "Graphite",
        hex: "383428",
        rgb: &[56, 52, 40],
        hsl: &[31, 42, 48],
        shade: "Yellow"
      },
      Colour {
        name: "Gravel",
        hex: "4A4B46",
        rgb: &[74, 75, 70],
        hsl: &[50, 8, 72],
        shade: "Grey"
      },
      Colour {
        name: "Green",
        hex: "008000",
        rgb: &[0, 128, 0],
        hsl: &[85, 255, 64],
        shade: "Green"
      },
      Colour {
        name: "Green House",
        hex: "3E6334",
        rgb: &[62, 99, 52],
        hsl: &[75, 79, 75],
        shade: "Green"
      },
      Colour {
        name: "Green Kelp",
        hex: "393D2A",
        rgb: &[57, 61, 42],
        hsl: &[51, 47, 51],
        shade: "Green"
      },
      Colour {
        name: "Green Leaf",
        hex: "526B2D",
        rgb: &[82, 107, 45],
        hsl: &[59, 104, 76],
        shade: "Green"
      },
      Colour {
        name: "Green Mist",
        hex: "BFC298",
        rgb: &[191, 194, 152],
        hsl: &[45, 65, 173],
        shade: "Green"
      },
      Colour {
        name: "Green Pea",
        hex: "266242",
        rgb: &[38, 98, 66],
        hsl: &[104, 112, 68],
        shade: "Green"
      },
      Colour {
        name: "Green Smoke",
        hex: "9CA664",
        rgb: &[156, 166, 100],
        hsl: &[48, 68, 133],
        shade: "Green"
      },
      Colour {
        name: "Green Spring",
        hex: "A9AF99",
        rgb: &[169, 175, 153],
        hsl: &[54, 30, 163],
        shade: "Green"
      },
      Colour {
        name: "Green Vogue",
        hex: "23414E",
        rgb: &[35, 65, 78],
        hsl: &[140, 97, 56],
        shade: "Blue"
      },
      Colour {
        name: "Green Waterloo",
        hex: "2C2D24",
        rgb: &[44, 45, 36],
        hsl: &[47, 28, 40],
        shade: "Green"
      },
      Colour {
        name: "Green White",
        hex: "DEDDCB",
        rgb: &[222, 221, 203],
        hsl: &[40, 57, 212],
        shade: "Green"
      },
      Colour {
        name: "Green Yellow",
        hex: "ADFF2F",
        rgb: &[173, 255, 47],
        hsl: &[59, 255, 151],
        shade: "Green"
      },
      Colour {
        name: "Grenadier",
        hex: "C14D36",
        rgb: &[193, 77, 54],
        hsl: &[7, 143, 123],
        shade: "Orange"
      },
      Colour {
        name: "Grey",
        hex: "808080",
        rgb: &[128, 128, 128],
        hsl: &[0, 0, 128],
        shade: "Grey"
      },
      Colour {
        name: "Grey Chateau",
        hex: "9FA3A7",
        rgb: &[159, 163, 167],
        hsl: &[148, 11, 163],
        shade: "Grey"
      },
      Colour {
        name: "Grey Nickel",
        hex: "BDBAAE",
        rgb: &[189, 186, 174],
        hsl: &[33, 26, 181],
        shade: "Green"
      },
      Colour {
        name: "Grey Nurse",
        hex: "D1D3CC",
        rgb: &[209, 211, 204],
        hsl: &[54, 18, 207],
        shade: "Grey"
      },
      Colour {
        name: "Grey Olive",
        hex: "A19A7F",
        rgb: &[161, 154, 127],
        hsl: &[33, 39, 144],
        shade: "Yellow"
      },
      Colour {
        name: "Grey Suit",
        hex: "9391A0",
        rgb: &[147, 145, 160],
        hsl: &[175, 18, 152],
        shade: "Blue"
      },
      Colour {
        name: "Grey-Asparagus",
        hex: "465945",
        rgb: &[70, 89, 69],
        hsl: &[82, 32, 79],
        shade: "Green"
      },
      Colour {
        name: "Guardsman Red",
        hex: "952E31",
        rgb: &[149, 46, 49],
        hsl: &[-1, 134, 97],
        shade: "Red"
      },
      Colour {
        name: "Gulf Blue",
        hex: "343F5C",
        rgb: &[52, 63, 92],
        hsl: &[158, 70, 72],
        shade: "Blue"
      },
      Colour {
        name: "Gulf Stream",
        hex: "74B2A8",
        rgb: &[116, 178, 168],
        hsl: &[120, 73, 147],
        shade: "Green"
      },
      Colour {
        name: "Gull Grey",
        hex: "A4ADB0",
        rgb: &[164, 173, 176],
        hsl: &[138, 17, 170],
        shade: "Grey"
      },
      Colour {
        name: "Gum Leaf",
        hex: "ACC9B2",
        rgb: &[172, 201, 178],
        hsl: &[93, 53, 186],
        shade: "Green"
      },
      Colour {
        name: "Gumbo",
        hex: "718F8A",
        rgb: &[113, 143, 138],
        hsl: &[120, 30, 128],
        shade: "Green"
      },
      Colour {
        name: "Gun Powder",
        hex: "484753",
        rgb: &[72, 71, 83],
        hsl: &[173, 19, 77],
        shade: "Violet"
      },
      Colour {
        name: "Gunmetal",
        hex: "2C3539",
        rgb: &[44, 53, 57],
        hsl: &[140, 32, 50],
        shade: "Blue"
      },
      Colour {
        name: "Gunsmoke",
        hex: "7A7C76",
        rgb: &[122, 124, 118],
        hsl: &[56, 6, 121],
        shade: "Grey"
      },
      Colour {
        name: "Gurkha",
        hex: "989171",
        rgb: &[152, 145, 113],
        hsl: &[34, 40, 132],
        shade: "Green"
      },
      Colour {
        name: "Hacienda",
        hex: "9E8022",
        rgb: &[158, 128, 34],
        hsl: &[32, 164, 96],
        shade: "Yellow"
      },
      Colour {
        name: "Hairy Heath",
        hex: "633528",
        rgb: &[99, 53, 40],
        hsl: &[9, 108, 69],
        shade: "Brown"
      },
      Colour {
        name: "Haiti",
        hex: "2C2A35",
        rgb: &[44, 42, 53],
        hsl: &[177, 29, 47],
        shade: "Violet"
      },
      Colour {
        name: "Half And Half",
        hex: "EDE7C8",
        rgb: &[237, 231, 200],
        hsl: &[35, 129, 218],
        shade: "Green"
      },
      Colour {
        name: "Half Baked",
        hex: "558F93",
        rgb: &[85, 143, 147],
        hsl: &[130, 68, 115],
        shade: "Blue"
      },
      Colour {
        name: "Half Colonial White",
        hex: "F2E5BF",
        rgb: &[242, 229, 191],
        hsl: &[31, 168, 216],
        shade: "Yellow"
      },
      Colour {
        name: "Half Dutch White",
        hex: "FBF0D6",
        rgb: &[251, 240, 214],
        hsl: &[29, 209, 232],
        shade: "Yellow"
      },
      Colour {
        name: "Half Pearl Lusta",
        hex: "F1EAD7",
        rgb: &[241, 234, 215],
        hsl: &[31, 122, 227],
        shade: "Yellow"
      },
      Colour {
        name: "Half Spanish White",
        hex: "E6DBC7",
        rgb: &[230, 219, 199],
        hsl: &[27, 97, 214],
        shade: "Yellow"
      },
      Colour {
        name: "Hampton",
        hex: "E8D4A2",
        rgb: &[232, 212, 162],
        hsl: &[30, 153, 196],
        shade: "Yellow"
      },
      Colour {
        name: "Han Purple",
        hex: "5218FA",
        rgb: &[82, 24, 250],
        hsl: &[180, 244, 137],
        shade: "Violet"
      },
      Colour {
        name: "Harlequin",
        hex: "3FFF00",
        rgb: &[63, 255, 0],
        hsl: &[74, 255, 127],
        shade: "Green"
      },
      Colour {
        name: "Harley Davidson Orange",
        hex: "C93413",
        rgb: &[201, 52, 19],
        hsl: &[7, 210, 109],
        shade: "Orange"
      },
      Colour {
        name: "Harp",
        hex: "CBCEC0",
        rgb: &[203, 206, 192],
        hsl: &[51, 31, 199],
        shade: "Green"
      },
      Colour {
        name: "Harvest Gold",
        hex: "EAB76A",
        rgb: &[234, 183, 106],
        hsl: &[25, 191, 170],
        shade: "Yellow"
      },
      Colour {
        name: "Havana",
        hex: "3B2B2C",
        rgb: &[59, 43, 44],
        hsl: &[-2, 40, 51],
        shade: "Brown"
      },
      Colour {
        name: "Havelock Blue",
        hex: "5784C1",
        rgb: &[87, 132, 193],
        hsl: &[151, 117, 140],
        shade: "Blue"
      },
      Colour {
        name: "Hawaiian Tan",
        hex: "99522B",
        rgb: &[153, 82, 43],
        hsl: &[15, 143, 97],
        shade: "Orange"
      },
      Colour {
        name: "Hawkes Blue",
        hex: "D2DAED",
        rgb: &[210, 218, 237],
        hsl: &[157, 109, 223],
        shade: "Blue"
      },
      Colour {
        name: "Heath",
        hex: "4F2A2C",
        rgb: &[79, 42, 44],
        hsl: &[-2, 77, 60],
        shade: "Red"
      },
      Colour {
        name: "Heather",
        hex: "AEBBC1",
        rgb: &[174, 187, 193],
        hsl: &[140, 33, 183],
        shade: "Blue"
      },
      Colour {
        name: "Heathered Grey",
        hex: "948C7E",
        rgb: &[148, 140, 126],
        hsl: &[27, 23, 137],
        shade: "Brown"
      },
      Colour {
        name: "Heavy Metal",
        hex: "46473E",
        rgb: &[70, 71, 62],
        hsl: &[47, 17, 66],
        shade: "Grey"
      },
      Colour {
        name: "Heliotrope",
        hex: "DF73FF",
        rgb: &[223, 115, 255],
        hsl: &[202, 255, 185],
        shade: "Violet"
      },
      Colour {
        name: "Hemlock",
        hex: "69684B",
        rgb: &[105, 104, 75],
        hsl: &[41, 42, 89],
        shade: "Yellow"
      },
      Colour {
        name: "Hemp",
        hex: "987D73",
        rgb: &[152, 125, 115],
        hsl: &[11, 38, 133],
        shade: "Brown"
      },
      Colour {
        name: "Highball",
        hex: "928C3C",
        rgb: &[146, 140, 60],
        hsl: &[39, 106, 103],
        shade: "Green"
      },
      Colour {
        name: "Highland",
        hex: "7A9461",
        rgb: &[122, 148, 97],
        hsl: &[64, 53, 122],
        shade: "Green"
      },
      Colour {
        name: "Hillary",
        hex: "A7A07E",
        rgb: &[167, 160, 126],
        hsl: &[35, 48, 146],
        shade: "Green"
      },
      Colour {
        name: "Himalaya",
        hex: "736330",
        rgb: &[115, 99, 48],
        hsl: &[32, 104, 81],
        shade: "Yellow"
      },
      Colour {
        name: "Hint Of Green",
        hex: "DFF1D6",
        rgb: &[223, 241, 214],
        hsl: &[70, 125, 227],
        shade: "Green"
      },
      Colour {
        name: "Hint Of Red",
        hex: "F5EFEB",
        rgb: &[245, 239, 235],
        hsl: &[17, 85, 240],
        shade: "Grey"
      },
      Colour {
        name: "Hint Of Yellow",
        hex: "F6F5D7",
        rgb: &[246, 245, 215],
        hsl: &[41, 161, 230],
        shade: "Green"
      },
      Colour {
        name: "Hippie Blue",
        hex: "49889A",
        rgb: &[73, 136, 154],
        hsl: &[136, 90, 113],
        shade: "Blue"
      },
      Colour {
        name: "Hippie Green",
        hex: "608A5A",
        rgb: &[96, 138, 90],
        hsl: &[79, 53, 113],
        shade: "Green"
      },
      Colour {
        name: "Hippie Pink",
        hex: "AB495C",
        rgb: &[171, 73, 92],
        hsl: &[-8, 102, 121],
        shade: "Red"
      },
      Colour {
        name: "Hit Grey",
        hex: "A1A9A8",
        rgb: &[161, 169, 168],
        hsl: &[122, 11, 164],
        shade: "Grey"
      },
      Colour {
        name: "Hit Pink",
        hex: "FDA470",
        rgb: &[253, 164, 112],
        hsl: &[15, 247, 182],
        shade: "Orange"
      },
      Colour {
        name: "Hokey Pokey",
        hex: "BB8E34",
        rgb: &[187, 142, 52],
        hsl: &[28, 144, 119],
        shade: "Yellow"
      },
      Colour {
        name: "Hoki",
        hex: "647D86",
        rgb: &[100, 125, 134],
        hsl: &[138, 37, 117],
        shade: "Blue"
      },
      Colour {
        name: "Holly",
        hex: "25342B",
        rgb: &[37, 52, 43],
        hsl: &[101, 42, 44],
        shade: "Green"
      },
      Colour {
        name: "Hollywood Cerise",
        hex: "F400A1",
        rgb: &[244, 0, 161],
        hsl: &[-28, 255, 122],
        shade: "Red"
      },
      Colour {
        name: "Honey Flower",
        hex: "5C3C6D",
        rgb: &[92, 60, 109],
        hsl: &[197, 73, 84],
        shade: "Violet"
      },
      Colour {
        name: "Honeydew",
        hex: "F0FFF0",
        rgb: &[240, 255, 240],
        hsl: &[85, 255, 247],
        shade: "White"
      },
      Colour {
        name: "Honeysuckle",
        hex: "E8ED69",
        rgb: &[232, 237, 105],
        hsl: &[44, 200, 171],
        shade: "Green"
      },
      Colour {
        name: "Hopbush",
        hex: "CD6D93",
        rgb: &[205, 109, 147],
        hsl: &[-16, 124, 157],
        shade: "Violet"
      },
      Colour {
        name: "Horizon",
        hex: "648894",
        rgb: &[100, 136, 148],
        hsl: &[138, 49, 124],
        shade: "Blue"
      },
      Colour {
        name: "Horses Neck",
        hex: "6D562C",
        rgb: &[109, 86, 44],
        hsl: &[27, 108, 76],
        shade: "Yellow"
      },
      Colour {
        name: "Hot Curry",
        hex: "815B28",
        rgb: &[129, 91, 40],
        hsl: &[24, 134, 84],
        shade: "Yellow"
      },
      Colour {
        name: "Hot Magenta",
        hex: "FF00CC",
        rgb: &[255, 0, 204],
        hsl: &[-34, 255, 127],
        shade: "Red"
      },
      Colour {
        name: "Hot Pink",
        hex: "FF69B4",
        rgb: &[255, 105, 180],
        hsl: &[-21, 254, 179],
        shade: "Red"
      },
      Colour {
        name: "Hot Purple",
        hex: "4E2E53",
        rgb: &[78, 46, 83],
        hsl: &[206, 73, 64],
        shade: "Violet"
      },
      Colour {
        name: "Hot Toddy",
        hex: "A7752C",
        rgb: &[167, 117, 44],
        hsl: &[25, 148, 105],
        shade: "Yellow"
      },
      Colour {
        name: "Humming Bird",
        hex: "CEEFE4",
        rgb: &[206, 239, 228],
        hsl: &[113, 129, 222],
        shade: "Green"
      },
      Colour {
        name: "Hunter Green",
        hex: "355E3B",
        rgb: &[53, 94, 59],
        hsl: &[91, 71, 73],
        shade: "Green"
      },
      Colour {
        name: "Hurricane",
        hex: "8B7E77",
        rgb: &[139, 126, 119],
        hsl: &[14, 20, 129],
        shade: "Brown"
      },
      Colour {
        name: "Husk",
        hex: "B2994B",
        rgb: &[178, 153, 75],
        hsl: &[32, 103, 126],
        shade: "Yellow"
      },
      Colour {
        name: "Ice Cold",
        hex: "AFE3D6",
        rgb: &[175, 227, 214],
        hsl: &[116, 122, 201],
        shade: "Green"
      },
      Colour {
        name: "Iceberg",
        hex: "CAE1D9",
        rgb: &[202, 225, 217],
        hsl: &[112, 70, 213],
        shade: "Green"
      },
      Colour {
        name: "Illusion",
        hex: "EF95AE",
        rgb: &[239, 149, 174],
        hsl: &[-11, 188, 194],
        shade: "Red"
      },
      Colour {
        name: "Inch Worm",
        hex: "B0E313",
        rgb: &[176, 227, 19],
        hsl: &[52, 215, 122],
        shade: "Green"
      },
      Colour {
        name: "Indian Red",
        hex: "CD5C5C",
        rgb: &[205, 92, 92],
        hsl: &[0, 135, 148],
        shade: "Red"
      },
      Colour {
        name: "Indian Tan",
        hex: "4F301F",
        rgb: &[79, 48, 31],
        hsl: &[15, 111, 55],
        shade: "Brown"
      },
      Colour {
        name: "Indigo",
        hex: "4B0082",
        rgb: &[75, 0, 130],
        hsl: &[194, 255, 65],
        shade: "Violet"
      },
      Colour {
        name: "Indochine",
        hex: "9C5B34",
        rgb: &[156, 91, 52],
        hsl: &[15, 127, 104],
        shade: "Orange"
      },
      Colour {
        name: "International Klein Blue",
        hex: "002FA7",
        rgb: &[0, 47, 167],
        hsl: &[158, 255, 83],
        shade: "Blue"
      },
      Colour {
        name: "International Orange",
        hex: "FF4F00",
        rgb: &[255, 79, 0],
        hsl: &[13, 255, 127],
        shade: "Orange"
      },
      Colour {
        name: "Iris Blue",
        hex: "03B4C8",
        rgb: &[3, 180, 200],
        hsl: &[131, 247, 101],
        shade: "Blue"
      },
      Colour {
        name: "Irish Coffee",
        hex: "62422B",
        rgb: &[98, 66, 43],
        hsl: &[17, 99, 70],
        shade: "Brown"
      },
      Colour {
        name: "Iron",
        hex: "CBCDCD",
        rgb: &[203, 205, 205],
        hsl: &[127, 5, 204],
        shade: "Grey"
      },
      Colour {
        name: "Ironside Grey",
        hex: "706E66",
        rgb: &[112, 110, 102],
        hsl: &[34, 11, 107],
        shade: "Grey"
      },
      Colour {
        name: "Ironstone",
        hex: "865040",
        rgb: &[134, 80, 64],
        hsl: &[9, 90, 99],
        shade: "Brown"
      },
      Colour {
        name: "Islamic Green",
        hex: "009900",
        rgb: &[0, 153, 0],
        hsl: &[85, 255, 76],
        shade: "Green"
      },
      Colour {
        name: "Island Spice",
        hex: "F8EDDB",
        rgb: &[248, 237, 219],
        hsl: &[26, 171, 233],
        shade: "Yellow"
      },
      Colour {
        name: "Ivory",
        hex: "FFFFF0",
        rgb: &[255, 255, 240],
        hsl: &[42, 255, 247],
        shade: "White"
      },
      Colour {
        name: "Jacarta",
        hex: "3D325D",
        rgb: &[61, 50, 93],
        hsl: &[180, 76, 71],
        shade: "Violet"
      },
      Colour {
        name: "Jacko Bean",
        hex: "413628",
        rgb: &[65, 54, 40],
        hsl: &[23, 60, 52],
        shade: "Brown"
      },
      Colour {
        name: "Jacksons Purple",
        hex: "3D3F7D",
        rgb: &[61, 63, 125],
        hsl: &[168, 87, 93],
        shade: "Violet"
      },
      Colour {
        name: "Jade",
        hex: "00A86B",
        rgb: &[0, 168, 107],
        hsl: &[112, 255, 84],
        shade: "Green"
      },
      Colour {
        name: "Jaffa",
        hex: "E27945",
        rgb: &[226, 121, 69],
        hsl: &[14, 186, 147],
        shade: "Orange"
      },
      Colour {
        name: "Jagged Ice",
        hex: "CAE7E2",
        rgb: &[202, 231, 226],
        hsl: &[120, 96, 216],
        shade: "Green"
      },
      Colour {
        name: "Jagger",
        hex: "3F2E4C",
        rgb: &[63, 46, 76],
        hsl: &[194, 62, 60],
        shade: "Violet"
      },
      Colour {
        name: "Jaguar",
        hex: "29292F",
        rgb: &[41, 41, 47],
        hsl: &[170, 17, 44],
        shade: "Blue"
      },
      Colour {
        name: "Jambalaya",
        hex: "674834",
        rgb: &[103, 72, 52],
        hsl: &[16, 83, 77],
        shade: "Brown"
      },
      Colour {
        name: "Japanese Laurel",
        hex: "2F7532",
        rgb: &[47, 117, 50],
        hsl: &[86, 108, 81],
        shade: "Green"
      },
      Colour {
        name: "Japonica",
        hex: "CE7259",
        rgb: &[206, 114, 89],
        hsl: &[9, 138, 147],
        shade: "Orange"
      },
      Colour {
        name: "Java",
        hex: "259797",
        rgb: &[37, 151, 151],
        hsl: &[127, 154, 94],
        shade: "Green"
      },
      Colour {
        name: "Jazz",
        hex: "5F2C2F",
        rgb: &[95, 44, 47],
        hsl: &[-2, 93, 69],
        shade: "Red"
      },
      Colour {
        name: "Jazzberry Jam",
        hex: "A50B5E",
        rgb: &[165, 11, 94],
        hsl: &[-22, 223, 88],
        shade: "Red"
      },
      Colour {
        name: "Jelly Bean",
        hex: "44798E",
        rgb: &[68, 121, 142],
        hsl: &[139, 89, 105],
        shade: "Blue"
      },
      Colour {
        name: "Jet Stream",
        hex: "BBD0C9",
        rgb: &[187, 208, 201],
        hsl: &[113, 46, 197],
        shade: "Green"
      },
      Colour {
        name: "Jewel",
        hex: "136843",
        rgb: &[19, 104, 67],
        hsl: &[109, 176, 61],
        shade: "Green"
      },
      Colour {
        name: "Jon",
        hex: "463D3E",
        rgb: &[70, 61, 62],
        hsl: &[-4, 17, 65],
        shade: "Grey"
      },
      Colour {
        name: "Jonquil",
        hex: "EEF293",
        rgb: &[238, 242, 147],
        hsl: &[44, 200, 194],
        shade: "Green"
      },
      Colour {
        name: "Jordy Blue",
        hex: "7AAAE0",
        rgb: &[122, 170, 224],
        hsl: &[150, 158, 173],
        shade: "Blue"
      },
      Colour {
        name: "Judge Grey",
        hex: "5D5346",
        rgb: &[93, 83, 70],
        hsl: &[24, 35, 81],
        shade: "Brown"
      },
      Colour {
        name: "Jumbo",
        hex: "878785",
        rgb: &[135, 135, 133],
        hsl: &[42, 2, 134],
        shade: "Grey"
      },
      Colour {
        name: "Jungle Green",
        hex: "29AB87",
        rgb: &[41, 171, 135],
        hsl: &[115, 156, 105],
        shade: "Green"
      },
      Colour {
        name: "Jungle Mist",
        hex: "B0C4C4",
        rgb: &[176, 196, 196],
        hsl: &[127, 36, 186],
        shade: "Green"
      },
      Colour {
        name: "Juniper",
        hex: "74918E",
        rgb: &[116, 145, 142],
        hsl: &[123, 29, 130],
        shade: "Green"
      },
      Colour {
        name: "Just Right",
        hex: "DCBFAC",
        rgb: &[220, 191, 172],
        hsl: &[16, 103, 196],
        shade: "Orange"
      },
      Colour {
        name: "Kabul",
        hex: "6C5E53",
        rgb: &[108, 94, 83],
        hsl: &[18, 33, 95],
        shade: "Brown"
      },
      Colour {
        name: "Kaitoke Green",
        hex: "245336",
        rgb: &[36, 83, 54],
        hsl: &[101, 100, 59],
        shade: "Green"
      },
      Colour {
        name: "Kangaroo",
        hex: "C5C3B0",
        rgb: &[197, 195, 176],
        hsl: &[38, 39, 186],
        shade: "Green"
      },
      Colour {
        name: "Karaka",
        hex: "2D2D24",
        rgb: &[45, 45, 36],
        hsl: &[42, 28, 40],
        shade: "Green"
      },
      Colour {
        name: "Karry",
        hex: "FEDCC1",
        rgb: &[254, 220, 193],
        hsl: &[18, 246, 223],
        shade: "Orange"
      },
      Colour {
        name: "Kashmir Blue",
        hex: "576D8E",
        rgb: &[87, 109, 142],
        hsl: &[153, 61, 114],
        shade: "Blue"
      },
      Colour {
        name: "Kelly Green",
        hex: "4CBB17",
        rgb: &[76, 187, 23],
        hsl: &[71, 199, 105],
        shade: "Green"
      },
      Colour {
        name: "Kelp",
        hex: "4D503C",
        rgb: &[77, 80, 60],
        hsl: &[48, 36, 70],
        shade: "Green"
      },
      Colour {
        name: "Kenyan Copper",
        hex: "6C322E",
        rgb: &[108, 50, 46],
        hsl: &[2, 102, 77],
        shade: "Red"
      },
      Colour {
        name: "Keppel",
        hex: "5FB69C",
        rgb: &[95, 182, 156],
        hsl: &[114, 95, 138],
        shade: "Green"
      },
      Colour {
        name: "Khaki",
        hex: "F0E68C",
        rgb: &[240, 230, 140],
        hsl: &[38, 196, 190],
        shade: "Yellow"
      },
      Colour {
        name: "Kidnapper",
        hex: "BFC0AB",
        rgb: &[191, 192, 171],
        hsl: &[44, 36, 181],
        shade: "Green"
      },
      Colour {
        name: "Kilamanjaro",
        hex: "3A3532",
        rgb: &[58, 53, 50],
        hsl: &[15, 18, 54],
        shade: "Grey"
      },
      Colour {
        name: "Killarney",
        hex: "49764F",
        rgb: &[73, 118, 79],
        hsl: &[90, 60, 95],
        shade: "Green"
      },
      Colour {
        name: "Kimberly",
        hex: "695D87",
        rgb: &[105, 93, 135],
        hsl: &[182, 46, 113],
        shade: "Violet"
      },
      Colour {
        name: "Kingfisher Daisy",
        hex: "583580",
        rgb: &[88, 53, 128],
        hsl: &[189, 105, 90],
        shade: "Violet"
      },
      Colour {
        name: "Kobi",
        hex: "E093AB",
        rgb: &[224, 147, 171],
        hsl: &[-13, 141, 185],
        shade: "Red"
      },
      Colour {
        name: "Kokoda",
        hex: "7B785A",
        rgb: &[123, 120, 90],
        hsl: &[38, 39, 106],
        shade: "Green"
      },
      Colour {
        name: "Korma",
        hex: "804E2C",
        rgb: &[128, 78, 44],
        hsl: &[17, 124, 86],
        shade: "Orange"
      },
      Colour {
        name: "Koromiko",
        hex: "FEB552",
        rgb: &[254, 181, 82],
        hsl: &[24, 252, 168],
        shade: "Yellow"
      },
      Colour {
        name: "Kournikova",
        hex: "F9D054",
        rgb: &[249, 208, 84],
        hsl: &[31, 237, 166],
        shade: "Yellow"
      },
      Colour {
        name: "La Palma",
        hex: "428929",
        rgb: &[66, 137, 41],
        hsl: &[73, 137, 89],
        shade: "Green"
      },
      Colour {
        name: "La Rioja",
        hex: "BAC00E",
        rgb: &[186, 192, 14],
        hsl: &[43, 220, 103],
        shade: "Green"
      },
      Colour {
        name: "Las Palmas",
        hex: "C6DA36",
        rgb: &[198, 218, 54],
        hsl: &[47, 175, 136],
        shade: "Green"
      },
      Colour {
        name: "Laser",
        hex: "C6A95E",
        rgb: &[198, 169, 94],
        hsl: &[30, 121, 146],
        shade: "Yellow"
      },
      Colour {
        name: "Laser Lemon",
        hex: "FFFF66",
        rgb: &[255, 255, 102],
        hsl: &[42, 254, 178],
        shade: "Yellow"
      },
      Colour {
        name: "Laurel",
        hex: "6E8D71",
        rgb: &[110, 141, 113],
        hsl: &[89, 31, 125],
        shade: "Green"
      },
      Colour {
        name: "Lavender",
        hex: "E6E6FA",
        rgb: &[230, 230, 250],
        hsl: &[170, 169, 240],
        shade: "Violet"
      },
      Colour {
        name: "Lavender Blue",
        hex: "CCCCFF",
        rgb: &[204, 204, 255],
        hsl: &[170, 255, 229],
        shade: "Blue"
      },
      Colour {
        name: "Lavender Blush",
        hex: "FFF0F5",
        rgb: &[255, 240, 245],
        hsl: &[-14, 255, 247],
        shade: "Violet"
      },
      Colour {
        name: "Lavender Grey",
        hex: "BDBBD7",
        rgb: &[189, 187, 215],
        hsl: &[173, 66, 201],
        shade: "Grey"
      },
      Colour {
        name: "Lavender Pink",
        hex: "FBAED2",
        rgb: &[251, 174, 210],
        hsl: &[-19, 230, 212],
        shade: "Red"
      },
      Colour {
        name: "Lavender Rose",
        hex: "FBA0E3",
        rgb: &[251, 160, 227],
        hsl: &[-31, 234, 205],
        shade: "Red"
      },
      Colour {
        name: "Lawn Green",
        hex: "7CFC00",
        rgb: &[124, 252, 0],
        hsl: &[64, 255, 126],
        shade: "Green"
      },
      Colour {
        name: "Leather",
        hex: "906A54",
        rgb: &[144, 106, 84],
        hsl: &[15, 67, 113],
        shade: "Brown"
      },
      Colour {
        name: "Lemon",
        hex: "FDE910",
        rgb: &[253, 233, 16],
        hsl: &[38, 250, 134],
        shade: "Yellow"
      },
      Colour {
        name: "Lemon Chiffon",
        hex: "FFFACD",
        rgb: &[255, 250, 205],
        hsl: &[38, 255, 230],
        shade: "Yellow"
      },
      Colour {
        name: "Lemon Ginger",
        hex: "968428",
        rgb: &[150, 132, 40],
        hsl: &[35, 147, 95],
        shade: "Yellow"
      },
      Colour {
        name: "Lemon Grass",
        hex: "999A86",
        rgb: &[153, 154, 134],
        hsl: &[44, 22, 144],
        shade: "Green"
      },
      Colour {
        name: "Licorice",
        hex: "2E3749",
        rgb: &[46, 55, 73],
        hsl: &[155, 57, 59],
        shade: "Blue"
      },
      Colour {
        name: "Light Blue",
        hex: "ADD8E6",
        rgb: &[173, 216, 230],
        hsl: &[137, 135, 201],
        shade: "Blue"
      },
      Colour {
        name: "Light Coral",
        hex: "F08080",
        rgb: &[240, 128, 128],
        hsl: &[0, 201, 184],
        shade: "Orange"
      },
      Colour {
        name: "Light Cyan",
        hex: "E0FFFF",
        rgb: &[224, 255, 255],
        hsl: &[127, 255, 239],
        shade: "Blue"
      },
      Colour {
        name: "Light Goldenrod",
        hex: "EEDD82",
        rgb: &[238, 221, 130],
        hsl: &[35, 193, 184],
        shade: "Yellow"
      },
      Colour {
        name: "Light Goldenrod Yellow",
        hex: "FAFAD2",
        rgb: &[250, 250, 210],
        hsl: &[42, 203, 229],
        shade: "Yellow"
      },
      Colour {
        name: "Light Green",
        hex: "90EE90",
        rgb: &[144, 238, 144],
        hsl: &[85, 187, 191],
        shade: "Green"
      },
      Colour {
        name: "Light Grey",
        hex: "D3D3D3",
        rgb: &[211, 211, 211],
        hsl: &[0, 0, 211],
        shade: "Grey"
      },
      Colour {
        name: "Light Pink",
        hex: "FFB6C1",
        rgb: &[255, 182, 193],
        hsl: &[-6, 255, 218],
        shade: "Red"
      },
      Colour {
        name: "Light Salmon",
        hex: "FFA07A",
        rgb: &[255, 160, 122],
        hsl: &[12, 255, 188],
        shade: "Orange"
      },
      Colour {
        name: "Light Sea Green",
        hex: "20B2AA",
        rgb: &[32, 178, 170],
        hsl: &[125, 177, 105],
        shade: "Green"
      },
      Colour {
        name: "Light Sky Blue",
        hex: "87CEFA",
        rgb: &[135, 206, 250],
        hsl: &[143, 234, 192],
        shade: "Blue"
      },
      Colour {
        name: "Light Slate Blue",
        hex: "8470FF",
        rgb: &[132, 112, 255],
        hsl: &[175, 255, 183],
        shade: "Blue"
      },
      Colour {
        name: "Light Slate Grey",
        hex: "778899",
        rgb: &[119, 136, 153],
        hsl: &[148, 36, 136],
        shade: "Grey"
      },
      Colour {
        name: "Light Steel Blue",
        hex: "B0C4DE",
        rgb: &[176, 196, 222],
        hsl: &[151, 104, 199],
        shade: "Blue"
      },
      Colour {
        name: "Light Wood",
        hex: "856363",
        rgb: &[133, 99, 99],
        hsl: &[0, 37, 116],
        shade: "Brown"
      },
      Colour {
        name: "Light Yellow",
        hex: "FFFFE0",
        rgb: &[255, 255, 224],
        hsl: &[42, 255, 239],
        shade: "Yellow"
      },
      Colour {
        name: "Lightning Yellow",
        hex: "F7A233",
        rgb: &[247, 162, 51],
        hsl: &[24, 235, 149],
        shade: "Yellow"
      },
      Colour {
        name: "Lilac",
        hex: "C8A2C8",
        rgb: &[200, 162, 200],
        hsl: &[-42, 65, 180],
        shade: "Violet"
      },
      Colour {
        name: "Lilac Bush",
        hex: "9470C4",
        rgb: &[148, 112, 196],
        hsl: &[188, 106, 154],
        shade: "Violet"
      },
      Colour {
        name: "Lily",
        hex: "C19FB3",
        rgb: &[193, 159, 179],
        hsl: &[-24, 54, 176],
        shade: "Violet"
      },
      Colour {
        name: "Lily White",
        hex: "E9EEEB",
        rgb: &[233, 238, 235],
        hsl: &[101, 32, 235],
        shade: "Grey"
      },
      Colour {
        name: "Lima",
        hex: "7AAC21",
        rgb: &[122, 172, 33],
        hsl: &[57, 172, 102],
        shade: "Green"
      },
      Colour {
        name: "Lime",
        hex: "00FF00",
        rgb: &[0, 255, 0],
        hsl: &[85, 255, 127],
        shade: "Green"
      },
      Colour {
        name: "Lime Green",
        hex: "32CD32",
        rgb: &[50, 205, 50],
        hsl: &[85, 155, 127],
        shade: "Green"
      },
      Colour {
        name: "Limeade",
        hex: "5F9727",
        rgb: &[95, 151, 39],
        hsl: &[63, 150, 95],
        shade: "Green"
      },
      Colour {
        name: "Limerick",
        hex: "89AC27",
        rgb: &[137, 172, 39],
        hsl: &[53, 160, 105],
        shade: "Green"
      },
      Colour {
        name: "Linen",
        hex: "FAF0E6",
        rgb: &[250, 240, 230],
        hsl: &[21, 169, 240],
        shade: "White"
      },
      Colour {
        name: "Link Water",
        hex: "C7CDD8",
        rgb: &[199, 205, 216],
        hsl: &[155, 45, 207],
        shade: "Blue"
      },
      Colour {
        name: "Lipstick",
        hex: "962C54",
        rgb: &[150, 44, 84],
        hsl: &[-16, 139, 97],
        shade: "Red"
      },
      Colour {
        name: "Liver",
        hex: "534B4F",
        rgb: &[83, 75, 79],
        hsl: &[-21, 12, 79],
        shade: "Brown"
      },
      Colour {
        name: "Livid Brown",
        hex: "312A29",
        rgb: &[49, 42, 41],
        hsl: &[5, 22, 45],
        shade: "Brown"
      },
      Colour {
        name: "Loafer",
        hex: "DBD9C2",
        rgb: &[219, 217, 194],
        hsl: &[39, 65, 206],
        shade: "Green"
      },
      Colour {
        name: "Loblolly",
        hex: "B3BBB7",
        rgb: &[179, 187, 183],
        hsl: &[106, 14, 182],
        shade: "Green"
      },
      Colour {
        name: "Lochinvar",
        hex: "489084",
        rgb: &[72, 144, 132],
        hsl: &[120, 85, 108],
        shade: "Green"
      },
      Colour {
        name: "Lochmara",
        hex: "316EA0",
        rgb: &[49, 110, 160],
        hsl: &[146, 135, 104],
        shade: "Blue"
      },
      Colour {
        name: "Locust",
        hex: "A2A580",
        rgb: &[162, 165, 128],
        hsl: &[45, 43, 146],
        shade: "Green"
      },
      Colour {
        name: "Log Cabin",
        hex: "393E2E",
        rgb: &[57, 62, 46],
        hsl: &[55, 37, 54],
        shade: "Green"
      },
      Colour {
        name: "Logan",
        hex: "9D9CB4",
        rgb: &[157, 156, 180],
        hsl: &[171, 35, 168],
        shade: "Blue"
      },
      Colour {
        name: "Lola",
        hex: "B9ACBB",
        rgb: &[185, 172, 187],
        hsl: &[206, 25, 179],
        shade: "Violet"
      },
      Colour {
        name: "London Hue",
        hex: "AE94AB",
        rgb: &[174, 148, 171],
        hsl: &[-37, 35, 161],
        shade: "Violet"
      },
      Colour {
        name: "Lonestar",
        hex: "522426",
        rgb: &[82, 36, 38],
        hsl: &[-1, 99, 59],
        shade: "Red"
      },
      Colour {
        name: "Lotus",
        hex: "8B504B",
        rgb: &[139, 80, 75],
        hsl: &[3, 76, 106],
        shade: "Brown"
      },
      Colour {
        name: "Loulou",
        hex: "4C3347",
        rgb: &[76, 51, 71],
        hsl: &[-34, 50, 63],
        shade: "Violet"
      },
      Colour {
        name: "Lucky",
        hex: "AB9A1C",
        rgb: &[171, 154, 28],
        hsl: &[37, 183, 99],
        shade: "Green"
      },
      Colour {
        name: "Lucky Point",
        hex: "292D4F",
        rgb: &[41, 45, 79],
        hsl: &[165, 80, 60],
        shade: "Blue"
      },
      Colour {
        name: "Lunar Green",
        hex: "4E5541",
        rgb: &[78, 85, 65],
        hsl: &[57, 34, 74],
        shade: "Green"
      },
      Colour {
        name: "Lusty",
        hex: "782E2C",
        rgb: &[120, 46, 44],
        hsl: &[1, 118, 81],
        shade: "Red"
      },
      Colour {
        name: "Luxor Gold",
        hex: "AB8D3F",
        rgb: &[171, 141, 63],
        hsl: &[30, 117, 117],
        shade: "Yellow"
      },
      Colour {
        name: "Lynch",
        hex: "697D89",
        rgb: &[105, 125, 137],
        hsl: &[143, 33, 121],
        shade: "Blue"
      },
      Colour {
        name: "Mabel",
        hex: "CBE8E8",
        rgb: &[203, 232, 232],
        hsl: &[127, 98, 217],
        shade: "Blue"
      },
      Colour {
        name: "Macaroni And Cheese",
        hex: "FFB97B",
        rgb: &[255, 185, 123],
        hsl: &[19, 255, 189],
        shade: "Orange"
      },
      Colour {
        name: "Madang",
        hex: "B7E3A8",
        rgb: &[183, 227, 168],
        hsl: &[74, 130, 197],
        shade: "Green"
      },
      Colour {
        name: "Madison",
        hex: "2D3C54",
        rgb: &[45, 60, 84],
        hsl: &[153, 77, 64],
        shade: "Blue"
      },
      Colour {
        name: "Madras",
        hex: "473E23",
        rgb: &[71, 62, 35],
        hsl: &[31, 86, 53],
        shade: "Brown"
      },
      Colour {
        name: "Magenta",
        hex: "FF00FF",
        rgb: &[255, 0, 255],
        hsl: &[-42, 255, 127],
        shade: "Violet"
      },
      Colour {
        name: "Magic Mint",
        hex: "AAF0D1",
        rgb: &[170, 240, 209],
        hsl: &[108, 178, 205],
        shade: "Green"
      },
      Colour {
        name: "Magnolia",
        hex: "F8F4FF",
        rgb: &[248, 244, 255],
        hsl: &[185, 255, 249],
        shade: "White"
      },
      Colour {
        name: "Mahogany",
        hex: "CA3435",
        rgb: &[202, 52, 53],
        hsl: &[0, 150, 127],
        shade: "Brown"
      },
      Colour {
        name: "Mai Tai",
        hex: "A56531",
        rgb: &[165, 101, 49],
        hsl: &[19, 138, 107],
        shade: "Orange"
      },
      Colour {
        name: "Maire",
        hex: "2A2922",
        rgb: &[42, 41, 34],
        hsl: &[37, 26, 38],
        shade: "Yellow"
      },
      Colour {
        name: "Maize",
        hex: "E3B982",
        rgb: &[227, 185, 130],
        hsl: &[24, 161, 178],
        shade: "Yellow"
      },
      Colour {
        name: "Makara",
        hex: "695F50",
        rgb: &[105, 95, 80],
        hsl: &[25, 34, 92],
        shade: "Brown"
      },
      Colour {
        name: "Mako",
        hex: "505555",
        rgb: &[80, 85, 85],
        hsl: &[127, 7, 82],
        shade: "Grey"
      },
      Colour {
        name: "Malachite",
        hex: "0BDA51",
        rgb: &[11, 218, 81],
        hsl: &[99, 230, 114],
        shade: "Green"
      },
      Colour {
        name: "Malachite Green",
        hex: "97976F",
        rgb: &[151, 151, 111],
        hsl: &[42, 41, 131],
        shade: "Green"
      },
      Colour {
        name: "Malibu",
        hex: "66B7E1",
        rgb: &[102, 183, 225],
        hsl: &[142, 171, 163],
        shade: "Blue"
      },
      Colour {
        name: "Mallard",
        hex: "3A4531",
        rgb: &[58, 69, 49],
        hsl: &[65, 43, 59],
        shade: "Green"
      },
      Colour {
        name: "Malta",
        hex: "A59784",
        rgb: &[165, 151, 132],
        hsl: &[24, 39, 148],
        shade: "Brown"
      },
      Colour {
        name: "Mamba",
        hex: "766D7C",
        rgb: &[118, 109, 124],
        hsl: &[195, 16, 116],
        shade: "Violet"
      },
      Colour {
        name: "Manatee",
        hex: "8D90A1",
        rgb: &[141, 144, 161],
        hsl: &[163, 24, 151],
        shade: "Blue"
      },
      Colour {
        name: "Mandalay",
        hex: "B57B2E",
        rgb: &[181, 123, 46],
        hsl: &[24, 151, 113],
        shade: "Yellow"
      },
      Colour {
        name: "Mandarian Orange",
        hex: "8E2323",
        rgb: &[142, 35, 35],
        hsl: &[0, 154, 88],
        shade: "Orange"
      },
      Colour {
        name: "Mandy",
        hex: "CD525B",
        rgb: &[205, 82, 91],
        hsl: &[-3, 140, 143],
        shade: "Red"
      },
      Colour {
        name: "Mandys Pink",
        hex: "F5B799",
        rgb: &[245, 183, 153],
        hsl: &[13, 209, 199],
        shade: "Orange"
      },
      Colour {
        name: "Mango Tango",
        hex: "E77200",
        rgb: &[231, 114, 0],
        hsl: &[20, 255, 115],
        shade: "Orange"
      },
      Colour {
        name: "Manhattan",
        hex: "E2AF80",
        rgb: &[226, 175, 128],
        hsl: &[20, 160, 177],
        shade: "Orange"
      },
      Colour {
        name: "Mantis",
        hex: "7FC15C",
        rgb: &[127, 193, 92],
        hsl: &[70, 114, 142],
        shade: "Green"
      },
      Colour {
        name: "Mantle",
        hex: "96A793",
        rgb: &[150, 167, 147],
        hsl: &[78, 26, 157],
        shade: "Green"
      },
      Colour {
        name: "Manz",
        hex: "E4DB55",
        rgb: &[228, 219, 85],
        hsl: &[39, 185, 156],
        shade: "Green"
      },
      Colour {
        name: "Mardi Gras",
        hex: "352235",
        rgb: &[53, 34, 53],
        hsl: &[-42, 55, 43],
        shade: "Violet"
      },
      Colour {
        name: "Marigold",
        hex: "B88A3D",
        rgb: &[184, 138, 61],
        hsl: &[26, 128, 122],
        shade: "Yellow"
      },
      Colour {
        name: "Mariner",
        hex: "42639F",
        rgb: &[66, 99, 159],
        hsl: &[154, 105, 112],
        shade: "Blue"
      },
      Colour {
        name: "Maroon",
        hex: "800000",
        rgb: &[128, 0, 0],
        hsl: &[0, 255, 64],
        shade: "Brown"
      },
      Colour {
        name: "Marshland",
        hex: "2B2E26",
        rgb: &[43, 46, 38],
        hsl: &[58, 24, 42],
        shade: "Green"
      },
      Colour {
        name: "Martini",
        hex: "B7A8A3",
        rgb: &[183, 168, 163],
        hsl: &[10, 31, 173],
        shade: "Brown"
      },
      Colour {
        name: "Martinique",
        hex: "3C3748",
        rgb: &[60, 55, 72],
        hsl: &[182, 34, 63],
        shade: "Violet"
      },
      Colour {
        name: "Marzipan",
        hex: "EBC881",
        rgb: &[235, 200, 129],
        hsl: &[28, 185, 181],
        shade: "Yellow"
      },
      Colour {
        name: "Masala",
        hex: "57534B",
        rgb: &[87, 83, 75],
        hsl: &[28, 18, 81],
        shade: "Brown"
      },
      Colour {
        name: "Matisse",
        hex: "365C7D",
        rgb: &[54, 92, 125],
        hsl: &[147, 101, 89],
        shade: "Blue"
      },
      Colour {
        name: "Matrix",
        hex: "8E4D45",
        rgb: &[142, 77, 69],
        hsl: &[4, 88, 105],
        shade: "Red"
      },
      Colour {
        name: "Matterhorn",
        hex: "524B4B",
        rgb: &[82, 75, 75],
        hsl: &[0, 11, 78],
        shade: "Grey"
      },
      Colour {
        name: "Mauve",
        hex: "E0B0FF",
        rgb: &[224, 176, 255],
        hsl: &[195, 255, 215],
        shade: "Violet"
      },
      Colour {
        name: "Mauve Taupe",
        hex: "915F6D",
        rgb: &[145, 95, 109],
        hsl: &[-11, 53, 120],
        shade: "Red"
      },
      Colour {
        name: "Mauvelous",
        hex: "F091A9",
        rgb: &[240, 145, 169],
        hsl: &[-10, 193, 192],
        shade: "Red"
      },
      Colour {
        name: "Maverick",
        hex: "C8B1C0",
        rgb: &[200, 177, 192],
        hsl: &[-27, 44, 188],
        shade: "Violet"
      },
      Colour {
        name: "Maya Blue",
        hex: "73C2FB",
        rgb: &[115, 194, 251],
        hsl: &[145, 240, 183],
        shade: "Blue"
      },
      Colour {
        name: "McKenzie",
        hex: "8C6338",
        rgb: &[140, 99, 56],
        hsl: &[21, 109, 98],
        shade: "Orange"
      },
      Colour {
        name: "Medium Aquamarine",
        hex: "66CDAA",
        rgb: &[102, 205, 170],
        hsl: &[113, 129, 153],
        shade: "Blue"
      },
      Colour {
        name: "Medium Blue",
        hex: "0000CD",
        rgb: &[0, 0, 205],
        hsl: &[170, 255, 102],
        shade: "Blue"
      },
      Colour {
        name: "Medium Carmine",
        hex: "AF4035",
        rgb: &[175, 64, 53],
        hsl: &[3, 136, 114],
        shade: "Red"
      },
      Colour {
        name: "Medium Goldenrod",
        hex: "EAEAAE",
        rgb: &[234, 234, 174],
        hsl: &[42, 150, 204],
        shade: "Yellow"
      },
      Colour {
        name: "Medium Orchid",
        hex: "BA55D3",
        rgb: &[186, 85, 211],
        hsl: &[204, 150, 147],
        shade: "Violet"
      },
      Colour {
        name: "Medium Purple",
        hex: "9370DB",
        rgb: &[147, 112, 219],
        hsl: &[183, 152, 165],
        shade: "Violet"
      },
      Colour {
        name: "Medium Sea Green",
        hex: "3CB371",
        rgb: &[60, 179, 113],
        hsl: &[103, 126, 119],
        shade: "Green"
      },
      Colour {
        name: "Medium Slate Blue",
        hex: "7B68EE",
        rgb: &[123, 104, 238],
        hsl: &[176, 203, 171],
        shade: "Blue"
      },
      Colour {
        name: "Medium Spring Green",
        hex: "00FA9A",
        rgb: &[0, 250, 154],
        hsl: &[111, 255, 125],
        shade: "Green"
      },
      Colour {
        name: "Medium Turquoise",
        hex: "48D1CC",
        rgb: &[72, 209, 204],
        hsl: &[125, 152, 140],
        shade: "Blue"
      },
      Colour {
        name: "Medium Violet Red",
        hex: "C71585",
        rgb: &[199, 21, 133],
        hsl: &[-26, 206, 110],
        shade: "Red"
      },
      Colour {
        name: "Medium Wood",
        hex: "A68064",
        rgb: &[166, 128, 100],
        hsl: &[18, 68, 133],
        shade: "Brown"
      },
      Colour {
        name: "Melanie",
        hex: "E0B7C2",
        rgb: &[224, 183, 194],
        hsl: &[-11, 101, 203],
        shade: "Red"
      },
      Colour {
        name: "Melanzane",
        hex: "342931",
        rgb: &[52, 41, 49],
        hsl: &[-30, 30, 46],
        shade: "Violet"
      },
      Colour {
        name: "Melon",
        hex: "FEBAAD",
        rgb: &[254, 186, 173],
        hsl: &[6, 248, 213],
        shade: "Red"
      },
      Colour {
        name: "Melrose",
        hex: "C3B9DD",
        rgb: &[195, 185, 221],
        hsl: &[181, 88, 203],
        shade: "Violet"
      },
      Colour {
        name: "Mercury",
        hex: "D5D2D1",
        rgb: &[213, 210, 209],
        hsl: &[10, 11, 211],
        shade: "Grey"
      },
      Colour {
        name: "Merino",
        hex: "E1DBD0",
        rgb: &[225, 219, 208],
        hsl: &[27, 56, 216],
        shade: "Yellow"
      },
      Colour {
        name: "Merlin",
        hex: "4F4E48",
        rgb: &[79, 78, 72],
        hsl: &[36, 11, 75],
        shade: "Grey"
      },
      Colour {
        name: "Merlot",
        hex: "73343A",
        rgb: &[115, 52, 58],
        hsl: &[-4, 96, 83],
        shade: "Red"
      },
      Colour {
        name: "Metallic Bronze",
        hex: "554A3C",
        rgb: &[85, 74, 60],
        hsl: &[23, 43, 72],
        shade: "Red"
      },
      Colour {
        name: "Metallic Copper",
        hex: "6E3D34",
        rgb: &[110, 61, 52],
        hsl: &[6, 91, 81],
        shade: "Red"
      },
      Colour {
        name: "Metallic Gold",
        hex: "D4AF37",
        rgb: &[212, 175, 55],
        hsl: &[32, 164, 133],
        shade: "Yellow"
      },
      Colour {
        name: "Meteor",
        hex: "BB7431",
        rgb: &[187, 116, 49],
        hsl: &[20, 149, 117],
        shade: "Orange"
      },
      Colour {
        name: "Meteorite",
        hex: "4A3B6A",
        rgb: &[74, 59, 106],
        hsl: &[183, 72, 82],
        shade: "Violet"
      },
      Colour {
        name: "Mexican Red",
        hex: "9B3D3D",
        rgb: &[155, 61, 61],
        hsl: &[0, 110, 108],
        shade: "Red"
      },
      Colour {
        name: "Mid Grey",
        hex: "666A6D",
        rgb: &[102, 106, 109],
        hsl: &[145, 8, 105],
        shade: "Grey"
      },
      Colour {
        name: "Midnight",
        hex: "21303E",
        rgb: &[33, 48, 62],
        hsl: &[148, 77, 47],
        shade: "Blue"
      },
      Colour {
        name: "Midnight Blue",
        hex: "191970",
        rgb: &[25, 25, 112],
        hsl: &[170, 161, 68],
        shade: "Blue"
      },
      Colour {
        name: "Midnight Express",
        hex: "21263A",
        rgb: &[33, 38, 58],
        hsl: &[161, 70, 45],
        shade: "Blue"
      },
      Colour {
        name: "Midnight Moss",
        hex: "242E28",
        rgb: &[36, 46, 40],
        hsl: &[101, 31, 40],
        shade: "Green"
      },
      Colour {
        name: "Mikado",
        hex: "3F3623",
        rgb: &[63, 54, 35],
        hsl: &[28, 72, 49],
        shade: "Brown"
      },
      Colour {
        name: "Milan",
        hex: "F6F493",
        rgb: &[246, 244, 147],
        hsl: &[41, 215, 196],
        shade: "Green"
      },
      Colour {
        name: "Milano Red",
        hex: "9E3332",
        rgb: &[158, 51, 50],
        hsl: &[0, 132, 104],
        shade: "Red"
      },
      Colour {
        name: "Milk Punch",
        hex: "F3E5C0",
        rgb: &[243, 229, 192],
        hsl: &[30, 173, 217],
        shade: "Yellow"
      },
      Colour {
        name: "Milk White",
        hex: "DCD9CD",
        rgb: &[220, 217, 205],
        hsl: &[33, 45, 212],
        shade: "Grey"
      },
      Colour {
        name: "Millbrook",
        hex: "595648",
        rgb: &[89, 86, 72],
        hsl: &[35, 26, 80],
        shade: "Brown"
      },
      Colour {
        name: "Mimosa",
        hex: "F5F5CC",
        rgb: &[245, 245, 204],
        hsl: &[42, 171, 224],
        shade: "Green"
      },
      Colour {
        name: "Mindaro",
        hex: "DAEA6F",
        rgb: &[218, 234, 111],
        hsl: &[48, 190, 172],
        shade: "Green"
      },
      Colour {
        name: "Mine Shaft",
        hex: "373E41",
        rgb: &[55, 62, 65],
        hsl: &[140, 21, 60],
        shade: "Blue"
      },
      Colour {
        name: "Mineral Green",
        hex: "506355",
        rgb: &[80, 99, 85],
        hsl: &[96, 27, 89],
        shade: "Green"
      },
      Colour {
        name: "Ming",
        hex: "407577",
        rgb: &[64, 117, 119],
        hsl: &[129, 76, 91],
        shade: "Green"
      },
      Colour {
        name: "Minsk",
        hex: "3E3267",
        rgb: &[62, 50, 103],
        hsl: &[179, 88, 76],
        shade: "Violet"
      },
      Colour {
        name: "Mint Cream",
        hex: "F5FFFA",
        rgb: &[245, 255, 250],
        hsl: &[106, 255, 250],
        shade: "White"
      },
      Colour {
        name: "Mint Green",
        hex: "98FF98",
        rgb: &[152, 255, 152],
        hsl: &[85, 255, 203],
        shade: "Green"
      },
      Colour {
        name: "Mint Julep",
        hex: "E0D8A7",
        rgb: &[224, 216, 167],
        hsl: &[36, 122, 195],
        shade: "Green"
      },
      Colour {
        name: "Mint Tulip",
        hex: "C6EADD",
        rgb: &[198, 234, 221],
        hsl: &[112, 117, 216],
        shade: "Green"
      },
      Colour {
        name: "Mirage",
        hex: "373F43",
        rgb: &[55, 63, 67],
        hsl: &[141, 25, 61],
        shade: "Blue"
      },
      Colour {
        name: "Mischka",
        hex: "A5A9B2",
        rgb: &[165, 169, 178],
        hsl: &[156, 19, 171],
        shade: "Blue"
      },
      Colour {
        name: "Mist Grey",
        hex: "BAB9A9",
        rgb: &[186, 185, 169],
        hsl: &[40, 27, 177],
        shade: "Grey"
      },
      Colour {
        name: "Misty Rose",
        hex: "FFE4E1",
        rgb: &[255, 228, 225],
        hsl: &[4, 255, 240],
        shade: "Violet"
      },
      Colour {
        name: "Mobster",
        hex: "605A67",
        rgb: &[96, 90, 103],
        hsl: &[189, 17, 96],
        shade: "Violet"
      },
      Colour {
        name: "Moccaccino",
        hex: "582F2B",
        rgb: &[88, 47, 43],
        hsl: &[3, 87, 65],
        shade: "Red"
      },
      Colour {
        name: "Moccasin",
        hex: "FFE4B5",
        rgb: &[255, 228, 181],
        hsl: &[26, 255, 218],
        shade: "Yellow"
      },
      Colour {
        name: "Mocha",
        hex: "6F372D",
        rgb: &[111, 55, 45],
        hsl: &[6, 107, 78],
        shade: "Red"
      },
      Colour {
        name: "Mojo",
        hex: "97463C",
        rgb: &[151, 70, 60],
        hsl: &[4, 109, 105],
        shade: "Red"
      },
      Colour {
        name: "Mona Lisa",
        hex: "FF9889",
        rgb: &[255, 152, 137],
        hsl: &[5, 254, 195],
        shade: "Red"
      },
      Colour {
        name: "Monarch",
        hex: "6B252C",
        rgb: &[107, 37, 44],
        hsl: &[-4, 123, 72],
        shade: "Red"
      },
      Colour {
        name: "Mondo",
        hex: "554D42",
        rgb: &[85, 77, 66],
        hsl: &[24, 32, 75],
        shade: "Brown"
      },
      Colour {
        name: "Mongoose",
        hex: "A58B6F",
        rgb: &[165, 139, 111],
        hsl: &[22, 58, 138],
        shade: "Brown"
      },
      Colour {
        name: "Monsoon",
        hex: "7A7679",
        rgb: &[122, 118, 121],
        hsl: &[-31, 4, 120],
        shade: "Grey"
      },
      Colour {
        name: "Montana",
        hex: "393B3C",
        rgb: &[57, 59, 60],
        hsl: &[141, 6, 58],
        shade: "Grey"
      },
      Colour {
        name: "Monte Carlo",
        hex: "7AC5B4",
        rgb: &[122, 197, 180],
        hsl: &[117, 100, 159],
        shade: "Green"
      },
      Colour {
        name: "Moody Blue",
        hex: "8378C7",
        rgb: &[131, 120, 199],
        hsl: &[175, 105, 159],
        shade: "Violet"
      },
      Colour {
        name: "Moon Glow",
        hex: "F5F3CE",
        rgb: &[245, 243, 206],
        hsl: &[40, 168, 225],
        shade: "Green"
      },
      Colour {
        name: "Moon Mist",
        hex: "CECDB8",
        rgb: &[206, 205, 184],
        hsl: &[40, 46, 195],
        shade: "Green"
      },
      Colour {
        name: "Moon Raker",
        hex: "C0B2D7",
        rgb: &[192, 178, 215],
        hsl: &[186, 80, 196],
        shade: "Violet"
      },
      Colour {
        name: "Moon Yellow",
        hex: "F0C420",
        rgb: &[240, 196, 32],
        hsl: &[33, 222, 136],
        shade: "Yellow"
      },
      Colour {
        name: "Morning Glory",
        hex: "9ED1D3",
        rgb: &[158, 209, 211],
        hsl: &[129, 95, 184],
        shade: "Blue"
      },
      Colour {
        name: "Morocco Brown",
        hex: "442D21",
        rgb: &[68, 45, 33],
        hsl: &[14, 88, 50],
        shade: "Brown"
      },
      Colour {
        name: "Mortar",
        hex: "565051",
        rgb: &[86, 80, 81],
        hsl: &[-7, 9, 83],
        shade: "Grey"
      },
      Colour {
        name: "Mosque",
        hex: "005F5B",
        rgb: &[0, 95, 91],
        hsl: &[125, 255, 47],
        shade: "Green"
      },
      Colour {
        name: "Moss Green",
        hex: "ADDFAD",
        rgb: &[173, 223, 173],
        hsl: &[85, 111, 198],
        shade: "Green"
      },
      Colour {
        name: "Mountain Meadow",
        hex: "1AB385",
        rgb: &[26, 179, 133],
        hsl: &[114, 190, 102],
        shade: "Green"
      },
      Colour {
        name: "Mountain Mist",
        hex: "A09F9C",
        rgb: &[160, 159, 156],
        hsl: &[31, 5, 158],
        shade: "Grey"
      },
      Colour {
        name: "Mountbatten Pink",
        hex: "997A8D",
        rgb: &[153, 122, 141],
        hsl: &[-26, 33, 137],
        shade: "Violet"
      },
      Colour {
        name: "Muddy Waters",
        hex: "A9844F",
        rgb: &[169, 132, 79],
        hsl: &[25, 92, 124],
        shade: "Yellow"
      },
      Colour {
        name: "Muesli",
        hex: "9E7E53",
        rgb: &[158, 126, 83],
        hsl: &[24, 79, 120],
        shade: "Brown"
      },
      Colour {
        name: "Mulberry",
        hex: "C54B8C",
        rgb: &[197, 75, 140],
        hsl: &[-22, 130, 136],
        shade: "Violet"
      },
      Colour {
        name: "Mule Fawn",
        hex: "884F40",
        rgb: &[136, 79, 64],
        hsl: &[8, 91, 100],
        shade: "Brown"
      },
      Colour {
        name: "Mulled Wine",
        hex: "524D5B",
        rgb: &[82, 77, 91],
        hsl: &[185, 21, 84],
        shade: "Violet"
      },
      Colour {
        name: "Mustard",
        hex: "FFDB58",
        rgb: &[255, 219, 88],
        hsl: &[33, 255, 171],
        shade: "Yellow"
      },
      Colour {
        name: "My Pink",
        hex: "D68B80",
        rgb: &[214, 139, 128],
        hsl: &[5, 130, 171],
        shade: "Red"
      },
      Colour {
        name: "My Sin",
        hex: "FDAE45",
        rgb: &[253, 174, 69],
        hsl: &[24, 249, 161],
        shade: "Yellow"
      },
      Colour {
        name: "Myrtle",
        hex: "21421E",
        rgb: &[33, 66, 30],
        hsl: &[81, 95, 48],
        shade: "Green"
      },
      Colour {
        name: "Mystic",
        hex: "D8DDDA",
        rgb: &[216, 221, 218],
        hsl: &[101, 17, 218],
        shade: "Grey"
      },
      Colour {
        name: "Nandor",
        hex: "4E5D4E",
        rgb: &[78, 93, 78],
        hsl: &[85, 22, 85],
        shade: "Green"
      },
      Colour {
        name: "Napa",
        hex: "A39A87",
        rgb: &[163, 154, 135],
        hsl: &[28, 33, 148],
        shade: "Yellow"
      },
      Colour {
        name: "Narvik",
        hex: "E9E6DC",
        rgb: &[233, 230, 220],
        hsl: &[32, 58, 226],
        shade: "Green"
      },
      Colour {
        name: "Navajo White",
        hex: "FFDEAD",
        rgb: &[255, 222, 173],
        hsl: &[25, 255, 214],
        shade: "Brown"
      },
      Colour {
        name: "Navy",
        hex: "000080",
        rgb: &[0, 0, 128],
        hsl: &[170, 255, 64],
        shade: "Blue"
      },
      Colour {
        name: "Navy Blue",
        hex: "0066CC",
        rgb: &[0, 102, 204],
        hsl: &[148, 255, 102],
        shade: "Blue"
      },
      Colour {
        name: "Nebula",
        hex: "B8C6BE",
        rgb: &[184, 198, 190],
        hsl: &[103, 27, 191],
        shade: "Green"
      },
      Colour {
        name: "Negroni",
        hex: "EEC7A2",
        rgb: &[238, 199, 162],
        hsl: &[20, 176, 200],
        shade: "Orange"
      },
      Colour {
        name: "Neon Blue",
        hex: "4D4DFF",
        rgb: &[77, 77, 255],
        hsl: &[170, 255, 166],
        shade: "Blue"
      },
      Colour {
        name: "Neon Carrot",
        hex: "FF9933",
        rgb: &[255, 153, 51],
        hsl: &[21, 255, 153],
        shade: "Orange"
      },
      Colour {
        name: "Neon Pink",
        hex: "FF6EC7",
        rgb: &[255, 110, 199],
        hsl: &[-26, 255, 182],
        shade: "Violet"
      },
      Colour {
        name: "Nepal",
        hex: "93AAB9",
        rgb: &[147, 170, 185],
        hsl: &[144, 54, 165],
        shade: "Blue"
      },
      Colour {
        name: "Neptune",
        hex: "77A8AB",
        rgb: &[119, 168, 171],
        hsl: &[129, 60, 145],
        shade: "Green"
      },
      Colour {
        name: "Nero",
        hex: "252525",
        rgb: &[37, 37, 37],
        hsl: &[0, 0, 37],
        shade: "Grey"
      },
      Colour {
        name: "Neutral Green",
        hex: "AAA583",
        rgb: &[170, 165, 131],
        hsl: &[37, 47, 150],
        shade: "Green"
      },
      Colour {
        name: "Nevada",
        hex: "666F6F",
        rgb: &[102, 111, 111],
        hsl: &[127, 10, 106],
        shade: "Grey"
      },
      Colour {
        name: "New Amber",
        hex: "6D3B24",
        rgb: &[109, 59, 36],
        hsl: &[13, 128, 72],
        shade: "Orange"
      },
      Colour {
        name: "New Midnight Blue",
        hex: "00009C",
        rgb: &[0, 0, 156],
        hsl: &[170, 255, 78],
        shade: "Blue"
      },
      Colour {
        name: "New Orleans",
        hex: "E4C385",
        rgb: &[228, 195, 133],
        hsl: &[27, 162, 180],
        shade: "Yellow"
      },
      Colour {
        name: "New Tan",
        hex: "EBC79E",
        rgb: &[235, 199, 158],
        hsl: &[22, 167, 196],
        shade: "Brown"
      },
      Colour {
        name: "New York Pink",
        hex: "DD8374",
        rgb: &[221, 131, 116],
        hsl: &[6, 154, 168],
        shade: "Red"
      },
      Colour {
        name: "Niagara",
        hex: "29A98B",
        rgb: &[41, 169, 139],
        hsl: &[117, 155, 105],
        shade: "Green"
      },
      Colour {
        name: "Night Rider",
        hex: "332E2E",
        rgb: &[51, 46, 46],
        hsl: &[0, 13, 48],
        shade: "Grey"
      },
      Colour {
        name: "Night Shadz",
        hex: "A23D54",
        rgb: &[162, 61, 84],
        hsl: &[-9, 115, 111],
        shade: "Red"
      },
      Colour {
        name: "Nile Blue",
        hex: "253F4E",
        rgb: &[37, 63, 78],
        hsl: &[143, 90, 57],
        shade: "Blue"
      },
      Colour {
        name: "Nobel",
        hex: "A99D9D",
        rgb: &[169, 157, 157],
        hsl: &[0, 16, 163],
        shade: "Grey"
      },
      Colour {
        name: "Nomad",
        hex: "A19986",
        rgb: &[161, 153, 134],
        hsl: &[29, 32, 147],
        shade: "Yellow"
      },
      Colour {
        name: "Nordic",
        hex: "1D393C",
        rgb: &[29, 57, 60],
        hsl: &[131, 88, 44],
        shade: "Blue"
      },
      Colour {
        name: "Norway",
        hex: "A4B88F",
        rgb: &[164, 184, 143],
        hsl: &[63, 57, 163],
        shade: "Green"
      },
      Colour {
        name: "Nugget",
        hex: "BC9229",
        rgb: &[188, 146, 41],
        hsl: &[30, 163, 114],
        shade: "Yellow"
      },
      Colour {
        name: "Nutmeg",
        hex: "7E4A3B",
        rgb: &[126, 74, 59],
        hsl: &[9, 92, 92],
        shade: "Brown"
      },
      Colour {
        name: "Oasis",
        hex: "FCEDC5",
        rgb: &[252, 237, 197],
        hsl: &[30, 229, 224],
        shade: "Yellow"
      },
      Colour {
        name: "Observatory",
        hex: "008F70",
        rgb: &[0, 143, 112],
        hsl: &[118, 255, 71],
        shade: "Green"
      },
      Colour {
        name: "Ocean Green",
        hex: "4CA973",
        rgb: &[76, 169, 115],
        hsl: &[102, 96, 122],
        shade: "Green"
      },
      Colour {
        name: "Ochre",
        hex: "CC7722",
        rgb: &[204, 119, 34],
        hsl: &[21, 182, 119],
        shade: "Brown"
      },
      Colour {
        name: "Off Green",
        hex: "DFF0E2",
        rgb: &[223, 240, 226],
        hsl: &[92, 92, 231],
        shade: "Green"
      },
      Colour {
        name: "Off Yellow",
        hex: "FAF3DC",
        rgb: &[250, 243, 220],
        hsl: &[32, 191, 235],
        shade: "Yellow"
      },
      Colour {
        name: "Oil",
        hex: "313330",
        rgb: &[49, 51, 48],
        hsl: &[70, 7, 49],
        shade: "Grey"
      },
      Colour {
        name: "Old Brick",
        hex: "8A3335",
        rgb: &[138, 51, 53],
        hsl: &[0, 117, 94],
        shade: "Red"
      },
      Colour {
        name: "Old Copper",
        hex: "73503B",
        rgb: &[115, 80, 59],
        hsl: &[15, 82, 87],
        shade: "Red"
      },
      Colour {
        name: "Old Gold",
        hex: "CFB53B",
        rgb: &[207, 181, 59],
        hsl: &[35, 154, 133],
        shade: "Yellow"
      },
      Colour {
        name: "Old Lace",
        hex: "FDF5E6",
        rgb: &[253, 245, 230],
        hsl: &[27, 217, 241],
        shade: "White"
      },
      Colour {
        name: "Old Lavender",
        hex: "796878",
        rgb: &[121, 104, 120],
        hsl: &[-40, 19, 112],
        shade: "Violet"
      },
      Colour {
        name: "Old Rose",
        hex: "C02E4C",
        rgb: &[192, 46, 76],
        hsl: &[-8, 156, 119],
        shade: "Red"
      },
      Colour {
        name: "Olive",
        hex: "808000",
        rgb: &[128, 128, 0],
        hsl: &[42, 255, 64],
        shade: "Green"
      },
      Colour {
        name: "Olive Drab",
        hex: "6B8E23",
        rgb: &[107, 142, 35],
        hsl: &[56, 154, 88],
        shade: "Green"
      },
      Colour {
        name: "Olive Green",
        hex: "B5B35C",
        rgb: &[181, 179, 92],
        hsl: &[41, 95, 136],
        shade: "Green"
      },
      Colour {
        name: "Olive Haze",
        hex: "888064",
        rgb: &[136, 128, 100],
        hsl: &[33, 38, 118],
        shade: "Yellow"
      },
      Colour {
        name: "Olivetone",
        hex: "747028",
        rgb: &[116, 112, 40],
        hsl: &[40, 124, 78],
        shade: "Green"
      },
      Colour {
        name: "Olivine",
        hex: "9AB973",
        rgb: &[154, 185, 115],
        hsl: &[61, 85, 150],
        shade: "Orange"
      },
      Colour {
        name: "Onahau",
        hex: "C2E6EC",
        rgb: &[194, 230, 236],
        hsl: &[133, 133, 215],
        shade: "Blue"
      },
      Colour {
        name: "Onion",
        hex: "48412B",
        rgb: &[72, 65, 43],
        hsl: &[32, 64, 57],
        shade: "Yellow"
      },
      Colour {
        name: "Opal",
        hex: "A8C3BC",
        rgb: &[168, 195, 188],
        hsl: &[116, 46, 181],
        shade: "Green"
      },
      Colour {
        name: "Opium",
        hex: "987E7E",
        rgb: &[152, 126, 126],
        hsl: &[0, 28, 139],
        shade: "Brown"
      },
      Colour {
        name: "Oracle",
        hex: "395555",
        rgb: &[57, 85, 85],
        hsl: &[127, 50, 71],
        shade: "Green"
      },
      Colour {
        name: "Orange",
        hex: "FFA500",
        rgb: &[255, 165, 0],
        hsl: &[27, 255, 127],
        shade: "Orange"
      },
      Colour {
        name: "Orange Peel",
        hex: "FFA000",
        rgb: &[255, 160, 0],
        hsl: &[26, 255, 127],
        shade: "Orange"
      },
      Colour {
        name: "Orange Red",
        hex: "FF4500",
        rgb: &[255, 69, 0],
        hsl: &[11, 255, 127],
        shade: "Orange"
      },
      Colour {
        name: "Orange Roughy",
        hex: "A85335",
        rgb: &[168, 83, 53],
        hsl: &[11, 132, 110],
        shade: "Orange"
      },
      Colour {
        name: "Orange White",
        hex: "EAE3CD",
        rgb: &[234, 227, 205],
        hsl: &[32, 104, 219],
        shade: "Yellow"
      },
      Colour {
        name: "Orchid",
        hex: "DA70D6",
        rgb: &[218, 112, 214],
        hsl: &[-40, 150, 164],
        shade: "Violet"
      },
      Colour {
        name: "Orchid White",
        hex: "F1EBD9",
        rgb: &[241, 235, 217],
        hsl: &[31, 117, 228],
        shade: "Yellow"
      },
      Colour {
        name: "Orient",
        hex: "255B77",
        rgb: &[37, 91, 119],
        hsl: &[142, 134, 78],
        shade: "Blue"
      },
      Colour {
        name: "Oriental Pink",
        hex: "C28E88",
        rgb: &[194, 142, 136],
        hsl: &[4, 82, 164],
        shade: "Red"
      },
      Colour {
        name: "Orinoco",
        hex: "D2D3B3",
        rgb: &[210, 211, 179],
        hsl: &[43, 67, 195],
        shade: "Green"
      },
      Colour {
        name: "Oslo Grey",
        hex: "818988",
        rgb: &[129, 137, 136],
        hsl: &[122, 8, 132],
        shade: "Grey"
      },
      Colour {
        name: "Ottoman",
        hex: "D3DBCB",
        rgb: &[211, 219, 203],
        hsl: &[63, 46, 211],
        shade: "Green"
      },
      Colour {
        name: "Outer Space",
        hex: "2D383A",
        rgb: &[45, 56, 58],
        hsl: &[134, 32, 51],
        shade: "Grey"
      },
      Colour {
        name: "Outrageous Orange",
        hex: "FF6037",
        rgb: &[255, 96, 55],
        hsl: &[8, 255, 155],
        shade: "Orange"
      },
      Colour {
        name: "Oxford Blue",
        hex: "28353A",
        rgb: &[40, 53, 58],
        hsl: &[139, 46, 48],
        shade: "Blue"
      },
      Colour {
        name: "Oxley",
        hex: "6D9A78",
        rgb: &[109, 154, 120],
        hsl: &[95, 46, 131],
        shade: "Green"
      },
      Colour {
        name: "Oyster Bay",
        hex: "D1EAEA",
        rgb: &[209, 234, 234],
        hsl: &[127, 95, 221],
        shade: "Blue"
      },
      Colour {
        name: "Oyster Pink",
        hex: "D4B5B0",
        rgb: &[212, 181, 176],
        hsl: &[5, 75, 194],
        shade: "Red"
      },
      Colour {
        name: "Paarl",
        hex: "864B36",
        rgb: &[134, 75, 54],
        hsl: &[11, 108, 94],
        shade: "Orange"
      },
      Colour {
        name: "Pablo",
        hex: "7A715C",
        rgb: &[122, 113, 92],
        hsl: &[29, 35, 107],
        shade: "Yellow"
      },
      Colour {
        name: "Pacific Blue",
        hex: "009DC4",
        rgb: &[0, 157, 196],
        hsl: &[135, 255, 98],
        shade: "Blue"
      },
      Colour {
        name: "Paco",
        hex: "4F4037",
        rgb: &[79, 64, 55],
        hsl: &[15, 45, 67],
        shade: "Brown"
      },
      Colour {
        name: "Padua",
        hex: "7EB394",
        rgb: &[126, 179, 148],
        hsl: &[102, 65, 152],
        shade: "Green"
      },
      Colour {
        name: "Palatinate Purple",
        hex: "682860",
        rgb: &[104, 40, 96],
        hsl: &[-37, 113, 72],
        shade: "Violet"
      },
      Colour {
        name: "Pale Brown",
        hex: "987654",
        rgb: &[152, 118, 84],
        hsl: &[21, 73, 118],
        shade: "Brown"
      },
      Colour {
        name: "Pale Chestnut",
        hex: "DDADAF",
        rgb: &[221, 173, 175],
        hsl: &[-1, 105, 197],
        shade: "Red"
      },
      Colour {
        name: "Pale Cornflower Blue",
        hex: "ABCDEF",
        rgb: &[171, 205, 239],
        hsl: &[148, 173, 205],
        shade: "Blue"
      },
      Colour {
        name: "Pale Goldenrod",
        hex: "EEE8AA",
        rgb: &[238, 232, 170],
        hsl: &[38, 170, 204],
        shade: "Yellow"
      },
      Colour {
        name: "Pale Green",
        hex: "98FB98",
        rgb: &[152, 251, 152],
        hsl: &[85, 235, 201],
        shade: "Green"
      },
      Colour {
        name: "Pale Leaf",
        hex: "BDCAA8",
        rgb: &[189, 202, 168],
        hsl: &[58, 61, 185],
        shade: "Green"
      },
      Colour {
        name: "Pale Magenta",
        hex: "F984E5",
        rgb: &[249, 132, 229],
        hsl: &[-35, 231, 190],
        shade: "Violet"
      },
      Colour {
        name: "Pale Oyster",
        hex: "9C8D72",
        rgb: &[156, 141, 114],
        hsl: &[27, 44, 135],
        shade: "Brown"
      },
      Colour {
        name: "Pale Pink",
        hex: "FADADD",
        rgb: &[250, 218, 221],
        hsl: &[-3, 194, 234],
        shade: "Red"
      },
      Colour {
        name: "Pale Prim",
        hex: "F9F59F",
        rgb: &[249, 245, 159],
        hsl: &[40, 225, 204],
        shade: "Green"
      },
      Colour {
        name: "Pale Rose",
        hex: "EFD6DA",
        rgb: &[239, 214, 218],
        hsl: &[-6, 111, 226],
        shade: "Red"
      },
      Colour {
        name: "Pale Sky",
        hex: "636D70",
        rgb: &[99, 109, 112],
        hsl: &[137, 15, 105],
        shade: "Blue"
      },
      Colour {
        name: "Pale Slate",
        hex: "C3BEBB",
        rgb: &[195, 190, 187],
        hsl: &[15, 15, 191],
        shade: "Grey"
      },
      Colour {
        name: "Pale Taupe",
        hex: "BC987E",
        rgb: &[188, 152, 126],
        hsl: &[17, 80, 157],
        shade: "Grey"
      },
      Colour {
        name: "Pale Turquoise",
        hex: "AFEEEE",
        rgb: &[175, 238, 238],
        hsl: &[127, 165, 206],
        shade: "Blue"
      },
      Colour {
        name: "Pale Violet Red",
        hex: "DB7093",
        rgb: &[219, 112, 147],
        hsl: &[-13, 152, 165],
        shade: "Red"
      },
      Colour {
        name: "Palm Green",
        hex: "20392C",
        rgb: &[32, 57, 44],
        hsl: &[105, 71, 44],
        shade: "Green"
      },
      Colour {
        name: "Palm Leaf",
        hex: "36482F",
        rgb: &[54, 72, 47],
        hsl: &[73, 53, 59],
        shade: "Green"
      },
      Colour {
        name: "Pampas",
        hex: "EAE4DC",
        rgb: &[234, 228, 220],
        hsl: &[24, 63, 227],
        shade: "Grey"
      },
      Colour {
        name: "Panache",
        hex: "EBF7E4",
        rgb: &[235, 247, 228],
        hsl: &[69, 138, 237],
        shade: "Green"
      },
      Colour {
        name: "Pancho",
        hex: "DFB992",
        rgb: &[223, 185, 146],
        hsl: &[21, 139, 184],
        shade: "Orange"
      },
      Colour {
        name: "Panda",
        hex: "544F3A",
        rgb: &[84, 79, 58],
        hsl: &[34, 46, 71],
        shade: "Yellow"
      },
      Colour {
        name: "Papaya Whip",
        hex: "FFEFD5",
        rgb: &[255, 239, 213],
        hsl: &[26, 255, 234],
        shade: "Yellow"
      },
      Colour {
        name: "Paprika",
        hex: "7C2D37",
        rgb: &[124, 45, 55],
        hsl: &[-5, 119, 84],
        shade: "Red"
      },
      Colour {
        name: "Paradiso",
        hex: "488084",
        rgb: &[72, 128, 132],
        hsl: &[130, 75, 102],
        shade: "Green"
      },
      Colour {
        name: "Parchment",
        hex: "D0C8B0",
        rgb: &[208, 200, 176],
        hsl: &[31, 64, 192],
        shade: "Yellow"
      },
      Colour {
        name: "Paris Daisy",
        hex: "FBEB50",
        rgb: &[251, 235, 80],
        hsl: &[38, 243, 165],
        shade: "Green"
      },
      Colour {
        name: "Paris M",
        hex: "312760",
        rgb: &[49, 39, 96],
        hsl: &[177, 107, 67],
        shade: "Violet"
      },
      Colour {
        name: "Paris White",
        hex: "BFCDC0",
        rgb: &[191, 205, 192],
        hsl: &[88, 31, 198],
        shade: "Green"
      },
      Colour {
        name: "Parsley",
        hex: "305D35",
        rgb: &[48, 93, 53],
        hsl: &[89, 81, 70],
        shade: "Green"
      },
      Colour {
        name: "Pastel Green",
        hex: "77DD77",
        rgb: &[119, 221, 119],
        hsl: &[85, 153, 170],
        shade: "Green"
      },
      Colour {
        name: "Patina",
        hex: "639283",
        rgb: &[99, 146, 131],
        hsl: &[113, 48, 122],
        shade: "Green"
      },
      Colour {
        name: "Pattens Blue",
        hex: "D3E5EF",
        rgb: &[211, 229, 239],
        hsl: &[142, 119, 225],
        shade: "Blue"
      },
      Colour {
        name: "Paua",
        hex: "2A2551",
        rgb: &[42, 37, 81],
        hsl: &[174, 95, 59],
        shade: "Violet"
      },
      Colour {
        name: "Pavlova",
        hex: "BAAB87",
        rgb: &[186, 171, 135],
        hsl: &[29, 68, 160],
        shade: "Yellow"
      },
      Colour {
        name: "Payne's Grey",
        hex: "404048",
        rgb: &[64, 64, 72],
        hsl: &[170, 15, 68],
        shade: "Grey"
      },
      Colour {
        name: "Peach",
        hex: "FFCBA4",
        rgb: &[255, 203, 164],
        hsl: &[18, 254, 209],
        shade: "Orange"
      },
      Colour {
        name: "Peach Puff",
        hex: "FFDAB9",
        rgb: &[255, 218, 185],
        hsl: &[20, 255, 220],
        shade: "Yellow"
      },
      Colour {
        name: "Peach-Orange",
        hex: "FFCC99",
        rgb: &[255, 204, 153],
        hsl: &[21, 255, 204],
        shade: "Orange"
      },
      Colour {
        name: "Peach-Yellow",
        hex: "FADFAD",
        rgb: &[250, 223, 173],
        hsl: &[27, 225, 211],
        shade: "Yellow"
      },
      Colour {
        name: "Peanut",
        hex: "7A4434",
        rgb: &[122, 68, 52],
        hsl: &[9, 102, 87],
        shade: "Brown"
      },
      Colour {
        name: "Pear",
        hex: "D1E231",
        rgb: &[209, 226, 49],
        hsl: &[46, 192, 137],
        shade: "Yellow"
      },
      Colour {
        name: "Pearl Bush",
        hex: "DED1C6",
        rgb: &[222, 209, 198],
        hsl: &[19, 67, 210],
        shade: "Orange"
      },
      Colour {
        name: "Pearl Lusta",
        hex: "EAE0C8",
        rgb: &[234, 224, 200],
        hsl: &[30, 114, 217],
        shade: "Yellow"
      },
      Colour {
        name: "Peat",
        hex: "766D52",
        rgb: &[118, 109, 82],
        hsl: &[31, 45, 100],
        shade: "Yellow"
      },
      Colour {
        name: "Pelorous",
        hex: "2599B2",
        rgb: &[37, 153, 178],
        hsl: &[135, 167, 107],
        shade: "Blue"
      },
      Colour {
        name: "Peppermint",
        hex: "D7E7D0",
        rgb: &[215, 231, 208],
        hsl: &[72, 82, 219],
        shade: "Green"
      },
      Colour {
        name: "Perano",
        hex: "ACB9E8",
        rgb: &[172, 185, 232],
        hsl: &[160, 144, 202],
        shade: "Blue"
      },
      Colour {
        name: "Perfume",
        hex: "C2A9DB",
        rgb: &[194, 169, 219],
        hsl: &[191, 104, 194],
        shade: "Violet"
      },
      Colour {
        name: "Periglacial Blue",
        hex: "ACB6B2",
        rgb: &[172, 182, 178],
        hsl: &[110, 16, 177],
        shade: "Green"
      },
      Colour {
        name: "Periwinkle",
        hex: "C3CDE6",
        rgb: &[195, 205, 230],
        hsl: &[157, 105, 212],
        shade: "Blue"
      },
      Colour {
        name: "Persian Blue",
        hex: "1C39BB",
        rgb: &[28, 57, 187],
        hsl: &[162, 188, 107],
        shade: "Blue"
      },
      Colour {
        name: "Persian Green",
        hex: "00A693",
        rgb: &[0, 166, 147],
        hsl: &[122, 255, 83],
        shade: "Green"
      },
      Colour {
        name: "Persian Indigo",
        hex: "32127A",
        rgb: &[50, 18, 122],
        hsl: &[183, 189, 70],
        shade: "Violet"
      },
      Colour {
        name: "Persian Pink",
        hex: "F77FBE",
        rgb: &[247, 127, 190],
        hsl: &[-22, 225, 187],
        shade: "Red"
      },
      Colour {
        name: "Persian Plum",
        hex: "683332",
        rgb: &[104, 51, 50],
        hsl: &[0, 89, 77],
        shade: "Red"
      },
      Colour {
        name: "Persian Red",
        hex: "CC3333",
        rgb: &[204, 51, 51],
        hsl: &[0, 153, 127],
        shade: "Red"
      },
      Colour {
        name: "Persian Rose",
        hex: "FE28A2",
        rgb: &[254, 40, 162],
        hsl: &[-24, 252, 147],
        shade: "Red"
      },
      Colour {
        name: "Persimmon",
        hex: "EC5800",
        rgb: &[236, 88, 0],
        hsl: &[15, 255, 118],
        shade: "Red"
      },
      Colour {
        name: "Peru",
        hex: "CD853F",
        rgb: &[205, 133, 63],
        hsl: &[20, 149, 134],
        shade: "Brown"
      },
      Colour {
        name: "Peru Tan",
        hex: "733D1F",
        rgb: &[115, 61, 31],
        hsl: &[15, 146, 73],
        shade: "Orange"
      },
      Colour {
        name: "Pesto",
        hex: "7A7229",
        rgb: &[122, 114, 41],
        hsl: &[38, 126, 81],
        shade: "Yellow"
      },
      Colour {
        name: "Petite Orchid",
        hex: "DA9790",
        rgb: &[218, 151, 144],
        hsl: &[4, 127, 180],
        shade: "Red"
      },
      Colour {
        name: "Pewter",
        hex: "91A092",
        rgb: &[145, 160, 146],
        hsl: &[87, 18, 152],
        shade: "Green"
      },
      Colour {
        name: "Pharlap",
        hex: "826663",
        rgb: &[130, 102, 99],
        hsl: &[4, 34, 114],
        shade: "Brown"
      },
      Colour {
        name: "Picasso",
        hex: "F8EA97",
        rgb: &[248, 234, 151],
        hsl: &[36, 222, 199],
        shade: "Green"
      },
      Colour {
        name: "Picton Blue",
        hex: "5BA0D0",
        rgb: &[91, 160, 208],
        hsl: &[144, 141, 149],
        shade: "Blue"
      },
      Colour {
        name: "Pig Pink",
        hex: "FDD7E4",
        rgb: &[253, 215, 228],
        hsl: &[-14, 230, 234],
        shade: "Red"
      },
      Colour {
        name: "Pigment Green",
        hex: "00A550",
        rgb: &[0, 165, 80],
        hsl: &[105, 255, 82],
        shade: "Green"
      },
      Colour {
        name: "Pine Cone",
        hex: "756556",
        rgb: &[117, 101, 86],
        hsl: &[20, 38, 101],
        shade: "Brown"
      },
      Colour {
        name: "Pine Glade",
        hex: "BDC07E",
        rgb: &[189, 192, 126],
        hsl: &[44, 87, 159],
        shade: "Green"
      },
      Colour {
        name: "Pine Green",
        hex: "01796F",
        rgb: &[1, 121, 111],
        hsl: &[123, 250, 60],
        shade: "Green"
      },
      Colour {
        name: "Pine Tree",
        hex: "2A2F23",
        rgb: &[42, 47, 35],
        hsl: &[60, 37, 41],
        shade: "Green"
      },
      Colour {
        name: "Pink",
        hex: "FFC0CB",
        rgb: &[255, 192, 203],
        hsl: &[-7, 255, 223],
        shade: "Red"
      },
      Colour {
        name: "Pink Flamingo",
        hex: "FF66FF",
        rgb: &[255, 102, 255],
        hsl: &[-42, 254, 178],
        shade: "Red"
      },
      Colour {
        name: "Pink Flare",
        hex: "D8B4B6",
        rgb: &[216, 180, 182],
        hsl: &[-2, 80, 198],
        shade: "Red"
      },
      Colour {
        name: "Pink Lace",
        hex: "F6CCD7",
        rgb: &[246, 204, 215],
        hsl: &[-11, 178, 225],
        shade: "Red"
      },
      Colour {
        name: "Pink Lady",
        hex: "F3D7B6",
        rgb: &[243, 215, 182],
        hsl: &[22, 182, 212],
        shade: "Orange"
      },
      Colour {
        name: "Pink Swan",
        hex: "BFB3B2",
        rgb: &[191, 179, 178],
        hsl: &[3, 23, 184],
        shade: "Grey"
      },
      Colour {
        name: "Piper",
        hex: "9D5432",
        rgb: &[157, 84, 50],
        hsl: &[13, 131, 103],
        shade: "Orange"
      },
      Colour {
        name: "Pipi",
        hex: "F5E6C4",
        rgb: &[245, 230, 196],
        hsl: &[29, 181, 220],
        shade: "Yellow"
      },
      Colour {
        name: "Pippin",
        hex: "FCDBD2",
        rgb: &[252, 219, 210],
        hsl: &[9, 223, 231],
        shade: "Red"
      },
      Colour {
        name: "Pirate Gold",
        hex: "BA782A",
        rgb: &[186, 120, 42],
        hsl: &[23, 161, 113],
        shade: "Yellow"
      },
      Colour {
        name: "Pixie Green",
        hex: "BBCDA5",
        rgb: &[187, 205, 165],
        hsl: &[61, 72, 185],
        shade: "Green"
      },
      Colour {
        name: "Pizazz",
        hex: "E57F3D",
        rgb: &[229, 127, 61],
        hsl: &[16, 194, 145],
        shade: "Orange"
      },
      Colour {
        name: "Pizza",
        hex: "BF8D3C",
        rgb: &[191, 141, 60],
        hsl: &[26, 133, 125],
        shade: "Yellow"
      },
      Colour {
        name: "Plantation",
        hex: "3E594C",
        rgb: &[62, 89, 76],
        hsl: &[107, 45, 75],
        shade: "Green"
      },
      Colour {
        name: "Plum",
        hex: "DDA0DD",
        rgb: &[221, 160, 221],
        hsl: &[-42, 120, 190],
        shade: "Violet"
      },
      Colour {
        name: "Pohutukawa",
        hex: "651C26",
        rgb: &[101, 28, 38],
        hsl: &[-5, 144, 64],
        shade: "Red"
      },
      Colour {
        name: "Polar",
        hex: "E5F2E7",
        rgb: &[229, 242, 231],
        hsl: &[91, 84, 235],
        shade: "Green"
      },
      Colour {
        name: "Polo Blue",
        hex: "8AA7CC",
        rgb: &[138, 167, 204],
        hsl: &[151, 100, 171],
        shade: "Blue"
      },
      Colour {
        name: "Pompadour",
        hex: "6A1F44",
        rgb: &[106, 31, 68],
        hsl: &[-20, 139, 68],
        shade: "Violet"
      },
      Colour {
        name: "Porcelain",
        hex: "DDDCDB",
        rgb: &[221, 220, 219],
        hsl: &[21, 7, 220],
        shade: "Grey"
      },
      Colour {
        name: "Porsche",
        hex: "DF9D5B",
        rgb: &[223, 157, 91],
        hsl: &[21, 171, 157],
        shade: "Orange"
      },
      Colour {
        name: "Port Gore",
        hex: "3B436C",
        rgb: &[59, 67, 108],
        hsl: &[163, 74, 83],
        shade: "Blue"
      },
      Colour {
        name: "Portafino",
        hex: "F4F09B",
        rgb: &[244, 240, 155],
        hsl: &[40, 204, 199],
        shade: "Green"
      },
      Colour {
        name: "Portage",
        hex: "8B98D8",
        rgb: &[139, 152, 216],
        hsl: &[162, 126, 177],
        shade: "Blue"
      },
      Colour {
        name: "Portica",
        hex: "F0D555",
        rgb: &[240, 213, 85],
        hsl: &[35, 213, 162],
        shade: "Yellow"
      },
      Colour {
        name: "Pot Pourri",
        hex: "EFDCD4",
        rgb: &[239, 220, 212],
        hsl: &[12, 116, 225],
        shade: "Orange"
      },
      Colour {
        name: "Potters Clay",
        hex: "845C40",
        rgb: &[132, 92, 64],
        hsl: &[17, 88, 98],
        shade: "Brown"
      },
      Colour {
        name: "Powder Blue",
        hex: "B0E0E6",
        rgb: &[176, 224, 230],
        hsl: &[132, 132, 203],
        shade: "Blue"
      },
      Colour {
        name: "Prairie Sand",
        hex: "883C32",
        rgb: &[136, 60, 50],
        hsl: &[4, 117, 93],
        shade: "Red"
      },
      Colour {
        name: "Prelude",
        hex: "CAB4D4",
        rgb: &[202, 180, 212],
        hsl: &[199, 69, 196],
        shade: "Violet"
      },
      Colour {
        name: "Prim",
        hex: "E2CDD5",
        rgb: &[226, 205, 213],
        hsl: &[-16, 67, 215],
        shade: "Violet"
      },
      Colour {
        name: "Primrose",
        hex: "E4DE8E",
        rgb: &[228, 222, 142],
        hsl: &[39, 156, 185],
        shade: "Green"
      },
      Colour {
        name: "Promenade",
        hex: "F8F6DF",
        rgb: &[248, 246, 223],
        hsl: &[39, 163, 235],
        shade: "Green"
      },
      Colour {
        name: "Provincial Pink",
        hex: "F6E3DA",
        rgb: &[246, 227, 218],
        hsl: &[13, 155, 232],
        shade: "Orange"
      },
      Colour {
        name: "Prussian Blue",
        hex: "003366",
        rgb: &[0, 51, 102],
        hsl: &[148, 255, 51],
        shade: "Blue"
      },
      Colour {
        name: "Psychedelic Purple",
        hex: "DD00FF",
        rgb: &[221, 0, 255],
        hsl: &[206, 255, 127],
        shade: "Violet"
      },
      Colour {
        name: "Puce",
        hex: "CC8899",
        rgb: &[204, 136, 153],
        hsl: &[-10, 102, 170],
        shade: "Red"
      },
      Colour {
        name: "Pueblo",
        hex: "6E3326",
        rgb: &[110, 51, 38],
        hsl: &[7, 124, 74],
        shade: "Orange"
      },
      Colour {
        name: "Puerto Rico",
        hex: "59BAA3",
        rgb: &[89, 186, 163],
        hsl: &[117, 105, 137],
        shade: "Green"
      },
      Colour {
        name: "Pumice",
        hex: "BAC0B4",
        rgb: &[186, 192, 180],
        hsl: &[63, 22, 186],
        shade: "Green"
      },
      Colour {
        name: "Pumpkin",
        hex: "FF7518",
        rgb: &[255, 117, 24],
        hsl: &[17, 255, 139],
        shade: "Orange"
      },
      Colour {
        name: "Punga",
        hex: "534931",
        rgb: &[83, 73, 49],
        hsl: &[29, 65, 66],
        shade: "Yellow"
      },
      Colour {
        name: "Purple",
        hex: "800080",
        rgb: &[128, 0, 128],
        hsl: &[-42, 255, 64],
        shade: "Violet"
      },
      Colour {
        name: "Purple Heart",
        hex: "652DC1",
        rgb: &[101, 45, 193],
        hsl: &[186, 158, 119],
        shade: "Violet"
      },
      Colour {
        name: "Purple Mountain's Majesty",
        hex: "9678B6",
        rgb: &[150, 120, 182],
        hsl: &[190, 76, 151],
        shade: "Violet"
      },
      Colour {
        name: "Purple Taupe",
        hex: "50404D",
        rgb: &[80, 64, 77],
        hsl: &[-34, 28, 72],
        shade: "Grey"
      },
      Colour {
        name: "Putty",
        hex: "CDAE70",
        rgb: &[205, 174, 112],
        hsl: &[28, 122, 158],
        shade: "Yellow"
      },
      Colour {
        name: "Quarter Pearl Lusta",
        hex: "F2EDDD",
        rgb: &[242, 237, 221],
        hsl: &[32, 113, 231],
        shade: "Green"
      },
      Colour {
        name: "Quarter Spanish White",
        hex: "EBE2D2",
        rgb: &[235, 226, 210],
        hsl: &[27, 98, 222],
        shade: "Yellow"
      },
      Colour {
        name: "Quartz",
        hex: "D9D9F3",
        rgb: &[217, 217, 243],
        hsl: &[170, 132, 229],
        shade: "White"
      },
      Colour {
        name: "Quicksand",
        hex: "C3988B",
        rgb: &[195, 152, 139],
        hsl: &[9, 81, 166],
        shade: "Brown"
      },
      Colour {
        name: "Quill Grey",
        hex: "CBC9C0",
        rgb: &[203, 201, 192],
        hsl: &[34, 24, 197],
        shade: "Grey"
      },
      Colour {
        name: "Quincy",
        hex: "6A5445",
        rgb: &[106, 84, 69],
        hsl: &[17, 53, 87],
        shade: "Brown"
      },
      Colour {
        name: "Racing Green",
        hex: "232F2C",
        rgb: &[35, 47, 44],
        hsl: &[116, 37, 41],
        shade: "Green"
      },
      Colour {
        name: "Radical Red",
        hex: "FF355E",
        rgb: &[255, 53, 94],
        hsl: &[-8, 254, 154],
        shade: "Red"
      },
      Colour {
        name: "Raffia",
        hex: "DCC6A0",
        rgb: &[220, 198, 160],
        hsl: &[26, 117, 190],
        shade: "Yellow"
      },
      Colour {
        name: "Rain Forest",
        hex: "667028",
        rgb: &[102, 112, 40],
        hsl: &[48, 120, 76],
        shade: "Green"
      },
      Colour {
        name: "Rainee",
        hex: "B3C1B1",
        rgb: &[179, 193, 177],
        hsl: &[79, 29, 185],
        shade: "Green"
      },
      Colour {
        name: "Rajah",
        hex: "FCAE60",
        rgb: &[252, 174, 96],
        hsl: &[21, 245, 174],
        shade: "Orange"
      },
      Colour {
        name: "Rangoon Green",
        hex: "2B2E25",
        rgb: &[43, 46, 37],
        hsl: &[56, 27, 41],
        shade: "Green"
      },
      Colour {
        name: "Raven",
        hex: "6F747B",
        rgb: &[111, 116, 123],
        hsl: &[152, 13, 117],
        shade: "Blue"
      },
      Colour {
        name: "Raw Sienna",
        hex: "D27D46",
        rgb: &[210, 125, 70],
        hsl: &[16, 155, 140],
        shade: "Brown"
      },
      Colour {
        name: "Raw Umber",
        hex: "734A12",
        rgb: &[115, 74, 18],
        hsl: &[24, 185, 66],
        shade: "Brown"
      },
      Colour {
        name: "Razzle Dazzle Rose",
        hex: "FF33CC",
        rgb: &[255, 51, 204],
        hsl: &[-31, 255, 153],
        shade: "Red"
      },
      Colour {
        name: "Razzmatazz",
        hex: "E30B5C",
        rgb: &[227, 11, 92],
        hsl: &[-15, 231, 118],
        shade: "Red"
      },
      Colour {
        name: "Rebel",
        hex: "453430",
        rgb: &[69, 52, 48],
        hsl: &[8, 45, 58],
        shade: "Brown"
      },
      Colour {
        name: "Red",
        hex: "FF0000",
        rgb: &[255, 0, 0],
        hsl: &[0, 255, 127],
        shade: "Red"
      },
      Colour {
        name: "Red Berry",
        hex: "701F28",
        rgb: &[112, 31, 40],
        hsl: &[-4, 144, 71],
        shade: "Red"
      },
      Colour {
        name: "Red Damask",
        hex: "CB6F4A",
        rgb: &[203, 111, 74],
        hsl: &[12, 141, 138],
        shade: "Orange"
      },
      Colour {
        name: "Red Devil",
        hex: "662A2C",
        rgb: &[102, 42, 44],
        hsl: &[-1, 106, 72],
        shade: "Red"
      },
      Colour {
        name: "Red Orange",
        hex: "FF3F34",
        rgb: &[255, 63, 52],
        hsl: &[2, 255, 153],
        shade: "Orange"
      },
      Colour {
        name: "Red Oxide",
        hex: "5D1F1E",
        rgb: &[93, 31, 30],
        hsl: &[0, 130, 61],
        shade: "Red"
      },
      Colour {
        name: "Red Robin",
        hex: "7D4138",
        rgb: &[125, 65, 56],
        hsl: &[5, 97, 90],
        shade: "Red"
      },
      Colour {
        name: "Red Stage",
        hex: "AD522E",
        rgb: &[173, 82, 46],
        hsl: &[12, 147, 109],
        shade: "Orange"
      },
      Colour {
        name: "Medium Red Violet",
        hex: "BB3385",
        rgb: &[187, 51, 133],
        hsl: &[-25, 145, 119],
        shade: "Violet"
      },
      Colour {
        name: "Redwood",
        hex: "5B342E",
        rgb: &[91, 52, 46],
        hsl: &[5, 83, 68],
        shade: "Red"
      },
      Colour {
        name: "Reef",
        hex: "D1EF9F",
        rgb: &[209, 239, 159],
        hsl: &[58, 182, 199],
        shade: "Green"
      },
      Colour {
        name: "Reef Gold",
        hex: "A98D36",
        rgb: &[169, 141, 54],
        hsl: &[32, 131, 111],
        shade: "Yellow"
      },
      Colour {
        name: "Regal Blue",
        hex: "203F58",
        rgb: &[32, 63, 88],
        hsl: &[146, 119, 60],
        shade: "Blue"
      },
      Colour {
        name: "Regent Grey",
        hex: "798488",
        rgb: &[121, 132, 136],
        hsl: &[138, 15, 128],
        shade: "Blue"
      },
      Colour {
        name: "Regent St Blue",
        hex: "A0CDD9",
        rgb: &[160, 205, 217],
        hsl: &[136, 109, 188],
        shade: "Blue"
      },
      Colour {
        name: "Remy",
        hex: "F6DEDA",
        rgb: &[246, 222, 218],
        hsl: &[6, 155, 232],
        shade: "Red"
      },
      Colour {
        name: "Reno Sand",
        hex: "B26E33",
        rgb: &[178, 110, 51],
        hsl: &[19, 141, 114],
        shade: "Orange"
      },
      Colour {
        name: "Resolution Blue",
        hex: "323F75",
        rgb: &[50, 63, 117],
        hsl: &[161, 102, 83],
        shade: "Blue"
      },
      Colour {
        name: "Revolver",
        hex: "37363F",
        rgb: &[55, 54, 63],
        hsl: &[174, 19, 58],
        shade: "Violet"
      },
      Colour {
        name: "Rhino",
        hex: "3D4653",
        rgb: &[61, 70, 83],
        hsl: &[152, 38, 72],
        shade: "Blue"
      },
      Colour {
        name: "Rice Cake",
        hex: "EFECDE",
        rgb: &[239, 236, 222],
        hsl: &[35, 88, 230],
        shade: "Green"
      },
      Colour {
        name: "Rice Flower",
        hex: "EFF5D1",
        rgb: &[239, 245, 209],
        hsl: &[49, 163, 227],
        shade: "Green"
      },
      Colour {
        name: "Rich Blue",
        hex: "5959AB",
        rgb: &[89, 89, 171],
        hsl: &[170, 83, 130],
        shade: "Blue"
      },
      Colour {
        name: "Rich Gold",
        hex: "A15226",
        rgb: &[161, 82, 38],
        hsl: &[15, 157, 99],
        shade: "Orange"
      },
      Colour {
        name: "Rio Grande",
        hex: "B7C61A",
        rgb: &[183, 198, 26],
        hsl: &[46, 195, 112],
        shade: "Green"
      },
      Colour {
        name: "Riptide",
        hex: "89D9C8",
        rgb: &[137, 217, 200],
        hsl: &[118, 130, 177],
        shade: "Green"
      },
      Colour {
        name: "River Bed",
        hex: "556061",
        rgb: &[85, 96, 97],
        hsl: &[131, 16, 90],
        shade: "Blue"
      },
      Colour {
        name: "Rob Roy",
        hex: "DDAD56",
        rgb: &[221, 173, 86],
        hsl: &[27, 169, 153],
        shade: "Yellow"
      },
      Colour {
        name: "Robin's Egg Blue",
        hex: "00CCCC",
        rgb: &[0, 204, 204],
        hsl: &[127, 255, 102],
        shade: "Blue"
      },
      Colour {
        name: "Rock",
        hex: "5A4D41",
        rgb: &[90, 77, 65],
        hsl: &[20, 41, 77],
        shade: "Brown"
      },
      Colour {
        name: "Rock Blue",
        hex: "93A2BA",
        rgb: &[147, 162, 186],
        hsl: &[153, 56, 166],
        shade: "Blue"
      },
      Colour {
        name: "Rock Spray",
        hex: "9D442D",
        rgb: &[157, 68, 45],
        hsl: &[8, 141, 101],
        shade: "Orange"
      },
      Colour {
        name: "Rodeo Dust",
        hex: "C7A384",
        rgb: &[199, 163, 132],
        hsl: &[19, 95, 165],
        shade: "Brown"
      },
      Colour {
        name: "Rolling Stone",
        hex: "6D7876",
        rgb: &[109, 120, 118],
        hsl: &[119, 12, 114],
        shade: "Green"
      },
      Colour {
        name: "Roman",
        hex: "D8625B",
        rgb: &[216, 98, 91],
        hsl: &[2, 157, 153],
        shade: "Red"
      },
      Colour {
        name: "Roman Coffee",
        hex: "7D6757",
        rgb: &[125, 103, 87],
        hsl: &[17, 45, 105],
        shade: "Brown"
      },
      Colour {
        name: "Romance",
        hex: "F4F0E6",
        rgb: &[244, 240, 230],
        hsl: &[30, 99, 237],
        shade: "Grey"
      },
      Colour {
        name: "Romantic",
        hex: "FFC69E",
        rgb: &[255, 198, 158],
        hsl: &[17, 255, 206],
        shade: "Orange"
      },
      Colour {
        name: "Ronchi",
        hex: "EAB852",
        rgb: &[234, 184, 82],
        hsl: &[28, 199, 158],
        shade: "Yellow"
      },
      Colour {
        name: "Roof Terracotta",
        hex: "A14743",
        rgb: &[161, 71, 67],
        hsl: &[1, 105, 113],
        shade: "Red"
      },
      Colour {
        name: "Rope",
        hex: "8E593C",
        rgb: &[142, 89, 60],
        hsl: &[15, 103, 101],
        shade: "Orange"
      },
      Colour {
        name: "Rose",
        hex: "D3A194",
        rgb: &[211, 161, 148],
        hsl: &[8, 106, 179],
        shade: "Red"
      },
      Colour {
        name: "Rose Bud",
        hex: "FEAB9A",
        rgb: &[254, 171, 154],
        hsl: &[7, 250, 204],
        shade: "Red"
      },
      Colour {
        name: "Rose Bud Cherry",
        hex: "8A2D52",
        rgb: &[138, 45, 82],
        hsl: &[-16, 129, 91],
        shade: "Red"
      },
      Colour {
        name: "Rose Of Sharon",
        hex: "AC512D",
        rgb: &[172, 81, 45],
        hsl: &[12, 149, 108],
        shade: "Orange"
      },
      Colour {
        name: "Rose Taupe",
        hex: "905D5D",
        rgb: &[144, 93, 93],
        hsl: &[0, 54, 118],
        shade: "Violet"
      },
      Colour {
        name: "Rose White",
        hex: "FBEEE8",
        rgb: &[251, 238, 232],
        hsl: &[13, 179, 241],
        shade: "Red"
      },
      Colour {
        name: "Rosy Brown",
        hex: "BC8F8F",
        rgb: &[188, 143, 143],
        hsl: &[0, 64, 165],
        shade: "Brown"
      },
      Colour {
        name: "Roti",
        hex: "B69642",
        rgb: &[182, 150, 66],
        hsl: &[30, 119, 124],
        shade: "Yellow"
      },
      Colour {
        name: "Rouge",
        hex: "A94064",
        rgb: &[169, 64, 100],
        hsl: &[-14, 114, 116],
        shade: "Red"
      },
      Colour {
        name: "Royal Blue",
        hex: "4169E1",
        rgb: &[65, 105, 225],
        hsl: &[159, 185, 145],
        shade: "Blue"
      },
      Colour {
        name: "Royal Heath",
        hex: "B54B73",
        rgb: &[181, 75, 115],
        hsl: &[-16, 106, 128],
        shade: "Red"
      },
      Colour {
        name: "Royal Purple",
        hex: "6B3FA0",
        rgb: &[107, 63, 160],
        hsl: &[189, 110, 111],
        shade: "Violet"
      },
      Colour {
        name: "Ruby",
        hex: "E0115F",
        rgb: &[224, 17, 95],
        hsl: &[-16, 219, 120],
        shade: "Red"
      },
      Colour {
        name: "Rum",
        hex: "716675",
        rgb: &[113, 102, 117],
        hsl: &[201, 17, 109],
        shade: "Violet"
      },
      Colour {
        name: "Rum Swizzle",
        hex: "F1EDD4",
        rgb: &[241, 237, 212],
        hsl: &[36, 129, 226],
        shade: "Green"
      },
      Colour {
        name: "Russet",
        hex: "80461B",
        rgb: &[128, 70, 27],
        hsl: &[18, 166, 77],
        shade: "Brown"
      },
      Colour {
        name: "Russett",
        hex: "7D655C",
        rgb: &[125, 101, 92],
        hsl: &[11, 38, 108],
        shade: "Brown"
      },
      Colour {
        name: "Rust",
        hex: "B7410E",
        rgb: &[183, 65, 14],
        hsl: &[12, 218, 98],
        shade: "Red"
      },
      Colour {
        name: "Rustic Red",
        hex: "3A181A",
        rgb: &[58, 24, 26],
        hsl: &[-2, 105, 40],
        shade: "Red"
      },
      Colour {
        name: "Rusty Nail",
        hex: "8D5F2C",
        rgb: &[141, 95, 44],
        hsl: &[22, 133, 92],
        shade: "Orange"
      },
      Colour {
        name: "Saddle",
        hex: "5D4E46",
        rgb: &[93, 78, 70],
        hsl: &[14, 35, 81],
        shade: "Brown"
      },
      Colour {
        name: "Saddle Brown",
        hex: "8B4513",
        rgb: &[139, 69, 19],
        hsl: &[17, 193, 78],
        shade: "Brown"
      },
      Colour {
        name: "Safety Orange",
        hex: "FF6600",
        rgb: &[255, 102, 0],
        hsl: &[17, 255, 127],
        shade: "Orange"
      },
      Colour {
        name: "Saffron",
        hex: "F4C430",
        rgb: &[244, 196, 48],
        hsl: &[32, 229, 146],
        shade: "Yellow"
      },
      Colour {
        name: "Sage",
        hex: "989F7A",
        rgb: &[152, 159, 122],
        hsl: &[50, 41, 140],
        shade: "Green"
      },
      Colour {
        name: "Sahara",
        hex: "B79826",
        rgb: &[183, 152, 38],
        hsl: &[33, 167, 110],
        shade: "Yellow"
      },
      Colour {
        name: "Sail",
        hex: "A5CEEC",
        rgb: &[165, 206, 236],
        hsl: &[145, 166, 200],
        shade: "Blue"
      },
      Colour {
        name: "Salem",
        hex: "177B4D",
        rgb: &[23, 123, 77],
        hsl: &[107, 174, 73],
        shade: "Green"
      },
      Colour {
        name: "Salmon",
        hex: "FA8072",
        rgb: &[250, 128, 114],
        hsl: &[4, 237, 182],
        shade: "Red"
      },
      Colour {
        name: "Salomie",
        hex: "FFD67B",
        rgb: &[255, 214, 123],
        hsl: &[29, 255, 189],
        shade: "Yellow"
      },
      Colour {
        name: "Salt Box",
        hex: "696268",
        rgb: &[105, 98, 104],
        hsl: &[-36, 8, 101],
        shade: "Violet"
      },
      Colour {
        name: "Saltpan",
        hex: "EEF3E5",
        rgb: &[238, 243, 229],
        hsl: &[57, 93, 236],
        shade: "Grey"
      },
      Colour {
        name: "Sambuca",
        hex: "3B2E25",
        rgb: &[59, 46, 37],
        hsl: &[17, 58, 48],
        shade: "Brown"
      },
      Colour {
        name: "San Felix",
        hex: "2C6E31",
        rgb: &[44, 110, 49],
        hsl: &[88, 109, 77],
        shade: "Green"
      },
      Colour {
        name: "San Juan",
        hex: "445761",
        rgb: &[68, 87, 97],
        hsl: &[142, 44, 82],
        shade: "Blue"
      },
      Colour {
        name: "San Marino",
        hex: "4E6C9D",
        rgb: &[78, 108, 157],
        hsl: &[153, 85, 117],
        shade: "Blue"
      },
      Colour {
        name: "Sand Dune",
        hex: "867665",
        rgb: &[134, 118, 101],
        hsl: &[21, 35, 117],
        shade: "Brown"
      },
      Colour {
        name: "Sandal",
        hex: "A3876A",
        rgb: &[163, 135, 106],
        hsl: &[21, 60, 134],
        shade: "Brown"
      },
      Colour {
        name: "Sandrift",
        hex: "AF937D",
        rgb: &[175, 147, 125],
        hsl: &[18, 60, 150],
        shade: "Brown"
      },
      Colour {
        name: "Sandstone",
        hex: "786D5F",
        rgb: &[120, 109, 95],
        hsl: &[23, 29, 107],
        shade: "Brown"
      },
      Colour {
        name: "Sandwisp",
        hex: "DECB81",
        rgb: &[222, 203, 129],
        hsl: &[33, 149, 175],
        shade: "Yellow"
      },
      Colour {
        name: "Sandy Beach",
        hex: "FEDBB7",
        rgb: &[254, 219, 183],
        hsl: &[21, 248, 218],
        shade: "Orange"
      },
      Colour {
        name: "Sandy Brown",
        hex: "F4A460",
        rgb: &[244, 164, 96],
        hsl: &[19, 222, 170],
        shade: "Brown"
      },
      Colour {
        name: "Sangria",
        hex: "92000A",
        rgb: &[146, 0, 10],
        hsl: &[-2, 255, 73],
        shade: "Red"
      },
      Colour {
        name: "Sanguine Brown",
        hex: "6C3736",
        rgb: &[108, 55, 54],
        hsl: &[0, 85, 81],
        shade: "Red"
      },
      Colour {
        name: "Santas Grey",
        hex: "9998A7",
        rgb: &[153, 152, 167],
        hsl: &[172, 20, 159],
        shade: "Blue"
      },
      Colour {
        name: "Sante Fe",
        hex: "A96A50",
        rgb: &[169, 106, 80],
        hsl: &[12, 91, 124],
        shade: "Orange"
      },
      Colour {
        name: "Sapling",
        hex: "E1D5A6",
        rgb: &[225, 213, 166],
        hsl: &[33, 126, 195],
        shade: "Yellow"
      },
      Colour {
        name: "Sapphire",
        hex: "082567",
        rgb: &[8, 37, 103],
        hsl: &[157, 218, 55],
        shade: "Blue"
      },
      Colour {
        name: "Saratoga",
        hex: "555B2C",
        rgb: &[85, 91, 44],
        hsl: &[47, 88, 67],
        shade: "Green"
      },
      Colour {
        name: "Sauvignon",
        hex: "F4EAE4",
        rgb: &[244, 234, 228],
        hsl: &[15, 107, 236],
        shade: "Red"
      },
      Colour {
        name: "Sazerac",
        hex: "F5DEC4",
        rgb: &[245, 222, 196],
        hsl: &[22, 181, 220],
        shade: "Orange"
      },
      Colour {
        name: "Scampi",
        hex: "6F63A0",
        rgb: &[111, 99, 160],
        hsl: &[178, 61, 129],
        shade: "Violet"
      },
      Colour {
        name: "Scandal",
        hex: "ADD9D1",
        rgb: &[173, 217, 209],
        hsl: &[119, 93, 195],
        shade: "Green"
      },
      Colour {
        name: "Scarlet",
        hex: "FF2400",
        rgb: &[255, 36, 0],
        hsl: &[6, 255, 127],
        shade: "Red"
      },
      Colour {
        name: "Scarlet Gum",
        hex: "4A2D57",
        rgb: &[74, 45, 87],
        hsl: &[199, 81, 66],
        shade: "Violet"
      },
      Colour {
        name: "Scarlett",
        hex: "7E2530",
        rgb: &[126, 37, 48],
        hsl: &[-5, 139, 81],
        shade: "Red"
      },
      Colour {
        name: "Scarpa Flow",
        hex: "6B6A6C",
        rgb: &[107, 106, 108],
        hsl: &[191, 2, 107],
        shade: "Grey"
      },
      Colour {
        name: "Schist",
        hex: "87876F",
        rgb: &[135, 135, 111],
        hsl: &[42, 24, 123],
        shade: "Green"
      },
      Colour {
        name: "School Bus Yellow",
        hex: "FFD800",
        rgb: &[255, 216, 0],
        hsl: &[36, 255, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Schooner",
        hex: "8D8478",
        rgb: &[141, 132, 120],
        hsl: &[24, 21, 130],
        shade: "Brown"
      },
      Colour {
        name: "Scooter",
        hex: "308EA0",
        rgb: &[48, 142, 160],
        hsl: &[134, 137, 104],
        shade: "Blue"
      },
      Colour {
        name: "Scorpion",
        hex: "6A6466",
        rgb: &[106, 100, 102],
        hsl: &[-14, 7, 103],
        shade: "Grey"
      },
      Colour {
        name: "Scotch Mist",
        hex: "EEE7C8",
        rgb: &[238, 231, 200],
        hsl: &[34, 134, 219],
        shade: "Yellow"
      },
      Colour {
        name: "Screamin' Green",
        hex: "66FF66",
        rgb: &[102, 255, 102],
        hsl: &[85, 254, 178],
        shade: "Green"
      },
      Colour {
        name: "Scrub",
        hex: "3D4031",
        rgb: &[61, 64, 49],
        hsl: &[50, 33, 56],
        shade: "Green"
      },
      Colour {
        name: "Sea Buckthorn",
        hex: "EF9548",
        rgb: &[239, 149, 72],
        hsl: &[19, 213, 155],
        shade: "Orange"
      },
      Colour {
        name: "Sea Fog",
        hex: "DFDDD6",
        rgb: &[223, 221, 214],
        hsl: &[33, 31, 218],
        shade: "Grey"
      },
      Colour {
        name: "Sea Green",
        hex: "2E8B57",
        rgb: &[46, 139, 87],
        hsl: &[103, 128, 92],
        shade: "Green"
      },
      Colour {
        name: "Sea Mist",
        hex: "C2D5C4",
        rgb: &[194, 213, 196],
        hsl: &[89, 47, 203],
        shade: "Green"
      },
      Colour {
        name: "Sea Nymph",
        hex: "8AAEA4",
        rgb: &[138, 174, 164],
        hsl: &[115, 46, 156],
        shade: "Green"
      },
      Colour {
        name: "Sea Pink",
        hex: "DB817E",
        rgb: &[219, 129, 126],
        hsl: &[1, 143, 172],
        shade: "Red"
      },
      Colour {
        name: "Seagull",
        hex: "77B7D0",
        rgb: &[119, 183, 208],
        hsl: &[139, 124, 163],
        shade: "Blue"
      },
      Colour {
        name: "Seal Brown",
        hex: "321414",
        rgb: &[50, 20, 20],
        hsl: &[0, 109, 35],
        shade: "Brown"
      },
      Colour {
        name: "Seance",
        hex: "69326E",
        rgb: &[105, 50, 110],
        hsl: &[208, 95, 80],
        shade: "Violet"
      },
      Colour {
        name: "Seashell",
        hex: "FFF5EE",
        rgb: &[255, 245, 238],
        hsl: &[17, 255, 246],
        shade: "White"
      },
      Colour {
        name: "Seaweed",
        hex: "37412A",
        rgb: &[55, 65, 42],
        hsl: &[60, 54, 53],
        shade: "Green"
      },
      Colour {
        name: "Selago",
        hex: "E6DFE7",
        rgb: &[230, 223, 231],
        hsl: &[207, 36, 227],
        shade: "Violet"
      },
      Colour {
        name: "Selective Yellow",
        hex: "FFBA00",
        rgb: &[255, 186, 0],
        hsl: &[31, 255, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Semi-Sweet Chocolate",
        hex: "6B4226",
        rgb: &[107, 66, 38],
        hsl: &[17, 121, 72],
        shade: "Brown"
      },
      Colour {
        name: "Sepia",
        hex: "9E5B40",
        rgb: &[158, 91, 64],
        hsl: &[12, 107, 111],
        shade: "Brown"
      },
      Colour {
        name: "Serenade",
        hex: "FCE9D7",
        rgb: &[252, 233, 215],
        hsl: &[20, 219, 233],
        shade: "Orange"
      },
      Colour {
        name: "Shadow",
        hex: "837050",
        rgb: &[131, 112, 80],
        hsl: &[26, 61, 105],
        shade: "Green"
      },
      Colour {
        name: "Shadow Green",
        hex: "9AC0B6",
        rgb: &[154, 192, 182],
        hsl: &[116, 59, 173],
        shade: "Green"
      },
      Colour {
        name: "Shady Lady",
        hex: "9F9B9D",
        rgb: &[159, 155, 157],
        hsl: &[-21, 5, 157],
        shade: "Grey"
      },
      Colour {
        name: "Shakespeare",
        hex: "609AB8",
        rgb: &[96, 154, 184],
        hsl: &[141, 97, 140],
        shade: "Blue"
      },
      Colour {
        name: "Shalimar",
        hex: "F8F6A8",
        rgb: &[248, 246, 168],
        hsl: &[41, 217, 208],
        shade: "Green"
      },
      Colour {
        name: "Shamrock",
        hex: "33CC99",
        rgb: &[51, 204, 153],
        hsl: &[113, 153, 127],
        shade: "Green"
      },
      Colour {
        name: "Shamrock Green",
        hex: "009E60",
        rgb: &[0, 158, 96],
        hsl: &[110, 255, 79],
        shade: "Green"
      },
      Colour {
        name: "Shark",
        hex: "34363A",
        rgb: &[52, 54, 58],
        hsl: &[155, 13, 55],
        shade: "Grey"
      },
      Colour {
        name: "Sherpa Blue",
        hex: "00494E",
        rgb: &[0, 73, 78],
        hsl: &[130, 255, 39],
        shade: "Green"
      },
      Colour {
        name: "Sherwood Green",
        hex: "1B4636",
        rgb: &[27, 70, 54],
        hsl: &[111, 113, 48],
        shade: "Green"
      },
      Colour {
        name: "Shilo",
        hex: "E6B2A6",
        rgb: &[230, 178, 166],
        hsl: &[7, 143, 198],
        shade: "Red"
      },
      Colour {
        name: "Shingle Fawn",
        hex: "745937",
        rgb: &[116, 89, 55],
        hsl: &[23, 90, 85],
        shade: "Brown"
      },
      Colour {
        name: "Ship Cove",
        hex: "7988AB",
        rgb: &[121, 136, 171],
        hsl: &[157, 58, 146],
        shade: "Blue"
      },
      Colour {
        name: "Ship Grey",
        hex: "4E4E4C",
        rgb: &[78, 78, 76],
        hsl: &[42, 3, 77],
        shade: "Grey"
      },
      Colour {
        name: "Shiraz",
        hex: "842833",
        rgb: &[132, 40, 51],
        hsl: &[-5, 136, 86],
        shade: "Red"
      },
      Colour {
        name: "Shocking",
        hex: "E899BE",
        rgb: &[232, 153, 190],
        hsl: &[-19, 161, 192],
        shade: "Violet"
      },
      Colour {
        name: "Shocking Pink",
        hex: "FC0FC0",
        rgb: &[252, 15, 192],
        hsl: &[-31, 248, 133],
        shade: "Red"
      },
      Colour {
        name: "Shuttle Grey",
        hex: "61666B",
        rgb: &[97, 102, 107],
        hsl: &[148, 12, 102],
        shade: "Grey"
      },
      Colour {
        name: "Siam",
        hex: "686B50",
        rgb: &[104, 107, 80],
        hsl: &[47, 36, 93],
        shade: "Green"
      },
      Colour {
        name: "Sidecar",
        hex: "E9D9A9",
        rgb: &[233, 217, 169],
        hsl: &[31, 151, 201],
        shade: "Yellow"
      },
      Colour {
        name: "Sienna",
        hex: "A0522D",
        rgb: &[160, 82, 45],
        hsl: &[13, 143, 102],
        shade: "Brown"
      },
      Colour {
        name: "Silk",
        hex: "BBADA1",
        rgb: &[187, 173, 161],
        hsl: &[19, 40, 174],
        shade: "Brown"
      },
      Colour {
        name: "Silver",
        hex: "C0C0C0",
        rgb: &[192, 192, 192],
        hsl: &[0, 0, 192],
        shade: "Grey"
      },
      Colour {
        name: "Silver Chalice",
        hex: "ACAEA9",
        rgb: &[172, 174, 169],
        hsl: &[59, 7, 171],
        shade: "Grey"
      },
      Colour {
        name: "Silver Sand",
        hex: "BEBDB6",
        rgb: &[190, 189, 182],
        hsl: &[37, 14, 186],
        shade: "Grey"
      },
      Colour {
        name: "Silver Tree",
        hex: "67BE90",
        rgb: &[103, 190, 144],
        hsl: &[105, 102, 146],
        shade: "Green"
      },
      Colour {
        name: "Sinbad",
        hex: "A6D5D0",
        rgb: &[166, 213, 208],
        hsl: &[122, 91, 189],
        shade: "Green"
      },
      Colour {
        name: "Siren",
        hex: "69293B",
        rgb: &[105, 41, 59],
        hsl: &[-11, 111, 73],
        shade: "Red"
      },
      Colour {
        name: "Sirocco",
        hex: "68766E",
        rgb: &[104, 118, 110],
        hsl: &[103, 16, 111],
        shade: "Green"
      },
      Colour {
        name: "Sisal",
        hex: "C5BAA0",
        rgb: &[197, 186, 160],
        hsl: &[29, 61, 178],
        shade: "Yellow"
      },
      Colour {
        name: "Skeptic",
        hex: "9DB4AA",
        rgb: &[157, 180, 170],
        hsl: &[109, 33, 168],
        shade: "Green"
      },
      Colour {
        name: "Sky Blue",
        hex: "87CEEB",
        rgb: &[135, 206, 235],
        hsl: &[139, 182, 185],
        shade: "Blue"
      },
      Colour {
        name: "Slate Blue",
        hex: "6A5ACD",
        rgb: &[106, 90, 205],
        hsl: &[175, 136, 147],
        shade: "Blue"
      },
      Colour {
        name: "Slate Grey",
        hex: "708090",
        rgb: &[112, 128, 144],
        hsl: &[148, 32, 128],
        shade: "Grey"
      },
      Colour {
        name: "Slugger",
        hex: "42342B",
        rgb: &[66, 52, 43],
        hsl: &[16, 53, 54],
        shade: "Brown"
      },
      Colour {
        name: "Smalt",
        hex: "003399",
        rgb: &[0, 51, 153],
        hsl: &[155, 255, 76],
        shade: "Blue"
      },
      Colour {
        name: "Smalt Blue",
        hex: "496267",
        rgb: &[73, 98, 103],
        hsl: &[134, 43, 88],
        shade: "Blue"
      },
      Colour {
        name: "Smoke Tree",
        hex: "BB5F34",
        rgb: &[187, 95, 52],
        hsl: &[13, 144, 119],
        shade: "Orange"
      },
      Colour {
        name: "Smoky",
        hex: "605D6B",
        rgb: &[96, 93, 107],
        hsl: &[179, 17, 100],
        shade: "Violet"
      },
      Colour {
        name: "Snow",
        hex: "FFFAFA",
        rgb: &[255, 250, 250],
        hsl: &[0, 255, 252],
        shade: "White"
      },
      Colour {
        name: "Snow Drift",
        hex: "E3E3DC",
        rgb: &[227, 227, 220],
        hsl: &[42, 28, 223],
        shade: "Grey"
      },
      Colour {
        name: "Snow Flurry",
        hex: "EAF7C9",
        rgb: &[234, 247, 201],
        hsl: &[54, 189, 224],
        shade: "Green"
      },
      Colour {
        name: "Snowy Mint",
        hex: "D6F0CD",
        rgb: &[214, 240, 205],
        hsl: &[74, 137, 222],
        shade: "Green"
      },
      Colour {
        name: "Snuff",
        hex: "E4D7E5",
        rgb: &[228, 215, 229],
        hsl: &[209, 54, 222],
        shade: "Violet"
      },
      Colour {
        name: "Soapstone",
        hex: "ECE5DA",
        rgb: &[236, 229, 218],
        hsl: &[25, 81, 227],
        shade: "Grey"
      },
      Colour {
        name: "Soft Amber",
        hex: "CFBEA5",
        rgb: &[207, 190, 165],
        hsl: &[25, 77, 186],
        shade: "Yellow"
      },
      Colour {
        name: "Soft Peach",
        hex: "EEDFDE",
        rgb: &[238, 223, 222],
        hsl: &[2, 81, 230],
        shade: "Red"
      },
      Colour {
        name: "Solid Pink",
        hex: "85494C",
        rgb: &[133, 73, 76],
        hsl: &[-2, 74, 103],
        shade: "Red"
      },
      Colour {
        name: "Solitaire",
        hex: "EADAC2",
        rgb: &[234, 218, 194],
        hsl: &[25, 124, 213],
        shade: "Yellow"
      },
      Colour {
        name: "Solitude",
        hex: "E9ECF1",
        rgb: &[233, 236, 241],
        hsl: &[154, 56, 237],
        shade: "Blue"
      },
      Colour {
        name: "Sorbus",
        hex: "DD6B38",
        rgb: &[221, 107, 56],
        hsl: &[13, 180, 138],
        shade: "Orange"
      },
      Colour {
        name: "Sorrell Brown",
        hex: "9D7F61",
        rgb: &[157, 127, 97],
        hsl: &[21, 60, 127],
        shade: "Brown"
      },
      Colour {
        name: "Sour Dough",
        hex: "C9B59A",
        rgb: &[201, 181, 154],
        hsl: &[24, 77, 177],
        shade: "Brown"
      },
      Colour {
        name: "Soya Bean",
        hex: "6F634B",
        rgb: &[111, 99, 75],
        hsl: &[28, 49, 93],
        shade: "Brown"
      },
      Colour {
        name: "Space Shuttle",
        hex: "4B433B",
        rgb: &[75, 67, 59],
        hsl: &[21, 30, 67],
        shade: "Brown"
      },
      Colour {
        name: "Spanish Green",
        hex: "7B8976",
        rgb: &[123, 137, 118],
        hsl: &[73, 18, 127],
        shade: "Green"
      },
      Colour {
        name: "Spanish White",
        hex: "DED1B7",
        rgb: &[222, 209, 183],
        hsl: &[28, 94, 202],
        shade: "Yellow"
      },
      Colour {
        name: "Spectra",
        hex: "375D4F",
        rgb: &[55, 93, 79],
        hsl: &[111, 65, 73],
        shade: "Green"
      },
      Colour {
        name: "Spice",
        hex: "6C4F3F",
        rgb: &[108, 79, 63],
        hsl: &[15, 67, 85],
        shade: "Brown"
      },
      Colour {
        name: "Spicy Mix",
        hex: "8B5F4D",
        rgb: &[139, 95, 77],
        hsl: &[12, 73, 107],
        shade: "Brown"
      },
      Colour {
        name: "Spicy Pink",
        hex: "FF1CAE",
        rgb: &[255, 28, 174],
        hsl: &[-27, 255, 141],
        shade: "Red"
      },
      Colour {
        name: "Spindle",
        hex: "B3C4D8",
        rgb: &[179, 196, 216],
        hsl: &[150, 82, 197],
        shade: "Blue"
      },
      Colour {
        name: "Splash",
        hex: "F1D79E",
        rgb: &[241, 215, 158],
        hsl: &[29, 190, 199],
        shade: "Yellow"
      },
      Colour {
        name: "Spray",
        hex: "7ECDDD",
        rgb: &[126, 205, 221],
        hsl: &[134, 148, 173],
        shade: "Blue"
      },
      Colour {
        name: "Spring Bud",
        hex: "A7FC00",
        rgb: &[167, 252, 0],
        hsl: &[56, 255, 126],
        shade: "Green"
      },
      Colour {
        name: "Spring Green",
        hex: "00FF7F",
        rgb: &[0, 255, 127],
        hsl: &[106, 255, 127],
        shade: "Green"
      },
      Colour {
        name: "Spring Rain",
        hex: "A3BD9C",
        rgb: &[163, 189, 156],
        hsl: &[75, 51, 172],
        shade: "Green"
      },
      Colour {
        name: "Spring Sun",
        hex: "F1F1C6",
        rgb: &[241, 241, 198],
        hsl: &[42, 154, 219],
        shade: "Green"
      },
      Colour {
        name: "Spring Wood",
        hex: "E9E1D9",
        rgb: &[233, 225, 217],
        hsl: &[21, 67, 225],
        shade: "Grey"
      },
      Colour {
        name: "Sprout",
        hex: "B8CA9D",
        rgb: &[184, 202, 157],
        hsl: &[59, 75, 179],
        shade: "Green"
      },
      Colour {
        name: "Spun Pearl",
        hex: "A2A1AC",
        rgb: &[162, 161, 172],
        hsl: &[173, 15, 166],
        shade: "Blue"
      },
      Colour {
        name: "Squirrel",
        hex: "8F7D6B",
        rgb: &[143, 125, 107],
        hsl: &[21, 36, 125],
        shade: "Brown"
      },
      Colour {
        name: "St Tropaz",
        hex: "325482",
        rgb: &[50, 84, 130],
        hsl: &[151, 113, 89],
        shade: "Blue"
      },
      Colour {
        name: "Stack",
        hex: "858885",
        rgb: &[133, 136, 133],
        hsl: &[85, 3, 134],
        shade: "Grey"
      },
      Colour {
        name: "Star Dust",
        hex: "A0A197",
        rgb: &[160, 161, 151],
        hsl: &[46, 12, 156],
        shade: "Grey"
      },
      Colour {
        name: "Stark White",
        hex: "D2C6B6",
        rgb: &[210, 198, 182],
        hsl: &[24, 60, 195],
        shade: "Yellow"
      },
      Colour {
        name: "Starship",
        hex: "E3DD39",
        rgb: &[227, 221, 57],
        hsl: &[41, 191, 142],
        shade: "Green"
      },
      Colour {
        name: "Steel Blue",
        hex: "4682B4",
        rgb: &[70, 130, 180],
        hsl: &[146, 112, 125],
        shade: "Blue"
      },
      Colour {
        name: "Steel Grey",
        hex: "43464B",
        rgb: &[67, 70, 75],
        hsl: &[154, 14, 71],
        shade: "Grey"
      },
      Colour {
        name: "Stiletto",
        hex: "833D3E",
        rgb: &[131, 61, 62],
        hsl: &[0, 92, 96],
        shade: "Red"
      },
      Colour {
        name: "Stonewall",
        hex: "807661",
        rgb: &[128, 118, 97],
        hsl: &[28, 35, 112],
        shade: "Yellow"
      },
      Colour {
        name: "Storm Dust",
        hex: "65645F",
        rgb: &[101, 100, 95],
        hsl: &[35, 7, 97],
        shade: "Grey"
      },
      Colour {
        name: "Storm Grey",
        hex: "747880",
        rgb: &[116, 120, 128],
        hsl: &[155, 12, 121],
        shade: "Blue"
      },
      Colour {
        name: "Straw",
        hex: "DABE82",
        rgb: &[218, 190, 130],
        hsl: &[28, 138, 174],
        shade: "Yellow"
      },
      Colour {
        name: "Strikemaster",
        hex: "946A81",
        rgb: &[148, 106, 129],
        hsl: &[-23, 42, 127],
        shade: "Violet"
      },
      Colour {
        name: "Stromboli",
        hex: "406356",
        rgb: &[64, 99, 86],
        hsl: &[111, 54, 81],
        shade: "Green"
      },
      Colour {
        name: "Studio",
        hex: "724AA1",
        rgb: &[114, 74, 161],
        hsl: &[189, 94, 117],
        shade: "Violet"
      },
      Colour {
        name: "Submarine",
        hex: "8C9C9C",
        rgb: &[140, 156, 156],
        hsl: &[127, 19, 148],
        shade: "Blue"
      },
      Colour {
        name: "Sugar Cane",
        hex: "EEEFDF",
        rgb: &[238, 239, 223],
        hsl: &[45, 85, 231],
        shade: "Green"
      },
      Colour {
        name: "Sulu",
        hex: "C6EA80",
        rgb: &[198, 234, 128],
        hsl: &[56, 182, 180],
        shade: "Green"
      },
      Colour {
        name: "Summer Green",
        hex: "8FB69C",
        rgb: &[143, 182, 156],
        hsl: &[99, 53, 162],
        shade: "Green"
      },
      Colour {
        name: "Summer Sky",
        hex: "38B0DE",
        rgb: &[56, 176, 222],
        hsl: &[139, 182, 139],
        shade: "Blue"
      },
      Colour {
        name: "Sun",
        hex: "EF8E38",
        rgb: &[239, 142, 56],
        hsl: &[19, 217, 147],
        shade: "Orange"
      },
      Colour {
        name: "Sundance",
        hex: "C4AA4D",
        rgb: &[196, 170, 77],
        hsl: &[33, 128, 136],
        shade: "Yellow"
      },
      Colour {
        name: "Sundown",
        hex: "F8AFA9",
        rgb: &[248, 175, 169],
        hsl: &[3, 216, 208],
        shade: "Red"
      },
      Colour {
        name: "Sunflower",
        hex: "DAC01A",
        rgb: &[218, 192, 26],
        hsl: &[36, 200, 121],
        shade: "Yellow"
      },
      Colour {
        name: "Sunglo",
        hex: "C76155",
        rgb: &[199, 97, 85],
        hsl: &[4, 128, 142],
        shade: "Red"
      },
      Colour {
        name: "Sunglow",
        hex: "FFCC33",
        rgb: &[255, 204, 51],
        hsl: &[31, 255, 153],
        shade: "Orange"
      },
      Colour {
        name: "Sunset",
        hex: "C0514A",
        rgb: &[192, 81, 74],
        hsl: &[2, 123, 133],
        shade: "Red"
      },
      Colour {
        name: "Sunset Orange",
        hex: "FE4C40",
        rgb: &[254, 76, 64],
        hsl: &[2, 252, 159],
        shade: "Orange"
      },
      Colour {
        name: "Sunshade",
        hex: "FA9D49",
        rgb: &[250, 157, 73],
        hsl: &[20, 241, 161],
        shade: "Orange"
      },
      Colour {
        name: "Supernova",
        hex: "FFB437",
        rgb: &[255, 180, 55],
        hsl: &[26, 255, 155],
        shade: "Yellow"
      },
      Colour {
        name: "Surf",
        hex: "B8D4BB",
        rgb: &[184, 212, 187],
        hsl: &[89, 62, 198],
        shade: "Green"
      },
      Colour {
        name: "Surf Crest",
        hex: "C3D6BD",
        rgb: &[195, 214, 189],
        hsl: &[74, 59, 201],
        shade: "Green"
      },
      Colour {
        name: "Surfie Green",
        hex: "007B77",
        rgb: &[0, 123, 119],
        hsl: &[126, 255, 61],
        shade: "Green"
      },
      Colour {
        name: "Sushi",
        hex: "7C9F2F",
        rgb: &[124, 159, 47],
        hsl: &[55, 138, 103],
        shade: "Green"
      },
      Colour {
        name: "Suva Grey",
        hex: "8B8685",
        rgb: &[139, 134, 133],
        hsl: &[7, 6, 136],
        shade: "Grey"
      },
      Colour {
        name: "Swamp",
        hex: "252F2F",
        rgb: &[37, 47, 47],
        hsl: &[127, 30, 42],
        shade: "Green"
      },
      Colour {
        name: "Swans Down",
        hex: "DAE6DD",
        rgb: &[218, 230, 221],
        hsl: &[95, 49, 224],
        shade: "Grey"
      },
      Colour {
        name: "Sweet Corn",
        hex: "F9E176",
        rgb: &[249, 225, 118],
        hsl: &[34, 233, 183],
        shade: "Yellow"
      },
      Colour {
        name: "Sweet Pink",
        hex: "EE918D",
        rgb: &[238, 145, 141],
        hsl: &[1, 188, 189],
        shade: "Red"
      },
      Colour {
        name: "Swirl",
        hex: "D7CEC5",
        rgb: &[215, 206, 197],
        hsl: &[21, 46, 206],
        shade: "Grey"
      },
      Colour {
        name: "Swiss Coffee",
        hex: "DBD0CA",
        rgb: &[219, 208, 202],
        hsl: &[15, 48, 210],
        shade: "Grey"
      },
      Colour {
        name: "Tacao",
        hex: "F6AE78",
        rgb: &[246, 174, 120],
        hsl: &[18, 223, 183],
        shade: "Orange"
      },
      Colour {
        name: "Tacha",
        hex: "D2B960",
        rgb: &[210, 185, 96],
        hsl: &[33, 142, 153],
        shade: "Yellow"
      },
      Colour {
        name: "Tahiti Gold",
        hex: "DC722A",
        rgb: &[220, 114, 42],
        hsl: &[17, 183, 131],
        shade: "Orange"
      },
      Colour {
        name: "Tahuna Sands",
        hex: "D8CC9B",
        rgb: &[216, 204, 155],
        hsl: &[34, 111, 185],
        shade: "Yellow"
      },
      Colour {
        name: "Tall Poppy",
        hex: "853534",
        rgb: &[133, 53, 52],
        hsl: &[0, 111, 92],
        shade: "Red"
      },
      Colour {
        name: "Tallow",
        hex: "A39977",
        rgb: &[163, 153, 119],
        hsl: &[32, 49, 141],
        shade: "Yellow"
      },
      Colour {
        name: "Tamarillo",
        hex: "752B2F",
        rgb: &[117, 43, 47],
        hsl: &[-2, 117, 80],
        shade: "Red"
      },
      Colour {
        name: "Tan",
        hex: "D2B48C",
        rgb: &[210, 180, 140],
        hsl: &[24, 111, 175],
        shade: "Brown"
      },
      Colour {
        name: "Tana",
        hex: "B8B5A1",
        rgb: &[184, 181, 161],
        hsl: &[36, 35, 172],
        shade: "Green"
      },
      Colour {
        name: "Tangaroa",
        hex: "1E2F3C",
        rgb: &[30, 47, 60],
        hsl: &[145, 85, 44],
        shade: "Blue"
      },
      Colour {
        name: "Tangerine",
        hex: "F28500",
        rgb: &[242, 133, 0],
        hsl: &[23, 255, 121],
        shade: "Orange"
      },
      Colour {
        name: "Tangerine Yellow",
        hex: "FFCC00",
        rgb: &[255, 204, 0],
        hsl: &[34, 255, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Tango",
        hex: "D46F31",
        rgb: &[212, 111, 49],
        hsl: &[16, 166, 130],
        shade: "Orange"
      },
      Colour {
        name: "Tapa",
        hex: "7C7C72",
        rgb: &[124, 124, 114],
        hsl: &[42, 10, 119],
        shade: "Green"
      },
      Colour {
        name: "Tapestry",
        hex: "B37084",
        rgb: &[179, 112, 132],
        hsl: &[-12, 78, 145],
        shade: "Red"
      },
      Colour {
        name: "Tara",
        hex: "DEF1DD",
        rgb: &[222, 241, 221],
        hsl: &[82, 106, 231],
        shade: "Green"
      },
      Colour {
        name: "Tarawera",
        hex: "253C48",
        rgb: &[37, 60, 72],
        hsl: &[142, 81, 54],
        shade: "Blue"
      },
      Colour {
        name: "Tasman",
        hex: "BAC0B3",
        rgb: &[186, 192, 179],
        hsl: &[62, 23, 185],
        shade: "Grey"
      },
      Colour {
        name: "Taupe",
        hex: "483C32",
        rgb: &[72, 60, 50],
        hsl: &[19, 45, 60],
        shade: "Grey"
      },
      Colour {
        name: "Taupe Grey",
        hex: "8B8589",
        rgb: &[139, 133, 137],
        hsl: &[-28, 6, 136],
        shade: "Grey"
      },
      Colour {
        name: "Tawny Port",
        hex: "643A48",
        rgb: &[100, 58, 72],
        hsl: &[-14, 67, 79],
        shade: "Red"
      },
      Colour {
        name: "Tax Break",
        hex: "496569",
        rgb: &[73, 101, 105],
        hsl: &[132, 45, 89],
        shade: "Blue"
      },
      Colour {
        name: "Te Papa Green",
        hex: "2B4B40",
        rgb: &[43, 75, 64],
        hsl: &[112, 69, 59],
        shade: "Green"
      },
      Colour {
        name: "Tea",
        hex: "BFB5A2",
        rgb: &[191, 181, 162],
        hsl: &[27, 47, 176],
        shade: "Yellow"
      },
      Colour {
        name: "Tea Green",
        hex: "D0F0C0",
        rgb: &[208, 240, 192],
        hsl: &[70, 156, 216],
        shade: "Green"
      },
      Colour {
        name: "Tea Rose",
        hex: "F883C2",
        rgb: &[248, 131, 194],
        hsl: &[-22, 227, 189],
        shade: "Orange"
      },
      Colour {
        name: "Teak",
        hex: "AB8953",
        rgb: &[171, 137, 83],
        hsl: &[26, 88, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Teal",
        hex: "008080",
        rgb: &[0, 128, 128],
        hsl: &[127, 255, 64],
        shade: "Blue"
      },
      Colour {
        name: "Teal Blue",
        hex: "254855",
        rgb: &[37, 72, 85],
        hsl: &[139, 100, 60],
        shade: "Blue"
      },
      Colour {
        name: "Temptress",
        hex: "3C2126",
        rgb: &[60, 33, 38],
        hsl: &[-7, 74, 46],
        shade: "Brown"
      },
      Colour {
        name: "Tenne (Tawny)",
        hex: "CD5700",
        rgb: &[205, 87, 0],
        hsl: &[18, 255, 102],
        shade: "Orange"
      },
      Colour {
        name: "Tequila",
        hex: "F4D0A4",
        rgb: &[244, 208, 164],
        hsl: &[23, 200, 204],
        shade: "Yellow"
      },
      Colour {
        name: "Terra Cotta",
        hex: "E2725B",
        rgb: &[226, 114, 91],
        hsl: &[7, 178, 158],
        shade: "Red"
      },
      Colour {
        name: "Texas",
        hex: "ECE67E",
        rgb: &[236, 230, 126],
        hsl: &[40, 189, 181],
        shade: "Green"
      },
      Colour {
        name: "Texas Rose",
        hex: "FCB057",
        rgb: &[252, 176, 87],
        hsl: &[22, 246, 169],
        shade: "Orange"
      },
      Colour {
        name: "Thatch",
        hex: "B1948F",
        rgb: &[177, 148, 143],
        hsl: &[6, 45, 160],
        shade: "Brown"
      },
      Colour {
        name: "Thatch Green",
        hex: "544E31",
        rgb: &[84, 78, 49],
        hsl: &[35, 67, 66],
        shade: "Yellow"
      },
      Colour {
        name: "Thistle",
        hex: "D8BFD8",
        rgb: &[216, 191, 216],
        hsl: &[-42, 61, 203],
        shade: "Violet"
      },
      Colour {
        name: "Thunder",
        hex: "4D4D4B",
        rgb: &[77, 77, 75],
        hsl: &[42, 3, 76],
        shade: "Grey"
      },
      Colour {
        name: "Thunderbird",
        hex: "923830",
        rgb: &[146, 56, 48],
        hsl: &[3, 128, 97],
        shade: "Red"
      },
      Colour {
        name: "Tia Maria",
        hex: "97422D",
        rgb: &[151, 66, 45],
        hsl: &[8, 137, 98],
        shade: "Orange"
      },
      Colour {
        name: "Tiara",
        hex: "B9C3BE",
        rgb: &[185, 195, 190],
        hsl: &[106, 19, 190],
        shade: "Grey"
      },
      Colour {
        name: "Tiber",
        hex: "184343",
        rgb: &[24, 67, 67],
        hsl: &[127, 120, 45],
        shade: "Green"
      },
      Colour {
        name: "Tickle Me Pink",
        hex: "FC80A5",
        rgb: &[252, 128, 165],
        hsl: &[-12, 243, 190],
        shade: "Red"
      },
      Colour {
        name: "Tidal",
        hex: "F0F590",
        rgb: &[240, 245, 144],
        hsl: &[44, 212, 194],
        shade: "Green"
      },
      Colour {
        name: "Tide",
        hex: "BEB4AB",
        rgb: &[190, 180, 171],
        hsl: &[20, 32, 180],
        shade: "Brown"
      },
      Colour {
        name: "Timber Green",
        hex: "324336",
        rgb: &[50, 67, 54],
        hsl: &[95, 37, 58],
        shade: "Green"
      },
      Colour {
        name: "Timberwolf",
        hex: "D9D6CF",
        rgb: &[217, 214, 207],
        hsl: &[29, 29, 211],
        shade: "Grey"
      },
      Colour {
        name: "Titan White",
        hex: "DDD6E1",
        rgb: &[221, 214, 225],
        hsl: &[197, 39, 219],
        shade: "Violet"
      },
      Colour {
        name: "Toast",
        hex: "9F715F",
        rgb: &[159, 113, 95],
        hsl: &[11, 64, 127],
        shade: "Brown"
      },
      Colour {
        name: "Tobacco Brown",
        hex: "6D5843",
        rgb: &[109, 88, 67],
        hsl: &[21, 60, 88],
        shade: "Brown"
      },
      Colour {
        name: "Tobago",
        hex: "44362D",
        rgb: &[68, 54, 45],
        hsl: &[16, 51, 56],
        shade: "Brown"
      },
      Colour {
        name: "Toledo",
        hex: "3E2631",
        rgb: &[62, 38, 49],
        hsl: &[-19, 61, 50],
        shade: "Violet"
      },
      Colour {
        name: "Tolopea",
        hex: "2D2541",
        rgb: &[45, 37, 65],
        hsl: &[182, 69, 51],
        shade: "Violet"
      },
      Colour {
        name: "Tom Thumb",
        hex: "4F6348",
        rgb: &[79, 99, 72],
        hsl: &[73, 40, 85],
        shade: "Green"
      },
      Colour {
        name: "Tomato",
        hex: "FF6347",
        rgb: &[255, 99, 71],
        hsl: &[6, 254, 163],
        shade: "Red"
      },
      Colour {
        name: "Tonys Pink",
        hex: "E79E88",
        rgb: &[231, 158, 136],
        hsl: &[9, 169, 183],
        shade: "Orange"
      },
      Colour {
        name: "Topaz",
        hex: "817C87",
        rgb: &[129, 124, 135],
        hsl: &[189, 11, 129],
        shade: "Violet"
      },
      Colour {
        name: "Torch Red",
        hex: "FD0E35",
        rgb: &[253, 14, 53],
        hsl: &[-6, 250, 133],
        shade: "Red"
      },
      Colour {
        name: "Torea Bay",
        hex: "353D75",
        rgb: &[53, 61, 117],
        hsl: &[164, 96, 85],
        shade: "Blue"
      },
      Colour {
        name: "Tory Blue",
        hex: "374E88",
        rgb: &[55, 78, 136],
        hsl: &[157, 108, 95],
        shade: "Blue"
      },
      Colour {
        name: "Tosca",
        hex: "744042",
        rgb: &[116, 64, 66],
        hsl: &[-1, 73, 89],
        shade: "Red"
      },
      Colour {
        name: "Tower Grey",
        hex: "9CACA5",
        rgb: &[156, 172, 165],
        hsl: &[108, 22, 164],
        shade: "Green"
      },
      Colour {
        name: "Tradewind",
        hex: "6DAFA7",
        rgb: &[109, 175, 167],
        hsl: &[122, 74, 142],
        shade: "Green"
      },
      Colour {
        name: "Tranquil",
        hex: "DDEDE9",
        rgb: &[221, 237, 233],
        hsl: &[116, 78, 229],
        shade: "Blue"
      },
      Colour {
        name: "Travertine",
        hex: "E2DDC7",
        rgb: &[226, 221, 199],
        hsl: &[34, 80, 212],
        shade: "Green"
      },
      Colour {
        name: "Tree Poppy",
        hex: "E2813B",
        rgb: &[226, 129, 59],
        hsl: &[17, 189, 142],
        shade: "Orange"
      },
      Colour {
        name: "Trendy Green",
        hex: "7E8424",
        rgb: &[126, 132, 36],
        hsl: &[45, 145, 84],
        shade: "Green"
      },
      Colour {
        name: "Trendy Pink",
        hex: "805D80",
        rgb: &[128, 93, 128],
        hsl: &[-42, 40, 110],
        shade: "Violet"
      },
      Colour {
        name: "Trinidad",
        hex: "C54F33",
        rgb: &[197, 79, 51],
        hsl: &[8, 150, 124],
        shade: "Orange"
      },
      Colour {
        name: "Tropical Blue",
        hex: "AEC9EB",
        rgb: &[174, 201, 235],
        hsl: &[151, 154, 204],
        shade: "Blue"
      },
      Colour {
        name: "Tropical Rain Forest",
        hex: "00755E",
        rgb: &[0, 117, 94],
        hsl: &[119, 255, 58],
        shade: "Green"
      },
      Colour {
        name: "Trout",
        hex: "4C5356",
        rgb: &[76, 83, 86],
        hsl: &[140, 15, 81],
        shade: "Grey"
      },
      Colour {
        name: "True V",
        hex: "8E72C7",
        rgb: &[142, 114, 199],
        hsl: &[184, 110, 156],
        shade: "Violet"
      },
      Colour {
        name: "Tuatara",
        hex: "454642",
        rgb: &[69, 70, 66],
        hsl: &[53, 7, 68],
        shade: "Grey"
      },
      Colour {
        name: "Tuft Bush",
        hex: "F9D3BE",
        rgb: &[249, 211, 190],
        hsl: &[15, 211, 219],
        shade: "Orange"
      },
      Colour {
        name: "Tulip Tree",
        hex: "E3AC3D",
        rgb: &[227, 172, 61],
        hsl: &[28, 190, 144],
        shade: "Yellow"
      },
      Colour {
        name: "Tumbleweed",
        hex: "DEA681",
        rgb: &[222, 166, 129],
        hsl: &[16, 149, 175],
        shade: "Brown"
      },
      Colour {
        name: "Tuna",
        hex: "46494E",
        rgb: &[70, 73, 78],
        hsl: &[154, 13, 74],
        shade: "Grey"
      },
      Colour {
        name: "Tundora",
        hex: "585452",
        rgb: &[88, 84, 82],
        hsl: &[14, 8, 85],
        shade: "Grey"
      },
      Colour {
        name: "Turbo",
        hex: "F5CC23",
        rgb: &[245, 204, 35],
        hsl: &[34, 232, 140],
        shade: "Yellow"
      },
      Colour {
        name: "Turkish Rose",
        hex: "A56E75",
        rgb: &[165, 110, 117],
        hsl: &[-5, 59, 137],
        shade: "Red"
      },
      Colour {
        name: "Turmeric",
        hex: "AE9041",
        rgb: &[174, 144, 65],
        hsl: &[30, 116, 119],
        shade: "Yellow"
      },
      Colour {
        name: "Turquoise",
        hex: "40E0D0",
        rgb: &[64, 224, 208],
        hsl: &[123, 183, 144],
        shade: "Blue"
      },
      Colour {
        name: "Turquoise Blue",
        hex: "6CDAE7",
        rgb: &[108, 218, 231],
        hsl: &[131, 183, 169],
        shade: "Blue"
      },
      Colour {
        name: "Turtle Green",
        hex: "363E1D",
        rgb: &[54, 62, 29],
        hsl: &[52, 92, 45],
        shade: "Green"
      },
      Colour {
        name: "Tuscany",
        hex: "AD6242",
        rgb: &[173, 98, 66],
        hsl: &[12, 114, 119],
        shade: "Orange"
      },
      Colour {
        name: "Tusk",
        hex: "E3E5B1",
        rgb: &[227, 229, 177],
        hsl: &[44, 127, 203],
        shade: "Green"
      },
      Colour {
        name: "Tussock",
        hex: "BF914B",
        rgb: &[191, 145, 75],
        hsl: &[25, 121, 133],
        shade: "Yellow"
      },
      Colour {
        name: "Tutu",
        hex: "F8E4E3",
        rgb: &[248, 228, 227],
        hsl: &[2, 153, 237],
        shade: "Red"
      },
      Colour {
        name: "Twilight",
        hex: "DAC0CD",
        rgb: &[218, 192, 205],
        hsl: &[-21, 66, 205],
        shade: "Violet"
      },
      Colour {
        name: "Twilight Blue",
        hex: "F4F6EC",
        rgb: &[244, 246, 236],
        hsl: &[50, 91, 241],
        shade: "Grey"
      },
      Colour {
        name: "Twine",
        hex: "C19156",
        rgb: &[193, 145, 86],
        hsl: &[23, 118, 139],
        shade: "Yellow"
      },
      Colour {
        name: "Tyrian Purple",
        hex: "66023C",
        rgb: &[102, 2, 60],
        hsl: &[-24, 245, 52],
        shade: "Violet"
      },
      Colour {
        name: "Ultra Pink",
        hex: "FF6FFF",
        rgb: &[255, 111, 255],
        hsl: &[-42, 255, 183],
        shade: "Red"
      },
      Colour {
        name: "Ultramarine",
        hex: "120A8F",
        rgb: &[18, 10, 143],
        hsl: &[172, 221, 76],
        shade: "Blue"
      },
      Colour {
        name: "Valencia",
        hex: "D4574E",
        rgb: &[212, 87, 78],
        hsl: &[2, 155, 145],
        shade: "Red"
      },
      Colour {
        name: "Valentino",
        hex: "382C38",
        rgb: &[56, 44, 56],
        hsl: &[-42, 30, 50],
        shade: "Violet"
      },
      Colour {
        name: "Valhalla",
        hex: "2A2B41",
        rgb: &[42, 43, 65],
        hsl: &[168, 54, 53],
        shade: "Violet"
      },
      Colour {
        name: "Van Cleef",
        hex: "523936",
        rgb: &[82, 57, 54],
        hsl: &[4, 52, 68],
        shade: "Brown"
      },
      Colour {
        name: "Vanilla",
        hex: "CCB69B",
        rgb: &[204, 182, 155],
        hsl: &[23, 82, 179],
        shade: "Brown"
      },
      Colour {
        name: "Vanilla Ice",
        hex: "EBD2D1",
        rgb: &[235, 210, 209],
        hsl: &[1, 100, 222],
        shade: "Red"
      },
      Colour {
        name: "Varden",
        hex: "FDEFD3",
        rgb: &[253, 239, 211],
        hsl: &[28, 232, 232],
        shade: "Yellow"
      },
      Colour {
        name: "Venetian Red",
        hex: "C80815",
        rgb: &[200, 8, 21],
        hsl: &[-2, 235, 104],
        shade: "Red"
      },
      Colour {
        name: "Venice Blue",
        hex: "2C5778",
        rgb: &[44, 87, 120],
        hsl: &[145, 118, 81],
        shade: "Blue"
      },
      Colour {
        name: "Venus",
        hex: "8B7D82",
        rgb: &[139, 125, 130],
        hsl: &[-15, 14, 131],
        shade: "Violet"
      },
      Colour {
        name: "Verdigris",
        hex: "62603E",
        rgb: &[98, 96, 62],
        hsl: &[40, 57, 80],
        shade: "Grey"
      },
      Colour {
        name: "Verdun Green",
        hex: "48531A",
        rgb: &[72, 83, 26],
        hsl: &[50, 133, 54],
        shade: "Green"
      },
      Colour {
        name: "Vermilion",
        hex: "FF4D00",
        rgb: &[255, 77, 0],
        hsl: &[12, 255, 127],
        shade: "Red"
      },
      Colour {
        name: "Very Dark Brown",
        hex: "5C4033",
        rgb: &[92, 64, 51],
        hsl: &[13, 73, 71],
        shade: "Brown"
      },
      Colour {
        name: "Very Light Grey",
        hex: "CDCDCD",
        rgb: &[205, 205, 205],
        hsl: &[0, 0, 205],
        shade: "Grey"
      },
      Colour {
        name: "Vesuvius",
        hex: "A85533",
        rgb: &[168, 85, 51],
        hsl: &[12, 136, 109],
        shade: "Orange"
      },
      Colour {
        name: "Victoria",
        hex: "564985",
        rgb: &[86, 73, 133],
        hsl: &[179, 74, 103],
        shade: "Violet"
      },
      Colour {
        name: "Vida Loca",
        hex: "5F9228",
        rgb: &[95, 146, 40],
        hsl: &[62, 145, 93],
        shade: "Green"
      },
      Colour {
        name: "Viking",
        hex: "4DB1C8",
        rgb: &[77, 177, 200],
        hsl: &[135, 134, 138],
        shade: "Blue"
      },
      Colour {
        name: "Vin Rouge",
        hex: "955264",
        rgb: &[149, 82, 100],
        hsl: &[-11, 73, 115],
        shade: "Red"
      },
      Colour {
        name: "Viola",
        hex: "C58F9D",
        rgb: &[197, 143, 157],
        hsl: &[-11, 81, 170],
        shade: "Red"
      },
      Colour {
        name: "Violent Violet",
        hex: "2E2249",
        rgb: &[46, 34, 73],
        hsl: &[183, 92, 53],
        shade: "Violet"
      },
      Colour {
        name: "Violet",
        hex: "EE82EE",
        rgb: &[238, 130, 238],
        hsl: &[-42, 193, 184],
        shade: "Violet"
      },
      Colour {
        name: "Violet Blue",
        hex: "9F5F9F",
        rgb: &[159, 95, 159],
        hsl: &[-42, 64, 127],
        shade: "Violet"
      },
      Colour {
        name: "Violet Red",
        hex: "F7468A",
        rgb: &[247, 70, 138],
        hsl: &[-16, 233, 158],
        shade: "Red"
      },
      Colour {
        name: "Viridian",
        hex: "40826D",
        rgb: &[64, 130, 109],
        hsl: &[113, 86, 97],
        shade: "Blue"
      },
      Colour {
        name: "Viridian Green",
        hex: "4B5F56",
        rgb: &[75, 95, 86],
        hsl: &[108, 29, 85],
        shade: "Green"
      },
      Colour {
        name: "Vis Vis",
        hex: "F9E496",
        rgb: &[249, 228, 150],
        hsl: &[33, 227, 199],
        shade: "Yellow"
      },
      Colour {
        name: "Vista Blue",
        hex: "97D5B3",
        rgb: &[151, 213, 179],
        hsl: &[104, 108, 182],
        shade: "Green"
      },
      Colour {
        name: "Vista White",
        hex: "E3DFD9",
        rgb: &[227, 223, 217],
        hsl: &[25, 38, 222],
        shade: "Grey"
      },
      Colour {
        name: "Vivid Tangerine",
        hex: "FF9980",
        rgb: &[255, 153, 128],
        hsl: &[8, 255, 191],
        shade: "Orange"
      },
      Colour {
        name: "Vivid Violet",
        hex: "803790",
        rgb: &[128, 55, 144],
        hsl: &[204, 114, 99],
        shade: "Violet"
      },
      Colour {
        name: "Volcano",
        hex: "4E2728",
        rgb: &[78, 39, 40],
        hsl: &[-1, 85, 58],
        shade: "Red"
      },
      Colour {
        name: "Voodoo",
        hex: "443240",
        rgb: &[68, 50, 64],
        hsl: &[-33, 38, 59],
        shade: "Violet"
      },
      Colour {
        name: "Vulcan",
        hex: "36383C",
        rgb: &[54, 56, 60],
        hsl: &[155, 13, 56],
        shade: "Grey"
      },
      Colour {
        name: "Wafer",
        hex: "D4BBB1",
        rgb: &[212, 187, 177],
        hsl: &[12, 73, 194],
        shade: "Orange"
      },
      Colour {
        name: "Waikawa Grey",
        hex: "5B6E91",
        rgb: &[91, 110, 145],
        hsl: &[155, 58, 118],
        shade: "Blue"
      },
      Colour {
        name: "Waiouru",
        hex: "4C4E31",
        rgb: &[76, 78, 49],
        hsl: &[45, 58, 63],
        shade: "Green"
      },
      Colour {
        name: "Wan White",
        hex: "E4E2DC",
        rgb: &[228, 226, 220],
        hsl: &[31, 32, 224],
        shade: "Grey"
      },
      Colour {
        name: "Wasabi",
        hex: "849137",
        rgb: &[132, 145, 55],
        hsl: &[48, 114, 100],
        shade: "Green"
      },
      Colour {
        name: "Water Leaf",
        hex: "B6ECDE",
        rgb: &[182, 236, 222],
        hsl: &[116, 149, 209],
        shade: "Green"
      },
      Colour {
        name: "Watercourse",
        hex: "006E4E",
        rgb: &[0, 110, 78],
        hsl: &[115, 255, 55],
        shade: "Green"
      },
      Colour {
        name: "Wattle",
        hex: "D6CA3D",
        rgb: &[214, 202, 61],
        hsl: &[39, 166, 137],
        shade: "Green"
      },
      Colour {
        name: "Watusi",
        hex: "F2CDBB",
        rgb: &[242, 205, 187],
        hsl: &[13, 173, 214],
        shade: "Orange"
      },
      Colour {
        name: "Wax Flower",
        hex: "EEB39E",
        rgb: &[238, 179, 158],
        hsl: &[11, 178, 198],
        shade: "Orange"
      },
      Colour {
        name: "We Peep",
        hex: "FDD7D8",
        rgb: &[253, 215, 216],
        hsl: &[-1, 230, 234],
        shade: "Red"
      },
      Colour {
        name: "Wedgewood",
        hex: "4C6B88",
        rgb: &[76, 107, 136],
        hsl: &[148, 72, 105],
        shade: "Blue"
      },
      Colour {
        name: "Well Read",
        hex: "8E3537",
        rgb: &[142, 53, 55],
        hsl: &[0, 116, 97],
        shade: "Red"
      },
      Colour {
        name: "West Coast",
        hex: "5C512F",
        rgb: &[92, 81, 47],
        hsl: &[32, 82, 69],
        shade: "Yellow"
      },
      Colour {
        name: "West Side",
        hex: "E5823A",
        rgb: &[229, 130, 58],
        hsl: &[17, 195, 143],
        shade: "Orange"
      },
      Colour {
        name: "Westar",
        hex: "D4CFC5",
        rgb: &[212, 207, 197],
        hsl: &[28, 37, 204],
        shade: "Grey"
      },
      Colour {
        name: "Wewak",
        hex: "F1919A",
        rgb: &[241, 145, 154],
        hsl: &[-3, 197, 193],
        shade: "Red"
      },
      Colour {
        name: "Wheat",
        hex: "F5DEB3",
        rgb: &[245, 222, 179],
        hsl: &[27, 195, 211],
        shade: "Brown"
      },
      Colour {
        name: "Wheatfield",
        hex: "DFD7BD",
        rgb: &[223, 215, 189],
        hsl: &[32, 88, 206],
        shade: "Yellow"
      },
      Colour {
        name: "Whiskey",
        hex: "D29062",
        rgb: &[210, 144, 98],
        hsl: &[17, 141, 154],
        shade: "Orange"
      },
      Colour {
        name: "Whiskey Sour",
        hex: "D4915D",
        rgb: &[212, 145, 93],
        hsl: &[18, 148, 152],
        shade: "Orange"
      },
      Colour {
        name: "Whisper",
        hex: "EFE6E6",
        rgb: &[239, 230, 230],
        hsl: &[0, 55, 234],
        shade: "Grey"
      },
      Colour {
        name: "White",
        hex: "FFFFFF",
        rgb: &[255, 255, 255],
        hsl: &[0, 0, 255],
        shade: "White"
      },
      Colour {
        name: "White Ice",
        hex: "D7EEE4",
        rgb: &[215, 238, 228],
        hsl: &[109, 102, 226],
        shade: "Green"
      },
      Colour {
        name: "White Lilac",
        hex: "E7E5E8",
        rgb: &[231, 229, 232],
        hsl: &[198, 15, 230],
        shade: "Blue"
      },
      Colour {
        name: "White Linen",
        hex: "EEE7DC",
        rgb: &[238, 231, 220],
        hsl: &[25, 88, 229],
        shade: "Grey"
      },
      Colour {
        name: "White Nectar",
        hex: "F8F6D8",
        rgb: &[248, 246, 216],
        hsl: &[39, 177, 232],
        shade: "Green"
      },
      Colour {
        name: "White Pointer",
        hex: "DAD6CC",
        rgb: &[218, 214, 204],
        hsl: &[30, 40, 211],
        shade: "Grey"
      },
      Colour {
        name: "White Rock",
        hex: "D4CFB4",
        rgb: &[212, 207, 180],
        hsl: &[35, 69, 196],
        shade: "Green"
      },
      Colour {
        name: "White Smoke",
        hex: "F5F5F5",
        rgb: &[245, 245, 245],
        hsl: &[0, 0, 245],
        shade: "White"
      },
      Colour {
        name: "Wild Blue Yonder",
        hex: "7A89B8",
        rgb: &[122, 137, 184],
        hsl: &[159, 77, 153],
        shade: "Blue"
      },
      Colour {
        name: "Wild Rice",
        hex: "E3D474",
        rgb: &[227, 212, 116],
        hsl: &[36, 169, 171],
        shade: "Green"
      },
      Colour {
        name: "Wild Sand",
        hex: "E7E4DE",
        rgb: &[231, 228, 222],
        hsl: &[28, 40, 226],
        shade: "Grey"
      },
      Colour {
        name: "Wild Strawberry",
        hex: "FF3399",
        rgb: &[255, 51, 153],
        hsl: &[-21, 255, 153],
        shade: "Red"
      },
      Colour {
        name: "Wild Watermelon",
        hex: "FD5B78",
        rgb: &[253, 91, 120],
        hsl: &[-7, 248, 172],
        shade: "Red"
      },
      Colour {
        name: "Wild Willow",
        hex: "BECA60",
        rgb: &[190, 202, 96],
        hsl: &[47, 127, 148],
        shade: "Green"
      },
      Colour {
        name: "William",
        hex: "53736F",
        rgb: &[83, 115, 111],
        hsl: &[122, 41, 99],
        shade: "Green"
      },
      Colour {
        name: "Willow Brook",
        hex: "DFE6CF",
        rgb: &[223, 230, 207],
        hsl: &[55, 80, 218],
        shade: "Green"
      },
      Colour {
        name: "Willow Grove",
        hex: "69755C",
        rgb: &[105, 117, 92],
        hsl: &[62, 30, 104],
        shade: "Green"
      },
      Colour {
        name: "Windsor",
        hex: "462C77",
        rgb: &[70, 44, 119],
        hsl: &[184, 117, 81],
        shade: "Violet"
      },
      Colour {
        name: "Wine Berry",
        hex: "522C35",
        rgb: &[82, 44, 53],
        hsl: &[-10, 76, 63],
        shade: "Red"
      },
      Colour {
        name: "Winter Hazel",
        hex: "D0C383",
        rgb: &[208, 195, 131],
        hsl: &[35, 114, 169],
        shade: "Yellow"
      },
      Colour {
        name: "Wisp Pink",
        hex: "F9E8E2",
        rgb: &[249, 232, 226],
        hsl: &[11, 167, 237],
        shade: "Red"
      },
      Colour {
        name: "Wisteria",
        hex: "C9A0DC",
        rgb: &[201, 160, 220],
        hsl: &[199, 117, 190],
        shade: "Violet"
      },
      Colour {
        name: "Wistful",
        hex: "A29ECD",
        rgb: &[162, 158, 205],
        hsl: &[173, 81, 181],
        shade: "Blue"
      },
      Colour {
        name: "Witch Haze",
        hex: "FBF073",
        rgb: &[251, 240, 115],
        hsl: &[39, 240, 183],
        shade: "Green"
      },
      Colour {
        name: "Wood Bark",
        hex: "302621",
        rgb: &[48, 38, 33],
        hsl: &[14, 47, 40],
        shade: "Brown"
      },
      Colour {
        name: "Woodburn",
        hex: "463629",
        rgb: &[70, 54, 41],
        hsl: &[19, 66, 55],
        shade: "Brown"
      },
      Colour {
        name: "Woodland",
        hex: "626746",
        rgb: &[98, 103, 70],
        hsl: &[48, 48, 86],
        shade: "Green"
      },
      Colour {
        name: "Woodrush",
        hex: "45402B",
        rgb: &[69, 64, 43],
        hsl: &[34, 59, 56],
        shade: "Yellow"
      },
      Colour {
        name: "Woodsmoke",
        hex: "2B3230",
        rgb: &[43, 50, 48],
        hsl: &[115, 19, 46],
        shade: "Grey"
      },
      Colour {
        name: "Woody Brown",
        hex: "554545",
        rgb: &[85, 69, 69],
        hsl: &[0, 26, 77],
        shade: "Brown"
      },
      Colour {
        name: "Xanadu",
        hex: "75876E",
        rgb: &[117, 135, 110],
        hsl: &[73, 26, 122],
        shade: "Green"
      },
      Colour {
        name: "Yellow",
        hex: "FFFF00",
        rgb: &[255, 255, 0],
        hsl: &[42, 255, 127],
        shade: "Yellow"
      },
      Colour {
        name: "Yellow Green",
        hex: "9ACD32",
        rgb: &[154, 205, 50],
        hsl: &[56, 155, 127],
        shade: "Green"
      },
      Colour {
        name: "Yellow Metal",
        hex: "73633E",
        rgb: &[115, 99, 62],
        hsl: &[29, 76, 88],
        shade: "Yellow"
      },
      Colour {
        name: "Yellow Orange",
        hex: "FFAE42",
        rgb: &[255, 174, 66],
        hsl: &[24, 255, 160],
        shade: "Orange"
      },
      Colour {
        name: "Yellow Sea",
        hex: "F49F35",
        rgb: &[244, 159, 53],
        hsl: &[23, 228, 148],
        shade: "Yellow"
      },
      Colour {
        name: "Your Pink",
        hex: "FFC5BB",
        rgb: &[255, 197, 187],
        hsl: &[6, 255, 221],
        shade: "Red"
      },
      Colour {
        name: "Yukon Gold",
        hex: "826A21",
        rgb: &[130, 106, 33],
        hsl: &[31, 151, 81],
        shade: "Yellow"
      },
      Colour {
        name: "Yuma",
        hex: "C7B882",
        rgb: &[199, 184, 130],
        hsl: &[33, 97, 164],
        shade: "Yellow"
      },
      Colour {
        name: "Zambezi",
        hex: "6B5A5A",
        rgb: &[107, 90, 90],
        hsl: &[0, 22, 98],
        shade: "Brown"
      },
      Colour {
        name: "Zanah",
        hex: "B2C6B1",
        rgb: &[178, 198, 177],
        hsl: &[82, 39, 187],
        shade: "Green"
      },
      Colour {
        name: "Zest",
        hex: "C6723B",
        rgb: &[198, 114, 59],
        hsl: &[16, 140, 128],
        shade: "Orange"
      },
      Colour {
        name: "Zeus",
        hex: "3B3C38",
        rgb: &[59, 60, 56],
        hsl: &[53, 8, 58],
        shade: "Grey"
      },
      Colour {
        name: "Ziggurat",
        hex: "81A6AA",
        rgb: &[129, 166, 170],
        hsl: &[131, 49, 149],
        shade: "Blue"
      },
      Colour {
        name: "Zinnwaldite",
        hex: "EBC2AF",
        rgb: &[235, 194, 175],
        hsl: &[13, 153, 205],
        shade: "Brown"
      },
      Colour {
        name: "Zircon",
        hex: "DEE3E3",
        rgb: &[222, 227, 227],
        hsl: &[127, 20, 224],
        shade: "Grey"
      },
      Colour {
        name: "Zombie",
        hex: "DDC283",
        rgb: &[221, 194, 131],
        hsl: &[29, 145, 176],
        shade: "Yellow"
      },
      Colour {
        name: "Zorba",
        hex: "A29589",
        rgb: &[162, 149, 137],
        hsl: &[20, 30, 149],
        shade: "Brown"
      },
      Colour {
        name: "Zuccini",
        hex: "17462E",
        rgb: &[23, 70, 46],
        hsl: &[105, 128, 46],
        shade: "Green"
      },
      Colour {
        name: "Zumthor",
        hex: "CDD5D5",
        rgb: &[205, 213, 213],
        hsl: &[127, 22, 209],
        shade: "Grey"
      }
];