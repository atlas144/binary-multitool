use std::io::stdin;
use std::num::ParseIntError;

pub fn read_uint() -> Result<u32, ParseIntError> {
    let mut input_str = String::new();

    stdin()
        .read_line(&mut input_str)
        .expect("Input reading failed!");

    let trimmed = input_str.trim();

    trimmed.parse::<u32>()
}