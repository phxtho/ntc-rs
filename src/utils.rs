pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn hex_to_i32(hex_string: &str) -> i32 {
    // Hex string to 4-bytes, aka. u32
    let parsed_int:i32 = i32::from_str_radix(hex_string, 16).unwrap();
    return parsed_int;
}
