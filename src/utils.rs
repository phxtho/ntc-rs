pub fn hex_to_i32(hex_string: &str) -> i32 {
    // Hex string to 4-bytes, aka. u32
    let parsed_int: i32 = i32::from_str_radix(hex_string, 16).unwrap();
    return parsed_int;
}
